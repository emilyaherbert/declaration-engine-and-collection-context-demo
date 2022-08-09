mod declaration;
mod expression;

pub(crate) use declaration::*;
pub(crate) use expression::*;

#[derive(Debug)]
pub(crate) struct Tree {
    pub(crate) nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Node {
    Declaration(Declaration),
    Expression(Expression),
    ReturnStatement(Expression),
}
