pub(crate) mod concurrent_slab;
pub(crate) mod resolved_type_info;
pub(crate) mod type_argument;
pub(crate) mod type_engine;
pub(crate) mod type_id;
pub(crate) mod type_info;
pub(crate) mod type_parameter;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum IntegerBits {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}
