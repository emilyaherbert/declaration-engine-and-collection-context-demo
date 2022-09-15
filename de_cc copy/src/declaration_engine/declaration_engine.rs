use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::{
    concurrent_slab::ConcurrentSlab,
    language::ty::typed_declaration::{
        TyFunctionDeclaration, TyStructDeclaration, TyTraitDeclaration, TyTraitFn, TyTraitImpl,
    },
};

use super::{declaration_id::DeclarationId, declaration_wrapper::DeclarationWrapper};

lazy_static! {
    static ref DECLARATION_ENGINE: DeclarationEngine = DeclarationEngine::default();
}

/// Used inside of type inference to store declarations.
#[derive(Default)]
struct DeclarationEngine {
    slab: ConcurrentSlab<DeclarationWrapper>,
    // *declaration_id -> vec of monomorphized copies
    // where the declaration_id is the original declaration
    monomorphized_copies: RwLock<HashMap<usize, Vec<DeclarationId>>>,
}

impl DeclarationEngine {
    fn clear(&self) {
        self.slab.clear();
        let mut monomorphized_copies = self.monomorphized_copies.write().unwrap();
        monomorphized_copies.clear();
    }

    fn insert(&self, value: DeclarationWrapper) -> DeclarationId {
        DeclarationId::new(self.slab.insert(value))
    }

    fn look_up_decl_id(&self, index: DeclarationId) -> DeclarationWrapper {
        self.slab.get(*index)
    }

    fn add_monomorphized_copy(&self, original_id: DeclarationId, new_id: DeclarationId) {
        let mut monomorphized_copies = self.monomorphized_copies.write().unwrap();
        match monomorphized_copies.get_mut(&*original_id) {
            Some(prev) => {
                prev.push(new_id);
            }
            None => {
                monomorphized_copies.insert(*original_id, vec![new_id]);
            }
        }
    }

    fn get_monomorphized_copies(&self, original_id: DeclarationId) -> Vec<DeclarationWrapper> {
        let monomorphized_copies = self.monomorphized_copies.write().unwrap();
        match monomorphized_copies.get(&*original_id).cloned() {
            Some(copies) => copies
                .into_iter()
                .map(|copy| self.slab.get(*copy))
                .collect(),
            None => vec![],
        }
    }

    fn insert_function(&self, function: TyFunctionDeclaration) -> DeclarationId {
        self.insert(DeclarationWrapper::Function(function))
    }

    fn get_function(&self, index: DeclarationId) -> Result<TyFunctionDeclaration, String> {
        self.slab.get(*index).expect_function()
    }

    fn add_monomorphized_function_copy(
        &self,
        original_id: DeclarationId,
        new_copy: TyFunctionDeclaration,
    ) {
        let new_id = self.insert(DeclarationWrapper::Function(new_copy));
        self.add_monomorphized_copy(original_id, new_id)
    }

    fn get_monomorphized_function_copies(
        &self,
        original_id: DeclarationId,
    ) -> Result<Vec<TyFunctionDeclaration>, String> {
        self.get_monomorphized_copies(original_id)
            .into_iter()
            .map(|x| x.expect_function())
            .collect::<Result<_, _>>()
    }

