mod typed_declaration;
mod typed_expression;

pub(crate) use typed_declaration::*;
pub(crate) use typed_expression::*;

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
