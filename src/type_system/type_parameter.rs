use std::fmt;

use super::type_id::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameter {
    pub(crate) name_ident: String,
    pub(crate) type_id: TypeId,
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name_ident)
    }
}
