use crate::{
    language::{
        resolved::resolved_expression::{
            ResolvedExpression, ResolvedExpressionVariant, ResolvedStructExpressionField,
        },
        ty::typed_expression::{TyExpression, TyExpressionVariant, TyStructExpressionField},
    },
    type_system::type_engine::resolve_type,
};

pub(super) fn to_resolved_expression(expression: TyExpression) -> ResolvedExpression {
    let variant = resolve_expression_variant(expression.variant);
    ResolvedExpression {
        variant,
        type_info: resolve_type(expression.type_id).unwrap(),
    }
}

fn resolve_expression_variant(variant: TyExpressionVariant) -> ResolvedExpressionVariant {
    match variant {
        TyExpressionVariant::Literal { value } => ResolvedExpressionVariant::Literal { value },
        TyExpressionVariant::Variable { name } => ResolvedExpressionVariant::Variable { name },
        TyExpressionVariant::FunctionApplication {
            name,
            type_arguments: _,
            arguments,
        } => {
            let resolved_arguments = arguments
                .into_iter()
                .map(to_resolved_expression)
                .collect::<Vec<_>>();
            ResolvedExpressionVariant::FunctionApplication {
                name,
                arguments: resolved_arguments,
            }
        }
        TyExpressionVariant::Struct {
            struct_name,
            type_arguments: _,
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
        TyExpressionVariant::MethodCall {
            parent_name,
            func_name,
            type_arguments: _,
            arguments,
        } => {
            let resolved_arguments = arguments
                .into_iter()
                .map(to_resolved_expression)
                .collect::<Vec<_>>();
            ResolvedExpressionVariant::MethodCall {
                parent_name,
                func_name,
                arguments: resolved_arguments,
            }
        }
        TyExpressionVariant::FunctionParameter => {
            panic!("did not expect to find function param here")
        }
    }
}

fn resolve_struct_expression_field(
    struct_expression_field: TyStructExpressionField,
) -> ResolvedStructExpressionField {
    let new_value = to_resolved_expression(struct_expression_field.value);
    ResolvedStructExpressionField {
        name: struct_expression_field.name,
        value: new_value,
    }
}
