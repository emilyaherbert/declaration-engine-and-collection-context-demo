use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use crate::language::partial::partial_declaration::PartialDeclaration;

use linked_hash_map::LinkedHashMap;

#[derive(Default)]
pub(crate) struct CollectionNamespace {
    symbols: LinkedHashMap<String, PartialDeclaration>,
}

impl fmt::Display for CollectionNamespace {
    fn fmt(&self, mut f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f).unwrap();
        {
            let mut indent = IndentWriter::new("  ", &mut f);
            for symbol in self.symbols.keys() {
                writeln!(indent, "{}", symbol).unwrap();
            }
        }
        Ok(())
    }
}

impl CollectionNamespace {
    pub(crate) fn scoped(&self) -> CollectionNamespace {
        CollectionNamespace {
            symbols: self.symbols.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        println!(
            "\n\n~~~~~~~~~~\n\nCollectionNamespace:\n{}\n\n~~~~~~~~~~",
            self
        );
    }

    pub(crate) fn insert_symbol(&mut self, name: String, symbol: PartialDeclaration) {
        self.symbols.insert(name, symbol);
    }

    pub(crate) fn get_symbol(&self, name: &str) -> Result<PartialDeclaration, String> {
        self.symbols
            .get(name)
            .cloned()
            .ok_or_else(|| "not found".to_string())
    }
}
