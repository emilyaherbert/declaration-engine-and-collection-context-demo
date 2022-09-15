use itertools::Itertools;

use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_edge::CollectionEdge,
        collection_index::{CollectionIndex, CCIdx}, collection_node::CollectionNode,
    },
    declaration_engine::declaration_engine::{
        de_insert_function, de_insert_struct, de_insert_trait, de_insert_trait_fn,
        de_insert_trait_impl,
    },
    language::{
        parsed::{
            declaration::{
                Declaration, FunctionDeclaration, FunctionParameter, StructDeclaration,
                StructField, TraitDeclaration, TraitFn, TraitImpl, VariableDeclaration,
            },
            Node,
        },
        ty::typed_declaration::{
            TyDeclaration, TyFunctionDeclaration, TyFunctionParameter, TyStructDeclaration,
            TyStructField, TyTraitDeclaration, TyTraitFn, TyTraitImpl, TyVariableDeclaration,
        },
    },
    type_system::{
        type_engine::insert_type,
        type_mapping::{insert_type_parameters, TypeMapping},
    },
    types::copy_types::CopyTypes,
};

use super::{collect_graph_node, expression::collect_graph_exp};

pub(super) fn collect_graph_decl(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    decl: Declaration,
) -> CCIdx<TyDeclaration> {
    match decl {
        Declaration::Variable(var_decl) => {
            let var_decl = collect_graph_var_decl(cc, type_mapping, var_decl);
            TyDeclaration::Variable(var_decl)
        }
        Declaration::Function(func_decl) => {
            let func_decl = collect_graph_function(cc, type_mapping, func_decl);
            let func_decl_id = de_insert_function(func_decl.clone());
            let func_decl_idx = cc.add_node(CollectionNode::Function(func_decl.name, func_decl_id));
            let cc_idx = CCIdx::new(func_decl_id, func_decl_idx)
            // add an edge to every node in the function body
            func_decl.body.iter().for_each(|node_idx| {
                cc.add_edge(*node_idx, func_decl_idx, CollectionEdge::ScopedChild);
            });
            TyDeclaration::Function(cc_idx)
        }
        Declaration::Trait(trait_decl) => {
            let trait_decl = collect_graph_trait(cc, type_mapping, trait_decl);
            let trait_decl_id = de_insert_trait(trait_decl);
            let trait_decl_idx = cc.add_node(CollectionNode::Trait(trait_decl.name, trait_decl_id));
            TyDeclaration::Trait((trait_decl_id, trait_decl_idx))
        }
        Declaration::TraitImpl(trait_impl) => {
            let trait_impl = collect_graph_trait_impl(cc, type_mapping, trait_impl);
            let decl = TyDeclaration::TraitImpl(de_insert_trait_impl(trait_impl));
            cc.add_node(decl.into())
        }
        Declaration::Struct(struct_decl) => {
            let struct_decl = collect_graph_struct(cc, type_mapping, struct_decl);
            let decl = TyDeclaration::Struct(de_insert_struct(struct_decl));
            cc.add_node(decl.into())
        }
    }
}

fn collect_graph_var_decl(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    var_decl: VariableDeclaration,
) -> TyVariableDeclaration {
    let new_body = collect_graph_exp(cc, type_mapping, var_decl.body);
    let mut new_type_ascription = insert_type(var_decl.type_ascription);
    new_type_ascription.copy_types(cc, type_mapping);
    TyVariableDeclaration {
        name: var_decl.name,
        body: new_body,
        type_ascription: new_type_ascription,
    }
}

