use std::fmt;

use self::{typed_declaration::TypedDeclaration, typed_expression::TypedExpression};

pub(crate) mod typed_declaration;
pub(crate) mod typed_expression;

#[derive(Debug)]
pub(crate) struct TypedTree {
    pub(crate) nodes: Vec<TypedNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypedNode {
    Declaration(TypedDeclaration),
    Expression(TypedExpression),
    ReturnStatement(TypedExpression),
}

impl fmt::Display for TypedNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypedNode::Declaration(declaration) => write!(f, "{}", declaration),
            TypedNode::Expression(expression) => write!(f, "{}", expression),
            TypedNode::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
}
