use super::{typed_expression::*, TypedNode};

use crate::language::untyped::FunctionDeclaration;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypedDeclaration {
    Variable(String),
    Function(String),
    Trait(String),
    Struct(String),
    Enum(String),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedVariableDeclaration {
    pub(crate) name: String,
    pub(crate) body: TypedExpression,
    //pub(crate) type_ascription: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedFunctionDeclaration {
    pub(crate) name: String,
    //pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) body: Vec<TypedNode>,
    //pub(crate) return_type: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedFunctionParameter {
    pub(crate) name: String,
    //pub(crate) type_id: TypeId,
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
    //pub(crate) return_type: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedStructDeclaration {
    pub(crate) name: String,
    pub(crate) fields: Vec<TypedStructField>,
    //pub(crate) type_parameters: Vec<TypeParameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedStructField {
    pub(crate) name: String,
    //pub(crate) type_info: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedEnumDeclaration {
    pub(crate) name: String,
    //pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) variants: Vec<TypedEnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedEnumVariant {
    pub(crate) name: String,
    //pub(crate) type_info: TypeId,
    pub(crate) tag: usize,
}
