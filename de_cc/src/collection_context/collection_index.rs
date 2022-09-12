use std::fmt;

use petgraph::prelude::NodeIndex;

use crate::{type_system::type_mapping::TypeMapping, types::copy_types::CopyTypes};

use super::collection_context::{cc_get_node, cc_get_node_mut};

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct CollectionIndex(NodeIndex);

impl std::ops::Deref for CollectionIndex {
    type Target = NodeIndex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CopyTypes for CollectionIndex {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        cc_get_node_mut(self).as_mut().copy_types(type_mapping);
    }
}

impl fmt::Display for CollectionIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", cc_get_node(self).as_ref())
    }
}

impl CollectionIndex {
    pub(crate) fn new(index: NodeIndex) -> CollectionIndex {
        CollectionIndex(index)
    }
}