fn collect_graph_function(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    func_decl: FunctionDeclaration,
) -> TyFunctionDeclaration {
    // new local mutable copy of type_mapping
    let mut type_mapping = type_mapping.clone();

    // extend type mapping with the type parameters
    type_mapping.extend(insert_type_parameters(func_decl.type_parameters.clone()));

    // collect the type parameters
    let type_parameters = func_decl
        .type_parameters
        .into_iter()
        .map(|mut type_param| {
            type_param.copy_types(cc, &type_mapping);
            type_param
        })
        .collect::<Vec<_>>();

    // collect the parameters
    let parameters = func_decl
        .parameters
        .into_iter()
        .map(|param| collect_graph_function_parameter(cc, &type_mapping, param))
        .collect::<Vec<_>>();

    // collect the return type
    let mut return_type = insert_type(func_decl.return_type);
    return_type.copy_types(cc, &type_mapping);

    // collect the body
    let body = collect_graph_code_block(cc, &type_mapping, func_decl.body);

    let mut decl = TyFunctionDeclaration {
        name: func_decl.name,
        type_parameters,
        parameters,
        body,
        return_type,
    };
    decl.copy_types(cc, &type_mapping);

    decl
}

fn collect_graph_code_block(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    nodes: Vec<Node>,
) -> Vec<CollectionIndex> {
    let nodes = nodes
        .into_iter()
        .map(|node| collect_graph_node(cc, type_mapping, node))
        .collect::<Vec<_>>();

    // for every node in this scope, connect them under the same shared scope
    nodes
        .clone()
        .into_iter()
        .permutations(2)
        .for_each(|inner_nodes| {
            let a = inner_nodes[0];
            let b = inner_nodes[1];
            cc.add_edge(a, b, CollectionEdge::SharedScope);
        });
    nodes
}

fn collect_graph_function_parameter(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    function_parameter: FunctionParameter,
) -> TyFunctionParameter {
    let mut type_id = insert_type(function_parameter.type_info);
    type_id.copy_types(cc, type_mapping);
    TyFunctionParameter {
        name: function_parameter.name,
        type_id,
    }
}

fn collect_graph_trait(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    trait_decl: TraitDeclaration,
) -> TyTraitDeclaration {
    let interface_surface = trait_decl
        .interface_surface
        .into_iter()
        .map(|trait_fn| {
            let trait_fn = collect_graph_trait_fn(cc, type_mapping, trait_fn);
            de_insert_trait_fn(trait_fn)
        })
        .collect::<Vec<_>>();
    TyTraitDeclaration {
        name: trait_decl.name,
        interface_surface,
    }
}

fn collect_graph_trait_fn(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    trait_fn: TraitFn,
) -> TyTraitFn {
    let parameters = trait_fn
        .parameters
        .into_iter()
        .map(|param| collect_graph_function_parameter(cc, type_mapping, param))
        .collect::<Vec<_>>();
    let mut return_type = insert_type(trait_fn.return_type);
    return_type.copy_types(cc, type_mapping);
    TyTraitFn {
        name: trait_fn.name,
        parameters,
        return_type,
    }
}

fn collect_graph_trait_impl(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    trait_impl: TraitImpl,
) -> TyTraitImpl {
    if !trait_impl.type_parameters.is_empty() {
        panic!()
    }
    let methods = trait_impl
        .methods
        .into_iter()
        .map(|method| de_insert_function(collect_graph_function(cc, type_mapping, method)))
        .collect::<Vec<_>>();
    let mut type_implementing_for = insert_type(trait_impl.type_implementing_for);
    type_implementing_for.copy_types(cc, type_mapping);
    TyTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for,
        type_parameters: vec![],
        methods,
    }
}

fn collect_graph_struct(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    struct_decl: StructDeclaration,
) -> TyStructDeclaration {
    let mut type_mapping = type_mapping.clone();
    type_mapping.extend(insert_type_parameters(struct_decl.type_parameters.clone()));
    let type_parameters = struct_decl
        .type_parameters
        .into_iter()
        .map(|mut type_param| {
            type_param.copy_types(cc, &type_mapping);
            type_param
        })
        .collect::<Vec<_>>();
    let fields = struct_decl
        .fields
        .into_iter()
        .map(|field| collect_graph_struct_field(cc, &type_mapping, field))
        .collect::<Vec<_>>();
    TyStructDeclaration {
        name: struct_decl.name,
        type_parameters,
        fields,
    }
}

fn collect_graph_struct_field(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    struct_field: StructField,
) -> TyStructField {
    let mut type_id = insert_type(struct_field.type_info);
    type_id.copy_types(cc, type_mapping);
    TyStructField {
        name: struct_field.name,
        type_id,
    }
}
