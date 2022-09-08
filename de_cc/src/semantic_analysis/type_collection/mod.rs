mod declaration;
mod expression;

use declaration::*;
use expression::*;

use crate::{
    language::{
        typed::{TyApplication, TyFile, TyNode},
        untyped::{Application, File, Node},
    },
    namespace::namespace::Namespace,
};

/// Type collection is the process of iterating through the AST nodes to gain information about
/// the types present in the AST and the declarations present in the AST.
///
/// What happens during type collection:
/// 1. `TypeInfo`'s are inserted into the `TypeEngine` and then referred to by `TypeId`'s
/// 2. `Expressions` are transformed into `TypedExpression`, `Declaration` into `TypedDeclaration`, etc
/// 3. instances of function declarations, struct declarations, etc, are inserted into the `DeclarationEngine`
///
/// What does not happen during type collection:
/// 1. no type checking
/// 2. no type inference
/// 3. no unification of types
///
pub(crate) fn type_collect(namespace: &mut Namespace, application: Application) -> TyApplication {
    let files = application
        .files
        .into_iter()
        .map(|file| type_collect_file(namespace, file))
        .collect();
    TyApplication { files }
}

fn type_collect_file(namespace: &mut Namespace, file: File) -> TyFile {
    let nodes = file
        .nodes
        .into_iter()
        .map(|node| type_collect_node(namespace, node))
        .collect::<Vec<_>>();
    TyFile {
        name: file.name,
        nodes,
    }
}

fn type_collect_node(namespace: &mut Namespace, node: Node) -> TyNode {
    match node {
        Node::StarImport(_) => todo!(),
        Node::Declaration(decl) => TyNode::Declaration(type_collect_declaration(namespace, decl)),
        Node::Expression(expression) => {
            TyNode::Expression(type_collect_expression(namespace, expression))
        }
        Node::ReturnStatement(expression) => {
            TyNode::Expression(type_collect_expression(namespace, expression))
        }
    }
}
