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

pub(crate) fn collect_graph(
    cc: &mut CollectionContext,
    application: Application,
) -> CollectionIndex {
    // create graph nodes for each of the files
    let file_idxs = application
        .files
        .into_iter()
        .map(|file| collect_graph_file(cc, file))
        .collect::<Vec<_>>();

    // create a graph node for this application
    let application = TyApplication {
        files: file_idxs.clone(),
    };
    let application_idx = cc.add_node(application.into());

    // add a graph edge from every file to the appliction
    file_idxs.iter().for_each(|file_idx| {
        cc.add_edge(
            *file_idx,
            application_idx,
            CollectionEdge::ApplicationContents,
        );
    });

    application_idx
}

fn collect_graph_file(cc: &mut CollectionContext, file: File) -> CollectionIndex {
    // create graph nodes for the nodes
    let nodes = collect_graph_nodes(cc, file.nodes);

    // create a graph node for this file
    let file = TyFile {
        name: file.name,
        nodes: nodes.clone(),
    };
    let file_idx = cc.add_node(file.into());

    // add a graph edge from every ast node to the file
    nodes.iter().for_each(|node_idx| {
        cc.add_edge(*node_idx, file_idx, CollectionEdge::FileContents);
    });

    file_idx
}

fn collect_graph_nodes(cc: &mut CollectionContext, nodes: Vec<Node>) -> Vec<CollectionIndex> {
    // create graph nodes for each of the ast nodes
    let nodes = nodes
        .into_iter()
        .map(|node| collect_graph_node(cc, &HashMap::new(), node))
        .collect::<Vec<_>>();

    // for every ast node in this scope, connect them under the same shared scope with graph edges
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

fn collect_graph_node(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    node: Node,
) -> CollectionIndex {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => {
            // create a graph node for the declaration
            let decl_idx = collect_graph_declaration(cc, type_mapping, decl);

            // create a graph node for the ast node
            let node_idx = cc.add_node(TyNode::Declaration(decl_idx).into());

            // add a graph edge from the declaration to the ast node
            cc.add_edge(decl_idx, node_idx, CollectionEdge::NodeContents);
            node_idx
        }
        Node::Expression(expression) => {
            let node = TyNode::Expression(collect_graph_expression(cc, type_mapping, expression));
            cc.add_node(node.into())
        }
        Node::ReturnStatement(expression) => {
            let node =
                TyNode::ReturnStatement(collect_graph_expression(cc, type_mapping, expression));
            cc.add_node(node.into())
        }
    }
}
