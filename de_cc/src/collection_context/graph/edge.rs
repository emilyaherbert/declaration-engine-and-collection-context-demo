use super::{direction::Direction, node::NodeIndex};

use std::ops::Deref;

// https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#161
#[derive(PartialEq, Clone, Copy)]
pub(crate) struct EdgeIndex(usize);

impl Deref for EdgeIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EdgeIndex {
    // https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#165
    pub(super) fn new(x: usize) -> EdgeIndex {
        EdgeIndex(x)
    }

    // https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#177
    pub(super) fn end() -> EdgeIndex {
        EdgeIndex(usize::MAX)
    }
}

// https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#241
#[derive(Clone)]
pub(super) struct Edge<E> {
    pub(crate) weight: E,
    pub(super) next: [EdgeIndex; 2],
    pub(super) node: [NodeIndex; 2],
}

pub(crate) struct EdgeReference<'a, E: 'a> {
    #[allow(dead_code)]
    index: EdgeIndex,
    #[allow(dead_code)]
    node: [NodeIndex; 2],
    pub(crate) weight: &'a E,
}

// https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#1633
pub(crate) struct Edges<'a, E: 'a> {
    pub(super) skip_start: NodeIndex,
    pub(super) edges: &'a [Edge<E>],
    pub(super) next: [EdgeIndex; 2],
    pub(super) direction: Direction,
}

// https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#1651
impl<'a, E> Iterator for Edges<'a, E> {
    type Item = EdgeReference<'a, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let (iterate_over, reverse) = (Some(self.direction), None);

        if iterate_over.unwrap_or(Direction::Outgoing) == Direction::Outgoing {
            let i = self.next[0];
            if let Some(Edge { node, weight, next }) = self.edges.get(*i) {
                self.next[0] = next[0];
                return Some(EdgeReference {
                    index: EdgeIndex::new(*i),
                    node: if reverse == Some(Direction::Outgoing) {
                        swap_pair(*node)
                    } else {
                        *node
                    },
                    weight,
                });
            }
        }

        if iterate_over.unwrap_or(Direction::Incoming) == Direction::Incoming {
            while let Some(Edge { node, weight, next }) = self.edges.get(*self.next[1]) {
                let edge_index = self.next[1];
                self.next[1] = next[1];
                // In any of the "both" situations, self-loops would be iterated over twice.
                // Skip them here.
                if iterate_over.is_none() && node[0] == self.skip_start {
                    continue;
                }

                return Some(EdgeReference {
                    index: edge_index,
                    node: if reverse == Some(Direction::Incoming) {
                        swap_pair(*node)
                    } else {
                        *node
                    },
                    weight,
                });
            }
        }

        None
    }
}

fn swap_pair<T>(mut x: [T; 2]) -> [T; 2] {
    x.swap(0, 1);
    x
}
