use std::fmt;

use crate::language::typed::typed_declaration::{
    TypedEnumDeclaration, TypedFunctionDeclaration, TypedStructDeclaration, TypedTraitDeclaration,
    TypedTraitImpl, TypedVariableDeclaration,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ResolvedDeclaration {
    Variable(TypedVariableDeclaration),
    Function(TypedFunctionDeclaration),
    Trait(TypedTraitDeclaration),
    Struct(TypedStructDeclaration),
    Enum(TypedEnumDeclaration),
    ImplTrait(TypedTraitImpl),
}

impl fmt::Display for ResolvedDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedDeclaration::Variable(decl) => write!(f, "{}", decl),
            ResolvedDeclaration::Function(_) => todo!(),
            ResolvedDeclaration::Trait(_) => todo!(),
            ResolvedDeclaration::Struct(_) => todo!(),
            ResolvedDeclaration::Enum(_) => todo!(),
            ResolvedDeclaration::ImplTrait(_) => todo!(),
        }
    }
}
