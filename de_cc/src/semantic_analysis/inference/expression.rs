use std::collections::{HashMap, HashSet};

use crate::declaration_engine::declaration_engine::*;
use crate::type_system::type_engine::eval_type;
use crate::{
    language::{
        typed::typed_expression::{TyExpression, TyExpressionVariant, TyStructExpressionField},
        untyped::expression::Expression,
    },
    namespace::namespace::Namespace,
    type_system::{
        type_engine::{insert_type, monomorphize, unify_types},
        type_info::TypeInfo,
    },
    types::create_type_id::CreateTypeId,
};

pub(super) fn analyze_expression(namespace: &mut Namespace, expression: &TyExpression) {
    unimplemented!()
}

fn analyze_expression_variant(namespace: &mut Namespace, expression: &TyExpressionVariant) {
    match expression {
        TyExpressionVariant::Literal { value } => {
            let type_id = insert_type(value.to_type());
            let variant = TyExpressionVariant::Literal { value };
            TyExpression { variant, type_id }
        }
        TyExpressionVariant::Variable { name } => {
            let variable_decl = namespace
                .get_symbol(&name)
                .unwrap()
                .expect_variable()
                .unwrap();
            let type_id = variable_decl.type_ascription;
            let variant = TyExpressionVariant::Variable { name };
            TyExpression { variant, type_id }
        }
        TyExpressionVariant::FunctionApplication {
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
            let mut typed_function_declaration = de_get_function_typed(decl_id).unwrap();

            // make sure we have the correct number of arguments
            if typed_function_declaration.parameters.len() != arguments.len() {
                panic!();
            }

            // type check the type arguments
            for type_argument in type_arguments.iter_mut() {
                type_argument.type_id = eval_type(type_argument.type_id, namespace).unwrap();
            }

            // monomorphize the function declaration into a new copy
            monomorphize(&mut typed_function_declaration, &type_arguments).unwrap();

            // add the new copy to the declaration engine
            de_add_monomorphized_function_copy(decl_id, typed_function_declaration.clone());

            // type check the arguments
            let new_arguments = arguments
                .into_iter()
                .zip(typed_function_declaration.parameters.iter())
                .map(|(argument, parameter)| {
                    let typed_argument = analyze_expression(namespace, argument);
                    unify_types(typed_argument.type_id, parameter.type_id).unwrap();
                    typed_argument
                })
                .collect::<Vec<_>>();

            // the type id is the functions return type id
            let type_id = insert_type(TypeInfo::Ref(typed_function_declaration.return_type));

            let variant = TyExpressionVariant::FunctionApplication {
                name,
                arguments: new_arguments,
            };
            TyExpression { variant, type_id }
        }
        TyExpressionVariant::Struct {
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
            let mut typed_struct_declaration = de_get_struct(decl_id).unwrap();

            // type check the type arguments
            for type_argument in type_arguments.iter_mut() {
                type_argument.type_id = eval_type(type_argument.type_id, namespace).unwrap();
            }

            // monomorphize the struct declaration into a new copy
            monomorphize(&mut typed_struct_declaration, &type_arguments).unwrap();

            // add the new copy to the declaration engine
            de_add_monomorphized_struct_copy(decl_id, typed_struct_declaration.clone());

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
                    let typed_value = analyze_expression(namespace, value);
                    let oracle_field = oracle_fields_map.get(&name).unwrap();
                    unify_types(typed_value.type_id, *oracle_field).unwrap();
                    TyStructExpressionField {
                        name,
                        value: typed_value,
                    }
                })
                .collect::<Vec<_>>();

            let variant = TyExpressionVariant::Struct {
                struct_name,
                fields: typed_fields,
            };
            TyExpression {
                variant,
                type_id: typed_struct_declaration.create_type_id(),
            }
        }
        TyExpressionVariant::MethodCall {
            parent_name,
            func_name,
            type_arguments,
            arguments,
        } => {
            if !type_arguments.is_empty() {
                panic!()
            }

            // get the variable decl for this method call
            let parent = namespace
                .get_symbol(&parent_name)
                .unwrap()
                .expect_variable()
                .unwrap();

            // get the function call
            let typed_function_declaration = namespace
                .get_method(parent.type_ascription, &func_name)
                .unwrap();

            // type check the arguments
            let new_arguments = arguments
                .into_iter()
                .zip(typed_function_declaration.parameters.iter())
                .map(|(argument, parameter)| {
                    let typed_argument = analyze_expression(namespace, argument);
                    unify_types(typed_argument.type_id, parameter.type_id).unwrap();
                    typed_argument
                })
                .collect::<Vec<_>>();

            // the type id is the functions return type id
            let type_id = insert_type(TypeInfo::Ref(typed_function_declaration.return_type));

            let variant = TyExpressionVariant::MethodCall {
                parent_name,
                func_name,
                arguments: new_arguments,
            };
            TyExpression { variant, type_id }
        }
    }
}
