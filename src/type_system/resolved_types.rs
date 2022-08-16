use std::fmt;

use crate::type_system::IntegerBits;

pub(crate) enum ResolvedType {
    UnsignedInteger(IntegerBits),
    Enum {
        name: String,
        type_parameters: Vec<ResolvedTypeParameter>,
        variant_types: Vec<ResolvedEnumVariant>,
    },
    Struct {
        name: String,
        type_parameters: Vec<ResolvedTypeParameter>,
        fields: Vec<ResolvedStructField>,
    },
}

impl fmt::Display for ResolvedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedType::UnsignedInteger(bits) => write!(f, "{}", bits),
            ResolvedType::Enum { .. } => todo!(),
            ResolvedType::Struct { .. } => todo!(),
        }
    }
}

pub(crate) struct ResolvedTypeParameter {
    pub(crate) name_ident: String,
    pub(crate) type_info: ResolvedType,
}

impl fmt::Display for ResolvedTypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name_ident)
    }
}

pub(crate) struct ResolvedEnumVariant {
    pub(crate) name: String,
    pub(crate) type_info: ResolvedType,
    pub(crate) tag: usize,
}

pub(crate) struct ResolvedStructField {
    pub(crate) name: String,
    pub(crate) type_info: ResolvedType,
}
