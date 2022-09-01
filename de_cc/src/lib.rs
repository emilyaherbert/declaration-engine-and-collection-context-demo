use language::{resolved::ResolvedApplication, untyped::Application};
use namespace::namespace::Namespace;
use semantic_analysis::{inference::analyze, resolution::resolve, type_collection::collect_types};

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

    // collect
    collect_types(&application);

    // do type inference
    let mut namespace = Namespace::default();
    let typed_application = analyze(&mut namespace, application);

    // resolve all types
    let resolved_application = resolve(typed_application);

    // ir generation happens

    resolved_application
}
