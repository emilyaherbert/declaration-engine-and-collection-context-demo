use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_edge::CollectionEdge,
        collection_index::CCIdx, collection_node::CollectionNode,
    },
    declaration_engine::{
        declaration_engine::{
            de_insert_function, de_insert_struct, de_insert_trait, de_insert_trait_fn,
            de_insert_trait_impl,
        },
        declaration_id::DeclarationId,
    },
    language::{
        parsed::{
            declaration::{
                Declaration, FunctionDeclaration, FunctionParameter, StructDeclaration,
                TraitDeclaration, TraitFn, TraitImpl, VariableDeclaration,
            },
            Node,
        },
        ty::typed_declaration::{
            TyCodeBlock, TyDeclaration, TyFunctionDeclaration, TyFunctionParameter,
            TyStructDeclaration, TyStructField, TyTraitDeclaration, TyTraitFn, TyTraitImpl,
            TyVariableDeclaration,
        },
    },
    type_system::type_engine::insert_type,
};

use super::{collect_graph_node, expression::collect_graph_exp};

pub(super) fn collect_graph_decl(
    cc: &mut CollectionContext,
    decl: Declaration,
) -> CCIdx<TyDeclaration> {
    match decl {
        Declaration::Variable(var_decl) => {
            let var_decl = collect_graph_var_decl(cc, var_decl);
            let decl = TyDeclaration::Variable(var_decl.clone());
            let decl_idx = cc.add_node(CollectionNode::Variable(var_decl.name.clone(), var_decl));
            CCIdx::new(decl, decl_idx)
        }
        Declaration::Function(func_decl) => {
            let func_decl_cc_idx = collect_graph_function(cc, func_decl);
            let decl = TyDeclaration::Function(func_decl_cc_idx.clone());
            CCIdx::new(decl, func_decl_cc_idx.idx())
        }
        Declaration::Trait(trait_decl) => {
            let trait_decl_cc_idx = collect_graph_trait(cc, trait_decl);
            let decl = TyDeclaration::Trait(trait_decl_cc_idx.clone());
            CCIdx::new(decl, trait_decl_cc_idx.idx())
        }
        Declaration::TraitImpl(trait_impl) => {
            let trait_impl_cc_idx = collect_graph_trait_impl(cc, trait_impl);
            let decl = TyDeclaration::TraitImpl(trait_impl_cc_idx.clone());
            CCIdx::new(decl, trait_impl_cc_idx.idx())
        }
        Declaration::Struct(struct_decl) => {
            let struct_decl_cc_idx = collect_graph_struct(cc, struct_decl);
            let decl = TyDeclaration::Struct(struct_decl_cc_idx.clone());
            CCIdx::new(decl, struct_decl_cc_idx.idx())
        }
    }
}

fn collect_graph_var_decl(
    cc: &mut CollectionContext,
    var_decl: VariableDeclaration,
) -> TyVariableDeclaration {
    TyVariableDeclaration {
        name: var_decl.name,
        body: collect_graph_exp(cc, var_decl.body),
        type_ascription: insert_type(var_decl.type_ascription),
    }
}

fn collect_graph_function(
    cc: &mut CollectionContext,
    func_decl: FunctionDeclaration,
) -> CCIdx<DeclarationId> {
    // collect the parameters
    let parameters = func_decl
        .parameters
        .into_iter()
        .map(collect_graph_function_parameter)
        .collect::<Vec<_>>();

    // collect the body
    let body = collect_graph_code_block(cc, func_decl.body);

    let func_decl = TyFunctionDeclaration {
        name: func_decl.name,
        type_parameters: func_decl.type_parameters,
        parameters,
        body,
        return_type: insert_type(func_decl.return_type),
    };

    // insert the function into the declaration engine
    let func_decl_id = de_insert_function(func_decl.clone());

    // add the function id to the graph
    let func_decl_idx = cc.add_node(CollectionNode::Function(func_decl.name, func_decl_id));

    // create an Idx for the function
    let func_decl_cc_idx = CCIdx::new(func_decl_id, func_decl_idx);

    // add an edge from body to the function declaration
    CCIdx::add_edge(
        &func_decl.body,
        &func_decl_cc_idx,
        CollectionEdge::ScopedChild,
        cc,
    );

    func_decl_cc_idx
}

fn collect_graph_code_block(cc: &mut CollectionContext, nodes: Vec<Node>) -> CCIdx<TyCodeBlock> {
    // collect the nodes
    let nodes = nodes
        .into_iter()
        .map(|node| collect_graph_node(cc, node))
        .collect::<Vec<_>>();

    // for every node in this scope, connect them under the same shared scope
    CCIdx::add_edges_many(&nodes, CollectionEdge::SharedScope, cc);

    let code_block = TyCodeBlock { contents: nodes };

    // add the code block to the graph
    let code_block_idx = cc.add_node(CollectionNode::CodeBlock(code_block.clone()));

    // create an Idx for the code block
    let code_block_cc_idx = CCIdx::new(code_block.clone(), code_block_idx);

    // add an edge from every node to the code block
    CCIdx::add_edges_many_to_one(
        &code_block.contents,
        &code_block_cc_idx,
        CollectionEdge::ScopedChild,
        cc,
    );

    code_block_cc_idx
}

