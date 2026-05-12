//! ABI invariant tests for `ts2wasm-runtime-abi`.
//!
//! These integration tests verify that ABI constants, tag values, layout
//! offsets, and other invariants remain consistent.  They serve as a
//! regression guardrail whenever the ABI is modified.

use ts2wasm_runtime_abi::{
    HeapPtr, Layout, LocalRawValue, RuntimeConst, RuntimeString, StackEffect, TaggedValue, ValueTag,
};

// ---------------------------------------------------------------------------
// ValueTag invariants
// ---------------------------------------------------------------------------

#[test]
fn tag_values_are_unique() {
    let tags = [
        ValueTag::UNDEFINED,
        ValueTag::NULL,
        ValueTag::FALSE,
        ValueTag::TRUE,
        ValueTag::NUMBER,
        ValueTag::ARRAY,
        ValueTag::STRING,
        ValueTag::OBJECT,
    ];
    let mut sorted = tags.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        tags.len(),
        "ValueTag constants are not all unique: {tags:?}"
    );
}

#[test]
fn tag_values_are_consecutive() {
    // Tags 0..=7 should be assigned in order.
    assert_eq!(ValueTag::UNDEFINED, 0);
    assert_eq!(ValueTag::NULL, 1);
    assert_eq!(ValueTag::FALSE, 2);
    assert_eq!(ValueTag::TRUE, 3);
    assert_eq!(ValueTag::NUMBER, 4);
    assert_eq!(ValueTag::ARRAY, 5);
    assert_eq!(ValueTag::STRING, 6);
    assert_eq!(ValueTag::OBJECT, 7);
}

#[test]
fn heap_tags_are_the_high_tags() {
    // STRING=6, ARRAY=5, OBJECT=7 are the only heap-allocated tags.
    // All three must have their top bit (bit 2 of TAG_MASK) set.
    for t in [ValueTag::STRING, ValueTag::ARRAY, ValueTag::OBJECT] {
        assert!(t & 4 != 0, "heap tag {} must have bit 2 set", t);
    }
}

#[test]
fn non_heap_tags_excluding_number_clear_bit_2() {
    // NUMBER (tag 4 = 0b100) necessarily has bit 2 set.
    for t in [
        ValueTag::UNDEFINED,
        ValueTag::NULL,
        ValueTag::FALSE,
        ValueTag::TRUE,
    ] {
        assert!(t & 4 == 0, "non-heap tag {} must clear bit 2", t);
    }
}

#[test]
fn tag_mask_is_correct() {
    assert_eq!(ValueTag::TAG_MASK, 0b111);
}

#[test]
fn heap_mask_clears_low_3_bits() {
    // HEAP_MASK = -8 = !0x7
    assert_eq!(ValueTag::HEAP_MASK, -8);
    // Verify it clears the tag bits
    for val in [0i32, 1, 2, 3, 4, 5, 6, 7] {
        assert_eq!(val & ValueTag::HEAP_MASK, val & !0b111);
    }
}

#[test]
fn number_shift_aligns_tag_and_payload() {
    // NUMBER_SHIFT=3: low 3 bits = tag, upper 29 bits = payload.
    assert_eq!(ValueTag::NUMBER_SHIFT, 3);
    let encoded = ValueTag::encode_number(42);
    assert_eq!(encoded & 0b111, ValueTag::NUMBER);
    assert_eq!(encoded >> 3, 42);
}

// ---------------------------------------------------------------------------
// Sentinel payload invariants
// ---------------------------------------------------------------------------

