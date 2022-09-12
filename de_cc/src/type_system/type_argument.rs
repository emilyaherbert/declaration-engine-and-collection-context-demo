use std::fmt;

use crate::types::copy_types::CopyTypes;

use super::{type_id::TypeId, type_mapping::TypeMapping};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeArgument {
    pub(crate) type_id: TypeId,
}

impl fmt::Display for TypeArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_id)
    }
}

impl CopyTypes for TypeArgument {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_id.copy_types(type_mapping);
    }
}
