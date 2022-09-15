use std::{collections::VecDeque, ops::Index};

use petgraph::{
    visit::{VisitMap, Visitable},
    Direction,
};

use crate::{
    declaration_engine::{
        declaration_engine::{de_get_function, de_get_struct, de_get_trait, de_get_trait_impl},
        declaration_id::DeclarationId,
    },
    language::ty::{typed_declaration::TyDeclaration, TyNode},
};

use super::{
    collection_edge::CollectionEdge,
    collection_index::{CCIdx, CollectionIndex},
    collection_node::CollectionNode,
    CollectionGraph,
};

// https://docs.rs/petgraph/latest/src/petgraph/visit/traversal.rs.html#253
pub(super) fn get_all_declarations_in_scope(
    graph: &CollectionGraph,
    index: CollectionIndex,
) -> Result<Vec<(String, CCIdx<DeclarationId>)>, String> {
    let mut discovered = graph.visit_map();
    discovered.visit(*index);

    let mut stack = VecDeque::new();
    stack.push_front(index);

    let mut declarations = vec![];

    while let Some(node_index) = stack.pop_front() {
        let node = graph.index(*node_index);

        match node {
            CollectionNode::Node(TyNode::Declaration(decl)) => match decl.inner_ref() {
                TyDeclaration::Variable(_) => {}
                TyDeclaration::Function(decl_id) => {
                    let decl = de_get_function(*decl_id.inner_ref())?;
                    declarations.push((decl.name, decl_id.clone()));
                }
                TyDeclaration::Trait(decl_id) => {
                    let decl = de_get_trait(*decl_id.inner_ref())?;
                    declarations.push((decl.name, decl_id.clone()));
                }
                TyDeclaration::TraitImpl(decl_id) => {
                    let decl = de_get_trait_impl(*decl_id.inner_ref())?;
                    declarations.push((decl.trait_name, decl_id.clone()));
                }
                TyDeclaration::Struct(decl_id) => {
                    let decl = de_get_struct(*decl_id.inner_ref())?;
                    declarations.push((decl.name, decl_id.clone()));
                }
            },
            CollectionNode::Declaration(name, decl) => match decl {
                TyDeclaration::Variable(_) => {}
                TyDeclaration::Function(decl_id) => {
                    declarations.push((name.to_string(), decl_id.clone()));
                }
                TyDeclaration::Trait(decl_id) => {
                    declarations.push((name.to_string(), decl_id.clone()));
                }
                TyDeclaration::TraitImpl(decl_id) => {
                    declarations.push((name.to_string(), decl_id.clone()));
                }
                TyDeclaration::Struct(decl_id) => {
                    declarations.push((name.to_string(), decl_id.clone()));
                }
            },
            CollectionNode::Function(name, decl_id) => {
                declarations.push((name.to_string(), CCIdx::new(*decl_id, node_index)));
            }
            CollectionNode::Trait(name, decl_id) => {
                declarations.push((name.to_string(), CCIdx::new(*decl_id, node_index)));
            }
            CollectionNode::TraitFn(name, decl_id) => {
                declarations.push((name.to_string(), CCIdx::new(*decl_id, node_index)));
            }
            CollectionNode::TraitImpl(name, decl_id) => {
                declarations.push((name.to_string(), CCIdx::new(*decl_id, node_index)));
            }
            CollectionNode::Struct(name, decl_id) => {
                declarations.push((name.to_string(), CCIdx::new(*decl_id, node_index)));
            }
            _ => {}
        }

        for edge in graph.edges_directed(*node_index, Direction::Outgoing) {
            let valid = match edge.weight() {
                CollectionEdge::ApplicationContents => false,
                CollectionEdge::FileContents => false,
                CollectionEdge::SharedScope => true,
                CollectionEdge::NodeContents => true,
                CollectionEdge::DeclarationContents => true,
                CollectionEdge::ScopedChild => true,
            };
            if valid {
                for next_node in graph
                    .neighbors_directed(*node_index, Direction::Outgoing)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                {
                    if discovered.visit(next_node) {
                        stack.push_back(CollectionIndex::new(next_node));
                    }
                }
            }
        }
    }

    Ok(declarations)
}
