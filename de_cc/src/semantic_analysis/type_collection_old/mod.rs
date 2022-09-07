mod declaration;

use declaration::*;

use crate::{
    language::{
        typed::{TypedApplication, TypedFile, TypedNode},
    },
    namespace::namespace::Namespace,
};

pub(crate) fn collect_types(
    namespace: &mut Namespace,
    application: &TypedApplication,
) {
    application
        .files
        .into_iter()
        .for_each(|file| collect_types_file(namespace, file));
}

fn collect_types_file(namespace: &mut Namespace, file: TypedFile) {
    file
        .nodes
        .into_iter()
        .for_each(|node| collect_types_node(namespace, node));
}

fn collect_types_node(namespace: &mut Namespace, node: TypedNode) {
    match node {
        TypedNode::Declaration(decl) => collect_types_declaration(namespace, decl),
        TypedNode::Expression(exp) => TypedNode::Expression(exp),
        TypedNode::ReturnStatement(exp) => TypedNode::ReturnStatement(exp),
    }
}
