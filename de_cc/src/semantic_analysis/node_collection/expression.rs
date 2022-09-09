use crate::{
    language::{
        parsed::expression::Expression,
        ty::typed_expression::{TyExpression, TyExpressionVariant, TyStructExpressionField},
    },
    type_system::{type_engine::insert_type, type_info::TypeInfo, type_mapping::TypeMapping},
    types::copy_types::CopyTypes,
};

pub(super) fn collect_nodes_expression(
    type_mapping: &TypeMapping,
    expression: Expression,
) -> TyExpression {
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
            mut type_arguments,
            arguments,
        } => {
            // don't allow type arguments in the prototype
            if !type_arguments.is_empty() {
                panic!()
            }

            // apply the type mapping to the type arguments
            type_arguments
                .iter_mut()
                .for_each(|type_arg| type_arg.copy_types(type_mapping));

            // transform the arguments into Ty AST nodes
            let new_arguments = arguments
                .into_iter()
                .map(|argument| collect_nodes_expression(type_mapping, argument))
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
            mut type_arguments,
            arguments,
        } => {
            // don't allow type arguments in the prototype
            if !type_arguments.is_empty() {
                panic!()
            }

            // apply the type mapping to the type arguments
            type_arguments
                .iter_mut()
                .for_each(|type_arg| type_arg.copy_types(type_mapping));

            // transform the arguments into Ty AST nodes
            let new_arguments = arguments
                .into_iter()
                .map(|argument| collect_nodes_expression(type_mapping, argument))
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
            mut type_arguments,
            fields,
        } => {
            // don't allow type arguments in the prototype
            if !type_arguments.is_empty() {
                panic!()
            }

            // apply the type mapping to the type arguments
            type_arguments
                .iter_mut()
                .for_each(|type_arg| type_arg.copy_types(type_mapping));

            // transform the fields into Ty AST nodes
            let typed_fields = fields
                .into_iter()
                .map(|field| TyStructExpressionField {
                    name: field.name,
                    value: collect_nodes_expression(type_mapping, field.value),
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
