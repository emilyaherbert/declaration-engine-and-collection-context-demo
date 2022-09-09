use crate::language::ty::{TyFile, TyNode};

pub(crate) enum GraphNode<'gn> {
    File(&'gn TyFile),
    Node(&'gn TyNode),
}

impl<'gn> From<&'gn TyFile> for GraphNode<'gn> {
    fn from(node: &'gn TyFile) -> Self {
        GraphNode::File(node)
    }
}

impl<'gn> From<&'gn TyNode> for GraphNode<'gn> {
    fn from(node: &'gn TyNode) -> Self {
        GraphNode::Node(node)
    }
}
