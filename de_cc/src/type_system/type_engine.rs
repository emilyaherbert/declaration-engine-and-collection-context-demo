use crate::{
    concurrent_slab::ConcurrentSlab,
    declaration_engine::declaration_engine::*,
    language::{
        resolved::resolved_declaration::ResolvedStructField, ty::typed_declaration::TyDeclaration,
    },
    namespace::namespace::Namespace,
    types::{copy_types::CopyTypes, create_type_id::CreateTypeId},
};

use super::{
    resolved_types::{ResolvedType, ResolvedTypeParameter},
    type_argument::TypeArgument,
    type_id::TypeId,
    type_info::TypeInfo,
    type_mapping::insert_type_parameters,
    type_parameter::TypeParameter,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref TYPE_ENGINE: TypeEngine = TypeEngine::default();
}

#[derive(Default)]
struct TypeEngine {
    slab: ConcurrentSlab<TypeInfo>,
}

impl TypeEngine {
    #[allow(dead_code)]
    fn debug_print(&self) {
        self.slab.debug_print();
    }

    fn insert_type(&self, ty: TypeInfo) -> TypeId {
        TypeId::new(self.slab.insert(ty))
    }

    fn look_up_type_id(&self, id: TypeId) -> TypeInfo {
        match self.slab.get(*id) {
            TypeInfo::Ref(other) => self.look_up_type_id(other),
            ty => ty,
        }
    }

    fn unify_types(&self, received: TypeId, expected: TypeId) -> Result<(), String> {
        match (self.slab.get(*received), self.slab.get(*expected)) {
            // if the two types are the same literal then we are done
            (TypeInfo::Unit, TypeInfo::Unit) => Ok(()),
            (TypeInfo::UnsignedInteger(a), TypeInfo::UnsignedInteger(b)) if a == b => Ok(()),

            // if either of the types are unknown
            (TypeInfo::Unknown, _) => {
                match self
                    .slab
                    .replace(*received, &TypeInfo::Unknown, TypeInfo::Ref(expected))
                {
                    None => Ok(()),
                    Some(_) => self.unify_types(received, expected),
                }
            }
            (_, TypeInfo::Unknown) => {
                match self
                    .slab
                    .replace(*expected, &TypeInfo::Unknown, TypeInfo::Ref(received))
                {
                    None => Ok(()),
                    Some(_) => self.unify_types(received, expected),
                }
            }

            // follow any references
            (TypeInfo::Ref(received), TypeInfo::Ref(expected)) if received == expected => Ok(()),
            (TypeInfo::Ref(received), _) => self.unify_types(received, expected),
            (_, TypeInfo::Ref(expected)) => self.unify_types(received, expected),

            (
                TypeInfo::UnknownGeneric { name: l_name },
                TypeInfo::UnknownGeneric { name: r_name },
            ) if l_name.as_str() == r_name.as_str() => Ok(()),
            (ref received_info @ TypeInfo::UnknownGeneric { .. }, _) => {
                self.slab
                    .replace(*received, received_info, TypeInfo::Ref(expected));
                Ok(())
            }
            (_, ref expected_info @ TypeInfo::UnknownGeneric { .. }) => {
                self.slab
                    .replace(*expected, expected_info, TypeInfo::Ref(received));
                Ok(())
            }

            (
                TypeInfo::Struct {
                    name: a_name,
                    fields: a_fields,
                    type_parameters: a_parameters,
                },
                TypeInfo::Struct {
                    name: b_name,
                    fields: b_fields,
                    type_parameters: b_parameters,
                },
            ) => {
                if a_name != b_name
                    || a_fields.len() != b_fields.len()
                    || a_parameters.len() != b_parameters.len()
                {
                    return Err("type mismatch".to_string());
                }
                for (a_field, b_field) in a_fields.iter().zip(b_fields.iter()) {
                    self.unify_types(a_field.type_id, b_field.type_id)?;
                }
                for (a_param, b_param) in a_parameters.iter().zip(b_parameters.iter()) {
                    self.unify_types(a_param.type_id, b_param.type_id)?;
                }
                Ok(())
            }

            (received_info, expected_info) => Err(format!(
                "type mismatch, expected: {}, received: {}",
                expected_info, received_info
            )),
        }
    }

