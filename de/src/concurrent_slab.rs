use std::sync::RwLock;

use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    type_system::{type_id::TypeId, type_info::TypeInfo},
    types::pretty_print::PrettyPrint,
};

#[derive(Debug, Default)]
pub struct ConcurrentSlab<T> {
    inner: RwLock<Vec<T>>,
}

impl<T> PrettyPrint for ConcurrentSlab<T>
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

impl<T> ConcurrentSlab<T>
where
    T: Clone,
{
    pub fn insert<U>(&self, value: T) -> U
    where
        U: From<usize>,
    {
        let mut inner = self.inner.write().unwrap();
        let ret = inner.len();
        inner.push(value);
        ret.into()
    }

    pub fn get<U>(&self, index: U) -> T
    where
        U: std::ops::Deref<Target = usize>,
    {
        let inner = self.inner.read().unwrap();
        inner[*index].clone()
    }

    pub fn clear(&self) {
        let mut inner = self.inner.write().unwrap();
        *inner = Vec::new();
    }

    pub fn exists<F: Fn(&T) -> bool>(&self, f: F) -> bool {
        let inner = self.inner.read().unwrap();
        inner.iter().any(f)
    }
}

impl ConcurrentSlab<TypeInfo> {
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
