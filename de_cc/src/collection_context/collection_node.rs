use std::fmt;

use crate::{
    declaration_engine::declaration_id::DeclarationId,
    language::ty::{
        typed_declaration::TyVariableDeclaration, typed_expression::TyExpression, TyApplication,
        TyFile,
    },
};

#[derive(Clone, Debug)]
pub(crate) enum CollectionNode {
    StarImport(String),
    Application(TyApplication),
    File(TyFile),
    Expression(TyExpression),
    Return(TyExpression),
    Variable(String, TyVariableDeclaration),
    Function(String, DeclarationId),
    Trait(String, DeclarationId),
    TraitFn(String, DeclarationId),
    TraitImpl(String, DeclarationId),
    Struct(String, DeclarationId),
}

// impl fmt::Debug for CollectionNode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             CollectionNode::Application(node) => write!(f, "{:?}", node),
//             CollectionNode::File(node) => write!(f, "{:?}", node),
//             CollectionNode::Node(node) => write!(f, "{:?}", node),
//             CollectionNode::Declaration(_, node) => write!(f, "{:?}", node),
//             CollectionNode::Expression(node) => write!(f, "{:?}", node),
//             CollectionNode::Function(_, node) => write!(f, "{:?}", node),
//             CollectionNode::Trait(_, node) => write!(f, "{:?}", node),
//             CollectionNode::TraitFn(_, node) => write!(f, "{:?}", node),
//             CollectionNode::TraitImpl(_, node) => write!(f, "{:?}", node),
//             CollectionNode::Struct(_, node) => write!(f, "{:?}", node),
//         }
//     }
// }

impl fmt::Display for CollectionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionNode::StarImport(name) => write!(f, "{}", name),
            CollectionNode::Application(node) => write!(f, "{}", node),
            CollectionNode::File(node) => write!(f, "{}", node),
            CollectionNode::Expression(node) => write!(f, "{}", node),
            CollectionNode::Return(node) => write!(f, "{}", node),
            CollectionNode::Variable(_, node) => write!(f, "{}", node),
            CollectionNode::Function(_, node) => write!(f, "{}", node),
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
