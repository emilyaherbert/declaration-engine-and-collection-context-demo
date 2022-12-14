use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        typed::{
            typed_declaration::{
                TypedDeclaration, TypedFunctionDeclaration, TypedFunctionParameter,
                TypedStructDeclaration, TypedStructField, TypedTraitDeclaration, TypedTraitFn,
                TypedTraitImpl, TypedVariableDeclaration,
            },
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
                declaration_engine,
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
            namespace.insert_methods(
                typed_trait_impl.type_implementing_for,
                typed_trait_impl.trait_name.clone(),
                typed_trait_impl.methods.clone(),
            );
            let decl_id = declaration_engine.insert_trait_impl(typed_trait_impl);
            TypedDeclaration::TraitImpl(decl_id)
        }
        Declaration::Struct(struct_declaration) => {
            let typed_struct_declaration = analyze_struct(
                &mut namespace.scoped(),
                declaration_engine,
                struct_declaration,
            );
            let name = typed_struct_declaration.name.clone();
            let decl_id = declaration_engine.insert_struct(typed_struct_declaration);
            let decl = TypedDeclaration::Struct(decl_id);
            namespace.insert_symbol(name, decl.clone());
            decl
        }
    }
}

fn analyze_variable(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    variable_declaration: VariableDeclaration,
) -> TypedVariableDeclaration {
    let new_body = analyze_expression(namespace, declaration_engine, variable_declaration.body);
    let new_type_ascription = eval_type(
        insert_type(variable_declaration.type_ascription),
        namespace,
        declaration_engine,
    )
    .unwrap();
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
    // insert type params into namespace and handle trait constraints
    for type_parameter in function_declaration.type_parameters.iter() {
        let type_parameter_decl = TypedDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);

        // if the type param has a trait constraint, take the TypedTraitFn's from
        // the trait it is constrained upon and insert them into the namespace
        // under the type param
        if let Some(constraint) = &type_parameter.trait_constraint {
            let decl_id = namespace
                .get_symbol(&constraint.trait_name)
                .unwrap()
                .expect_trait()
                .unwrap();
            let trait_decl = declaration_engine.get_trait(decl_id).unwrap();
            namespace.insert_methods(
                type_parameter.type_id,
                constraint.trait_name.clone(),
                trait_decl.interface_surface,
            );
        }
    }

    // type check the function params
    let typed_parameters = function_declaration
        .parameters
        .into_iter()
        .map(|parameter| {
            let typed_parameter =
                analyze_function_parameter(namespace, declaration_engine, parameter);
            namespace.insert_symbol(typed_parameter.name.clone(), (&typed_parameter).into());
            typed_parameter
        })
        .collect::<Vec<_>>();

    // type check the function return type
    let return_type = eval_type(
        insert_type(function_declaration.return_type),
        namespace,
        declaration_engine,
    )
    .unwrap();

    // type check the function body
    let (typed_body, typed_body_return_type) =
        analyze_code_block(namespace, declaration_engine, function_declaration.body);

    // unify the function return type and body return type
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
    declaration_engine: &mut DeclarationEngine,
    function_parameter: FunctionParameter,
) -> TypedFunctionParameter {
    let type_id = eval_type(
        insert_type(function_parameter.type_info),
        namespace,
        declaration_engine,
    )
    .unwrap();
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
    declaration_engine: &mut DeclarationEngine,
    trait_declaration: TraitDeclaration,
) -> TypedTraitDeclaration {
    let new_interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| {
            let trait_fn = analyze_trait_fn(&mut namespace.scoped(), declaration_engine, trait_fn);
            declaration_engine.insert_trait_fn(trait_fn)
        })
        .collect::<Vec<_>>();
    TypedTraitDeclaration {
        name: trait_declaration.name,
        interface_surface: new_interface_surface,
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
    let return_type = eval_type(
        insert_type(trait_fn.return_type),
        namespace,
        declaration_engine,
    )
    .unwrap();
    TypedTraitFn {
        name: trait_fn.name,
        parameters: new_parameters,
        return_type,
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
    // when generic traits are implemented add the monomorphized copies to the declaration
    // engine

    // type check the type we are implementing for
    let type_implementing_for = eval_type(
        insert_type(trait_impl.type_implementing_for),
        namespace,
        declaration_engine,
    )
    .unwrap();

    // type check the methods
    let typed_method_ids = trait_impl
        .methods
        .into_iter()
        .map(|method| {
            let typed_method = analyze_function(namespace, declaration_engine, method);
            declaration_engine.insert_function(typed_method)
        })
        .collect::<Vec<_>>();

    TypedTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for,
        type_parameters: trait_impl.type_parameters,
        methods: typed_method_ids,
    }
}

fn analyze_struct(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
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
        .map(|field| analyze_struct_field(namespace, declaration_engine, field))
        .collect::<Vec<_>>();

    TypedStructDeclaration {
        name: struct_declaration.name,
        type_parameters: struct_declaration.type_parameters,
        fields: typed_fields,
    }
}

fn analyze_struct_field(
    namespace: &mut Namespace,
    declaration_engine: &mut DeclarationEngine,
    struct_field: StructField,
) -> TypedStructField {
    TypedStructField {
        name: struct_field.name,
        type_id: eval_type(
            insert_type(struct_field.type_info),
            namespace,
            declaration_engine,
        )
        .unwrap(),
    }
}
