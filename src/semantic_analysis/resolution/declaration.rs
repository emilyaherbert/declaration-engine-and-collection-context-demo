use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        resolved::resolved_declaration::ResolvedDeclaration,
        typed::typed_declaration::TypedDeclaration,
    },
};

pub(super) fn resolve_declaration(
    declaration_engine: &DeclarationEngine,
    declaration: TypedDeclaration,
) -> ResolvedDeclaration {
    match declaration {
        TypedDeclaration::Variable(variable_declaration) => {
            ResolvedDeclaration::Variable(variable_declaration)
        }
        TypedDeclaration::Function(name) => {
            let declaration = declaration_engine.get_function(name).cloned().unwrap();
            ResolvedDeclaration::Function(declaration)
        }
        TypedDeclaration::Trait(name) => {
            let declaration = declaration_engine.get_trait(name).cloned().unwrap();
            ResolvedDeclaration::Trait(declaration)
        }
        TypedDeclaration::Struct(name) => {
            let declaration = declaration_engine.get_struct(name).cloned().unwrap();
            ResolvedDeclaration::Struct(declaration)
        }
        TypedDeclaration::Enum(name) => {
            let declaration = declaration_engine.get_enum(name).cloned().unwrap();
            ResolvedDeclaration::Enum(declaration)
        }
        TypedDeclaration::TraitImpl(_) => todo!(),
    }
}
