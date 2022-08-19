use crate::{
    language::typed::typed_declaration::{TypedFunctionDeclaration, TypedTraitDeclaration},
    types::pretty_print::PrettyPrint,
};

use super::declaration_engine::DeclarationEngine;

#[derive(Clone)]
pub(crate) enum DeclarationWrapper {
    // no-op variant to fufill the default trait
    Default,
    Function(TypedFunctionDeclaration),
    Trait(TypedTraitDeclaration),
}

impl Default for DeclarationWrapper {
    fn default() -> Self {
        DeclarationWrapper::Default
    }
}

impl PrettyPrint for DeclarationWrapper {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            DeclarationWrapper::Default => "default case".to_string(),
            DeclarationWrapper::Function(decl) => decl.pretty_print(declaration_engine),
            DeclarationWrapper::Trait(decl) => decl.to_string(),
        }
    }
}

impl DeclarationWrapper {
    pub(super) fn expect_function(self) -> Result<TypedFunctionDeclaration, String> {
        match self {
            DeclarationWrapper::Function(decl) => Ok(decl),
            _ => Err("expected to find function declaration".to_string()),
        }
    }

    pub(super) fn expect_trait(self) -> Result<TypedTraitDeclaration, String> {
        match self {
            DeclarationWrapper::Trait(decl) => Ok(decl),
            _ => Err("expected to find trait declaration".to_string()),
        }
    }
}
