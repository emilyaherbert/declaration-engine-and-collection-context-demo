mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{collection_context::CollectionContext, collection_index::CCIdx},
    language::{
        resolved::{ResolvedApplication, ResolvedFile, ResolvedNode},
        ty::{TyApplication, TyFile, TyNode},
    },
};

pub(crate) fn to_resolved(
    cc: &CollectionContext,
    application: CCIdx<TyApplication>,
) -> ResolvedApplication {
    let files = application
        .inner()
        .files
        .into_iter()
        .map(|file| to_resolved_file(cc, file))
        .collect();
    ResolvedApplication { files }
}

fn to_resolved_file(cc: &CollectionContext, file: CCIdx<TyFile>) -> ResolvedFile {
    let TyFile { name, nodes } = file.inner();
    let new_nodes = to_resolved_nodes(cc, nodes);
    ResolvedFile {
        name,
        nodes: new_nodes,
    }
}

fn to_resolved_nodes(cc: &CollectionContext, nodes: Vec<CCIdx<TyNode>>) -> Vec<ResolvedNode> {
    nodes
        .into_iter()
        .flat_map(|node| to_resolved_node(cc, node))
        .collect()
}

fn to_resolved_node(cc: &CollectionContext, node: CCIdx<TyNode>) -> Vec<ResolvedNode> {
    match node.inner() {
        TyNode::Declaration(decl) => {
            let declarations = to_resolved_declaration(cc, decl);
            declarations
                .into_iter()
                .map(ResolvedNode::Declaration)
                .collect()
        }
        TyNode::Expression(exp) => {
            vec![ResolvedNode::Expression(to_resolved_expression(exp))]
        }
        TyNode::ReturnStatement(exp) => {
            vec![ResolvedNode::ReturnStatement(to_resolved_expression(exp))]
        }
    }
}
