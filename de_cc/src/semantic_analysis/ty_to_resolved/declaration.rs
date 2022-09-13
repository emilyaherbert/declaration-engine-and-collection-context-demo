use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_index::CollectionIndex,
    },
    declaration_engine::{declaration_engine::*, declaration_id::DeclarationId},
    language::{
        resolved::resolved_declaration::{
            ResolvedDeclaration, ResolvedFunctionDeclaration, ResolvedFunctionParameter,
            ResolvedStructDeclaration, ResolvedStructField, ResolvedTraitDeclaration,
            ResolvedTraitFn, ResolvedTraitImpl, ResolvedVariableDeclaration,
        },
        ty::typed_declaration::{
            TyDeclaration, TyFunctionDeclaration, TyFunctionParameter, TyStructDeclaration,
            TyStructField, TyVariableDeclaration,
        },
    },
    type_system::{
        resolved_types::ResolvedTypeParameter, type_engine::resolve_type,
        type_parameter::TypeParameter,
    },
};

use super::{expression::to_resolved_expression, to_resolved_nodes};

pub(super) fn to_resolved_declaration(
    cc: &CollectionContext,
    node_index: &CollectionIndex,
) -> Vec<ResolvedDeclaration> {
    let declaration = cc.get_node(*node_index).expect_declaration().unwrap();
    match declaration {
        TyDeclaration::Variable(variable_declaration) => {
            let variable_declaration = to_resolved_variable_declaration(variable_declaration);
            vec![ResolvedDeclaration::Variable(variable_declaration)]
        }
        TyDeclaration::Function(id) => {
            let function_declarations = to_resolved_function_declaration(cc, id);
            function_declarations
                .into_iter()
                .map(ResolvedDeclaration::Function)
                .collect()
        }
        TyDeclaration::Trait(id) => {
            let trait_declaration = to_resolved_trait_declaration(id);
            vec![ResolvedDeclaration::Trait(trait_declaration)]
        }
        TyDeclaration::TraitImpl(id) => {
            let trait_impl = to_resolved_trait_impl(cc, id);
            vec![ResolvedDeclaration::TraitImpl(trait_impl)]
        }
        TyDeclaration::Struct(id) => {
            let struct_declarations = to_resolved_struct_declaration(id);
            struct_declarations
                .into_iter()
                .map(ResolvedDeclaration::Struct)
                .collect()
        }
    }
}

fn to_resolved_variable_declaration(
    variable_declaration: &TyVariableDeclaration,
) -> ResolvedVariableDeclaration {
    let type_ascription = resolve_type(variable_declaration.type_ascription).unwrap();
    let body = to_resolved_expression(&variable_declaration.body);
    ResolvedVariableDeclaration {
        name: variable_declaration.name.to_string(),
        type_ascription,
        body,
    }
}

fn to_resolved_function_declaration(
    cc: &CollectionContext,
    function_id: &DeclarationId,
) -> Vec<ResolvedFunctionDeclaration> {
    let original_copy = de_get_function(*function_id).unwrap();
    if original_copy.type_parameters.is_empty() {
        to_resolved_function_declaration_inner(cc, vec![original_copy])
    } else {
        let monomorphized_copies = de_get_monomorphized_function_copies(*function_id).unwrap();
        to_resolved_function_declaration_inner(cc, monomorphized_copies)
    }
}

fn to_resolved_function_declaration_inner(
    cc: &CollectionContext,
    function_declarations: Vec<TyFunctionDeclaration>,
) -> Vec<ResolvedFunctionDeclaration> {
    function_declarations
        .into_iter()
        .map(|function_declaration| {
            let resolved_type_parameters = function_declaration
                .type_parameters
                .into_iter()
                .map(resolve_type_parameter)
                .collect::<Vec<_>>();
            let resolved_parameters = function_declaration
                .parameters
                .into_iter()
                .map(to_resolved_function_parameter)
                .collect::<Vec<_>>();
            let resolved_body = to_resolved_nodes(cc, &function_declaration.body);
            let resolved_type = resolve_type(function_declaration.return_type).unwrap();
            ResolvedFunctionDeclaration {
                name: function_declaration.name,
                type_parameters: resolved_type_parameters,
                parameters: resolved_parameters,
                body: resolved_body,
                return_type: resolved_type,
            }
        })
        .collect()
}

