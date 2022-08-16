use crate::type_system::{type_argument::TypeArgument, type_id::TypeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeclarationRef {
    Function(String, Vec<TypeId>, Vec<TypeId>),
}
