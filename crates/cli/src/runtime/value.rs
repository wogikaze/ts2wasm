/// Value tag encoding for the `i32` tagged-value representation.
///
/// Encoding summary:
/// ```text
/// undefined : UNDEFINED (0)
/// null      : NULL (1)
/// false     : FALSE (2)
/// true      : TRUE (3)
/// number    : (n << NUMBER_SHIFT) | NUMBER   (tag 4, payload in upper bits)
/// string    : heap_ptr | STRING_TAG          (tag 6, pointer is 8-byte aligned)
/// ```
///
/// Tag bits occupy the low 3 bits of the i32.
pub struct ValueTag;

impl ValueTag {
    pub const UNDEFINED: i32 = 0;
    pub const NULL: i32 = 1;
    pub const FALSE: i32 = 2;
    pub const TRUE: i32 = 3;
    pub const NUMBER: i32 = 4;
    /// Low-3-bit tag for heap-allocated string values.  `u32` so it can be
    /// OR-ed directly with `u32` heap offsets stored in the string table.
    pub const STRING_TAG: u32 = 6;
    pub const TAG_MASK: i32 = 7;
    pub const HEAP_MASK: i32 = -8;
    /// Right-shift / left-shift amount used to pack/unpack the number payload.
    pub const NUMBER_SHIFT: u32 = 3;

    /// Encode a small signed integer as a tagged i32 value.
    pub fn encode_number(n: i32) -> i32 {
        (n << Self::NUMBER_SHIFT) | Self::NUMBER
    }
}
