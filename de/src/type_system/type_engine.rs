use crate::{
    concurrent_slab::ConcurrentSlab, declaration_engine::declaration_engine::DeclarationEngine,
    language::typed::typed_declaration::TypedDeclaration, namespace::namespace::Namespace,
    types::copy_types::CopyTypes,
};

use super::{
    resolved_types::ResolvedType, type_argument::TypeArgument, type_id::TypeId,
    type_info::TypeInfo, type_mapping::insert_type_parameters, type_parameter::TypeParameter,
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
    fn insert_type(&self, ty: TypeInfo) -> TypeId {
        self.slab.insert(ty)
    }

    fn look_up_type_id_raw(&self, id: TypeId) -> TypeInfo {
        self.slab.get(id)
    }

    fn look_up_type_id(&self, id: TypeId) -> TypeInfo {
        match self.slab.get(id) {
            TypeInfo::Ref(other) => self.look_up_type_id(other),
            ty => ty,
        }
    }

    fn unify_types(&self, received: TypeId, expected: TypeId) -> Result<(), String> {
        match (self.slab.get(received), self.slab.get(expected)) {
            // if the two types are the same literal then we are done
            (TypeInfo::Unit, TypeInfo::Unit) => Ok(()),
            (TypeInfo::UnsignedInteger(a), TypeInfo::UnsignedInteger(b)) if a == b => Ok(()),

            // if either of the types are unknown
            (TypeInfo::Unknown, _) => {
                match self
                    .slab
                    .replace(received, &TypeInfo::Unknown, TypeInfo::Ref(expected))
                {
                    None => Ok(()),
                    Some(_) => self.unify_types(received, expected),
                }
            }
            (_, TypeInfo::Unknown) => {
                match self
                    .slab
                    .replace(expected, &TypeInfo::Unknown, TypeInfo::Ref(received))
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
                    .replace(received, received_info, TypeInfo::Ref(expected));
                Ok(())
            }

            (_, ref expected_info @ TypeInfo::UnknownGeneric { .. }) => {
                self.slab
                    .replace(expected, expected_info, TypeInfo::Ref(received));
                Ok(())
            }

            // (
            //     TypeInfo::Struct {
            //         name: a_name,
            //         fields: a_fields,
            //         type_parameters: a_parameters,
            //     },
            //     TypeInfo::Struct {
            //         name: b_name,
            //         fields: b_fields,
            //         type_parameters: b_parameters,
            //     },
            // ) => {
            //     if a_name != b_name
            //         || a_fields.len() != b_fields.len()
            //         || a_parameters.len() != b_parameters.len()
            //     {
            //         return Err("type mismatch".to_string());
            //     }
            //     for (a_field, b_field) in a_fields.iter().zip(b_fields.iter()) {
            //         self.unify_types(a_field.type_id, b_field.type_id)?;
            //     }
            //     for (a_param, b_param) in a_parameters.iter().zip(b_parameters.iter()) {
            //         self.unify_types(a_param.type_id, b_param.type_id)?;
            //     }
            //     Ok(())
            // }

            // (
            //     TypeInfo::Enum {
            //         name: a_name,
            //         variant_types: a_variants,
            //         type_parameters: a_parameters,
            //     },
            //     TypeInfo::Enum {
            //         name: b_name,
            //         variant_types: b_variants,
            //         type_parameters: b_parameters,
            //     },
            // ) => {
            //     if a_name != b_name
            //         || a_variants.len() != b_variants.len()
            //         || a_parameters.len() != b_parameters.len()
            //     {
            //         return Err("type mismatch".to_string());
            //     }
            //     for (a_variant, b_variant) in a_variants.iter().zip(b_variants.iter()) {
            //         self.unify_types(a_variant.type_id, b_variant.type_id)?;
            //     }
            //     for (a_param, b_param) in a_parameters.iter().zip(b_parameters.iter()) {
            //         self.unify_types(a_param.type_id, b_param.type_id)?;
            //     }
            //     Ok(())
            // }
            (received_info, expected_info) => Err(format!(
                "type mismatch, expected: {}, received: {}",
                expected_info, received_info
            )),
        }
    }

    fn resolve_type(
        &self,
        declaration_engine: &DeclarationEngine,
        type_id: TypeId,
    ) -> Result<ResolvedType, String> {
        match self.slab.get(type_id) {
            TypeInfo::UnsignedInteger(bits) => Ok(ResolvedType::UnsignedInteger(bits)),
            TypeInfo::Ref(id) => self.resolve_type(declaration_engine, id),
            TypeInfo::Unit => Ok(ResolvedType::Unit),
            found => Err(format!("type error, found: {}", found)),
            // TypeInfo::Enum {
            //     name,
            //     type_parameters,
            //     variant_types,
            // } => {
            //     let type_parameters = type_parameters
            //         .into_iter()
            //         .map(|type_parameter| {
            //             Ok(ResolvedTypeParameter {
            //                 name_ident: type_parameter.name_ident,
            //                 type_info: self
            //                     .resolve_type(declaration_engine, type_parameter.type_id)?,
            //             })
            //         })
            //         .collect::<Result<_, String>>()?;
            //     let variant_types = variant_types
            //         .into_iter()
            //         .map(|variant_type| {
            //             Ok(ResolvedEnumVariant {
            //                 name: variant_type.name,
            //                 type_info: self
            //                     .resolve_type(declaration_engine, variant_type.type_id)?,
            //                 tag: variant_type.tag,
            //             })
            //         })
            //         .collect::<Result<_, String>>()?;
            //     Ok(ResolvedType::Enum {
            //         name,
            //         type_parameters,
            //         variant_types,
            //     })
            // }
            // TypeInfo::Struct { .. } => todo!(),
        }
    }

    fn eval_type(&self, id: TypeId, namespace: &Namespace) -> Result<TypeId, String> {
        match self.slab.get(id) {
            TypeInfo::UnknownGeneric { name } => match namespace.get_symbol(&name)? {
                TypedDeclaration::GenericTypeForFunctionScope { type_id, .. } => {
                    Ok(insert_type(TypeInfo::Ref(type_id)))
                }
                _ => Err("could not find generic declaration".to_string()),
            },
            TypeInfo::Ref(id) => Ok(id),
            o => Ok(insert_type(o)),
        }
    }

    fn monomorphize<T>(
        &self,
        value: &mut T,
        type_arguments: &mut [TypeArgument],
        namespace: &Namespace,
    ) -> Result<(), String>
    where
        T: MonomorphizeHelper + CopyTypes,
    {
        match (
            value.type_parameters().is_empty(),
            type_arguments.is_empty(),
        ) {
            (true, true) => Ok(()),
            (false, true) => {
                let type_mapping = insert_type_parameters(value.type_parameters());
                value.copy_types(&type_mapping);
                Ok(())
            }
            (true, false) => Err("does not take type arguments".to_string()),
            (false, false) => {
                if value.type_parameters().len() != type_arguments.len() {
                    return Err("incorrect number of type arguments".to_string());
                }
                for type_argument in type_arguments.iter_mut() {
                    type_argument.type_id = self.eval_type(type_argument.type_id, namespace)?;
                }
                let type_mapping = insert_type_parameters(value.type_parameters());
                for ((_, interim_type), type_argument) in
                    type_mapping.iter().zip(type_arguments.iter())
                {
                    self.unify_types(*interim_type, type_argument.type_id)?;
                }
                value.copy_types(&type_mapping);
                Ok(())
            }
        }
    }
}

