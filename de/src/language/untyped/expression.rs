use indent_write::fmt::IndentWriter;
use std::fmt;
use std::fmt::Write;

use crate::{language::literal::Literal, type_system::type_argument::TypeArgument};

#[derive(Clone)]
pub enum Expression {
    Literal {
        value: Literal,
    },
    Variable {
        name: String,
    },
    FunctionApplication {
        name: String,
        type_arguments: Vec<TypeArgument>,
        arguments: Vec<Expression>,
    },
    MethodCall {
        parent: String,
        name: String,
        type_arguments: Vec<TypeArgument>,
        arguments: Vec<Expression>,
    },
    Struct {
        struct_name: String,
        type_arguments: Vec<TypeArgument>,
        fields: Vec<StructExpressionField>,
    },
    // Enum {
    //     enum_name: String,
    //     variant_name: String,
    //     type_arguments: Vec<TypeArgument>,
    //     value: Box<Expression>,
    // },
}

impl fmt::Display for Expression {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal { value } => write!(f, "{}", value),
            Expression::Variable { name } => write!(f, "{}", name),
            Expression::FunctionApplication {
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
            Expression::MethodCall {
                parent,
                name,
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
            Expression::Struct {
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
        }
    }
}

#[derive(Clone)]
pub struct StructExpressionField {
    pub(crate) name: String,
    pub(crate) value: Expression,
}

impl fmt::Display for StructExpressionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

pub mod constructors {
    use std::u8;

    use crate::{language::literal::Literal, type_system::type_argument::TypeArgument};

    use super::{Expression, StructExpressionField};

    pub fn u8(value: u8) -> Expression {
        Expression::Literal {
            value: Literal::U8(value),
        }
    }

    pub fn u16(value: u16) -> Expression {
        Expression::Literal {
            value: Literal::U16(value),
        }
    }

    pub fn u32(value: u32) -> Expression {
        Expression::Literal {
            value: Literal::U32(value),
        }
    }

    pub fn u64(value: u64) -> Expression {
        Expression::Literal {
            value: Literal::U64(value),
        }
    }

    pub fn var(name: &str) -> Expression {
        Expression::Variable {
            name: name.to_string(),
        }
    }

    pub fn func_app(
        name: &str,
        type_arguments: &[TypeArgument],
        arguments: &[Expression],
    ) -> Expression {
        Expression::FunctionApplication {
            name: name.to_string(),
            type_arguments: type_arguments.to_vec(),
            arguments: arguments.to_vec(),
        }
    }

    pub fn struct_exp(
        struct_name: &str,
        type_arguments: &[TypeArgument],
        fields: &[StructExpressionField],
    ) -> Expression {
        Expression::Struct {
            struct_name: struct_name.to_string(),
            type_arguments: type_arguments.to_vec(),
            fields: fields.to_vec(),
        }
    }

    pub fn struct_exp_field(name: &str, value: Expression) -> StructExpressionField {
        StructExpressionField {
            name: name.to_string(),
            value,
        }
    }
}