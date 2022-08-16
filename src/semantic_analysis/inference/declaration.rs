use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::typed_declaration::{
            TypedDeclaration, TypedEnumDeclaration, TypedEnumVariant, TypedFunctionDeclaration,
            TypedFunctionParameter, TypedStructDeclaration, TypedStructField,
            TypedTraitDeclaration, TypedTraitFn, TypedVariableDeclaration,
        },
        untyped::declaration::{
            Declaration, EnumDeclaration, EnumVariant, FunctionDeclaration, FunctionParameter,
            StructDeclaration, StructField, TraitDeclaration, TraitFn, VariableDeclaration,
        },
    },
    namespace::Namespace,
    type_system::type_engine::{insert_type, unify},
};

use super::{analyze_expression, analyze_nodes};

pub(super) fn analyze_declaration(
    namespace: &mut Namespace,

    declaration_engine: &mut DeclarationEngine,
    declaration: Declaration,
) -> Result<TypedDeclaration, String> {
    let decl = match declaration {
        Declaration::Variable(variable_declaration) => {
            let typed_variable_declaration =
                analyze_variable(namespace, declaration_engine, variable_declaration)?;
            let name = typed_variable_declaration.name.clone();
            let decl = TypedDeclaration::Variable(typed_variable_declaration);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        Declaration::Function(function_declaration) => {
            let typed_function_declaration =
                analyze_function(namespace, declaration_engine, function_declaration)?;
            let name = typed_function_declaration.name.clone();
            declaration_engine.insert_function(name.clone(), typed_function_declaration);
            TypedDeclaration::Function(name)
        }
        Declaration::Trait(trait_declaration) => {
            let typed_trait_declaration =
                analyze_trait(namespace, declaration_engine, trait_declaration);
            let name = typed_trait_declaration.name.clone();
            declaration_engine.insert_trait(name.clone(), typed_trait_declaration);
            TypedDeclaration::Trait(name)
        }
        Declaration::Struct(struct_declaration) => {
            let typed_struct_declaration =
                analyze_struct(namespace, declaration_engine, struct_declaration);
            let name = typed_struct_declaration.name.clone();
            declaration_engine.insert_struct(name.clone(), typed_struct_declaration);
            TypedDeclaration::Struct(name)
        }
        Declaration::Enum(enum_declaration) => {
            let typed_enum_declaration =
                analyze_enum(namespace, declaration_engine, enum_declaration);
            let name = typed_enum_declaration.name.clone();
            declaration_engine.insert_enum(name.clone(), typed_enum_declaration);
            TypedDeclaration::Enum(name)
        }
        Declaration::TraitImpl(_) => unimplemented!(),
        Declaration::SelfImpl(_) => unimplemented!(),
    };
    Ok(decl)
}

fn analyze_variable(
    namespace: &mut Namespace,

    declaration_engine: &mut DeclarationEngine,
    variable_declaration: VariableDeclaration,
) -> Result<TypedVariableDeclaration, String> {
    let new_body = analyze_expression(namespace, declaration_engine, variable_declaration.body)?;
    let new_type_ascription = insert_type(variable_declaration.type_ascription);
    unify(new_body.type_id, new_type_ascription)?;
    Ok(TypedVariableDeclaration {
        name: variable_declaration.name,
        body: new_body,
        type_ascription: new_type_ascription,
    })
}

fn analyze_function(
    namespace: &mut Namespace,

    declaration_engine: &mut DeclarationEngine,
    function_declaration: FunctionDeclaration,
) -> Result<TypedFunctionDeclaration, String> {
    let new_parameters = function_declaration
        .parameters
        .into_iter()
        .map(|parameter| analyze_function_parameter(namespace, declaration_engine, parameter))
        .collect::<Vec<_>>();
    let new_body = analyze_nodes(namespace, declaration_engine, function_declaration.body)?;
    Ok(TypedFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: todo!(),
        parameters: new_parameters,
        body: new_body,
        return_type: insert_type(function_declaration.return_type),
    })
}

fn analyze_function_parameter(
    namespace: &mut Namespace,

    _declaration_engine: &mut DeclarationEngine,
    function_parameter: FunctionParameter,
) -> TypedFunctionParameter {
    TypedFunctionParameter {
        name: function_parameter.name,
        type_id: insert_type(function_parameter.type_info),
    }
}

fn analyze_trait(
    namespace: &mut Namespace,

    declaration_engine: &mut DeclarationEngine,
    trait_declaration: TraitDeclaration,
) -> TypedTraitDeclaration {
    let new_interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| analyze_trait_fn(namespace, declaration_engine, trait_fn))
        .collect::<Vec<_>>();
    TypedTraitDeclaration {
        name: trait_declaration.name,
        interface_surface: new_interface_surface,
        methods: trait_declaration.methods,
    }
}

fn analyze_trait_fn(
    namespace: &mut Namespace,

    declaration_engine: &mut DeclarationEngine,
    trait_fn: TraitFn,
) -> TypedTraitFn {
    let new_parameters = trait_fn
        .parameters
        .into_iter()
        .map(|parameter| analyze_function_parameter(namespace, declaration_engine, parameter))
        .collect::<Vec<_>>();
    TypedTraitFn {
        name: trait_fn.name,
        parameters: new_parameters,
        return_type: insert_type(trait_fn.return_type),
    }
}

fn analyze_struct(
    namespace: &mut Namespace,

    declaration_engine: &mut DeclarationEngine,
    struct_declaration: StructDeclaration,
) -> TypedStructDeclaration {
    let new_fields = struct_declaration
        .fields
        .into_iter()
        .map(|field| analyze_struct_field(namespace, declaration_engine, field))
        .collect::<Vec<_>>();
    TypedStructDeclaration {
        name: struct_declaration.name,
        type_parameters: todo!(),
        fields: new_fields,
    }
}

fn analyze_struct_field(
    namespace: &mut Namespace,

    _declaration_engine: &mut DeclarationEngine,
    struct_field: StructField,
) -> TypedStructField {
    TypedStructField {
        name: struct_field.name,
        type_id: insert_type(struct_field.type_info),
    }
}

fn analyze_enum(
    namespace: &mut Namespace,

    declaration_engine: &mut DeclarationEngine,
    enum_declaration: EnumDeclaration,
) -> TypedEnumDeclaration {
    let new_variants = enum_declaration
        .variants
        .into_iter()
        .map(|variant| analyze_enum_variant(namespace, declaration_engine, variant))
        .collect::<Vec<_>>();
    TypedEnumDeclaration {
        name: enum_declaration.name,
        type_parameters: todo!(),
        variants: new_variants,
    }
}

fn analyze_enum_variant(
    namespace: &mut Namespace,

    _declaration_engine: &mut DeclarationEngine,
    enum_variant: EnumVariant,
) -> TypedEnumVariant {
    TypedEnumVariant {
        name: enum_variant.name,
        tag: enum_variant.tag,
        type_id: insert_type(enum_variant.type_info),
    }
}
