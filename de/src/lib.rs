use crate::declaration_engine::declaration_engine::DeclarationEngine;
use language::{resolved::ResolvedApplication, untyped::Application};
use namespace::namespace::Namespace;
use semantic_analysis::{inference::analyze, resolution::resolve};

mod concurrent_slab;
mod declaration_engine;
pub mod language;
mod namespace;
mod semantic_analysis;
pub mod type_system;
mod types;

#[allow(clippy::let_and_return)]
pub fn compile(application: Application) -> ResolvedApplication {
    // parsing happens here

    // do type inference
    let mut namespace = Namespace::default();
    let mut declaration_engine = DeclarationEngine::default();
    let typed_application = analyze(&mut namespace, &mut declaration_engine, application);

    // resolve all types
    let resolved_application = resolve(&declaration_engine, typed_application);

    // namespace.debug_print();
    // declaration_engine.debug_print();

    // ir generation happens

    resolved_application
}
