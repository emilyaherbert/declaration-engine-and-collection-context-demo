use std::fmt;

use crate::type_system::resolved_types::{ResolvedType, ResolvedTypeParameter};

use super::{resolved_expression::ResolvedExpression, ResolvedNode};

pub(crate) enum ResolvedDeclaration {
    Variable(ResolvedVariableDeclaration),
    Function(ResolvedFunctionDeclaration),
    // Trait(TypedTraitDeclaration),
    // Struct(TypedStructDeclaration),
    // Enum(TypedEnumDeclaration),
    // ImplTrait(TypedTraitImpl),
}

impl fmt::Display for ResolvedDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedDeclaration::Variable(decl) => write!(f, "{}", decl),
            ResolvedDeclaration::Function(decl) => write!(f, "{}", decl),
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            builder.push_str(&line.to_string());
            builder.push(';');
        }
        builder.push_str("\n{");
        write!(f, "{}", builder)
    }
}

pub(crate) struct ResolvedFunctionParameter {
    pub(crate) name: String,
    pub(crate) type_info: ResolvedType,
}

impl fmt::Display for ResolvedFunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_info)
    }
}
