mod declaration;

use declaration::*;

use crate::{
    language::{
        semi::{SemiApplication, SemiFile, SemiNode},
        untyped::{Application, File, Node},
    },
    namespace::collection_namespace::CollectionNamespace,
};

pub(crate) fn collect_types(
    namespace: &mut CollectionNamespace,
    application: Application,
) -> SemiApplication {
    let files = application
        .files
        .into_iter()
        .map(|file| collect_types_file(namespace, file))
        .collect();
    SemiApplication { files }
}

fn collect_types_file(namespace: &mut CollectionNamespace, file: File) -> SemiFile {
    let nodes = file
        .nodes
        .into_iter()
        .map(|node| collect_types_node(namespace, node))
        .collect::<Vec<_>>();
    SemiFile {
        name: file.name,
        nodes,
    }
}

fn collect_types_node(namespace: &mut CollectionNamespace, node: Node) -> SemiNode {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => {
            SemiNode::Declaration(collect_types_declaration(namespace, decl))
        }
        Node::Expression(exp) => SemiNode::Expression(exp),
        Node::ReturnStatement(exp) => SemiNode::ReturnStatement(exp),
    }
}
