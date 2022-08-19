use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        resolved::resolved_declaration::{
            ResolvedDeclaration, ResolvedFunctionDeclaration, ResolvedFunctionParameter,
            ResolvedVariableDeclaration,
        },
        typed::typed_declaration::{
            TypedDeclaration, TypedFunctionDeclaration, TypedFunctionParameter,
            TypedVariableDeclaration,
        },
    },
    type_system::type_engine::resolve_type,
};

use super::{expression::resolve_expression, resolve_nodes};

pub(super) fn resolve_declaration(
    declaration_engine: &DeclarationEngine,
    declaration: TypedDeclaration,
) -> ResolvedDeclaration {
    match declaration {
        TypedDeclaration::Variable(variable_declaration) => {
            let variable_declaration =
                resolve_variable_declaration(declaration_engine, variable_declaration);
            ResolvedDeclaration::Variable(variable_declaration)
        }
        TypedDeclaration::Function(name) => {
            let function_declaration = declaration_engine.get_function(name).unwrap();
            let function_declaration =
                resolve_function_declaration(declaration_engine, function_declaration);
            ResolvedDeclaration::Function(function_declaration)
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
    function_declaration: TypedFunctionDeclaration,
) -> ResolvedFunctionDeclaration {
    if !function_declaration.type_parameters.is_empty() {
        panic!()
    }
    let resolved_parameters = function_declaration
        .parameters
        .into_iter()
        .map(|parameter| resolve_function_parameter(declaration_engine, parameter))
        .collect::<Vec<_>>();
    let resolved_body = resolve_nodes(declaration_engine, function_declaration.body);
    let resolved_type = resolve_type(declaration_engine, function_declaration.return_type).unwrap();
    ResolvedFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: vec![],
        parameters: resolved_parameters,
        body: resolved_body,
        return_type: resolved_type,
    }
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
