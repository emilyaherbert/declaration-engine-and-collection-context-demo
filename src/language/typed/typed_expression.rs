use crate::type_system::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedExpression {
    variant: TypedExpressionVariant,
    type_id: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypedExpressionVariant {
    Literal {
        value: Literal,
    },
    VariableExpression {
        name: String,
    },
    FunctionApplication {
        name: String,
        type_arguments: Vec<TypeArgument>,
        arguments: Vec<TypedExpression>,
    },
    StructExpression {
        struct_name: String,
        type_arguments: Vec<TypeArgument>,
        fields: Vec<TypedStructExpressionField>,
    },
    EnumExpression {
        enum_name: String,
        variant_name: String,
        type_arguments: Vec<TypeArgument>,
        value: Box<TypedExpression>,
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
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: TypedExpression,
}
