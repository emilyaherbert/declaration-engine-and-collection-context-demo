use std::collections::HashMap;
use std::sync::RwLock;

use either::Either;
use lazy_static::lazy_static;

use crate::{
    concurrent_slab::ConcurrentSlab,
    language::{
        semi::semi_declaration::SemiTypedFunctionDeclaration,
        typed::typed_declaration::{
            TypedFunctionDeclaration, TypedStructDeclaration, TypedTraitDeclaration, TypedTraitFn,
            TypedTraitImpl,
        },
    }, namespace::function_signature::TypedFunctionSignature,
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

    fn insert_function(&self, function: TypedFunctionDeclaration) -> DeclarationId {
        DeclarationId::new(
            self.slab
                .insert(DeclarationWrapper::Function(Either::Right(function))),
        )
    }

    fn get_function_typed(&self, index: DeclarationId) -> Result<TypedFunctionDeclaration, String> {
        self.slab.get(*index).expect_function_typed()
    }

    fn get_function_semi_typed(
        &self,
        index: DeclarationId,
    ) -> Result<SemiTypedFunctionDeclaration, String> {
        self.slab.get(*index).expect_function_semi_typed()
    }

    fn get_function_signature(
        &self,
        index: DeclarationId,
    ) -> Result<TypedFunctionSignature, String> {
        self.slab.get(*index).expect_function_signature()
    }

    fn add_monomorphized_function_copy(
        &self,
        original_id: DeclarationId,
        new_copy: TypedFunctionDeclaration,
    ) {
        let new_id = DeclarationId::new(
            self.slab
                .insert(DeclarationWrapper::Function(Either::Right(new_copy))),
        );
        self.add_monomorphized_copy(original_id, new_id)
    }

    fn get_monomorphized_function_copies(
        &self,
        original_id: DeclarationId,
    ) -> Result<Vec<TypedFunctionDeclaration>, String> {
        self.get_monomorphized_copies(original_id)
            .into_iter()
            .map(|x| x.expect_function_typed())
            .collect::<Result<_, _>>()
    }

    fn insert_trait(&self, r#trait: TypedTraitDeclaration) -> DeclarationId {
        DeclarationId::new(self.slab.insert(DeclarationWrapper::Trait(r#trait)))
    }

    fn get_trait(&self, index: DeclarationId) -> Result<TypedTraitDeclaration, String> {
        self.slab.get(*index).expect_trait()
    }

    fn insert_trait_fn(&self, trait_fn: TypedTraitFn) -> DeclarationId {
        DeclarationId::new(self.slab.insert(DeclarationWrapper::TypedTraitFn(trait_fn)))
    }

    fn get_trait_fn(&self, index: DeclarationId) -> Result<TypedTraitFn, String> {
        self.slab.get(*index).expect_trait_fn()
    }

    fn insert_trait_impl(&self, trait_impl: TypedTraitImpl) -> DeclarationId {
        DeclarationId::new(self.slab.insert(DeclarationWrapper::TraitFn(trait_impl)))
    }

    fn get_trait_impl(&self, index: DeclarationId) -> Result<TypedTraitImpl, String> {
        self.slab.get(*index).expect_trait_impl()
    }

    fn insert_struct(&self, r#struct: TypedStructDeclaration) -> DeclarationId {
        DeclarationId::new(self.slab.insert(DeclarationWrapper::Struct(r#struct)))
    }

    fn get_struct(&self, index: DeclarationId) -> Result<TypedStructDeclaration, String> {
        self.slab.get(*index).expect_struct()
    }

    fn add_monomorphized_struct_copy(
        &self,
        original_id: DeclarationId,
        new_copy: TypedStructDeclaration,
    ) {
        let new_id = DeclarationId::new(self.slab.insert(DeclarationWrapper::Struct(new_copy)));
        self.add_monomorphized_copy(original_id, new_id)
    }

    fn get_monomorphized_struct_copies(
        &self,
        original_id: DeclarationId,
    ) -> Result<Vec<TypedStructDeclaration>, String> {
        self.get_monomorphized_copies(original_id)
            .into_iter()
            .map(|x| x.expect_struct())
            .collect::<Result<_, _>>()
    }
}

pub(crate) fn de_clear() {
    DECLARATION_ENGINE.clear()
}

pub(crate) fn de_insert(value: DeclarationWrapper) -> DeclarationId {
    DECLARATION_ENGINE.insert(value)
}

pub(crate) fn de_look_up_decl_id(index: DeclarationId) -> DeclarationWrapper {
    DECLARATION_ENGINE.look_up_decl_id(index)
}

pub(crate) fn de_insert_function(function: TypedFunctionDeclaration) -> DeclarationId {
    DECLARATION_ENGINE.insert_function(function)
}

pub(crate) fn de_get_function_typed(
    index: DeclarationId,
) -> Result<TypedFunctionDeclaration, String> {
    DECLARATION_ENGINE.get_function_typed(index)
}

pub(crate) fn de_get_function_semi_typed(
    index: DeclarationId,
) -> Result<SemiTypedFunctionDeclaration, String> {
    DECLARATION_ENGINE.get_function_semi_typed(index)
}

pub(crate) fn de_get_function_signature(
    index: DeclarationId,
) -> Result<TypedFunctionSignature, String> {
    DECLARATION_ENGINE.get_function_signature(index)
}

pub(crate) fn de_add_monomorphized_function_copy(
    original_id: DeclarationId,
    new_copy: TypedFunctionDeclaration,
) {
    DECLARATION_ENGINE.add_monomorphized_function_copy(original_id, new_copy);
}

pub(crate) fn de_get_monomorphized_function_copies(
    original_id: DeclarationId,
) -> Result<Vec<TypedFunctionDeclaration>, String> {
    DECLARATION_ENGINE.get_monomorphized_function_copies(original_id)
}

pub(crate) fn de_insert_trait(r#trait: TypedTraitDeclaration) -> DeclarationId {
    DECLARATION_ENGINE.insert_trait(r#trait)
}

pub(crate) fn de_get_trait(index: DeclarationId) -> Result<TypedTraitDeclaration, String> {
    DECLARATION_ENGINE.get_trait(index)
}

pub(crate) fn de_insert_trait_fn(trait_fn: TypedTraitFn) -> DeclarationId {
    DECLARATION_ENGINE.insert_trait_fn(trait_fn)
}

pub(crate) fn de_get_trait_fn(index: DeclarationId) -> Result<TypedTraitFn, String> {
    DECLARATION_ENGINE.get_trait_fn(index)
}

pub(crate) fn de_insert_trait_impl(trait_impl: TypedTraitImpl) -> DeclarationId {
    DECLARATION_ENGINE.insert_trait_impl(trait_impl)
}

pub(crate) fn de_get_trait_impl(index: DeclarationId) -> Result<TypedTraitImpl, String> {
    DECLARATION_ENGINE.get_trait_impl(index)
}

pub(crate) fn de_insert_struct(r#struct: TypedStructDeclaration) -> DeclarationId {
    DECLARATION_ENGINE.insert_struct(r#struct)
}

pub(crate) fn de_get_struct(index: DeclarationId) -> Result<TypedStructDeclaration, String> {
    DECLARATION_ENGINE.get_struct(index)
}

pub(crate) fn de_add_monomorphized_struct_copy(
    original_id: DeclarationId,
    new_copy: TypedStructDeclaration,
) {
    DECLARATION_ENGINE.add_monomorphized_struct_copy(original_id, new_copy);
}

pub(crate) fn de_get_monomorphized_struct_copies(
    original_id: DeclarationId,
) -> Result<Vec<TypedStructDeclaration>, String> {
    DECLARATION_ENGINE.get_monomorphized_struct_copies(original_id)
}
