use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_index::CollectionIndex,
    },
    language::ty::typed_declaration::TyDeclaration,
    namespace::namespace::Namespace,
};

pub(super) fn collect_types_declaration(
    cc: &CollectionContext,
    _namespace: &mut Namespace,
    node_index: &CollectionIndex,
) {
    let declaration = cc.get_node(*node_index).expect_declaration().unwrap();
    match declaration {
        TyDeclaration::Variable(_) => {}
        TyDeclaration::Function(_) => {}
        TyDeclaration::Trait(_) => {}
        TyDeclaration::TraitImpl(_) => {}
        TyDeclaration::Struct(_) => {}
    }
}
