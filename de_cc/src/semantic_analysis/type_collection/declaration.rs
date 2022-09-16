use crate::{
    collection_context::{collection_context::CollectionContext, collection_index::CCIdx},
    language::ty::typed_declaration::TyDeclaration,
};

pub(super) fn collect_types_declaration(_cc: &CollectionContext, decl: &mut CCIdx<TyDeclaration>) {
    match decl.inner_ref() {
        TyDeclaration::Variable(_) => {}
        TyDeclaration::Function(_) => {}
        TyDeclaration::Trait(_) => {}
        TyDeclaration::TraitImpl(_) => {}
        TyDeclaration::Struct(_) => {}
    }
}
