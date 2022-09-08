mod declaration;

use declaration::*;

use crate::{
    language::ty::{TyApplication, TyFile, TyNode},
    namespace::namespace::Namespace,
};

pub(crate) fn collect_types(namespace: &mut Namespace, application: &TyApplication) {
    application
        .files
        .iter()
        .for_each(|file| collect_types_file(namespace, file));
}

fn collect_types_file(namespace: &mut Namespace, file: &TyFile) {
    file.nodes
        .iter()
        .for_each(|node| collect_types_node(namespace, node));
}

fn collect_types_node(namespace: &mut Namespace, node: &TyNode) {
    match node {
        TyNode::Declaration(decl) => collect_types_declaration(namespace, decl),
        TyNode::Expression(_) => {}
        TyNode::ReturnStatement(_) => {}
    }
}
