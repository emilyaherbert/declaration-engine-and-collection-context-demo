use crate::{
    declaration_engine::DeclarationEngine,
    language::{
        Declaration, EnumDeclaration, EnumVariant, FunctionDeclaration, FunctionParameter,
        StructDeclaration, StructField, TraitDeclaration, TraitFn, TypedDeclaration,
        TypedEnumDeclaration, TypedEnumVariant, TypedFunctionDeclaration, TypedFunctionParameter,
        TypedStructDeclaration, TypedStructField, TypedTraitDeclaration, TypedTraitFn,
        TypedVariableDeclaration, VariableDeclaration,
    },
};

use super::{analyze_expression, analyze_nodes};

pub(super) fn analyze_declaration(
    declaration_engine: &mut DeclarationEngine,
    declaration: Declaration,
) -> TypedDeclaration {
    match declaration {
        Declaration::Variable(variable_declaration) => {
            let typed_variable_declaration =
                analyze_variable(declaration_engine, variable_declaration);
            let name = typed_variable_declaration.name.clone();
            declaration_engine.insert_variable(name.clone(), typed_variable_declaration);
            TypedDeclaration::Variable(name)
        }
        Declaration::Function(function_declaration) => {
            let typed_function_declaration =
                analyze_function(declaration_engine, function_declaration);
            let name = typed_function_declaration.name.clone();
            declaration_engine.insert_function(name.clone(), typed_function_declaration);
            TypedDeclaration::Function(name)
        }
        Declaration::Trait(trait_declaration) => {
            let typed_trait_declaration = analyze_trait(declaration_engine, trait_declaration);
            let name = typed_trait_declaration.name.clone();
            declaration_engine.insert_trait(name.clone(), typed_trait_declaration);
            TypedDeclaration::Trait(name)
        }
        Declaration::Struct(struct_declaration) => {
            let typed_struct_declaration = analyze_struct(declaration_engine, struct_declaration);
            let name = typed_struct_declaration.name.clone();
            declaration_engine.insert_struct(name.clone(), typed_struct_declaration);
            TypedDeclaration::Struct(name)
        }
        Declaration::Enum(enum_declaration) => {
            let typed_enum_declaration = analyze_enum(declaration_engine, enum_declaration);
            let name = typed_enum_declaration.name.clone();
            declaration_engine.insert_enum(name.clone(), typed_enum_declaration);
            TypedDeclaration::Enum(name)
        }
    }
}

fn analyze_variable(
    declaration_engine: &mut DeclarationEngine,
    variable_declaration: VariableDeclaration,
) -> TypedVariableDeclaration {
    let new_body = analyze_expression(declaration_engine, variable_declaration.body);
    TypedVariableDeclaration {
        name: variable_declaration.name,
        body: new_body,
    }
}

fn analyze_function(
    declaration_engine: &mut DeclarationEngine,
    function_declaration: FunctionDeclaration,
) -> TypedFunctionDeclaration {
    let new_parameters = function_declaration
        .parameters
        .into_iter()
        .map(|parameter| analyze_function_parameter(declaration_engine, parameter))
        .collect::<Vec<_>>();
    let new_body = analyze_nodes(declaration_engine, function_declaration.body);
    TypedFunctionDeclaration {
        name: function_declaration.name,
        parameters: new_parameters,
        body: new_body,
    }
}

fn analyze_function_parameter(
    _declaration_engine: &mut DeclarationEngine,
    function_parameter: FunctionParameter,
) -> TypedFunctionParameter {
    TypedFunctionParameter {
        name: function_parameter.name,
    }
}

fn analyze_trait(
    declaration_engine: &mut DeclarationEngine,
    trait_declaration: TraitDeclaration,
) -> TypedTraitDeclaration {
    let new_interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| analyze_trait_fn(declaration_engine, trait_fn))
        .collect::<Vec<_>>();
    TypedTraitDeclaration {
        name: trait_declaration.name,
        interface_surface: new_interface_surface,
        methods: trait_declaration.methods,
    }
}

fn analyze_trait_fn(declaration_engine: &mut DeclarationEngine, trait_fn: TraitFn) -> TypedTraitFn {
    let new_parameters = trait_fn
        .parameters
        .into_iter()
        .map(|parameter| analyze_function_parameter(declaration_engine, parameter))
        .collect::<Vec<_>>();
    TypedTraitFn {
        name: trait_fn.name,
        parameters: new_parameters,
    }
}

fn analyze_struct(
    declaration_engine: &mut DeclarationEngine,
    struct_declaration: StructDeclaration,
) -> TypedStructDeclaration {
    let new_fields = struct_declaration
        .fields
        .into_iter()
        .map(|field| analyze_struct_field(declaration_engine, field))
        .collect::<Vec<_>>();
    TypedStructDeclaration {
        name: struct_declaration.name,
        fields: new_fields,
    }
}

fn analyze_struct_field(
    _declaration_engine: &mut DeclarationEngine,
    struct_field: StructField,
) -> TypedStructField {
    TypedStructField {
        name: struct_field.name,
    }
}

fn analyze_enum(
    declaration_engine: &mut DeclarationEngine,
    enum_declaration: EnumDeclaration,
) -> TypedEnumDeclaration {
    let new_variants = enum_declaration
        .variants
        .into_iter()
        .map(|variant| analyze_enum_variant(declaration_engine, variant))
        .collect::<Vec<_>>();
    TypedEnumDeclaration {
        name: enum_declaration.name,
        variants: new_variants,
    }
}

fn analyze_enum_variant(
    _declaration_engine: &mut DeclarationEngine,
    enum_variant: EnumVariant,
) -> TypedEnumVariant {
    TypedEnumVariant {
        name: enum_variant.name,
        tag: enum_variant.tag,
    }
}
