use crate::{
    collection_context::{
        collection_context::CollectionContext,
        collection_index::{CCIdx, CollectionIndex},
    },
    declaration_engine::{declaration_engine::*, declaration_id::DeclarationId},
    language::ty::{
        typed_declaration::{TyDeclaration, TyVariableDeclaration},
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
    decl: &mut CCIdx<TyDeclaration>,
) {
    let cc_idx = decl.idx();
    match decl.inner_ref_mut() {
        TyDeclaration::Variable(var_decl) => {
            analyze_variable(cc, cc_idx, ns, var_decl);
            let name = var_decl.name.clone();
            ns.insert_symbol(name, TyDeclaration::Variable(var_decl.clone()));
        }
        TyDeclaration::Function(decl_id) => {
            analyze_function(cc, &mut ns.scoped(), decl_id);
            let func_decl = de_get_function(*decl_id.inner_ref()).unwrap();
            let name = func_decl.name;
            ns.insert_symbol(name, TyDeclaration::Function(decl_id.clone()));
        }
        TyDeclaration::Trait(decl_id) => {
            analyze_trait(&mut ns.scoped(), decl_id);
            let trait_decl = de_get_trait(*decl_id.inner_ref()).unwrap();
            let name = trait_decl.name;
            ns.insert_symbol(name, TyDeclaration::Trait(decl_id.clone()));
        }
        TyDeclaration::TraitImpl(decl_id) => {
            analyze_trait_impl(cc, &mut ns.scoped(), decl_id);
            let trait_impl = de_get_trait_impl(*decl_id.inner_ref()).unwrap();
            ns.insert_methods(
                trait_impl.type_implementing_for,
                trait_impl.trait_name.clone(),
                trait_impl.methods,
            );
        }
        TyDeclaration::Struct(decl_id) => {
            analyze_struct(&mut ns.scoped(), decl_id);
            let struct_decl = de_get_struct(*decl_id.inner_ref()).unwrap();
            let name = struct_decl.name;
            ns.insert_symbol(name, TyDeclaration::Struct(decl_id.clone()));
        }
    }
}

fn analyze_variable(
    cc: &CollectionContext,
    current_index: CollectionIndex,
    ns: &mut Namespace,
    var_decl: &mut TyVariableDeclaration,
) {
    // do type inference on the value
    analyze_expression(cc, current_index, ns, &mut var_decl.body);

    // resolve any custom types in the type ascription
    resolve_custom_types(var_decl.type_ascription, ns).unwrap();

    // unify the type of the value and the type ascription
    unify_types(var_decl.body.type_id, var_decl.type_ascription).unwrap();
}

fn analyze_function(
    cc: &CollectionContext,
    ns: &mut Namespace,
    decl_id: &mut CCIdx<DeclarationId>,
) {
    let mut func_decl = de_get_function(*decl_id.inner_ref()).unwrap();

    // import the trait constraints into the ns
    for type_param in func_decl.type_parameters.iter() {
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
    for param in func_decl.parameters.iter() {
        resolve_custom_types(param.type_id, ns).unwrap();
        ns.insert_symbol(param.name.clone(), param.into());
    }

    // do type inference on the function body
    let typed_body_return_type = analyze_code_block(cc, ns, &mut func_decl.body);

    // resolve any custom types in the function return type
    resolve_custom_types(func_decl.return_type, ns).unwrap();

    // unify the function return type and body return type
    unify_types(typed_body_return_type, func_decl.return_type).unwrap();
}

fn analyze_code_block(
    cc: &CollectionContext,
    ns: &mut Namespace,
    nodes: &mut [CCIdx<TyNode>],
) -> TypeId {
    for node in nodes.iter_mut() {
        analyze_node(cc, ns, node);
        if let TyNode::ReturnStatement(exp) = node.inner_ref() {
            return exp.type_id;
        }
    }
    insert_type(TypeInfo::Unit)
}

fn analyze_trait_impl(
    cc: &CollectionContext,
    ns: &mut Namespace,
    decl_id: &mut CCIdx<DeclarationId>,
) {
    let mut trait_impl = de_get_trait_impl(*decl_id.inner_ref()).unwrap();

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
    trait_impl
        .methods
        .iter_mut()
        .for_each(|method_id| analyze_function(cc, ns, method_id));
}

fn analyze_struct(ns: &mut Namespace, decl_id: &mut CCIdx<DeclarationId>) {
    let struct_decl = de_get_struct(*decl_id.inner_ref()).unwrap();

    // do type inference on the fields
    struct_decl.fields.iter().for_each(|field| {
        resolve_custom_types(field.type_id, ns).unwrap();
    });
}

fn analyze_trait(ns: &mut Namespace, decl_id: &mut CCIdx<DeclarationId>) {
    let mut trait_decl = de_get_trait(*decl_id.inner_ref()).unwrap();

    // do type inference on the interface
    trait_decl
        .interface_surface
        .iter_mut()
        .for_each(|trait_fn_id| analyze_trait_fn(ns, trait_fn_id));
}

fn analyze_trait_fn(ns: &mut Namespace, decl_id: &mut CCIdx<DeclarationId>) {
    let trait_fn = de_get_trait_fn(*decl_id.inner_ref()).unwrap();

    // resolve any custom types in the parameters
    for parameter in trait_fn.parameters.iter() {
        resolve_custom_types(parameter.type_id, ns).unwrap();
    }

    // resolve any custom types in the return type
    resolve_custom_types(trait_fn.return_type, ns).unwrap();
}
