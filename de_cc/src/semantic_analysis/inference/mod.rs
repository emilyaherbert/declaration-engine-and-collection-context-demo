mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    language::ty::{TyApplication, TyFile, TyNode},
    namespace::namespace::Namespace,
};

pub(crate) fn analyze(namespace: &mut Namespace, application: &TyApplication) {
    application
        .files
        .iter()
        .for_each(|program| analyze_file(namespace, program));
}

fn analyze_file(namespace: &mut Namespace, file: &TyFile) {
    file.nodes
        .iter()
        .for_each(|node| analyze_node(namespace, node));
}

fn analyze_node(namespace: &mut Namespace, node: &TyNode) {
    match node {
        TyNode::Declaration(declaration) => analyze_declaration(namespace, declaration),
        TyNode::Expression(expression) => analyze_expression(namespace, expression),
        TyNode::ReturnStatement(expression) => analyze_expression(namespace, expression),
    }
}
