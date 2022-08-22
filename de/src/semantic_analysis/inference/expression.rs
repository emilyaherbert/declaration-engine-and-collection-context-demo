use std::collections::{HashMap, HashSet};

use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::typed_expression::{
            TypedExpression, TypedExpressionVariant, TypedStructExpressionField,
        },
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

            // get the original decl id for the function from the namespace
            let decl_id = namespace
                .get_symbol(&name)
                .unwrap()
                .expect_function()
                .unwrap();

            // get the original function declaration
            let mut typed_function_declaration = declaration_engine.get_function(decl_id).unwrap();

            // make sure we have the correct number of arguments
            if typed_function_declaration.parameters.len() != arguments.len() {
                panic!();
            }

            // monomorphize the function declaration into a new copy
            monomorphize(
                &mut typed_function_declaration,
                &mut type_arguments,
                namespace,
            )
            .unwrap();

            // add the new copy to the declaration engine
            declaration_engine
                .add_monomorphized_function_copy(decl_id, typed_function_declaration.clone());

            // type check the arguments
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

            // the type id is the functions return type id
            let type_id = insert_type(TypeInfo::Ref(typed_function_declaration.return_type));

            let variant = TypedExpressionVariant::FunctionApplication {
                name,
                arguments: new_arguments,
            };
            TypedExpression { variant, type_id }
        }
        #[allow(unreachable_code)]
        Expression::Struct {
            struct_name,
            mut type_arguments,
            fields,
        } => {
            if !type_arguments.is_empty() {
                panic!()
            }

            // get the original decl id for the struct from the namespace
            let decl_id = namespace
                .get_symbol(&struct_name)
                .unwrap()
                .expect_struct()
                .unwrap();

            // get the original struct declaration
            let mut typed_struct_declaration = declaration_engine.get_struct(decl_id).unwrap();

            // monomorphize the struct declaration into a new copy
            monomorphize(
                &mut typed_struct_declaration,
                &mut type_arguments,
                namespace,
            )
            .unwrap();

            // add the new copy to the declaration engine
            declaration_engine
                .add_monomorphized_struct_copy(decl_id, typed_struct_declaration.clone());

            // type check the fields
            let given_fields_map: HashMap<_, _> = fields
                .into_iter()
                .map(|field| (field.name, field.value))
                .collect();
            let oracle_fields_map: HashMap<_, _> = typed_struct_declaration
                .fields
                .iter()
                .cloned()
                .map(|field| (field.name, field.type_id))
                .collect();
            if given_fields_map.keys().into_iter().collect::<HashSet<_>>()
                != oracle_fields_map.keys().into_iter().collect::<HashSet<_>>()
            {
                panic!();
            }

            let typed_fields = given_fields_map
                .into_iter()
                .map(|(name, value)| {
                    let typed_value = analyze_expression(namespace, declaration_engine, value);
                    let oracle_field = oracle_fields_map.get(&name).unwrap();
                    unify_types(typed_value.type_id, *oracle_field).unwrap();
                    TypedStructExpressionField {
                        name,
                        value: typed_value,
                    }
                })
                .collect::<Vec<_>>();

            let variant = TypedExpressionVariant::Struct {
                struct_name,
                fields: typed_fields,
            };
            let type_id = insert_type(TypeInfo::Struct {
                name: typed_struct_declaration.name,
                type_parameters: typed_struct_declaration.type_parameters,
                fields: typed_struct_declaration.fields,
            });
            TypedExpression { variant, type_id }
        }
        Expression::MethodCall {
            parent,
            name,
            type_arguments,
            arguments,
        } => {
            if !type_arguments.is_empty() {
                panic!()
            }

            unimplemented!()
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
