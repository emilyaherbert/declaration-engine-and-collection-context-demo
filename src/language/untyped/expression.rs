use crate::type_system::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expression {
    Literal {
        value: Literal,
    },
    VariableExpression {
        name: String,
    },
    FunctionApplication {
        name: String,
        type_arguments: Vec<TypeArgument>,
        arguments: Vec<Expression>,
    },
    StructExpression {
        struct_name: String,
        type_arguments: Vec<TypeArgument>,
        fields: Vec<StructExpressionField>,
    },
    EnumExpression {
        enum_name: String,
        variant_name: String,
        type_arguments: Vec<TypeArgument>,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Literal {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StructExpressionField {
    pub(crate) name: String,
    pub(crate) value: Expression,
}
