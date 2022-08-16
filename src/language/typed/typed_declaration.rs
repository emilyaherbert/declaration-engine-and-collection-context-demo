use std::fmt;

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

impl TypedDeclaration {
    pub(crate) fn expect_variable(self) -> Result<TypedVariableDeclaration, String> {
        if let TypedDeclaration::Variable(variable_declaration) = self {
            Ok(variable_declaration)
        } else {
            Err("not a variable declaration".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedVariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: TypeId,
    pub(crate) body: TypedExpression,
}

impl fmt::Display for TypedVariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {}: {} = {}",
            self.name, self.type_ascription, self.body
        )
    }
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
