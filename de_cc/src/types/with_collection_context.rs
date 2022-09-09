use std::fmt;

use crate::collection_context::collection_context::CollectionContext;

pub(crate) use de_cc_macros::*;

pub struct WithCC<'a, 'c, T: ?Sized> {
    thing: &'a T,
    cc: &'c CollectionContext,
}

pub trait DebugWithCC {
    fn fmt_with_cc(&self, f: &mut fmt::Formatter, cc: &CollectionContext) -> fmt::Result;

    fn with_cc<'a, 'c>(&'a self, cc: &'c CollectionContext) -> WithCC<'a, 'c, Self> {
        WithCC { thing: self, cc }
    }
}

impl<'t, T> DebugWithCC for &'t T
where
    T: fmt::Debug,
{
    fn fmt_with_cc(&self, f: &mut fmt::Formatter, _cc: &CollectionContext) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<'a, 'c, T> fmt::Debug for WithCC<'a, 'c, T>
where
    T: DebugWithCC,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let WithCC { thing, cc } = self;
        (*thing).fmt_with_cc(f, cc)
    }
}

impl<T> DebugWithCC for Vec<T>
where
    T: DebugWithCC,
{
    fn fmt_with_cc(&self, f: &mut fmt::Formatter, cc: &CollectionContext) -> fmt::Result {
        f.debug_list()
            .entries(self.iter().map(|value| (*value).with_cc(cc)))
            .finish()
    }
}

impl<T> DebugWithCC for [T]
where
    T: DebugWithCC,
{
    fn fmt_with_cc(&self, f: &mut fmt::Formatter, cc: &CollectionContext) -> fmt::Result {
        f.debug_list()
            .entries(self.iter().map(|value| (*value).with_cc(cc)))
            .finish()
    }
}

impl<T> DebugWithCC for Option<T>
where
    T: DebugWithCC,
{
    fn fmt_with_cc(&self, f: &mut fmt::Formatter, cc: &CollectionContext) -> fmt::Result {
        match self {
            Some(value) => f.debug_tuple("Some").field(&(*value).with_cc(cc)).finish(),
            None => f.write_str("None"),
        }
    }
}