fn collect_graph_function_parameter(function_parameter: FunctionParameter) -> TyFunctionParameter {
    TyFunctionParameter {
        name: function_parameter.name,
        type_id: insert_type(function_parameter.type_info),
    }
}

fn collect_graph_trait(
    cc: &mut CollectionContext,
    trait_decl: TraitDeclaration,
) -> CCIdx<DeclarationId> {
    // connect the interface surface
    let interface_surface = trait_decl
        .interface_surface
        .into_iter()
        .map(|trait_fn| collect_graph_trait_fn(cc, trait_fn))
        .collect::<Vec<_>>();

    let trait_decl = TyTraitDeclaration {
        name: trait_decl.name,
        interface_surface,
    };

    // insert the trait into the declaration engine
    let trait_decl_id = de_insert_trait(trait_decl.clone());

    // add the trait to the graph
    let trait_decl_idx = cc.add_node(CollectionNode::Trait(trait_decl.name, trait_decl_id));

    // create an Idx for the trait
    let trait_decl_cc_idx = CCIdx::new(trait_decl_id, trait_decl_idx);

    // connect every trait fn in the interface surface
    CCIdx::add_edges_many(
        &trait_decl.interface_surface,
        CollectionEdge::SharedScope,
        cc,
    );

    // connect every trait fn to the trait decl
    CCIdx::add_edges_many_to_one(
        &trait_decl.interface_surface,
        &trait_decl_cc_idx,
        CollectionEdge::ScopedChild,
        cc,
    );

    trait_decl_cc_idx
}

fn collect_graph_trait_fn(cc: &mut CollectionContext, trait_fn: TraitFn) -> CCIdx<DeclarationId> {
    // collect the parameters
    let parameters = trait_fn
        .parameters
        .into_iter()
        .map(collect_graph_function_parameter)
        .collect::<Vec<_>>();

    let trait_fn = TyTraitFn {
        name: trait_fn.name,
        parameters,
        return_type: insert_type(trait_fn.return_type),
    };

    // insert the trait fn into the declaration engine
    let trait_fn_id = de_insert_trait_fn(trait_fn.clone());

    // add the trait fn to the graph
    let trait_fn_idx = cc.add_node(CollectionNode::TraitFn(trait_fn.name, trait_fn_id));

    // create an Idx for the trait fn
    CCIdx::new(trait_fn_id, trait_fn_idx)
}

fn collect_graph_trait_impl(
    cc: &mut CollectionContext,
    trait_impl: TraitImpl,
) -> CCIdx<DeclarationId> {
    if !trait_impl.type_parameters.is_empty() {
        panic!()
    }

    // collect the methods
    let methods = trait_impl
        .methods
        .into_iter()
        .map(|method| collect_graph_function(cc, method))
        .collect::<Vec<_>>();

    let trait_impl = TyTraitImpl {
        trait_name: trait_impl.trait_name,
        type_implementing_for: insert_type(trait_impl.type_implementing_for),
        type_parameters: vec![],
        methods,
    };

    // insert the trait into the declaration engine
    let trait_impl_id = de_insert_trait_impl(trait_impl.clone());

    // add the trait to the graph
    let trait_impl_idx = cc.add_node(CollectionNode::TraitImpl(
        trait_impl.trait_name,
        trait_impl_id,
    ));

    // create an Idx for the trait
    let trait_impl_cc_idx = CCIdx::new(trait_impl_id, trait_impl_idx);

    // connect every method
    CCIdx::add_edges_many(&trait_impl.methods, CollectionEdge::SharedScope, cc);

    // connect every method to the trait impl
    CCIdx::add_edges_many_to_one(
        &trait_impl.methods,
        &trait_impl_cc_idx,
        CollectionEdge::ScopedChild,
        cc,
    );

    trait_impl_cc_idx
}

fn collect_graph_struct(
    cc: &mut CollectionContext,
    struct_decl: StructDeclaration,
) -> CCIdx<DeclarationId> {
    // collect the fields
    let fields = struct_decl
        .fields
        .into_iter()
        .map(|field| TyStructField {
            name: field.name,
            type_id: insert_type(field.type_info),
        })
        .collect::<Vec<_>>();

    let struct_decl = TyStructDeclaration {
        name: struct_decl.name,
        type_parameters: struct_decl.type_parameters,
        fields,
    };

    // insert the struct into the declaration engine
    let struct_decl_id = de_insert_struct(struct_decl.clone());

    // add the trait to the graph
    let struct_decl_idx = cc.add_node(CollectionNode::Struct(struct_decl.name, struct_decl_id));

    // create an Idx for the trait
    CCIdx::new(struct_decl_id, struct_decl_idx)
}
