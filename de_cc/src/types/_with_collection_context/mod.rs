pub(crate) mod debug;
pub(crate) mod partial_eq;
pub(crate) mod with_cc;

pub(crate) use debug::*;
pub(crate) use partial_eq::*;
pub(crate) use with_cc::*;

use crate::collection_context::collection_context::CollectionContext;

pub(crate) struct WrapperCC<'a, 'c, T: ?Sized> {
    thing: &'a T,
    cc: &'c CollectionContext,
}
