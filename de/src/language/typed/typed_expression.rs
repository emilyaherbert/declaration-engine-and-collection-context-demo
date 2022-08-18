use crate::{
    declaration_engine::declaration_engine::DeclarationEngine, language::literal::Literal,
    type_system::type_id::TypeId, types::pretty_print::PrettyPrint,
};

#[derive(Clone, PartialEq)]
pub(crate) struct TypedExpression {
    pub(crate) variant: TypedExpressionVariant,
    pub(crate) type_id: TypeId,
}

impl PrettyPrint for TypedExpression {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        format!("{}", self.variant.pretty_print(declaration_engine))
    }
}

#[derive(Clone, PartialEq)]
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
    // Struct {
    //     struct_name: String,
    //     fields: Vec<TypedStructExpressionField>,
    // },
    // Enum {
    //     enum_name: String,
    //     variant_name: String,
    //     value: Box<TypedExpression>,
    // },
}

impl PrettyPrint for TypedExpressionVariant {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            TypedExpressionVariant::Literal { value } => format!("{}", value),
            TypedExpressionVariant::Variable { name } => format!("{}", name),
            TypedExpressionVariant::FunctionApplication { name, arguments } => {
                format!(
                    "{}({})",
                    name,
                    &arguments
                        .iter()
                        .map(|argument| argument.pretty_print(declaration_engine))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: TypedExpression,
}
