mod bfs;
#[allow(clippy::module_inception)]
pub(crate) mod collection_context;
pub(crate) mod collection_index;
pub(crate) mod graph_edge;
mod graph_node;

use self::{graph_edge::GraphEdge, graph_node::GraphNode};

type CollectionGraph = petgraph::Graph<GraphNode, GraphEdge>;
