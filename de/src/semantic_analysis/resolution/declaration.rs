use crate::{
    declaration_engine::{declaration_engine::DeclarationEngine, declaration_id::DeclarationId},
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
) -> Vec<ResolvedDeclaration> {
    match declaration {
        TypedDeclaration::Variable(variable_declaration) => {
            let variable_declaration =
                resolve_variable_declaration(declaration_engine, variable_declaration);
            vec!(ResolvedDeclaration::Variable(variable_declaration))
        }
        TypedDeclaration::Function(id) => {
            let function_declarations =
                resolve_function_declaration(declaration_engine, id);
            function_declarations.into_iter().map(ResolvedDeclaration::Function).collect()
        },
        TypedDeclaration::GenericTypeForFunctionScope { .. } => panic!("should not see this here")
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
        println!("{:#?}", monomorphized_copies);
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
