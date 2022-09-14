use collection_context::collection_context::CollectionContext;
use language::{parsed::Application, resolved::ResolvedApplication};
use namespace::namespace::Namespace;
use semantic_analysis::{
    graph_collection::collect_graph, ty_to_resolved::to_resolved, type_collection::collect_types,
    type_inference::analyze,
};

mod collection_context;
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

    // 1. parsing happens here

    // 2. transform to the Ty AST and do graph collection
    let mut collection_context = CollectionContext::default();
    let application_idx = collect_graph(&mut collection_context, application);

    // 3. do type collection
    let mut namespace = Namespace::default();
    collect_types(&collection_context, &mut namespace, application_idx);

    // 4. do type inference with new namespace
    let mut namespace = Namespace::default();
    analyze(&collection_context, &mut namespace, application_idx);

    // 5. resolve all types
    let resolved_application = to_resolved(&collection_context, application_idx);

    // 6. ir generation happens here

    resolved_application
}
