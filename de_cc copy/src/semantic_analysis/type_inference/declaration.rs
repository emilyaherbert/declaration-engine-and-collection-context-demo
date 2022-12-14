use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_index::CollectionIndex,
    },
    declaration_engine::declaration_engine::*,
    language::ty::{
        typed_declaration::{
            TyDeclaration, TyFunctionDeclaration, TyStructDeclaration, TyTraitDeclaration,
            TyTraitFn, TyTraitImpl, TyVariableDeclaration,
        },
        TyNode,
    },
    namespace::namespace::Namespace,
    type_system::{
        type_engine::{insert_type, resolve_custom_types, unify_types},
        type_id::TypeId,
        type_info::TypeInfo,
    },
};

use super::{analyze_expression, analyze_node};

pub(super) fn analyze_declaration(
    cc: &CollectionContext,
    ns: &mut Namespace,
    node_index: CollectionIndex,
) {
    let declaration = cc.get_node(node_index).expect_declaration().unwrap();
    match declaration {
        TyDeclaration::Variable(variable_declaration) => {
            analyze_variable(cc, node_index, ns, variable_declaration);
            let name = variable_declaration.name.clone();
            ns.insert_symbol(name, TyDeclaration::Variable(variable_declaration.clone()));
        }
        TyDeclaration::Function(decl_id) => {
            let function_declaration = de_get_function(*decl_id).unwrap();
            let name = function_declaration.name.clone();
            analyze_function(cc, &mut ns.scoped(), function_declaration);
            ns.insert_symbol(name, TyDeclaration::Function(*decl_id));
        }
        TyDeclaration::Trait(decl_id) => {
            let trait_declaration = de_get_trait(*decl_id).unwrap();
            analyze_trait(&mut ns.scoped(), &trait_declaration);
            let name = trait_declaration.name;
            ns.insert_symbol(name, TyDeclaration::Trait(*decl_id));
        }
        TyDeclaration::TraitImpl(decl_id) => {
            let trait_impl = de_get_trait_impl(*decl_id).unwrap();
            analyze_trait_impl(cc, &mut ns.scoped(), &trait_impl);
            ns.insert_methods(
                trait_impl.type_implementing_for,
                trait_impl.trait_name.clone(),
                trait_impl.methods,
            );
        }
        TyDeclaration::Struct(decl_id) => {
            let struct_declaration = de_get_struct(*decl_id).unwrap();
            analyze_struct(&mut ns.scoped(), &struct_declaration);
            let name = struct_declaration.name;
            ns.insert_symbol(name, TyDeclaration::Struct(*decl_id));
        }
    }
}

fn analyze_variable(
    cc: &CollectionContext,
    current_index: CollectionIndex,
    ns: &mut Namespace,
    variable_declaration: &TyVariableDeclaration,
) {
    // do type inference on the value
    analyze_expression(cc, current_index, ns, &variable_declaration.body);

    // resolve any custom types in the type ascription
    resolve_custom_types(variable_declaration.type_ascription, ns).unwrap();

    // unify the type of the value and the type ascription
    unify_types(
        variable_declaration.body.type_id,
        variable_declaration.type_ascription,
    )
    .unwrap();
}

fn analyze_function(
    cc: &CollectionContext,
    ns: &mut Namespace,
    function_declaration: TyFunctionDeclaration,
) {
    // import the trait constraints into the ns
    for type_param in function_declaration.type_parameters.iter() {
        // if the type param has a trait constraint, take the TypedTraitFn's from
        // the trait it is constrained upon and insert them into the ns
        // under the type param
        if let Some(constraint) = &type_param.trait_constraint {
            let decl_id = ns
                .get_symbol(&constraint.trait_name)
                .unwrap()
                .expect_trait()
                .unwrap();
            let trait_decl = de_get_trait(decl_id).unwrap();
            ns.insert_methods(
                type_param.type_id,
                constraint.trait_name.clone(),
                trait_decl.interface_surface,
            );
        }
    }

    // resolve any custom types in the parameters and
    // insert the type parameters into the ns
    for param in function_declaration.parameters.iter() {
        resolve_custom_types(param.type_id, ns).unwrap();
        ns.insert_symbol(param.name.clone(), param.into());
    }

    // do type inference on the function body
    let typed_body_return_type = analyze_code_block(cc, ns, function_declaration.body);

    // resolve any custom types in the function return type
    resolve_custom_types(function_declaration.return_type, ns).unwrap();

    // unify the function return type and body return type
    unify_types(typed_body_return_type, function_declaration.return_type).unwrap();
}

fn analyze_code_block(
    cc: &CollectionContext,
    ns: &mut Namespace,
    nodes: Vec<CollectionIndex>,
) -> TypeId {
    for node_index in nodes.into_iter() {
        analyze_node(cc, ns, node_index);
        let node = cc.get_node(node_index).expect_node().unwrap();
        if let TyNode::ReturnStatement(exp) = node {
            return exp.type_id;
        }
    }
    insert_type(TypeInfo::Unit)
}

fn analyze_trait_impl(cc: &CollectionContext, ns: &mut Namespace, trait_impl: &TyTraitImpl) {
    if !trait_impl.type_parameters.is_empty() {
        panic!("no type parameters yet");
    }

    // get the trait from the declaration engine
    let trait_id = ns
        .get_symbol(&trait_impl.trait_name)
        .unwrap()
        .expect_trait()
        .unwrap();
    let _trait_decl = de_get_trait(trait_id).unwrap();

    // resolve any custom types in the type we are implementing for
    resolve_custom_types(trait_impl.type_implementing_for, ns).unwrap();

    // TODO: check to see if all of the methods are implementing, no new methods implementing,
    // when generic traits are implemented add the monomorphized copies to the declaration
    // engine

    // do type inference on the methods
    trait_impl.methods.iter().for_each(|method_id| {
        let method = de_get_function(*method_id).unwrap();
        analyze_function(cc, ns, method);
    });
}

fn analyze_struct(ns: &mut Namespace, struct_declaration: &TyStructDeclaration) {
    // do type inference on the fields
    struct_declaration.fields.iter().for_each(|field| {
        resolve_custom_types(field.type_id, ns).unwrap();
    });
}

fn analyze_trait(ns: &mut Namespace, trait_declaration: &TyTraitDeclaration) {
    // do type inference on the interface
    trait_declaration
        .interface_surface
        .iter()
        .for_each(|trait_fn_id| {
            let trait_fn = de_get_trait_fn(*trait_fn_id).unwrap();
            analyze_trait_fn(ns, &trait_fn)
        });
}

fn analyze_trait_fn(ns: &mut Namespace, trait_fn: &TyTraitFn) {
    // resolve any custom types in the parameters
    for parameter in trait_fn.parameters.iter() {
        resolve_custom_types(parameter.type_id, ns).unwrap();
    }

    // resolve any custom types in the return type
    resolve_custom_types(trait_fn.return_type, ns).unwrap();
}
