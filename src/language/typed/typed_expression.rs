use std::fmt;

use crate::{language::literal::Literal, type_system::type_id::TypeId};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedExpression {
    pub(crate) variant: TypedExpressionVariant,
    pub(crate) type_id: TypeId,
}

impl fmt::Display for TypedExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    Struct {
        struct_name: String,
        fields: Vec<TypedStructExpressionField>,
    },
    Enum {
        enum_name: String,
        variant_name: String,
        value: Box<TypedExpression>,
    },
}

impl fmt::Display for TypedExpressionVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypedExpressionVariant::Literal { value } => write!(f, "{}", value),
            TypedExpressionVariant::Variable { name } => write!(f, "{}", name),
            TypedExpressionVariant::FunctionApplication { name, arguments } => todo!(),
            TypedExpressionVariant::Struct {
                struct_name,
                fields,
            } => todo!(),
            TypedExpressionVariant::Enum {
                enum_name,
                variant_name,
                value,
            } => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: TypedExpression,
}
