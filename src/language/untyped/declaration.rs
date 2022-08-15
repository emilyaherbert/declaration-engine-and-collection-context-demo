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

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: TypeInfo,
    pub(crate) body: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) body: Vec<Node>,
    pub(crate) return_type: TypeInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
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
