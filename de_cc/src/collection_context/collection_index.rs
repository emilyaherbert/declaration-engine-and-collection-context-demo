use petgraph::prelude::NodeIndex;

use crate::{
    type_system::type_mapping::TypeMapping,
    types::{copy_types::CopyTypes, pretty_print::PrettyPrint},
};

use super::collection_context::CollectionContext;

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct CollectionIndex(NodeIndex);

impl std::ops::Deref for CollectionIndex {
    type Target = NodeIndex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CopyTypes for CollectionIndex {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        let mut new_node = cc.get_node_mut(self).clone();
        new_node.copy_types(cc, type_mapping);
        let old_node = cc.get_node_mut(self);
        *old_node = new_node;
    }
}

impl PrettyPrint for CollectionIndex {
    fn pretty_print(&self, cc: &CollectionContext) -> String {
        cc.get_node(self).pretty_print(cc)
    }
}

impl CollectionIndex {
    pub(crate) fn new(index: NodeIndex) -> CollectionIndex {
        CollectionIndex(index)
    }
}
