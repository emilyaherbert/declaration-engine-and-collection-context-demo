mod declaration;
mod expression;

use std::collections::HashMap;

use declaration::*;
use expression::*;

use crate::{
    language::{
        parsed::{Application, File, Node},
        ty::{TyApplication, TyFile, TyNode},
    },
    type_system::type_mapping::TypeMapping,
};

pub(crate) fn to_ty(application: Application) -> TyApplication {
    let files = application
        .files
        .into_iter()
        .map(|file| to_ty_file(file))
        .collect();
    TyApplication { files }
}

fn to_ty_file(file: File) -> TyFile {
    TyFile {
        name: file.name,
        nodes: to_ty_nodes(file.nodes),
    }
}

fn to_ty_nodes(nodes: Vec<Node>) -> Vec<TyNode> {
    let type_mapping = HashMap::new();
    nodes
        .into_iter()
        .map(|node| to_ty_node(&type_mapping, node))
        .collect()
}

fn to_ty_node(type_mapping: &TypeMapping, node: Node) -> TyNode {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => TyNode::Declaration(to_ty_declaration(type_mapping, decl)),
        Node::Expression(expression) => TyNode::Expression(to_ty_expression(expression)),
        Node::ReturnStatement(expression) => TyNode::Expression(to_ty_expression(expression)),
    }
}
