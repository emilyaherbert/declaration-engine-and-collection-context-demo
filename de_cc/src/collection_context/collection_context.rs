use std::ops::Index;

use petgraph::prelude::EdgeIndex;

use super::{collection_index::CollectionIndex, graph_node::GraphNode};

type CollectionGraph = petgraph::Graph<GraphNode, ()>;

#[derive(Default)]
pub(crate) struct CollectionContext {
    graph: CollectionGraph,
}

impl CollectionContext {
    pub(crate) fn add_node(&mut self, node: GraphNode) -> CollectionIndex {
        CollectionIndex::new(self.graph.add_node(node))
    }

    pub(crate) fn get_node(&self, index: &CollectionIndex) -> &GraphNode {
        self.graph.index(**index)
    }

    pub(crate) fn add_edge(&mut self, from: CollectionIndex, to: CollectionIndex) -> EdgeIndex {
        self.graph.add_edge(*from, *to, ())
    }
}
