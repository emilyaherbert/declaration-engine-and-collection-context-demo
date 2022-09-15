use std::fmt;

use colored::Colorize;

use crate::{
    collection_context::{collection_context::CollectionContext, collection_index::CCIdx},
    type_system::type_mapping::TypeMapping,
    types::copy_types::CopyTypes,
};

use self::{typed_declaration::TyDeclaration, typed_expression::TyExpression};

pub(crate) mod typed_declaration;
pub(crate) mod typed_expression;

#[derive(Clone, Debug)]
pub(crate) struct TyApplication {
    files: Vec<CCIdx<TyFile>>,
}

impl fmt::Display for TyApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}\n{}",
            format!("\n++++++++ RESOLVED").blue(),
            self.files
                .iter()
                .map(|file| file.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
            format!("++++++++").blue(),
        )
    }
}

impl CopyTypes for TyApplication {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        self.files
            .iter_mut()
            .for_each(|file| file.copy_types(cc, type_mapping));
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TyFile {
    pub(crate) name: String,
    pub(crate) nodes: Vec<CCIdx<TyNode>>,
}

impl fmt::Display for TyFile {
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

impl CopyTypes for TyFile {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        self.nodes
            .iter_mut()
            .for_each(|node| node.copy_types(cc, type_mapping));
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum TyNode {
    Declaration(CCIdx<TyDeclaration>),
    Expression(CCIdx<TyExpression>),
    ReturnStatement(CCIdx<TyExpression>),
}

impl fmt::Display for TyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyNode::Declaration(decl) => write!(f, "{}", decl),
            TyNode::Expression(exp) => write!(f, "{}", exp),
            TyNode::ReturnStatement(exp) => write!(f, "{}", exp),
        }
    }
}

impl CopyTypes for TyNode {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        match self {
            TyNode::Declaration(decl) => decl.copy_types(cc, type_mapping),
            TyNode::Expression(exp) => exp.copy_types(cc, type_mapping),
            TyNode::ReturnStatement(exp) => exp.copy_types(cc, type_mapping),
        }
    }
}
