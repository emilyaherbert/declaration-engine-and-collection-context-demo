use either::Either;

use crate::{
    declaration_engine::{
        declaration_engine::{
            de_insert, de_insert_struct, de_insert_trait, de_insert_trait_fn, de_insert_trait_impl,
        },
        declaration_wrapper::DeclarationWrapper,
    },
    language::{
        semi::{
            semi_declaration::{SemiDeclaration, SemiTypedFunctionDeclaration},
            SemiNode,
        },
        typed::typed_declaration::{
            TypedFunctionParameter, TypedStructDeclaration, TypedStructField,
            TypedTraitDeclaration, TypedTraitFn, TypedTraitImpl,
        },
        untyped::{
            declaration::{
                Declaration, FunctionDeclaration, FunctionParameter, StructDeclaration,
                StructField, TraitDeclaration, TraitFn, TraitImpl,
            },
            Node,
        },
    },
    type_system::type_engine::insert_type,
};

use super::collect_types_node;

pub(super) fn collect_types_declaration(declaration: Declaration) -> SemiDeclaration {
    match declaration {
        Declaration::Variable(decl) => SemiDeclaration::Variable(decl),
        Declaration::Function(function_declaration) => {
            let function_declaration = collect_types_function(function_declaration);
            let decl_id = de_insert(DeclarationWrapper::Function(Either::Left(
                function_declaration,
            )));
            SemiDeclaration::Function(decl_id)
        }
        Declaration::Trait(trait_declaration) => {
            let trait_declaration = collect_types_trait(trait_declaration);
            let decl_id = de_insert_trait(trait_declaration);
            SemiDeclaration::Trait(decl_id)
        }
        Declaration::TraitImpl(trait_impl) => {
            let trait_impl = collect_types_trait_impl(trait_impl);
            let decl_id = de_insert_trait_impl(trait_impl);
            SemiDeclaration::TraitImpl(decl_id)
        }
        Declaration::Struct(struct_declaration) => {
            let struct_declaration = collect_types_struct(struct_declaration);
            let decl_id = de_insert_struct(struct_declaration);
            SemiDeclaration::Struct(decl_id)
        }
    }
}

fn collect_types_function(
    function_declaration: FunctionDeclaration,
) -> SemiTypedFunctionDeclaration {
    if !function_declaration.type_parameters.is_empty() {
        panic!()
    }

    // type check the function params
    let parameters = function_declaration
        .parameters
        .into_iter()
        .map(collect_types_function_parameter)
        .collect::<Vec<_>>();

    // type check the function return type
    let return_type = insert_type(function_declaration.return_type);

    SemiTypedFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: vec![],
        parameters,
        body: collect_types_code_block(function_declaration.body),
        return_type,
    }
}

fn collect_types_function_parameter(
    function_parameter: FunctionParameter,
) -> TypedFunctionParameter {
    TypedFunctionParameter {
        name: function_parameter.name,
        type_id: insert_type(function_parameter.type_info),
    }
}

fn collect_types_code_block(nodes: Vec<Node>) -> Vec<SemiNode> {
    nodes.into_iter().map(collect_types_node).collect()
}

fn collect_types_trait(trait_declaration: TraitDeclaration) -> TypedTraitDeclaration {
    let interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| {
            let trait_fn = collect_types_trait_fn(trait_fn);
            de_insert_trait_fn(trait_fn)
        })
        .collect::<Vec<_>>();
    TypedTraitDeclaration {
        name: trait_declaration.name,
        interface_surface,
    }
}

fn collect_types_trait_fn(trait_fn: TraitFn) -> TypedTraitFn {
    let parameters = trait_fn
        .parameters
        .into_iter()
        .map(collect_types_function_parameter)
        .collect::<Vec<_>>();
    TypedTraitFn {
        name: trait_fn.name,
        parameters,
        return_type: insert_type(trait_fn.return_type),
    }
}

fn collect_types_trait_impl(trait_impl: TraitImpl) -> TypedTraitImpl {
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
            let method = collect_types_function(method);
            de_insert(DeclarationWrapper::Function(Either::Left(method)))
        })
        .collect::<Vec<_>>();

    TypedTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for: insert_type(trait_impl.type_implementing_for),
        type_parameters: vec![],
        methods,
    }
}

fn collect_types_struct(struct_declaration: StructDeclaration) -> TypedStructDeclaration {
    if !struct_declaration.type_parameters.is_empty() {
        panic!()
    }

    // type check the fields
    let fields = struct_declaration
        .fields
        .into_iter()
        .map(collect_types_struct_field)
        .collect::<Vec<_>>();

    TypedStructDeclaration {
        name: struct_declaration.name,
        type_parameters: vec![],
        fields,
    }
}

fn collect_types_struct_field(struct_field: StructField) -> TypedStructField {
    TypedStructField {
        name: struct_field.name,
        type_id: insert_type(struct_field.type_info),
    }
}
