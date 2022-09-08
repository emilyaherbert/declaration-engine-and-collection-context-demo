use crate::{language::ty::typed_declaration::TyDeclaration, namespace::namespace::Namespace};

pub(super) fn collect_types_declaration(_namespace: &mut Namespace, declaration: &TyDeclaration) {
    match declaration {
        TyDeclaration::Variable(_) => {}
        TyDeclaration::Function(_) => {}
        TyDeclaration::Trait(_) => {}
        TyDeclaration::TraitImpl(_) => {}
        TyDeclaration::Struct(_) => {}
    }
}
