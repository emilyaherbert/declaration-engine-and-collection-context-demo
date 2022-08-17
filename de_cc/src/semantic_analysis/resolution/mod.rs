mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        resolved::{ResolvedApplication, ResolvedFile, ResolvedNode},
        typed::{TypedApplication, TypedFile, TypedNode},
    },
};

pub(crate) fn resolve(
    declaration_engine: &DeclarationEngine,
    application: TypedApplication,
) -> ResolvedApplication {
    let resolved_programs = application
        .files
        .into_iter()
        .map(|program| resolve_file(declaration_engine, program))
        .collect();
    ResolvedApplication {
        files: resolved_programs,
    }
}

fn resolve_file(declaration_engine: &DeclarationEngine, file: TypedFile) -> ResolvedFile {
    let new_nodes = resolve_nodes(declaration_engine, file.nodes);
    ResolvedFile {
        name: file.name,
        nodes: new_nodes,
    }
}

fn resolve_nodes(
    declaration_engine: &DeclarationEngine,
    nodes: Vec<TypedNode>,
) -> Vec<ResolvedNode> {
    nodes
        .into_iter()
        .map(|node| resolve_node(declaration_engine, node))
        .collect()
}

fn resolve_node(declaration_engine: &DeclarationEngine, node: TypedNode) -> ResolvedNode {
    match node {
        TypedNode::Declaration(declaration) => {
            ResolvedNode::Declaration(resolve_declaration(declaration_engine, declaration))
        }
        TypedNode::Expression(expression) => {
            ResolvedNode::Expression(resolve_expression(declaration_engine, expression))
        }
        TypedNode::ReturnStatement(expression) => {
            ResolvedNode::ReturnStatement(resolve_expression(declaration_engine, expression))
        }
        TypedNode::StarImport(_) => todo!(),
    }
}
