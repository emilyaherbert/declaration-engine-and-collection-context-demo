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
