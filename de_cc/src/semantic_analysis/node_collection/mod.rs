mod declaration;
mod expression;

use itertools::Itertools;
use std::collections::HashMap;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_edge::CollectionEdge,
        collection_index::CollectionIndex,
    },
    language::{
        parsed::{Application, File, Node},
        ty::{TyApplication, TyFile, TyNode},
    },
    type_system::type_mapping::TypeMapping,
};

pub(crate) fn collect_nodes(
    cc: &mut CollectionContext,
    application: Application,
) -> CollectionIndex {
    let file_indices = application
        .files
        .into_iter()
        .map(|file| collect_nodes_file(cc, file))
        .collect::<Vec<_>>();
    let application = TyApplication {
        files: file_indices.clone(),
    };

    // create a node for this application
    let application_index = cc.add_node(application.into());

    // add an edge to every file in this application
    file_indices.iter().for_each(|file_index| {
        cc.add_edge(
            *file_index,
            application_index,
            CollectionEdge::ApplicationContents,
        );
    });
    application_index
}

fn collect_nodes_file(cc: &mut CollectionContext, file: File) -> CollectionIndex {
    let nodes = collect_nodes_nodes(cc, file.nodes);
    let file = TyFile {
        name: file.name,
        nodes: nodes.clone(),
    };

    // create a node for this file
    let file_index = cc.add_node(file.into());

    // add an edge to every (AST) node in file
    nodes.iter().for_each(|node_index| {
        cc.add_edge(*node_index, file_index, CollectionEdge::FileContents);
    });
    file_index
}

fn collect_nodes_nodes(cc: &mut CollectionContext, nodes: Vec<Node>) -> Vec<CollectionIndex> {
    let nodes = nodes
        .into_iter()
        .map(|node| collect_nodes_node(cc, &HashMap::new(), node))
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

fn collect_nodes_node(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    node: Node,
) -> CollectionIndex {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => {
            let decl_index = collect_nodes_declaration(cc, type_mapping, decl);
            let node = TyNode::Declaration(decl_index);
            let node_index = cc.add_node(node.into());
            cc.add_edge(decl_index, node_index, CollectionEdge::NodeContents);
            node_index
        }
        Node::Expression(expression) => {
            let node = TyNode::Expression(collect_nodes_expression(cc, type_mapping, expression));
            cc.add_node(node.into())
        }
        Node::ReturnStatement(expression) => {
            let node =
                TyNode::ReturnStatement(collect_nodes_expression(cc, type_mapping, expression));
            cc.add_node(node.into())
        }
    }
}
