use crate::{
    declaration_engine::DeclarationEngine,
    language::{Node, Tree, TypedNode, TypedTree},
};

pub(crate) fn semantic_analysis(tree: Tree) -> TypedTree {
    let mut declaration_engine = DeclarationEngine::default();
    let new_nodes = tree
        .nodes
        .into_iter()
        .map(|node| semantic_analysis_node(&mut declaration_engine, node))
        .collect::<Vec<_>>();
    TypedTree { nodes: new_nodes }
}

fn semantic_analysis_node(declaration_engine: &mut DeclarationEngine, node: Node) -> TypedNode {
    unimplemented!()
}
