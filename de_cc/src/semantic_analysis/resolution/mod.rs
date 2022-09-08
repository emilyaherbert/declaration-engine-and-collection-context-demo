mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::language::{
    resolved::{ResolvedApplication, ResolvedFile, ResolvedNode},
    typed::{TyApplication, TyFile, TyNode},
};

pub(crate) fn resolve(application: TyApplication) -> ResolvedApplication {
    let resolved_programs = application.files.into_iter().map(resolve_file).collect();
    ResolvedApplication {
        files: resolved_programs,
    }
}

fn resolve_file(file: TyFile) -> ResolvedFile {
    let new_nodes = resolve_nodes(file.nodes);
    ResolvedFile {
        name: file.name,
        nodes: new_nodes,
    }
}

fn resolve_nodes(nodes: Vec<TyNode>) -> Vec<ResolvedNode> {
    nodes.into_iter().flat_map(resolve_node).collect()
}

fn resolve_node(node: TyNode) -> Vec<ResolvedNode> {
    match node {
        TyNode::Declaration(declaration) => {
            let declarations = resolve_declaration(declaration);
            declarations
                .into_iter()
                .map(ResolvedNode::Declaration)
                .collect()
        }
        TyNode::Expression(expression) => {
            vec![ResolvedNode::Expression(resolve_expression(expression))]
        }
        TyNode::ReturnStatement(expression) => {
            vec![ResolvedNode::ReturnStatement(resolve_expression(
                expression,
            ))]
        } // TypedNode::StarImport(_) => todo!(),
    }
}
