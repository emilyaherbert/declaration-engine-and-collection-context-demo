use std::fmt;
use std::hash::{Hash, Hasher};

use super::type_engine::look_up_type_id;

#[derive(Eq, Clone, Copy, Debug)]
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

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
