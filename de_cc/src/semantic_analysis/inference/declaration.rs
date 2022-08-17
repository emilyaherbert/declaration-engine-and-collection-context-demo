use crate::{
    collection_context::collection_context::CollectionContext,
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
    namespace::namespace::Namespace,
    type_system::type_engine::{insert_type, unify_types},
};

use super::{analyze_expression, analyze_nodes};

pub(super) fn analyze_declaration(
    namespace: &mut Namespace,
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    declaration: Declaration,
) -> TypedDeclaration {
    match declaration {
        Declaration::Variable(variable_declaration) => {
            let typed_variable_declaration = analyze_variable(
                namespace,
                collection_context,
                declaration_engine,
                variable_declaration,
            );
            let name = typed_variable_declaration.name.clone();
            let decl = TypedDeclaration::Variable(typed_variable_declaration);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        Declaration::Function(function_declaration) => {
            let typed_function_declaration = analyze_function(
                namespace,
                collection_context,
                declaration_engine,
                function_declaration,
            );
            let name = typed_function_declaration.name.clone();
            declaration_engine.insert_function(name.clone(), typed_function_declaration);
            TypedDeclaration::Function(name)
        }
        Declaration::Trait(_) => {
            unimplemented!();
            // let typed_trait_declaration = analyze_trait(
            //     namespace,
            //     collection_context,
            //     declaration_engine,
            //     trait_declaration,
            // );
            // let name = typed_trait_declaration.name.clone();
            // declaration_engine.insert_trait(name.clone(), typed_trait_declaration);
            // TypedDeclaration::Trait(name)
        }
        Declaration::Struct(_) => {
            unimplemented!();
            // let typed_struct_declaration = analyze_struct(
            //     namespace,
            //     collection_context,
            //     declaration_engine,
            //     struct_declaration,
            // );
            // let name = typed_struct_declaration.name.clone();
            // declaration_engine.insert_struct(name.clone(), typed_struct_declaration);
            // TypedDeclaration::Struct(name)
        }
        Declaration::Enum(_) => {
            unimplemented!();
            // let typed_enum_declaration = analyze_enum(
            //     namespace,
            //     collection_context,
            //     declaration_engine,
            //     enum_declaration,
            // );
            // let name = typed_enum_declaration.name.clone();
            // declaration_engine.insert_enum(name.clone(), typed_enum_declaration);
            // TypedDeclaration::Enum(name)
        }
        Declaration::TraitImpl(_) => unimplemented!(),
        Declaration::SelfImpl(_) => unimplemented!(),
    }
}

fn analyze_variable(
    namespace: &mut Namespace,
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    variable_declaration: VariableDeclaration,
) -> TypedVariableDeclaration {
    let new_body = analyze_expression(
        namespace,
        collection_context,
        declaration_engine,
        variable_declaration.body,
    );
    let new_type_ascription = insert_type(variable_declaration.type_ascription);
    unify_types(new_body.type_id, new_type_ascription).unwrap();
    TypedVariableDeclaration {
        name: variable_declaration.name,
        body: new_body,
        type_ascription: new_type_ascription,
    }
}

fn analyze_function(
    namespace: &mut Namespace,
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    function_declaration: FunctionDeclaration,
) -> TypedFunctionDeclaration {
    if !function_declaration.type_parameters.is_empty() {
        panic!()
    }
    let new_parameters = function_declaration
        .parameters
        .into_iter()
        .map(|parameter| {
            analyze_function_parameter(namespace, collection_context, declaration_engine, parameter)
        })
        .collect::<Vec<_>>();
    let new_body = analyze_nodes(
        &mut namespace.scoped(function_declaration.name.clone()),
        collection_context,
        declaration_engine,
        function_declaration.body,
    );
    TypedFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: vec![],
        parameters: new_parameters,
        body: new_body,
        return_type: insert_type(function_declaration.return_type),
    }
}

fn analyze_function_parameter(
    _namespace: &mut Namespace,
    _collection_context: &CollectionContext,
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
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    trait_declaration: TraitDeclaration,
) -> TypedTraitDeclaration {
    let new_interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| {
            analyze_trait_fn(namespace, collection_context, declaration_engine, trait_fn)
        })
        .collect::<Vec<_>>();
    TypedTraitDeclaration {
        name: trait_declaration.name,
        interface_surface: new_interface_surface,
        methods: trait_declaration.methods,
    }
}

fn analyze_trait_fn(
    namespace: &mut Namespace,
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    trait_fn: TraitFn,
) -> TypedTraitFn {
    let new_parameters = trait_fn
        .parameters
        .into_iter()
        .map(|parameter| {
            analyze_function_parameter(namespace, collection_context, declaration_engine, parameter)
        })
        .collect::<Vec<_>>();
    TypedTraitFn {
        name: trait_fn.name,
        parameters: new_parameters,
        return_type: insert_type(trait_fn.return_type),
    }
}

fn analyze_struct(
    namespace: &mut Namespace,
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    struct_declaration: StructDeclaration,
) -> TypedStructDeclaration {
    if !struct_declaration.type_parameters.is_empty() {
        panic!()
    }
    let new_fields = struct_declaration
        .fields
        .into_iter()
        .map(|field| analyze_struct_field(namespace, collection_context, declaration_engine, field))
        .collect::<Vec<_>>();
    TypedStructDeclaration {
        name: struct_declaration.name,
        type_parameters: vec![],
        fields: new_fields,
    }
}

fn analyze_struct_field(
    _namespace: &mut Namespace,
    _collection_context: &CollectionContext,
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
    collection_context: &CollectionContext,
    declaration_engine: &mut DeclarationEngine,
    enum_declaration: EnumDeclaration,
) -> TypedEnumDeclaration {
    if !enum_declaration.type_parameters.is_empty() {
        panic!()
    }
    let new_variants = enum_declaration
        .variants
        .into_iter()
        .map(|variant| {
            analyze_enum_variant(namespace, collection_context, declaration_engine, variant)
        })
        .collect::<Vec<_>>();
    TypedEnumDeclaration {
        name: enum_declaration.name,
        type_parameters: vec![],
        variants: new_variants,
    }
}

fn analyze_enum_variant(
    _namespace: &mut Namespace,
    _collection_context: &CollectionContext,
    _declaration_engine: &mut DeclarationEngine,
    enum_variant: EnumVariant,
) -> TypedEnumVariant {
    TypedEnumVariant {
        name: enum_variant.name,
        tag: enum_variant.tag,
        type_id: insert_type(enum_variant.type_info),
    }
}