fn resolve_type_parameter(type_parameter: TypeParameter) -> ResolvedTypeParameter {
    ResolvedTypeParameter {
        type_info: resolve_type(type_parameter.type_id).unwrap(),
    }
}

fn to_resolved_function_parameter(
    function_parameter: TyFunctionParameter,
) -> ResolvedFunctionParameter {
    ResolvedFunctionParameter {
        name: function_parameter.name,
        type_info: resolve_type(function_parameter.type_id).unwrap(),
    }
}

fn to_resolved_trait_declaration(trait_id: &DeclarationId) -> ResolvedTraitDeclaration {
    let trait_decl = de_get_trait(*trait_id).unwrap();
    let new_interface_surface = trait_decl
        .interface_surface
        .iter()
        .map(to_resolved_trait_fn)
        .collect::<Vec<_>>();
    ResolvedTraitDeclaration {
        name: trait_decl.name,
        interface_surface: new_interface_surface,
    }
}

fn to_resolved_trait_fn(trait_fn_id: &DeclarationId) -> ResolvedTraitFn {
    let trait_fn = de_get_trait_fn(*trait_fn_id).unwrap();
    let resolved_parameters = trait_fn
        .parameters
        .into_iter()
        .map(to_resolved_function_parameter)
        .collect::<Vec<_>>();
    let resolved_type = resolve_type(trait_fn.return_type).unwrap();
    ResolvedTraitFn {
        name: trait_fn.name,
        parameters: resolved_parameters,
        return_type: resolved_type,
    }
}

fn to_resolved_trait_impl(cc: &CollectionContext, impl_id: &DeclarationId) -> ResolvedTraitImpl {
    let trait_impl = de_get_trait_impl(*impl_id).unwrap();
    let type_implementing_for = resolve_type(trait_impl.type_implementing_for).unwrap();
    let methods = trait_impl
        .methods
        .iter()
        .flat_map(|method| to_resolved_function_declaration(cc, method))
        .collect::<Vec<_>>();
    ResolvedTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for,
        methods,
    }
}

fn to_resolved_struct_declaration(struct_id: &DeclarationId) -> Vec<ResolvedStructDeclaration> {
    let original_copy = de_get_struct(*struct_id).unwrap();
    if original_copy.type_parameters.is_empty() {
        to_resolved_struct_declaration_inner(vec![original_copy])
    } else {
        let monomorphized_copies = de_get_monomorphized_struct_copies(*struct_id).unwrap();
        to_resolved_struct_declaration_inner(monomorphized_copies)
    }
}

fn to_resolved_struct_declaration_inner(
    struct_declarations: Vec<TyStructDeclaration>,
) -> Vec<ResolvedStructDeclaration> {
    struct_declarations
        .into_iter()
        .map(|struct_declaration| {
            let resolved_type_parameters = struct_declaration
                .type_parameters
                .into_iter()
                .map(resolve_type_parameter)
                .collect::<Vec<_>>();
            let resolved_fields = struct_declaration
                .fields
                .into_iter()
                .map(to_resolved_struct_field)
                .collect::<Vec<_>>();
            ResolvedStructDeclaration {
                name: struct_declaration.name,
                type_parameters: resolved_type_parameters,
                fields: resolved_fields,
            }
        })
        .collect()
}

fn to_resolved_struct_field(field: TyStructField) -> ResolvedStructField {
    ResolvedStructField {
        name: field.name,
        type_info: resolve_type(field.type_id).unwrap(),
    }
}
