use crate::types::with_collection_context::*;
use crate::CollectionContext;

use petgraph::prelude::NodeIndex;

#[derive(Clone, WithCC)]
pub(crate) struct CollectionIndex(NodeIndex);

impl std::ops::Deref for CollectionIndex {
    type Target = NodeIndex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DebugWithCC for CollectionIndex {
    fn fmt_with_cc(&self, f: &mut std::fmt::Formatter, cc: &CollectionContext) -> std::fmt::Result {
        todo!()
        // let w = cc.get_node(self).with_cc(cc);
        // write!(f, "{:?}", w)
    }
}

impl CollectionIndex {
    pub(crate) fn new(index: NodeIndex) -> CollectionIndex {
        CollectionIndex(index)
    }
}
