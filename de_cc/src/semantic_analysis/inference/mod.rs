mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    collection_context::collection_context::cc_get_node,
    language::ty::{TyApplication, TyFile, TyNode},
    namespace::namespace::Namespace,
};

pub(crate) fn analyze(namespace: &mut Namespace, application: &TyApplication) {
    application.files.iter().for_each(|node_index| {
        let node = cc_get_node(node_index);
        let file = node.expect_file().unwrap();
        analyze_file(namespace, file)
    });
}

fn analyze_file(namespace: &mut Namespace, file: &TyFile) {
    file.nodes.iter().for_each(|node_index| {
        let node = cc_get_node(node_index);
        let node = node.expect_node().unwrap();
        analyze_node(namespace, node)
    });
}

fn analyze_node(namespace: &mut Namespace, node: &TyNode) {
    match node {
        TyNode::Declaration(declaration) => analyze_declaration(namespace, declaration),
        TyNode::Expression(expression) => analyze_expression(namespace, expression),
        TyNode::ReturnStatement(expression) => analyze_expression(namespace, expression),
    }
}
