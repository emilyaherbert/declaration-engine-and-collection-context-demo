use colored::Colorize;
use std::fmt;

use self::{declaration::Declaration, expression::Expression};

pub mod declaration;
pub mod expression;

pub struct Application {
    pub files: Vec<File>,
}

impl fmt::Display for Application {
    #[allow(clippy::useless_format)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}\n{}",
            format!("\n++++++++ UNTYPED").red(),
            self.files
                .iter()
                .map(|program| program.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
            format!("++++++++").red(),
        )
    }
}

pub struct File {
    pub name: String,
    pub nodes: Vec<Node>,
}

impl fmt::Display for File {
    #[allow(clippy::useless_format)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut nodes_str = self
            .nodes
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<_>>()
            .join(";\n");
        nodes_str.insert(0, '\n');
        nodes_str.push(';');
        write!(
            f,
            "{}{}{}",
            format!("\n>>> {}", self.name).green(),
            nodes_str,
            format!("\n<<<").green(),
        )
    }
}

#[derive(Clone)]
pub enum Node {
    StarImport(String),
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
            Node::StarImport(name) => write!(f, "use {}::*", name),
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
