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
pub(crate) enum CollectionNode {
    Application(TyApplication),
    File(TyFile),
    Node(TyNode),
    Declaration(String, TyDeclaration),
}

impl PrettyPrint for CollectionNode {
    fn pretty_print(&self, cc: &CollectionContext) -> String {
        match self {
            CollectionNode::Application(node) => node.pretty_print(cc),
            CollectionNode::File(node) => node.pretty_print(cc),
            CollectionNode::Node(node) => node.pretty_print(cc),
            CollectionNode::Declaration(_, node) => node.to_string(),
        }
    }

    fn pretty_print_debug(&self, cc: &CollectionContext) -> String {
        match self {
            CollectionNode::Application(node) => node.pretty_print_debug(cc),
            CollectionNode::File(node) => node.pretty_print_debug(cc),
            CollectionNode::Node(node) => node.pretty_print_debug(cc),
            CollectionNode::Declaration(_, node) => format!("{:?}", node),
        }
    }
}

impl CopyTypes for CollectionNode {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        match self {
            CollectionNode::Application(node) => node.copy_types(cc, type_mapping),
            CollectionNode::File(node) => node.copy_types(cc, type_mapping),
            CollectionNode::Node(node) => node.copy_types(cc, type_mapping),
            CollectionNode::Declaration(_, node) => node.copy_types(cc, type_mapping),
        }
    }
}

impl From<TyApplication> for CollectionNode {
    fn from(node: TyApplication) -> Self {
        CollectionNode::Application(node)
    }
}

impl From<TyFile> for CollectionNode {
    fn from(node: TyFile) -> Self {
        CollectionNode::File(node)
    }
}

impl From<TyNode> for CollectionNode {
    fn from(node: TyNode) -> Self {
        CollectionNode::Node(node)
    }
}

impl From<TyDeclaration> for CollectionNode {
    fn from(node: TyDeclaration) -> Self {
        match &node {
            TyDeclaration::Variable(decl) => CollectionNode::Declaration(decl.name.clone(), node),
            TyDeclaration::Function(decl_id) => {
                let decl = de_get_function(*decl_id).unwrap();
                CollectionNode::Declaration(decl.name, node)
            }
            TyDeclaration::Trait(decl_id) => {
                let decl = de_get_trait(*decl_id).unwrap();
                CollectionNode::Declaration(decl.name, node)
            }
            TyDeclaration::TraitImpl(decl_id) => {
                let decl = de_get_trait_impl(*decl_id).unwrap();
                CollectionNode::Declaration(
                    format!("{}+for+{}", decl.trait_name, decl.type_implementing_for),
                    node,
                )
            }
            TyDeclaration::Struct(decl_id) => {
                let decl = de_get_struct(*decl_id).unwrap();
                CollectionNode::Declaration(decl.name, node)
            }
        }
    }
}

impl CollectionNode {
    pub(crate) fn expect_application(&self) -> Result<&TyApplication, String> {
        match self {
            CollectionNode::Application(node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }

    pub(crate) fn expect_file(&self) -> Result<&TyFile, String> {
        match self {
            CollectionNode::File(node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }

    pub(crate) fn expect_node(&self) -> Result<&TyNode, String> {
        match self {
            CollectionNode::Node(node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }

    pub(crate) fn expect_declaration(&self) -> Result<&TyDeclaration, String> {
        match self {
            CollectionNode::Declaration(_, node) => Ok(node),
            _ => Err("did not expect to find this declaration".to_string()),
        }
    }
}
