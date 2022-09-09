mod declaration;
mod expression;

use std::collections::HashMap;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_index::CollectionIndex,
    },
    language::{
        parsed::{Application, File, Node},
        ty::{TyApplication, TyFile, TyNode},
    },
    type_system::type_mapping::TypeMapping,
};

pub(crate) fn collect_nodes(collection_ctxt: &mut CollectionContext, application: Application) {
    let application_index = collection_ctxt.add_node((TyApplication { files: vec![] }).into());
    let files = application
        .files
        .into_iter()
        .map(|file| collect_nodes_file(collection_ctxt, application_index, file))
        .collect::<Vec<TyFile>>();
}

fn collect_nodes_file(
    collection_ctxt: &mut CollectionContext,
    parent_index: CollectionIndex,
    file: File,
) -> TyFile {
    TyFile {
        name: file.name,
        nodes: collect_nodes_nodes(collection_ctxt, file.nodes),
    }
}

fn collect_nodes_nodes(collection_ctxt: &mut CollectionContext, nodes: Vec<Node>) -> Vec<TyNode> {
    let type_mapping = HashMap::new();
    nodes
        .into_iter()
        .map(|node| collect_nodes_node(collection_ctxt, &type_mapping, node))
        .collect()
}

fn collect_nodes_node(
    collection_ctxt: &mut CollectionContext,
    type_mapping: &TypeMapping,
    node: Node,
) -> TyNode {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => TyNode::Declaration(collect_nodes_declaration(
            collection_ctxt,
            type_mapping,
            decl,
        )),
        Node::Expression(expression) => {
            TyNode::Expression(collect_nodes_expression(type_mapping, expression))
        }
        Node::ReturnStatement(expression) => {
            TyNode::ReturnStatement(collect_nodes_expression(type_mapping, expression))
        }
    }
}
