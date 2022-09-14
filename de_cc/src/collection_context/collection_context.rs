use std::collections::HashMap;

use petgraph::dot::{Config, Dot};

use crate::declaration_engine::declaration_id::DeclarationId;

use super::{
    bfs,
    collection_edge::CollectionEdge,
    collection_index::CollectionIndex,
    collection_node::CollectionNode,
    graph::{graph::Graph, node::NodeIndex},
};

#[derive(Default, Clone)]
pub(crate) struct CollectionContext {
    bypass_map: HashMap<CollectionIndex, NodeIndex>,
    pub(crate) graph: Graph<CollectionNode, CollectionEdge>,
}

impl CollectionContext {
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        println!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeIndexLabel])
        );
    }

    pub(crate) fn add_node(&mut self, node: CollectionNode) -> Result<CollectionIndex, String> {
        Ok(CollectionIndex::new(self.graph.add_node(node)?))
    }

    pub(crate) fn get_node(&self, index: CollectionIndex) -> &CollectionNode {
        self.graph.index(*index)
    }

    pub(crate) fn get_node_mut(&mut self, index: CollectionIndex) -> &mut CollectionNode {
        self.graph.index_mut(*index)
    }

    pub(crate) fn add_edge(
        &mut self,
        from: CollectionIndex,
        to: CollectionIndex,
        edge: CollectionEdge,
    ) {
        self.graph.add_edge(*from, *to, edge);
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
