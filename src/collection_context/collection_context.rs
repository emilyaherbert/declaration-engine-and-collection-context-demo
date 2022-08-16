use std::collections::HashMap;

use crate::{
    language::untyped::declaration::{
        EnumDeclaration, FunctionDeclaration, SelfImpl, StructDeclaration, TraitDeclaration,
        TraitImpl,
    },
    namespace::path::Path,
};

use super::collection_info::{EnumInfo, FunctionInfo, StructInfo, TraitInfo};

#[derive(Default)]
pub(crate) struct CollectionContext {
    functions: HashMap<String, Vec<(Path, FunctionInfo)>>,
    structs: HashMap<String, Vec<(Path, StructInfo)>>,
    enums: HashMap<String, Vec<(Path, EnumInfo)>>,
    traits: HashMap<String, Vec<(Path, TraitInfo)>>,
    // trait_impls: HashMap<(String, String), Vec<TraitImplInfo>>,
    // self_impls: HashMap<String, Vec<SelfImplInfo>>,
}

impl CollectionContext {
    pub(crate) fn insert_function(
        &mut self,
        current_path: Path,
        name: String,
        function: &FunctionDeclaration,
    ) {
        let function_info = FunctionInfo {
            name: function.name.clone(),
            type_parameters: function.type_parameters.clone(),
            parameters: function.parameters.clone(),
            return_type: function.return_type.clone(),
        };
        match self.functions.get_mut(&name) {
            Some(prev) => {
                prev.push((current_path, function_info));
            }
            None => {
                self.functions
                    .insert(name, vec![(current_path, function_info)]);
            }
        }
    }

    pub(crate) fn insert_struct(&mut self, name: String, r#struct: &StructDeclaration) {
        unimplemented!();
        // let struct_info = StructInfo {
        //     name: r#struct.name.clone(),
        //     type_parameters: r#struct.type_parameters.clone(),
        //     fields: r#struct.fields.clone(),
        // };
        // self.structs.insert(name, struct_info);
    }

    pub(crate) fn insert_enum(&mut self, name: String, r#enum: &EnumDeclaration) {
        unimplemented!();
        // let enum_info = EnumInfo {
        //     name: r#enum.name.clone(),
        //     type_parameters: r#enum.type_parameters.clone(),
        //     variants: r#enum.variants.clone(),
        // };
        // self.enums.insert(name, enum_info);
    }

    pub(crate) fn insert_trait(&mut self, name: String, r#trait: &TraitDeclaration) {
        unimplemented!();
        // let interface_surface = r#trait
        //     .interface_surface
        //     .iter()
        //     .map(|trait_fn| TraitFnInfo {
        //         name: trait_fn.name.clone(),
        //         parameters: trait_fn.parameters.clone(),
        //         return_type: trait_fn.return_type.clone(),
        //     })
        //     .collect::<Vec<_>>();
        // let methods = r#trait
        //     .methods
        //     .iter()
        //     .map(|method| FunctionInfo {
        //         name: method.name.clone(),
        //         type_parameters: method.type_parameters.clone(),
        //         parameters: method.parameters.clone(),
        //         return_type: method.return_type.clone(),
        //     })
        //     .collect::<Vec<_>>();
        // let trait_info = TraitInfo {
        //     name: r#trait.name.clone(),
        //     interface_surface,
        //     methods,
        // };
        // self.traits.insert(name, trait_info);
    }

    pub(crate) fn insert_trait_impl(
        &mut self,
        trait_name: String,
        type_implementing_for: String,
        trait_impl: &TraitImpl,
    ) {
        unimplemented!();
        // let functions = trait_impl
        //     .functions
        //     .iter()
        //     .map(|function| FunctionInfo {
        //         name: function.name.clone(),
        //         type_parameters: function.type_parameters.clone(),
        //         parameters: function.parameters.clone(),
        //         return_type: function.return_type.clone(),
        //     })
        //     .collect::<Vec<_>>();
        // let trait_impl_info = TraitImplInfo {
        //     trait_name: trait_impl.trait_name.clone(),
        //     type_implementing_for: trait_impl.type_implementing_for.clone(),
        //     type_parameters: trait_impl.type_parameters.clone(),
        //     functions,
        // };
        // match self
        //     .trait_impls
        //     .remove(&(trait_name.clone(), type_implementing_for.clone()))
        // {
        //     Some(mut list) => {
        //         list.push(trait_impl_info);
        //         self.trait_impls
        //             .insert((trait_name, type_implementing_for), list);
        //     }
        //     None => {
        //         self.trait_impls
        //             .insert((trait_name, type_implementing_for), vec![trait_impl_info]);
        //     }
        // }
    }

    pub(crate) fn insert_self_impl(&mut self, type_implementing_for: String, self_impl: &SelfImpl) {
        unimplemented!();
        // let functions = self_impl
        // .functions
        // .iter()
        // .map(|function| FunctionInfo {
        //     name: function.name.clone(),
        //     type_parameters: function.type_parameters.clone(),
        //     parameters: function.parameters.clone(),
        //     return_type: function.return_type.clone(),
        // })
        // .collect::<Vec<_>>();
        // let self_impl_info = SelfImplInfo {
        //     type_implementing_for: self_impl.type_implementing_for.clone(),
        //     type_parameters: self_impl.type_parameters.clone(),
        //     functions,
        // };
        // match self
        //     .self_impls
        //     .remove(&type_implementing_for)
        // {
        //     Some(mut list) => {
        //         list.push(self_impl_info);
        //         self.self_impls
        //             .insert(type_implementing_for.clone(), list);
        //     }
        //     None => {
        //         self.self_impls
        //             .insert(type_implementing_for.clone(), vec![self_impl_info]);
        //     }
        // }
    }

    pub(crate) fn get_function(
        &mut self,
        current_path: Path,
        name: String,
    ) -> Option<&FunctionInfo> {
        for (path, info) in self.functions.get(&name)?.iter() {
            if path == &current_path {
                return Some(info);
            }
        }
        None
    }

    pub(crate) fn get_struct(&mut self, _name: String) -> Option<&StructInfo> {
        unimplemented!();
        // self.structs.get(&name)
    }

    pub(crate) fn get_enum(&mut self, _name: String) -> Option<&EnumInfo> {
        unimplemented!();
        // self.enums.get(&name)
    }

    pub(crate) fn get_trait(&mut self, _name: String) -> Option<&TraitInfo> {
        unimplemented!();
        // self.traits.get(&name)
    }
}
