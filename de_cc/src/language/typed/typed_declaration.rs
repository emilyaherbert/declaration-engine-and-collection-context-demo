use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use super::{typed_expression::*, TypedNode};

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

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TypedDeclaration {
    Variable(TypedVariableDeclaration),
    Function(DeclarationId),
    Trait(DeclarationId),
    TraitImpl(DeclarationId),
    GenericTypeForFunctionScope { type_id: TypeId },
    Struct(DeclarationId),
}

impl fmt::Display for TypedDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypedDeclaration::Variable(decl) => write!(f, "{}", decl),
            TypedDeclaration::Function(decl) => write!(f, "\n{}", decl),
            TypedDeclaration::Trait(decl) => write!(f, "\n{}", decl),
            TypedDeclaration::TraitImpl(decl) => write!(f, "\n{}", decl),
            TypedDeclaration::Struct(decl) => write!(f, "\n{}", decl),
            TypedDeclaration::GenericTypeForFunctionScope { type_id } => write!(f, "{}", type_id),
        }
    }
}

impl CopyTypes for TypedDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TypedDeclaration::Variable(decl) => decl.copy_types(type_mapping),
            TypedDeclaration::Function(_)
            | TypedDeclaration::Trait(_)
            | TypedDeclaration::TraitImpl(_)
            | TypedDeclaration::Struct(_)
            | TypedDeclaration::GenericTypeForFunctionScope { .. } => {}
        }
    }
}

impl From<&TypedFunctionParameter> for TypedDeclaration {
    fn from(param: &TypedFunctionParameter) -> Self {
        TypedDeclaration::Variable(TypedVariableDeclaration {
            name: param.name.clone(),
            type_ascription: param.type_id,
            body: TypedExpression {
                variant: TypedExpressionVariant::FunctionParameter,
                type_id: param.type_id,
            },
        })
    }
}

impl TypedDeclaration {
    pub(crate) fn expect_variable(self) -> Result<TypedVariableDeclaration, String> {
        if let TypedDeclaration::Variable(variable_declaration) = self {
            Ok(variable_declaration)
        } else {
            Err("not a variable declaration".to_string())
        }
    }

    pub(crate) fn expect_function(self) -> Result<DeclarationId, String> {
        if let TypedDeclaration::Function(decl_id) = self {
            Ok(decl_id)
        } else {
            Err("not a function declaration".to_string())
        }
    }

    pub(crate) fn expect_trait(self) -> Result<DeclarationId, String> {
        if let TypedDeclaration::Trait(decl_id) = self {
            Ok(decl_id)
        } else {
            Err("not a trait declaration".to_string())
        }
    }

    pub(crate) fn expect_struct(self) -> Result<DeclarationId, String> {
        if let TypedDeclaration::Struct(decl_id) = self {
            Ok(decl_id)
        } else {
            Err("not a struct declaration".to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedVariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: TypeId,
    pub(crate) body: TypedExpression,
}

impl fmt::Display for TypedVariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "let {}: {} = {}",
            self.name, self.type_ascription, self.body
        )
    }
}

impl CopyTypes for TypedVariableDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_ascription.copy_types(type_mapping);
        self.body.copy_types(type_mapping);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedFunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) body: Vec<TypedNode>,
    pub(crate) return_type: TypeId,
}

impl CopyTypes for TypedFunctionDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_parameters
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
        self.parameters
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
        self.return_type.copy_types(type_mapping);
        self.body
            .iter_mut()
            .for_each(|node| node.copy_types(type_mapping));
    }
}

impl fmt::Display for TypedFunctionDeclaration {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "fn {}{}({}) -> {}{} {{",
            self.name,
            if self.type_parameters.is_empty() {
                "".to_string()
            } else {
                format!(
                    "<{}>",
                    self.type_parameters
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
            self.parameters
                .iter()
                .map(|parameter| parameter.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.return_type,
            if self.type_parameters.is_empty() {
                "".to_string()
            } else {
                format!(
                    " where {}",
                    self.type_parameters
                        .iter()
                        .filter(|x| x.trait_constraint.is_some())
                        .map(|x| format!(
                            "{}: {}",
                            x.type_id,
                            x.trait_constraint.clone().unwrap().trait_name
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
        )
        .unwrap();
        {
            let mut indent = IndentWriter::new("  ", &mut f);
            for node in self.body.iter() {
                writeln!(indent, "{};", node).unwrap();
            }
        }
        write!(f, "}}")
    }
}

impl MonomorphizeHelper for TypedFunctionDeclaration {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_parameters(&self) -> &[TypeParameter] {
        &self.type_parameters
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TypedFunctionParameter {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

impl CopyTypes for TypedFunctionParameter {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_id.copy_types(type_mapping);
    }
}

impl fmt::Display for TypedFunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_id)
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TypedTraitDeclaration {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<DeclarationId>,
}

impl CopyTypes for TypedTraitDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.interface_surface
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TypedTraitFn {
    pub(crate) name: String,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) return_type: TypeId,
}

impl CopyTypes for TypedTraitFn {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.return_type.copy_types(type_mapping);
    }
}

impl fmt::Display for TypedTraitFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fn {}({}) -> {}",
            self.name,
            self.parameters
                .iter()
                .map(|parameter| parameter.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.return_type
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedTraitImpl {
    pub(crate) trait_name: String,
    pub(crate) type_implementing_for: TypeId,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) methods: Vec<DeclarationId>,
}

impl CopyTypes for TypedTraitImpl {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.methods
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TypedStructDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) fields: Vec<TypedStructField>,
}

impl CreateTypeId for TypedStructDeclaration {
    fn create_type_id(&self) -> TypeId {
        insert_type(TypeInfo::Struct {
            name: self.name.clone(),
            type_parameters: self.type_parameters.clone(),
            fields: self.fields.clone(),
        })
    }
}

impl CopyTypes for TypedStructDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_parameters
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
        self.fields
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
    }
}

impl MonomorphizeHelper for TypedStructDeclaration {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_parameters(&self) -> &[TypeParameter] {
        &self.type_parameters
    }
}

impl fmt::Display for TypedStructDeclaration {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "struct {}{} {{",
            self.name,
            if self.type_parameters.is_empty() {
                "".to_string()
            } else {
                format!(
                    "<{}>",
                    self.type_parameters
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        )
        .unwrap();
        {
            let mut indent = IndentWriter::new("  ", &mut f);
            for field in self.fields.iter() {
                writeln!(indent, "{},", field).unwrap();
            }
        }
        write!(f, "}}")
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct TypedStructField {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

impl CopyTypes for TypedStructField {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_id.copy_types(type_mapping);
    }
}

impl fmt::Display for TypedStructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.type_id)
    }
}
