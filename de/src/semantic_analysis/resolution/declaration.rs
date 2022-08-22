use crate::{
    declaration_engine::{declaration_engine::DeclarationEngine, declaration_id::DeclarationId},
    language::{
        resolved::resolved_declaration::{
            ResolvedDeclaration, ResolvedFunctionDeclaration, ResolvedFunctionParameter,
            ResolvedTraitDeclaration, ResolvedTraitFn, ResolvedTraitImpl,
            ResolvedVariableDeclaration,
        },
        typed::typed_declaration::{
            TypedDeclaration, TypedFunctionDeclaration, TypedFunctionParameter, TypedTraitFn,
            TypedVariableDeclaration,
        },
    },
    type_system::type_engine::resolve_type,
};

use super::{expression::resolve_expression, resolve_nodes};

pub(super) fn resolve_declaration(
    declaration_engine: &DeclarationEngine,
    declaration: TypedDeclaration,
) -> Vec<ResolvedDeclaration> {
    match declaration {
        TypedDeclaration::Variable(variable_declaration) => {
            let variable_declaration =
                resolve_variable_declaration(declaration_engine, variable_declaration);
            vec![ResolvedDeclaration::Variable(variable_declaration)]
        }
        TypedDeclaration::Function(id) => {
            let function_declarations = resolve_function_declaration(declaration_engine, id);
            function_declarations
                .into_iter()
                .map(ResolvedDeclaration::Function)
                .collect()
        }
        TypedDeclaration::GenericTypeForFunctionScope { .. } => panic!("should not see this here"),
        TypedDeclaration::Trait(id) => {
            let trait_declaration = resolve_trait_declaration(declaration_engine, id);
            vec![ResolvedDeclaration::Trait(trait_declaration)]
        }
        TypedDeclaration::TraitImpl(id) => {
            let trait_impl = resolve_trait_impl(declaration_engine, id);
            vec![ResolvedDeclaration::TraitImpl(trait_impl)]
        }
        // TypedDeclaration::Trait(_) => todo!(),
        // TypedDeclaration::Struct(_) => todo!(),
        // TypedDeclaration::Enum(_) => todo!(),
        // TypedDeclaration::TraitImpl(_) => todo!(),
    }
}

fn resolve_variable_declaration(
    declaration_engine: &DeclarationEngine,
    variable_declaration: TypedVariableDeclaration,
) -> ResolvedVariableDeclaration {
    let type_ascription =
        resolve_type(declaration_engine, variable_declaration.type_ascription).unwrap();
    let body = resolve_expression(declaration_engine, variable_declaration.body);
    ResolvedVariableDeclaration {
        name: variable_declaration.name,
        type_ascription,
        body,
    }
}

fn resolve_function_declaration(
    declaration_engine: &DeclarationEngine,
    function_id: DeclarationId,
) -> Vec<ResolvedFunctionDeclaration> {
    let original_copy = declaration_engine.get_function(function_id).unwrap();
    if original_copy.type_parameters.is_empty() {
        resolve_function_declaration_inner(declaration_engine, vec![original_copy])
    } else {
        let monomorphized_copies = declaration_engine
            .get_monomorphized_function_copies(function_id)
            .unwrap();
        resolve_function_declaration_inner(declaration_engine, monomorphized_copies)
    }
}

fn resolve_function_declaration_inner(
    declaration_engine: &DeclarationEngine,
    function_declarations: Vec<TypedFunctionDeclaration>,
) -> Vec<ResolvedFunctionDeclaration> {
    function_declarations
        .into_iter()
        .map(|function_declaration| {
            let resolved_parameters = function_declaration
                .parameters
                .into_iter()
                .map(|parameter| resolve_function_parameter(declaration_engine, parameter))
                .collect::<Vec<_>>();
            let resolved_body = resolve_nodes(declaration_engine, function_declaration.body);
            let resolved_type =
                resolve_type(declaration_engine, function_declaration.return_type).unwrap();
            ResolvedFunctionDeclaration {
                name: function_declaration.name,
                parameters: resolved_parameters,
                body: resolved_body,
                return_type: resolved_type,
            }
        })
        .collect()
}

fn resolve_function_parameter(
    declaration_engine: &DeclarationEngine,
    function_parameter: TypedFunctionParameter,
) -> ResolvedFunctionParameter {
    ResolvedFunctionParameter {
        name: function_parameter.name,
        type_info: resolve_type(declaration_engine, function_parameter.type_id).unwrap(),
    }
}

fn resolve_trait_declaration(
    declaration_engine: &DeclarationEngine,
    trait_id: DeclarationId,
) -> ResolvedTraitDeclaration {
    let trait_decl = declaration_engine.get_trait(trait_id).unwrap();
    let new_interface_surface = trait_decl
        .interface_surface
        .into_iter()
        .map(|trait_fn| resolve_trait_fn(declaration_engine, trait_fn))
        .collect::<Vec<_>>();
    ResolvedTraitDeclaration {
        name: trait_decl.name,
        interface_surface: new_interface_surface,
    }
}

fn resolve_trait_fn(
    declaration_engine: &DeclarationEngine,
    trait_fn: TypedTraitFn,
) -> ResolvedTraitFn {
    let resolved_parameters = trait_fn
        .parameters
        .into_iter()
        .map(|parameter| resolve_function_parameter(declaration_engine, parameter))
        .collect::<Vec<_>>();
    let resolved_type = resolve_type(declaration_engine, trait_fn.return_type).unwrap();
    ResolvedTraitFn {
        name: trait_fn.name,
        parameters: resolved_parameters,
        return_type: resolved_type,
    }
}

fn resolve_trait_impl(
    declaration_engine: &DeclarationEngine,
    impl_id: DeclarationId,
) -> ResolvedTraitImpl {
    let trait_impl = declaration_engine.get_trait_impl(impl_id).unwrap();
    let type_implementing_for =
        resolve_type(declaration_engine, trait_impl.type_implementing_for).unwrap();
    let methods = trait_impl
        .methods
        .into_iter()
        .flat_map(|method_id| resolve_function_declaration(declaration_engine, method_id))
        .collect::<Vec<_>>();
    ResolvedTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for,
        methods,
    }
}
