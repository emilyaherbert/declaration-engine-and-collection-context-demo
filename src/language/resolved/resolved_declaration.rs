use crate::language::{
    TypedEnumDeclaration, TypedFunctionDeclaration, TypedStructDeclaration, TypedTraitDeclaration,
    TypedVariableDeclaration,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ResolvedDeclaration {
    Variable(TypedVariableDeclaration),
    Function(TypedFunctionDeclaration),
    Trait(TypedTraitDeclaration),
    Struct(TypedStructDeclaration),
    Enum(TypedEnumDeclaration),
}
