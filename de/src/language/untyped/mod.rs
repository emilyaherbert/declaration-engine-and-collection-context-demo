use colored::Colorize;

use crate::{
    declaration_engine::declaration_engine::DeclarationEngine, types::pretty_print::PrettyPrint,
};

use self::{declaration::Declaration, expression::Expression};

pub mod declaration;
pub mod expression;

pub struct Application {
    pub files: Vec<File>,
}

impl PrettyPrint for Application {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!(
            "{}{}\n{}",
            format!("\n++++++++ UNTYPED").red(),
            self.files
                .iter()
                .map(|file| file.pretty_print(declaration_engine))
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

impl PrettyPrint for File {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        let mut nodes_str = self
            .nodes
            .iter()
            .map(|node| node.pretty_print(declaration_engine))
            .collect::<Vec<_>>()
            .join(";\n");
        nodes_str.insert(0, '\n');
        nodes_str.push(';');
        format!(
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

impl PrettyPrint for Node {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            Node::Declaration(declaration) => {
                format!("{}", declaration.pretty_print(declaration_engine))
            }
            Node::Expression(expression) => {
                format!("{}", expression.pretty_print(declaration_engine))
            }
            Node::ReturnStatement(expression) => {
                format!("return {}", expression.pretty_print(declaration_engine))
            }
            Node::StarImport(name) => format!("use {}::*", name),
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
