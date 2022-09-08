use std::fmt;

use crate::{
    language::typed::typed_declaration::{
        TyFunctionDeclaration, TyStructDeclaration, TyTraitDeclaration, TyTraitFn, TyTraitImpl,
    },
    namespace::function_signature::TypedFunctionSignature,
    type_system::type_mapping::TypeMapping,
    types::copy_types::CopyTypes,
};

/// The [DeclarationWrapper] type is used in the [DeclarationEngine]
/// as a means of placing all declaration types into the same type.
#[derive(Clone)]
pub(crate) enum DeclarationWrapper {
    // no-op variant to fulfill the default trait
    Unknown,
    Function(TyFunctionDeclaration),
    Trait(TyTraitDeclaration),
    TraitFn(TyTraitFn),
    TraitImpl(TyTraitImpl),
    Struct(TyStructDeclaration),
}

impl Default for DeclarationWrapper {
    fn default() -> Self {
        DeclarationWrapper::Unknown
    }
}

// NOTE: Hash and PartialEq must uphold the invariant:
// k1 == k2 -> hash(k1) == hash(k2)
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
impl PartialEq for DeclarationWrapper {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DeclarationWrapper::Unknown, DeclarationWrapper::Unknown) => true,
            (DeclarationWrapper::Function(l), DeclarationWrapper::Function(r)) => l == r,
            (DeclarationWrapper::Trait(l), DeclarationWrapper::Trait(r)) => l == r,
            (DeclarationWrapper::TraitFn(l), DeclarationWrapper::TraitFn(r)) => l == r,
            (DeclarationWrapper::TraitImpl(l), DeclarationWrapper::TraitImpl(r)) => l == r,
            (DeclarationWrapper::Struct(l), DeclarationWrapper::Struct(r)) => l == r,
            _ => false,
        }
    }
}

impl fmt::Display for DeclarationWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "decl({})", self.friendly_name())
    }
}

impl CopyTypes for DeclarationWrapper {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            DeclarationWrapper::Unknown => {}
            DeclarationWrapper::Function(decl) => decl.copy_types(type_mapping),
            DeclarationWrapper::Trait(decl) => decl.copy_types(type_mapping),
            DeclarationWrapper::TraitFn(decl) => decl.copy_types(type_mapping),
            DeclarationWrapper::TraitImpl(decl) => decl.copy_types(type_mapping),
            DeclarationWrapper::Struct(decl) => decl.copy_types(type_mapping),
        }
    }
}

impl DeclarationWrapper {
    /// friendly name string used for error reporting.
    fn friendly_name(&self) -> &'static str {
        match self {
            DeclarationWrapper::Unknown => "unknown",
            DeclarationWrapper::Function(_) => "function",
            DeclarationWrapper::Trait(_) => "trait",
            DeclarationWrapper::Struct(_) => "struct",
            DeclarationWrapper::TraitImpl(_) => "impl trait",
            DeclarationWrapper::TraitFn(_) => "trait function",
        }
    }

    pub(super) fn expect_function(self) -> Result<TyFunctionDeclaration, String> {
        match self {
            DeclarationWrapper::Function(decl) => Ok(decl),
            DeclarationWrapper::Unknown => {
                Err("did not expect to find unknown declaration".to_string())
            }
            actually => Err(format!(
                "did not expect to find {} declaration",
                actually.friendly_name()
            )),
        }
    }

    // pub(super) fn expect_function_typed(self) -> Result<TypedFunctionDeclaration, String> {
    //     match self {
    //         DeclarationWrapper::Function(decl) => decl.expect_typed(),
    //         DeclarationWrapper::Unknown => {
    //             Err("did not expect to find unknown declaration".to_string())
    //         }
    //         actually => Err(format!(
    //             "did not expect to find {} declaration",
    //             actually.friendly_name()
    //         )),
    //     }
    // }

    // pub(super) fn expect_function_partial(self) -> Result<PartialFunctionDeclaration, String> {
    //     match self {
    //         DeclarationWrapper::Function(decl) => decl.expect_partial(),
    //         DeclarationWrapper::Unknown => {
    //             Err("did not expect to find unknown declaration".to_string())
    //         }
    //         actually => Err(format!(
    //             "did not expect to find {} declaration",
    //             actually.friendly_name()
    //         )),
    //     }
    // }

    pub(super) fn expect_function_signature(self) -> Result<TypedFunctionSignature, String> {
        match self {
            DeclarationWrapper::Function(decl) => Ok(decl.into()),
            DeclarationWrapper::Unknown => {
                Err("did not expect to find unknown declaration".to_string())
            }
            actually => Err(format!(
                "did not expect to find {} declaration",
                actually.friendly_name()
            )),
        }
    }

    pub(super) fn expect_trait(self) -> Result<TyTraitDeclaration, String> {
        match self {
            DeclarationWrapper::Trait(decl) => Ok(decl),
            DeclarationWrapper::Unknown => {
                Err("did not expect to find unknown declaration".to_string())
            }
            actually => Err(format!(
                "did not expect to find {} declaration",
                actually.friendly_name()
            )),
        }
    }

    pub(super) fn expect_trait_fn(self) -> Result<TyTraitFn, String> {
        match self {
            DeclarationWrapper::TraitFn(decl) => Ok(decl),
            DeclarationWrapper::Unknown => {
                Err("did not expect to find unknown declaration".to_string())
            }
            actually => Err(format!(
                "did not expect to find {} declaration",
                actually.friendly_name()
            )),
        }
    }

    pub(super) fn expect_trait_impl(self) -> Result<TyTraitImpl, String> {
        match self {
            DeclarationWrapper::TraitImpl(decl) => Ok(decl),
            DeclarationWrapper::Unknown => {
                Err("did not expect to find unknown declaration".to_string())
            }
            actually => Err(format!(
                "did not expect to find {} declaration",
                actually.friendly_name()
            )),
        }
    }

    pub(super) fn expect_struct(self) -> Result<TyStructDeclaration, String> {
        match self {
            DeclarationWrapper::Struct(decl) => Ok(decl),
            DeclarationWrapper::Unknown => {
                Err("did not expect to find unknown declaration".to_string())
            }
            actually => Err(format!(
                "did not expect to find {} declaration",
                actually.friendly_name()
            )),
        }
    }
}
