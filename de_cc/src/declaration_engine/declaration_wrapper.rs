use either::Either;
use std::fmt;

use crate::{
    language::{
        semi::semi_declaration::SemiTypedFunctionDeclaration,
        typed::typed_declaration::{
            TypedFunctionDeclaration, TypedStructDeclaration, TypedTraitDeclaration, TypedTraitFn,
            TypedTraitImpl,
        },
    },
    type_system::type_mapping::TypeMapping,
    types::copy_types::CopyTypes,
};

/// The [DeclarationWrapper] type is used in the [DeclarationEngine]
/// as a means of placing all declaration types into the same type.
#[derive(Clone)]
pub(crate) enum DeclarationWrapper {
    // no-op variant to fulfill the default trait
    Unknown,
    Function(Either<SemiTypedFunctionDeclaration, TypedFunctionDeclaration>),
    Trait(TypedTraitDeclaration),
    TypedTraitFn(TypedTraitFn),
    TraitFn(TypedTraitImpl),
    Struct(TypedStructDeclaration),
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
            (DeclarationWrapper::TypedTraitFn(l), DeclarationWrapper::TypedTraitFn(r)) => l == r,
            (DeclarationWrapper::TraitFn(l), DeclarationWrapper::TraitFn(r)) => l == r,
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
            DeclarationWrapper::TypedTraitFn(decl) => decl.copy_types(type_mapping),
            DeclarationWrapper::TraitFn(decl) => decl.copy_types(type_mapping),
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
            DeclarationWrapper::TraitFn(_) => "impl trait",
            DeclarationWrapper::TypedTraitFn(_) => "trait function",
        }
    }

    pub(super) fn expect_function_typed(self) -> Result<TypedFunctionDeclaration, String> {
        match self {
            DeclarationWrapper::Function(decl) => match decl {
                Either::Left(_) => Err("did not expect to find semi typed declaration".to_string()),
                Either::Right(decl) => Ok(decl),
            },
            DeclarationWrapper::Unknown => {
                Err("did not expect to find unknown declaration".to_string())
            }
            actually => Err(format!(
                "did not expect to find {} declaration",
                actually.friendly_name()
            )),
        }
    }

    pub(super) fn expect_function_semi_typed(self) -> Result<SemiTypedFunctionDeclaration, String> {
        match self {
            DeclarationWrapper::Function(decl) => match decl {
                Either::Left(decl) => Ok(decl),
                Either::Right(_) => Err("did not expect to find typed declaration".to_string()),
            },
            DeclarationWrapper::Unknown => {
                Err("did not expect to find unknown declaration".to_string())
            }
            actually => Err(format!(
                "did not expect to find {} declaration",
                actually.friendly_name()
            )),
        }
    }

    pub(super) fn expect_trait(self) -> Result<TypedTraitDeclaration, String> {
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

    pub(super) fn expect_trait_fn(self) -> Result<TypedTraitFn, String> {
        match self {
            DeclarationWrapper::TypedTraitFn(decl) => Ok(decl),
            DeclarationWrapper::Unknown => {
                Err("did not expect to find unknown declaration".to_string())
            }
            actually => Err(format!(
                "did not expect to find {} declaration",
                actually.friendly_name()
            )),
        }
    }

    pub(super) fn expect_trait_impl(self) -> Result<TypedTraitImpl, String> {
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

    pub(super) fn expect_struct(self) -> Result<TypedStructDeclaration, String> {
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
