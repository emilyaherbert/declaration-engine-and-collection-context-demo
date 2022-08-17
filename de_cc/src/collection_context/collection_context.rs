use std::fmt;

use linked_hash_map::LinkedHashMap;

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
    functions: LinkedHashMap<String, Vec<(Path, FunctionInfo)>>,
    // structs: LinkedHashMap<String, Vec<(Path, StructInfo)>>,
    // enums: LinkedHashMap<String, Vec<(Path, EnumInfo)>>,
    // traits: LinkedHashMap<String, Vec<(Path, TraitInfo)>>,
    // trait_impls: LinkedHashMap<(String, String), Vec<TraitImplInfo>>,
    // self_impls: LinkedHashMap<String, Vec<SelfImplInfo>>,
}

impl fmt::Display for CollectionContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();

        builder.push_str("\n  functions:\n");
        for (name, values) in self.functions.iter() {
            for (path, _) in values.iter() {
                builder.push_str("\n    ");
                builder.push_str(name);
                builder.push_str(", ");
                builder.push_str(&path.to_string());
            }
        }

        // builder.push_str("\n\n  structs:\n");
        // for (name, values) in self.structs.iter() {
        //     for (path, _) in values.iter() {
        //         builder.push_str("\n    ");
        //         builder.push_str(name);
        //         builder.push_str(", ");
        //         builder.push_str(&path.to_string());
        //     }
        // }

        // builder.push_str("\n\n  enums:\n");
        // for (name, values) in self.enums.iter() {
        //     for (path, _) in values.iter() {
        //         builder.push_str("\n    ");
        //         builder.push_str(name);
        //         builder.push_str(", ");
        //         builder.push_str(&path.to_string());
        //     }
        // }

        // builder.push_str("\n\n  traits:\n");
        // for (name, values) in self.traits.iter() {
        //     for (path, _) in values.iter() {
        //         builder.push_str("\n    ");
        //         builder.push_str(name);
        //         builder.push_str(", ");
        //         builder.push_str(&path.to_string());
        //     }
        // }

        write!(f, "{}", builder)
    }
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

    pub(crate) fn get_function(&self, current_path: &Path, name: &str) -> Option<&FunctionInfo> {
        for (path, info) in self.functions.get(name)?.iter() {
            if path == current_path {
                return Some(info);
            }
        }
        None
    }

    pub(crate) fn get_struct(&self, _name: String) -> Option<&StructInfo> {
        unimplemented!();
        // self.structs.get(&name)
    }

    pub(crate) fn get_enum(&self, _name: String) -> Option<&EnumInfo> {
        unimplemented!();
        // self.enums.get(&name)
    }

    pub(crate) fn get_trait(&self, _name: String) -> Option<&TraitInfo> {
        unimplemented!();
        // self.traits.get(&name)
    }

    pub fn debug_print(&self) {
        println!(
            "\n\n~~~~~~~~~~\n\nCollection Context:\n{}\n\n~~~~~~~~~~",
            self
        );
    }
}
