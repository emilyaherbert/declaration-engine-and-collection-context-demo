use std::fmt;

use crate::{
    declaration_engine::declaration_id::DeclarationId,
    language::ty::{
        typed_declaration::{TyCodeBlock, TyVariableDeclaration},
        typed_expression::TyExpression,
        TyApplication, TyFile,
    },
};

#[derive(Clone)]
pub(crate) enum CollectionNode {
    StarImport(String),
    Application(TyApplication),
    File(TyFile),
    Expression(TyExpression),
    Return(TyExpression),
    Variable(String, TyVariableDeclaration),
    Function(String, DeclarationId),
    CodeBlock(TyCodeBlock),
    Trait(String, DeclarationId),
    TraitFn(String, DeclarationId),
    TraitImpl(String, DeclarationId),
    Struct(String, DeclarationId),
}

impl fmt::Debug for CollectionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionNode::StarImport(name) => write!(f, "use {}::*;", name),
            CollectionNode::Application(_) => write!(f, "entire application"),
            CollectionNode::File(node) => write!(f, "{}", node.name),
            CollectionNode::Expression(node) => write!(f, "{:?}", node),
            CollectionNode::Return(node) => write!(f, "return {:?};", node),
            CollectionNode::Variable(_, node) => write!(f, "{}", node),
            CollectionNode::Function(_, node) => write!(f, "{:?}", node),
            CollectionNode::CodeBlock(node) => write!(f, "{}", node),
            CollectionNode::Trait(_, node) => write!(f, "{:?}", node),
            CollectionNode::TraitFn(_, node) => write!(f, "{:?}", node),
            CollectionNode::TraitImpl(_, node) => write!(f, "{:?}", node),
            CollectionNode::Struct(_, node) => write!(f, "{:?}", node),
        }
    }
}

impl fmt::Display for CollectionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionNode::StarImport(name) => write!(f, "use {}::*;", name),
            CollectionNode::Application(_) => write!(f, "entire application"),
            CollectionNode::File(node) => write!(f, "{}", node.name),
            CollectionNode::Expression(node) => write!(f, "{}", node),
            CollectionNode::Return(node) => write!(f, "return {};", node),
            CollectionNode::Variable(_, node) => write!(f, "{}", node),
            CollectionNode::Function(_, node) => write!(f, "{}", node),
            CollectionNode::CodeBlock(node) => write!(f, "{}", node),
            CollectionNode::Trait(_, node) => write!(f, "{}", node),
            CollectionNode::TraitFn(_, node) => write!(f, "{}", node),
            CollectionNode::TraitImpl(_, node) => write!(f, "{}", node),
            CollectionNode::Struct(_, node) => write!(f, "{}", node),
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

impl From<TyExpression> for CollectionNode {
    fn from(node: TyExpression) -> Self {
        CollectionNode::Expression(node)
    }
}
