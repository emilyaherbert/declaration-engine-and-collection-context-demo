use crate::language::Literal;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expression {
    Literal {
        value: Literal,
    },
    Variable {
        name: String,
    },
    FunctionApplication {
        name: String,
        //type_arguments: Vec<TypeArgument>,
        arguments: Vec<Expression>,
    },
    Struct {
        struct_name: String,
        //type_arguments: Vec<TypeArgument>,
        fields: Vec<StructExpressionField>,
    },
    Enum {
        enum_name: String,
        variant_name: String,
        //type_arguments: Vec<TypeArgument>,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StructExpressionField {
    pub(crate) name: String,
    pub(crate) value: Expression,
}
