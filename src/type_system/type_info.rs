use std::fmt;

use crate::{
    declaration_engine::declaration_ref::DeclarationRef,
    language::typed::typed_declaration::{TypedEnumVariant, TypedStructField},
};

use super::{type_id::*, type_parameter::*, IntegerBits};

#[derive(Debug, Clone, PartialEq)]
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
            TypeInfo::Enum {
                name,
                type_parameters,
                variant_types,
            } => todo!(),
            TypeInfo::Struct {
                name,
                type_parameters,
                fields,
            } => todo!(),
            TypeInfo::Ref(_) => todo!(),
            TypeInfo::DeclarationRef(_) => todo!(),
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
