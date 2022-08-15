//use crate::type_system::*;

// #[derive(Debug, Clone, PartialEq)]
// pub(crate) struct ResolvedExpression {
//     variant: ResolvedExpressionVariant,
//     type_id: TypeId,
// }

use crate::language::{literal::Literal, typed::typed_declaration::TypedFunctionDeclaration};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ResolvedExpression {
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResolvedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: ResolvedExpression,
}
