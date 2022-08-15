use self::{resolved_declaration::ResolvedDeclaration, resolved_expression::ResolvedExpression};

pub(crate) mod resolved_declaration;
pub(crate) mod resolved_expression;

#[derive(Debug)]
pub struct ResolvedTree {
    pub(crate) nodes: Vec<ResolvedNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ResolvedNode {
    Declaration(ResolvedDeclaration),
    Expression(ResolvedExpression),
    ReturnStatement(ResolvedExpression),
}
