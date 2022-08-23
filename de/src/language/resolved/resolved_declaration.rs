use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use crate::type_system::resolved_types::{ResolvedType, ResolvedTypeParameter};

use super::{resolved_expression::ResolvedExpression, ResolvedNode};

pub(crate) enum ResolvedDeclaration {
    Variable(ResolvedVariableDeclaration),
    Function(ResolvedFunctionDeclaration),
    Trait(ResolvedTraitDeclaration),
    TraitImpl(ResolvedTraitImpl),
    Struct(ResolvedStructDeclaration),
}

impl fmt::Display for ResolvedDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedDeclaration::Variable(decl) => write!(f, "{}", decl),
            ResolvedDeclaration::Function(decl) => write!(f, "\n{}", decl),
            ResolvedDeclaration::Trait(decl) => write!(f, "\n{}", decl),
            ResolvedDeclaration::TraitImpl(decl) => write!(f, "\n{}", decl),
            ResolvedDeclaration::Struct(decl) => write!(f, "\n{}", decl),
        }
    }
}

pub(crate) struct ResolvedVariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: ResolvedType,
    pub(crate) body: ResolvedExpression,
}

impl fmt::Display for ResolvedVariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {}: {} = {}",
            self.name, self.type_ascription, self.body
        )
    }
}

pub(crate) struct ResolvedFunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<ResolvedTypeParameter>,
    pub(crate) parameters: Vec<ResolvedFunctionParameter>,
    pub(crate) body: Vec<ResolvedNode>,
    pub(crate) return_type: ResolvedType,
}

impl fmt::Display for ResolvedFunctionDeclaration {
    fn fmt(&self, mut f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "fn {}{}({}) -> {} {{",
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

#[derive(Clone)]
pub(crate) struct ResolvedFunctionParameter {
    pub(crate) name: String,
    pub(crate) type_info: ResolvedType,
}

impl fmt::Display for ResolvedFunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_info)
    }
}

#[derive(Clone)]
pub(crate) struct ResolvedTraitDeclaration {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<ResolvedTraitFn>,
}

impl fmt::Display for ResolvedTraitDeclaration {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "trait {} {{", self.name).unwrap();
        {
            let mut indent = IndentWriter::new("  ", &mut f);
            for trait_fn in self.interface_surface.iter() {
                writeln!(indent, "{};", trait_fn).unwrap();
            }
        }
        write!(f, "}}")
    }
}

#[derive(Clone)]
pub(crate) struct ResolvedTraitFn {
    pub(crate) name: String,
    pub(crate) parameters: Vec<ResolvedFunctionParameter>,
    pub(crate) return_type: ResolvedType,
}

impl fmt::Display for ResolvedTraitFn {
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

pub(crate) struct ResolvedTraitImpl {
    pub(crate) trait_name: String,
    pub(crate) type_implementing_for: ResolvedType,
    pub(crate) methods: Vec<ResolvedFunctionDeclaration>,
}

impl fmt::Display for ResolvedTraitImpl {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "impl {} for {} {{",
            self.trait_name, self.type_implementing_for
        )
        .unwrap();
        {
            let mut indent = IndentWriter::new("  ", &mut f);
            for method in self.methods.iter() {
                writeln!(indent, "{}", method).unwrap();
            }
        }
        write!(f, "}}")
    }
}

pub(crate) struct ResolvedStructDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<ResolvedTypeParameter>,
    pub(crate) fields: Vec<ResolvedStructField>,
}

impl fmt::Display for ResolvedStructDeclaration {
    fn fmt(&self, mut f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Clone)]
pub(crate) struct ResolvedStructField {
    pub(crate) name: String,
    pub(crate) type_info: ResolvedType,
}

impl fmt::Display for ResolvedStructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.type_info)
    }
}
