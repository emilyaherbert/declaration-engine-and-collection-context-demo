use language::{parsed::Application, resolved::ResolvedApplication};
use namespace::namespace::Namespace;
use semantic_analysis::{
    inference::analyze, parsed_to_ty::to_ty, ty_to_resolved::to_resolved,
    type_collection::collect_types,
};

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
    let ty_application = to_ty(application);

    // 3. do type collection
    let mut namespace = Namespace::default();
    collect_types(&mut namespace, &ty_application);

    // 4. do type inference with new namespace
    let mut namespace = Namespace::default();
    analyze(&mut namespace, &ty_application);

    // 5. resolve all types
    let resolved_application = to_resolved(ty_application);

    // 6. ir generation happens here

    resolved_application
}
