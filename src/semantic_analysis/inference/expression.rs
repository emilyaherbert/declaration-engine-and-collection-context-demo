use crate::{
    declaration_engine::DeclarationEngine,
    language::{Expression, StructExpressionField, TypedExpression, TypedStructExpressionField, TypedExpressionVariant},
    type_system::TypeEngine,
};

pub(super) fn analyze_expression(
    type_engine: &mut TypeEngine,
    declaration_engine: &mut DeclarationEngine,
    expression: Expression,
) -> TypedExpression {
    match expression {
        Expression::Literal { value } => {
            let type_id = todo!();
            let variant = TypedExpressionVariant::Literal { value };
            TypedExpression {
                variant,
                type_id
            }
        },
        Expression::Variable { name } => TypedExpressionVariant::Variable { name },
        Expression::FunctionApplication { name, arguments } => {
            let new_arguments = arguments
                .into_iter()
                .map(|argument| analyze_expression(type_engine, declaration_engine, argument))
                .collect::<Vec<_>>();
            TypedExpressionVariant::FunctionApplication {
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
                .map(|field| {
                    analyze_struct_expression_field(type_engine, declaration_engine, field)
                })
                .collect::<Vec<_>>();
            TypedExpressionVariant::Struct {
                struct_name,
                fields: new_fields,
            }
        }
        Expression::Enum {
            enum_name,
            variant_name,
            value,
        } => {
            let new_value = analyze_expression(type_engine, declaration_engine, *value);
            TypedExpressionVariant::Enum {
                enum_name,
                variant_name,
                value: Box::new(new_value),
            }
        }
    }
}

fn analyze_struct_expression_field(
    type_engine: &mut TypeEngine,
    declaration_engine: &mut DeclarationEngine,
    struct_expression_field: StructExpressionField,
) -> TypedStructExpressionField {
    let new_value = analyze_expression(
        type_engine,
        declaration_engine,
        struct_expression_field.value,
    );
    TypedStructExpressionField {
        name: struct_expression_field.name,
        value: new_value,
    }
}
