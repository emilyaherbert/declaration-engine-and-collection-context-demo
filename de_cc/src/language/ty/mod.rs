use std::fmt;

use colored::Colorize;

use crate::{
    collection_context::collection_index::CollectionIndex, type_system::type_mapping::TypeMapping,
    types::copy_types::CopyTypes,
};

use self::{typed_declaration::TyDeclaration, typed_expression::TyExpression};

pub(crate) mod typed_declaration;
pub(crate) mod typed_expression;

pub(crate) struct TyApplication {
    pub files: Vec<CollectionIndex>,
}

impl fmt::Display for TyApplication {
    #[allow(clippy::useless_format)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}\n{}",
            format!("\n++++++++ RESOLVED").blue(),
            self.files
                .iter()
                .map(|program| program.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
            format!("++++++++").blue(),
        )
    }
}

pub(crate) struct TyFile {
    pub(crate) name: String,
    pub(crate) nodes: Vec<CollectionIndex>,
}

impl fmt::Display for TyFile {
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

#[derive(Clone, PartialEq)]
pub(crate) enum TyNode {
    // StarImport(String),
    Declaration(TyDeclaration),
    Expression(TyExpression),
    ReturnStatement(TyExpression),
}

impl fmt::Display for TyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyNode::Declaration(declaration) => write!(f, "{}", declaration),
            TyNode::Expression(expression) => write!(f, "{}", expression),
            TyNode::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
}

impl CopyTypes for TyNode {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TyNode::Declaration(declaration) => declaration.copy_types(type_mapping),
            TyNode::Expression(expression) => expression.copy_types(type_mapping),
            TyNode::ReturnStatement(expression) => expression.copy_types(type_mapping),
        }
    }
}
