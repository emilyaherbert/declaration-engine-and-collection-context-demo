use std::collections::{HashMap, HashSet};

use crate::collection_context::collection_context::CollectionContext;
use crate::collection_context::collection_index::CollectionIndex;
use crate::declaration_engine::declaration_engine::*;
use crate::type_system::type_engine::resolve_custom_types;
use crate::type_system::type_id::TypeId;
use crate::{
    language::ty::typed_expression::{TyExpression, TyExpressionVariant},
    namespace::namespace::Namespace,
    type_system::type_engine::{monomorphize, unify_types},
    types::create_type_id::CreateTypeId,
};

pub(super) fn analyze_expression(
    cc: &CollectionContext,
    current_index: CollectionIndex,
    ns: &mut Namespace,
    expression: &mut TyExpression,
) {
    analyze_expression_variant(
        cc,
        current_index,
        ns,
        &mut expression.variant,
        &mut expression.type_id,
    );
}

fn analyze_expression_variant(
    cc: &CollectionContext,
    current_index: CollectionIndex,
    ns: &mut Namespace,
    variant: &mut TyExpressionVariant,
    type_id: &mut TypeId,
) {
    match variant {
        TyExpressionVariant::Literal { .. } => {}
        TyExpressionVariant::Variable { name } => {
            let variable_decl = ns.get_symbol(name).unwrap().expect_variable().unwrap();
            unify_types(variable_decl.type_ascription, *type_id).unwrap();
        }
        TyExpressionVariant::FunctionApplication {
            name,
            type_arguments,
            arguments,
        } => {
            // don't allow type arguments in the prototype
            if !type_arguments.is_empty() {
                panic!()
            }

            // get the original decl id for the function from the CC
            let decl_id = cc.get_symbol(current_index, name).unwrap();

            // get the original function declaration
            let mut typed_function_declaration = de_get_function(*decl_id.inner_ref()).unwrap();

            // make sure we have the correct number of arguments
            if typed_function_declaration.parameters.len() != arguments.len() {
                panic!();
            }

            // do type inference on the type arguments
            for type_argument in type_arguments.iter_mut() {
                resolve_custom_types(type_argument.type_id, cc, current_index).unwrap();
            }

            // monomorphize the function declaration into a new copy, in place
            monomorphize(&mut typed_function_declaration, type_arguments).unwrap();

            // add the new copy to the declaration engine
            de_add_monomorphized_function_copy(
                *decl_id.inner_ref(),
                typed_function_declaration.clone(),
            );

            // do type inference on the arguments
            arguments
                .iter_mut()
                .zip(typed_function_declaration.parameters.iter())
                .for_each(|(argument, parameter)| {
                    analyze_expression(cc, current_index, ns, argument);
                    unify_types(argument.type_id, parameter.type_id).unwrap();
                });

            // unify the return type of the function declaration and the expression
            unify_types(typed_function_declaration.return_type, *type_id).unwrap();
        }
        TyExpressionVariant::Struct {
            struct_name,
            type_arguments,
            fields,
        } => {
            // don't allow type arguments in the prototype
            if !type_arguments.is_empty() {
                panic!()
            }

            // get the original decl id for the struct from the cc
            let decl_id = cc.get_symbol(current_index, struct_name).unwrap().inner();

            // get the original struct declaration
            let mut typed_struct_declaration = de_get_struct(decl_id).unwrap();

            // do type inference on the type arguments
            for type_argument in type_arguments.iter_mut() {
                resolve_custom_types(type_argument.type_id, cc, current_index).unwrap();
            }

            // monomorphize the struct declaration into a new copy, in place
            monomorphize(&mut typed_struct_declaration, type_arguments).unwrap();

            // add the new copy to the declaration engine
            de_add_monomorphized_struct_copy(decl_id, typed_struct_declaration.clone());

            // create reference maps for the expression and the declaration
            let mut given_fields_map: HashMap<_, _> = fields
                .iter()
                .map(|field| (field.name.clone(), field.value.clone()))
                .collect();
            let oracle_fields_map: HashMap<_, _> = typed_struct_declaration
                .fields
                .iter()
                .cloned()
                .map(|field| (field.name, field.type_id))
                .collect();

            // check to see that all of the necessary fields are provided and that no erroneous
            // fields are provided
            if given_fields_map.keys().into_iter().collect::<HashSet<_>>()
                != oracle_fields_map.keys().into_iter().collect::<HashSet<_>>()
            {
                panic!();
            }

            // do type inference on the fields
            given_fields_map.iter_mut().for_each(|(name, value)| {
                analyze_expression(cc, current_index, ns, value);
                let oracle_field = oracle_fields_map.get(name).unwrap();
                unify_types(value.type_id, *oracle_field).unwrap();
            });

            // unify the struct type id with the expression type id
            unify_types(typed_struct_declaration.create_type_id(), *type_id).unwrap();
        }
        TyExpressionVariant::MethodCall {
            parent_name,
            func_name,
            type_arguments,
            arguments,
        } => {
            // don't allow type arguments in the prototype
            if !type_arguments.is_empty() {
                panic!()
            }

            // get the variable decl for this method call
            let parent = ns
                .get_symbol(parent_name)
                .unwrap()
                .expect_variable()
                .unwrap();

            // get the method declaration
            let typed_method_declaration =
                ns.get_method(parent.type_ascription, func_name).unwrap();

            // do type inference on the type arguments
            type_arguments.iter().for_each(|type_arg| {
                resolve_custom_types(type_arg.type_id, cc, current_index).unwrap()
            });

            // do type inference on the arguments
            arguments
                .iter_mut()
                .zip(typed_method_declaration.parameters.iter())
                .for_each(|(argument, parameter)| {
                    analyze_expression(cc, current_index, ns, argument);
                    unify_types(argument.type_id, parameter.type_id).unwrap();
                });

            // unify the return type of the method declaration and the expression
            unify_types(typed_method_declaration.return_type, *type_id).unwrap();
        }
        TyExpressionVariant::FunctionParameter => todo!(),
    }
}
