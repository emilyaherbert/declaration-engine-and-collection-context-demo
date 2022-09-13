use std::{collections::VecDeque, ops::Index};

use petgraph::{
    visit::{VisitMap, Visitable},
    Direction,
};

use crate::{
    declaration_engine::{
        declaration_engine::{de_get_function, de_get_struct, de_get_trait},
        declaration_id::DeclarationId,
    },
    language::ty::{typed_declaration::TyDeclaration, TyNode},
};

use super::{
    collection_index::CollectionIndex, graph_edge::GraphEdge, graph_node::GraphNode,
    CollectionGraph,
};

pub(super) fn search_shared_scope_for_declaration(
    graph: &CollectionGraph,
    index: CollectionIndex,
    symbol: String,
) -> Result<Option<DeclarationId>, String> {
    let mut discovered = graph.visit_map();
    discovered.visit(*index);
    let mut stack = VecDeque::new();
    stack.push_front(*index);

    while let Some(node_index) = stack.pop_front() {
        let node = graph.index(node_index);

        if let GraphNode::Node(TyNode::Declaration(decl_index)) = node {
            let decl = graph.index(**decl_index).expect_declaration()?;
            match decl {
                TyDeclaration::Variable(_) => {}
                TyDeclaration::Function(decl_id) => {
                    let decl = de_get_function(*decl_id)?;
                    if decl.name == symbol {
                        return Ok(Some(*decl_id));
                    }
                }
                TyDeclaration::Trait(decl_id) => {
                    let decl = de_get_trait(*decl_id)?;
                    if decl.name == symbol {
                        return Ok(Some(*decl_id));
                    }
                }
                TyDeclaration::TraitImpl(_) => todo!(),
                TyDeclaration::Struct(decl_id) => {
                    let decl = de_get_struct(*decl_id)?;
                    if decl.name == symbol {
                        return Ok(Some(*decl_id));
                    }
                }
            }
        }

        for edge in graph.edges_directed(node_index, Direction::Outgoing) {
            let valid = match edge.weight() {
                GraphEdge::ApplicationContents => false,
                GraphEdge::FileContents => false,
                GraphEdge::SharedScope => true,
                GraphEdge::NodeContents => true,
                GraphEdge::DeclarationContents => true,
            };
            if valid {
                for next_node in graph
                    .neighbors_directed(node_index, Direction::Outgoing)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                {
                    if discovered.visit(next_node) {
                        stack.push_back(next_node);
                    }
                }
            }
        }
    }

    Ok(None)
}