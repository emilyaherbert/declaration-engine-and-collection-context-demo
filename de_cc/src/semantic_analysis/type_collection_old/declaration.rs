use crate::{
    declaration_engine::{
        declaration_engine::{
            de_insert, de_insert_struct, de_insert_trait, de_insert_trait_fn, de_insert_trait_impl, de_get_function,
        },
        declaration_wrapper::DeclarationWrapper,
    },
    language::{
        partial::{
            partial_declaration::{PartialDeclaration, PartialFunctionDeclaration},
        },
        typed::{typed_declaration::{
            TypedFunctionParameter, TypedStructDeclaration, TypedStructField,
            TypedTraitDeclaration, TypedTraitFn, TypedTraitImpl, TypedDeclaration, TypedFunctionDeclaration,
        }, TypedNode},
        typing_context::function::TyFunctionContext,
    },
    namespace::namespace::Namespace,
    type_system::type_engine::{eval_type2, insert_type},
};

use super::collect_types_node;

pub(super) fn collect_types_declaration(
    namespace: &mut Namespace,
    declaration: &TypedDeclaration,
) {
    match declaration {
        TypedDeclaration::Variable(decl) => {},
        TypedDeclaration::Function(decl_id) => {
            let function_declaration = de_get_function(*decl_id).unwrap();
            collect_types_function(&mut namespace.scoped(), &function_declaration);
            let decl_id = de_insert(DeclarationWrapper::Function(TyFunctionContext::partial(
                function_declaration,
            )));
            unimplemented!()
        }
        TypedDeclaration::Trait(trait_declaration) => {
            let trait_declaration = collect_types_trait(&mut namespace.scoped(), trait_declaration);
            let decl_id = de_insert_trait(trait_declaration);
            unimplemented!()
        }
        TypedDeclaration::TraitImpl(trait_impl) => {
            let trait_impl = collect_types_trait_impl(&mut namespace.scoped(), trait_impl);
            unimplemented!()
        }
        TypedDeclaration::Struct(struct_declaration) => {
            let struct_declaration =
                collect_types_struct(&mut namespace.scoped(), struct_declaration);
            let name = struct_declaration.name.clone();
            let decl = PartialDeclaration::Struct(de_insert_struct(struct_declaration));
            namespace.insert_symbol(name, decl.clone());
            unimplemented!()
        }
        TypedDeclaration::GenericTypeForFunctionScope { type_id } => todo!(),
    }
}

fn collect_types_function(
    namespace: &mut Namespace,
    function_declaration: &TypedFunctionDeclaration,
) {
    // insert type params into namespace
    for type_parameter in function_declaration.type_parameters.iter() {
        let type_parameter_decl = PartialDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);
    }

    // type check the function params
    let parameters = function_declaration
        .parameters
        .into_iter()
        .map(|param| collect_types_function_parameter(namespace, param))
        .collect::<Vec<_>>();

    // type check the function return type
    let return_type = eval_type2(insert_type(function_declaration.return_type), namespace).unwrap();

    PartialFunctionDeclaration {
        name: function_declaration.name,
        type_parameters: function_declaration.type_parameters,
        parameters,
        body: collect_types_code_block(namespace, function_declaration.body),
        return_type,
    }
}

fn collect_types_function_parameter(
    namespace: &mut Namespace,
    function_parameter: &TypedFunctionParameter,
) {
    TypedFunctionParameter {
        name: function_parameter.name,
        type_id: eval_type2(insert_type(function_parameter.type_info), namespace).unwrap(),
    }
}

fn collect_types_code_block(
    namespace: &mut Namespace,
    nodes: &[TypedNode],
) {
    nodes
        .into_iter()
        .map(|node| collect_types_node(namespace, node))
        .collect()
}

fn collect_types_trait(
    namespace: &mut Namespace,
    trait_declaration: &TypedTraitDeclaration,
) {
    let interface_surface = trait_declaration
        .interface_surface
        .into_iter()
        .map(|trait_fn| {
            let trait_fn = collect_types_trait_fn(namespace, trait_fn);
            de_insert_trait_fn(trait_fn)
        })
        .collect::<Vec<_>>();
        unimplemented!()
}

fn collect_types_trait_fn(namespace: &mut Namespace, trait_fn: &TypedTraitFn) {
    let parameters = trait_fn
        .parameters
        .into_iter()
        .map(|param| collect_types_function_parameter(namespace, param))
        .collect::<Vec<_>>();
    let return_type = eval_type2(insert_type(trait_fn.return_type), namespace).unwrap();
    unimplemented!()
}

fn collect_types_trait_impl(
    namespace: &mut Namespace,
    trait_impl: &TypedTraitImpl,
) {
    if !trait_impl.type_parameters.is_empty() {
        panic!()
    }

    // TODO: get the trait from the declaration engine,
    // check to see if all of the methods are implementing, no new methods implementing,
    // when generic traits are implemented add the monomorphized copies to the declaration
    // engine

    // type check the methods
    let methods = trait_impl
        .methods
        .into_iter()
        .map(|method| {
            let method = collect_types_function(namespace, method);
            de_insert(DeclarationWrapper::Function(TyFunctionContext::partial(
                method,
            )))
        })
        .collect::<Vec<_>>();

    let type_implementing_for = eval_type2(insert_type(trait_impl.type_implementing_for), namespace)
    .unwrap();

    unimplemented!()
}

fn collect_types_struct(
    namespace: &mut Namespace,
    struct_declaration: &TypedStructDeclaration,
) {
    // insert type params into namespace
    for type_parameter in struct_declaration.type_parameters.iter() {
        let type_parameter_decl = PartialDeclaration::GenericTypeForFunctionScope {
            type_id: type_parameter.type_id,
        };
        namespace.insert_symbol(type_parameter.name.clone(), type_parameter_decl);
    }

    // type check the fields
    let fields = struct_declaration
        .fields
        .into_iter()
        .map(|field| collect_types_struct_field(namespace, field))
        .collect::<Vec<_>>();

    unimplemented!()
}

fn collect_types_struct_field(
    namespace: &mut Namespace,
    struct_field: &TypedStructField,
) {
    let type_id = eval_type2(insert_type(struct_field.type_info), namespace).unwrap();

    unimplemented!()
}
