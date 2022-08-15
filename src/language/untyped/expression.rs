use crate::{language::literal::Literal, type_system::type_argument::TypeArgument};

#[derive(Debug, Clone, PartialEq)]
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
    Struct {
        struct_name: String,
        type_arguments: Vec<TypeArgument>,
        fields: Vec<StructExpressionField>,
    },
    Enum {
        enum_name: String,
        variant_name: String,
        type_arguments: Vec<TypeArgument>,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructExpressionField {
    pub(crate) name: String,
    pub(crate) value: Expression,
}

pub mod constructors {
    use std::u8;

    use crate::language::literal::Literal;

    use super::Expression;

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
}
