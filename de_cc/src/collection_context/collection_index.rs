use std::fmt;

use itertools::Itertools;
use petgraph::prelude::{EdgeIndex, NodeIndex};

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

// impl<T> fmt::Debug for CCIdx<T>
// where
//     T: fmt::Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self.inner)
//     }
// }

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
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.inner.copy_types(type_mapping);
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

    pub(crate) fn idx(&self) -> CollectionIndex {
        self.idx
    }

    pub(crate) fn inner(self) -> T {
        self.inner
    }

    pub(crate) fn inner_ref(&self) -> &T {
        &self.inner
    }

    pub(crate) fn inner_ref_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub(crate) fn add_edge<F>(
        from: &CCIdx<T>,
        to: &CCIdx<F>,
        edge: CollectionEdge,
        cc: &mut CollectionContext,
    ) -> EdgeIndex {
        cc.add_edge(from.idx, to.idx, edge)
    }

    pub(crate) fn add_edges_many_to_one<F>(
        from: &[CCIdx<T>],
        to: &CCIdx<F>,
        edge: CollectionEdge,
        cc: &mut CollectionContext,
    ) {
        from.iter().for_each(|from| {
            CCIdx::add_edge(from, to, edge.clone(), cc);
        });
    }

    pub(crate) fn add_edges_many(
        nodes: &[CCIdx<T>],
        edge: CollectionEdge,
        cc: &mut CollectionContext,
    ) where
        T: fmt::Debug,
    {
        nodes.iter().permutations(2).for_each(|inner_nodes| {
            let a = inner_nodes[0];
            let b = inner_nodes[1];
            CCIdx::add_edge(a, b, edge.clone(), cc);
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
