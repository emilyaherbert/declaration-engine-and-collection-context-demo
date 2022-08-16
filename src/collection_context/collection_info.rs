use crate::{
    language::untyped::declaration::{EnumVariant, FunctionParameter, StructField},
    type_system::{type_info::TypeInfo, type_parameter::TypeParameter},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionInfo {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) return_type: TypeInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct StructInfo {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct EnumInfo {
    pub(crate) name: String,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TraitInfo {
    pub(crate) name: String,
    pub(crate) interface_surface: Vec<TraitFnInfo>,
    pub(crate) methods: Vec<FunctionInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TraitFnInfo {
    pub(crate) name: String,
    pub(crate) parameters: Vec<FunctionParameter>,
    pub(crate) return_type: TypeInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TraitImplInfo {
    pub(crate) trait_name: String,
    pub(crate) type_implementing_for: TypeInfo,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) functions: Vec<FunctionInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SelfImplInfo {
    pub(crate) type_implementing_for: TypeInfo,
    pub(crate) type_parameters: Vec<TypeParameter>,
    pub(crate) functions: Vec<FunctionInfo>,
}
