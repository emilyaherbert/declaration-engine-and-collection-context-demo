use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use crate::{
    collection_context::collection_index::CCIdx,
    declaration_engine::{
        declaration_engine::*, declaration_id::DeclarationId,
        declaration_wrapper::DeclarationWrapper,
    },
    language::ty::typed_declaration::TyDeclaration,
    type_system::{type_engine::look_up_type_id, type_id::TypeId},
};
use linked_hash_map::LinkedHashMap;

use super::function_signature::TypedFunctionSignature;

type MethodList = Vec<CCIdx<DeclarationId>>;

#[derive(Default)]
pub(crate) struct Namespace {
    symbols: LinkedHashMap<String, TyDeclaration>,
    // this should be (type info, trait name) -> declaration id
    methods: Vec<(TypeId, MethodList)>,
}

impl fmt::Display for Namespace {
    fn fmt(&self, mut f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f).unwrap();
        {
            let mut indent = IndentWriter::new("  ", &mut f);
            for symbol in self.symbols.keys() {
                writeln!(indent, "{}", symbol).unwrap();
            }
        }
        writeln!(f).unwrap();
        {
            let mut indent = IndentWriter::new("  ", &mut f);
            for (k, _) in self.methods.iter() {
                writeln!(indent, "{}", k).unwrap();
            }
        }
        Ok(())
    }
}

impl Namespace {
    pub(crate) fn scoped(&self) -> Namespace {
        Namespace {
            symbols: self.symbols.clone(),
            methods: self.methods.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        println!("\n\n~~~~~~~~~~\n\nNamespace:\n{}\n\n~~~~~~~~~~", self);
    }

    pub(crate) fn insert_symbol(&mut self, name: String, symbol: TyDeclaration) {
        self.symbols.insert(name, symbol);
    }

    pub(crate) fn get_symbol(&self, name: &str) -> Result<TyDeclaration, String> {
        self.symbols
            .get(name)
            .cloned()
            .ok_or_else(|| "symbol not found".to_string())
    }

    pub(crate) fn insert_methods(
        &mut self,
        type_id: TypeId,
        _trait_name: String,
        mut methods: Vec<CCIdx<DeclarationId>>,
    ) {
        for (k, v) in self.methods.iter_mut() {
            // TODO: consider semantic similarity
            if look_up_type_id(*k) == look_up_type_id(type_id) {
                v.append(&mut methods);
                return;
            }
        }
        self.methods.push((type_id, methods));
    }

    pub(crate) fn get_method(
        &self,
        type_id: TypeId,
        func_name: &str,
    ) -> Result<TypedFunctionSignature, String> {
        for (k, method_ids) in self.methods.iter() {
            // TODO: consider semantic similarity
            if look_up_type_id(*k) == look_up_type_id(type_id) {
                for method_id in method_ids.iter() {
                    let (name, signature) = match de_look_up_decl_id(*method_id.inner_ref()) {
                        DeclarationWrapper::Function(decl) => (decl.name.clone(), decl.into()),
                        DeclarationWrapper::TraitFn(decl) => (decl.name.clone(), decl.into()),
                        _ => {
                            return Err("found bad item in self.methods".to_string());
                        }
                    };
                    if name == func_name {
                        return Ok(signature);
                    }
                }
            }
        }
        Err("could not find function".to_string())
    }
}
