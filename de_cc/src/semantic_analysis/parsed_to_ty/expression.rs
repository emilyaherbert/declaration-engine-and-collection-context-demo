use crate::{
    language::{
        parsed::expression::Expression,
        ty::typed_expression::{TyExpression, TyExpressionVariant, TyStructExpressionField},
    },
    type_system::{type_engine::insert_type, type_info::TypeInfo},
};

pub(super) fn to_ty_expression(expression: Expression) -> TyExpression {
    match expression {
        Expression::Literal { value } => {
            let type_id = insert_type(value.to_type());
            let variant = TyExpressionVariant::Literal { value };
            TyExpression { variant, type_id }
        }
        Expression::Variable { name } => {
            let type_id = insert_type(TypeInfo::Unknown);
            let variant = TyExpressionVariant::Variable { name };
            TyExpression { variant, type_id }
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
                .map(|argument| to_ty_expression(argument))
                .collect::<Vec<_>>();
            let type_id = insert_type(TypeInfo::Unknown);
            let variant = TyExpressionVariant::FunctionApplication {
                name,
                arguments: new_arguments,
            };
            TyExpression { variant, type_id }
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
                .map(|argument| to_ty_expression(argument))
                .collect::<Vec<_>>();
            let type_id = insert_type(TypeInfo::Unknown);
            let variant = TyExpressionVariant::MethodCall {
                parent_name,
                func_name,
                arguments: new_arguments,
            };
            TyExpression { variant, type_id }
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
                .map(|field| TyStructExpressionField {
                    name: field.name,
                    value: to_ty_expression(field.value),
                })
                .collect::<Vec<_>>();
            let variant = TyExpressionVariant::Struct {
                struct_name,
                fields: typed_fields,
            };
            TyExpression {
                variant,
                type_id: insert_type(TypeInfo::Unknown),
            }
        }
    }
}
