use linked_hash_map::LinkedHashMap;

use crate::{
    concurrent_slab::ConcurrentSlab,
    language::typed::typed_declaration::{
        TypedFunctionDeclaration, TypedTraitDeclaration, TypedTraitImpl,
    },
    types::pretty_print::PrettyPrint,
};

use super::{declaration_id::DeclarationId, declaration_wrapper::DeclarationWrapper};

// TODO: will need to use concurrent structure like https://github.com/xacrimon/dashmaps
pub struct DeclarationEngine {
    slab: ConcurrentSlab<DeclarationWrapper>,
    monomorphized_copies: LinkedHashMap<usize, Vec<DeclarationId>>,
}

impl DeclarationEngine {
    pub(crate) fn new() -> DeclarationEngine {
        DeclarationEngine {
            slab: ConcurrentSlab::default(),
            monomorphized_copies: LinkedHashMap::new(),
        }
    }

    pub(crate) fn look_up_decl_id(&self, index: DeclarationId) -> DeclarationWrapper {
        self.slab.get(index)
    }

    pub(crate) fn insert_function(&self, function: TypedFunctionDeclaration) -> DeclarationId {
        self.slab.insert(DeclarationWrapper::Function(function))
    }

    pub(crate) fn get_function(
        &self,
        index: DeclarationId,
    ) -> Result<TypedFunctionDeclaration, String> {
        self.slab.get(index).expect_function()
    }

    pub(crate) fn add_monomorphized_function_copy(
        &mut self,
        original_function_id: DeclarationId,
        new_copy: TypedFunctionDeclaration,
    ) {
        let new_id = self.slab.insert(DeclarationWrapper::Function(new_copy));
        match self.monomorphized_copies.get_mut(&*original_function_id) {
            Some(prev) => {
                prev.push(new_id);
            }
            None => {
                self.monomorphized_copies
                    .insert(*original_function_id, vec![new_id]);
            }
        }
    }

    pub(crate) fn get_monomorphized_function_copies(
        &self,
        original_function_id: DeclarationId,
    ) -> Result<Vec<TypedFunctionDeclaration>, String> {
        match self
            .monomorphized_copies
            .get(&*original_function_id)
            .cloned()
        {
            Some(copies) => Ok(copies
                .into_iter()
                .map(|copy| self.slab.get(&*copy).expect_function())
                .collect::<Result<_, _>>()?),
            None => Ok(vec![]),
        }
    }

    pub(crate) fn insert_trait(&self, r#trait: TypedTraitDeclaration) -> DeclarationId {
        self.slab.insert(DeclarationWrapper::Trait(r#trait))
    }

    pub(crate) fn get_trait(&self, index: DeclarationId) -> Result<TypedTraitDeclaration, String> {
        self.slab.get(index).expect_trait()
    }

    pub(crate) fn insert_trait_impl(&self, trait_impl: TypedTraitImpl) -> DeclarationId {
        self.slab.insert(DeclarationWrapper::TraitImpl(trait_impl))
    }

    pub(crate) fn get_trait_impl(&self, index: DeclarationId) -> Result<TypedTraitImpl, String> {
        self.slab.get(index).expect_trait_impl()
    }

    pub fn debug_print(&self) {
        println!(
            "\n\n~~~~~~~~~~\n\nDeclaration Engine:\n{}\n\n~~~~~~~~~~",
            self.slab.pretty_print(self)
        );
    }
}
