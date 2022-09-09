use crate::collection_context::collection_context::CollectionContext;

pub(crate) use de_cc_macros::*;

use super::WrapperCC;

pub(crate) trait WithCC {
    fn with_cc<'a, 'c>(&'a self, cc: &'c CollectionContext) -> WrapperCC<'a, 'c, Self> {
        WrapperCC { thing: self, cc }
    }
}

impl<'t, T> WithCC for &'t T {}

impl<T> WithCC for Vec<T> where T: WithCC {}

impl<T> WithCC for [T] where T: WithCC {}

impl<T> WithCC for Option<T> where T: WithCC {}
