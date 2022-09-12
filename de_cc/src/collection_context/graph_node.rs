use std::fmt;

use crate::{
    language::ty::{
        typed_declaration::TyDeclaration, typed_expression::TyExpression, TyApplication, TyFile,
        TyNode,
    },
    type_system::type_mapping::TypeMapping,
    types::copy_types::CopyTypes,
};

pub(crate) enum GraphNode {
    Application(TyApplication),
    File(TyFile),
    Node(TyNode),
    Declaration(TyDeclaration),
    Expression(TyExpression),
}

impl fmt::Display for GraphNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphNode::Application(node) => write!(f, "{}", node),
            GraphNode::File(node) => write!(f, "{}", node),
            GraphNode::Node(node) => write!(f, "{}", node),
            GraphNode::Declaration(node) => write!(f, "{}", node),
            GraphNode::Expression(node) => write!(f, "{}", node),
        }
    }
}

impl CopyTypes for GraphNode {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            GraphNode::Application(node) => node.copy_types(type_mapping),
            GraphNode::File(node) => node.copy_types(type_mapping),
            GraphNode::Node(node) => node.copy_types(type_mapping),
            GraphNode::Declaration(node) => node.copy_types(type_mapping),
            GraphNode::Expression(node) => node.copy_types(type_mapping),
        }
    }
}

impl From<TyApplication> for GraphNode {
    fn from(node: TyApplication) -> Self {
        GraphNode::Application(node)
    }
}

impl From<TyFile> for GraphNode {
    fn from(node: TyFile) -> Self {
        GraphNode::File(node)
    }
}

impl From<TyNode> for GraphNode {
    fn from(node: TyNode) -> Self {
        GraphNode::Node(node)
    }
}

impl From<TyDeclaration> for GraphNode {
    fn from(node: TyDeclaration) -> Self {
        GraphNode::Declaration(node)
    }
}

impl From<TyExpression> for GraphNode {
    fn from(node: TyExpression) -> Self {
        GraphNode::Expression(node)
    }
}

impl GraphNode {
    pub(crate) fn expect_application(&self) -> Result<&TyApplication, String> {
        match self {
            GraphNode::Application(node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }

    pub(crate) fn expect_file(&self) -> Result<&TyFile, String> {
        match self {
            GraphNode::File(node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }

    pub(crate) fn expect_node(&self) -> Result<&TyNode, String> {
        match self {
            GraphNode::Node(node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }

    pub(crate) fn expect_declaration(&self) -> Result<&TyDeclaration, String> {
        match self {
            GraphNode::Declaration(node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }

    pub(crate) fn expect_expression(&self) -> Result<&TyExpression, String> {
        match self {
            GraphNode::Expression(node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }
}
