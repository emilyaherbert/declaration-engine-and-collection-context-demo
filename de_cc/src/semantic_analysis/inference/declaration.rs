use crate::{
    collection_context::{collection_context::cc_get_node, collection_index::CollectionIndex},
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

pub(super) fn analyze_declaration(namespace: &mut Namespace, declaration: &TyDeclaration) {
    match declaration {
        decl @ TyDeclaration::Variable(variable_declaration) => {
            analyze_variable(namespace, variable_declaration);
            let name = variable_declaration.name.clone();
            namespace.insert_symbol(name, decl.clone());
        }
        decl @ TyDeclaration::Function(decl_id) => {
            let function_declaration = de_get_function(*decl_id).unwrap();
            analyze_function(&mut namespace.scoped(), &function_declaration);
            let name = function_declaration.name;
            namespace.insert_symbol(name, decl.clone());
        }
        decl @ TyDeclaration::Trait(decl_id) => {
            let trait_declaration = de_get_trait(*decl_id).unwrap();
            analyze_trait(namespace, &trait_declaration);
            let name = trait_declaration.name;
            namespace.insert_symbol(name, decl.clone());
        }
        TyDeclaration::TraitImpl(decl_id) => {
            let trait_impl = de_get_trait_impl(*decl_id).unwrap();
            analyze_trait_impl(&mut namespace.scoped(), &trait_impl);
            namespace.insert_methods(
                trait_impl.type_implementing_for,
                trait_impl.trait_name.clone(),
                trait_impl.methods,
            );
        }
        decl @ TyDeclaration::Struct(decl_id) => {
            let struct_declaration = de_get_struct(*decl_id).unwrap();
            analyze_struct(&mut namespace.scoped(), &struct_declaration);
            let name = struct_declaration.name;
            namespace.insert_symbol(name, decl.clone());
        }
    }
}

fn analyze_variable(namespace: &mut Namespace, variable_declaration: &TyVariableDeclaration) {
    // do type inference on the value
    analyze_expression(namespace, &variable_declaration.body);

    // resolve any custom types in the type ascription
    resolve_custom_types(variable_declaration.type_ascription, namespace).unwrap();

    // unify the type of the value and the type ascription
    unify_types(
        variable_declaration.body.type_id,
        variable_declaration.type_ascription,
    )
    .unwrap();
}

fn analyze_function(namespace: &mut Namespace, function_declaration: &TyFunctionDeclaration) {
    // import the trait constraints into the namespace
    for type_parameter in function_declaration.type_parameters.iter() {
        // if the type param has a trait constraint, take the TypedTraitFn's from
        // the trait it is constrained upon and insert them into the namespace
        // under the type param
        if let Some(constraint) = &type_parameter.trait_constraint {
            let decl_id = namespace
                .get_symbol(&constraint.trait_name)
                .unwrap()
                .expect_trait()
                .unwrap();
            let trait_decl = de_get_trait(decl_id).unwrap();
            namespace.insert_methods(
                type_parameter.type_id,
                constraint.trait_name.clone(),
                trait_decl.interface_surface,
            );
        }
    }

    // resolve any custom types in the parameters and
    // insert the type parameters into the namespace
    for parameter in function_declaration.parameters.iter() {
        resolve_custom_types(parameter.type_id, namespace).unwrap();
        namespace.insert_symbol(parameter.name.clone(), parameter.into());
    }

    // do type inference on the function body
    let typed_body_return_type = analyze_code_block(namespace, &function_declaration.body);

    // resolve any custom types in the function return type
    resolve_custom_types(function_declaration.return_type, namespace).unwrap();

    // unify the function return type and body return type
    unify_types(typed_body_return_type, function_declaration.return_type).unwrap();
}

fn analyze_code_block(namespace: &mut Namespace, nodes: &[CollectionIndex]) -> TypeId {
    for node_index in nodes.iter() {
        let node = cc_get_node(node_index);
        let node = node.expect_node().unwrap();
        analyze_node(namespace, node);
        if let TyNode::ReturnStatement(exp) = node {
            return exp.type_id;
        }
    }
    insert_type(TypeInfo::Unit)
}

fn analyze_trait_impl(namespace: &mut Namespace, trait_impl: &TyTraitImpl) {
    if !trait_impl.type_parameters.is_empty() {
        panic!("no type parameters yet");
    }

    // get the trait from the declaration engine
    let trait_id = namespace
        .get_symbol(&trait_impl.trait_name)
        .unwrap()
        .expect_trait()
        .unwrap();
    let _trait_decl = de_get_trait(trait_id).unwrap();

    // resolve any custom types in the type we are implementing for
    resolve_custom_types(trait_impl.type_implementing_for, namespace).unwrap();

    // TODO: check to see if all of the methods are implementing, no new methods implementing,
    // when generic traits are implemented add the monomorphized copies to the declaration
    // engine

    // do type inference on the methods
    trait_impl.methods.iter().for_each(|method_id| {
        let method = de_get_function(*method_id).unwrap();
        analyze_function(namespace, &method);
    });
}

fn analyze_struct(namespace: &mut Namespace, struct_declaration: &TyStructDeclaration) {
    // do type inference on the fields
    struct_declaration.fields.iter().for_each(|field| {
        resolve_custom_types(field.type_id, namespace).unwrap();
    });
}

fn analyze_trait(namespace: &mut Namespace, trait_declaration: &TyTraitDeclaration) {
    // do type inference on the interface
    trait_declaration
        .interface_surface
        .iter()
        .for_each(|trait_fn_id| {
            let trait_fn = de_get_trait_fn(*trait_fn_id).unwrap();
            analyze_trait_fn(namespace, &trait_fn)
        });
}

fn analyze_trait_fn(namespace: &mut Namespace, trait_fn: &TyTraitFn) {
    // resolve any custom types in the parameters
    for parameter in trait_fn.parameters.iter() {
        resolve_custom_types(parameter.type_id, namespace).unwrap();
    }

    // resolve any custom types in the return type
    resolve_custom_types(trait_fn.return_type, namespace).unwrap();
}
