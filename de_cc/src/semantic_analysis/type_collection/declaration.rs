use crate::{
    declaration_engine::{
        declaration_engine::{
            de_insert, de_insert_struct, de_insert_trait, de_insert_trait_fn, de_insert_trait_impl,
        },
        declaration_wrapper::DeclarationWrapper,
    },
    language::{
        partial::{
            partial_declaration::{PartialDeclaration, PartialFunctionDeclaration},
            PartialNode,
        },
        typed::typed_declaration::{
            TypedFunctionParameter, TypedStructDeclaration, TypedStructField,
            TypedTraitDeclaration, TypedTraitFn, TypedTraitImpl,
        },
        typing_context::function::TyFunctionContext,
        untyped::{
            declaration::{
                Declaration, FunctionDeclaration, FunctionParameter, StructDeclaration,
                StructField, TraitDeclaration, TraitFn, TraitImpl,
            },
            Node,
        },
    },
    namespace::collection_namespace::CollectionNamespace,
    type_system::type_engine::{eval_type2, insert_type},
};

use super::collect_types_node;

pub(super) fn collect_types_declaration(
    namespace: &mut CollectionNamespace,
    declaration: Declaration,
) -> PartialDeclaration {
    match declaration {
        Declaration::Variable(decl) => PartialDeclaration::Variable(decl),
        Declaration::Function(function_declaration) => {
            let function_declaration =
                collect_types_function(&mut namespace.scoped(), function_declaration);
            let decl_id = de_insert(DeclarationWrapper::Function(TyFunctionContext::partial(
                function_declaration,
            )));
            PartialDeclaration::Function(decl_id)
        }
        Declaration::Trait(trait_declaration) => {
            let trait_declaration = collect_types_trait(&mut namespace.scoped(), trait_declaration);
            let decl_id = de_insert_trait(trait_declaration);
            PartialDeclaration::Trait(decl_id)
        }
        Declaration::TraitImpl(trait_impl) => {
            let trait_impl = collect_types_trait_impl(&mut namespace.scoped(), trait_impl);
            PartialDeclaration::TraitImpl(de_insert_trait_impl(trait_impl))
        }
        Declaration::Struct(struct_declaration) => {
            let struct_declaration =
                collect_types_struct(&mut namespace.scoped(), struct_declaration);
            let name = struct_declaration.name.clone();
            let decl = PartialDeclaration::Struct(de_insert_struct(struct_declaration));
            namespace.insert_symbol(name, decl.clone());
            decl
        }
    }
}

fn collect_types_function(
    namespace: &mut CollectionNamespace,
    function_declaration: FunctionDeclaration,
) -> PartialFunctionDeclaration {
    // insert type params into namespace
    for type_parameter in function_declaration.type_parameters.iter() {
        let type_parameter_decl = PartialDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);
    }

    // type check the function params
    let parameters = function_declaration
        .parameters
        .into_iter()
        .map(|param| collect_types_function_parameter(namespace, param))
        .collect::<Vec<_>>();

    // type check the function return type
    let return_type = eval_type2(insert_type(function_declaration.return_type), namespace).unwrap();

    PartialFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: function_declaration.type_parameters,
        parameters,
        body: collect_types_code_block(namespace, function_declaration.body),
        return_type,
    }
}

fn collect_types_function_parameter(
    namespace: &mut CollectionNamespace,
    function_parameter: FunctionParameter,
) -> TypedFunctionParameter {
    TypedFunctionParameter {
        name: function_parameter.name,
        type_id: eval_type2(insert_type(function_parameter.type_info), namespace).unwrap(),
    }
}

fn collect_types_code_block(
    namespace: &mut CollectionNamespace,
    nodes: Vec<Node>,
) -> Vec<PartialNode> {
    nodes
        .into_iter()
        .map(|node| collect_types_node(namespace, node))
        .collect()
}

fn collect_types_trait(
    namespace: &mut CollectionNamespace,
    trait_declaration: TraitDeclaration,
) -> TypedTraitDeclaration {
    let interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| {
            let trait_fn = collect_types_trait_fn(namespace, trait_fn);
            de_insert_trait_fn(trait_fn)
        })
        .collect::<Vec<_>>();
    TypedTraitDeclaration {
        name: trait_declaration.name,
        interface_surface,
    }
}

fn collect_types_trait_fn(namespace: &mut CollectionNamespace, trait_fn: TraitFn) -> TypedTraitFn {
    let parameters = trait_fn
        .parameters
        .into_iter()
        .map(|param| collect_types_function_parameter(namespace, param))
        .collect::<Vec<_>>();
    TypedTraitFn {
        name: trait_fn.name,
        parameters,
        return_type: eval_type2(insert_type(trait_fn.return_type), namespace).unwrap(),
    }
}

fn collect_types_trait_impl(
    namespace: &mut CollectionNamespace,
    trait_impl: TraitImpl,
) -> TypedTraitImpl {
    if !trait_impl.type_parameters.is_empty() {
        panic!()
    }

    // TODO: get the trait from the declaration engine,
    // check to see if all of the methods are implementing, no new methods implementing,
    // when generic traits are implemented add the monomorphized copies to the declaration
    // engine

    // type check the methods
    let methods = trait_impl
        .methods
        .into_iter()
        .map(|method| {
            let method = collect_types_function(namespace, method);
            de_insert(DeclarationWrapper::Function(TyFunctionContext::partial(
                method,
            )))
        })
        .collect::<Vec<_>>();

    TypedTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for: eval_type2(insert_type(trait_impl.type_implementing_for), namespace)
            .unwrap(),
        type_parameters: vec![],
        methods,
    }
}

fn collect_types_struct(
    namespace: &mut CollectionNamespace,
    struct_declaration: StructDeclaration,
) -> TypedStructDeclaration {
    // insert type params into namespace
    for type_parameter in struct_declaration.type_parameters.iter() {
        let type_parameter_decl = PartialDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);
    }

    // type check the fields
    let fields = struct_declaration
        .fields
        .into_iter()
        .map(|field| collect_types_struct_field(namespace, field))
        .collect::<Vec<_>>();

    TypedStructDeclaration {
        name: struct_declaration.name,
        type_parameters: struct_declaration.type_parameters,
        fields,
    }
}

fn collect_types_struct_field(
    namespace: &mut CollectionNamespace,
    struct_field: StructField,
) -> TypedStructField {
    TypedStructField {
        name: struct_field.name,
        type_id: eval_type2(insert_type(struct_field.type_info), namespace).unwrap(),
    }
}
