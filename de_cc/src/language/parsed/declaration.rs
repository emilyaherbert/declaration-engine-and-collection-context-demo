use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use crate::type_system::{type_info::TypeInfo, type_parameter::TypeParameter};

use super::{expression::*, Node};

#[derive(Clone, PartialEq)]
pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
    Trait(TraitDeclaration),
    TraitImpl(TraitImpl),
    Struct(StructDeclaration),
    // SelfImpl(SelfImpl),
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Declaration::Variable(decl) => write!(f, "{}", decl),
            Declaration::Function(decl) => write!(f, "\n{}", decl),
            Declaration::Trait(decl) => write!(f, "\n{}", decl),
            Declaration::TraitImpl(decl) => write!(f, "\n{}", decl),
            Declaration::Struct(decl) => write!(f, "\n{}", decl),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct VariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: TypeInfo,
    pub(crate) body: Expression,
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "let {}: {} = {}",
            self.name, self.type_ascription, self.body
        )
    }
}

#[derive(Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) body: Vec<Node>,
    pub(crate) return_type: TypeInfo,
}

impl fmt::Display for FunctionDeclaration {
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

#[derive(Clone, Hash, PartialEq)]
pub struct FunctionParameter {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
}

impl fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_info)
    }
}

#[derive(Clone, PartialEq)]
pub struct TraitDeclaration {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<TraitFn>,
}

impl fmt::Display for TraitDeclaration {
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

#[derive(Clone, PartialEq)]
pub struct TraitFn {
    pub(crate) name: String,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) return_type: TypeInfo,
}

impl fmt::Display for TraitFn {
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
pub struct TraitImpl {
    pub(crate) trait_name: String,
    pub(crate) type_implementing_for: TypeInfo,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) methods: Vec<FunctionDeclaration>,
}

impl fmt::Display for TraitImpl {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "impl{} {} for {} {{",
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
            self.type_implementing_for
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

#[derive(Clone, PartialEq, Debug)]
pub struct StructDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) fields: Vec<StructField>,
}

impl fmt::Display for StructDeclaration {
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

#[derive(Clone, Hash, PartialEq, Debug)]
pub struct StructField {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
}

impl fmt::Display for StructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.type_info)
    }
}

#[derive(Clone, Hash, Debug)]
pub struct EnumVariant {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
    pub(crate) tag: usize,
}

// #[derive(Clone)]
// pub struct SelfImpl {
//     pub(crate) type_implementing_for: TypeInfo,
//     pub(crate) type_parameters: Vec<TypeParameter>,
//     pub(crate) functions: Vec<FunctionDeclaration>,
// }

pub mod constructors {
    use crate::{
        language::parsed::{Expression, Node},
        type_system::{
            trait_constraint::TraitConstraint, type_engine::insert_type, type_info::TypeInfo,
            type_parameter::TypeParameter,
        },
    };

    use super::{
        Declaration, FunctionDeclaration, FunctionParameter, StructDeclaration, StructField,
        TraitDeclaration, TraitFn, TraitImpl, VariableDeclaration,
    };

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

    pub fn func_decl_raw(
        name: &str,
        type_parameters: &[TypeParameter],
        parameters: &[FunctionParameter],
        body: &[Node],
        return_type: TypeInfo,
    ) -> FunctionDeclaration {
        FunctionDeclaration {
            name: name.to_string(),
            type_parameters: type_parameters.to_vec(),
            parameters: parameters.to_vec(),
            body: body.to_vec(),
            return_type,
        }
    }

    pub fn func_param(name: &str, type_info: TypeInfo) -> FunctionParameter {
        FunctionParameter {
            name: name.to_string(),
            type_info,
        }
    }

    pub fn type_param(name: &str, trait_constraint: Option<&str>) -> TypeParameter {
        TypeParameter {
            name: name.to_string(),
            type_id: insert_type(TypeInfo::UnknownGeneric {
                name: name.to_string(),
            }),
            trait_constraint: trait_constraint.map(|x| TraitConstraint {
                trait_name: x.to_string(),
            }),
        }
    }

    pub fn trait_(name: &str, interface_surface: &[TraitFn]) -> Node {
        Node::Declaration(Declaration::Trait(TraitDeclaration {
            name: name.to_string(),
            interface_surface: interface_surface.to_vec(),
        }))
    }

    pub fn trait_fn(
        name: &str,
        parameters: &[FunctionParameter],
        return_type: TypeInfo,
    ) -> TraitFn {
        TraitFn {
            name: name.to_string(),
            parameters: parameters.to_vec(),
            return_type,
        }
    }

    pub fn trait_impl(
        trait_name: &str,
        type_implementing_for: TypeInfo,
        type_parameters: &[TypeParameter],
        methods: &[FunctionDeclaration],
    ) -> Node {
        Node::Declaration(Declaration::TraitImpl(TraitImpl {
            trait_name: trait_name.to_string(),
            type_implementing_for,
            type_parameters: type_parameters.to_vec(),
            methods: methods.to_vec(),
        }))
    }

    pub fn struct_(name: &str, type_parameters: &[TypeParameter], fields: &[StructField]) -> Node {
        Node::Declaration(Declaration::Struct(StructDeclaration {
            name: name.to_string(),
            type_parameters: type_parameters.to_vec(),
            fields: fields.to_vec(),
        }))
    }

    pub fn struct_field(name: &str, type_info: TypeInfo) -> StructField {
        StructField {
            name: name.to_string(),
            type_info,
        }
    }
}
