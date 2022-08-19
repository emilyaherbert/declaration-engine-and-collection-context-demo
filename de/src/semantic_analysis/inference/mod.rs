mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::{TypedApplication, TypedFile, TypedNode},
        untyped::{Application, File, Node},
    },
    namespace::namespace::Namespace,
};

pub(crate) fn analyze(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    application: Application,
) -> TypedApplication {
    let typed_programs = application
        .files
        .into_iter()
        .map(|program| analyze_file(namespace, declaration_engine, program))
        .collect();
    TypedApplication {
        files: typed_programs,
    }
}

fn analyze_file(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    file: File,
) -> TypedFile {
    let new_nodes = file
        .nodes
        .into_iter()
        .map(|node| analyze_node(namespace, declaration_engine, node))
        .collect::<Vec<_>>();
    TypedFile {
        name: file.name,
        nodes: new_nodes,
    }
}

fn analyze_node(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    node: Node,
) -> TypedNode {
    match node {
        Node::Declaration(declaration) => TypedNode::Declaration(analyze_declaration(
            namespace,
            declaration_engine,
            declaration,
        )),
        Node::Expression(expression) => TypedNode::Expression(analyze_expression(
            namespace,
            declaration_engine,
            expression,
        )),
        Node::ReturnStatement(expression) => TypedNode::ReturnStatement(analyze_expression(
            namespace,
            declaration_engine,
            expression,
        )),
        Node::StarImport(_) => todo!(),
    }
}
