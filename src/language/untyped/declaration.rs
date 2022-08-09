use super::{expression::*, Node};

use crate::type_system::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
    Trait(TraitDeclaration),
    Struct(StructDeclaration),
    Enum(EnumDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VariableDeclaration {
    pub(crate) name: String,
    pub(crate) body: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) body: Vec<Node>,
    pub(crate) return_type: TypeInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FunctionParameter {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TraitDeclaration {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<TraitFn>,
    pub(crate) methods: Vec<FunctionDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TraitFn {
    pub(crate) name: String,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) return_type: TypeInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StructDeclaration {
    pub(crate) name: String,
    pub(crate) fields: Vec<StructField>,
    pub(crate) type_parameters: Vec<TypeParameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StructField {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EnumDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EnumVariant {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
    pub(crate) tag: usize,
}
