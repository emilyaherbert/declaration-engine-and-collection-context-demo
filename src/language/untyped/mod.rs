use self::{declaration::Declaration, expression::Expression};

pub mod declaration;
pub mod expression;

#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Declaration(Declaration),
    Expression(Expression),
    ReturnStatement(Expression),
}

pub mod constructors {
    use super::{Expression, Node};

    pub fn exp(exp: Expression) -> Node {
        Node::Expression(exp)
    }

    pub fn return_(exp: Expression) -> Node {
        Node::ReturnStatement(exp)
    }
}
