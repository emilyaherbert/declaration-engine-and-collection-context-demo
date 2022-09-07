use crate::{
    language::{
        typed::typed_expression::{
            TypedExpression, TypedExpressionVariant, TypedStructExpressionField,
        },
        untyped::expression::Expression,
    },
    namespace::namespace::Namespace,
    type_system::{type_engine::insert_type, type_info::TypeInfo},
};

pub(super) fn type_collect_expression(
    namespace: &mut Namespace,
    expression: Expression,
) -> TypedExpression {
    match expression {
        Expression::Literal { value } => {
            let type_id = insert_type(value.to_type());
            let variant = TypedExpressionVariant::Literal { value };
            TypedExpression { variant, type_id }
        }
        Expression::Variable { name } => {
            let type_id = insert_type(TypeInfo::Unknown);
            let variant = TypedExpressionVariant::Variable { name };
            TypedExpression { variant, type_id }
        }
        Expression::FunctionApplication {
            name,
            type_arguments,
            arguments,
        } => {
            if !type_arguments.is_empty() {
                panic!()
            }
            let new_arguments = arguments
                .into_iter()
                .map(|argument| type_collect_expression(namespace, argument))
                .collect::<Vec<_>>();
            let type_id = insert_type(TypeInfo::Unknown);
            let variant = TypedExpressionVariant::FunctionApplication {
                name,
                arguments: new_arguments,
            };
            TypedExpression { variant, type_id }
        }
        Expression::MethodCall {
            parent_name,
            func_name,
            type_arguments,
            arguments,
        } => {
            if !type_arguments.is_empty() {
                panic!()
            }
            let new_arguments = arguments
                .into_iter()
                .map(|argument| type_collect_expression(namespace, argument))
                .collect::<Vec<_>>();
            let type_id = insert_type(TypeInfo::Unknown);
            let variant = TypedExpressionVariant::MethodCall {
                parent_name,
                func_name,
                arguments: new_arguments,
            };
            TypedExpression { variant, type_id }
        }
        Expression::Struct {
            struct_name,
            type_arguments,
            fields,
        } => {
            if !type_arguments.is_empty() {
                panic!()
            }
            let typed_fields = fields
                .into_iter()
                .map(|field| TypedStructExpressionField {
                    name: field.name,
                    value: type_collect_expression(namespace, field.value),
                })
                .collect::<Vec<_>>();
            let variant = TypedExpressionVariant::Struct {
                struct_name,
                fields: typed_fields,
            };
            TypedExpression {
                variant,
                type_id: insert_type(TypeInfo::Unknown),
            }
        }
    }
}
