use crate::declaration_engine::declaration_engine::DeclarationEngine;

pub trait PrettyPrint {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String;
}
