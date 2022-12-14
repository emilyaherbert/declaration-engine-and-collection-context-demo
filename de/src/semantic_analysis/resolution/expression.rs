use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        resolved::resolved_expression::{
            ResolvedExpression, ResolvedExpressionVariant, ResolvedStructExpressionField,
        },
        typed::typed_expression::{
            TypedExpression, TypedExpressionVariant, TypedStructExpressionField,
        },
    },
    type_system::type_engine::resolve_type,
};

pub(super) fn resolve_expression(
    declaration_engine: &DeclarationEngine,
    expression: TypedExpression,
) -> ResolvedExpression {
    let variant = resolve_expression_variant(declaration_engine, expression.variant);
    ResolvedExpression {
        variant,
        type_info: resolve_type(declaration_engine, expression.type_id).unwrap(),
    }
}

fn resolve_expression_variant(
    declaration_engine: &DeclarationEngine,
    variant: TypedExpressionVariant,
) -> ResolvedExpressionVariant {
    match variant {
        TypedExpressionVariant::Literal { value } => ResolvedExpressionVariant::Literal { value },
        TypedExpressionVariant::Variable { name } => ResolvedExpressionVariant::Variable { name },
        TypedExpressionVariant::FunctionApplication { name, arguments } => {
            let resolved_arguments = arguments
                .into_iter()
                .map(|argument| resolve_expression(declaration_engine, argument))
                .collect::<Vec<_>>();
            ResolvedExpressionVariant::FunctionApplication {
                name,
                arguments: resolved_arguments,
            }
        }
        TypedExpressionVariant::Struct {
            struct_name,
            fields,
        } => {
            let resolved_fields = fields
                .into_iter()
                .map(|field| resolve_struct_expression_field(declaration_engine, field))
                .collect::<Vec<_>>();
            ResolvedExpressionVariant::Struct {
                struct_name,
                fields: resolved_fields,
            }
        }
        TypedExpressionVariant::MethodCall {
            parent_name,
            func_name,
            arguments,
        } => {
            let resolved_arguments = arguments
                .into_iter()
                .map(|argument| resolve_expression(declaration_engine, argument))
                .collect::<Vec<_>>();
            ResolvedExpressionVariant::MethodCall {
                parent_name,
                func_name,
                arguments: resolved_arguments,
            }
        }
        TypedExpressionVariant::FunctionParameter => {
            panic!("did not expect to find function param here")
        }
    }
}

fn resolve_struct_expression_field(
    declaration_engine: &DeclarationEngine,
    struct_expression_field: TypedStructExpressionField,
) -> ResolvedStructExpressionField {
    let new_value = resolve_expression(declaration_engine, struct_expression_field.value);
    ResolvedStructExpressionField {
        name: struct_expression_field.name,
        value: new_value,
    }
}
