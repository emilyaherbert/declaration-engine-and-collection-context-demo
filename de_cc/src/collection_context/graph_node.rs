use crate::language::ty::{TyFile, TyNode};

pub(crate) enum GraphNode<'n> {
    File(&'n TyFile),
    Node(&'n TyNode),
}

impl<'n> From<&'n TyFile> for GraphNode<'n> {
    fn from(node: &'n TyFile) -> Self {
        GraphNode::File(node)
    }
}

impl<'n> From<&'n TyNode> for GraphNode<'n> {
    fn from(node: &'n TyNode) -> Self {
        GraphNode::Node(node)
    }
}