    fn insert_trait(&self, r#trait: TyTraitDeclaration) -> DeclarationId {
        self.insert(DeclarationWrapper::Trait(r#trait))
    }

    fn get_trait(&self, index: DeclarationId) -> Result<TyTraitDeclaration, String> {
        self.slab.get(*index).expect_trait()
    }

    fn insert_trait_fn(&self, trait_fn: TyTraitFn) -> DeclarationId {
        self.insert(DeclarationWrapper::TraitFn(trait_fn))
    }

    fn get_trait_fn(&self, index: DeclarationId) -> Result<TyTraitFn, String> {
        self.slab.get(*index).expect_trait_fn()
    }

    fn insert_trait_impl(&self, trait_impl: TyTraitImpl) -> DeclarationId {
        self.insert(DeclarationWrapper::TraitImpl(trait_impl))
    }

    fn get_trait_impl(&self, index: DeclarationId) -> Result<TyTraitImpl, String> {
        self.slab.get(*index).expect_trait_impl()
    }

    fn insert_struct(&self, r#struct: TyStructDeclaration) -> DeclarationId {
        self.insert(DeclarationWrapper::Struct(r#struct))
    }

    fn get_struct(&self, index: DeclarationId) -> Result<TyStructDeclaration, String> {
        self.slab.get(*index).expect_struct()
    }

    fn add_monomorphized_struct_copy(
        &self,
        original_id: DeclarationId,
        new_copy: TyStructDeclaration,
    ) {
        let new_id = self.insert(DeclarationWrapper::Struct(new_copy));
        self.add_monomorphized_copy(original_id, new_id)
    }

    fn get_monomorphized_struct_copies(
        &self,
        original_id: DeclarationId,
    ) -> Result<Vec<TyStructDeclaration>, String> {
        self.get_monomorphized_copies(original_id)
            .into_iter()
            .map(|x| x.expect_struct())
            .collect::<Result<_, _>>()
    }
}

pub(crate) fn de_clear() {
    DECLARATION_ENGINE.clear()
}

pub(crate) fn de_look_up_decl_id(index: DeclarationId) -> DeclarationWrapper {
    DECLARATION_ENGINE.look_up_decl_id(index)
}

pub(crate) fn de_insert_function(function: TyFunctionDeclaration) -> DeclarationId {
    DECLARATION_ENGINE.insert_function(function)
}

pub(crate) fn de_get_function(index: DeclarationId) -> Result<TyFunctionDeclaration, String> {
    DECLARATION_ENGINE.get_function(index)
}

pub(crate) fn de_add_monomorphized_function_copy(
    original_id: DeclarationId,
    new_copy: TyFunctionDeclaration,
) {
    DECLARATION_ENGINE.add_monomorphized_function_copy(original_id, new_copy);
}

pub(crate) fn de_get_monomorphized_function_copies(
    original_id: DeclarationId,
) -> Result<Vec<TyFunctionDeclaration>, String> {
    DECLARATION_ENGINE.get_monomorphized_function_copies(original_id)
}

pub(crate) fn de_insert_trait(r#trait: TyTraitDeclaration) -> DeclarationId {
    DECLARATION_ENGINE.insert_trait(r#trait)
}

pub(crate) fn de_get_trait(index: DeclarationId) -> Result<TyTraitDeclaration, String> {
    DECLARATION_ENGINE.get_trait(index)
}

pub(crate) fn de_insert_trait_fn(trait_fn: TyTraitFn) -> DeclarationId {
    DECLARATION_ENGINE.insert_trait_fn(trait_fn)
}

pub(crate) fn de_get_trait_fn(index: DeclarationId) -> Result<TyTraitFn, String> {
    DECLARATION_ENGINE.get_trait_fn(index)
}

pub(crate) fn de_insert_trait_impl(trait_impl: TyTraitImpl) -> DeclarationId {
    DECLARATION_ENGINE.insert_trait_impl(trait_impl)
}

pub(crate) fn de_get_trait_impl(index: DeclarationId) -> Result<TyTraitImpl, String> {
    DECLARATION_ENGINE.get_trait_impl(index)
}

pub(crate) fn de_insert_struct(r#struct: TyStructDeclaration) -> DeclarationId {
    DECLARATION_ENGINE.insert_struct(r#struct)
}

pub(crate) fn de_get_struct(index: DeclarationId) -> Result<TyStructDeclaration, String> {
    DECLARATION_ENGINE.get_struct(index)
}

pub(crate) fn de_add_monomorphized_struct_copy(
    original_id: DeclarationId,
    new_copy: TyStructDeclaration,
) {
    DECLARATION_ENGINE.add_monomorphized_struct_copy(original_id, new_copy);
}

pub(crate) fn de_get_monomorphized_struct_copies(
    original_id: DeclarationId,
) -> Result<Vec<TyStructDeclaration>, String> {
    DECLARATION_ENGINE.get_monomorphized_struct_copies(original_id)
}
