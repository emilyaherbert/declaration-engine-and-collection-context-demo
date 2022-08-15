use super::type_id::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameter {
    pub(crate) name_ident: String,
    pub(crate) type_id: TypeId,
}
