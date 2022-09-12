use std::fmt;

use crate::{
    collection_context::collection_context::CollectionContext,
    type_system::type_mapping::TypeMapping, types::copy_types::CopyTypes,
};

use super::declaration_engine::de_look_up_decl_id;

/// An ID used to refer to an item in the [DeclarationEngine](super::declaration_engine::DeclarationEngine)
#[derive(Debug, Eq, Copy)]
pub struct DeclarationId(usize);

impl Clone for DeclarationId {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

// NOTE: Hash and PartialEq must uphold the invariant:
// k1 == k2 -> hash(k1) == hash(k2)
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
impl PartialEq for DeclarationId {
    fn eq(&self, other: &Self) -> bool {
        de_look_up_decl_id(*self) == de_look_up_decl_id(*other)
    }
}

impl fmt::Display for DeclarationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&de_look_up_decl_id(*self).to_string())
    }
}

impl std::ops::Deref for DeclarationId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(clippy::from_over_into)]
impl Into<usize> for DeclarationId {
    fn into(self) -> usize {
        self.0
    }
}

impl CopyTypes for DeclarationId {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        de_look_up_decl_id(*self).copy_types(cc, type_mapping)
    }
}

impl DeclarationId {
    pub(super) fn new(index: usize) -> DeclarationId {
        DeclarationId(index)
    }
}
