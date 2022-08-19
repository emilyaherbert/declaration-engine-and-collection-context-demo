use crate::types::pretty_print::PrettyPrint;

use super::declaration_engine::DeclarationEngine;

#[derive(Clone)]
pub struct DeclarationId(usize);

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
            .look_up_decl_id(self.clone())
            .pretty_print(declaration_engine)
    }
}