    fn resolve_type(&self, type_id: TypeId) -> Result<ResolvedType, String> {
        match self.slab.get(*type_id) {
            TypeInfo::UnsignedInteger(bits) => Ok(ResolvedType::UnsignedInteger(bits)),
            TypeInfo::Ref(id) => self.resolve_type(id),
            TypeInfo::Unit => Ok(ResolvedType::Unit),
            TypeInfo::Struct {
                name,
                type_parameters,
                fields,
            } => {
                let type_parameters = type_parameters
                    .into_iter()
                    .map(|type_parameter| {
                        Ok(ResolvedTypeParameter {
                            type_info: self.resolve_type(type_parameter.type_id)?,
                        })
                    })
                    .collect::<Result<_, String>>()?;
                let fields = fields
                    .into_iter()
                    .map(|field| {
                        Ok(ResolvedStructField {
                            name: field.name,
                            type_info: self.resolve_type(field.type_id)?,
                        })
                    })
                    .collect::<Result<_, String>>()?;
                Ok(ResolvedType::Struct {
                    name,
                    type_parameters,
                    fields,
                })
            }
            found @ TypeInfo::ErrorRecovery
            | found @ TypeInfo::Unknown
            | found @ TypeInfo::UnknownGeneric { .. }
            | found @ TypeInfo::Custom { .. } => {
                Err(format!("type error in resolution, found: {}", found))
            }
        }
    }

    fn resolve_custom_types(&self, id: TypeId, namespace: &mut Namespace) -> Result<(), String> {
        match self.slab.get(*id) {
            TypeInfo::Ref(inner_id) => resolve_custom_types(inner_id, namespace),
            TypeInfo::Custom { name } => {
                match namespace.get_symbol(&name)? {
                    TyDeclaration::Struct(decl_id) => {
                        let decl_id = decl_id.inner();

                        // get the original struct declaration
                        let mut struct_decl = de_get_struct(decl_id)?;

                        // monomorphize the struct declaration into a new copy
                        self.monomorphize(&mut struct_decl, &[])?;

                        // add the new copy to the declaration engine
                        de_add_monomorphized_struct_copy(decl_id, struct_decl.clone());

                        // recreate the previous type info
                        let prev_info = TypeInfo::Custom { name };

                        // get the new type info
                        let new_info = self.look_up_type_id(struct_decl.create_type_id());

                        // replace the id with the new type info
                        self.slab.replace(*id, &prev_info, new_info);

                        Ok(())
                    }
                    got => Err(format!("err, found: {}", got)),
                }
            }
            _ => Ok(()),
        }
    }

    fn monomorphize<T>(&self, value: &mut T, type_arguments: &[TypeArgument]) -> Result<(), String>
    where
        T: MonomorphizeHelper + CopyTypes,
    {
        match (
            value.type_parameters().is_empty(),
            type_arguments.is_empty(),
        ) {
            (true, true) => Ok(()),
            (false, true) => {
                let type_mapping = insert_type_parameters(value.type_parameters().to_vec());
                value.copy_types(&type_mapping);
                Ok(())
            }
            (true, false) => Err("does not take type arguments".to_string()),
            (false, false) => {
                if value.type_parameters().len() != type_arguments.len() {
                    return Err("incorrect number of type arguments".to_string());
                }
                let type_mapping = insert_type_parameters(value.type_parameters().to_vec());
                for ((_, interim_type), type_arg) in type_mapping.iter().zip(type_arguments.iter())
                {
                    self.unify_types(*interim_type, type_arg.type_id)?;
                }
                value.copy_types(&type_mapping);
                Ok(())
            }
        }
    }
}

#[allow(dead_code)]
pub(crate) fn te_debug_print() {
    TYPE_ENGINE.debug_print()
}

pub(crate) fn insert_type(ty: TypeInfo) -> TypeId {
    TYPE_ENGINE.insert_type(ty)
}

pub(crate) fn look_up_type_id(id: TypeId) -> TypeInfo {
    TYPE_ENGINE.look_up_type_id(id)
}

pub(crate) fn unify_types(received: TypeId, expected: TypeId) -> Result<(), String> {
    TYPE_ENGINE.unify_types(received, expected)
}

pub(crate) fn resolve_type(type_id: TypeId) -> Result<ResolvedType, String> {
    TYPE_ENGINE.resolve_type(type_id)
}

pub(crate) fn resolve_custom_types(id: TypeId, namespace: &mut Namespace) -> Result<(), String> {
    TYPE_ENGINE.resolve_custom_types(id, namespace)
}

pub(crate) fn monomorphize<T>(value: &mut T, type_arguments: &[TypeArgument]) -> Result<(), String>
where
    T: MonomorphizeHelper + CopyTypes,
{
    TYPE_ENGINE.monomorphize(value, type_arguments)
}

pub(crate) trait MonomorphizeHelper {
    fn name(&self) -> &str;
    fn type_parameters(&self) -> &[TypeParameter];
    fn type_parameters_mut(&mut self) -> &mut [TypeParameter];
}
