use std::hash::Hash;

use crate::declaration_engine::declaration_engine::DeclarationEngine;
use crate::types::pretty_print::PrettyPrint;

use super::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParameter {
    pub(crate) name_ident: String,
    pub(crate) type_id: TypeId,
}

impl PrettyPrint for TypeParameter {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!("{}", self.name_ident)
    }
}
