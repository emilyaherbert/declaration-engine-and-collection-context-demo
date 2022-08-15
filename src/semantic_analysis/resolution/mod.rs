mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        resolved::{ResolvedNode, ResolvedTree},
        typed::{TypedNode, TypedTree},
    },
};

pub(crate) fn resolve(tree: TypedTree) -> ResolvedTree {
    let mut declaration_engine = DeclarationEngine::default();
    let new_nodes = resolve_nodes(&mut declaration_engine, tree.nodes);
    ResolvedTree { nodes: new_nodes }
}

fn resolve_nodes(
    declaration_engine: &mut DeclarationEngine,
    nodes: Vec<TypedNode>,
) -> Vec<ResolvedNode> {
    nodes
        .into_iter()
        .map(|node| resolve_node(declaration_engine, node))
        .collect()
}

fn resolve_node(declaration_engine: &mut DeclarationEngine, node: TypedNode) -> ResolvedNode {
    match node {
        TypedNode::Declaration(declaration) => {
            ResolvedNode::Declaration(resolve_declaration(declaration_engine, declaration))
        }
        TypedNode::Expression(expression) => {
            ResolvedNode::Expression(resolve_expression(declaration_engine, expression))
        }
        TypedNode::ReturnStatement(expression) => {
            ResolvedNode::ReturnStatement(resolve_expression(declaration_engine, expression))
        }
    }
}
