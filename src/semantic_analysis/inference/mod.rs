mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    declaration_engine::DeclarationEngine,
    language::{Node, Tree, TypedNode, TypedTree},
};

pub(crate) fn analyze(tree: Tree) -> TypedTree {
    let mut declaration_engine = DeclarationEngine::default();
    let new_nodes = analyze_nodes(&mut declaration_engine, tree.nodes);
    TypedTree { nodes: new_nodes }
}

fn analyze_nodes(declaration_engine: &mut DeclarationEngine, nodes: Vec<Node>) -> Vec<TypedNode> {
    nodes
        .into_iter()
        .map(|node| analyze_node(declaration_engine, node))
        .collect()
}

fn analyze_node(declaration_engine: &mut DeclarationEngine, node: Node) -> TypedNode {
    match node {
        Node::Declaration(declaration) => {
            TypedNode::Declaration(analyze_declaration(declaration_engine, declaration))
        }
        Node::Expression(expression) => {
            TypedNode::Expression(analyze_expression(declaration_engine, expression))
        }
        Node::ReturnStatement(expression) => {
            TypedNode::ReturnStatement(analyze_expression(declaration_engine, expression))
        }
    }
}
