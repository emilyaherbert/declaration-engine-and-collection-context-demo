use either::Either;

use crate::{
    collection_context::collection_context::CollectionContext,
    type_system::type_mapping::TypeMapping,
};

pub(crate) trait CopyTypes {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping);
}

impl<L, R> CopyTypes for Either<L, R>
where
    L: CopyTypes,
    R: CopyTypes,
{
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        match self {
            Either::Left(l) => l.copy_types(cc, type_mapping),
            Either::Right(r) => r.copy_types(cc, type_mapping),
        }
    }
}
