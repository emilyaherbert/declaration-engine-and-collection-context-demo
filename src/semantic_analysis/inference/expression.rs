use crate::{
    declaration_engine::DeclarationEngine,
    language::{Expression, StructExpressionField, TypedExpression, TypedStructExpressionField},
};

pub(super) fn analyze_expression(
    declaration_engine: &mut DeclarationEngine,
    expression: Expression,
) -> TypedExpression {
    match expression {
        Expression::Literal { value } => TypedExpression::Literal { value },
        Expression::Variable { name } => TypedExpression::Variable { name },
        Expression::FunctionApplication { name, arguments } => {
            let new_arguments = arguments
                .into_iter()
                .map(|argument| analyze_expression(declaration_engine, argument))
                .collect::<Vec<_>>();
            TypedExpression::FunctionApplication {
                name,
                arguments: new_arguments,
            }
        }
        Expression::Struct {
            struct_name,
            fields,
        } => {
            let new_fields = fields
                .into_iter()
                .map(|field| analyze_struct_expression_field(declaration_engine, field))
                .collect::<Vec<_>>();
            TypedExpression::Struct {
                struct_name,
                fields: new_fields,
            }
        }
        Expression::Enum {
            enum_name,
            variant_name,
            value,
        } => {
            let new_value = analyze_expression(declaration_engine, *value);
            TypedExpression::Enum {
                enum_name,
                variant_name,
                value: Box::new(new_value),
            }
        }
    }
}

fn analyze_struct_expression_field(
    declaration_engine: &mut DeclarationEngine,
    struct_expression_field: StructExpressionField,
) -> TypedStructExpressionField {
    let new_value = analyze_expression(declaration_engine, struct_expression_field.value);
    TypedStructExpressionField {
        name: struct_expression_field.name,
        value: new_value,
    }
}
