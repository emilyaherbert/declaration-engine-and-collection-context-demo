use crate::{
    language::{
        partial::partial_declaration::PartialFunctionDeclaration,
        typed::typed_declaration::{
            TypedFunctionDeclaration, TypedFunctionParameter, TypedTraitFn,
        },
        typing_context::function::TyFunctionContext,
    },
    type_system::{type_id::TypeId, type_parameter::TypeParameter},
};

pub(crate) struct TypedFunctionSignature {
    #[allow(dead_code)]
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) return_type: TypeId,
}

impl From<TypedFunctionDeclaration> for TypedFunctionSignature {
    fn from(decl: TypedFunctionDeclaration) -> Self {
        TypedFunctionSignature {
            name: decl.name,
            type_parameters: decl.type_parameters,
            parameters: decl.parameters,
            return_type: decl.return_type,
        }
    }
}

impl From<TypedTraitFn> for TypedFunctionSignature {
    fn from(decl: TypedTraitFn) -> Self {
        TypedFunctionSignature {
            name: decl.name,
            type_parameters: vec![],
            parameters: decl.parameters,
            return_type: decl.return_type,
        }
    }
}

impl From<PartialFunctionDeclaration> for TypedFunctionSignature {
    fn from(decl: PartialFunctionDeclaration) -> Self {
        TypedFunctionSignature {
            name: decl.name,
            type_parameters: decl.type_parameters,
            parameters: decl.parameters,
            return_type: decl.return_type,
        }
    }
}

impl From<TyFunctionContext> for TypedFunctionSignature {
    fn from(ctx: TyFunctionContext) -> Self {
        match ctx.inner {
            either::Either::Left(partial) => partial.into(),
            either::Either::Right(typed) => typed.into(),
        }
    }
}
