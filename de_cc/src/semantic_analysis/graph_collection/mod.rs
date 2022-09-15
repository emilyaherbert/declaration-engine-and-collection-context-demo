mod declaration;
mod expression;

use std::collections::HashMap;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_edge::CollectionEdge,
        collection_index::CCIdx,
    },
    language::{
        parsed::{Application, File, Node},
        ty::{TyApplication, TyFile, TyNode},
    },
    type_system::type_mapping::TypeMapping,
};

pub(crate) fn collect_graph(cc: &mut CollectionContext, app: Application) -> CCIdx<TyApplication> {
    // create graph nodes for each of the files
    let file_idxs = app
        .files
        .into_iter()
        .map(|file| collect_graph_file(cc, file))
        .collect::<Vec<_>>();

    // create a graph node for this application
    let app = TyApplication {
        files: file_idxs.clone(),
    };
    let app_idx = cc.add_node(app.clone().into());
    let cc_idx = CCIdx::new(app, app_idx);

    // add a graph edge from every file to the application
    CCIdx::add_edges_many_to_one(&file_idxs, &cc_idx, CollectionEdge::ApplicationContents, cc);

    cc_idx
}

fn collect_graph_file(cc: &mut CollectionContext, file: File) -> CCIdx<TyFile> {
    // create graph nodes for the nodes
    let nodes = collect_graph_nodes(cc, file.nodes);

    // create a graph node for this file
    let file = TyFile {
        name: file.name,
        nodes: nodes.clone(),
    };
    let file_idx = cc.add_node(file.clone().into());
    let cc_idx = CCIdx::new(file, file_idx);

    // add a graph edge from every ast node to the file
    CCIdx::add_edges_many_to_one(&nodes, &cc_idx, CollectionEdge::FileContents, cc);

    cc_idx
}

fn collect_graph_nodes(cc: &mut CollectionContext, nodes: Vec<Node>) -> Vec<CCIdx<TyNode>> {
    // create graph nodes for each of the ast nodes
    let nodes = nodes
        .into_iter()
        .map(|node| collect_graph_node(cc, &HashMap::new(), node))
        .collect::<Vec<_>>();

    // for every ast node in this scope, connect them under the same shared scope with graph edges
    CCIdx::add_edges_many(&nodes, CollectionEdge::SharedScope, cc);

    nodes
}

fn collect_graph_node(
    cc: &mut CollectionContext,
    type_mapping: &TypeMapping,
    node: Node,
) -> CCIdx<TyNode> {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => {
            let decl_cc_idx = collect_graph_decl(cc, type_mapping, decl);
            let node = TyNode::Declaration(decl_cc_idx.clone());
            let node_idx = cc.add_node(node.clone().into());
            let node_cc_idx = CCIdx::new(node, node_idx);
            // connect from the inside of the node
            CCIdx::add_edge(&decl_cc_idx, &node_cc_idx, CollectionEdge::ScopedChild, cc);
            node_cc_idx
        }
        Node::Expression(expression) => {
            let exp = collect_graph_exp(cc, expression);
            let node = TyNode::Expression(exp);
            let node_idx = cc.add_node(node.clone().into());
            CCIdx::new(node, node_idx)
        }
        Node::ReturnStatement(expression) => {
            let exp = collect_graph_exp(cc, expression);
            let node = TyNode::ReturnStatement(exp);
            let node_idx = cc.add_node(node.clone().into());
            CCIdx::new(node, node_idx)
        }
    }
}
