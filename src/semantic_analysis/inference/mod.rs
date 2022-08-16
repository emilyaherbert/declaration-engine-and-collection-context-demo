mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::{TypedNode, TypedTree},
        untyped::{Node, Tree},
    },
    namespace::Namespace,
};

pub(crate) fn analyze(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    tree: Tree,
) -> TypedTree {
    let new_nodes = analyze_nodes(namespace, declaration_engine, tree.nodes);
    TypedTree { nodes: new_nodes }
}

fn analyze_nodes(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    nodes: Vec<Node>,
) -> Vec<TypedNode> {
    nodes
        .into_iter()
        .map(|node| analyze_node(namespace, declaration_engine, node))
        .collect()
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
    }
}
