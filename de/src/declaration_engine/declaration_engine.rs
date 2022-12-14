use linked_hash_map::LinkedHashMap;

use crate::{
    concurrent_slab::ConcurrentSlab,
    language::typed::typed_declaration::{
        TypedFunctionDeclaration, TypedStructDeclaration, TypedTraitDeclaration, TypedTraitFn,
        TypedTraitImpl,
    },
    types::pretty_print::PrettyPrint,
};

use super::{declaration_id::DeclarationId, declaration_wrapper::DeclarationWrapper};

// TODO: will need to use concurrent structure like https://github.com/xacrimon/dashmaps or im::HashMap

/// Used inside of type inference to store declarations.
pub(crate) struct DeclarationEngine {
    slab: ConcurrentSlab<DeclarationId, DeclarationWrapper>,
    // *declaration_id -> vec of monomorphized copies
    // where the declaration_id is the original declartion
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

    pub(crate) fn add_monomorphized_copy(
        &mut self,
        original_id: DeclarationId,
        new_id: DeclarationId,
    ) {
        match self.monomorphized_copies.get_mut(&*original_id) {
            Some(prev) => {
                prev.push(new_id);
            }
            None => {
                self.monomorphized_copies.insert(*original_id, vec![new_id]);
            }
        }
    }

    pub(crate) fn get_monomorphized_copies(
        &self,
        original_id: DeclarationId,
    ) -> Vec<DeclarationWrapper> {
        match self.monomorphized_copies.get(&*original_id).cloned() {
            Some(copies) => copies.into_iter().map(|copy| self.slab.get(copy)).collect(),
            None => vec![],
        }
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        println!(
            "\n\n~~~~~~~~~~\n\nDeclaration Engine:\n{}\n\n~~~~~~~~~~",
            self.slab.pretty_print(self)
        );
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

    // TODO(joao): consider only adding unique copies, if you get a non unique copy, throw it away
    pub(crate) fn add_monomorphized_function_copy(
        &mut self,
        original_id: DeclarationId,
        new_copy: TypedFunctionDeclaration,
    ) {
        let new_id = self.slab.insert(DeclarationWrapper::Function(new_copy));
        self.add_monomorphized_copy(original_id, new_id)
    }

    pub(crate) fn get_monomorphized_function_copies(
        &self,
        original_id: DeclarationId,
    ) -> Result<Vec<TypedFunctionDeclaration>, String> {
        self.get_monomorphized_copies(original_id)
            .into_iter()
            .map(|x| x.expect_function())
            .collect::<Result<_, _>>()
    }

    pub(crate) fn insert_trait(&self, r#trait: TypedTraitDeclaration) -> DeclarationId {
        self.slab.insert(DeclarationWrapper::Trait(r#trait))
    }

    pub(crate) fn get_trait(&self, index: DeclarationId) -> Result<TypedTraitDeclaration, String> {
        self.slab.get(index).expect_trait()
    }

    pub(crate) fn insert_trait_fn(&self, trait_fn: TypedTraitFn) -> DeclarationId {
        self.slab.insert(DeclarationWrapper::TraitFn(trait_fn))
    }

    pub(crate) fn get_trait_fn(&self, index: DeclarationId) -> Result<TypedTraitFn, String> {
        self.slab.get(index).expect_trait_fn()
    }

    pub(crate) fn insert_trait_impl(&self, trait_impl: TypedTraitImpl) -> DeclarationId {
        self.slab.insert(DeclarationWrapper::TraitImpl(trait_impl))
    }

    pub(crate) fn get_trait_impl(&self, index: DeclarationId) -> Result<TypedTraitImpl, String> {
        self.slab.get(index).expect_trait_impl()
    }

    pub(crate) fn insert_struct(&self, r#struct: TypedStructDeclaration) -> DeclarationId {
        self.slab.insert(DeclarationWrapper::Struct(r#struct))
    }

    pub(crate) fn get_struct(
        &self,
        index: DeclarationId,
    ) -> Result<TypedStructDeclaration, String> {
        self.slab.get(index).expect_struct()
    }

    // TODO(joao): consider only adding unique copies, if you get a non unique copy, throw it away
    pub(crate) fn add_monomorphized_struct_copy(
        &mut self,
        original_id: DeclarationId,
        new_copy: TypedStructDeclaration,
    ) {
        let new_id = self.slab.insert(DeclarationWrapper::Struct(new_copy));
        self.add_monomorphized_copy(original_id, new_id)
    }

    pub(crate) fn get_monomorphized_struct_copies(
        &self,
        original_id: DeclarationId,
    ) -> Result<Vec<TypedStructDeclaration>, String> {
        self.get_monomorphized_copies(original_id)
            .into_iter()
            .map(|x| x.expect_struct())
            .collect::<Result<_, _>>()
    }
}
