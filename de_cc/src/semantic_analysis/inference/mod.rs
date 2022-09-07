mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    language::typed::{TypedApplication, TypedFile, TypedNode},
    namespace::namespace::Namespace,
};

pub(crate) fn analyze(namespace: &mut Namespace, application: &TypedApplication) {
    application
        .files
        .iter()
        .for_each(|program| analyze_file(namespace, program));
}

fn analyze_file(namespace: &mut Namespace, file: &TypedFile) {
    file.nodes
        .iter()
        .for_each(|node| analyze_node(namespace, node));
}

fn analyze_node(namespace: &mut Namespace, node: &TypedNode) {
    match node {
        TypedNode::Declaration(declaration) => analyze_declaration(namespace, declaration),
        TypedNode::Expression(expression) => analyze_expression(namespace, expression),
        TypedNode::ReturnStatement(expression) => analyze_expression(namespace, expression),
    }
}
