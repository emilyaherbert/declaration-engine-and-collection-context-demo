use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use crate::{
    language::literal::Literal,
    type_system::{type_argument::TypeArgument, type_id::TypeId, type_mapping::TypeMapping},
    types::{copy_types::CopyTypes, with_collection_context::*},
    CollectionContext,
};

#[derive(Clone, PartialEq, Debug, DebugWithCC)]
pub(crate) struct TyExpression {
    pub(crate) variant: TyExpressionVariant,
    pub(crate) type_id: TypeId,
}

impl fmt::Display for TyExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}

impl CopyTypes for TyExpression {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.variant.copy_types(type_mapping);
        self.type_id.copy_types(type_mapping);
    }
}

#[derive(Clone, PartialEq, Debug, DebugWithCC)]
pub(crate) enum TyExpressionVariant {
    Literal {
        value: Literal,
    },
    Variable {
        name: String,
    },
    FunctionApplication {
        name: String,
        type_arguments: Vec<TypeArgument>,
        arguments: Vec<TyExpression>,
    },
    // a no-op variant used to indicate that a variable is in scope
    // as a result of a function parameter
    FunctionParameter,
    Struct {
        struct_name: String,
        type_arguments: Vec<TypeArgument>,
        fields: Vec<TyStructExpressionField>,
    },
    MethodCall {
        parent_name: String,
        func_name: String,
        type_arguments: Vec<TypeArgument>,
        arguments: Vec<TyExpression>,
    },
}

impl fmt::Display for TyExpressionVariant {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyExpressionVariant::Literal { value } => write!(f, "{}", value),
            TyExpressionVariant::Variable { name } => write!(f, "{}", name),
            TyExpressionVariant::FunctionApplication {
                name,
                type_arguments,
                arguments,
            } => {
                write!(
                    f,
                    "{}{}({})",
                    name,
                    if type_arguments.is_empty() {
                        "".to_string()
                    } else {
                        format!(
                            "::<{}>",
                            type_arguments
                                .iter()
                                .map(|type_argument| type_argument.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    },
                    &arguments
                        .iter()
                        .map(|argument| argument.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            TyExpressionVariant::MethodCall {
                parent_name: parent,
                func_name: name,
                type_arguments,
                arguments,
            } => {
                write!(
                    f,
                    "{}.{}{}({})",
                    parent,
                    name,
                    if type_arguments.is_empty() {
                        "".to_string()
                    } else {
                        format!(
                            "::<{}>",
                            type_arguments
                                .iter()
                                .map(|type_argument| type_argument.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    },
                    &arguments
                        .iter()
                        .map(|argument| argument.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            TyExpressionVariant::Struct {
                struct_name,
                type_arguments,
                fields,
            } => {
                writeln!(
                    f,
                    "{}{} {{",
                    struct_name,
                    if type_arguments.is_empty() {
                        "".to_string()
                    } else {
                        format!(
                            "::<{}>",
                            type_arguments
                                .iter()
                                .map(|type_argument| type_argument.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    }
                )
                .unwrap();
                {
                    let mut indent = IndentWriter::new("  ", &mut f);
                    for field in fields.iter() {
                        writeln!(indent, "{},", field).unwrap();
                    }
                }
                write!(f, "}}")
            }
            TyExpressionVariant::FunctionParameter => write!(f, "function param"),
        }
    }
}

impl CopyTypes for TyExpressionVariant {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TyExpressionVariant::FunctionApplication { arguments, .. } => {
                arguments
                    .iter_mut()
                    .for_each(|argument| argument.copy_types(type_mapping));
            }
            TyExpressionVariant::Struct { fields, .. } => fields
                .iter_mut()
                .for_each(|field| field.copy_types(type_mapping)),
            TyExpressionVariant::MethodCall { arguments, .. } => {
                arguments
                    .iter_mut()
                    .for_each(|argument| argument.copy_types(type_mapping));
            }
            TyExpressionVariant::Literal { .. }
            | TyExpressionVariant::Variable { .. }
            | TyExpressionVariant::FunctionParameter => {}
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TyStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: TyExpression,
}

impl fmt::Display for TyStructExpressionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl CopyTypes for TyStructExpressionField {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.value.copy_types(type_mapping)
    }
}
