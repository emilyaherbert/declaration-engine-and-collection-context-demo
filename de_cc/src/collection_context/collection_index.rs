use std::fmt;

use itertools::Itertools;
use petgraph::prelude::NodeIndex;

use crate::{
    type_system::type_mapping::TypeMapping,
    types::{copy_types::CopyTypes, pretty_print::PrettyPrint},
};

use super::{collection_context::CollectionContext, collection_edge::CollectionEdge};

#[derive(Clone, Debug)]
pub(crate) struct CCIdx<T> {
    inner: T,
    idx: CollectionIndex,
}

impl<T> fmt::Display for CCIdx<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T> CopyTypes for CCIdx<T>
where
    T: CopyTypes,
{
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        self.inner.copy_types(cc, type_mapping);
    }
}

impl<T> PartialEq for CCIdx<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> CCIdx<T> {
    pub(crate) fn new(inner: T, idx: CollectionIndex) -> CCIdx<T> {
        CCIdx { inner, idx }
    }

    pub(crate) fn inner(self) -> T {
        self.inner
    }

    pub(crate) fn add_edge<F>(
        from: &CCIdx<T>,
        to: &CCIdx<F>,
        edge: CollectionEdge,
        cc: &mut CollectionContext,
    ) {
        cc.add_edge(from.idx, to.idx, edge);
    }

    pub(crate) fn add_edges_one_to_many<F>(
        from: &CCIdx<T>,
        to: &[CCIdx<F>],
        edge: CollectionEdge,
        cc: &mut CollectionContext,
    ) {
        to.iter().for_each(|to| CCIdx::add_edge(from, to, edge, cc));
    }

    pub(crate) fn add_edges_many_to_one<F>(
        from: &[CCIdx<T>],
        to: &CCIdx<F>,
        edge: CollectionEdge,
        cc: &mut CollectionContext,
    ) {
        from.iter()
            .for_each(|from| CCIdx::add_edge(from, to, edge, cc));
    }

    pub(crate) fn add_edges_many(
        nodes: &[CCIdx<T>],
        edge: CollectionEdge,
        cc: &mut CollectionContext,
    ) {
        nodes.iter().permutations(2).for_each(|inner_nodes| {
            let a = inner_nodes[0];
            let b = inner_nodes[1];
            CCIdx::add_edge(a, b, edge, cc);
        });
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct CollectionIndex(NodeIndex);

impl std::ops::Deref for CollectionIndex {
    type Target = NodeIndex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CopyTypes for CollectionIndex {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        let mut new_node = cc.get_node_mut(*self).clone();
        new_node.copy_types(cc, type_mapping);
    }
}

impl PrettyPrint for CollectionIndex {
    fn pretty_print(&self, cc: &CollectionContext) -> String {
        cc.get_node(*self).to_string()
    }

    fn pretty_print_debug(&self, cc: &CollectionContext) -> String {
        format!("{:?}", cc.get_node(*self))
    }
}

impl CollectionIndex {
    pub(crate) fn new(index: NodeIndex) -> CollectionIndex {
        CollectionIndex(index)
    }
}
