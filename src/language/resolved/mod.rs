use colored::Colorize;
use std::fmt;

use self::{resolved_declaration::ResolvedDeclaration, resolved_expression::ResolvedExpression};

pub(crate) mod resolved_declaration;
pub(crate) mod resolved_expression;

pub struct ResolvedApplication {
    pub files: Vec<ResolvedFile>,
}

impl fmt::Display for ResolvedApplication {
    #[allow(clippy::useless_format)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}\n{}",
            format!("\n++++++++ RESOLVED").red(),
            self.files
                .iter()
                .map(|program| program.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
            format!("++++++++").red(),
        )
    }
}

#[derive(Debug)]
pub struct ResolvedFile {
    pub(crate) name: String,
    pub(crate) nodes: Vec<ResolvedNode>,
}

impl fmt::Display for ResolvedFile {
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ResolvedNode {
    Declaration(ResolvedDeclaration),
    Expression(ResolvedExpression),
    ReturnStatement(ResolvedExpression),
}

impl fmt::Display for ResolvedNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedNode::Declaration(declaration) => write!(f, "{}", declaration),
            ResolvedNode::Expression(expression) => write!(f, "{}", expression),
            ResolvedNode::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
}
