use std::ops::{Index, IndexMut};

use petgraph::prelude::EdgeIndex;

use super::{collection_index::CollectionIndex, graph_node::GraphNode};

use lazy_static::lazy_static;

lazy_static! {
    static ref COLLECTION_CONTEXT: CollectionContext = CollectionContext::default();
}

type CollectionGraph = petgraph::Graph<GraphNode, ()>;

#[derive(Default)]
struct CollectionContext {
    graph: CollectionGraph,
}

impl CollectionContext {
    fn add_node(&mut self, node: GraphNode) -> CollectionIndex {
        CollectionIndex::new(self.graph.add_node(node))
    }

    fn get_node(&self, index: &CollectionIndex) -> &GraphNode {
        self.graph.index(**index)
    }

    fn get_node_mut(&mut self, index: &CollectionIndex) -> &mut GraphNode {
        self.graph.index_mut(**index)
    }

    fn add_edge(&mut self, from: CollectionIndex, to: CollectionIndex) -> EdgeIndex {
        self.graph.add_edge(*from, *to, ())
    }
}

pub(crate) fn cc_add_node(node: GraphNode) -> CollectionIndex {
    COLLECTION_CONTEXT.add_node(node)
}

pub(crate) fn cc_get_node(index: &CollectionIndex) -> &GraphNode {
    COLLECTION_CONTEXT.get_node(index)
}

pub(crate) fn cc_get_node_mut(index: &CollectionIndex) -> &mut GraphNode {
    COLLECTION_CONTEXT.get_node_mut(index)
}

pub(crate) fn cc_add_edge(from: CollectionIndex, to: CollectionIndex) -> EdgeIndex {
    COLLECTION_CONTEXT.add_edge(from, to)
}
