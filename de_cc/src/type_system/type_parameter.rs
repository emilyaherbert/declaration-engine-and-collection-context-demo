use std::fmt;
use std::hash::Hash;

use crate::collection_context::collection_context::CollectionContext;
use crate::types::copy_types::CopyTypes;

use super::trait_constraint::TraitConstraint;
use super::type_id::TypeId;
use super::type_mapping::TypeMapping;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParameter {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
    pub(crate) trait_constraint: Option<TraitConstraint>,
}

impl CopyTypes for TypeParameter {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        self.type_id.copy_types(cc, type_mapping);
    }
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_id)
    }
}
