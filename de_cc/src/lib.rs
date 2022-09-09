use collection_context::collection_context::CollectionContext;
use language::{parsed::Application, resolved::ResolvedApplication};
use namespace::namespace::Namespace;
use semantic_analysis::{
    inference::analyze, node_collection::collect_nodes, ty_to_resolved::to_resolved,
    type_collection::collect_types,
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

    // 2. transform to the Ty AST
    let mut collection_ctxt = CollectionContext::default();
    let ty_application = collect_nodes(&mut collection_ctxt, application);

    // 3. do node collection
    //collect_nodes(&mut collection_ctxt, ty_application);

    // 4. do type collection
    let mut namespace = Namespace::default();
    collect_types(&mut namespace, todo!());

    // 5. do type inference with new namespace
    let mut namespace = Namespace::default();
    analyze(&mut namespace, todo!());

    // 6. resolve all types
    let resolved_application = to_resolved(todo!());

    // 7. ir generation happens here

    resolved_application
}
