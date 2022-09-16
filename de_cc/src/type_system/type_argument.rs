use std::fmt;
use std::hash::{Hash, Hasher};

use crate::types::copy_types::CopyTypes;

use super::{type_id::TypeId, type_mapping::TypeMapping};

#[derive(Debug, Clone, Eq)]
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

impl Hash for TypeArgument {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
    }
}

impl PartialEq for TypeArgument {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}
