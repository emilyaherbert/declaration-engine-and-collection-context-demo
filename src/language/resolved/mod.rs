use std::fmt;

use self::{resolved_declaration::ResolvedDeclaration, resolved_expression::ResolvedExpression};

pub(crate) mod resolved_declaration;
pub(crate) mod resolved_expression;

#[derive(Debug)]
pub struct ResolvedTree {
    pub(crate) nodes: Vec<ResolvedNode>,
}

impl fmt::Display for ResolvedTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();
        builder.push_str("\n\n>>>\n");
        for node in self.nodes.iter() {
            builder.push_str(&node.to_string());
            builder.push_str(";\n");
        }
        builder.push_str("<<<\n");
        write!(f, "{}", builder)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ResolvedNode {
    Declaration(ResolvedDeclaration),
    Expression(ResolvedExpression),
    ReturnStatement(ResolvedExpression),
}

impl fmt::Display for ResolvedNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedNode::Declaration(declaration) => write!(f, "{}", declaration),
            ResolvedNode::Expression(expression) => write!(f, "{}", expression),
            ResolvedNode::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
}
