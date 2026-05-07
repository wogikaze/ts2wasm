use crate::layout::Layout;

/// Wasm linear-module representation of a JavaScript value in the current small-int pipeline: always
/// **i32** tagged bits (`ValueTag`). This is not interchangeable with logical ABI
/// `AbiType::JsVal` (`i64`); see `docs/14-runtime-abi.md` and `crates/shared::AbiType`.
pub type WasmTaggedJsWire = i32;

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
    /// Inclusive minimum payload representable in small-int tagged encoding.
    pub const NUMBER_PAYLOAD_MIN: i32 = i32::MIN >> Self::NUMBER_SHIFT;
    /// Inclusive maximum payload representable in small-int tagged encoding.
    pub const NUMBER_PAYLOAD_MAX: i32 = i32::MAX >> Self::NUMBER_SHIFT;

    /// Sentinel payload for NaN — uses a payload outside the normal representable
    /// range so it does not conflict with any small-int value.
    pub const NAN_PAYLOAD: i32 = (i32::MAX >> Self::NUMBER_SHIFT) + 1;
    /// Sentinel payload for +Infinity.
    pub const INFINITY_PAYLOAD: i32 = Self::NAN_PAYLOAD + 1;
    /// Sentinel payload for -Infinity.
    pub const NEG_INFINITY_PAYLOAD: i32 = Self::NAN_PAYLOAD + 2;
    /// Sentinel payload for -0 (negative zero).
    pub const NEG_ZERO_PAYLOAD: i32 = Self::NAN_PAYLOAD + 3;

    /// Encode a small signed integer as a tagged i32 value.
    pub fn encode_number(n: i32) -> WasmTaggedJsWire {
        (n << Self::NUMBER_SHIFT) | Self::NUMBER
    }

    /// Returns true when `n` can be represented by the small-int tagged payload.
    pub fn can_encode_number(n: i32) -> bool {
        (Self::NUMBER_PAYLOAD_MIN..=Self::NUMBER_PAYLOAD_MAX).contains(&n)
    }
}

// ---------------------------------------------------------------------------
// Typed wrappers for tagged values, heap pointers, and raw local values
// ---------------------------------------------------------------------------

/// A complete JavaScript value in the tagged i32 representation.
///
/// Low 3 bits hold the tag (see [`ValueTag`]); upper 29 bits hold either a
/// small-integer payload or a heap pointer.  Use this type instead of raw
/// `i32` / `WasmTaggedJsWire` to get compile-time protection against
/// accidentally mixing tagged values with heap pointers or raw offsets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaggedValue(i32);

impl TaggedValue {
    /// The `undefined` value.
    pub const UNDEFINED: Self = Self(0);
    /// The `null` value.
    pub const NULL: Self = Self(1);
    /// The `false` value.
    pub const FALSE: Self = Self(2);
    /// The `true` value.
    pub const TRUE: Self = Self(3);
    /// NaN sentinel — number-tagged reserved out-of-range payload.
    pub const NAN: Self =
        Self((ValueTag::NAN_PAYLOAD << ValueTag::NUMBER_SHIFT) | ValueTag::NUMBER);
    /// +Infinity sentinel.
    pub const INFINITY: Self =
        Self((ValueTag::INFINITY_PAYLOAD << ValueTag::NUMBER_SHIFT) | ValueTag::NUMBER);
    /// -Infinity sentinel.
    pub const NEG_INFINITY: Self =
        Self((ValueTag::NEG_INFINITY_PAYLOAD << ValueTag::NUMBER_SHIFT) | ValueTag::NUMBER);
    /// -0 (negative zero) sentinel.
    pub const NEG_ZERO: Self =
        Self((ValueTag::NEG_ZERO_PAYLOAD << ValueTag::NUMBER_SHIFT) | ValueTag::NUMBER);

    /// Encode a small signed integer as a tagged number value.
    ///
    /// This is the typed equivalent of [`ValueTag::encode_number`].
    ///
    /// # Panics
    ///
    /// Panics (via `debug_assert`) when `n` is outside the representable
    /// range [`ValueTag::NUMBER_PAYLOAD_MIN`, `ValueTag::NUMBER_PAYLOAD_MAX`].
    pub fn encode_number(n: i32) -> Self {
        debug_assert!(
            ValueTag::can_encode_number(n),
            "number {n} is outside the tagged-value payload range"
        );
        Self(ValueTag::encode_number(n))
    }

