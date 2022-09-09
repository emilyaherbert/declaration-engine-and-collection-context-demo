use petgraph::prelude::{EdgeIndex, NodeIndex};

use crate::language::ty::TyNode;

use super::graph_node::GraphNode;

type Graph<'n> = petgraph::Graph<GraphNode<'n>, ()>;

#[derive(Default)]
pub(crate) struct CollectionContext<'n> {
    graph: Graph<'n>,
}

impl<'n> CollectionContext<'n> {
    pub(crate) fn add_node(&mut self, node: GraphNode<'n>) -> NodeIndex {
        self.graph.add_node(node)
    }

    pub(crate) fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) -> EdgeIndex {
        self.graph.add_edge(from, to, ())
    }
}
