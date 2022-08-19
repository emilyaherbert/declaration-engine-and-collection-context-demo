use std::fmt;
use std::hash::Hash;

use crate::types::copy_types::CopyTypes;

use super::type_engine::{insert_type, look_up_type_id, look_up_type_id_raw};
use super::type_id::TypeId;
use super::type_info::TypeInfo;
use super::type_mapping::TypeMapping;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParameter {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

impl CopyTypes for TypeParameter {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_id = match look_up_type_id(self.type_id).matches_type_parameter(type_mapping) {
            Some(matching_id) => insert_type(TypeInfo::Ref(matching_id)),
            None => {
                let ty = TypeInfo::Ref(insert_type(look_up_type_id_raw(self.type_id)));
                insert_type(ty)
            }
        };
    }
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
