mod declaration;

use declaration::*;

use crate::{
    collection_context::{collection_context::CollectionContext, collection_index::CCIdx},
    language::ty::{TyApplication, TyFile, TyNode},
    namespace::namespace::Namespace,
};

pub(crate) fn collect_types(
    cc: &CollectionContext,
    ns: &mut Namespace,
    application: &mut CCIdx<TyApplication>,
) {
    application
        .inner_ref_mut()
        .files
        .iter_mut()
        .for_each(|file| collect_types_file(cc, ns, file));
}

fn collect_types_file(cc: &CollectionContext, ns: &mut Namespace, file: &mut CCIdx<TyFile>) {
    file.inner_ref_mut()
        .nodes
        .iter_mut()
        .for_each(|node| collect_types_node(cc, ns, node));
}

fn collect_types_node(cc: &CollectionContext, ns: &mut Namespace, node: &mut CCIdx<TyNode>) {
    match node.inner_ref_mut() {
        TyNode::Declaration(decl) => collect_types_declaration(cc, ns, decl),
        TyNode::Expression(_) => {}
        TyNode::ReturnStatement(_) => {}
    }
}
