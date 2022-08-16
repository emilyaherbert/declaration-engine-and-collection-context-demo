use super::{concurrent_slab::ConcurrentSlab, type_id::TypeId, type_info::TypeInfo, IntegerBits};

use lazy_static::lazy_static;

lazy_static! {
    static ref TYPE_ENGINE: TypeEngine = TypeEngine::default();
}

#[derive(Debug, Default)]
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

    fn unify(&self, received: TypeId, expected: TypeId) -> Result<(), String> {
        match (self.slab.get(received), self.slab.get(expected)) {
            // if the two types are the same literal then we are done
            (TypeInfo::UnsignedInteger(a), TypeInfo::UnsignedInteger(b)) => match (a, b) {
                (IntegerBits::Eight, IntegerBits::Eight) => Ok(()),
                (IntegerBits::Sixteen, IntegerBits::Sixteen) => Ok(()),
                (IntegerBits::ThirtyTwo, IntegerBits::ThirtyTwo) => Ok(()),
                (IntegerBits::SixtyFour, IntegerBits::SixtyFour) => Ok(()),
                _ => Err("type mismatch".to_string()),
            },

            // if either of the types are unknown
            (TypeInfo::Unknown, _) => {
                match self
                    .slab
                    .replace(received, &TypeInfo::Unknown, TypeInfo::Ref(expected))
                {
                    None => Ok(()),
                    Some(_) => self.unify(received, expected),
                }
            }
            (_, TypeInfo::Unknown) => {
                match self
                    .slab
                    .replace(expected, &TypeInfo::Unknown, TypeInfo::Ref(received))
                {
                    None => Ok(()),
                    Some(_) => self.unify(received, expected),
                }
            }

            // follow any references
            (TypeInfo::Ref(received), TypeInfo::Ref(expected)) if received == expected => Ok(()),
            (TypeInfo::Ref(received), _) => self.unify(received, expected),
            (_, TypeInfo::Ref(expected)) => self.unify(received, expected),

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
                    self.unify(a_field.type_id, b_field.type_id)?;
                }
                for (a_param, b_param) in a_parameters.iter().zip(b_parameters.iter()) {
                    self.unify(a_param.type_id, b_param.type_id)?;
                }
                Ok(())
            }

            (
                TypeInfo::Enum {
                    name: a_name,
                    variant_types: a_variants,
                    type_parameters: a_parameters,
                },
                TypeInfo::Enum {
                    name: b_name,
                    variant_types: b_variants,
                    type_parameters: b_parameters,
                },
            ) => {
                if a_name != b_name
                    || a_variants.len() != b_variants.len()
                    || a_parameters.len() != b_parameters.len()
                {
                    return Err("type mismatch".to_string());
                }
                for (a_variant, b_variant) in a_variants.iter().zip(b_variants.iter()) {
                    self.unify(a_variant.type_id, b_variant.type_id)?;
                }
                for (a_param, b_param) in a_parameters.iter().zip(b_parameters.iter()) {
                    self.unify(a_param.type_id, b_param.type_id)?;
                }
                Ok(())
            }

            (TypeInfo::DeclarationRef(_), _) => Err("not done yet".to_string()),
            _ => Err("type mismatch".to_string()),
        }
    }
}

pub(crate) fn insert_type(ty: TypeInfo) -> TypeId {
    TYPE_ENGINE.insert_type(ty)
}

pub(crate) fn look_up_type_id(id: TypeId) -> TypeInfo {
    TYPE_ENGINE.look_up_type_id(id)
}

pub(crate) fn unify(a: TypeId, b: TypeId) -> Result<(), String> {
    TYPE_ENGINE.unify(a, b)
}
