use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

use either::Either;

use crate::type_system::OccursCheck;
use crate::types::copy_types::CopyTypes;

use super::type_engine::{
    insert_type, look_up_type_id, look_up_type_id_raw, type_matches_type_parameter,
};
use super::type_info::TypeInfo;
use super::type_mapping::TypeMapping;

#[derive(Eq, Clone, Copy, Default)]
pub struct TypeId(usize);

impl std::ops::Deref for TypeId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for TypeId {
    fn from(o: usize) -> Self {
        TypeId(o)
    }
}

impl fmt::Debug for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.occurs_check() {
            panic!("recursive type has infinite size");
        }
        write!(f, "{}-{:?}", self.0, look_up_type_id_raw(*self))
    }
}

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", look_up_type_id(*self))
    }
}

impl Hash for TypeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        look_up_type_id(*self).hash(state);
    }
}

impl PartialEq for TypeId {
    fn eq(&self, other: &Self) -> bool {
        look_up_type_id(*self) == look_up_type_id(*other)
    }
}

impl CopyTypes for TypeId {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        if let Some(matching_id) = type_matches_type_parameter(*self, type_mapping) {
            *self = insert_type(TypeInfo::Ref(matching_id));
        }
    }
}

impl TypeId {
    pub(crate) fn new(index: usize) -> TypeId {
        TypeId(index)
    }

    /// Returns true if the given type occurs recursively within itself---i.e. if evaluating this type would create an infinite cycle.
    ///
    /// "occurs check: a check for whether the same variable occurs on both sides and, if it does, decline to unify"
    /// https://papl.cs.brown.edu/2016/Type_Inference.html
    pub(crate) fn occurs_check(&self) -> bool {
        match self.occurs_check_memo(HashSet::new()) {
            Either::Left(_) => false,
            Either::Right(_) => true,
        }
    }

    /// Performs the 'occurs check' using memoization.
    pub(super) fn occurs_check_memo(
        &self,
        mut memo: HashSet<usize>,
    ) -> Either<HashSet<usize>, OccursCheck> {
        let s = look_up_type_id_raw(*self);
        match s {
            TypeInfo::UnknownGeneric { name: _ } => {
                memo.insert(**self);
                Either::Left(memo)
            }
            TypeInfo::Custom {
                name: _,
                type_arguments,
            } => {
                memo.insert(**self);
                let mut next_memo = memo.clone();
                for ta in type_arguments.into_iter() {
                    if memo.contains(&*ta.type_id) {
                        return Either::Right(OccursCheck::Occurs);
                    }
                    match ta.type_id.occurs_check_memo(memo.clone()) {
                        Either::Left(ids) => {
                            next_memo.extend(ids.into_iter());
                        }
                        occurs @ Either::Right(_) => {
                            return occurs;
                        }
                    }
                }
                Either::Left(next_memo)
            }
            TypeInfo::Ref(next) => {
                memo.insert(**self);
                if memo.contains(&*next) {
                    return Either::Right(OccursCheck::Occurs);
                }
                next.occurs_check_memo(memo)
            }
            TypeInfo::Struct {
                name: _,
                type_parameters,
                fields,
            } => {
                memo.insert(**self);
                let mut next_memo = memo.clone();
                for tp in type_parameters.into_iter() {
                    if memo.contains(&*tp.type_id) {
                        return Either::Right(OccursCheck::Occurs);
                    }
                    match tp.type_id.occurs_check_memo(memo.clone()) {
                        Either::Left(ids) => {
                            next_memo.extend(ids.into_iter());
                        }
                        occurs @ Either::Right(_) => {
                            return occurs;
                        }
                    }
                }
                for f in fields.into_iter() {
                    if memo.contains(&*f.type_id) {
                        return Either::Right(OccursCheck::Occurs);
                    }
                    match f.type_id.occurs_check_memo(memo.clone()) {
                        Either::Left(ids) => {
                            next_memo.extend(ids.into_iter());
                        }
                        occurs @ Either::Right(_) => {
                            return occurs;
                        }
                    }
                }
                Either::Left(next_memo)
            }
            TypeInfo::ErrorRecovery
            | TypeInfo::Unknown
            | TypeInfo::Unit
            | TypeInfo::UnsignedInteger(_) => Either::Left(memo),
        }
    }
}
