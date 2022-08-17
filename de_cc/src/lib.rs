#![allow(dead_code)]

use crate::declaration_engine::declaration_engine::DeclarationEngine;
use collection_context::collection_context::CollectionContext;
use language::{resolved::ResolvedApplication, untyped::Application};
use namespace::namespace::Namespace;
use semantic_analysis::{collection::collect, inference::analyze, resolution::resolve};

mod collection_context;
mod declaration_engine;
pub mod language;
mod namespace;
mod semantic_analysis;
pub mod type_system;

#[allow(clippy::let_and_return)]
pub fn compile(application: Application) -> ResolvedApplication {
    // parsing happens here

    // fill the collection context
    let mut collection_context = CollectionContext::default();
    collect(&mut collection_context, &application);

    collection_context.debug_print();

    // do type inference
    let mut namespace = Namespace::default();
    let mut declaration_engine = DeclarationEngine::default();
    let typed_application = analyze(
        &mut namespace,
        &collection_context,
        &mut declaration_engine,
        application,
    );

    // resolve all types
    let resolved_application = resolve(&declaration_engine, typed_application);

    // namespace.debug_print();
    // declaration_engine.debug_print();

    // ir generation happens

    resolved_application
}
