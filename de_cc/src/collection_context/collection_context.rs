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

/// Represents a typeable AST as a graph by adding typeable AST nodes into a graph
/// and by drawing edges between relevant AST nodes. An edge is created between two
/// AST nodes when those two AST nodes share some scope relationship---sharing the
/// same scope, one being a scoped child of another, etc.
///
/// At a high level, the [CollectionContext] is a cyclical directed graph that allows
/// for both parallel edges and for self-referential edges. This is useful for the
/// AST because it allows us to model scoping relationships. The 'leaves' of the
/// [CollectionContext] are nodes from which no other nodes derive any nested scoping
/// (think literal expressions, variable expression, etc). Edges in the graph are
/// directed towards outward scope. For example, an AST node in a function body will
/// have an edge pointing outward in scope towards the AST node of the function
/// declaration itself. And that function declaration will have and edging pointing
/// outward in scope to the file its in. In this way, the root of the graph is the
/// entire application itself, as it encompasses the most outward scope.
///
/// Modeling scope in this way is useful because it allows us to 'look ahead' at
/// 'future AST nodes' during type collection, type inference, etc. We can think of it
/// this way. Given this file:
///
/// ```ignore
/// fn ping(n: u64) -> u64 {
///     return pong(n-1);
/// }
///
/// fn pong(n: u64) -> u64 {
///     return ping(n-1);
/// }
/// ```
///
/// During type inference, when evaluating the AST node for `return pong(n-1);`, we will
/// need to know 1) the type signature of `pong` 2) if `pong` exists at all and is in scope,
/// but we don't know this information because we haven't done type inference on `pong`
/// yet. Because these functions are mutually recursive, it is not possible to determine
/// an ordering for which to do type inference. The [CollectionContext] gives the ability
/// for the compiler to either 'look forward' or 'look backward' in the AST given any
/// location where type inference is currently 'standing'.
#[derive(Default, Clone)]
pub(crate) struct CollectionContext {
    pub(super) graph: CollectionGraph,
    pub(super) files: HashMap<String, CollectionIndex>,
}

impl CollectionContext {
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        println!("{}", self.create_link());
    }

    pub(crate) fn create_link(&self) -> String {
        let dot_str = format!(
            "https://dreampuf.github.io/GraphvizOnline/#{}",
            Dot::with_config(&self.graph, &[Config::EdgeIndexLabel])
        )
        .replace('\n', "%0A")
        .replace('=', "%3D")
        .replace("   ", "%20%20%20");
        std::str::from_utf8(dot_str.as_bytes()).unwrap().to_string()
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

    pub(crate) fn add_edge(
        &mut self,
        from: CollectionIndex,
        to: CollectionIndex,
        edge: CollectionEdge,
    ) -> EdgeIndex {
        self.graph.add_edge(*from, *to, edge)
    }

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
