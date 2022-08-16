use std::collections::HashMap;

use crate::language::typed::typed_declaration::TypedDeclaration;

#[derive(Debug, Default)]
pub(crate) struct Namespace {
    symbols: HashMap<String, TypedDeclaration>,
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
}
