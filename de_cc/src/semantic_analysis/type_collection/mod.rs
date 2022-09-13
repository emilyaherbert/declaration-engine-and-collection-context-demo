mod declaration;

use declaration::*;

use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_index::CollectionIndex,
    },
    language::ty::TyNode,
    namespace::namespace::Namespace,
};

pub(crate) fn collect_types(
    cc: &CollectionContext,
    namespace: &mut Namespace,
    node_index: &CollectionIndex,
) {
    let application = cc.get_node(*node_index).expect_application().unwrap();
    application
        .files
        .iter()
        .for_each(|node_index| collect_types_file(cc, namespace, node_index));
}

fn collect_types_file(
    cc: &CollectionContext,
    namespace: &mut Namespace,
    node_index: &CollectionIndex,
) {
    let file = cc.get_node(*node_index).expect_file().unwrap();
    file.nodes
        .iter()
        .for_each(|node_index| collect_types_node(cc, namespace, node_index));
}

fn collect_types_node(
    cc: &CollectionContext,
    namespace: &mut Namespace,
    node_index: &CollectionIndex,
) {
    let node = cc.get_node(*node_index).expect_node().unwrap();
    match node {
        TyNode::Declaration(node_index) => collect_types_declaration(cc, namespace, node_index),
        TyNode::Expression(_) => {}
        TyNode::ReturnStatement(_) => {}
    }
}
