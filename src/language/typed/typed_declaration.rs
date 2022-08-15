use super::{typed_expression::*, TypedNode};

use crate::{
    language::untyped::declaration::FunctionDeclaration,
    type_system::{type_id::TypeId, type_parameter::TypeParameter},
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypedDeclaration {
    Variable(TypedVariableDeclaration),
    Function(String),
    Trait(String),
    Struct(String),
    Enum(String),
    TraitImpl(TypedTraitImpl),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedVariableDeclaration {
    pub(crate) name: String,
    pub(crate) body: TypedExpression,
    pub(crate) type_ascription: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedFunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) body: Vec<TypedNode>,
    pub(crate) return_type: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedFunctionParameter {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedTraitDeclaration {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<TypedTraitFn>,
    pub(crate) methods: Vec<FunctionDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedTraitFn {
    pub(crate) name: String,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) return_type: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedStructDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) fields: Vec<TypedStructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedStructField {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedEnumDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) variants: Vec<TypedEnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedEnumVariant {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
    pub(crate) tag: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedTraitImpl {
    pub(crate) trait_name: String,
    pub(crate) type_implementing_for: TypeId,
    pub(crate) methods: Vec<TypedFunctionDeclaration>,
}
