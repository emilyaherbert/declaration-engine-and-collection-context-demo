#![allow(dead_code)]

use collection_context::collection_context::CollectionContext;
use declaration_engine::declaration_engine::DeclarationEngine;
use language::{resolved::ResolvedTree, untyped::Tree};
use semantic_analysis::{collection::collect, inference::analyze, resolution::resolve};
use type_system::type_engine::TypeEngine;

mod collection_context;
mod declaration_engine;
pub mod language;
mod semantic_analysis;
mod type_system;

#[allow(clippy::let_and_return)]
pub fn compile(program: Tree) -> ResolvedTree {
    // parsing happens here

    // fill the collection context
    let mut collection_context = CollectionContext::default();
    collect(&mut collection_context, &program);

    // do type inference
    let mut type_engine = TypeEngine::default();
    let mut declaration_engine = DeclarationEngine::default();
    let typed_program = analyze(&mut type_engine, &mut declaration_engine, program);

    // resolve all types
    let resolved_program = resolve(typed_program);

    // ir generation happens

    resolved_program
}
