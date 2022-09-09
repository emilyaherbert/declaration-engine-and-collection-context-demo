mod declaration;
mod expression;

use std::collections::HashMap;

use declaration::*;
use expression::*;

use crate::{
    collection_context::collection_context::{CollectionContext, CollectionIndex},
    language::{
        parsed::{Application, File, Node},
        ty::{TyApplication, TyFile, TyNode},
    },
    type_system::type_mapping::TypeMapping,
};

pub(crate) fn to_ty(collection_ctxt: &mut CollectionContext, application: Application) {
    let application_index = collection_ctxt.add_node((TyApplication { files: vec![] }).into());
    let files = application
        .files
        .into_iter()
        .map(|file| to_ty_file(collection_ctxt, application_index, file))
        .collect::<Vec<TyFile>>();
}

fn to_ty_file(
    collection_ctxt: &mut CollectionContext,
    parent_index: CollectionIndex,
    file: File,
) -> TyFile {
    TyFile {
        name: file.name,
        nodes: to_ty_nodes(collection_ctxt, file.nodes),
    }
}

fn to_ty_nodes(collection_ctxt: &mut CollectionContext, nodes: Vec<Node>) -> Vec<TyNode> {
    let type_mapping = HashMap::new();
    nodes
        .into_iter()
        .map(|node| to_ty_node(collection_ctxt, &type_mapping, node))
        .collect()
}

fn to_ty_node(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    node: Node,
) -> TyNode {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => {
            TyNode::Declaration(to_ty_declaration(collection_ctxt, type_mapping, decl))
        }
        Node::Expression(expression) => {
            TyNode::Expression(to_ty_expression(type_mapping, expression))
        }
        Node::ReturnStatement(expression) => {
            TyNode::ReturnStatement(to_ty_expression(type_mapping, expression))
        }
    }
}
