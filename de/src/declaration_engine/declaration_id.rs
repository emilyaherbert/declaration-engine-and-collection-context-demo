use std::fmt;

use crate::types::pretty_print::PrettyPrint;

use super::declaration_engine::DeclarationEngine;

#[derive(Clone, Copy, Debug)]
pub struct DeclarationId(usize);

impl fmt::Display for DeclarationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for DeclarationId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for DeclarationId {
    fn from(o: usize) -> Self {
        DeclarationId(o)
    }
}

impl PrettyPrint for DeclarationId {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        declaration_engine
            .look_up_decl_id(*self)
            .pretty_print(declaration_engine)
    }
}
