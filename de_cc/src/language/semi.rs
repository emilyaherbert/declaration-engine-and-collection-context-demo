use std::fmt;

use indent_write::fmt::IndentWriter;

use crate::{
    declaration_engine::declaration_id::DeclarationId,
    type_system::{
        type_engine::{insert_type, MonomorphizeHelper},
        type_id::TypeId,
        type_info::TypeInfo,
        type_mapping::TypeMapping,
        type_parameter::TypeParameter,
    },
    types::{copy_types::CopyTypes, create_type_id::CreateTypeId},
};

use super::untyped::Node;

#[derive(Clone)]
pub(crate) struct SemiTypedFunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<SemiTypedFunctionParameter>,
    pub(crate) body: Vec<Node>,
    pub(crate) return_type: TypeId,
}

// impl CopyTypes for SemiTypedFunctionDeclaration {
//     fn copy_types(&mut self, type_mapping: &TypeMapping) {
//         self.type_parameters
//             .iter_mut()
//             .for_each(|x| x.copy_types(type_mapping));
//         self.parameters
//             .iter_mut()
//             .for_each(|x| x.copy_types(type_mapping));
//         self.return_type.copy_types(type_mapping);
//         self.body
//             .iter_mut()
//             .for_each(|node| node.copy_types(type_mapping));
//     }
// }

// impl MonomorphizeHelper for SemiTypedFunctionDeclaration {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn type_parameters(&self) -> &[TypeParameter] {
//         &self.type_parameters
//     }
// }

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct SemiTypedFunctionParameter {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

// impl CopyTypes for SemiTypedFunctionParameter {
//     fn copy_types(&mut self, type_mapping: &TypeMapping) {
//         self.type_id.copy_types(type_mapping);
//     }
// }

// impl fmt::Display for SemiTypedFunctionParameter {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}: {}", self.name, self.type_id)
//     }
// }

#[derive(Clone)]
pub(crate) struct SemiTypedTraitDeclaration {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<DeclarationId>,
}

#[derive(Clone)]
pub(crate) struct SemiTypedTraitFn {
    pub(crate) name: String,
    pub(crate) parameters: Vec<SemiTypedFunctionParameter>,
    pub(crate) return_type: TypeId,
}

// impl fmt::Display for SemiTypedTraitFn {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "fn {}({}) -> {}",
//             self.name,
//             self.parameters
//                 .iter()
//                 .map(|parameter| parameter.to_string())
//                 .collect::<Vec<_>>()
//                 .join(", "),
//             self.return_type
//         )
//     }
// }

#[derive(Clone, Debug)]
pub(crate) struct SemiTypedTraitImpl {
    pub(crate) trait_name: String,
    pub(crate) type_implementing_for: TypeId,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) methods: Vec<DeclarationId>,
}

#[derive(Clone)]
pub(crate) struct SemiTypedStructDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) fields: Vec<SemiTypedStructField>,
}

// impl CreateTypeId for SemiTypedStructDeclaration {
//     fn create_type_id(&self) -> TypeId {
//         insert_type(TypeInfo::Struct {
//             name: self.name.clone(),
//             type_parameters: self.type_parameters.clone(),
//             fields: self.fields.clone(),
//         })
//     }
// }

// impl CopyTypes for SemiTypedStructDeclaration {
//     fn copy_types(&mut self, type_mapping: &TypeMapping) {
//         self.type_parameters
//             .iter_mut()
//             .for_each(|x| x.copy_types(type_mapping));
//         self.fields
//             .iter_mut()
//             .for_each(|x| x.copy_types(type_mapping));
//     }
// }

// impl MonomorphizeHelper for SemiTypedStructDeclaration {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn type_parameters(&self) -> &[TypeParameter] {
//         &self.type_parameters
//     }
// }

// impl fmt::Display for SemiTypedStructDeclaration {
//     fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(
//             f,
//             "struct {}{} {{",
//             self.name,
//             if self.type_parameters.is_empty() {
//                 "".to_string()
//             } else {
//                 format!(
//                     "<{}>",
//                     self.type_parameters
//                         .iter()
//                         .map(|x| x.to_string())
//                         .collect::<Vec<_>>()
//                         .join(", ")
//                 )
//             }
//         )
//         .unwrap();
//         {
//             let mut indent = IndentWriter::new("  ", &mut f);
//             for field in self.fields.iter() {
//                 writeln!(indent, "{},", field).unwrap();
//             }
//         }
//         write!(f, "}}")
//     }
// }

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct SemiTypedStructField {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

// impl CopyTypes for SemiTypedStructField {
//     fn copy_types(&mut self, type_mapping: &TypeMapping) {
//         self.type_id.copy_types(type_mapping);
//     }
// }

// impl fmt::Display for SemiTypedStructField {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}({})", self.name, self.type_id)
//     }
// }