    /// Construct a tagged heap value from a heap pointer and a heap tag.
    ///
    /// `tag` must be `ValueTag::STRING`, `ValueTag::ARRAY`, or `ValueTag::OBJECT`.
    pub fn from_heap_ptr(ptr: HeapPtr, tag: i32) -> Self {
        debug_assert!(
            tag == ValueTag::STRING || tag == ValueTag::ARRAY || tag == ValueTag::OBJECT,
            "from_heap_ptr requires a heap tag (STRING/ARRAY/OBJECT), got {tag}"
        );
        Self(ptr.as_i32() | tag)
    }

    /// Return the raw i32 representation.
    ///
    /// Use this when the value must be passed to wasm-emission or other
    /// `i32`-based APIs that have not yet been migrated to the typed wrappers.
    pub fn as_i32(self) -> i32 {
        self.0
    }

    /// Extract the low-3-bit tag.
    pub fn tag(self) -> i32 {
        self.0 & ValueTag::TAG_MASK
    }

    /// Returns true when the value is heap-allocated (string, array, or object).
    pub fn is_heap_allocated(self) -> bool {
        let t = self.tag();
        t == ValueTag::STRING || t == ValueTag::ARRAY || t == ValueTag::OBJECT
    }

    /// Decode the small-integer payload.
    ///
    /// # Panics
    ///
    /// Panics (via `debug_assert`) when the tag is not [`ValueTag::NUMBER`].
    pub fn decode_number(self) -> i32 {
        debug_assert_eq!(
            self.tag(),
            ValueTag::NUMBER,
            "decode_number called on non-number tagged value"
        );
        self.0 >> ValueTag::NUMBER_SHIFT
    }

    /// Extract the heap pointer from a heap-allocated value.
    ///
    /// # Panics
    ///
    /// Panics (via `debug_assert`) when the value is not heap-allocated.
    pub fn heap_ptr(self) -> HeapPtr {
        debug_assert!(
            self.is_heap_allocated(),
            "heap_ptr called on non-heap value (tag={})",
            self.tag()
        );
        HeapPtr((self.0 & ValueTag::HEAP_MASK) as u32)
    }
}

impl From<TaggedValue> for i32 {
    fn from(v: TaggedValue) -> Self {
        v.0
    }
}

/// An 8-byte-aligned pointer into the wasm linear-memory heap.
///
/// Heap pointers are offsets into linear memory starting at
/// [`Layout::HEAP_START`].  They are always aligned to [`Layout::ALIGN`] (8
/// bytes).  Use this type instead of raw `i32`/`u32` to distinguish heap
/// addresses from tagged values at the type level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeapPtr(u32);

impl HeapPtr {
    /// The null/invalid heap pointer.
    pub const NULL: Self = Self(0);

    /// Create a `HeapPtr` from a raw offset, **without** alignment checking.
    ///
    /// Prefer [`HeapPtr::new`] when the alignment guarantee is available.
    pub const fn new_unchecked(ptr: u32) -> Self {
        Self(ptr)
    }

    /// Create a `HeapPtr` from a raw offset, verifying 8-byte alignment.
    ///
    /// # Panics
    ///
    /// Panics (via `debug_assert`) when `ptr` is not aligned to
    /// [`Layout::ALIGN`] bytes.
    pub fn new(ptr: u32) -> Self {
        debug_assert_eq!(
            ptr % Layout::ALIGN,
            0,
            "HeapPtr value {ptr:#x} is not aligned to {} bytes",
            Layout::ALIGN
        );
        Self(ptr)
    }

    /// Return the raw offset as `u32`.
    pub fn as_u32(self) -> u32 {
        self.0
    }

    /// Return the raw offset as `i32` (for wasm-emission code that expects
    /// `i32` operands).
    pub fn as_i32(self) -> i32 {
        self.0 as i32
    }

    /// Return the offset as a `usize` (for array-indexing into host memory).
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }

    /// Produce a tagged value by OR-ing this heap pointer with the given
    /// heap tag (`ValueTag::STRING`, `ValueTag::ARRAY`, or `ValueTag::OBJECT`).
    pub fn tag(self, tag: i32) -> TaggedValue {
        TaggedValue::from_heap_ptr(self, tag)
    }

    /// Add an offset to this heap pointer.
    ///
    /// The caller is responsible for ensuring the result stays within the
    /// allocated object boundary.
    pub fn offset(self, amount: u32) -> Self {
        Self(self.0 + amount)
    }
}

impl From<HeapPtr> for u32 {
    fn from(p: HeapPtr) -> Self {
        p.0
    }
}

impl From<HeapPtr> for i32 {
    fn from(p: HeapPtr) -> Self {
        p.0 as i32
    }
}

/// A raw `i32` value stored in a wasm local variable slot.
///
/// Every wasm local is an `i32`.  `LocalRawValue` wraps such a value before
/// it has been interpreted as a [`TaggedValue`], a [`HeapPtr`], or some other
/// typed representation.  Use this type at function-entry points and local
/// variable boundaries to make the interpretation step explicit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LocalRawValue(i32);

