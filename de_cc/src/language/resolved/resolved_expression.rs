use std::fmt;

use crate::{language::literal::Literal, type_system::resolved_types::ResolvedType};

pub(crate) struct ResolvedExpression {
    pub(crate) variant: ResolvedExpressionVariant,
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
    Enum {
        enum_name: String,
        variant_name: String,
        value: Box<ResolvedExpression>,
    },
}

impl fmt::Display for ResolvedExpressionVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            ResolvedExpressionVariant::Struct { .. } => todo!(),
            ResolvedExpressionVariant::Enum { .. } => todo!(),
        }
    }
}

pub(crate) struct ResolvedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: ResolvedExpression,
}
