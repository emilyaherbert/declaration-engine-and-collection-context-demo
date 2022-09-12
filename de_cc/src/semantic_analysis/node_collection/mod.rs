mod declaration;
mod expression;

use std::collections::HashMap;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{collection_context::*, collection_index::CollectionIndex},
    language::{
        parsed::{Application, File, Node},
        ty::{TyApplication, TyFile, TyNode},
    },
    type_system::type_mapping::TypeMapping,
};

pub(crate) fn collect_nodes(application: Application) {
    let file_indices = application
        .files
        .into_iter()
        .map(|file| collect_nodes_file(file))
        .collect::<Vec<_>>();
    let application = TyApplication {
        files: file_indices,
    };
    let application_index = cc_add_node(application.into());
    application.files.iter().for_each(|file_index| {
        cc_add_edge(application_index, *file_index);
    });
}

fn collect_nodes_file(file: File) -> CollectionIndex {
    let file = TyFile {
        name: file.name,
        nodes: collect_nodes_nodes(file.nodes),
    };
    let file_index = cc_add_node(file.into());
    file.nodes.iter().for_each(|node_index| {
        cc_add_edge(file_index, *node_index);
    });
    file_index
}

fn collect_nodes_nodes(nodes: Vec<Node>) -> Vec<CollectionIndex> {
    let type_mapping = HashMap::new();
    nodes
        .into_iter()
        .map(|node| collect_nodes_node(&type_mapping, node))
        .collect()
}

fn collect_nodes_node(type_mapping: &TypeMapping, node: Node) -> CollectionIndex {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => {
            let node = TyNode::Declaration(collect_nodes_declaration(type_mapping, decl));
            cc_add_node(node.into())
        }
        Node::Expression(expression) => {
            let node = TyNode::Expression(collect_nodes_expression(type_mapping, expression));
            cc_add_node(node.into())
        }
        Node::ReturnStatement(expression) => {
            let node = TyNode::ReturnStatement(collect_nodes_expression(type_mapping, expression));
            cc_add_node(node.into())
        }
    }
}
