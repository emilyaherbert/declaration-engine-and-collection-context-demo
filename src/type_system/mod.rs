mod resolved_type_info;
mod type_argument;
mod type_id;
mod type_info;
mod type_parameter;

pub(crate) use resolved_type_info::*;
pub(crate) use type_argument::*;
pub(crate) use type_id::*;
pub(crate) use type_info::*;
pub(crate) use type_parameter::*;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub(crate) enum IntegerBits {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}
