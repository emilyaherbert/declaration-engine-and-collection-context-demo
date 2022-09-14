use super::{
    edge::{Edge, EdgeIndex},
    node::NodeIndex,
};

pub(super) struct Neighbors<'a, E: 'a> {
    /// starting node to skip over
    skip_start: NodeIndex,
    edges: &'a [Edge<E>],
    next: [EdgeIndex; 2],
}
