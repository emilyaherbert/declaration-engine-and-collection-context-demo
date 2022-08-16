use std::fmt;

use crate::type_system::{type_info::TypeInfo, IntegerBits};

#[derive(Clone, Eq, PartialEq)]
pub enum Literal {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl Literal {
    pub(crate) fn to_type(&self) -> TypeInfo {
        let bits = match self {
            Literal::U8(_) => IntegerBits::Eight,
            Literal::U16(_) => IntegerBits::Sixteen,
            Literal::U32(_) => IntegerBits::ThirtyTwo,
            Literal::U64(_) => IntegerBits::SixtyFour,
        };
        TypeInfo::UnsignedInteger(bits)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::U8(value) => write!(f, "{}u8", value),
            Literal::U16(value) => write!(f, "{}u16", value),
            Literal::U32(value) => write!(f, "{}u32", value),
            Literal::U64(value) => write!(f, "{}u64", value),
        }
    }
}
