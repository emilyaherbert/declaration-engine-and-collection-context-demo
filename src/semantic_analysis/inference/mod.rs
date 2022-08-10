mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    declaration_engine::DeclarationEngine,
    language::{Node, Tree, TypedNode, TypedTree},
    type_system::TypeEngine,
};

pub(crate) fn analyze(tree: Tree) -> TypedTree {
    let mut type_engine = TypeEngine::default();
    let mut declaration_engine = DeclarationEngine::default();
    let new_nodes = analyze_nodes(&mut type_engine, &mut declaration_engine, tree.nodes);
    TypedTree { nodes: new_nodes }
}

fn analyze_nodes(
    type_engine: &mut TypeEngine,
    declaration_engine: &mut DeclarationEngine,
    nodes: Vec<Node>,
) -> Vec<TypedNode> {
    nodes
        .into_iter()
        .map(|node| analyze_node(type_engine, declaration_engine, node))
        .collect()
}

fn analyze_node(
    type_engine: &mut TypeEngine,
    declaration_engine: &mut DeclarationEngine,
    node: Node,
) -> TypedNode {
    match node {
        Node::Declaration(declaration) => TypedNode::Declaration(analyze_declaration(
            type_engine,
            declaration_engine,
            declaration,
        )),
        Node::Expression(expression) => TypedNode::Expression(analyze_expression(
            type_engine,
            declaration_engine,
            expression,
        )),
        Node::ReturnStatement(expression) => TypedNode::ReturnStatement(analyze_expression(
            type_engine,
            declaration_engine,
            expression,
        )),
    }
}
