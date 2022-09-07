use crate::{
    declaration_engine::{declaration_engine::*, declaration_wrapper::DeclarationWrapper},
    language::{
        partial::{
            partial_declaration::{PartialDeclaration, PartialFunctionDeclaration},
            PartialNode,
        },
        typed::{
            typed_declaration::{
                TypedDeclaration, TypedFunctionDeclaration, TypedTraitImpl,
                TypedVariableDeclaration,
            },
            TypedNode,
        },
        typing_context::function::TyFunctionContext,
        untyped::declaration::VariableDeclaration,
    },
    namespace::namespace::Namespace,
    type_system::{
        type_engine::{eval_type, insert_type, unify_types},
        type_id::TypeId,
        type_info::TypeInfo,
    },
};

use super::{analyze_expression, analyze_node};

pub(super) fn analyze_declaration(
    namespace: &mut Namespace,
    declaration: PartialDeclaration,
) -> TypedDeclaration {
    match declaration {
        PartialDeclaration::Variable(variable_declaration) => {
            let typed_variable_declaration = analyze_variable(namespace, variable_declaration);
            let name = typed_variable_declaration.name.clone();
            let decl = TypedDeclaration::Variable(typed_variable_declaration);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        PartialDeclaration::Function(decl_id) => {
            let function_declaration = de_get_function_partial(decl_id).unwrap();
            let typed_function_declaration =
                analyze_function(&mut namespace.scoped(), function_declaration.clone());
            let name = typed_function_declaration.name.clone();
            de_replace(
                decl_id,
                &DeclarationWrapper::Function(TyFunctionContext::partial(function_declaration)),
                DeclarationWrapper::Function(TyFunctionContext::typed(typed_function_declaration)),
            );
            let decl = TypedDeclaration::Function(decl_id);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        PartialDeclaration::Trait(decl_id) => {
            let typed_trait_declaration = de_get_trait(decl_id).unwrap();
            let name = typed_trait_declaration.name;
            let decl = TypedDeclaration::Trait(decl_id);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        PartialDeclaration::TraitImpl(decl_id) => {
            let trait_impl = de_get_trait_impl(decl_id).unwrap();
            let typed_trait_impl = analyze_trait_impl(&mut namespace.scoped(), trait_impl.clone());
            namespace.insert_methods(
                typed_trait_impl.type_implementing_for,
                typed_trait_impl.trait_name.clone(),
                typed_trait_impl.methods.clone(),
            );
            de_replace(
                decl_id,
                &DeclarationWrapper::TraitImpl(trait_impl),
                DeclarationWrapper::TraitImpl(typed_trait_impl),
            );
            TypedDeclaration::TraitImpl(decl_id)
        }
        PartialDeclaration::Struct(decl_id) => {
            let typed_struct_declaration = de_get_struct(decl_id).unwrap();
            let name = typed_struct_declaration.name;
            let decl = TypedDeclaration::Struct(decl_id);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        PartialDeclaration::GenericTypeForFunctionScope { .. } => {
            panic!("should not see this here")
        }
    }
}

fn analyze_variable(
    namespace: &mut Namespace,
    variable_declaration: VariableDeclaration,
) -> TypedVariableDeclaration {
    let new_body = analyze_expression(namespace, variable_declaration.body);
    let new_type_ascription =
        eval_type(insert_type(variable_declaration.type_ascription), namespace).unwrap();
    unify_types(new_body.type_id, new_type_ascription).unwrap();
    TypedVariableDeclaration {
        name: variable_declaration.name,
        body: new_body,
        type_ascription: new_type_ascription,
    }
}

fn analyze_function(
    namespace: &mut Namespace,
    function_declaration: PartialFunctionDeclaration,
) -> TypedFunctionDeclaration {
    // insert type params into namespace and handle trait constraints
    for type_parameter in function_declaration.type_parameters.iter() {
        let type_parameter_decl = TypedDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);

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

    // insert the typed function params into the namespace
    for param in function_declaration.parameters.iter() {
        namespace.insert_symbol(param.name.clone(), param.into());
    }

    // type check the function body
    let (typed_body, typed_body_return_type) =
        analyze_code_block(namespace, function_declaration.body);

    // unify the function return type and body return type
    unify_types(typed_body_return_type, function_declaration.return_type).unwrap();

    TypedFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: function_declaration.type_parameters,
        parameters: function_declaration.parameters,
        body: typed_body,
        return_type: function_declaration.return_type,
    }
}

fn analyze_code_block(
    namespace: &mut Namespace,
    nodes: Vec<PartialNode>,
) -> (Vec<TypedNode>, TypeId) {
    let mut typed_nodes = vec![];
    for node in nodes.into_iter() {
        let typed_node = analyze_node(namespace, node);
        let possibly_return = match &typed_node {
            TypedNode::ReturnStatement(exp) => Some(exp.type_id),
            _ => None,
        };
        typed_nodes.push(typed_node);
        if let Some(return_type) = possibly_return {
            return (typed_nodes, return_type);
        }
    }
    (typed_nodes, insert_type(TypeInfo::Unit))
}

fn analyze_trait_impl(namespace: &mut Namespace, trait_impl: TypedTraitImpl) -> TypedTraitImpl {
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

    // type check the methods
    let typed_method_ids = trait_impl
        .methods
        .into_iter()
        .map(|method_id| {
            let method = de_get_function_partial(method_id).unwrap();
            let typed_method = analyze_function(namespace, method.clone());
            de_replace(
                method_id,
                &DeclarationWrapper::Function(TyFunctionContext::partial(method)),
                DeclarationWrapper::Function(TyFunctionContext::typed(typed_method)),
            );
            method_id
        })
        .collect::<Vec<_>>();

    TypedTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for: trait_impl.type_implementing_for,
        type_parameters: trait_impl.type_parameters,
        methods: typed_method_ids,
    }
}
