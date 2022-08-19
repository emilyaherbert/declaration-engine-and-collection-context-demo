use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::typed_expression::{TypedExpression, TypedExpressionVariant},
        untyped::expression::Expression,
    },
    namespace::namespace::Namespace,
    type_system::{
        type_engine::{insert_type, monomorphize, unify_types},
        type_info::TypeInfo,
    },
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
            let variable_decl = namespace
                .get_symbol(&name)
                .unwrap()
                .expect_variable()
                .unwrap();
            let type_id = variable_decl.type_ascription;
            let variant = TypedExpressionVariant::Variable { name };
            TypedExpression { variant, type_id }
        }
        Expression::FunctionApplication {
            name,
            mut type_arguments,
            arguments,
        } => {
            if !type_arguments.is_empty() {
                panic!()
            }
            let decl_id = namespace
                .get_symbol(&name)
                .unwrap()
                .expect_function()
                .unwrap();
            let mut typed_function_declaration = declaration_engine.get_function(decl_id).unwrap();
            monomorphize(
                &mut typed_function_declaration,
                &mut type_arguments,
                namespace,
            )
            .unwrap();
            declaration_engine
                .add_monomorphized_function_copy(decl_id, typed_function_declaration.clone());
            if typed_function_declaration.parameters.len() != arguments.len() {
                panic!("wrong number of arguments");
            }
            let new_arguments = arguments
                .into_iter()
                .zip(typed_function_declaration.parameters.iter())
                .map(|(argument, parameter)| {
                    let typed_argument =
                        analyze_expression(namespace, declaration_engine, argument);
                    unify_types(typed_argument.type_id, parameter.type_id).unwrap();
                    typed_argument
                })
                .collect::<Vec<_>>();
            let type_id = insert_type(TypeInfo::Ref(typed_function_declaration.return_type));
            let variant = TypedExpressionVariant::FunctionApplication {
                name,
                arguments: new_arguments,
            };
            TypedExpression { variant, type_id }
        } // Expression::Struct { .. } => {
          //     let new_fields = fields
          //         .into_iter()
          //         .map(|field| {
          //             analyze_struct_expression_field(
          //                 namespace,

          //                 declaration_engine,
          //                 field,
          //             )
          //         })
          //         .collect();
          //     let type_id = todo!();
          //     let variant = TypedExpressionVariant::Struct {
          //         struct_name,
          //         fields: new_fields,
          //     };
          //     TypedExpression { variant, type_id }
          // }
          // Expression::Enum { .. } => {
          //     let new_value =
          //         analyze_expression(namespace,  declaration_engine, *value);
          //     let type_id = todo!();
          //     let variant = TypedExpressionVariant::Enum {
          //         enum_name,
          //         variant_name,
          //         value: Box::new(new_value),
          //     };
          //     TypedExpression { variant, type_id }
          // }
    }
}

// fn analyze_struct_expression_field(
//     namespace: &mut Namespace,
//     declaration_engine: &mut DeclarationEngine,
//     struct_expression_field: StructExpressionField,
// ) -> TypedStructExpressionField {
//     let new_value =
//         analyze_expression(namespace, declaration_engine, struct_expression_field.value);
//     TypedStructExpressionField {
//         name: struct_expression_field.name,
//         value: new_value,
//     }
// }
