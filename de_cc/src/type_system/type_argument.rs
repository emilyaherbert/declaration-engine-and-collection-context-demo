use std::fmt;

use super::type_id::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeArgument {
    pub(crate) type_id: TypeId,
}

impl fmt::Display for TypeArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_id)
    }
}
