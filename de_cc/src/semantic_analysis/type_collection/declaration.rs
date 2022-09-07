use crate::{
    declaration_engine::declaration_engine::{
        de_insert_function, de_insert_struct, de_insert_trait, de_insert_trait_fn,
        de_insert_trait_impl,
    },
    language::{
        typed::{
            typed_declaration::{
                TypedDeclaration, TypedFunctionDeclaration, TypedFunctionParameter,
                TypedStructDeclaration, TypedStructField, TypedTraitDeclaration, TypedTraitFn,
                TypedTraitImpl, TypedVariableDeclaration,
            },
            TypedNode,
        },
        untyped::{
            declaration::{
                Declaration, FunctionDeclaration, FunctionParameter, StructDeclaration,
                StructField, TraitDeclaration, TraitFn, TraitImpl, VariableDeclaration,
            },
            Node,
        },
    },
    namespace::namespace::Namespace,
    type_system::type_engine::{eval_type, insert_type},
};

use super::{expression::type_collect_expression, type_collect_node};

pub(super) fn type_collect_declaration(
    namespace: &mut Namespace,
    declaration: Declaration,
) -> TypedDeclaration {
    match declaration {
        Declaration::Variable(variable_declaration) => {
            let variable_declaration =
                type_collect_variable_declaration(namespace, variable_declaration);
            TypedDeclaration::Variable(variable_declaration)
        }
        Declaration::Function(function_declaration) => {
            let function_declaration =
                type_collect_function(&mut namespace.scoped(), function_declaration);
            TypedDeclaration::Function(de_insert_function(function_declaration))
        }
        Declaration::Trait(trait_declaration) => {
            let trait_declaration = type_collect_trait(&mut namespace.scoped(), trait_declaration);
            TypedDeclaration::Trait(de_insert_trait(trait_declaration))
        }
        Declaration::TraitImpl(trait_impl) => {
            let trait_impl = type_collect_trait_impl(&mut namespace.scoped(), trait_impl);
            TypedDeclaration::TraitImpl(de_insert_trait_impl(trait_impl))
        }
        Declaration::Struct(struct_declaration) => {
            let struct_declaration =
                type_collect_struct(&mut namespace.scoped(), struct_declaration);
            let name = struct_declaration.name.clone();
            let decl = TypedDeclaration::Struct(de_insert_struct(struct_declaration));
            namespace.insert_symbol(name, decl.clone());
            decl
        }
    }
}

fn type_collect_variable_declaration(
    namespace: &mut Namespace,
    variable_declaration: VariableDeclaration,
) -> TypedVariableDeclaration {
    let new_body = type_collect_expression(namespace, variable_declaration.body);
    let new_type_ascription =
        eval_type(insert_type(variable_declaration.type_ascription), namespace).unwrap();
    TypedVariableDeclaration {
        name: variable_declaration.name,
        body: new_body,
        type_ascription: new_type_ascription,
    }
}

fn type_collect_function(
    namespace: &mut Namespace,
    function_declaration: FunctionDeclaration,
) -> TypedFunctionDeclaration {
    for type_parameter in function_declaration.type_parameters.iter() {
        let type_parameter_decl = TypedDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);
    }
    let parameters = function_declaration
        .parameters
        .into_iter()
        .map(|param| type_collect_function_parameter(namespace, param))
        .collect::<Vec<_>>();
    let return_type = eval_type(insert_type(function_declaration.return_type), namespace).unwrap();
    TypedFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: function_declaration.type_parameters,
        parameters,
        body: type_collect_code_block(namespace, function_declaration.body),
        return_type,
    }
}

fn type_collect_function_parameter(
    namespace: &mut Namespace,
    function_parameter: FunctionParameter,
) -> TypedFunctionParameter {
    TypedFunctionParameter {
        name: function_parameter.name,
        type_id: eval_type(insert_type(function_parameter.type_info), namespace).unwrap(),
    }
}

fn type_collect_code_block(namespace: &mut Namespace, nodes: Vec<Node>) -> Vec<TypedNode> {
    nodes
        .into_iter()
        .map(|node| type_collect_node(namespace, node))
        .collect()
}

fn type_collect_trait(
    namespace: &mut Namespace,
    trait_declaration: TraitDeclaration,
) -> TypedTraitDeclaration {
    let interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| {
            let trait_fn = type_collect_trait_fn(namespace, trait_fn);
            de_insert_trait_fn(trait_fn)
        })
        .collect::<Vec<_>>();
    TypedTraitDeclaration {
        name: trait_declaration.name,
        interface_surface,
    }
}

fn type_collect_trait_fn(namespace: &mut Namespace, trait_fn: TraitFn) -> TypedTraitFn {
    let parameters = trait_fn
        .parameters
        .into_iter()
        .map(|param| type_collect_function_parameter(namespace, param))
        .collect::<Vec<_>>();
    let return_type = eval_type(insert_type(trait_fn.return_type), namespace).unwrap();
    TypedTraitFn {
        name: trait_fn.name,
        parameters,
        return_type,
    }
}

fn type_collect_trait_impl(namespace: &mut Namespace, trait_impl: TraitImpl) -> TypedTraitImpl {
    if !trait_impl.type_parameters.is_empty() {
        panic!()
    }
    let methods = trait_impl
        .methods
        .into_iter()
        .map(|method| de_insert_function(type_collect_function(namespace, method)))
        .collect::<Vec<_>>();
    let type_implementing_for =
        eval_type(insert_type(trait_impl.type_implementing_for), namespace).unwrap();
    TypedTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for,
        type_parameters: vec![],
        methods,
    }
}

fn type_collect_struct(
    namespace: &mut Namespace,
    struct_declaration: StructDeclaration,
) -> TypedStructDeclaration {
    for type_parameter in struct_declaration.type_parameters.iter() {
        let type_parameter_decl = TypedDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);
    }
    let fields = struct_declaration
        .fields
        .into_iter()
        .map(|field| type_collect_struct_field(namespace, field))
        .collect::<Vec<_>>();
    TypedStructDeclaration {
        name: struct_declaration.name,
        type_parameters: struct_declaration.type_parameters,
        fields,
    }
}

fn type_collect_struct_field(
    namespace: &mut Namespace,
    struct_field: StructField,
) -> TypedStructField {
    let type_id = eval_type(insert_type(struct_field.type_info), namespace).unwrap();
    TypedStructField {
        name: struct_field.name,
        type_id,
    }
}
