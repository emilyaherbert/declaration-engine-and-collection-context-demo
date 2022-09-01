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

#[derive(Clone, Debug, PartialEq)]
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

impl CopyTypes for TypedNode {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TypedNode::Declaration(declaration) => declaration.copy_types(type_mapping),
            TypedNode::Expression(expression) => expression.copy_types(type_mapping),
            TypedNode::ReturnStatement(expression) => expression.copy_types(type_mapping),
        }
    }
}
