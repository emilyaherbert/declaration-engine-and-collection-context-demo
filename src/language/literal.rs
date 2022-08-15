use crate::type_system::{IntegerBits, TypeInfo};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Literal {
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
