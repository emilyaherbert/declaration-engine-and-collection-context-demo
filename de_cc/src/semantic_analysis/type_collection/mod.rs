//! This module performs type collection on the typeable AST.
//!
//! Type collection includes:
//! 1. visiting all types that touch intraprocedural objects
//!     (struct/enum/function/trait/etc declarations)
//! 2. resolving custom types
//! 3. applying CopyTypes to associate type parameters with generics
//!
//! Type collection does not include:
//! - visiting types that do not touch intraprocedural objects
//!     (function bodies are not visited)

mod declaration;

use declaration::*;

use crate::{
    collection_context::{collection_context::CollectionContext, collection_index::CCIdx},
    language::ty::{TyApplication, TyFile, TyNode},
};

pub(crate) fn collect_types(cc: &CollectionContext, application: &mut CCIdx<TyApplication>) {
    application
        .inner_ref_mut()
        .files
        .iter_mut()
        .for_each(|file| collect_types_file(cc, file));
}

fn collect_types_file(cc: &CollectionContext, file: &mut CCIdx<TyFile>) {
    file.inner_ref_mut()
        .nodes
        .iter_mut()
        .for_each(|node| collect_types_node(cc, node));
}

fn collect_types_node(cc: &CollectionContext, node: &mut CCIdx<TyNode>) {
    match node.inner_ref_mut() {
        TyNode::Declaration(decl) => collect_types_declaration(cc, decl),
        TyNode::Expression(_) => {}
        TyNode::ReturnStatement(_) => {}
        TyNode::StarImport(_) => {}
    }
}
