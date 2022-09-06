use std::fmt;

use colored::Colorize;

use self::semi_declaration::SemiDeclaration;

use super::untyped::expression::Expression;

pub(crate) mod semi_declaration;

pub struct SemiApplication {
    pub files: Vec<SemiFile>,
}

impl fmt::Display for SemiApplication {
    #[allow(clippy::useless_format)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}\n{}",
            format!("\n++++++++ UNTYPED").red(),
            self.files
                .iter()
                .map(|file| file.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
            format!("++++++++").red(),
        )
    }
}

pub struct SemiFile {
    pub name: String,
    pub nodes: Vec<SemiNode>,
}

impl fmt::Display for SemiFile {
    #[allow(clippy::useless_format)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(Clone, PartialEq, Debug)]
pub enum SemiNode {
    //StarImport(String),
    Declaration(SemiDeclaration),
    Expression(Expression),
    ReturnStatement(Expression),
}

impl fmt::Display for SemiNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemiNode::Declaration(declaration) => write!(f, "{}", declaration),
            SemiNode::Expression(expression) => write!(f, "{}", expression),
            SemiNode::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
}
