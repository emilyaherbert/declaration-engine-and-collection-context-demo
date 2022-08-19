use std::fmt;
use std::hash::Hash;

use super::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParameter {
    pub(crate) name_ident: String,
    pub(crate) type_id: TypeId,
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_ident)
    }
}
