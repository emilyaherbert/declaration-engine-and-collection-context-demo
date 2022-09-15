mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    collection_context::{collection_context::CollectionContext, collection_index::CCIdx},
    language::ty::{TyApplication, TyFile, TyNode},
    namespace::namespace::Namespace,
};

pub(crate) fn analyze(
    cc: &CollectionContext,
    ns: &mut Namespace,
    application: &mut CCIdx<TyApplication>,
) {
    application
        .inner_ref_mut()
        .files
        .iter_mut()
        .for_each(|file| analyze_file(cc, ns, file));
}

fn analyze_file(cc: &CollectionContext, ns: &mut Namespace, file: &mut CCIdx<TyFile>) {
    file.inner_ref_mut()
        .nodes
        .iter_mut()
        .for_each(|node| analyze_node(cc, ns, node));
}

fn analyze_node(cc: &CollectionContext, ns: &mut Namespace, node: &mut CCIdx<TyNode>) {
    let cc_idx = node.idx();
    match node.inner_ref_mut() {
        TyNode::Declaration(decl) => analyze_declaration(cc, ns, decl),
        TyNode::Expression(expression) => analyze_expression(cc, cc_idx, ns, expression),
        TyNode::ReturnStatement(expression) => analyze_expression(cc, cc_idx, ns, expression),
    }
}
