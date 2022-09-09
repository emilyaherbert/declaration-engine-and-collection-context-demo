use petgraph::prelude::{EdgeIndex, NodeIndex};

use super::graph_node::GraphNode;

type CollectionGraph<'gn> = petgraph::Graph<GraphNode<'gn>, ()>;

#[derive(Default)]
pub(crate) struct CollectionContext<'gn> {
    graph: CollectionGraph<'gn>,
}

impl<'gn> CollectionContext<'gn> {
    pub(crate) fn add_node<'ast>(&mut self, node: GraphNode<'ast>) -> NodeIndex
    where
        'ast: 'gn,
    {
        self.graph.add_node(node)
    }

    pub(crate) fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) -> EdgeIndex {
        self.graph.add_edge(from, to, ())
    }
}
