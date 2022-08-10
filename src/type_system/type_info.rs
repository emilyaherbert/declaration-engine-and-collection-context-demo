use crate::language::{TypedEnumVariant, TypedStructField};

use super::{type_id::*, type_parameter::*, IntegerBits};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypeInfo {
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
    ErrorRecovery,
}

impl Default for TypeInfo {
    fn default() -> Self {
        TypeInfo::Unknown
    }
}
