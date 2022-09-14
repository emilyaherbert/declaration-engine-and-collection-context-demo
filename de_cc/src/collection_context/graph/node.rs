use super::edge::EdgeIndex;

use std::ops::Deref;

// https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#102
#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) struct NodeIndex(usize);

impl Deref for NodeIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NodeIndex {
    // https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#106
    pub(super) fn new(x: usize) -> NodeIndex {
        NodeIndex(x)
    }

    // https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#116
    pub(super) fn end() -> NodeIndex {
        NodeIndex(usize::MAX)
    }

    pub(super) fn max(a: NodeIndex, b: NodeIndex) -> NodeIndex {
        if *a > *b {
            a
        } else {
            b
        }
    }
}

// https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#217
#[derive(Clone)]
pub(super) struct Node<N> {
    pub(super) weight: N,
    pub(super) next: [EdgeIndex; 2],
}
