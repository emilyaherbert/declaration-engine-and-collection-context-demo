use crate::{
    declaration_engine::declaration_engine::*,
    language::untyped::{declaration::Declaration, Application, File, Node},
};

pub(crate) fn collect_types(application: &Application) {
    application.files.iter().for_each(collect_types_file);
}

fn collect_types_file(file: &File) {
    file.nodes.iter().for_each(collect_types_node);
}

fn collect_types_node(node: &Node) {
    if let Node::Declaration(declaration) = node {
        collect_types_declaration(declaration);
    }
}

pub(super) fn collect_types_declaration(declaration: &Declaration) {
    match declaration {
        Declaration::Variable(_) => todo!(),
        Declaration::Function(_) => todo!(),
        Declaration::Trait(_) => todo!(),
        Declaration::TraitImpl(_) => todo!(),
        Declaration::Struct(_) => todo!(),
    }
}
