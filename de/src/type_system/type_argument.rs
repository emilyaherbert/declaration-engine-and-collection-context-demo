use crate::{
    declaration_engine::declaration_engine::DeclarationEngine, types::pretty_print::PrettyPrint,
};

use super::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct TypeArgument {
    pub(crate) type_id: TypeId,
}

impl PrettyPrint for TypeArgument {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!("{}", self.type_id.pretty_print(declaration_engine))
    }
}
