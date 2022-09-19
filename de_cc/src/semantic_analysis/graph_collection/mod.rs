//! This module transforms the untyped AST into a typeable AST and collects a [CollectionContext]
//! in the process.
//!
//! In transforming the untyped AST into a typeable AST, this module:
//! 1. inserts instances of [TypeInfo](crate::type_system::type_info::TypeInfo)
//!     into the [TypeEngine](crate::type_system::type_engine::TypeEngine)
//! 2. inserts declarations into the [DeclarationEngine](crate::declaration_engine::declaration_engine::DeclarationEngine)
//!
//! This module does not *and should not*:
//! - evaluate types in any way
//! - create constraints on types or perform type unification
//! - resolve instances of custom types

mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_edge::CollectionEdge,
        collection_index::CCIdx, collection_node::CollectionNode,
    },
    language::{
        parsed::{Application, File, Node},
        ty::{TyApplication, TyFile, TyNode},
    },
};

/// Takes an untyped [Application] struct and transforms it into a typeable [TyApplication],
/// wrapped in a [CCIdx] representing its index in the [CollectionContext]
pub(crate) fn collect_graph(cc: &mut CollectionContext, app: Application) -> CCIdx<TyApplication> {
    // create graph nodes for each of the files
    let file_idxs = app
        .files
        .into_iter()
        .map(|file| collect_graph_file(cc, file))
        .collect::<Vec<_>>();

    // create a graph node for this application
    let app = TyApplication {
        files: file_idxs.clone(),
    };
    let app_idx = cc.add_node(app.clone().into());
    let cc_idx = CCIdx::new(app, app_idx);

    // add a graph edge from every file to the application
    CCIdx::add_edges_many_to_one(&file_idxs, &cc_idx, CollectionEdge::ApplicationContents, cc);

    cc_idx
}

fn collect_graph_file(cc: &mut CollectionContext, file: File) -> CCIdx<TyFile> {
    // create graph nodes for the ast nodes
    let nodes = file
        .nodes
        .into_iter()
        .map(|node| collect_graph_node(cc, node))
        .collect::<Vec<_>>();

    // for every ast node in this scope, connect them under the same shared scope with graph edges
    CCIdx::add_edges_many(&nodes, CollectionEdge::SharedScope, cc);

    // create a graph node for this file
    let file = TyFile {
        name: file.name,
        nodes: nodes.clone(),
    };
    let file_idx = cc.add_node(file.clone().into());

    // register the file with the collection context
    cc.register_file_index(file.name.clone(), file_idx);

    let cc_idx = CCIdx::new(file, file_idx);

    // add a graph edge from every ast node to the file
    CCIdx::add_edges_many_to_one(&nodes, &cc_idx, CollectionEdge::FileContents, cc);

    cc_idx
}

fn collect_graph_node(cc: &mut CollectionContext, node: Node) -> CCIdx<TyNode> {
    match node {
        Node::StarImport(filename) => {
            let node = TyNode::StarImport(filename.clone());
            let node_idx = cc.add_node(CollectionNode::StarImport(filename));
            CCIdx::new(node, node_idx)
        }
        Node::Declaration(decl) => {
            let decl_cc_idx = collect_graph_decl(cc, decl);
            let node = TyNode::Declaration(decl_cc_idx.clone());
            CCIdx::new(node, decl_cc_idx.idx())
        }
        Node::Expression(expression) => {
            let exp = collect_graph_exp(cc, expression);
            let node = TyNode::Expression(exp.clone());
            let node_idx = cc.add_node(CollectionNode::Expression(exp));
            CCIdx::new(node, node_idx)
        }
        Node::ReturnStatement(expression) => {
            let exp = collect_graph_exp(cc, expression);
            let node = TyNode::ReturnStatement(exp.clone());
            let node_idx = cc.add_node(CollectionNode::Return(exp));
            CCIdx::new(node, node_idx)
        }
    }
}
