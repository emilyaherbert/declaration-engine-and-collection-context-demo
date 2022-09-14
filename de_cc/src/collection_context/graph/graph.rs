use super::{
    direction::Direction,
    edge::{Edge, EdgeIndex, Edges},
    node::{Node, NodeIndex},
    visit_map::VisitMap,
};

// https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#345
#[derive(Clone)]
pub(crate) struct Graph<N, E>
where
    N: Clone,
    E: Clone,
{
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

impl<N, E> Default for Graph<N, E>
where
    N: Clone,
    E: Clone,
{
    fn default() -> Graph<N, E> {
        Graph::new()
    }
}

impl<N, E> Graph<N, E>
where
    N: Clone,
    E: Clone,
{
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub(crate) fn add_node(&mut self, weight: N) -> Result<NodeIndex, String> {
        let node = Node {
            weight,
            next: [EdgeIndex::end(), EdgeIndex::end()],
        };
        let node_idx = NodeIndex::new(self.nodes.len());
        if node_idx == NodeIndex::end() {
            return Err("reached max number of nodes".to_string());
        }
        self.nodes.push(node);
        Ok(node_idx)
    }

    pub(crate) fn replace_node(&mut self, index: NodeIndex, new_weight: N) {
        self.nodes
            .get_mut(*index)
            .map(|old_node| old_node.weight = new_weight);
    }

    pub(crate) fn add_edge(
        &mut self,
        a: NodeIndex,
        b: NodeIndex,
        weight: E,
    ) -> Result<EdgeIndex, String> {
        let edge_idx = EdgeIndex::new(self.edges.len());
        if edge_idx == EdgeIndex::end() {
            return Err("reached max number of edges".to_string());
        }
        let mut edge = Edge {
            weight,
            node: [a, b],
            next: [EdgeIndex::end(); 2],
        };
        match self.index_twice(a, b) {
            Pair::None => panic!("Graph::add_edge: node indices out of bounds"),
            Pair::One(an) => {
                edge.next = an.next;
                an.next[0] = edge_idx;
                an.next[1] = edge_idx;
            }
            Pair::Both(an, bn) => {
                // a and b are different indices
                edge.next = [an.next[0], bn.next[1]];
                an.next[0] = edge_idx;
                bn.next[1] = edge_idx;
            }
        }
        self.edges.push(edge);
        Ok(edge_idx)
    }

    // https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#1860
    pub(crate) fn index(&self, index: NodeIndex) -> &N {
        &self.nodes[*index].weight
    }

    // https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#1873
    pub(crate) fn index_mut(&mut self, index: NodeIndex) -> &mut N {
        &mut self.nodes[*index].weight
    }

    // https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#2144
    pub(crate) fn visit_map(&self) -> VisitMap {
        VisitMap::new(self.nodes.len())
    }

    // https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#859
    pub(crate) fn edges_directed(&self, a: NodeIndex, dir: Direction) -> Edges<E> {
        Edges {
            skip_start: a,
            edges: &self.edges,
            direction: dir,
            next: match self.nodes.get(*a) {
                None => [EdgeIndex::end(), EdgeIndex::end()],
                Some(n) => n.next,
            },
        }
    }

    // https://docs.rs/petgraph/0.6.2/src/petgraph/graph_impl/mod.rs.html#437
    fn index_twice(&mut self, a: NodeIndex, b: NodeIndex) -> Pair<&mut Node<N>> {
        if *NodeIndex::max(a, b) >= self.nodes.len() {
            Pair::None
        } else if a == b {
            Pair::One(&mut self.nodes[*NodeIndex::max(a, b)])
        } else {
            // safe because a, b are in bounds and distinct
            unsafe {
                let ptr = self.nodes.as_mut_ptr();
                let ar = &mut *ptr.add(*a);
                let br = &mut *ptr.add(*b);
                Pair::Both(ar, br)
            }
        }
    }
}

enum Pair<T> {
    Both(T, T),
    One(T),
    None,
}
