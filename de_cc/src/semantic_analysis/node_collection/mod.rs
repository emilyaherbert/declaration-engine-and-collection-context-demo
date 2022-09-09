mod declaration;

use declaration::*;
use petgraph::prelude::NodeIndex;

use crate::{
    collection_context::collection_context::CollectionContext,
    language::ty::{TyApplication, TyFile, TyNode},
};

pub(crate) fn collect_nodes(collection_ctxt: &mut CollectionContext, application: &TyApplication) {
    application
        .files
        .iter()
        .for_each(|file| collect_nodes_file(collection_ctxt, file));
}

fn collect_nodes_file(collection_ctxt: &mut CollectionContext, file: &TyFile) {
    let file_node = collection_ctxt.add_node(file.into());
    collect_nodes_nodes(collection_ctxt, &file.nodes);
}

fn collect_nodes_nodes(
    collection_ctxt: &mut CollectionContext,
    file_node: NodeIndex,
    nodes: &[TyNode],
) {
    nodes.iter().for_each(|node| {
        let node_node = collection_ctxt.add_node(node.into());
        collection_ctxt.add_edge(file_node, node_node);
        collect_nodes_node(collection_ctxt, node)
    });
}

fn collect_nodes_node(collection_ctxt: &mut CollectionContext, node: &TyNode) {
    match node {
        TyNode::Declaration(decl) => collect_nodes_declaration(collection_ctxt, decl),
        TyNode::Expression(_) => {}
        TyNode::ReturnStatement(_) => {}
    }
}
