use crate::{
    concurrent_slab::ConcurrentSlab, language::typed::typed_declaration::TypedFunctionDeclaration,
    types::pretty_print::PrettyPrint,
};

use super::{declaration_id::DeclarationId, declaration_wrapper::DeclarationWrapper};

// TODO: will need to use concurrent structure like https://github.com/xacrimon/dashmaps
#[derive(Default)]
pub(crate) struct DeclarationEngine {
    slab: ConcurrentSlab<DeclarationWrapper>,
}

impl DeclarationEngine {
    pub(crate) fn look_up_decl_id(&self, index: DeclarationId) -> DeclarationWrapper {
        self.slab.get(index)
    }

    pub(crate) fn insert_function(&self, function: TypedFunctionDeclaration) -> DeclarationId {
        self.slab.insert(DeclarationWrapper::Function(function))
    }

    pub(crate) fn get_function(
        &self,
        index: DeclarationId,
    ) -> Result<TypedFunctionDeclaration, String> {
        self.slab.get(index).expect_function()
    }

    pub fn debug_print(&self) {
        println!(
            "\n\n~~~~~~~~~~\n\nDeclaration Engine:\n{}\n\n~~~~~~~~~~",
            self.slab.pretty_print(self)
        );
    }
}
