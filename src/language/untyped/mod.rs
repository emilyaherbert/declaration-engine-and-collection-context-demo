use std::fmt;

use self::{declaration::Declaration, expression::Expression};

pub mod declaration;
pub mod expression;

#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();
        builder.push_str("\n\n>>>\n");
        for node in self.nodes.iter() {
            builder.push_str(&node.to_string());
            builder.push_str(";\n");
        }
        builder.push_str("<<<\n");
        write!(f, "{}", builder)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Declaration(Declaration),
    Expression(Expression),
    ReturnStatement(Expression),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Declaration(declaration) => write!(f, "{}", declaration),
            Node::Expression(expression) => write!(f, "{}", expression),
            Node::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
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
