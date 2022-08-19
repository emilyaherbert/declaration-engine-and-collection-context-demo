use std::fmt;

use crate::type_system::resolved_types::ResolvedType;

use super::{resolved_expression::ResolvedExpression, ResolvedNode};

pub(crate) enum ResolvedDeclaration {
    Variable(ResolvedVariableDeclaration),
    Function(ResolvedFunctionDeclaration),
    Trait(ResolvedTraitDeclaration),
    // Struct(TypedStructDeclaration),
    // Enum(TypedEnumDeclaration),
    // ImplTrait(TypedTraitImpl),
}

impl fmt::Display for ResolvedDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedDeclaration::Variable(decl) => write!(f, "{}", decl),
            ResolvedDeclaration::Function(decl) => write!(f, "{}", decl),
            ResolvedDeclaration::Trait(decl) => write!(f, "{}", decl),
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
    pub(crate) parameters: Vec<ResolvedFunctionParameter>,
    pub(crate) body: Vec<ResolvedNode>,
    pub(crate) return_type: ResolvedType,
}

impl fmt::Display for ResolvedFunctionDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();
        builder.push_str("fn ");
        builder.push_str(&self.name);
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
            builder.push_str(&line.to_string());
            builder.push(';');
        }
        builder.push_str("\n{");
        write!(f, "{}", builder)
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "trait {} {{\n  {};\n}}",
            self.name,
            self.interface_surface
                .iter()
                .map(|trait_fn| trait_fn.to_string())
                .collect::<Vec<_>>()
                .join(";\n  "),
        )
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
