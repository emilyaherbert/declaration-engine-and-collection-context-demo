use crate::language::{
    TypedEnumDeclaration, TypedFunctionDeclaration, TypedStructDeclaration, TypedTraitDeclaration,
};

use super::{ConcurrentSlab, DeclarationId};

#[derive(Default)]
pub(crate) struct DeclarationEngine {
    functions: ConcurrentSlab<TypedFunctionDeclaration>,
    structs: ConcurrentSlab<TypedStructDeclaration>,
    enums: ConcurrentSlab<TypedEnumDeclaration>,
    traits: ConcurrentSlab<TypedTraitDeclaration>,
}

impl DeclarationEngine {
    pub(crate) fn insert_function(&mut self, function: TypedFunctionDeclaration) -> DeclarationId {
        self.functions.insert(function)
    }

    pub(crate) fn insert_struct(&mut self, r#struct: TypedStructDeclaration) -> DeclarationId {
        self.structs.insert(r#struct)
    }

    pub(crate) fn insert_enum(&mut self, r#enum: TypedEnumDeclaration) -> DeclarationId {
        self.enums.insert(r#enum)
    }

    pub(crate) fn insert_trait(&mut self, r#trait: TypedTraitDeclaration) -> DeclarationId {
        self.traits.insert(r#trait)
    }
}
