use std::fmt;

use crate::{
    declaration_engine::declaration_id::DeclarationId,
    language::{
        typed::typed_declaration::TypedFunctionParameter, untyped::declaration::VariableDeclaration,
    },
    type_system::{type_id::TypeId, type_mapping::TypeMapping, type_parameter::TypeParameter},
    types::copy_types::CopyTypes,
};

use super::PartialNode;

#[derive(Clone, PartialEq, Debug)]
pub enum PartialDeclaration {
    Variable(VariableDeclaration),
    Function(DeclarationId),
    Trait(DeclarationId),
    TraitImpl(DeclarationId),
    Struct(DeclarationId),
    GenericTypeForFunctionScope { type_id: TypeId },
}

impl fmt::Display for PartialDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PartialDeclaration::Variable(decl) => write!(f, "{}", decl),
            PartialDeclaration::Function(decl) => write!(f, "\n{}", decl),
            PartialDeclaration::Trait(decl) => write!(f, "\n{}", decl),
            PartialDeclaration::TraitImpl(decl) => write!(f, "\n{}", decl),
            PartialDeclaration::Struct(decl) => write!(f, "\n{}", decl),
            PartialDeclaration::GenericTypeForFunctionScope { type_id } => {
                write!(f, "{}", type_id)
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct PartialFunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) body: Vec<PartialNode>,
    pub(crate) return_type: TypeId,
}

impl CopyTypes for PartialFunctionDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_parameters
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
        self.parameters
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
        self.return_type.copy_types(type_mapping);
    }
}

// impl MonomorphizeHelper for PartialFunctionDeclaration {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn type_parameters(&self) -> &[TypeParameter] {
//         &self.type_parameters
//     }
// }
