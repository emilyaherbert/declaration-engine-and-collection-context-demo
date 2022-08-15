use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::typed_expression::{
            TypedExpression, TypedExpressionVariant, TypedStructExpressionField,
        },
        untyped::expression::{Expression, StructExpressionField},
    },
    type_system::type_engine::TypeEngine,
};

pub(super) fn analyze_expression(
    type_engine: &mut TypeEngine,
    declaration_engine: &mut DeclarationEngine,
    expression: Expression,
) -> TypedExpression {
    match expression {
        Expression::Literal { value } => {
            let type_id = type_engine.insert_type(value.to_type());
            let variant = TypedExpressionVariant::Literal { value };
            TypedExpression { variant, type_id }
        }
        Expression::Variable { name } => {
            let type_id = todo!();
            let variant = TypedExpressionVariant::Variable { name };
            TypedExpression { variant, type_id }
        }
        Expression::FunctionApplication {
            name,
            arguments,
            type_arguments,
        } => {
            let new_arguments = arguments
                .into_iter()
                .map(|argument| analyze_expression(type_engine, declaration_engine, argument))
                .collect::<Vec<_>>();
            let type_id = todo!();
            let variant = TypedExpressionVariant::FunctionApplication {
                name,
                arguments: new_arguments,
            };
            TypedExpression { variant, type_id }
        }
        Expression::Struct {
            struct_name,
            fields,
            type_arguments,
        } => {
            let new_fields = fields
                .into_iter()
                .map(|field| {
                    analyze_struct_expression_field(type_engine, declaration_engine, field)
                })
                .collect::<Vec<_>>();
            let type_id = todo!();
            let variant = TypedExpressionVariant::Struct {
                struct_name,
                fields: new_fields,
            };
            TypedExpression { variant, type_id }
        }
        Expression::Enum {
            enum_name,
            variant_name,
            value,
            type_arguments,
        } => {
            let new_value = analyze_expression(type_engine, declaration_engine, *value);
            let type_id = todo!();
            let variant = TypedExpressionVariant::Enum {
                enum_name,
                variant_name,
                value: Box::new(new_value),
            };
            TypedExpression { variant, type_id }
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
