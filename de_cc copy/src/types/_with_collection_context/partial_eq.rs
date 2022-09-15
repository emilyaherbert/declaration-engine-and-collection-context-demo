use crate::collection_context::collection_context::CollectionContext;

pub(crate) use de_cc_macros::*;

use super::{with_cc::WithCC, WrapperCC};

pub(crate) trait PartialEqWithCC {
    fn eq_with_cc(&self, other: &Self, cc: &CollectionContext) -> bool;
}

impl<'t, T> PartialEqWithCC for &'t T
where
    T: PartialEq,
{
    fn eq_with_cc(&self, other: &Self, cc: &CollectionContext) -> bool {
        todo!()
    }
}

impl<'a, 'c, T> PartialEq for WrapperCC<'a, 'c, T>
where
    T: PartialEqWithCC,
{
    fn eq(&self, other: &Self) -> bool {
        let WrapperCC { thing: l, cc } = self;
        let WrapperCC { thing: r, .. } = self;
        (*l).eq_with_cc(*r, cc)
    }
}

impl<T> PartialEqWithCC for Vec<T>
where
    T: PartialEqWithCC + WithCC,
{
    fn eq_with_cc(&self, other: &Self, cc: &CollectionContext) -> bool {
        let l = self.iter().map(|x| x.with_cc(cc)).collect::<Vec<_>>();
        let r = self.iter().map(|x| x.with_cc(cc)).collect::<Vec<_>>();
        l == r
    }
}

impl<T> PartialEqWithCC for [T]
where
    T: PartialEqWithCC + WithCC,
{
    fn eq_with_cc(&self, other: &Self, cc: &CollectionContext) -> bool {
        let l = self.iter().map(|x| x.with_cc(cc)).collect::<Vec<_>>();
        let r = self.iter().map(|x| x.with_cc(cc)).collect::<Vec<_>>();
        l == r
    }
}

impl<T> PartialEqWithCC for Option<T>
where
    T: PartialEqWithCC + WithCC,
{
    fn eq_with_cc(&self, other: &Self, cc: &CollectionContext) -> bool {
        let l = self.map(|x| x.with_cc(cc));
        let r = self.map(|x| x.with_cc(cc));
        l == r
    }
}
