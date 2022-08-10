use super::{concurrent_slab::ConcurrentSlab, TypeId, TypeInfo};

#[derive(Debug, Default)]
pub(crate) struct TypeEngine {
    slab: ConcurrentSlab<TypeInfo>,
}

impl TypeEngine {
    pub fn insert_type(&self, ty: TypeInfo) -> TypeId {
        self.slab.insert(ty)
    }

    pub fn look_up_type_id_raw(&self, id: TypeId) -> TypeInfo {
        self.slab.get(id)
    }

    pub fn look_up_type_id(&self, id: TypeId) -> TypeInfo {
        match self.slab.get(id) {
            TypeInfo::Ref(other) => self.look_up_type_id(other),
            ty => ty,
        }
    }
}
