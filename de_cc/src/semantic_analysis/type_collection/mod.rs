mod declaration;

use declaration::*;

use crate::{
    language::{
        partial::{PartialApplication, PartialFile, PartialNode},
        untyped::{Application, File, Node},
    },
    namespace::collection_namespace::CollectionNamespace,
};

pub(crate) fn collect_types(
    namespace: &mut CollectionNamespace,
    application: Application,
) -> PartialApplication {
    let files = application
        .files
        .into_iter()
        .map(|file| collect_types_file(namespace, file))
        .collect();
    PartialApplication { files }
}

fn collect_types_file(namespace: &mut CollectionNamespace, file: File) -> PartialFile {
    let nodes = file
        .nodes
        .into_iter()
        .map(|node| collect_types_node(namespace, node))
        .collect::<Vec<_>>();
    PartialFile {
        name: file.name,
        nodes,
    }
}

fn collect_types_node(namespace: &mut CollectionNamespace, node: Node) -> PartialNode {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => {
            PartialNode::Declaration(collect_types_declaration(namespace, decl))
        }
        Node::Expression(exp) => PartialNode::Expression(exp),
        Node::ReturnStatement(exp) => PartialNode::ReturnStatement(exp),
    }
}
