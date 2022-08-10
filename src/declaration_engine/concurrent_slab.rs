// https://github.com/FuelLabs/sway/blob/0a4ac5da5ef926a4fe2a9c92ff2b6a3f87d8cb75/sway-core/src/concurrent_slab.rs

use std::sync::RwLock;

use super::DeclarationId;

#[derive(Debug)]
pub struct ConcurrentSlab<T> {
    inner: RwLock<Vec<T>>,
}

impl<T> Default for ConcurrentSlab<T> {
    fn default() -> ConcurrentSlab<T> {
        ConcurrentSlab {
            inner: RwLock::default(),
        }
    }
}

impl<T> ConcurrentSlab<T>
where
    T: Clone + PartialEq,
{
    pub fn insert(&self, value: T) -> DeclarationId {
        let mut inner = self.inner.write().unwrap();
        let ret = inner.len();
        inner.push(value);
        ret.into()
    }

    pub fn get(&self, index: DeclarationId) -> T {
        let inner = self.inner.read().unwrap();
        inner[*index].clone()
    }

    pub fn replace(&self, index: DeclarationId, prev_value: &T, new_value: T) -> Option<T> {
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

    pub fn clear(&self) {
        let mut inner = self.inner.write().unwrap();
        *inner = Vec::new();
    }

    pub fn exists<F: Fn(&T) -> bool>(&self, f: F) -> bool {
        let inner = self.inner.read().unwrap();
        inner.iter().any(f)
    }
}
