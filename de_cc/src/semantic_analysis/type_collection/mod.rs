mod declaration;

use declaration::*;

use crate::language::{
    semi::{SemiApplication, SemiFile, SemiNode},
    untyped::{Application, File, Node},
};

pub(crate) fn collect_types(application: Application) -> SemiApplication {
    let files = application
        .files
        .into_iter()
        .map(collect_types_file)
        .collect();
    SemiApplication { files }
}

fn collect_types_file(file: File) -> SemiFile {
    let nodes = file
        .nodes
        .into_iter()
        .map(collect_types_node)
        .collect::<Vec<_>>();
    SemiFile {
        name: file.name,
        nodes,
    }
}

fn collect_types_node(node: Node) -> SemiNode {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => SemiNode::Declaration(collect_types_declaration(decl)),
        Node::Expression(exp) => SemiNode::Expression(exp),
        Node::ReturnStatement(exp) => SemiNode::ReturnStatement(exp),
    }
}
