use crate::language::{TypedEnumVariant, TypedStructField};

use super::{type_argument::*, type_id::*, type_parameter::*, IntegerBits};

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
    Custom {
        name: String,
        type_arguments: Option<Vec<TypeArgument>>,
    },
    SelfType,
    Numeric,
    ErrorRecovery,
}
