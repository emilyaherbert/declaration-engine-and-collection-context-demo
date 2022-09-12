use crate::{
    language::ty::{typed_declaration::TyDeclaration, TyApplication, TyFile, TyNode},
    type_system::type_mapping::TypeMapping,
    types::{copy_types::CopyTypes, pretty_print::PrettyPrint},
};

use super::collection_context::CollectionContext;

#[derive(Clone)]
pub(crate) enum GraphNode {
    Application(TyApplication),
    File(TyFile),
    Node(TyNode),
    Declaration(TyDeclaration),
}

impl PrettyPrint for GraphNode {
    fn pretty_print(&self, cc: &CollectionContext) -> String {
        match self {
            GraphNode::Application(node) => node.pretty_print(cc),
            GraphNode::File(node) => node.pretty_print(cc),
            GraphNode::Node(node) => node.to_string(),
            GraphNode::Declaration(node) => node.to_string(),
        }
    }
}

impl CopyTypes for GraphNode {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        match self {
            GraphNode::Application(node) => node.copy_types(cc, type_mapping),
            GraphNode::File(node) => node.copy_types(cc, type_mapping),
            GraphNode::Node(node) => node.copy_types(cc, type_mapping),
            GraphNode::Declaration(node) => node.copy_types(cc, type_mapping),
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
}
