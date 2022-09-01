mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    language::{
        typed::{TypedApplication, TypedFile, TypedNode},
        untyped::{Application, File, Node},
    },
    namespace::namespace::Namespace,
};

pub(crate) fn analyze(namespace: &mut Namespace, application: Application) -> TypedApplication {
    let typed_programs = application
        .files
        .into_iter()
        .map(|program| analyze_file(namespace, program))
        .collect();
    TypedApplication {
        files: typed_programs,
    }
}

fn analyze_file(namespace: &mut Namespace, file: File) -> TypedFile {
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

fn analyze_node(namespace: &mut Namespace, node: Node) -> TypedNode {
    match node {
        Node::Declaration(declaration) => {
            TypedNode::Declaration(analyze_declaration(namespace, declaration))
        }
        Node::Expression(expression) => {
            TypedNode::Expression(analyze_expression(namespace, expression))
        }
        Node::ReturnStatement(expression) => {
            TypedNode::ReturnStatement(analyze_expression(namespace, expression))
        }
        Node::StarImport(_) => todo!(),
    }
}
