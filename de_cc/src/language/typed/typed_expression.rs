use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use crate::{
    language::literal::Literal,
    type_system::{type_id::TypeId, type_mapping::TypeMapping},
    types::copy_types::CopyTypes,
};

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TypedExpression {
    pub(crate) variant: TypedExpressionVariant,
    pub(crate) type_id: TypeId,
}

impl fmt::Display for TypedExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}

impl CopyTypes for TypedExpression {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.variant.copy_types(type_mapping);
        self.type_id.copy_types(type_mapping);
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

impl fmt::Display for TypedExpressionVariant {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypedExpressionVariant::Literal { value } => write!(f, "{}", value),
            TypedExpressionVariant::Variable { name } => write!(f, "{}", name),
            TypedExpressionVariant::FunctionApplication { name, arguments } => {
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
            TypedExpressionVariant::MethodCall {
                parent_name: parent,
                func_name: name,
                arguments,
            } => {
                write!(
                    f,
                    "{}.{}({})",
                    parent,
                    name,
                    &arguments
                        .iter()
                        .map(|argument| argument.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            TypedExpressionVariant::Struct {
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
            TypedExpressionVariant::FunctionParameter => write!(f, "function param"),
        }
    }
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

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: TypedExpression,
}

impl fmt::Display for TypedStructExpressionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl CopyTypes for TypedStructExpressionField {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.value.copy_types(type_mapping)
    }
}
