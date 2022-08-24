use std::{
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    type_system::{type_id::TypeId, type_info::TypeInfo},
    types::pretty_print::PrettyPrint,
};

#[derive(Debug, Default, Clone)]
pub(crate) struct ConcurrentSlab<I, T> {
    indexer: PhantomData<I>,
    inner: Arc<RwLock<Vec<T>>>,
}

impl<I, T> PrettyPrint for ConcurrentSlab<I, T>
where
    T: PrettyPrint,
{
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        let inner = self.inner.write().unwrap();
        inner
            .iter()
            .map(|i| i.pretty_print(declaration_engine))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl<I, T> ConcurrentSlab<I, T>
where
    T: Clone,
    I: From<usize> + std::ops::Deref<Target = usize>,
{
    pub fn insert(&self, value: T) -> I {
        let mut inner = self.inner.write().unwrap();
        let ret = inner.len();
        inner.push(value);
        ret.into()
    }

    pub fn get(&self, index: I) -> T {
        let inner = self.inner.read().unwrap();
        inner[*index].clone()
    }
}

impl ConcurrentSlab<TypeId, TypeInfo> {
    pub fn replace(
        &self,
        index: TypeId,
        prev_value: &TypeInfo,
        new_value: TypeInfo,
    ) -> Option<TypeInfo> {
        // The comparison below ends up calling functions in the slab, which
        // can lead to deadlocks if we used a single read/write lock.
        // So we split the operation: we do the read only operations with
        // a single scoped read lock below, and only after the scope do
        // we get a write lock for writing into the slab.
        {
            let inner = self.inner.read().unwrap();
            let actual_prev_value = &inner[*index];
            if actual_prev_value != prev_value {
                return Some(actual_prev_value.clone());
            }
        }

        let mut inner = self.inner.write().unwrap();
        inner[*index] = new_value;
        None
    }
}
