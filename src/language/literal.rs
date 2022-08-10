#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Literal {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}
