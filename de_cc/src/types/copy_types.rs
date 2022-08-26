use crate::type_system::type_mapping::TypeMapping;

pub(crate) trait CopyTypes {
    fn copy_types(&mut self, type_mapping: &TypeMapping);
}
