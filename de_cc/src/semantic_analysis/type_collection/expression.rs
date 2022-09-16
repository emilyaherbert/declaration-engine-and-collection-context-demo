use crate::{
    collection_context::collection_context::CollectionContext,
    language::{
        parsed::expression::Expression,
        ty::typed_expression::{TyExpression, TyExpressionVariant, TyStructExpressionField},
    },
    type_system::{type_engine::insert_type, type_info::TypeInfo},
};

pub(super) fn collect_types_exp(cc: &CollectionContext, exp: &TyExpression) -> TyExpression {
    match exp {
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
            // don't allow type arguments in the prototype
            if !type_arguments.is_empty() {
                panic!()
            }

            // transform the arguments into Ty AST nodes
            let new_arguments = arguments
                .into_iter()
                .map(|argument| collect_types_exp(cc, argument))
                .collect::<Vec<_>>();

            // return!
            let variant = TyExpressionVariant::FunctionApplication {
                name,
                type_arguments,
                arguments: new_arguments,
            };
            TyExpression {
                variant,
                type_id: insert_type(TypeInfo::Unknown),
            }
        }
        Expression::MethodCall {
            parent_name,
            func_name,
            type_arguments,
            arguments,
        } => {
            // don't allow type arguments in the prototype
            if !type_arguments.is_empty() {
                panic!()
            }

            // transform the arguments into Ty AST nodes
            let new_arguments = arguments
                .into_iter()
                .map(|argument| collect_types_exp(cc, argument))
                .collect::<Vec<_>>();

            // return!
            let variant = TyExpressionVariant::MethodCall {
                parent_name,
                func_name,
                type_arguments,
                arguments: new_arguments,
            };
            TyExpression {
                variant,
                type_id: insert_type(TypeInfo::Unknown),
            }
        }
        Expression::Struct {
            struct_name,
            type_arguments,
            fields,
        } => {
            // don't allow type arguments in the prototype
            if !type_arguments.is_empty() {
                panic!()
            }

            // transform the fields into Ty AST nodes
            let typed_fields = fields
                .into_iter()
                .map(|field| TyStructExpressionField {
                    name: field.name,
                    value: collect_types_exp(cc, field.value),
                })
                .collect::<Vec<_>>();

            // return!
            let variant = TyExpressionVariant::Struct {
                struct_name,
                type_arguments,
                fields: typed_fields,
            };
            TyExpression {
                variant,
                type_id: insert_type(TypeInfo::Unknown),
            }
        }
    }
}
