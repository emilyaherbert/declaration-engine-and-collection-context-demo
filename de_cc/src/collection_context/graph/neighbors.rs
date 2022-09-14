use super::{
    edge::{Edge, EdgeIndex},
    node::NodeIndex,
};

// https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#1511
pub(crate) struct Neighbors<'a, E: 'a> {
    pub(super) skip_start: NodeIndex,
    pub(super) edges: &'a [Edge<E>],
    pub(super) next: [EdgeIndex; 2],
}

// https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#1518
impl<'a, E> Iterator for Neighbors<'a, E> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        // First any outgoing edges
        match self.edges.get(*self.next[0]) {
            None => {}
            Some(edge) => {
                self.next[0] = edge.next[0];
                return Some(edge.node[1]);
            }
        }
        while let Some(edge) = self.edges.get(*self.next[1]) {
            self.next[1] = edge.next[1];
            if edge.node[0] != self.skip_start {
                return Some(edge.node[0]);
            }
        }
        None
    }
}
