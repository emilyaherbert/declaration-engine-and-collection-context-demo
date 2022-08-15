use crate::language::typed::typed_declaration::{TypedEnumVariant, TypedStructField};

use super::{type_parameter::*, IntegerBits};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ResolvedTypeInfo {
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
    Numeric,
    ErrorRecovery,
}
