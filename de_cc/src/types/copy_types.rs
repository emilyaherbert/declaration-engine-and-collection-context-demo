use either::Either;

use crate::type_system::type_mapping::TypeMapping;

pub(crate) trait CopyTypes {
    fn copy_types(&mut self, type_mapping: &TypeMapping);
}

impl<L, R> CopyTypes for Either<L, R>
where
    L: CopyTypes,
    R: CopyTypes,
{
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            Either::Left(l) => l.copy_types(type_mapping),
            Either::Right(r) => r.copy_types(type_mapping),
        }
    }
}
