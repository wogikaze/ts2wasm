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
    /// Tag for heap-allocated array values (ptr | 5).
    pub const ARRAY: i32 = 5;
    pub const STRING: i32 = 6;
    /// Tag for heap-allocated object values (ptr | 7).
    pub const OBJECT: i32 = 7;
    /// Low-3-bit tag for heap-allocated string values.  `u32` so it can be
    /// OR-ed directly with `u32` heap offsets stored in the string table.
    pub const STRING_TAG: u32 = Self::STRING as u32;
    /// Low-3-bit tag for heap-allocated array values.  `u32` for OR-ing.
    pub const ARRAY_TAG: u32 = Self::ARRAY as u32;
    /// Low-3-bit tag for heap-allocated object values.  `u32` for OR-ing.
    pub const OBJECT_TAG: u32 = Self::OBJECT as u32;
    pub const TAG_MASK: i32 = 7;
    pub const HEAP_MASK: i32 = -8;
    /// Right-shift / left-shift amount used to pack/unpack the number payload.
    pub const NUMBER_SHIFT: i32 = 3;
    /// Inclusive minimum payload representable in M0 tagged-int encoding.
    pub const NUMBER_PAYLOAD_MIN: i32 = i32::MIN >> Self::NUMBER_SHIFT;
    /// Inclusive maximum payload representable in M0 tagged-int encoding.
    pub const NUMBER_PAYLOAD_MAX: i32 = i32::MAX >> Self::NUMBER_SHIFT;

    /// Encode a small signed integer as a tagged i32 value.
    pub fn encode_number(n: i32) -> i32 {
        (n << Self::NUMBER_SHIFT) | Self::NUMBER
    }

    /// Returns true when `n` can be represented by the M0 tagged-int payload.
    pub fn can_encode_number(n: i32) -> bool {
        (Self::NUMBER_PAYLOAD_MIN..=Self::NUMBER_PAYLOAD_MAX).contains(&n)
    }
}
