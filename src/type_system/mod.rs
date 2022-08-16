use std::fmt;

pub(crate) mod concurrent_slab;
pub(crate) mod resolved_type_info;
pub(crate) mod type_argument;
pub(crate) mod type_engine;
pub(crate) mod type_id;
pub mod type_info;
pub(crate) mod type_parameter;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum IntegerBits {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

impl fmt::Display for IntegerBits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntegerBits::Eight => write!(f, "u8"),
            IntegerBits::Sixteen => write!(f, "u16"),
            IntegerBits::ThirtyTwo => write!(f, "u32"),
            IntegerBits::SixtyFour => write!(f, "u64"),
        }
    }
}
