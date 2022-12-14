use petgraph::prelude::NodeIndex;

use crate::{
    type_system::type_mapping::TypeMapping,
    types::{copy_types::CopyTypes, pretty_print::PrettyPrint},
};

use super::collection_context::CollectionContext;

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
        cc.get_node(*self).pretty_print(cc)
    }

    fn pretty_print_debug(&self, cc: &CollectionContext) -> String {
        cc.get_node(*self).pretty_print_debug(cc)
    }
}

impl CollectionIndex {
    pub(crate) fn new(index: NodeIndex) -> CollectionIndex {
        CollectionIndex(index)
    }
}
