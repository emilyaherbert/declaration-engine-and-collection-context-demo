/// Constraint placed on a type.
///
/// For example, this is used in where clauses:
///
/// ```ignore
/// fn add<T>(a: T, b: T) -> T where T: Math { .. }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TraitConstraint {
    pub(crate) trait_name: String,
}
