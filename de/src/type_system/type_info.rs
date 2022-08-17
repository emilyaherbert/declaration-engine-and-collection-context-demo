use std::hash::Hash;
use std::{fmt, hash::Hasher};

use crate::{
    declaration_engine::declaration_ref::DeclarationRef,
    language::typed::typed_declaration::{TypedEnumVariant, TypedStructField},
};

use super::type_engine::look_up_type_id;
use super::{type_id::*, type_parameter::*, IntegerBits};

#[derive(Clone, Eq)]
pub enum TypeInfo {
    Unknown,
    UnknownGeneric {
        name: String,
    },
    UnsignedInteger(IntegerBits),
    Enum {
        name: String,
        type_parameters: Vec<TypeParameter>,
        variant_types: Vec<TypedEnumVariant>,
    },
    Struct {
        name: String,
        type_parameters: Vec<TypeParameter>,
        fields: Vec<TypedStructField>,
    },
    Ref(TypeId),
    DeclarationRef(DeclarationRef),
}

impl Default for TypeInfo {
    fn default() -> Self {
        TypeInfo::Unknown
    }
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeInfo::Unknown => write!(f, "UNK"),
            TypeInfo::UnknownGeneric { name } => write!(f, "{}", name),
            TypeInfo::UnsignedInteger(bits) => write!(f, "{}", bits),
            TypeInfo::Enum { .. } => todo!(),
            TypeInfo::Struct { .. } => todo!(),
            TypeInfo::Ref(_) => todo!(),
            TypeInfo::DeclarationRef(decl_ref) => write!(f, "{}", decl_ref),
        }
    }
}

impl Hash for TypeInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
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
            TypeInfo::Enum {
                name,
                type_parameters,
                variant_types,
            } => {
                state.write_u8(4);
                name.hash(state);
                type_parameters.hash(state);
                variant_types.hash(state);
            }
            TypeInfo::Struct {
                name,
                type_parameters,
                fields,
            } => {
                state.write_u8(5);
                name.hash(state);
                type_parameters.hash(state);
                fields.hash(state);
            }
            TypeInfo::Ref(id) => {
                state.write_u8(6);
                look_up_type_id(*id).hash(state);
            }
            TypeInfo::DeclarationRef(_) => todo!(),
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
            (
                TypeInfo::Enum {
                    name: l_name,
                    variant_types: l_variant_types,
                    type_parameters: l_type_parameters,
                },
                TypeInfo::Enum {
                    name: r_name,
                    variant_types: r_variant_types,
                    type_parameters: r_type_parameters,
                },
            ) => {
                l_name == r_name
                    && l_variant_types == r_variant_types
                    && l_type_parameters == r_type_parameters
            }
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
            (TypeInfo::Ref(l), TypeInfo::Ref(r)) => look_up_type_id(*l) == look_up_type_id(*r),
            (TypeInfo::DeclarationRef(_), _) => todo!(),
            _ => false,
        }
    }
}

pub mod constructors {
    use crate::type_system::IntegerBits;

    use super::TypeInfo;

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
}
