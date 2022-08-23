use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use super::{typed_expression::*, TypedNode};

use crate::{
    declaration_engine::{declaration_engine::DeclarationEngine, declaration_id::DeclarationId},
    type_system::{
        type_engine::{insert_type, MonomorphizeHelper},
        type_id::TypeId,
        type_info::TypeInfo,
        type_mapping::TypeMapping,
        type_parameter::TypeParameter,
    },
    types::{copy_types::CopyTypes, create_type_id::CreateTypeId, pretty_print::PrettyPrint},
};

#[derive(Clone, Debug)]
pub(crate) enum TypedDeclaration {
    Variable(TypedVariableDeclaration),
    Function(DeclarationId),
    Trait(DeclarationId),
    TraitImpl(DeclarationId),
    GenericTypeForFunctionScope { type_id: TypeId },
    Struct(DeclarationId),
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

impl PrettyPrint for TypedDeclaration {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            TypedDeclaration::Variable(decl) => decl.pretty_print(declaration_engine),
            TypedDeclaration::Function(id) => id.pretty_print(declaration_engine),
            TypedDeclaration::Trait(id) => id.pretty_print(declaration_engine),
            TypedDeclaration::TraitImpl(id) => id.pretty_print(declaration_engine),
            TypedDeclaration::GenericTypeForFunctionScope { .. } => todo!(),
            TypedDeclaration::Struct(id) => id.pretty_print(declaration_engine),
        }
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

#[derive(Clone, Debug)]
pub(crate) struct TypedVariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: TypeId,
    pub(crate) body: TypedExpression,
}

impl CopyTypes for TypedVariableDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_ascription.copy_types(type_mapping);
        self.body.copy_types(type_mapping);
    }
}

impl PrettyPrint for TypedVariableDeclaration {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!(
            "let {}: {} = {}",
            self.name,
            self.type_ascription,
            self.body.pretty_print(declaration_engine)
        )
    }
}

#[derive(Clone, Debug)]
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

impl MonomorphizeHelper for TypedFunctionDeclaration {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_parameters(&self) -> &[TypeParameter] {
        &self.type_parameters
    }
}

impl PrettyPrint for TypedFunctionDeclaration {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        let mut builder = String::new();
        builder.push_str("fn ");
        builder.push_str(&self.name);
        if !self.type_parameters.is_empty() {
            builder.push('<');
            builder.push_str(
                &self
                    .type_parameters
                    .iter()
                    .map(|type_parameter| type_parameter.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            builder.push('>');
        }
        builder.push('(');
        builder.push_str(
            &self
                .parameters
                .iter()
                .map(|parameter| parameter.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        );
        builder.push_str(") -> ");
        builder.push_str(&self.return_type.to_string());
        builder.push_str(" {");
        for line in self.body.iter() {
            builder.push_str("\n  ");
            builder.push_str(&line.pretty_print(declaration_engine));
            builder.push(';');
        }
        builder.push_str("\n{");
        builder
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

#[derive(Clone)]
pub(crate) struct TypedTraitDeclaration {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<DeclarationId>,
}

impl PrettyPrint for TypedTraitDeclaration {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!(
            "trait {} {{\n{}\n}}",
            self.name,
            self.interface_surface
                .iter()
                .map(|trait_fn| trait_fn.pretty_print(declaration_engine))
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

#[derive(Clone)]
pub(crate) struct TypedTraitFn {
    pub(crate) name: String,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) return_type: TypeId,
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

#[derive(Clone, Debug)]
pub(crate) struct TypedTraitImpl {
    pub(crate) trait_name: String,
    pub(crate) type_implementing_for: TypeId,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) methods: Vec<DeclarationId>,
}

impl PrettyPrint for TypedTraitImpl {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!(
            "impl{} {} for {} {{\n{}\n}}",
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
            self.trait_name,
            self.type_implementing_for,
            self.methods
                .iter()
                .map(|method| method.pretty_print(declaration_engine))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Clone)]
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

#[derive(Clone, Hash, PartialEq, Eq)]
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
