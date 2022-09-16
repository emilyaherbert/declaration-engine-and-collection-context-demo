use crate::{
    collection_context::{collection_context::CollectionContext, collection_index::CCIdx},
    declaration_engine::{
        declaration_engine::{
            de_get_function, de_get_struct, de_get_trait, de_get_trait_fn, de_get_trait_impl,
        },
        declaration_id::DeclarationId,
    },
    language::ty::typed_declaration::TyDeclaration,
    type_system::{type_engine::resolve_custom_types, type_mapping::insert_type_parameters},
    types::copy_types::CopyTypes,
};

pub(super) fn collect_types_declaration(cc: &CollectionContext, decl: &mut CCIdx<TyDeclaration>) {
    match decl.inner_ref_mut() {
        TyDeclaration::Variable(_) => {}
        TyDeclaration::Function(decl_id) => collect_types_function(cc, decl_id),
        TyDeclaration::Trait(decl_id) => collect_types_trait(cc, decl_id),
        TyDeclaration::TraitImpl(decl_id) => collect_types_trait_impl(cc, decl_id),
        TyDeclaration::Struct(decl_id) => collect_types_struct(cc, decl_id),
    }
}

fn collect_types_function(cc: &CollectionContext, decl_id: &mut CCIdx<DeclarationId>) {
    let mut func_decl = de_get_function(*decl_id.inner_ref()).unwrap();

    // create type mapping
    let type_mapping = insert_type_parameters(func_decl.type_parameters.clone());

    // resolve any custom types in the parameters and
    // insert the type parameters into the ns
    for param in func_decl.parameters.iter_mut() {
        resolve_custom_types(param.type_id, cc, decl_id.idx()).unwrap();
        param.copy_types(&type_mapping);
    }

    // resolve any custom types in the function return type
    resolve_custom_types(func_decl.return_type, cc, decl_id.idx()).unwrap();
    func_decl.return_type.copy_types(&type_mapping);
}

fn collect_types_trait_impl(cc: &CollectionContext, decl_id: &mut CCIdx<DeclarationId>) {
    let mut trait_impl = de_get_trait_impl(*decl_id.inner_ref()).unwrap();

    if !trait_impl.type_parameters.is_empty() {
        panic!("no type parameters yet");
    }

    // create type mapping
    let type_mapping = insert_type_parameters(trait_impl.type_parameters.clone());

    // get the trait from the declaration engine
    let trait_id = cc
        .get_symbol(decl_id.idx(), &trait_impl.trait_name)
        .unwrap()
        .inner();
    let _trait_decl = de_get_trait(trait_id).unwrap();

    // resolve any custom types in the type we are implementing for
    resolve_custom_types(trait_impl.type_implementing_for, cc, decl_id.idx()).unwrap();
    trait_impl.type_implementing_for.copy_types(&type_mapping);

    // TODO: check to see if all of the methods are implementing, no new methods implementing,
    // when generic traits are implemented add the monomorphized copies to the declaration
    // engine

    // do type inference on the methods
    trait_impl.methods.iter_mut().for_each(|method_id| {
        collect_types_function(cc, method_id);
        method_id.copy_types(&type_mapping);
    });
}

fn collect_types_struct(cc: &CollectionContext, decl_id: &mut CCIdx<DeclarationId>) {
    let mut struct_decl = de_get_struct(*decl_id.inner_ref()).unwrap();

    // create type mapping
    let type_mapping = insert_type_parameters(struct_decl.type_parameters.clone());

    // do type inference on the fields
    struct_decl.fields.iter_mut().for_each(|field| {
        resolve_custom_types(field.type_id, cc, decl_id.idx()).unwrap();
        field.copy_types(&type_mapping);
    });
}

fn collect_types_trait(cc: &CollectionContext, decl_id: &mut CCIdx<DeclarationId>) {
    let mut trait_decl = de_get_trait(*decl_id.inner_ref()).unwrap();

    // do type inference on the interface
    trait_decl
        .interface_surface
        .iter_mut()
        .for_each(|trait_fn_id| collect_types_trait_fn(cc, trait_fn_id));
}

fn collect_types_trait_fn(cc: &CollectionContext, decl_id: &mut CCIdx<DeclarationId>) {
    let trait_fn = de_get_trait_fn(*decl_id.inner_ref()).unwrap();

    // resolve any custom types in the parameters
    for parameter in trait_fn.parameters.iter() {
        resolve_custom_types(parameter.type_id, cc, decl_id.idx()).unwrap();
    }

    // resolve any custom types in the return type
    resolve_custom_types(trait_fn.return_type, cc, decl_id.idx()).unwrap();
}
