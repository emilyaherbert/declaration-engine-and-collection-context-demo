mod declaration;

use declaration::*;

use crate::{
    collection_context::collection_context::cc_get_node,
    language::ty::{TyApplication, TyFile, TyNode},
    namespace::namespace::Namespace,
};

pub(crate) fn collect_types(namespace: &mut Namespace, application: &TyApplication) {
    application.files.iter().for_each(|node_index| {
        let node = cc_get_node(node_index);
        let file = node.expect_file().unwrap();
        collect_types_file(namespace, file)
    });
}

fn collect_types_file(namespace: &mut Namespace, file: &TyFile) {
    file.nodes.iter().for_each(|node_index| {
        let node = cc_get_node(node_index);
        let node = node.expect_node().unwrap();
        collect_types_node(namespace, node)
    });
}

fn collect_types_node(namespace: &mut Namespace, node: &TyNode) {
    match node {
        TyNode::Declaration(decl) => collect_types_declaration(namespace, decl),
        TyNode::Expression(_) => {}
        TyNode::ReturnStatement(_) => {}
    }
}
