use crate::{
    declaration_engine::declaration_engine::{
        de_get_function, de_get_struct, de_get_trait, de_get_trait_impl,
    },
    language::ty::{typed_declaration::TyDeclaration, TyApplication, TyFile, TyNode},
    type_system::type_mapping::TypeMapping,
    types::{copy_types::CopyTypes, pretty_print::PrettyPrint},
};

use super::collection_context::CollectionContext;

#[derive(Clone, Debug)]
pub(crate) enum GraphNode {
    Application(TyApplication),
    File(TyFile),
    Node(TyNode),
    Declaration(String, TyDeclaration),
}

impl PrettyPrint for GraphNode {
    fn pretty_print(&self, cc: &CollectionContext) -> String {
        match self {
            GraphNode::Application(node) => node.pretty_print(cc),
            GraphNode::File(node) => node.pretty_print(cc),
            GraphNode::Node(node) => node.pretty_print(cc),
            GraphNode::Declaration(_, node) => node.to_string(),
        }
    }

    fn pretty_print_debug(&self, cc: &CollectionContext) -> String {
        match self {
            GraphNode::Application(node) => node.pretty_print_debug(cc),
            GraphNode::File(node) => node.pretty_print_debug(cc),
            GraphNode::Node(node) => node.pretty_print_debug(cc),
            GraphNode::Declaration(_, node) => format!("{:?}", node),
        }
    }
}

impl CopyTypes for GraphNode {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        match self {
            GraphNode::Application(node) => node.copy_types(cc, type_mapping),
            GraphNode::File(node) => node.copy_types(cc, type_mapping),
            GraphNode::Node(node) => node.copy_types(cc, type_mapping),
            GraphNode::Declaration(_, node) => node.copy_types(cc, type_mapping),
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
        match &node {
            TyDeclaration::Variable(decl) => GraphNode::Declaration(decl.name.clone(), node),
            TyDeclaration::Function(decl_id) => {
                let decl = de_get_function(*decl_id).unwrap();
                GraphNode::Declaration(decl.name, node)
            }
            TyDeclaration::Trait(decl_id) => {
                let decl = de_get_trait(*decl_id).unwrap();
                GraphNode::Declaration(decl.name, node)
            }
            TyDeclaration::TraitImpl(decl_id) => {
                let decl = de_get_trait_impl(*decl_id).unwrap();
                GraphNode::Declaration(
                    format!("{}+for+{}", decl.trait_name, decl.type_implementing_for),
                    node,
                )
            }
            TyDeclaration::Struct(decl_id) => {
                let decl = de_get_struct(*decl_id).unwrap();
                GraphNode::Declaration(decl.name, node)
            }
        }
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
            GraphNode::Declaration(_, node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }
}
