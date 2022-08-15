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
