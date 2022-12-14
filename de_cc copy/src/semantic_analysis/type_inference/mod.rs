mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_index::CollectionIndex,
    },
    language::ty::TyNode,
    namespace::namespace::Namespace,
};

pub(crate) fn analyze(cc: &CollectionContext, ns: &mut Namespace, node_index: CollectionIndex) {
    let application = cc.get_node(node_index).expect_application().unwrap();
    application
        .files
        .clone()
        .into_iter()
        .for_each(|node_index| analyze_file(cc, ns, node_index));
}

fn analyze_file(cc: &CollectionContext, ns: &mut Namespace, node_index: CollectionIndex) {
    let file = cc.get_node(node_index).expect_file().unwrap();
    file.nodes
        .clone()
        .into_iter()
        .for_each(|node_index| analyze_node(cc, ns, node_index));
}

fn analyze_node(cc: &CollectionContext, ns: &mut Namespace, node_index: CollectionIndex) {
    let node = cc.get_node(node_index).expect_node().unwrap();
    match node {
        TyNode::Declaration(node_index) => analyze_declaration(cc, ns, *node_index),
        TyNode::Expression(expression) => analyze_expression(cc, node_index, ns, expression),
        TyNode::ReturnStatement(expression) => analyze_expression(cc, node_index, ns, expression),
    }
}
