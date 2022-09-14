mod bfs;
#[allow(clippy::module_inception)]
pub(crate) mod collection_context;
pub(crate) mod collection_edge;
pub(crate) mod collection_index;
mod collection_node;
mod graph;

use self::{collection_edge::CollectionEdge, collection_node::CollectionNode, graph::graph::Graph};

type CollectionGraph = petgraph::Graph<CollectionNode, CollectionEdge>;
