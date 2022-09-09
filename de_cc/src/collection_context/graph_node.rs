use crate::CollectionContext;
use crate::{
    language::ty::{
        typed_declaration::TyDeclaration, typed_expression::TyExpression, TyApplication, TyFile,
        TyNode,
    },
    types::with_collection_context::DebugWithCC,
};

#[derive(DebugWithCC)]
pub(crate) enum GraphNode {
    Application(TyApplication),
    File(TyFile),
    Node(TyNode),
    Declaration(TyDeclaration),
    Expression(TyExpression),
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
