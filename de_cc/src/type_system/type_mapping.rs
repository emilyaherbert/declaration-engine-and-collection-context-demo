use std::collections::HashMap;

use super::{
    type_engine::insert_type, type_id::TypeId, type_info::TypeInfo, type_parameter::TypeParameter,
};

/// old type id -> new type id
pub(crate) type TypeMapping = HashMap<TypeId, TypeId>;

// pub(crate) fn insert_type_parameters(type_parameters: Vec<TypeParameter>) -> TypeMapping {
//     type_parameters
//         .into_iter()
//         .map(|x| {
//             (
//                 x.type_id,
//                 insert_type(TypeInfo::TypeParamRef(
//                     x.name.clone(),
//                     Arc::new(RwLock::new(insert_type(TypeInfo::UnknownGeneric {
//                         name: x.name,
//                     }))),
//                 )),
//             )
//         })
//         .collect()
// }

pub(crate) fn insert_type_parameters(type_parameters: Vec<TypeParameter>) -> TypeMapping {
    type_parameters
        .into_iter()
        .map(|x| {
            (
                x.type_id,
                insert_type(TypeInfo::UnknownGeneric { name: x.name }),
            )
        })
        .collect()
}
