use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

use super::type_engine::look_up_type_id;
use super::type_mapping::TypeMapping;
use super::{type_id::*, IntegerBits};

#[derive(Clone, Eq)]
pub enum TypeInfo {
    ErrorRecovery,
    Unknown,
    UnknownGeneric { name: String },
    Unit,
    Ref(TypeId),
    UnsignedInteger(IntegerBits),
    // Enum {
    //     name: String,
    //     type_parameters: Vec<TypeParameter>,
    //     variant_types: Vec<TypedEnumVariant>,
    // },
    // Struct {
    //     name: String,
    //     type_parameters: Vec<TypeParameter>,
    //     fields: Vec<TypedStructField>,
    // },
}

impl Default for TypeInfo {
    fn default() -> Self {
        TypeInfo::Unknown
    }
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeInfo::ErrorRecovery => write!(f, "ERR"),
            TypeInfo::Unknown => write!(f, "UNK"),
            TypeInfo::UnknownGeneric { name } => write!(f, "{}", name),
            TypeInfo::UnsignedInteger(bits) => write!(f, "{}", bits),
            TypeInfo::Ref(_) => todo!(),
            TypeInfo::Unit => write!(f, "()"),
        }
    }
}

impl Hash for TypeInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TypeInfo::ErrorRecovery => {
                state.write_u8(0);
            }
            TypeInfo::Unknown => {
                state.write_u8(1);
            }
            TypeInfo::UnknownGeneric { name } => {
                state.write_u8(2);
                name.hash(state);
            }
            TypeInfo::UnsignedInteger(bits) => {
                state.write_u8(3);
                bits.hash(state);
            }
            TypeInfo::Ref(id) => {
                state.write_u8(4);
                look_up_type_id(*id).hash(state);
            }
            TypeInfo::Unit => {
                state.write_u8(5);
            } // TypeInfo::Enum {
              //     name,
              //     type_parameters,
              //     variant_types,
              // } => {
              //     state.write_u8(4);
              //     name.hash(state);
              //     type_parameters.hash(state);
              //     variant_types.hash(state);
              // }
              // TypeInfo::Struct {
              //     name,
              //     type_parameters,
              //     fields,
              // } => {
              //     state.write_u8(5);
              //     name.hash(state);
              //     type_parameters.hash(state);
              //     fields.hash(state);
              // }
        }
    }
}

impl PartialEq for TypeInfo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeInfo::Unknown, TypeInfo::Unknown) => true,
            (
                TypeInfo::UnknownGeneric { name: l_name },
                TypeInfo::UnknownGeneric { name: r_name },
            ) => l_name == r_name,
            (TypeInfo::UnsignedInteger(l), TypeInfo::UnsignedInteger(r)) => l == r,
            (TypeInfo::Ref(l), TypeInfo::Ref(r)) => look_up_type_id(*l) == look_up_type_id(*r),
            _ => false,
        }
    }
}

impl TypeInfo {
    pub(crate) fn matches_type_parameter(&self, mapping: &TypeMapping) -> Option<TypeId> {
        match self {
            TypeInfo::UnknownGeneric { .. } => {
                for (param, ty_id) in mapping.iter() {
                    if look_up_type_id(*param) == *self {
                        return Some(*ty_id);
                    }
                }
                None
            }
            TypeInfo::ErrorRecovery
            | TypeInfo::Unknown
            | TypeInfo::Unit
            | TypeInfo::Ref(_)
            | TypeInfo::UnsignedInteger(_) => None,
        }
    }
}

pub mod constructors {
    use crate::type_system::IntegerBits;

    use super::TypeInfo;

    pub fn t_(name: &str) -> TypeInfo {
        TypeInfo::UnknownGeneric {
            name: name.to_string(),
        }
    }

    pub fn t_u8() -> TypeInfo {
        TypeInfo::UnsignedInteger(IntegerBits::Eight)
    }

    pub fn t_u16() -> TypeInfo {
        TypeInfo::UnsignedInteger(IntegerBits::Sixteen)
    }

    pub fn t_u32() -> TypeInfo {
        TypeInfo::UnsignedInteger(IntegerBits::ThirtyTwo)
    }

    pub fn t_u64() -> TypeInfo {
        TypeInfo::UnsignedInteger(IntegerBits::SixtyFour)
    }

    pub fn t_unit() -> TypeInfo {
        TypeInfo::Unit
    }
}
