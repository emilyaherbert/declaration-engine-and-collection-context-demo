use crate::{language::Literal, type_system::TypeId};

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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: TypedExpression,
}
