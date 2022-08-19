use std::fmt;

use super::{typed_expression::*, TypedNode};

use crate::{
    declaration_engine::{declaration_engine::DeclarationEngine, declaration_id::DeclarationId},
    type_system::{type_id::TypeId, type_parameter::TypeParameter},
    types::pretty_print::PrettyPrint,
};

#[derive(Clone)]
pub(crate) enum TypedDeclaration {
    Variable(TypedVariableDeclaration),
    Function(DeclarationId),
    // Trait(String),
    // Struct(String),
    // Enum(String),
    // TraitImpl(TypedTraitImpl),
}

impl PrettyPrint for TypedDeclaration {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            TypedDeclaration::Variable(decl) => decl.pretty_print(declaration_engine),
            TypedDeclaration::Function(id) => id.pretty_print(declaration_engine),
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
}

#[derive(Clone)]
pub(crate) struct TypedVariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: TypeId,
    pub(crate) body: TypedExpression,
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

#[derive(Clone)]
pub(crate) struct TypedFunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) body: Vec<TypedNode>,
    pub(crate) return_type: TypeId,
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

#[derive(Clone, PartialEq)]
pub(crate) struct TypedFunctionParameter {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

impl fmt::Display for TypedFunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_id)
    }
}

// #[derive(Clone)]
// pub(crate) struct TypedTraitDeclaration {
//     pub(crate) name: String,
//     pub(crate) interface_surface: Vec<TypedTraitFn>,
//     pub(crate) methods: Vec<FunctionDeclaration>,
// }

// #[derive(Clone)]
// pub(crate) struct TypedTraitFn {
//     pub(crate) name: String,
//     pub(crate) parameters: Vec<TypedFunctionParameter>,
//     pub(crate) return_type: TypeId,
// }

// #[derive(Clone)]
// pub(crate) struct TypedStructDeclaration {
//     pub(crate) name: String,
//     pub(crate) type_parameters: Vec<TypeParameter>,
//     pub(crate) fields: Vec<TypedStructField>,
// }

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TypedStructField {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
}

// #[derive(Clone)]
// pub(crate) struct TypedEnumDeclaration {
//     pub(crate) name: String,
//     pub(crate) type_parameters: Vec<TypeParameter>,
//     pub(crate) variants: Vec<TypedEnumVariant>,
// }

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TypedEnumVariant {
    pub(crate) name: String,
    pub(crate) type_id: TypeId,
    pub(crate) tag: usize,
}

// #[derive(Clone)]
// pub(crate) struct TypedTraitImpl {
//     pub(crate) trait_name: String,
//     pub(crate) type_implementing_for: TypeId,
//     pub(crate) methods: Vec<TypedFunctionDeclaration>,
// }
