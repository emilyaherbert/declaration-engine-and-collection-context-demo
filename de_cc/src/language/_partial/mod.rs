use std::fmt;

use colored::Colorize;

use self::partial_declaration::PartialDeclaration;

use super::parsed::expression::Expression;

pub(crate) mod partial_declaration;

pub struct PartialApplication {
    pub files: Vec<PartialFile>,
}

impl fmt::Display for PartialApplication {
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

pub struct PartialFile {
    pub name: String,
    pub nodes: Vec<PartialNode>,
}

impl fmt::Display for PartialFile {
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
pub enum PartialNode {
    Declaration(PartialDeclaration),
    Expression(Expression),
    ReturnStatement(Expression),
}

impl fmt::Display for PartialNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PartialNode::Declaration(declaration) => write!(f, "{}", declaration),
            PartialNode::Expression(expression) => write!(f, "{}", expression),
            PartialNode::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
}
