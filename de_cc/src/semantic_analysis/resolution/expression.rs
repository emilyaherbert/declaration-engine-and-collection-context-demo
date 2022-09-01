use crate::{
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

pub(super) fn resolve_expression(expression: TypedExpression) -> ResolvedExpression {
    let variant = resolve_expression_variant(expression.variant);
    ResolvedExpression {
        variant,
        type_info: resolve_type(expression.type_id).unwrap(),
    }
}

fn resolve_expression_variant(variant: TypedExpressionVariant) -> ResolvedExpressionVariant {
    match variant {
        TypedExpressionVariant::Literal { value } => ResolvedExpressionVariant::Literal { value },
        TypedExpressionVariant::Variable { name } => ResolvedExpressionVariant::Variable { name },
        TypedExpressionVariant::FunctionApplication { name, arguments } => {
            let resolved_arguments = arguments
                .into_iter()
                .map(resolve_expression)
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
                .map(resolve_struct_expression_field)
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
                .map(resolve_expression)
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
    struct_expression_field: TypedStructExpressionField,
) -> ResolvedStructExpressionField {
    let new_value = resolve_expression(struct_expression_field.value);
    ResolvedStructExpressionField {
        name: struct_expression_field.name,
        value: new_value,
    }
}
