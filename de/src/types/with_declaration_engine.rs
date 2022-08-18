use crate::declaration_engine::declaration_engine::DeclarationEngine;

pub(crate) struct TypeWithDeclarationEngine<'a, T> {
    inner: &'a T,
    declaration_engine: &'a DeclarationEngine,
}

pub(crate) trait WithDeclarationEngine<'a> {
    fn with_declaration_engine(
        &'a self,
        declaration_engine: &'a DeclarationEngine,
    ) -> TypeWithDeclarationEngine<'a, Self>
    where
        Self: std::marker::Sized,
    {
        TypeWithDeclarationEngine {
            inner: self,
            declaration_engine,
        }
    }
}
