use crate::{
    declaration_engine::declaration_engine::*,
    language::ty::{
        typed_declaration::{
            TyDeclaration, TyFunctionDeclaration, TyTraitImpl, TyVariableDeclaration,
        },
        TyNode,
    },
    namespace::namespace::Namespace,
    type_system::{
        type_engine::{insert_type, unify_types},
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
            let typed_trait_declaration = de_get_trait(*decl_id).unwrap();
            let name = typed_trait_declaration.name;
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
            let typed_struct_declaration = de_get_struct(*decl_id).unwrap();
            let name = typed_struct_declaration.name;
            namespace.insert_symbol(name, decl.clone());
        }
    }
}

fn analyze_variable(namespace: &mut Namespace, variable_declaration: &TyVariableDeclaration) {
    analyze_expression(namespace, &variable_declaration.body);
    unify_types(
        variable_declaration.body.type_id,
        variable_declaration.type_ascription,
    )
    .unwrap();
}

fn analyze_function(namespace: &mut Namespace, function_declaration: &TyFunctionDeclaration) {
    // do type inference on the function body
    let typed_body_return_type = analyze_code_block(namespace, &function_declaration.body);

    // unify the function return type and body return type
    unify_types(typed_body_return_type, function_declaration.return_type).unwrap();
}

fn analyze_code_block(namespace: &mut Namespace, nodes: &[TyNode]) -> TypeId {
    for node in nodes.iter() {
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

    // TODO: check to see if all of the methods are implementing, no new methods implementing,
    // when generic traits are implemented add the monomorphized copies to the declaration
    // engine

    // do type inference on the methods
    trait_impl.methods.iter().for_each(|method_id| {
        let method = de_get_function(*method_id).unwrap();
        analyze_function(namespace, &method);
    });
}
