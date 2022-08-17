use std::fmt;

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

#[derive(Clone, PartialEq)]
pub(crate) enum TypedNode {
    StarImport(String),
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
            TypedNode::StarImport(name) => write!(f, "use {}::*", name),
        }
    }
}
