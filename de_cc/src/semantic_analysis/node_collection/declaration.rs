use crate::{
    collection_context::collection_context::CollectionContext,
    declaration_engine::declaration_engine::{
        de_insert_function, de_insert_struct, de_insert_trait, de_insert_trait_fn,
        de_insert_trait_impl,
    },
    language::{
        parsed::declaration::{
            Declaration, FunctionDeclaration, FunctionParameter, StructDeclaration, StructField,
            TraitDeclaration, TraitFn, TraitImpl, VariableDeclaration,
        },
        ty::typed_declaration::{
            TyDeclaration, TyFunctionDeclaration, TyFunctionParameter, TyStructDeclaration,
            TyStructField, TyTraitDeclaration, TyTraitFn, TyTraitImpl, TyVariableDeclaration,
        },
    },
    type_system::{
        type_engine::insert_type,
        type_mapping::{insert_type_parameters, TypeMapping},
    },
    types::copy_types::CopyTypes,
};

use super::{collect_nodes_nodes, expression::collect_nodes_expression};

pub(super) fn collect_nodes_declaration(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    declaration: Declaration,
) -> TyDeclaration {
    match declaration {
        Declaration::Variable(variable_declaration) => {
            let variable_declaration = collect_nodes_variable_declaration(
                collection_ctxt,
                type_mapping,
                variable_declaration,
            );
            TyDeclaration::Variable(variable_declaration)
        }
        Declaration::Function(function_declaration) => {
            let function_declaration =
                collect_nodes_function(collection_ctxt, type_mapping, function_declaration);
            TyDeclaration::Function(de_insert_function(function_declaration))
        }
        Declaration::Trait(trait_declaration) => {
            let trait_declaration =
                collect_nodes_trait(collection_ctxt, type_mapping, trait_declaration);
            TyDeclaration::Trait(de_insert_trait(trait_declaration))
        }
        Declaration::TraitImpl(trait_impl) => {
            let trait_impl = collect_nodes_trait_impl(collection_ctxt, type_mapping, trait_impl);
            TyDeclaration::TraitImpl(de_insert_trait_impl(trait_impl))
        }
        Declaration::Struct(struct_declaration) => {
            let struct_declaration =
                collect_nodes_struct(collection_ctxt, type_mapping, struct_declaration);
            TyDeclaration::Struct(de_insert_struct(struct_declaration))
        }
    }
}

fn collect_nodes_variable_declaration(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    variable_declaration: VariableDeclaration,
) -> TyVariableDeclaration {
    let new_body = collect_nodes_expression(type_mapping, variable_declaration.body);
    let mut new_type_ascription = insert_type(variable_declaration.type_ascription);
    new_type_ascription.copy_types(type_mapping);
    TyVariableDeclaration {
        name: variable_declaration.name,
        body: new_body,
        type_ascription: new_type_ascription,
    }
}

fn collect_nodes_function(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    function_declaration: FunctionDeclaration,
) -> TyFunctionDeclaration {
    let mut type_mapping = type_mapping.clone();
    type_mapping.extend(insert_type_parameters(
        &function_declaration.type_parameters,
    ));
    let parameters = function_declaration
        .parameters
        .into_iter()
        .map(|param| collect_nodes_function_parameter(collection_ctxt, &type_mapping, param))
        .collect::<Vec<_>>();
    let mut return_type = insert_type(function_declaration.return_type);
    return_type.copy_types(&type_mapping);
    TyFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: function_declaration.type_parameters,
        parameters,
        body: collect_nodes_nodes(collection_ctxt, function_declaration.body),
        return_type,
    }
}

fn collect_nodes_function_parameter(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    function_parameter: FunctionParameter,
) -> TyFunctionParameter {
    let mut type_id = insert_type(function_parameter.type_info);
    type_id.copy_types(type_mapping);
    TyFunctionParameter {
        name: function_parameter.name,
        type_id,
    }
}

fn collect_nodes_trait(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    trait_declaration: TraitDeclaration,
) -> TyTraitDeclaration {
    let interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| {
            let trait_fn = collect_nodes_trait_fn(collection_ctxt, type_mapping, trait_fn);
            de_insert_trait_fn(trait_fn)
        })
        .collect::<Vec<_>>();
    TyTraitDeclaration {
        name: trait_declaration.name,
        interface_surface,
    }
}

fn collect_nodes_trait_fn(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    trait_fn: TraitFn,
) -> TyTraitFn {
    let parameters = trait_fn
        .parameters
        .into_iter()
        .map(|param| collect_nodes_function_parameter(collection_ctxt, type_mapping, param))
        .collect::<Vec<_>>();
    let mut return_type = insert_type(trait_fn.return_type);
    return_type.copy_types(type_mapping);
    TyTraitFn {
        name: trait_fn.name,
        parameters,
        return_type,
    }
}

fn collect_nodes_trait_impl(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    trait_impl: TraitImpl,
) -> TyTraitImpl {
    if !trait_impl.type_parameters.is_empty() {
        panic!()
    }
    let methods = trait_impl
        .methods
        .into_iter()
        .map(|method| {
            de_insert_function(collect_nodes_function(
                collection_ctxt,
                type_mapping,
                method,
            ))
        })
        .collect::<Vec<_>>();
    let mut type_implementing_for = insert_type(trait_impl.type_implementing_for);
    type_implementing_for.copy_types(type_mapping);
    TyTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for,
        type_parameters: vec![],
        methods,
    }
}

fn collect_nodes_struct(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    struct_declaration: StructDeclaration,
) -> TyStructDeclaration {
    let mut type_mapping = type_mapping.clone();
    type_mapping.extend(insert_type_parameters(&struct_declaration.type_parameters));
    let fields = struct_declaration
        .fields
        .into_iter()
        .map(|field| collect_nodes_struct_field(collection_ctxt, &type_mapping, field))
        .collect::<Vec<_>>();
    TyStructDeclaration {
        name: struct_declaration.name,
        type_parameters: struct_declaration.type_parameters,
        fields,
    }
}

fn collect_nodes_struct_field(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    struct_field: StructField,
) -> TyStructField {
    let mut type_id = insert_type(struct_field.type_info);
    type_id.copy_types(type_mapping);
    TyStructField {
        name: struct_field.name,
        type_id,
    }
}
