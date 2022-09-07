mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    language::{
        partial::{PartialApplication, PartialFile, PartialNode},
        typed::{TypedApplication, TypedFile, TypedNode},
    },
    namespace::namespace::Namespace,
};

pub(crate) fn analyze(
    namespace: &mut Namespace,
    application: PartialApplication,
) -> TypedApplication {
    let typed_programs = application
        .files
        .into_iter()
        .map(|program| analyze_file(namespace, program))
        .collect();
    TypedApplication {
        files: typed_programs,
    }
}

fn analyze_file(namespace: &mut Namespace, file: PartialFile) -> TypedFile {
    let new_nodes = file
        .nodes
        .into_iter()
        .map(|node| analyze_node(namespace, node))
        .collect::<Vec<_>>();
    TypedFile {
        name: file.name,
        nodes: new_nodes,
    }
}

fn analyze_node(namespace: &mut Namespace, node: PartialNode) -> TypedNode {
    match node {
        PartialNode::Declaration(declaration) => {
            TypedNode::Declaration(analyze_declaration(namespace, declaration))
        }
        PartialNode::Expression(expression) => {
            TypedNode::Expression(analyze_expression(namespace, expression))
        }
        PartialNode::ReturnStatement(expression) => {
            TypedNode::ReturnStatement(analyze_expression(namespace, expression))
        }
    }
}
