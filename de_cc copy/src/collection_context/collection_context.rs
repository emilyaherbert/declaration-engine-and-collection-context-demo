use std::ops::{Index, IndexMut};

use petgraph::dot::{Config, Dot};

use crate::declaration_engine::declaration_id::DeclarationId;

use super::{
    bfs, collection_edge::CollectionEdge, collection_index::CollectionIndex,
    collection_node::CollectionNode, CollectionGraph,
};

#[derive(Default, Clone)]
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

    pub(crate) fn add_node(&mut self, node: CollectionNode) -> CollectionIndex {
        CollectionIndex::new(self.graph.add_node(node))
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
        let decls_in_scope = bfs::get_all_declarations_in_scope(&self.graph, index)?;
        for (name, decl_id) in decls_in_scope.into_iter() {
            if name == symbol {
                return Ok(*decl_id)
            }
        }
        Err("symbol not found in scope".to_string())
    }
}
