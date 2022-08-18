use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    type_system::{type_info::TypeInfo, type_parameter::TypeParameter},
    types::pretty_print::PrettyPrint,
};

use super::{expression::*, Node};

#[derive(Clone)]
pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
    // Trait(TraitDeclaration),
    // Struct(StructDeclaration),
    // Enum(EnumDeclaration),
    // TraitImpl(TraitImpl),
    // SelfImpl(SelfImpl),
}

impl PrettyPrint for Declaration {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            Declaration::Variable(decl) => format!("{}", decl.pretty_print(declaration_engine)),
            Declaration::Function(decl) => format!("{}", decl.pretty_print(declaration_engine)),
        }
    }
}

#[derive(Clone)]
pub struct VariableDeclaration {
    pub(crate) name: String,
    pub(crate) type_ascription: TypeInfo,
    pub(crate) body: Expression,
}

impl PrettyPrint for VariableDeclaration {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!(
            "let {}: {} = {}",
            self.name,
            self.type_ascription.pretty_print(declaration_engine),
            self.body.pretty_print(declaration_engine)
        )
    }
}

#[derive(Clone)]
pub struct FunctionDeclaration {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) body: Vec<Node>,
    pub(crate) return_type: TypeInfo,
}

impl PrettyPrint for FunctionDeclaration {
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
                    .map(|type_parameter| type_parameter.pretty_print(declaration_engine))
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
                .map(|parameter| parameter.pretty_print(declaration_engine))
                .collect::<Vec<_>>()
                .join(", "),
        );
        builder.push_str(") -> ");
        builder.push_str(&self.return_type.pretty_print(declaration_engine));
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

#[derive(Clone, Hash)]
pub struct FunctionParameter {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
}

impl PrettyPrint for FunctionParameter {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!(
            "{}: {}",
            self.name,
            self.type_info.pretty_print(declaration_engine)
        )
    }
}

// #[derive(Clone)]
// pub struct TraitDeclaration {
//     pub(crate) name: String,
//     pub(crate) interface_surface: Vec<TraitFn>,
//     pub(crate) methods: Vec<FunctionDeclaration>,
// }

// #[derive(Clone)]
// pub struct TraitFn {
//     pub(crate) name: String,
//     pub(crate) parameters: Vec<FunctionParameter>,
//     pub(crate) return_type: TypeInfo,
// }

// #[derive(Clone)]
// pub struct StructDeclaration {
//     pub(crate) name: String,
//     pub(crate) type_parameters: Vec<TypeParameter>,
//     pub(crate) fields: Vec<StructField>,
// }

#[derive(Clone, Hash)]
pub struct StructField {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
}

// #[derive(Clone)]
// pub struct EnumDeclaration {
//     pub(crate) name: String,
//     pub(crate) type_parameters: Vec<TypeParameter>,
//     pub(crate) variants: Vec<EnumVariant>,
// }

#[derive(Clone, Hash)]
pub struct EnumVariant {
    pub(crate) name: String,
    pub(crate) type_info: TypeInfo,
    pub(crate) tag: usize,
}

// #[derive(Clone)]
// pub struct TraitImpl {
//     pub(crate) trait_name: String,
//     pub(crate) type_implementing_for: TypeInfo,
//     pub(crate) type_parameters: Vec<TypeParameter>,
//     pub(crate) functions: Vec<FunctionDeclaration>,
// }

// #[derive(Clone)]
// pub struct SelfImpl {
//     pub(crate) type_implementing_for: TypeInfo,
//     pub(crate) type_parameters: Vec<TypeParameter>,
//     pub(crate) functions: Vec<FunctionDeclaration>,
// }

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
