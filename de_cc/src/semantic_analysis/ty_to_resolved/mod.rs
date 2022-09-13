mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_index::CollectionIndex,
    },
    language::{
        resolved::{ResolvedApplication, ResolvedFile, ResolvedNode},
        ty::TyNode,
    },
};

pub(crate) fn to_resolved(
    cc: &CollectionContext,
    node_index: &CollectionIndex,
) -> ResolvedApplication {
    let application = cc.get_node(*node_index).expect_application().unwrap();
    let files = application
        .files
        .iter()
        .map(|node_index| to_resolved_file(cc, node_index))
        .collect();
    ResolvedApplication { files }
}

fn to_resolved_file(cc: &CollectionContext, node_index: &CollectionIndex) -> ResolvedFile {
    let file = cc.get_node(*node_index).expect_file().unwrap();
    let new_nodes = to_resolved_nodes(cc, &file.nodes);
    ResolvedFile {
        name: file.name.clone(),
        nodes: new_nodes,
    }
}

fn to_resolved_nodes(cc: &CollectionContext, nodes: &[CollectionIndex]) -> Vec<ResolvedNode> {
    nodes
        .iter()
        .flat_map(|node_index| to_resolved_node(cc, node_index))
        .collect()
}

fn to_resolved_node(cc: &CollectionContext, node_index: &CollectionIndex) -> Vec<ResolvedNode> {
    let node = cc.get_node(*node_index).expect_node().unwrap();
    match node {
        TyNode::Declaration(declaration) => {
            let declarations = to_resolved_declaration(cc, declaration);
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
        }
    }
}
