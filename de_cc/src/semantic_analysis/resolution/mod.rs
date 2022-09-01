mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::language::{
    resolved::{ResolvedApplication, ResolvedFile, ResolvedNode},
    typed::{TypedApplication, TypedFile, TypedNode},
};

pub(crate) fn resolve(application: TypedApplication) -> ResolvedApplication {
    let resolved_programs = application.files.into_iter().map(resolve_file).collect();
    ResolvedApplication {
        files: resolved_programs,
    }
}

fn resolve_file(file: TypedFile) -> ResolvedFile {
    let new_nodes = resolve_nodes(file.nodes);
    ResolvedFile {
        name: file.name,
        nodes: new_nodes,
    }
}

fn resolve_nodes(nodes: Vec<TypedNode>) -> Vec<ResolvedNode> {
    nodes.into_iter().flat_map(resolve_node).collect()
}

fn resolve_node(node: TypedNode) -> Vec<ResolvedNode> {
    match node {
        TypedNode::Declaration(declaration) => {
            let declarations = resolve_declaration(declaration);
            declarations
                .into_iter()
                .map(ResolvedNode::Declaration)
                .collect()
        }
        TypedNode::Expression(expression) => {
            vec![ResolvedNode::Expression(resolve_expression(expression))]
        }
        TypedNode::ReturnStatement(expression) => {
            vec![ResolvedNode::ReturnStatement(resolve_expression(
                expression,
            ))]
        } // TypedNode::StarImport(_) => todo!(),
    }
}
