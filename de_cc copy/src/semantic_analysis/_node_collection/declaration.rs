use petgraph::prelude::NodeIndex;

use crate::{
    collection_context::collection_context::CollectionContext,
    declaration_engine::declaration_engine::de_get_function,
    language::ty::typed_declaration::{TyDeclaration, TyFunctionDeclaration},
};

use super::collect_nodes_nodes;

pub(super) fn collect_nodes_declaration(
    collection_ctxt: &mut CollectionContext,
    parent_node: NodeIndex,
    declaration: TyDeclaration,
) {
    match declaration {
        TyDeclaration::Variable(_) => {}
        decl @ TyDeclaration::Function(decl_id) => {
            let function_node = collection_ctxt.add_node(decl.into());
            collection_ctxt.add_edge(parent_node, function_node);
            let function_declaration = de_get_function(decl_id).unwrap();
            collect_nodes_function(collection_ctxt, function_node, function_declaration);
        }
        TyDeclaration::Trait(_) => {}
        TyDeclaration::TraitImpl(_) => {}
        TyDeclaration::Struct(_) => {}
    }
}

fn collect_nodes_function(
    collection_ctxt: &mut CollectionContext,
    parent_node: NodeIndex,
    function_declaration: TyFunctionDeclaration,
) {
    collect_nodes_nodes(collection_ctxt, parent_node, function_declaration.body);
}
