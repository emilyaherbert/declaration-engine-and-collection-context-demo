use std::fmt;
use std::hash::{Hash, Hasher};

use crate::collection_context::collection_context::CollectionContext;
use crate::types::copy_types::CopyTypes;

use super::type_engine::{insert_type, look_up_type_id};
use super::type_info::TypeInfo;
use super::type_mapping::TypeMapping;

#[derive(Eq, Clone, Copy, Default)]
pub struct TypeId(usize);

impl std::ops::Deref for TypeId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for TypeId {
    fn from(o: usize) -> Self {
        TypeId(o)
    }
}

impl fmt::Debug for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:?}", self.0, look_up_type_id(*self))
    }
}

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", look_up_type_id(*self))
    }
}

impl Hash for TypeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        look_up_type_id(*self).hash(state);
    }
}

impl PartialEq for TypeId {
    fn eq(&self, other: &Self) -> bool {
        look_up_type_id(*self) == look_up_type_id(*other)
    }
}

impl CopyTypes for TypeId {
    fn copy_types(&mut self, _cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        if let Some(matching_id) = look_up_type_id(*self).matches_type_parameter(type_mapping) {
            *self = insert_type(TypeInfo::Ref(matching_id));
        }
    }
}

impl TypeId {
    pub(super) fn new(index: usize) -> TypeId {
        TypeId(index)
    }
}
