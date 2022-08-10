mod resolved_declaration;
mod resolved_expression;

pub(crate) use resolved_declaration::*;
pub(crate) use resolved_expression::*;

#[derive(Debug)]
pub(crate) struct ResolvedTree {
    pub(crate) nodes: Vec<ResolvedNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ResolvedNode {
    Declaration(ResolvedDeclaration),
    Expression(ResolvedExpression),
    ReturnStatement(ResolvedExpression),
}
