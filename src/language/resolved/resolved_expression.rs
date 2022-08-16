use std::fmt;

use crate::{
    language::{literal::Literal, typed::typed_declaration::TypedFunctionDeclaration},
    type_system::type_id::TypeId,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResolvedExpression {
    pub(crate) variant: ResolvedExpressionVariant,
    pub(crate) type_id: TypeId,
}

impl fmt::Display for ResolvedExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ResolvedExpressionVariant {
    Literal {
        value: Literal,
    },
    Variable {
        name: String,
    },
    FunctionApplication {
        name: String,
        function_declaration: TypedFunctionDeclaration,
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
            ResolvedExpressionVariant::FunctionApplication {
                name,
                function_declaration,
                arguments,
            } => todo!(),
            ResolvedExpressionVariant::Struct {
                struct_name,
                fields,
            } => todo!(),
            ResolvedExpressionVariant::Enum {
                enum_name,
                variant_name,
                value,
            } => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResolvedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: ResolvedExpression,
}
