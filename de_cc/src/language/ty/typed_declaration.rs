use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use super::{typed_expression::*, TyNode};

use crate::{
    collection_context::{collection_context::CollectionContext, collection_index::CCIdx},
    declaration_engine::declaration_id::DeclarationId,
    type_system::{
        type_engine::{insert_type, MonomorphizeHelper},
        type_id::TypeId,
        type_info::TypeInfo,
        type_mapping::TypeMapping,
        type_parameter::TypeParameter,
    },
    types::{copy_types::CopyTypes, create_type_id::CreateTypeId, pretty_print::PrettyPrint},
};

#[derive(Clone, PartialEq)]
pub(crate) enum TyDeclaration {
    Variable(TyVariableDeclaration),
    Function(CCIdx<DeclarationId>),
    Trait(CCIdx<DeclarationId>),
    TraitImpl(CCIdx<DeclarationId>),
    Struct(CCIdx<DeclarationId>),
}

impl fmt::Display for TyDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyDeclaration::Variable(decl) => write!(f, "{}", decl),
            TyDeclaration::Function(decl_id) => write!(f, "\n{}", decl_id),
            TyDeclaration::Trait(decl_id) => write!(f, "\n{}", decl_id),
            TyDeclaration::TraitImpl(decl_id) => write!(f, "\n{}", decl_id),
            TyDeclaration::Struct(decl_id) => write!(f, "\n{}", decl_id),
        }
    }
}

impl fmt::Debug for TyDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyDeclaration::Variable(decl) => write!(f, "{:?}", decl),
            TyDeclaration::Function(decl_id) => write!(f, "\n{:?}", decl_id),
            TyDeclaration::Trait(decl_id) => write!(f, "\n{:?}", decl_id),
            TyDeclaration::TraitImpl(decl_id) => write!(f, "\n{:?}", decl_id),
            TyDeclaration::Struct(decl_id) => write!(f, "\n{:?}", decl_id),
        }
    }
}

impl CopyTypes for TyDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TyDeclaration::Variable(decl) => decl.copy_types(type_mapping),
            TyDeclaration::Function(decl_id) => decl_id.copy_types(type_mapping),
            TyDeclaration::Trait(decl_id) => decl_id.copy_types(type_mapping),
            TyDeclaration::TraitImpl(decl_id) => decl_id.copy_types(type_mapping),
            TyDeclaration::Struct(decl_id) => decl_id.copy_types(type_mapping),
        }
    }
}

impl From<&TyFunctionParameter> for TyDeclaration {
    fn from(param: &TyFunctionParameter) -> Self {
        TyDeclaration::Variable(TyVariableDeclaration {
            name: param.name.clone(),
            type_ascription: param.type_id,
            body: TyExpression {
                variant: TyExpressionVariant::FunctionParameter,
                type_id: param.type_id,
            },
        })
    }
}

impl TyDeclaration {
    pub(crate) fn expect_variable(self) -> Result<TyVariableDeclaration, String> {
        if let TyDeclaration::Variable(variable_declaration) = self {
            Ok(variable_declaration)
        } else {
            Err("not a variable declaration".to_string())
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TyVariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: TypeId,
    pub(crate) body: TyExpression,
}

impl fmt::Display for TyVariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "let {}: {} = {}",
            self.name, self.type_ascription, self.body
        )
    }
}

impl fmt::Debug for TyVariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "let {}: {:?} = {:?}",
            self.name, self.type_ascription, self.body
        )
    }
}

impl CopyTypes for TyVariableDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_ascription.copy_types(type_mapping);
        self.body.copy_types(type_mapping);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TyFunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<TyFunctionParameter>,
    pub(crate) body: Vec<CCIdx<TyNode>>,
    pub(crate) return_type: TypeId,
}

impl CopyTypes for TyFunctionDeclaration {
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

impl fmt::Display for TyFunctionDeclaration {
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

impl MonomorphizeHelper for TyFunctionDeclaration {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_parameters(&self) -> &[TypeParameter] {
        &self.type_parameters
    }

    fn type_parameters_mut(&mut self) -> &mut [TypeParameter] {
        &mut self.type_parameters
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TyFunctionParameter {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

impl CopyTypes for TyFunctionParameter {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_id.copy_types(type_mapping);
    }
}

impl fmt::Display for TyFunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_id)
    }
}

impl fmt::Debug for TyFunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.name, self.type_id)
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TyTraitDeclaration {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<CCIdx<DeclarationId>>,
}

impl CopyTypes for TyTraitDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.interface_surface
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TyTraitFn {
    pub(crate) name: String,
    pub(crate) parameters: Vec<TyFunctionParameter>,
    pub(crate) return_type: TypeId,
}

impl CopyTypes for TyTraitFn {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.return_type.copy_types(type_mapping);
    }
}

impl fmt::Display for TyTraitFn {
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

#[derive(Clone, PartialEq)]
pub(crate) struct TyTraitImpl {
    pub(crate) trait_name: String,
    pub(crate) type_implementing_for: TypeId,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) methods: Vec<CCIdx<DeclarationId>>,
}

impl CopyTypes for TyTraitImpl {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.methods
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TyStructDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) fields: Vec<TyStructField>,
}

impl CreateTypeId for TyStructDeclaration {
    fn create_type_id(&self) -> TypeId {
        insert_type(TypeInfo::Struct {
            name: self.name.clone(),
            type_parameters: self.type_parameters.clone(),
            fields: self.fields.clone(),
        })
    }
}

impl CopyTypes for TyStructDeclaration {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_parameters
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
        self.fields
            .iter_mut()
            .for_each(|x| x.copy_types(type_mapping));
    }
}

impl MonomorphizeHelper for TyStructDeclaration {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_parameters(&self) -> &[TypeParameter] {
        &self.type_parameters
    }

    fn type_parameters_mut(&mut self) -> &mut [TypeParameter] {
        &mut self.type_parameters
    }
}

impl PrettyPrint for TyStructDeclaration {
    fn pretty_print(&self, _cc: &CollectionContext) -> String {
        self.to_string()
    }

    fn pretty_print_debug(&self, cc: &CollectionContext) -> String {
        self.pretty_print(cc)
    }
}

impl fmt::Debug for TyStructDeclaration {
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
                        .map(|x| format!("{:?}", x))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        )
        .unwrap();
        {
            let mut indent = IndentWriter::new("  ", &mut f);
            for field in self.fields.iter() {
                writeln!(indent, "{:?},", field).unwrap();
            }
        }
        write!(f, "}}")
    }
}

impl fmt::Display for TyStructDeclaration {
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
pub struct TyStructField {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

impl CopyTypes for TyStructField {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_id.copy_types(type_mapping);
    }
}

impl fmt::Debug for TyStructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:?})", self.name, self.type_id)
    }
}

impl fmt::Display for TyStructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.type_id)
    }
}
