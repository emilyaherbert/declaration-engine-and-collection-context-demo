use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

use crate::language::ty::typed_declaration::TyStructField;

use super::type_argument::TypeArgument;
use super::type_engine::look_up_type_id;
use super::type_parameter::TypeParameter;
use super::{type_id::*, IntegerBits};

#[derive(Clone)]
pub enum TypeInfo {
    ErrorRecovery,
    Unknown,
    UnknownGeneric {
        name: String,
    },
    Custom {
        name: String,
        type_arguments: Vec<TypeArgument>,
    },
    Unit,
    Ref(TypeId),
    UnsignedInteger(IntegerBits),
    Struct {
        name: String,
        type_parameters: Vec<TypeParameter>,
        fields: Vec<TyStructField>,
    },
}

impl Eq for TypeInfo {}

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
            TypeInfo::Custom {
                name,
                type_arguments,
            } => {
                write!(
                    f,
                    "{{{}{}}}",
                    name,
                    if type_arguments.is_empty() {
                        "".to_string()
                    } else {
                        format!(
                            "<{}>",
                            type_arguments
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    },
                )
            }
            TypeInfo::UnsignedInteger(bits) => write!(f, "{}", bits),
            TypeInfo::Ref(id) => write!(f, "{}", look_up_type_id(*id)),
            TypeInfo::Unit => write!(f, "()"),
            TypeInfo::Struct {
                name,
                type_parameters,
                ..
            } => {
                write!(
                    f,
                    "{}{}",
                    name,
                    if type_parameters.is_empty() {
                        "".to_string()
                    } else {
                        format!(
                            "<{}>",
                            type_parameters
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    },
                )
            }
        }
    }
}

impl fmt::Debug for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeInfo::ErrorRecovery => write!(f, "ERR"),
            TypeInfo::Unknown => write!(f, "UNK"),
            TypeInfo::UnknownGeneric { name } => write!(f, "{}", name),
            TypeInfo::Custom {
                name,
                type_arguments,
            } => {
                write!(
                    f,
                    "{{{}{}}}",
                    name,
                    if type_arguments.is_empty() {
                        "".to_string()
                    } else {
                        format!(
                            "<{}>",
                            type_arguments
                                .iter()
                                .map(|x| format!("{:?}", x))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    },
                )
            }
            TypeInfo::UnsignedInteger(bits) => write!(f, "{}", bits),
            TypeInfo::Ref(id) => write!(f, "ref..{}", **id),
            TypeInfo::Unit => write!(f, "()"),
            TypeInfo::Struct {
                name,
                type_parameters,
                fields,
            } => {
                write!(
                    f,
                    "{}{}{{{}}}",
                    name,
                    if type_parameters.is_empty() {
                        "".to_string()
                    } else {
                        format!(
                            "<{}>",
                            type_parameters
                                .iter()
                                .map(|x| format!("{:?}", x))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    },
                    fields
                        .iter()
                        .map(|x| format!("{:?}", TypeInfo::Ref(x.type_id)))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
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
                id.hash(state);
            }
            TypeInfo::Unit => {
                state.write_u8(5);
            }
            TypeInfo::Struct {
                name,
                type_parameters,
                fields,
            } => {
                state.write_u8(6);
                name.hash(state);
                type_parameters.hash(state);
                fields.hash(state);
            }
            TypeInfo::Custom {
                name,
                type_arguments,
            } => {
                state.write_u8(7);
                name.hash(state);
                type_arguments.hash(state);
            }
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
            (TypeInfo::ErrorRecovery, TypeInfo::ErrorRecovery) => todo!(),
            (
                TypeInfo::Custom {
                    name: l_name,
                    type_arguments: l_type_args,
                },
                TypeInfo::Custom {
                    name: r_name,
                    type_arguments: r_type_args,
                },
            ) => l_name == r_name && l_type_args == r_type_args,
            (TypeInfo::Unit, TypeInfo::Unit) => true,
            (
                TypeInfo::Struct {
                    name: l_name,
                    fields: l_fields,
                    type_parameters: l_type_parameters,
                },
                TypeInfo::Struct {
                    name: r_name,
                    fields: r_fields,
                    type_parameters: r_type_parameters,
                },
            ) => l_name == r_name && l_fields == r_fields && l_type_parameters == r_type_parameters,
            _ => false,
        }
    }
}

pub mod constructors {
    use crate::type_system::{type_argument::TypeArgument, type_engine::insert_type, IntegerBits};

    use super::TypeInfo;

    pub fn t_gen_(name: &str) -> TypeInfo {
        TypeInfo::UnknownGeneric {
            name: name.to_string(),
        }
    }

    pub fn t_cus_(name: &str, type_arguments: &[TypeInfo]) -> TypeInfo {
        TypeInfo::Custom {
            name: name.to_string(),
            type_arguments: type_arguments
                .iter()
                .cloned()
                .map(|type_info| TypeArgument {
                    type_id: insert_type(type_info),
                })
                .collect(),
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
