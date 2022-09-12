use std::fmt;

use crate::language::ty::{
    typed_declaration::TyDeclaration, typed_expression::TyExpression, TyApplication, TyFile, TyNode,
};

pub(crate) enum GraphNode {
    Application(TyApplication),
    File(TyFile),
    Node(TyNode),
    Declaration(TyDeclaration),
    Expression(TyExpression),
}

impl fmt::Display for GraphNode {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphNode::Application(node) => write!(f, "{}", node),
            GraphNode::File(node) => write!(f, "{}", node),
            GraphNode::Node(node) => write!(f, "{}", node),
            GraphNode::Declaration(node) => write!(f, "{}", node),
            GraphNode::Expression(node) => write!(f, "{}", node),
        }
    }
}

impl From<TyApplication> for GraphNode {
    fn from(node: TyApplication) -> Self {
        GraphNode::Application(node)
    }
}

impl From<TyFile> for GraphNode {
    fn from(node: TyFile) -> Self {
        GraphNode::File(node)
    }
}

impl From<TyNode> for GraphNode {
    fn from(node: TyNode) -> Self {
        GraphNode::Node(node)
    }
}

impl From<TyDeclaration> for GraphNode {
    fn from(node: TyDeclaration) -> Self {
        GraphNode::Declaration(node)
    }
}

impl From<TyExpression> for GraphNode {
    fn from(node: TyExpression) -> Self {
        GraphNode::Expression(node)
    }
}
