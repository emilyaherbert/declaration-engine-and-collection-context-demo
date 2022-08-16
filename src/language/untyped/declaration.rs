use std::fmt;

use crate::type_system::{type_info::TypeInfo, type_parameter::TypeParameter};

use super::{expression::*, Node};

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
    Trait(TraitDeclaration),
    Struct(StructDeclaration),
    Enum(EnumDeclaration),
    TraitImpl(TraitImpl),
    SelfImpl(SelfImpl),
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Declaration::Variable(decl) => write!(f, "{}", decl),
            Declaration::Function(decl) => write!(f, "{}", decl),
            Declaration::Trait(_) => todo!(),
            Declaration::Struct(_) => todo!(),
            Declaration::Enum(_) => todo!(),
            Declaration::TraitImpl(_) => todo!(),
            Declaration::SelfImpl(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: TypeInfo,
    pub(crate) body: Expression,
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {}: {} = {}",
            self.name, self.type_ascription, self.body
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) body: Vec<Node>,
    pub(crate) return_type: TypeInfo,
}

impl fmt::Display for FunctionDeclaration {
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

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
}

impl fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_info)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDeclaration {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<TraitFn>,
    pub(crate) methods: Vec<FunctionDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitFn {
    pub(crate) name: String,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) return_type: TypeInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
    pub(crate) tag: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitImpl {
    pub(crate) trait_name: String,
    pub(crate) type_implementing_for: TypeInfo,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) functions: Vec<FunctionDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelfImpl {
    pub(crate) type_implementing_for: TypeInfo,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) functions: Vec<FunctionDeclaration>,
}

pub mod constructors {
    use crate::{
        language::untyped::{Expression, Node},
        type_system::{type_info::TypeInfo, type_parameter::TypeParameter},
    };

    use super::{Declaration, FunctionDeclaration, FunctionParameter, VariableDeclaration};

    pub fn var_decl(name: &str, type_ascription: Option<TypeInfo>, body: Expression) -> Node {
        Node::Declaration(Declaration::Variable(VariableDeclaration {
            name: name.to_string(),
            type_ascription: type_ascription.unwrap_or_default(),
            body,
        }))
    }

    pub fn func_decl(
        name: &str,
        type_parameters: &[TypeParameter],
        parameters: &[FunctionParameter],
        body: &[Node],
        return_type: TypeInfo,
    ) -> Node {
        Node::Declaration(Declaration::Function(FunctionDeclaration {
            name: name.to_string(),
            type_parameters: type_parameters.to_vec(),
            parameters: parameters.to_vec(),
            body: body.to_vec(),
            return_type,
        }))
    }
}
