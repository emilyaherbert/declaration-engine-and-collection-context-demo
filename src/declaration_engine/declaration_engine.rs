use std::collections::HashMap;

use crate::language::typed::typed_declaration::{
    TypedEnumDeclaration, TypedFunctionDeclaration, TypedStructDeclaration, TypedTraitDeclaration,
    TypedTraitImpl,
};

// TODO: will need to use concurrent structure like https://github.com/xacrimon/dashmaps
#[derive(Default)]
pub(crate) struct DeclarationEngine {
    functions: HashMap<String, TypedFunctionDeclaration>,
    structs: HashMap<String, TypedStructDeclaration>,
    enums: HashMap<String, TypedEnumDeclaration>,
    traits: HashMap<String, TypedTraitDeclaration>,
    trait_impl: HashMap<(String, String), TypedTraitImpl>,
}

impl DeclarationEngine {
    pub(crate) fn insert_function(&mut self, name: String, function: TypedFunctionDeclaration) {
        self.functions.insert(name, function);
    }

    pub(crate) fn insert_struct(&mut self, name: String, r#struct: TypedStructDeclaration) {
        self.structs.insert(name, r#struct);
    }

    pub(crate) fn insert_enum(&mut self, name: String, r#enum: TypedEnumDeclaration) {
        self.enums.insert(name, r#enum);
    }

    pub(crate) fn insert_trait(&mut self, name: String, r#trait: TypedTraitDeclaration) {
        self.traits.insert(name, r#trait);
    }

    pub(crate) fn get_function(&mut self, name: String) -> Option<&TypedFunctionDeclaration> {
        self.functions.get(&name)
    }

    pub(crate) fn get_struct(&mut self, name: String) -> Option<&TypedStructDeclaration> {
        self.structs.get(&name)
    }

    pub(crate) fn get_enum(&mut self, name: String) -> Option<&TypedEnumDeclaration> {
        self.enums.get(&name)
    }

    pub(crate) fn get_trait(&mut self, name: String) -> Option<&TypedTraitDeclaration> {
        self.traits.get(&name)
    }
}
