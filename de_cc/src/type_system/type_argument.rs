use std::fmt;

use super::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct TypeArgument {
    pub(crate) type_id: TypeId,
}

impl fmt::Display for TypeArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_id)
    }
}
