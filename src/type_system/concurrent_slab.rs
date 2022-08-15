use std::sync::RwLock;

use super::type_id::TypeId;

#[derive(Debug, Default)]
pub(super) struct ConcurrentSlab<T> {
    inner: RwLock<Vec<T>>,
}

impl<T> ConcurrentSlab<T>
where
    T: Clone + PartialEq,
{
    pub(super) fn insert(&self, value: T) -> TypeId {
        let mut inner = self.inner.write().unwrap();
        let ret = inner.len();
        inner.push(value);
        ret.into()
    }

    pub(super) fn get(&self, index: TypeId) -> T {
        let inner = self.inner.read().unwrap();
        inner[*index].clone()
    }

    pub(super) fn replace(&self, index: TypeId, prev_value: &T, new_value: T) -> Option<T> {
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

    pub(super) fn clear(&self) {
        let mut inner = self.inner.write().unwrap();
        *inner = Vec::new();
    }

    pub(super) fn exists<F: Fn(&T) -> bool>(&self, f: F) -> bool {
        let inner = self.inner.read().unwrap();
        inner.iter().any(f)
    }
}