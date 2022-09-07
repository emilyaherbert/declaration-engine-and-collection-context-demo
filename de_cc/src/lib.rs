use language::{resolved::ResolvedApplication, untyped::Application};
use namespace::namespace::Namespace;
use semantic_analysis::{inference::analyze, resolution::resolve, type_collection::type_collect};

mod concurrent_slab;
mod declaration_engine;
pub mod language;
mod namespace;
mod semantic_analysis;
pub mod type_system;
mod types;

use declaration_engine::declaration_engine as de;

#[allow(clippy::let_and_return)]
pub fn compile(application: Application) -> ResolvedApplication {
    de::de_clear();

    // parsing happens here

    // do type collection
    let mut namespace = Namespace::default();
    let typed_application = type_collect(&mut namespace, application);

    // do type inference with new namespace
    let mut namespace = Namespace::default();
    let typed_application = analyze(&mut namespace, typed_application);

    // resolve all types
    let resolved_application = resolve(typed_application);

    // ir generation happens

    resolved_application
}
