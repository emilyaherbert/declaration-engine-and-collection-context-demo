use std::fmt;

use linked_hash_map::LinkedHashMap;

use crate::language::typed::typed_declaration::TypedDeclaration;

#[derive(Debug, Default)]
pub(crate) struct Namespace {
    symbols: LinkedHashMap<String, TypedDeclaration>,
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
    pub(crate) fn scoped(&mut self) -> Namespace {
        Namespace {
            symbols: self.symbols.clone(),
        }
    }

    pub(crate) fn insert_symbol(&mut self, name: String, symbol: TypedDeclaration) {
        self.symbols.insert(name, symbol);
    }

    pub(crate) fn get_symbol(&mut self, name: &str) -> Result<TypedDeclaration, String> {
        self.symbols
            .get(name)
            .cloned()
            .ok_or_else(|| "not found".to_string())
    }

    pub fn debug_print(&self) {
        println!("\n\n~~~~~~~~~~\n\nNamespace:\n{}\n\n~~~~~~~~~~", self);
    }
}