#[test]
fn sentinel_payloads_are_distinct_and_ordered() {
    use std::collections::BTreeSet;

    let sentinels = [
        ValueTag::NAN_PAYLOAD,
        ValueTag::INFINITY_PAYLOAD,
        ValueTag::NEG_INFINITY_PAYLOAD,
        ValueTag::NEG_ZERO_PAYLOAD,
    ];

    let set: BTreeSet<i32> = sentinels.iter().copied().collect();
    assert_eq!(
        set.len(),
        sentinels.len(),
        "sentinel payloads are not distinct: {sentinels:?}"
    );

    // NAN_PAYLOAD must be outside the encodeable range: > NUMBER_PAYLOAD_MAX.
    assert!(
        ValueTag::NAN_PAYLOAD > ValueTag::NUMBER_PAYLOAD_MAX,
        "NAN_PAYLOAD ({}) must exceed NUMBER_PAYLOAD_MAX ({})",
        ValueTag::NAN_PAYLOAD,
        ValueTag::NUMBER_PAYLOAD_MAX
    );

    // Sentinels should be consecutive (NAN, INF, NEG_INF, NEG_ZERO).
    let expected = [
        ValueTag::NUMBER_PAYLOAD_MAX + 1,
        ValueTag::NUMBER_PAYLOAD_MAX + 2,
        ValueTag::NUMBER_PAYLOAD_MAX + 3,
        ValueTag::NUMBER_PAYLOAD_MAX + 4,
    ];
    assert_eq!(sentinels, expected, "sentinel payloads must be consecutive");
}

// ---------------------------------------------------------------------------
// TaggedValue constant invariants
// ---------------------------------------------------------------------------

#[test]
fn tagged_value_sentinels_match_value_tag() {
    // Use logical (unsigned) right-shift to extract the payload bits,
    // because the sentinel payloads when shifted left overflow the
    // signed i32 range and arithmetic right-shift would sign-extend.

    let nan = TaggedValue::NAN;
    assert_eq!(nan.tag(), ValueTag::NUMBER);
    assert_eq!(
        (nan.as_i32() as u32 >> ValueTag::NUMBER_SHIFT) as i32,
        ValueTag::NAN_PAYLOAD
    );

    let inf = TaggedValue::INFINITY;
    assert_eq!(inf.tag(), ValueTag::NUMBER);
    assert_eq!(
        (inf.as_i32() as u32 >> ValueTag::NUMBER_SHIFT) as i32,
        ValueTag::INFINITY_PAYLOAD
    );

    let neg_inf = TaggedValue::NEG_INFINITY;
    assert_eq!(neg_inf.tag(), ValueTag::NUMBER);
    assert_eq!(
        (neg_inf.as_i32() as u32 >> ValueTag::NUMBER_SHIFT) as i32,
        ValueTag::NEG_INFINITY_PAYLOAD
    );

    let neg_zero = TaggedValue::NEG_ZERO;
    assert_eq!(neg_zero.tag(), ValueTag::NUMBER);
    assert_eq!(
        (neg_zero.as_i32() as u32 >> ValueTag::NUMBER_SHIFT) as i32,
        ValueTag::NEG_ZERO_PAYLOAD
    );
}

// ---------------------------------------------------------------------------
// HeapPtr invariants
// ---------------------------------------------------------------------------

#[test]
fn heap_ptr_null_is_zero() {
    assert_eq!(HeapPtr::NULL, HeapPtr::new_unchecked(0));
    assert_eq!(HeapPtr::NULL.as_u32(), 0);
}

#[test]
fn heap_ptr_tag_roundtrip_for_all_heap_tags() {
    for tag in [ValueTag::STRING, ValueTag::ARRAY, ValueTag::OBJECT] {
        let ptr = HeapPtr::new_unchecked(4096);
        let tv = ptr.tag(tag);
        assert_eq!(tv.tag(), tag);
        assert!(tv.is_heap_allocated());
        assert_eq!(tv.heap_ptr(), ptr);
    }
}

// ---------------------------------------------------------------------------
// LocalRawValue invariants
// ---------------------------------------------------------------------------

#[test]
fn local_raw_value_zero_is_undefined() {
    let raw = LocalRawValue::new(0);
    assert_eq!(raw.as_tagged(), TaggedValue::UNDEFINED);
}

#[test]
fn local_raw_value_identity_for_all_tags() {
    for tag_val in [0i32, 1, 2, 3, 4, 5, 6, 7] {
        let raw = LocalRawValue::new(tag_val);
        assert_eq!(raw.as_tagged().tag(), tag_val);
    }
}

// ---------------------------------------------------------------------------
// StackEffect invariants
// ---------------------------------------------------------------------------

#[test]
fn stack_effect_take_one_return_one() {
    let se = StackEffect::take_one_return_one();
    assert_eq!(se.params, 1);
    assert_eq!(se.results, 1);
}

