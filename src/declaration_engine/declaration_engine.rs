use std::collections::HashMap;

use crate::language::{
    TypedEnumDeclaration, TypedFunctionDeclaration, TypedStructDeclaration, TypedTraitDeclaration,
    TypedVariableDeclaration,
};

// TODO: will need to use concurrent structure like https://github.com/xacrimon/dashmaps
#[derive(Default)]
pub(crate) struct DeclarationEngine {
    variables: HashMap<String, TypedVariableDeclaration>,
    functions: HashMap<String, TypedFunctionDeclaration>,
    structs: HashMap<String, TypedStructDeclaration>,
    enums: HashMap<String, TypedEnumDeclaration>,
    traits: HashMap<String, TypedTraitDeclaration>,
}

impl DeclarationEngine {
    pub(crate) fn insert_variable(&mut self, name: String, variable: TypedVariableDeclaration) {
        self.variables.insert(name, variable);
    }

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

    pub(crate) fn get_variable(&mut self, name: String) -> Option<&TypedVariableDeclaration> {
        self.variables.get(&name)
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
