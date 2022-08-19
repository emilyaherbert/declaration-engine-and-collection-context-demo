use std::fmt;

use crate::{
    declaration_engine::declaration_id::DeclarationId,
    language::typed::typed_declaration::TypedDeclaration, type_system::type_id::TypeId,
};
use linked_hash_map::LinkedHashMap;

#[derive(Default)]
pub(crate) struct Namespace {
    symbols: LinkedHashMap<String, TypedDeclaration>,
    // (type info, trait name) -> declaration
    trait_impls: LinkedHashMap<(TypeId, String), Vec<DeclarationId>>,
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();
        for symbol in self.symbols.keys() {
            builder.push_str("\n  ");
            builder.push_str(symbol);
        }
        write!(f, "{}", builder)
    }
}

impl Namespace {
    pub(crate) fn scoped(&self) -> Namespace {
        Namespace {
            symbols: self.symbols.clone(),
            trait_impls: self.trait_impls.clone(),
        }
    }

    pub(crate) fn insert_symbol(&mut self, name: String, symbol: TypedDeclaration) {
        self.symbols.insert(name, symbol);
    }

    pub(crate) fn get_symbol(&self, name: &str) -> Result<TypedDeclaration, String> {
        self.symbols
            .get(name)
            .cloned()
            .ok_or_else(|| "not found".to_string())
    }

    pub(crate) fn insert_trait_impl(
        &mut self,
        type_info: TypeId,
        trait_name: String,
        mut methods: Vec<DeclarationId>,
    ) {
        match self.trait_impls.get_mut(&(type_info, trait_name.clone())) {
            Some(prev) => {
                prev.append(&mut methods);
            }
            None => {
                self.trait_impls.insert((type_info, trait_name), methods);
            }
        }
    }

    pub fn debug_print(&self) {
        println!("\n\n~~~~~~~~~~\n\nNamespace:\n{}\n\n~~~~~~~~~~", self);
    }
}
