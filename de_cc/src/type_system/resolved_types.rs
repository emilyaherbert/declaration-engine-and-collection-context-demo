use std::fmt;

use crate::{
    language::resolved::resolved_declaration::ResolvedStructField, type_system::IntegerBits,
};

/// Subset of [TypeInfo](crate::type_system::type_info::TypeInfo) that gaurentees a well-formed AST.
#[derive(Clone)]
pub(crate) enum ResolvedType {
    UnsignedInteger(IntegerBits),
    Unit,
    Struct {
        name: String,
        type_parameters: Vec<ResolvedTypeParameter>,
        #[allow(dead_code)]
        fields: Vec<ResolvedStructField>,
    },
}

impl fmt::Display for ResolvedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedType::UnsignedInteger(bits) => write!(f, "{}", bits),
            ResolvedType::Unit => write!(f, "()"),
            ResolvedType::Struct {
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
            } // ResolvedType::Enum { .. } => todo!(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct ResolvedTypeParameter {
    pub(crate) type_info: ResolvedType,
}

impl fmt::Display for ResolvedTypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_info)
    }
}

// pub(crate) struct ResolvedEnumVariant {
//     pub(crate) name: String,
//     pub(crate) type_info: ResolvedType,
//     pub(crate) tag: usize,
// }

// pub(crate) struct ResolvedStructField {
//     pub(crate) name: String,
//     pub(crate) type_info: ResolvedType,
// }
