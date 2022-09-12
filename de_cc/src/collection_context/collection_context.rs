use std::{
    ops::{Index, IndexMut},
    sync::RwLock,
};

use owning_ref::{RwLockReadGuardRef, RwLockWriteGuardRefMut};
use petgraph::prelude::EdgeIndex;

use super::{collection_index::CollectionIndex, graph_node::GraphNode};

use lazy_static::lazy_static;

lazy_static! {
    static ref COLLECTION_CONTEXT: CollectionContext = CollectionContext::default();
}

type CollectionGraph = petgraph::Graph<GraphNode, ()>;

#[derive(Default)]
struct CollectionContext {
    graph: RwLock<CollectionGraph>,
}

impl CollectionContext {
    fn add_node(&self, node: GraphNode) -> CollectionIndex {
        let mut graph = self.graph.write().unwrap();
        CollectionIndex::new(graph.add_node(node))
    }

    fn get_node(
        &self,
        index: &CollectionIndex,
    ) -> RwLockReadGuardRef<'_, CollectionGraph, GraphNode> {
        RwLockReadGuardRef::new(self.graph.read().unwrap()).map(|graph| graph.index(**index))
    }

    fn get_node_mut(
        &self,
        index: &CollectionIndex,
    ) -> RwLockWriteGuardRefMut<'_, CollectionGraph, GraphNode> {
        RwLockWriteGuardRefMut::new(self.graph.write().unwrap())
            .map_mut(|graph| graph.index_mut(**index))
    }

    fn add_edge(&self, from: CollectionIndex, to: CollectionIndex) -> EdgeIndex {
        let mut graph = self.graph.write().unwrap();
        graph.add_edge(*from, *to, ())
    }
}

pub(crate) fn cc_add_node(node: GraphNode) -> CollectionIndex {
    COLLECTION_CONTEXT.add_node(node)
}

pub(crate) fn cc_get_node(
    index: &CollectionIndex,
) -> RwLockReadGuardRef<'_, CollectionGraph, GraphNode> {
    COLLECTION_CONTEXT.get_node(index)
}

pub(crate) fn cc_get_node_mut(
    index: &CollectionIndex,
) -> RwLockWriteGuardRefMut<'_, CollectionGraph, GraphNode> {
    COLLECTION_CONTEXT.get_node_mut(index)
}

pub(crate) fn cc_add_edge(from: CollectionIndex, to: CollectionIndex) -> EdgeIndex {
    COLLECTION_CONTEXT.add_edge(from, to)
}
