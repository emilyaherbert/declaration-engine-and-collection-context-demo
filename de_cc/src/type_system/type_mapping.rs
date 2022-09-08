use std::collections::HashMap;

use super::{
    type_engine::insert_type, type_id::TypeId, type_info::TypeInfo, type_parameter::TypeParameter,
};

pub(crate) type TypeMapping = HashMap<TypeId, TypeId>;

pub(crate) fn insert_type_parameters(type_parameters: &[TypeParameter]) -> TypeMapping {
    type_parameters
        .iter()
        .map(|x| {
            (
                x.type_id,
                insert_type(TypeInfo::UnknownGeneric {
                    name: x.name.clone(),
                }),
            )
        })
        .collect()
}
