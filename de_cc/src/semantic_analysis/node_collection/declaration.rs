use crate::{
    collection_context::collection_context::CollectionContext,
    language::ty::typed_declaration::TyDeclaration,
};

pub(super) fn collect_nodes_declaration(
    _collection_ctxt: &mut CollectionContext,
    declaration: &TyDeclaration,
) {
    match declaration {
        TyDeclaration::Variable(_) => {}
        TyDeclaration::Function(_) => {}
        TyDeclaration::Trait(_) => {}
        TyDeclaration::TraitImpl(_) => {}
        TyDeclaration::Struct(_) => {}
    }
}
