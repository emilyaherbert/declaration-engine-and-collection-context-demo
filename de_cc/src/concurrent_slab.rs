use std::{fmt, sync::RwLock};

#[derive(Debug)]
pub(crate) struct ConcurrentSlab<T> {
    inner: RwLock<Vec<T>>,
}

impl<T> Default for ConcurrentSlab<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T> ConcurrentSlab<T>
where
    T: Clone + PartialEq,
{
    pub(crate) fn debug_print(&self)
    where
        T: fmt::Debug,
    {
        let inner = self.inner.read().unwrap();
        inner.iter().enumerate().for_each(|(i, elem)| {
            println!("{} -> {:?}", i, elem);
        })
    }

    pub(crate) fn insert(&self, value: T) -> usize {
        let mut inner = self.inner.write().unwrap();
        let ret = inner.len();
        inner.push(value);
        ret
    }

    pub(crate) fn get(&self, index: usize) -> T {
        let inner = self.inner.read().unwrap();
        inner[index].clone()
    }

    pub(crate) fn replace(&self, index: usize, prev_value: &T, new_value: T) -> Option<T> {
        // The comparison below ends up calling functions in the slab, which
        // can lead to deadlocks if we used a single read/write lock.
        // So we split the operation: we do the read only operations with
        // a single scoped read lock below, and only after the scope do
        // we get a write lock for writing into the slab.
        {
            let inner = self.inner.read().unwrap();
            let actual_prev_value = &inner[index];
            if actual_prev_value != prev_value {
                return Some(actual_prev_value.clone());
            }
        }

        let mut inner = self.inner.write().unwrap();
        inner[index] = new_value;
        None
    }

    pub(crate) fn clear(&self) {
        let mut inner = self.inner.write().unwrap();
        *inner = Vec::new();
    }

    #[allow(dead_code)]
    pub(crate) fn exists<F: Fn(&T) -> bool>(&self, f: F) -> bool {
        let inner = self.inner.read().unwrap();
        inner.iter().any(f)
    }
}
