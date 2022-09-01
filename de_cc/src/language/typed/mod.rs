use std::fmt;

use crate::{type_system::type_mapping::TypeMapping, types::copy_types::CopyTypes};

use self::{typed_declaration::TypedDeclaration, typed_expression::TypedExpression};

pub(crate) mod typed_declaration;
pub(crate) mod typed_expression;

pub(crate) struct TypedApplication {
    pub files: Vec<TypedFile>,
}

pub(crate) struct TypedFile {
    pub(crate) name: String,
    pub(crate) nodes: Vec<TypedNode>,
}

#[derive(Clone, Debug)]
pub(crate) enum TypedNode {
    // StarImport(String),
    Declaration(TypedDeclaration),
    Expression(TypedExpression),
    ReturnStatement(TypedExpression),
}

impl fmt::Display for TypedNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypedNode::Declaration(declaration) => write!(f, "{}", declaration),
            TypedNode::Expression(expression) => write!(f, "{}", expression),
            TypedNode::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
}

impl PartialEq for TypedNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypedNode::Declaration(l), TypedNode::Declaration(r)) => l == r,
            (TypedNode::Expression(l), TypedNode::Expression(r)) => l == r,
            (TypedNode::ReturnStatement(l), TypedNode::ReturnStatement(r)) => l == r,
            _ => false,
        }
    }
}

impl CopyTypes for TypedNode {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TypedNode::Declaration(declaration) => declaration.copy_types(type_mapping),
            TypedNode::Expression(expression) => expression.copy_types(type_mapping),
            TypedNode::ReturnStatement(expression) => expression.copy_types(type_mapping),
        }
    }
}
