use crate::declaration_engine::declaration_engine::DeclarationEngine;

pub(crate) trait PrettyPrint {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String;
}
