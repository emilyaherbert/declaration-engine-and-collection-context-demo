use std::ops::{Index, IndexMut};

use petgraph::{
    dot::{Config, Dot},
    prelude::EdgeIndex,
};

use crate::declaration_engine::declaration_id::DeclarationId;

use super::{
    bfs, collection_index::CollectionIndex, graph_edge::GraphEdge, graph_node::GraphNode,
    CollectionGraph,
};

#[derive(Default)]
pub(crate) struct CollectionContext {
    pub(crate) graph: CollectionGraph,
}

impl CollectionContext {
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        println!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeIndexLabel])
        );
    }

    pub(crate) fn add_node(&mut self, node: GraphNode) -> CollectionIndex {
        CollectionIndex::new(self.graph.add_node(node))
    }

    pub(crate) fn get_node(&self, index: CollectionIndex) -> &GraphNode {
        self.graph.index(*index)
    }

    pub(crate) fn get_node_mut(&mut self, index: CollectionIndex) -> &mut GraphNode {
        self.graph.index_mut(*index)
    }

    pub(crate) fn add_edge(
        &mut self,
        from: CollectionIndex,
        to: CollectionIndex,
        edge: GraphEdge,
    ) -> EdgeIndex {
        self.graph.add_edge(*from, *to, edge)
    }

    // https://docs.rs/petgraph/latest/src/petgraph/visit/traversal.rs.html#253
    pub(crate) fn get_symbol(
        &self,
        index: CollectionIndex,
        symbol: String,
    ) -> Result<DeclarationId, String> {
        bfs::search_shared_scope_for_declaration(&self.graph, index, symbol)?
            .ok_or_else(|| "could not find symbol in the collection context".to_string())
    }
}
