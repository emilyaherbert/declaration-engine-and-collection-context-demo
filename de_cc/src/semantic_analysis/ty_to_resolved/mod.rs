mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{collection_context::cc_get_node, collection_index::CollectionIndex},
    language::{
        resolved::{ResolvedApplication, ResolvedFile, ResolvedNode},
        ty::{TyApplication, TyFile, TyNode},
    },
};

pub(crate) fn to_resolved(application: &TyApplication) -> ResolvedApplication {
    let files = application
        .files
        .iter()
        .map(|node_index| {
            let node = cc_get_node(node_index);
            let file = node.expect_file().unwrap();
            to_resolved_file(file)
        })
        .collect();
    ResolvedApplication { files }
}

fn to_resolved_file(file: &TyFile) -> ResolvedFile {
    let new_nodes = to_resolved_nodes(&file.nodes);
    ResolvedFile {
        name: file.name.clone(),
        nodes: new_nodes,
    }
}

fn to_resolved_nodes(nodes: &[CollectionIndex]) -> Vec<ResolvedNode> {
    nodes
        .iter()
        .flat_map(|node_index| {
            let node = cc_get_node(node_index);
            let node = node.expect_node().unwrap();
            to_resolved_node(node)
        })
        .collect()
}

fn to_resolved_node(node: &TyNode) -> Vec<ResolvedNode> {
    match node {
        TyNode::Declaration(declaration) => {
            let declarations = to_resolved_declaration(declaration);
            declarations
                .into_iter()
                .map(ResolvedNode::Declaration)
                .collect()
        }
        TyNode::Expression(expression) => {
            vec![ResolvedNode::Expression(to_resolved_expression(expression))]
        }
        TyNode::ReturnStatement(expression) => {
            vec![ResolvedNode::ReturnStatement(to_resolved_expression(
                expression,
            ))]
        } // TypedNode::StarImport(_) => todo!(),
    }
}
