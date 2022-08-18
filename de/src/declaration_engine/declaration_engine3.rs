use std::fmt;

use linked_hash_map::LinkedHashMap;

use crate::language::typed::typed_declaration::TypedFunctionDeclaration;

// TODO: will need to use concurrent structure like https://github.com/xacrimon/dashmaps
#[derive(Default)]
pub(crate) struct DeclarationEngine {
    functions: LinkedHashMap<String, TypedFunctionDeclaration>,
    // structs: LinkedHashMap<String, TypedStructDeclaration>,
    // enums: LinkedHashMap<String, TypedEnumDeclaration>,
    // traits: LinkedHashMap<String, TypedTraitDeclaration>,
    // trait_impls: LinkedHashMap<(String, String), TypedTraitImpl>,
}

impl fmt::Display for DeclarationEngine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();

        builder.push_str("\n  functions:\n");
        for name in self.functions.keys() {
            builder.push_str("\n    ");
            builder.push_str(name);
        }

        // builder.push_str("\n\n  structs:\n");
        // for r#struct in self.structs.keys() {
        //     builder.push_str("\n    ");
        //     builder.push_str(r#struct);
        // }

        // builder.push_str("\n\n  enums:\n");
        // for r#enum in self.enums.keys() {
        //     builder.push_str("\n    ");
        //     builder.push_str(r#enum);
        // }

        // builder.push_str("\n\n  traits:\n");
        // for r#trait in self.traits.keys() {
        //     builder.push_str("\n    ");
        //     builder.push_str(r#trait);
        // }

        // builder.push_str("\n\n  trait_impls:\n");
        // for (trait_name, type_implementing_for) in self.trait_impls.keys() {
        //     builder.push_str("\n    ");
        //     builder.push_str(trait_name);
        //     builder.push_str(" for ");
        //     builder.push_str(type_implementing_for);
        // }

        write!(f, "{}", builder)
    }
}

impl DeclarationEngine {
    pub(crate) fn insert_function(&mut self, name: String, function: TypedFunctionDeclaration) {
        self.functions.insert(name, function);
    }

    // pub(crate) fn insert_struct(&mut self, name: String, r#struct: TypedStructDeclaration) {
    //     self.structs.insert(name, r#struct);
    // }

    // pub(crate) fn insert_enum(&mut self, name: String, r#enum: TypedEnumDeclaration) {
    //     self.enums.insert(name, r#enum);
    // }

    // pub(crate) fn insert_trait(&mut self, name: String, r#trait: TypedTraitDeclaration) {
    //     self.traits.insert(name, r#trait);
    // }

    pub(crate) fn get_function(&self, name: String) -> Option<&TypedFunctionDeclaration> {
        self.functions.get(&name)
    }

    // pub(crate) fn get_struct(&self, name: String) -> Option<&TypedStructDeclaration> {
    //     self.structs.get(&name)
    // }

    // pub(crate) fn get_enum(&self, name: String) -> Option<&TypedEnumDeclaration> {
    //     self.enums.get(&name)
    // }

    // pub(crate) fn get_trait(&self, name: String) -> Option<&TypedTraitDeclaration> {
    //     self.traits.get(&name)
    // }

    pub fn debug_print(&self) {
        println!(
            "\n\n~~~~~~~~~~\n\nDeclaration Engine:\n{}\n\n~~~~~~~~~~",
            self
        );
    }
}
