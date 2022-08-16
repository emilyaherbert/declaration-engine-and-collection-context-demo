use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::typed_expression::{
            TypedExpression, TypedExpressionVariant, TypedStructExpressionField,
        },
        untyped::expression::{Expression, StructExpressionField},
    },
    namespace::Namespace,
    type_system::type_engine::insert_type,
};

pub(super) fn analyze_expression(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    expression: Expression,
) -> TypedExpression {
    match expression {
        Expression::Literal { value } => {
            let type_id = insert_type(value.to_type());
            let variant = TypedExpressionVariant::Literal { value };
            TypedExpression { variant, type_id }
        }
        Expression::Variable { name } => {
            let unknown_decl = namespace.get_symbol(&name).unwrap();
            let variable_decl = unknown_decl.expect_variable().unwrap();
            let type_id = variable_decl.type_ascription;
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
                .map(|argument| analyze_expression(namespace, declaration_engine, argument))
                .collect();
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
                .map(|field| analyze_struct_expression_field(namespace, declaration_engine, field))
                .collect();
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
            let new_value = analyze_expression(namespace, declaration_engine, *value);
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
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    struct_expression_field: StructExpressionField,
) -> TypedStructExpressionField {
    let new_value =
        analyze_expression(namespace, declaration_engine, struct_expression_field.value);
    TypedStructExpressionField {
        name: struct_expression_field.name,
        value: new_value,
    }
}
