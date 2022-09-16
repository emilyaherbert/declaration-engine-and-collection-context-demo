use crate::{
    collection_context::{collection_context::CollectionContext, collection_index::CCIdx},
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
    decl: CCIdx<TyDeclaration>,
) -> Vec<ResolvedDeclaration> {
    match decl.inner() {
        TyDeclaration::Variable(var_decl) => {
            let var_decl = to_resolved_variable_declaration(var_decl);
            vec![ResolvedDeclaration::Variable(var_decl)]
        }
        TyDeclaration::Function(decl_id) => {
            let func_decls = to_resolved_function_declaration(cc, decl_id);
            func_decls
                .into_iter()
                .map(ResolvedDeclaration::Function)
                .collect()
        }
        TyDeclaration::Trait(decl_id) => {
            let trait_decl = to_resolved_trait_declaration(decl_id);
            vec![ResolvedDeclaration::Trait(trait_decl)]
        }
        TyDeclaration::TraitImpl(decl_id) => {
            let trait_impl = to_resolved_trait_impl(cc, decl_id);
            vec![ResolvedDeclaration::TraitImpl(trait_impl)]
        }
        TyDeclaration::Struct(decl_id) => {
            let struct_decls = to_resolved_struct_declaration(decl_id);
            struct_decls
                .into_iter()
                .map(ResolvedDeclaration::Struct)
                .collect()
        }
    }
}

fn to_resolved_variable_declaration(
    variable_declaration: TyVariableDeclaration,
) -> ResolvedVariableDeclaration {
    let type_ascription = resolve_type(variable_declaration.type_ascription).unwrap();
    let body = to_resolved_expression(variable_declaration.body);
    ResolvedVariableDeclaration {
        name: variable_declaration.name,
        type_ascription,
        body,
    }
}

fn to_resolved_function_declaration(
    cc: &CollectionContext,
    function_id: CCIdx<DeclarationId>,
) -> Vec<ResolvedFunctionDeclaration> {
    let function_id = function_id.inner();
    let original_copy = de_get_function(function_id).unwrap();
    if original_copy.type_parameters.is_empty() {
        to_resolved_function_declaration_inner(cc, vec![original_copy])
    } else {
        let monomorphized_copies = de_get_monomorphized_function_copies(function_id).unwrap();
        to_resolved_function_declaration_inner(cc, monomorphized_copies)
    }
}

fn to_resolved_function_declaration_inner(
    cc: &CollectionContext,
    func_decls: Vec<TyFunctionDeclaration>,
) -> Vec<ResolvedFunctionDeclaration> {
    let mut new_func_decls = vec![];
    for func_decl in func_decls.into_iter() {
        let resolved_type_parameters = func_decl
            .type_parameters
            .into_iter()
            .map(resolve_type_parameter)
            .collect::<Result<_, _>>();
        let resolved_type_parameters = match resolved_type_parameters {
            Ok(resolved_type_parameters) => resolved_type_parameters,
            Err(_) => {
                // hack to prevent the ugly results from displaying on the screen
                println!("omitting a function");
                continue;
            }
        };
        let resolved_parameters = func_decl
            .parameters
            .into_iter()
            .map(to_resolved_function_parameter)
            .collect::<Vec<_>>();
        let resolved_body = to_resolved_nodes(cc, func_decl.body);
        let resolved_type = resolve_type(func_decl.return_type).unwrap();
        let func_decl = ResolvedFunctionDeclaration {
            name: func_decl.name,
            type_parameters: resolved_type_parameters,
            parameters: resolved_parameters,
            body: resolved_body,
            return_type: resolved_type,
        };
        new_func_decls.push(func_decl);
    }
    new_func_decls
}

fn resolve_type_parameter(type_parameter: TypeParameter) -> Result<ResolvedTypeParameter, String> {
    let type_param = ResolvedTypeParameter {
        type_info: resolve_type(type_parameter.type_id)?,
    };
    Ok(type_param)
}

fn to_resolved_function_parameter(
    function_parameter: TyFunctionParameter,
) -> ResolvedFunctionParameter {
    ResolvedFunctionParameter {
        name: function_parameter.name,
        type_info: resolve_type(function_parameter.type_id).unwrap(),
    }
}

fn to_resolved_trait_declaration(trait_id: CCIdx<DeclarationId>) -> ResolvedTraitDeclaration {
    let trait_id = trait_id.inner();
    let trait_decl = de_get_trait(trait_id).unwrap();
    let new_interface_surface = trait_decl
        .interface_surface
        .into_iter()
        .map(to_resolved_trait_fn)
        .collect::<Vec<_>>();
    ResolvedTraitDeclaration {
        name: trait_decl.name,
        interface_surface: new_interface_surface,
    }
}

fn to_resolved_trait_fn(trait_fn_id: CCIdx<DeclarationId>) -> ResolvedTraitFn {
    let trait_fn_id = trait_fn_id.inner();
    let trait_fn = de_get_trait_fn(trait_fn_id).unwrap();
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

fn to_resolved_trait_impl(
    cc: &CollectionContext,
    impl_id: CCIdx<DeclarationId>,
) -> ResolvedTraitImpl {
    let impl_id = impl_id.inner();
    let trait_impl = de_get_trait_impl(impl_id).unwrap();
    let type_implementing_for = resolve_type(trait_impl.type_implementing_for).unwrap();
    let methods = trait_impl
        .methods
        .into_iter()
        .flat_map(|method| to_resolved_function_declaration(cc, method))
        .collect::<Vec<_>>();
    ResolvedTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for,
        methods,
    }
}

fn to_resolved_struct_declaration(
    struct_id: CCIdx<DeclarationId>,
) -> Vec<ResolvedStructDeclaration> {
    let struct_id = struct_id.inner();
    let original_copy = de_get_struct(struct_id).unwrap();
    if original_copy.type_parameters.is_empty() {
        to_resolved_struct_declaration_inner(vec![original_copy])
    } else {
        let monomorphized_copies = de_get_monomorphized_struct_copies(struct_id).unwrap();
        to_resolved_struct_declaration_inner(monomorphized_copies)
    }
}

fn to_resolved_struct_declaration_inner(
    struct_decls: Vec<TyStructDeclaration>,
) -> Vec<ResolvedStructDeclaration> {
    let mut new_struct_decls = vec![];
    for struct_decl in struct_decls.into_iter() {
        let resolved_type_parameters = struct_decl
            .type_parameters
            .into_iter()
            .map(resolve_type_parameter)
            .collect::<Result<_, _>>();
        let resolved_type_parameters = match resolved_type_parameters {
            Ok(resolved_type_parameters) => resolved_type_parameters,
            Err(_) => {
                // hack to prevent the ugly results from displaying on the screen
                println!("omitting a struct");
                continue;
            }
        };
        let resolved_fields = struct_decl
            .fields
            .into_iter()
            .map(to_resolved_struct_field)
            .collect::<Result<_, _>>();
        let resolved_fields = match resolved_fields {
            Ok(resolved_fields) => resolved_fields,
            Err(_) => {
                // hack to prevent the ugly results from displaying on the screen
                println!("omitting a struct");
                continue;
            }
        };
        let struct_decl = ResolvedStructDeclaration {
            name: struct_decl.name,
            type_parameters: resolved_type_parameters,
            fields: resolved_fields,
        };
        new_struct_decls.push(struct_decl);
    }
    new_struct_decls
}

fn to_resolved_struct_field(field: TyStructField) -> Result<ResolvedStructField, String> {
    let field = ResolvedStructField {
        name: field.name,
        type_info: resolve_type(field.type_id)?,
    };
    Ok(field)
}
