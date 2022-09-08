use either::Either;

use crate::{
    language::{
        partial::partial_declaration::PartialFunctionDeclaration,
        typed::typed_declaration::TyFunctionDeclaration,
    },
    type_system::type_mapping::TypeMapping,
    types::copy_types::CopyTypes,
};

#[derive(Clone)]
pub(crate) struct TyFunctionContext {
    pub(crate) inner: Either<PartialFunctionDeclaration, TyFunctionDeclaration>,
}

impl CopyTypes for TyFunctionContext {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self.inner {
            Either::Left(ref mut partial) => partial.copy_types(type_mapping),
            Either::Right(ref mut typed) => typed.copy_types(type_mapping),
        }
    }
}

// NOTE: Hash and PartialEq must uphold the invariant:
// k1 == k2 -> hash(k1) == hash(k2)
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
impl PartialEq for TyFunctionContext {
    fn eq(&self, other: &Self) -> bool {
        match (&self.inner, &other.inner) {
            (Either::Left(ref l), Either::Left(ref r)) => l == r,
            (Either::Right(ref l), Either::Right(ref r)) => l == r,
            _ => false,
        }
    }
}

impl TyFunctionContext {
    pub(crate) fn partial(inner: PartialFunctionDeclaration) -> TyFunctionContext {
        TyFunctionContext {
            inner: Either::Left(inner),
        }
    }

    pub(crate) fn typed(inner: TyFunctionDeclaration) -> TyFunctionContext {
        TyFunctionContext {
            inner: Either::Right(inner),
        }
    }

    pub(crate) fn expect_typed(self) -> Result<TyFunctionDeclaration, String> {
        match self.inner {
            Either::Left(_) => Err("did not expect to find partial function".to_string()),
            Either::Right(typed) => Ok(typed),
        }
    }

    pub(crate) fn expect_partial(self) -> Result<PartialFunctionDeclaration, String> {
        match self.inner {
            Either::Left(partial) => Ok(partial),
            Either::Right(_) => Err("did not expect to find typed function".to_string()),
        }
    }

    pub(crate) fn name(&self) -> &str {
        match self.inner {
            Either::Left(ref partial) => &partial.name,
            Either::Right(ref typed) => &typed.name,
        }
    }
}
