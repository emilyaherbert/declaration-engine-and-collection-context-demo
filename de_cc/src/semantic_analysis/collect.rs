use crate::{
    language::untyped::{declaration::Declaration, Application, File, Node},
    namespace::namespace::Namespace,
    type_system::{type_engine::insert_type, type_info::TypeInfo},
};

pub(crate) fn collect(namespace: &mut Namespace, application: &Application) {
    application
        .files
        .iter()
        .for_each(|program| collect_file(namespace, program));
}

fn collect_file(namespace: &mut Namespace, file: &File) {
    file.nodes
        .iter()
        .for_each(|node| collect_node(namespace, node));
}

fn collect_node(namespace: &mut Namespace, node: &Node) {
    if let Node::Declaration(declaration) = node {
        collect_declaration(namespace, declaration);
    }
}

pub(super) fn collect_declaration(namespace: &mut Namespace, declaration: &Declaration) {
    if let Declaration::Struct(struct_declaration) = declaration {
        let type_id = insert_type(TypeInfo::Unknown);
        namespace.insert_into_collection_context(&struct_declaration.name, type_id);
    }
}
