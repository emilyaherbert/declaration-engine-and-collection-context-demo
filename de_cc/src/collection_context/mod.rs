//! This module is used in graph collection.

mod bfs;
#[allow(clippy::module_inception)]
pub(crate) mod collection_context;
pub(crate) mod collection_edge;
pub(crate) mod collection_index;
pub(crate) mod collection_node;

use self::{collection_edge::CollectionEdge, collection_node::CollectionNode};

type CollectionGraph = petgraph::Graph<CollectionNode, CollectionEdge>;