pub(crate) fn insert_type(ty: TypeInfo) -> TypeId {
    TYPE_ENGINE.insert_type(ty)
}

pub(crate) fn look_up_type_id(id: TypeId) -> TypeInfo {
    TYPE_ENGINE.look_up_type_id(id)
}

pub(crate) fn look_up_type_id_raw(id: TypeId) -> TypeInfo {
    TYPE_ENGINE.look_up_type_id_raw(id)
}

pub(crate) fn unify_types(received: TypeId, expected: TypeId) -> Result<(), String> {
    TYPE_ENGINE.unify_types(received, expected)
}

pub(crate) fn resolve_type(
    declaration_engine: &DeclarationEngine,
    type_id: TypeId,
) -> Result<ResolvedType, String> {
    TYPE_ENGINE.resolve_type(declaration_engine, type_id)
}

pub(crate) fn eval_type(id: TypeId, namespace: &Namespace) -> Result<TypeId, String> {
    TYPE_ENGINE.eval_type(id, namespace)
}

pub(crate) fn monomorphize<T>(
    value: &mut T,
    type_arguments: &mut [TypeArgument],
    namespace: &Namespace,
) -> Result<(), String>
where
    T: MonomorphizeHelper + CopyTypes,
{
    TYPE_ENGINE.monomorphize(value, type_arguments, namespace)
}

pub(crate) trait MonomorphizeHelper {
    fn name(&self) -> &str;
    fn type_parameters(&self) -> &[TypeParameter];
}
