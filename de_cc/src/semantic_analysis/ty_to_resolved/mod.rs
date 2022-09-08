mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::language::{
    resolved::{ResolvedApplication, ResolvedFile, ResolvedNode},
    ty::{TyApplication, TyFile, TyNode},
};

pub(crate) fn to_resolved(application: TyApplication) -> ResolvedApplication {
    let to_resolvedd_programs = application
        .files
        .into_iter()
        .map(to_resolved_file)
        .collect();
    ResolvedApplication {
        files: to_resolvedd_programs,
    }
}

fn to_resolved_file(file: TyFile) -> ResolvedFile {
    let new_nodes = to_resolved_nodes(file.nodes);
    ResolvedFile {
        name: file.name,
        nodes: new_nodes,
    }
}

fn to_resolved_nodes(nodes: Vec<TyNode>) -> Vec<ResolvedNode> {
    nodes.into_iter().flat_map(to_resolved_node).collect()
}

fn to_resolved_node(node: TyNode) -> Vec<ResolvedNode> {
    match node {
        TyNode::Declaration(declaration) => {
            let declarations = to_resolved_declaration(declaration);
            declarations
                .into_iter()
                .map(ResolvedNode::Declaration)
                .collect()
        }
        TyNode::Expression(expression) => {
            vec![ResolvedNode::Expression(to_resolved_expression(expression))]
        }
        TyNode::ReturnStatement(expression) => {
            vec![ResolvedNode::ReturnStatement(to_resolved_expression(
                expression,
            ))]
        } // TypedNode::StarImport(_) => todo!(),
    }
}
