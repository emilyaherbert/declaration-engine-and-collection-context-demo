use std::fmt;

use crate::{type_system::type_mapping::TypeMapping, types::copy_types::CopyTypes};

use self::{typed_declaration::TyDeclaration, typed_expression::TyExpression};

pub(crate) mod typed_declaration;
pub(crate) mod typed_expression;

pub(crate) struct TyApplication {
    pub files: Vec<TyFile>,
}

pub(crate) struct TyFile {
    pub(crate) name: String,
    pub(crate) nodes: Vec<TyNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TyNode {
    // StarImport(String),
    Declaration(TyDeclaration),
    Expression(TyExpression),
    ReturnStatement(TyExpression),
}

impl fmt::Display for TyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyNode::Declaration(declaration) => write!(f, "{}", declaration),
            TyNode::Expression(expression) => write!(f, "{}", expression),
            TyNode::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
}

impl CopyTypes for TyNode {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TyNode::Declaration(declaration) => declaration.copy_types(type_mapping),
            TyNode::Expression(expression) => expression.copy_types(type_mapping),
            TyNode::ReturnStatement(expression) => expression.copy_types(type_mapping),
        }
    }
}
