use std::fmt;

use crate::type_system::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeclarationRef {
    Function(String, Vec<TypeId>, Vec<TypeId>),
}

impl fmt::Display for DeclarationRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeclarationRef::Function(name, type_parameters, parameters) => {
                write!(
                    f,
                    "function_ref({}, [{}], [{}])",
                    name,
                    type_parameters
                        .iter()
                        .map(|type_id| type_id.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    parameters
                        .iter()
                        .map(|type_id| type_id.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }
        }
    }
}
