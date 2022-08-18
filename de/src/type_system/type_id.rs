use std::hash::{Hash, Hasher};

use crate::declaration_engine::declaration_engine::DeclarationEngine;
use crate::types::pretty_print::PrettyPrint;

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

impl PrettyPrint for TypeId {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!(
            "{}",
            look_up_type_id(*self).pretty_print(declaration_engine)
        )
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