#[test]
fn stack_effect_property_get() {
    let se = StackEffect::property_get();
    assert_eq!(se.params, 3);
    assert_eq!(se.results, 1);
}

#[test]
fn stack_effect_property_set() {
    let se = StackEffect::property_set();
    assert_eq!(se.params, 4);
    assert_eq!(se.results, 1);
}

// ---------------------------------------------------------------------------
// RuntimeConst invariants
// ---------------------------------------------------------------------------

#[test]
fn runtime_const_abi_version_is_one() {
    assert_eq!(RuntimeConst::ABI_VERSION, 1);
}

#[test]
fn runtime_const_bool_values_are_correct() {
    assert_eq!(RuntimeConst::TRUE, 1);
    assert_eq!(RuntimeConst::FALSE, 0);
}

#[test]
fn runtime_const_ascii_values_are_correct() {
    assert_eq!(RuntimeConst::ASCII_ZERO, b'0' as i32);
    assert_eq!(RuntimeConst::ASCII_MINUS, b'-' as i32);
}

#[test]
fn runtime_const_stdout_fd_is_one() {
    assert_eq!(RuntimeConst::STDOUT_FD, 1);
}

// ---------------------------------------------------------------------------
// RuntimeString invariants
// ---------------------------------------------------------------------------

#[test]
fn runtime_string_undefined_is_undefined() {
    assert_eq!(RuntimeString::UNDEFINED, "undefined");
}

#[test]
fn runtime_string_null_is_null() {
    assert_eq!(RuntimeString::NULL, "null");
}

#[test]
fn runtime_string_bool_values() {
    assert_eq!(RuntimeString::TRUE, "true");
    assert_eq!(RuntimeString::FALSE, "false");
}

#[test]
fn runtime_string_newline() {
    assert_eq!(RuntimeString::NEWLINE, "\n");
}

// ---------------------------------------------------------------------------
// Layout: array header offset invariants
// ---------------------------------------------------------------------------

#[test]
fn array_header_offsets_are_within_header_size() {
    // All offsets must be strictly less than ARRAY_HEADER_SIZE (20).
    assert!(Layout::ARRAY_CAPACITY_OFFSET < Layout::ARRAY_HEADER_SIZE);
    assert!(Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET < Layout::ARRAY_HEADER_SIZE);
    assert!(Layout::ARRAY_ELEMENTS_OFFSET_OFFSET < Layout::ARRAY_HEADER_SIZE);
    assert!(Layout::ARRAY_PRESENCE_WORDS_OFFSET < Layout::ARRAY_HEADER_SIZE);
}

#[test]
fn array_header_offsets_are_4_byte_aligned() {
    assert_eq!(Layout::ARRAY_CAPACITY_OFFSET % 4, 0);
    assert_eq!(Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET % 4, 0);
    assert_eq!(Layout::ARRAY_ELEMENTS_OFFSET_OFFSET % 4, 0);
    assert_eq!(Layout::ARRAY_PRESENCE_WORDS_OFFSET % 4, 0);
}

#[test]
fn array_header_offset_order() {
    // Offsets must be strictly increasing.
    assert!(Layout::ARRAY_CAPACITY_OFFSET > 0);
    assert!(Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET > Layout::ARRAY_CAPACITY_OFFSET);
    assert!(Layout::ARRAY_ELEMENTS_OFFSET_OFFSET > Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET);
    assert!(Layout::ARRAY_PRESENCE_WORDS_OFFSET > Layout::ARRAY_ELEMENTS_OFFSET_OFFSET);
}

// ---------------------------------------------------------------------------
// Layout: object header offset invariants
// ---------------------------------------------------------------------------

#[test]
fn object_header_offsets_are_within_header_size() {
    assert!(Layout::OBJECT_FLAGS_OFFSET < Layout::OBJECT_HEADER_SIZE);
    assert!(Layout::OBJECT_PROTOTYPE_OFFSET < Layout::OBJECT_HEADER_SIZE);
}

#[test]
fn object_header_offsets_are_4_byte_aligned() {
    assert_eq!(Layout::OBJECT_FLAGS_OFFSET % 4, 0);
    assert_eq!(Layout::OBJECT_PROTOTYPE_OFFSET % 4, 0);
    assert_eq!(Layout::OBJECT_ENTRIES_OFFSET % 4, 0);
}

