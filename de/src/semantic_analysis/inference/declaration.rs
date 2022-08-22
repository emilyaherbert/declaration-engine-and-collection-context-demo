use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::{
            typed_declaration::{
                TypedDeclaration, TypedFunctionDeclaration, TypedFunctionParameter,
                TypedStructDeclaration, TypedStructField, TypedTraitDeclaration, TypedTraitFn,
                TypedTraitImpl, TypedVariableDeclaration,
            },
            typed_expression::{TypedExpression, TypedExpressionVariant},
            TypedNode,
        },
        untyped::{
            declaration::{
                Declaration, FunctionDeclaration, FunctionParameter, StructDeclaration,
                StructField, TraitDeclaration, TraitFn, TraitImpl, VariableDeclaration,
            },
            Node,
        },
    },
    namespace::namespace::Namespace,
    type_system::{
        type_engine::{eval_type, insert_type, unify_types},
        type_id::TypeId,
        type_info::TypeInfo,
    },
};

use super::{analyze_expression, analyze_node};

pub(super) fn analyze_declaration(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    declaration: Declaration,
) -> TypedDeclaration {
    match declaration {
        Declaration::Variable(variable_declaration) => {
            let typed_variable_declaration =
                analyze_variable(namespace, declaration_engine, variable_declaration);
            let name = typed_variable_declaration.name.clone();
            let decl = TypedDeclaration::Variable(typed_variable_declaration);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        Declaration::Function(function_declaration) => {
            let typed_function_declaration = analyze_function(
                &mut namespace.scoped(),
                declaration_engine,
                function_declaration,
            );
            let name = typed_function_declaration.name.clone();
            let decl_id = declaration_engine.insert_function(typed_function_declaration);
            let decl = TypedDeclaration::Function(decl_id);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        Declaration::Trait(trait_declaration) => {
            let typed_trait_declaration = analyze_trait(
                &mut namespace.scoped(),
                trait_declaration,
            );
            let name = typed_trait_declaration.name.clone();
            let decl_id = declaration_engine.insert_trait(typed_trait_declaration);
            let decl = TypedDeclaration::Trait(decl_id);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        Declaration::TraitImpl(trait_impl) => {
            let typed_trait_impl =
                analyze_trait_impl(&mut namespace.scoped(), declaration_engine, trait_impl);
            let decl_id = declaration_engine.insert_trait_impl(typed_trait_impl);
            TypedDeclaration::TraitImpl(decl_id)
        }
        Declaration::Struct(struct_declaration) => {
            let typed_struct_declaration = analyze_struct(&mut namespace.scoped(), struct_declaration);
            let name = typed_struct_declaration.name.clone();
            let decl_id = declaration_engine.insert_struct(typed_struct_declaration);
            let decl = TypedDeclaration::Struct(decl_id);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
        // Declaration::Enum(_) => {
        //     let typed_enum_declaration = analyze_enum(
        //         namespace,
        //         declaration_engine,
        //         enum_declaration,
        //     );
        //     let name = typed_enum_declaration.name.clone();
        //     declaration_engine.insert_enum(name.clone(), typed_enum_declaration);
        //     TypedDeclaration::Enum(name)
        // }
        // Declaration::TraitImpl(_) => unimplemented!(),
        // Declaration::SelfImpl(_) => unimplemented!(),
    }
}

fn analyze_variable(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    variable_declaration: VariableDeclaration,
) -> TypedVariableDeclaration {
    let new_body = analyze_expression(namespace, declaration_engine, variable_declaration.body);
    let new_type_ascription =
        eval_type(insert_type(variable_declaration.type_ascription), namespace).unwrap();
    unify_types(new_body.type_id, new_type_ascription).unwrap();
    TypedVariableDeclaration {
        name: variable_declaration.name,
        body: new_body,
        type_ascription: new_type_ascription,
    }
}

fn analyze_function(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    function_declaration: FunctionDeclaration,
) -> TypedFunctionDeclaration {
    // insert type params into namespace
    for type_parameter in function_declaration.type_parameters.iter() {
        let type_parameter_decl = TypedDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);
    }

    // type check the function params
    let typed_parameters = function_declaration
        .parameters
        .into_iter()
        .map(|parameter| analyze_function_parameter(namespace, parameter))
        .collect::<Vec<_>>();

    // type check the function return type
    let return_type = eval_type(insert_type(function_declaration.return_type), namespace).unwrap();

    // type check the function body
    let (typed_body, typed_body_return_type) =
        analyze_code_block(namespace, declaration_engine, function_declaration.body);

    // unify the funtion return type and body return type
    unify_types(typed_body_return_type, return_type).unwrap();

    TypedFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: function_declaration.type_parameters,
        parameters: typed_parameters,
        body: typed_body,
        return_type,
    }
}

fn analyze_function_parameter(
    namespace: &mut Namespace,
    function_parameter: FunctionParameter,
) -> TypedFunctionParameter {
    let type_id = eval_type(insert_type(function_parameter.type_info), namespace).unwrap();
    let typed_parameter_decl = TypedDeclaration::Variable(TypedVariableDeclaration {
        name: function_parameter.name.clone(),
        type_ascription: type_id,
        body: TypedExpression {
            variant: TypedExpressionVariant::FunctionParameter,
            type_id,
        },
    });
    namespace.insert_symbol(function_parameter.name.clone(), typed_parameter_decl);
    TypedFunctionParameter {
        name: function_parameter.name,
        type_id,
    }
}

fn analyze_code_block(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    nodes: Vec<Node>,
) -> (Vec<TypedNode>, TypeId) {
    let mut typed_nodes = vec![];
    for node in nodes.into_iter() {
        let typed_node = analyze_node(namespace, declaration_engine, node);
        let possibly_return = match &typed_node {
            TypedNode::ReturnStatement(exp) => Some(exp.type_id),
            _ => None,
        };
        typed_nodes.push(typed_node);
        if let Some(return_type) = possibly_return {
            return (typed_nodes, return_type);
        }
    }
    (typed_nodes, insert_type(TypeInfo::Unit))
}

fn analyze_trait(
    namespace: &mut Namespace,
    trait_declaration: TraitDeclaration,
) -> TypedTraitDeclaration {
    let new_interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| analyze_trait_fn(namespace, trait_fn))
        .collect::<Vec<_>>();
    TypedTraitDeclaration {
        name: trait_declaration.name,
        interface_surface: new_interface_surface,
    }
}

fn analyze_trait_fn(namespace: &mut Namespace, trait_fn: TraitFn) -> TypedTraitFn {
    let new_parameters = trait_fn
        .parameters
        .into_iter()
        .map(|parameter| analyze_function_parameter(namespace, parameter))
        .collect::<Vec<_>>();
    TypedTraitFn {
        name: trait_fn.name,
        parameters: new_parameters,
        return_type: eval_type(insert_type(trait_fn.return_type), namespace).unwrap(),
    }
}

fn analyze_trait_impl(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    trait_impl: TraitImpl,
) -> TypedTraitImpl {
    // insert type params into namespace
    for type_parameter in trait_impl.type_parameters.iter() {
        let type_parameter_decl = TypedDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);
    }

    // get the trait from the declaration engine
    let trait_id = namespace
        .get_symbol(&trait_impl.trait_name)
        .unwrap()
        .expect_trait()
        .unwrap();
    let _trait_decl = declaration_engine.get_trait(trait_id).unwrap();

    // TODO: check to see if all of the methods are implementing, no new methods implementing,
    // when generic traits are implementing add the monomorphized copies to the declaration
    // engine

    // type check the type we are implementing for
    let type_implementing_for =
        eval_type(insert_type(trait_impl.type_implementing_for), namespace).unwrap();

    // type check the methods
    let typed_method_ids = trait_impl
        .methods
        .into_iter()
        .map(|method| {
            let typed_method = analyze_function(namespace, declaration_engine, method);
            declaration_engine.insert_function(typed_method)
        })
        .collect::<Vec<_>>();

    namespace.insert_trait_impl(
        type_implementing_for,
        trait_impl.trait_name.clone(),
        typed_method_ids.clone(),
    );

    TypedTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for,
        type_parameters: trait_impl.type_parameters,
        methods: typed_method_ids,
    }
}

