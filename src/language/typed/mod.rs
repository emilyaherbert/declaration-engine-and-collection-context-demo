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
