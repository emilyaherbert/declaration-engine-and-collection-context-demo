use crate::{
    declaration_engine::declaration_engine::DeclarationEngine, types::pretty_print::PrettyPrint,
};

use self::{typed_declaration::TypedDeclaration, typed_expression::TypedExpression};

pub(crate) mod typed_declaration;
pub(crate) mod typed_expression;

pub(crate) struct TypedApplication {
    pub files: Vec<TypedFile>,
}

pub(crate) struct TypedFile {
    pub(crate) name: String,
    pub(crate) nodes: Vec<TypedNode>,
}

#[derive(Clone)]
pub(crate) enum TypedNode {
    // StarImport(String),
    Declaration(TypedDeclaration),
    Expression(TypedExpression),
    ReturnStatement(TypedExpression),
}

impl PrettyPrint for TypedNode {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            TypedNode::Declaration(declaration) => {
                format!("{}", declaration.pretty_print(declaration_engine))
            }
            TypedNode::Expression(expression) => {
                format!("{}", expression.pretty_print(declaration_engine))
            }
            TypedNode::ReturnStatement(expression) => {
                format!("return {}", expression.pretty_print(declaration_engine))
            }
        }
    }
}
