use crate::{
    collection_context::collection_context::CollectionContext,
    language::untyped::{declaration::Declaration, Application, File, Node},
};

pub(crate) fn collect(collection_context: &mut CollectionContext, application: &Application) {
    for program in application.programs.iter() {
        collect_program(collection_context, program);
    }
}

fn collect_program(collection_context: &mut CollectionContext, program: &File) {
    collect_nodes(collection_context, &program.nodes);
}

fn collect_nodes(collection_context: &mut CollectionContext, nodes: &[Node]) {
    nodes
        .iter()
        .for_each(|node| collect_node(collection_context, node));
}

fn collect_node(collection_context: &mut CollectionContext, node: &Node) {
    if let Node::Declaration(declaration) = node {
        collect_declaration(collection_context, declaration);
    }
}

fn collect_declaration(collection_context: &mut CollectionContext, declaration: &Declaration) {
    match declaration {
        Declaration::Variable(_) => {}
        Declaration::Function(function_declaration) => {
            let name = function_declaration.name.clone();
            collection_context.insert_function(name, function_declaration);
        }
        Declaration::Trait(_) => {
            unimplemented!();
            // let name = trait_declaration.name.clone();
            // collection_context.insert_trait(name, trait_declaration);
        }
        Declaration::Struct(_) => {
            unimplemented!();
            // let name = struct_declaration.name.clone();
            // collection_context.insert_struct(name, struct_declaration);
        }
        Declaration::Enum(_) => {
            unimplemented!();
            // let name = enum_declaration.name.clone();
            // collection_context.insert_enum(name, enum_declaration);
        }
        Declaration::TraitImpl(_) => unimplemented!(),
        Declaration::SelfImpl(_) => unimplemented!(),
    }
}
