use crate::{
    language::typed::typed_declaration::{
        TypedFunctionDeclaration, TypedStructDeclaration, TypedTraitDeclaration, TypedTraitFn,
        TypedTraitImpl,
    },
    types::pretty_print::PrettyPrint,
};

use super::declaration_engine::DeclarationEngine;

#[derive(Clone)]
pub(crate) enum DeclarationWrapper {
    // no-op variant to fufill the default trait
    Default,
    Function(TypedFunctionDeclaration),
    Trait(TypedTraitDeclaration),
    TraitFn(TypedTraitFn),
    TraitImpl(TypedTraitImpl),
    Struct(TypedStructDeclaration),
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
            DeclarationWrapper::Trait(decl) => decl.pretty_print(declaration_engine),
            DeclarationWrapper::TraitImpl(decl) => decl.pretty_print(declaration_engine),
            DeclarationWrapper::Struct(decl) => decl.to_string(),
            DeclarationWrapper::TraitFn(decl) => decl.to_string(),
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

    pub(super) fn expect_trait_fn(self) -> Result<TypedTraitFn, String> {
        match self {
            DeclarationWrapper::TraitFn(decl) => Ok(decl),
            _ => Err("expected to find trait fn".to_string()),
        }
    }

    pub(super) fn expect_trait_impl(self) -> Result<TypedTraitImpl, String> {
        match self {
            DeclarationWrapper::TraitImpl(decl) => Ok(decl),
            _ => Err("expected to find trait impl".to_string()),
        }
    }

    pub(super) fn expect_struct(self) -> Result<TypedStructDeclaration, String> {
        match self {
            DeclarationWrapper::Struct(decl) => Ok(decl),
            _ => Err("expected to find struct declaration".to_string()),
        }
    }
}
