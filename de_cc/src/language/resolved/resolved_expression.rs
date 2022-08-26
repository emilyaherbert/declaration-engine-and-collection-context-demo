use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use crate::{language::literal::Literal, type_system::resolved_types::ResolvedType};

pub(crate) struct ResolvedExpression {
    pub(crate) variant: ResolvedExpressionVariant,
    #[allow(dead_code)]
    pub(crate) type_info: ResolvedType,
}

impl fmt::Display for ResolvedExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}

pub(crate) enum ResolvedExpressionVariant {
    Literal {
        value: Literal,
    },
    Variable {
        name: String,
    },
    FunctionApplication {
        name: String,
        arguments: Vec<ResolvedExpression>,
    },
    Struct {
        struct_name: String,
        fields: Vec<ResolvedStructExpressionField>,
    },
    MethodCall {
        parent_name: String,
        func_name: String,
        arguments: Vec<ResolvedExpression>,
    },
}

impl fmt::Display for ResolvedExpressionVariant {
    fn fmt(&self, mut f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedExpressionVariant::Literal { value } => write!(f, "{}", value),
            ResolvedExpressionVariant::Variable { name } => write!(f, "{}", name),
            ResolvedExpressionVariant::FunctionApplication { name, arguments } => {
                write!(
                    f,
                    "{}({})",
                    name,
                    &arguments
                        .iter()
                        .map(|argument| argument.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            ResolvedExpressionVariant::Struct {
                struct_name,
                fields,
            } => {
                writeln!(f, "{} {{", struct_name,).unwrap();
                {
                    let mut indent = IndentWriter::new("  ", &mut f);
                    for field in fields.iter() {
                        writeln!(indent, "{},", field).unwrap();
                    }
                }
                write!(f, "}}")
            }
            ResolvedExpressionVariant::MethodCall {
                parent_name,
                func_name,
                arguments,
            } => {
                write!(
                    f,
                    "{}.{}({})",
                    parent_name,
                    func_name,
                    &arguments
                        .iter()
                        .map(|argument| argument.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

pub(crate) struct ResolvedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: ResolvedExpression,
}

impl fmt::Display for ResolvedStructExpressionField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