fn analyze_struct(
    namespace: &mut Namespace,
    struct_declaration: StructDeclaration,
) -> TypedStructDeclaration {
    // insert type params into namespace
    for type_parameter in struct_declaration.type_parameters.iter() {
        let type_parameter_decl = TypedDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);
    }

    // type check the fields
    let typed_fields = struct_declaration
        .fields
        .into_iter()
        .map(|field| analyze_struct_field(namespace, field))
        .collect::<Vec<_>>();

    TypedStructDeclaration {
        name: struct_declaration.name,
        type_parameters: struct_declaration.type_parameters,
        fields: typed_fields,
    }
}

fn analyze_struct_field(namespace: &mut Namespace, struct_field: StructField) -> TypedStructField {
    TypedStructField {
        name: struct_field.name,
        type_id: eval_type(insert_type(struct_field.type_info), namespace).unwrap(),
    }
}

// fn analyze_enum(
//     namespace: &mut Namespace,
//     declaration_engine: &mut DeclarationEngine,
//     enum_declaration: EnumDeclaration,
// ) -> TypedEnumDeclaration {
//     if !enum_declaration.type_parameters.is_empty() {
//         panic!()
//     }
//     let new_variants = enum_declaration
//         .variants
//         .into_iter()
//         .map(|variant| analyze_enum_variant(namespace, declaration_engine, variant))
//         .collect::<Vec<_>>();
//     TypedEnumDeclaration {
//         name: enum_declaration.name,
//         type_parameters: vec![],
//         variants: new_variants,
//     }
// }

// fn analyze_enum_variant(
//     _namespace: &mut Namespace,
//     _declaration_engine: &mut DeclarationEngine,
//     enum_variant: EnumVariant,
// ) -> TypedEnumVariant {
//     TypedEnumVariant {
//         name: enum_variant.name,
//         tag: enum_variant.tag,
//         type_id: insert_type(enum_variant.type_info),
//     }
// }