#[test]
fn object_entry_fields_are_4_byte_aligned() {
    assert_eq!(Layout::OBJECT_ENTRY_SIZE, 8);
    assert_eq!(Layout::OBJECT_VALUE_OFFSET, 4);
    // OBJECT_ENTRY_SHIFT=3 means 2^3 = 8 bytes per entry.
    assert_eq!(1 << Layout::OBJECT_ENTRY_SHIFT, Layout::OBJECT_ENTRY_SIZE);
}

// ---------------------------------------------------------------------------
// Layout: GC kind invariants
// ---------------------------------------------------------------------------

#[test]
fn gc_kind_values_are_distinct() {
    use std::collections::HashSet;
    let kinds = [
        Layout::GC_KIND_UNKNOWN,
        Layout::GC_KIND_STRING,
        Layout::GC_KIND_ARRAY,
        Layout::GC_KIND_OBJECT,
        Layout::GC_KIND_BIGINT,
    ];
    let set: HashSet<u32> = kinds.iter().copied().collect();
    assert_eq!(set.len(), kinds.len(), "GC kind values are not distinct");
}

#[test]
fn gc_kind_mask_captures_all_kinds() {
    let kinds = [
        Layout::GC_KIND_UNKNOWN,
        Layout::GC_KIND_STRING,
        Layout::GC_KIND_ARRAY,
        Layout::GC_KIND_OBJECT,
        Layout::GC_KIND_BIGINT,
    ];
    for k in kinds {
        assert_eq!(
            k & Layout::GC_KIND_MASK,
            k,
            "GC kind {k} must be captured by GC_KIND_MASK ({})",
            Layout::GC_KIND_MASK
        );
    }
}

// ---------------------------------------------------------------------------
// Layout: BigInt payload offset invariants
// ---------------------------------------------------------------------------

#[test]
fn bigint_offsets_are_4_byte_aligned() {
    assert_eq!(Layout::BIGINT_SIGN_OFFSET % 4, 0);
    assert_eq!(Layout::BIGINT_LIMB_COUNT_OFFSET % 4, 0);
    assert_eq!(Layout::BIGINT_LIMB0_LOW_OFFSET % 4, 0);
    assert_eq!(Layout::BIGINT_LIMB0_HIGH_OFFSET % 4, 0);
    assert_eq!(Layout::BIGINT_DECIMAL_LEN_OFFSET % 4, 0);
    assert_eq!(Layout::BIGINT_DECIMAL_DATA_OFFSET % 4, 0);
}

#[test]
fn bigint_offsets_are_strictly_increasing() {
    assert!(Layout::BIGINT_SIGN_OFFSET < Layout::BIGINT_LIMB_COUNT_OFFSET);
    assert!(Layout::BIGINT_LIMB_COUNT_OFFSET < Layout::BIGINT_LIMB0_LOW_OFFSET);
    assert!(Layout::BIGINT_LIMB0_LOW_OFFSET < Layout::BIGINT_LIMB0_HIGH_OFFSET);
    assert!(Layout::BIGINT_LIMB0_HIGH_OFFSET < Layout::BIGINT_DECIMAL_LEN_OFFSET);
    assert!(Layout::BIGINT_DECIMAL_LEN_OFFSET < Layout::BIGINT_DECIMAL_DATA_OFFSET);
}

// ---------------------------------------------------------------------------
// Layout: module cache invariants
// ---------------------------------------------------------------------------

#[test]
fn module_cache_entry_size_is_reasonable() {
    assert_eq!(Layout::MODULE_CACHE_ENTRY_SIZE, 8);
    assert!(Layout::MODULE_CACHE_MAX > 0);
}

// ---------------------------------------------------------------------------
// WasmTaggedJsWire type alias
// ---------------------------------------------------------------------------

#[test]
fn wasm_tagged_wire_is_i32() {
    // Verify the type alias resolves correctly at runtime.
    use ts2wasm_runtime_abi::WasmTaggedJsWire;
    let v: WasmTaggedJsWire = TaggedValue::encode_number(100).as_i32();
    assert_eq!(v, (100 << 3) | 4);
    let _: WasmTaggedJsWire = 0i32; // type-level verification
}
