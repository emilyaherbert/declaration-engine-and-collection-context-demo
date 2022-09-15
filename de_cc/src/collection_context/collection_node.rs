use std::fmt;

use crate::{
    declaration_engine::{
        declaration_engine::{de_get_function, de_get_struct, de_get_trait, de_get_trait_impl},
        declaration_id::DeclarationId,
    },
    language::ty::{
        typed_declaration::TyDeclaration, typed_expression::TyExpression, TyApplication, TyFile,
        TyNode,
    },
    type_system::type_mapping::TypeMapping,
    types::copy_types::CopyTypes,
};

use super::collection_context::CollectionContext;

#[derive(Clone, Debug)]
pub(crate) enum CollectionNode<'cc> {
    Application(&'cc TyApplication),
    File(&'cc TyFile),
    Node(&'cc TyNode),
    Declaration(String, &'cc TyDeclaration),
    Expression(&'cc TyExpression),
    Function(String, DeclarationId),
    Trait(String, DeclarationId),
}

impl fmt::Display for CollectionNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionNode::Application(node) => write!(f, "{}", node),
            CollectionNode::File(node) => write!(f, "{}", node),
            CollectionNode::Node(node) => write!(f, "{}", node),
            CollectionNode::Declaration(_, node) => write!(f, "{}", node),
            CollectionNode::Expression(node) => write!(f, "{}", node),
            CollectionNode::Function(_, node) => write!(f, "{}", node),
            CollectionNode::Trait(_, node) => write!(f, "{}", node),
        }
    }
}

impl CopyTypes for CollectionNode<'_> {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        match self {
            CollectionNode::Application(node) => node.copy_types(cc, type_mapping),
            CollectionNode::File(node) => node.copy_types(cc, type_mapping),
            CollectionNode::Node(node) => node.copy_types(cc, type_mapping),
            CollectionNode::Declaration(_, node) => node.copy_types(cc, type_mapping),
            CollectionNode::Expression(node) => node.copy_types(cc, type_mapping),
            CollectionNode::Function(_, node) => node.copy_types(cc, type_mapping),
            CollectionNode::Trait(_, node) => node.copy_types(cc, type_mapping),
        }
    }
}

impl From<&TyApplication> for CollectionNode<'_> {
    fn from(node: &TyApplication) -> Self {
        CollectionNode::Application(node)
    }
}

impl From<&TyFile> for CollectionNode<'_> {
    fn from(node: &TyFile) -> Self {
        CollectionNode::File(node)
    }
}

impl From<&TyNode> for CollectionNode<'_> {
    fn from(node: &TyNode) -> Self {
        CollectionNode::Node(node)
    }
}

impl From<&TyDeclaration> for CollectionNode<'_> {
    fn from(node: &TyDeclaration) -> Self {
        match node {
            TyDeclaration::Variable(decl) => CollectionNode::Declaration(decl.name.clone(), node),
            TyDeclaration::Function(decl_id) => {
                let decl = de_get_function(decl_id.inner()).unwrap();
                CollectionNode::Declaration(decl.name, node)
            }
            TyDeclaration::Trait(decl_id) => {
                let decl = de_get_trait(decl_id.inner()).unwrap();
                CollectionNode::Declaration(decl.name, node)
            }
            TyDeclaration::TraitImpl(decl_id) => {
                let decl = de_get_trait_impl(decl_id.inner()).unwrap();
                CollectionNode::Declaration(
                    format!("{}+for+{}", decl.trait_name, decl.type_implementing_for),
                    node,
                )
            }
            TyDeclaration::Struct(decl_id) => {
                let decl = de_get_struct(decl_id.inner()).unwrap();
                CollectionNode::Declaration(decl.name, node)
            }
        }
    }
}

impl From<&TyExpression> for CollectionNode<'_> {
    fn from(node: &TyExpression) -> Self {
        CollectionNode::Expression(node)
    }
}

impl<'cc> CollectionNode<'cc> {
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
