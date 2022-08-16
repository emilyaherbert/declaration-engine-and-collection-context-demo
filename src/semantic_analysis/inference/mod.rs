mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    collection_context::collection_context::CollectionContext,
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::{TypedApplication, TypedFile, TypedNode},
        untyped::{Application, File, Node},
    },
    namespace::namespace::Namespace,
};

pub(crate) fn analyze(
    namespace: &mut Namespace,
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    application: Application,
) -> TypedApplication {
    let typed_programs = application
        .files
        .into_iter()
        .map(|program| analyze_file(namespace, collection_context, declaration_engine, program))
        .collect();
    TypedApplication {
        files: typed_programs,
    }
}

fn analyze_file(
    namespace: &mut Namespace,
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    file: File,
) -> TypedFile {
    let new_nodes = analyze_nodes(
        namespace,
        collection_context,
        declaration_engine,
        file.nodes,
    );
    TypedFile {
        name: file.name,
        nodes: new_nodes,
    }
}

fn analyze_nodes(
    namespace: &mut Namespace,
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    nodes: Vec<Node>,
) -> Vec<TypedNode> {
    nodes
        .into_iter()
        .map(|node| analyze_node(namespace, collection_context, declaration_engine, node))
        .collect()
}

fn analyze_node(
    namespace: &mut Namespace,
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    node: Node,
) -> TypedNode {
    match node {
        Node::Declaration(declaration) => TypedNode::Declaration(analyze_declaration(
            namespace,
            collection_context,
            declaration_engine,
            declaration,
        )),
        Node::Expression(expression) => TypedNode::Expression(analyze_expression(
            namespace,
            collection_context,
            declaration_engine,
            expression,
        )),
        Node::ReturnStatement(expression) => TypedNode::ReturnStatement(analyze_expression(
            namespace,
            collection_context,
            declaration_engine,
            expression,
        )),
        Node::StarImport(_) => todo!(),
    }
}