impl LocalRawValue {
    /// Wrap a raw `i32` local value.
    pub const fn new(val: i32) -> Self {
        Self(val)
    }

    /// Return the raw `i32`.
    pub fn as_i32(self) -> i32 {
        self.0
    }

    /// Interpret this raw value as a `TaggedValue`.
    ///
    /// This conversion is always valid because any `i32` bit pattern is a
    /// legal tagged value (the tag bits may decode to an unassigned tag, but
    /// the representation is well-defined).
    pub fn as_tagged(self) -> TaggedValue {
        TaggedValue(self.0)
    }
}

impl From<LocalRawValue> for i32 {
    fn from(v: LocalRawValue) -> Self {
        v.0
    }
}

#[cfg(test)]
mod typed_wrapper_tests {
    use super::*;

    #[test]
    fn tagged_value_constants() {
        assert_eq!(TaggedValue::UNDEFINED.as_i32(), 0);
        assert_eq!(TaggedValue::NULL.as_i32(), 1);
        assert_eq!(TaggedValue::FALSE.as_i32(), 2);
        assert_eq!(TaggedValue::TRUE.as_i32(), 3);
    }

    #[test]
    fn tagged_value_number_roundtrip() {
        for n in [
            0,
            1,
            -1,
            42,
            -100,
            ValueTag::NUMBER_PAYLOAD_MAX,
            ValueTag::NUMBER_PAYLOAD_MIN,
        ] {
            let tv = TaggedValue::encode_number(n);
            assert_eq!(tv.tag(), ValueTag::NUMBER);
            assert!(!tv.is_heap_allocated());
            assert_eq!(tv.decode_number(), n);
        }
    }

    #[test]
    fn tagged_value_heap_ptr_roundtrip() {
        let ptr = HeapPtr::new(2048);
        let tv = TaggedValue::from_heap_ptr(ptr, ValueTag::OBJECT);
        assert_eq!(tv.tag(), ValueTag::OBJECT);
        assert!(tv.is_heap_allocated());
        assert_eq!(tv.heap_ptr(), ptr);
    }

    #[test]
    fn tagged_value_from_i32_and_back() {
        let tv: TaggedValue = TaggedValue::UNDEFINED;
        let raw: i32 = tv.into();
        assert_eq!(raw, 0);
    }

    #[test]
    fn heap_ptr_alignment() {
        // 8-byte aligned pointers should be accepted
        let _ = HeapPtr::new(2048);
        let _ = HeapPtr::new(4096);
        HeapPtr::new_unchecked(0);

        // Test aligned value extraction
        let ptr = HeapPtr::new(4096);
        assert_eq!(ptr.as_u32(), 4096);
        assert_eq!(ptr.as_i32(), 4096);
        assert_eq!(ptr.as_usize(), 4096);
    }

    #[test]
    fn heap_ptr_offset() {
        let ptr = HeapPtr::new(2048);
        assert_eq!(ptr.offset(16), HeapPtr::new(2064));
    }

    #[test]
    fn heap_ptr_tag_produces_tagged_value() {
        let ptr = HeapPtr::new(4096);
        let tv = ptr.tag(ValueTag::STRING);
        assert_eq!(tv.tag(), ValueTag::STRING);
        assert!(tv.is_heap_allocated());
        assert_eq!(tv.heap_ptr(), ptr);
    }

    #[test]
    fn heap_ptr_conversions() {
        let ptr = HeapPtr::new(2048);
        assert_eq!(u32::from(ptr), 2048u32);
        assert_eq!(i32::from(ptr), 2048i32);
    }

    #[test]
    fn local_raw_value_roundtrip() {
        let raw = LocalRawValue::new(42);
        assert_eq!(raw.as_i32(), 42);
        let as_i32: i32 = raw.into();
        assert_eq!(as_i32, 42);
    }

    #[test]
    fn local_raw_value_as_tagged() {
        let raw = LocalRawValue::new(0);
        assert_eq!(raw.as_tagged(), TaggedValue::UNDEFINED);

        let raw = LocalRawValue::new((42 << 3) | 4);
        assert_eq!(raw.as_tagged(), TaggedValue::encode_number(42));
    }

    #[test]
    fn tagged_value_default_is_undefined() {
        assert_eq!(TaggedValue::default(), TaggedValue::UNDEFINED);
    }

    #[test]
    fn local_raw_value_default_is_zero() {
        assert_eq!(LocalRawValue::default(), LocalRawValue::new(0));
    }
}
