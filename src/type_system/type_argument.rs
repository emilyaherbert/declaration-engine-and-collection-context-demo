use super::type_id::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeArgument {
    pub(crate) type_id: TypeId,
}
