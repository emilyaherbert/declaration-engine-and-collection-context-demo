use crate::{
    language::ty::typed_declaration::{TyFunctionDeclaration, TyFunctionParameter, TyTraitFn},
    type_system::{type_id::TypeId, type_parameter::TypeParameter},
};

pub(crate) struct TypedFunctionSignature {
    #[allow(dead_code)]
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<TyFunctionParameter>,
    pub(crate) return_type: TypeId,
}

impl From<TyFunctionDeclaration> for TypedFunctionSignature {
    fn from(decl: TyFunctionDeclaration) -> Self {
        TypedFunctionSignature {
            name: decl.name,
            type_parameters: decl.type_parameters,
            parameters: decl.parameters,
            return_type: decl.return_type,
        }
    }
}

impl From<TyTraitFn> for TypedFunctionSignature {
    fn from(decl: TyTraitFn) -> Self {
        TypedFunctionSignature {
            name: decl.name,
            type_parameters: vec![],
            parameters: decl.parameters,
            return_type: decl.return_type,
        }
    }
}
