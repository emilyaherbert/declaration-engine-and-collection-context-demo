use indent_write::fmt::IndentWriter;
use std::fmt::Write;

use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::literal::Literal,
    type_system::{type_id::TypeId, type_mapping::TypeMapping},
    types::{copy_types::CopyTypes, pretty_print::PrettyPrint},
};

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TypedExpression {
    pub(crate) variant: TypedExpressionVariant,
    pub(crate) type_id: TypeId,
}

impl CopyTypes for TypedExpression {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.variant.copy_types(type_mapping);
        self.type_id.copy_types(type_mapping);
    }
}

impl PrettyPrint for TypedExpression {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        self.variant.pretty_print(declaration_engine)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum TypedExpressionVariant {
    Literal {
        value: Literal,
    },
    Variable {
        name: String,
    },
    FunctionApplication {
        name: String,
        arguments: Vec<TypedExpression>,
    },
    // a no-op variant used to indicate that a variable is in scope
    // as a result of a function parameter
    FunctionParameter,
    Struct {
        struct_name: String,
        fields: Vec<TypedStructExpressionField>,
    },
    MethodCall {
        parent_name: String,
        func_name: String,
        arguments: Vec<TypedExpression>,
    },
}

impl CopyTypes for TypedExpressionVariant {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TypedExpressionVariant::FunctionApplication { arguments, .. } => {
                arguments
                    .iter_mut()
                    .for_each(|argument| argument.copy_types(type_mapping));
            }
            TypedExpressionVariant::Struct { fields, .. } => fields
                .iter_mut()
                .for_each(|field| field.copy_types(type_mapping)),
            TypedExpressionVariant::MethodCall { arguments, .. } => {
                arguments
                    .iter_mut()
                    .for_each(|argument| argument.copy_types(type_mapping));
            }
            TypedExpressionVariant::Literal { .. }
            | TypedExpressionVariant::Variable { .. }
            | TypedExpressionVariant::FunctionParameter => {}
        }
    }
}

impl PrettyPrint for TypedExpressionVariant {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            TypedExpressionVariant::Literal { value } => format!("{}", value),
            TypedExpressionVariant::Variable { name } => name.to_string(),
            TypedExpressionVariant::FunctionApplication { name, arguments } => {
                format!(
                    "{}({})",
                    name,
                    &arguments
                        .iter()
                        .map(|argument| argument.pretty_print(declaration_engine))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            TypedExpressionVariant::FunctionParameter => "function param".to_string(),
            TypedExpressionVariant::Struct {
                struct_name,
                fields,
            } => {
                let mut builder = String::new();
                writeln!(builder, "{} {{", struct_name).unwrap();
                {
                    let mut indent = IndentWriter::new("  ", &mut builder);
                    for field in fields.iter() {
                        writeln!(indent, "{},", field.pretty_print(declaration_engine)).unwrap();
                    }
                }
                write!(builder, "}}").unwrap();
                builder
            }
            TypedExpressionVariant::MethodCall {
                parent_name: parent,
                func_name,
                arguments,
            } => {
                format!(
                    "{}.{}({})",
                    parent,
                    func_name,
                    &arguments
                        .iter()
                        .map(|argument| argument.pretty_print(declaration_engine))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: TypedExpression,
}

impl CopyTypes for TypedStructExpressionField {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.value.copy_types(type_mapping)
    }
}

impl PrettyPrint for TypedStructExpressionField {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!(
            "{}: {}",
            self.name,
            self.value.pretty_print(declaration_engine)
        )
    }
}
