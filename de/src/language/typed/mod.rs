use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    type_system::type_mapping::TypeMapping,
    types::{copy_types::CopyTypes, pretty_print::PrettyPrint},
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

#[derive(Clone, Debug)]
pub(crate) enum TypedNode {
    // StarImport(String),
    Declaration(TypedDeclaration),
    Expression(TypedExpression),
    ReturnStatement(TypedExpression),
}

impl CopyTypes for TypedNode {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TypedNode::Declaration(declaration) => declaration.copy_types(type_mapping),
            TypedNode::Expression(expression) => expression.copy_types(type_mapping),
            TypedNode::ReturnStatement(expression) => expression.copy_types(type_mapping),
        }
    }
}

impl PrettyPrint for TypedNode {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            TypedNode::Declaration(declaration) => declaration.pretty_print(declaration_engine),
            TypedNode::Expression(expression) => expression.pretty_print(declaration_engine),
            TypedNode::ReturnStatement(expression) => {
                format!("return {}", expression.pretty_print(declaration_engine))
            }
        }
    }
}
