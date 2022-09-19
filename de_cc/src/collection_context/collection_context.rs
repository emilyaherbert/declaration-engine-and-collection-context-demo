use std::{collections::HashMap, ops::Index};

use petgraph::{
    dot::{Config, Dot},
    prelude::EdgeIndex,
};

use crate::declaration_engine::declaration_id::DeclarationId;

use super::{
    bfs,
    collection_edge::CollectionEdge,
    collection_index::{CCIdx, CollectionIndex},
    collection_node::CollectionNode,
    CollectionGraph,
};

#[derive(Default, Clone)]
pub(crate) struct CollectionContext {
    pub(super) graph: CollectionGraph,
    pub(super) files: HashMap<String, CollectionIndex>,
}

impl CollectionContext {
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        println!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeIndexLabel])
        );
    }

    pub(crate) fn create_link(&self) {
        let dot_str = format!(
            "https://dreampuf.github.io/GraphvizOnline/#{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeIndexLabel])
        )
        .replace('\n', "%0A")
        .replace('=', "%3D")
        .replace("   ", "%20%20%20");
        let s = std::str::from_utf8(dot_str.as_bytes()).unwrap();
        println!("{}", s);
    }

    pub(crate) fn register_file_index(&mut self, filename: String, index: CollectionIndex) {
        self.files.insert(filename, index);
    }

    pub(crate) fn get_file_index(&self, filename: String) -> Result<CollectionIndex, String> {
        self.files
            .get(&filename)
            .cloned()
            .ok_or_else(|| "file not in file list".to_string())
    }

    pub(crate) fn add_node(&mut self, node: CollectionNode) -> CollectionIndex {
        CollectionIndex::new(self.graph.add_node(node))
    }

    pub(crate) fn get_node(&self, index: CollectionIndex) -> &CollectionNode {
        self.graph.index(*index)
    }

    // pub(crate) fn get_node_mut(&mut self, index: CollectionIndex) -> &mut CollectionNode {
    //     self.graph.index_mut(*index)
    // }

    pub(crate) fn add_edge(
        &mut self,
        from: CollectionIndex,
        to: CollectionIndex,
        edge: CollectionEdge,
    ) -> EdgeIndex {
        self.graph.add_edge(*from, *to, edge)
    }

    // https://docs.rs/petgraph/latest/src/petgraph/visit/traversal.rs.html#253
    pub(crate) fn get_symbol(
        &self,
        index: CollectionIndex,
        symbol: &str,
    ) -> Result<CCIdx<DeclarationId>, String> {
        let decls_in_scope = bfs::get_all_declarations_in_scope(self, index)?;
        for (name, decl_id) in decls_in_scope.into_iter() {
            if name == symbol {
                return Ok(decl_id);
            }
        }
        Err(format!("symbol {} not found in scope", symbol))
    }
}
