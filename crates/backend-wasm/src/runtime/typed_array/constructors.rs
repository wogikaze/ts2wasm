use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_typed_array_from_array(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $typed_array_from_array (param $source i32) (result i32)
    (local $tag i32)
    (local $source_base i32)
    (local $len_value i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $source) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $source_base (i32.and (local.get $source) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $source_base)))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (call $index
            (local.get $source)
            (i32.or
              (i32.shl (local.get $i) (i32.const {number_shift}))
              (i32.const {number_tag}))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (local.get $elem))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_typed_array_ctor_with_length(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $typed_array_ctor_with_length (param $len_tagged i32) (result i32)
    (local $len i32) (local $num_words i32) (local $elem_off i32)
    (local $size i32) (local $ptr i32) (local $i i32)
    (local.set $len (i32.shr_s (local.get $len_tagged) (i32.const {number_shift})))
    ;; presence_word_count = ceil(len / 32)
    (local.set $num_words
      (i32.shr_u (i32.add (local.get $len) (i32.const 31)) (i32.const 5)))
    ;; elements_offset = ARRAY_HEADER_SIZE + max(0, num_words - 1) * 4
    (local.set $elem_off (i32.const {array_header}))
    (if (i32.gt_u (local.get $num_words) (i32.const 1))
      (then
        (local.set $elem_off
          (i32.add
            (i32.const {array_header})
            (i32.shl (i32.sub (local.get $num_words) (i32.const 1)) (i32.const 2))))))
    ;; size = elements_offset + len * 4
    (local.set $size
      (i32.add (local.get $elem_off) (i32.shl (local.get $len) (i32.const 2))))
    (local.set $ptr (call $alloc_heap (local.get $size)))
    ;; Header fields
    (i32.store (local.get $ptr) (local.get $len))
    (i32.store (i32.add (local.get $ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $ptr) (i32.const 8)) (local.get $num_words))
    (i32.store (i32.add (local.get $ptr) (i32.const 12)) (local.get $elem_off))
    ;; Presence bits: set all words to all-ones (every slot is present/initialized)
    (local.set $i (i32.const 0))
    (block $presence_done
      (loop $presence_loop
        (br_if $presence_done (i32.ge_u (local.get $i) (local.get $num_words)))
        (i32.store
          (i32.add
            (local.get $ptr)
            (i32.add (i32.const {presence_words_offset}) (i32.shl (local.get $i) (i32.const 2))))
          (i32.const -1))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $presence_loop)))
    ;; Zero-fill elements: store tagged 0.0 for each slot
    (local.set $i (i32.const 0))
    (block $fill_done
      (loop $fill_loop
        (br_if $fill_done (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store
          (i32.add
            (local.get $ptr)
            (i32.add (local.get $elem_off) (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.const {tagged_zero}))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $fill_loop)))
    (i32.or (local.get $ptr) (i32.const {array_tag})))
"#,
            array_tag = ValueTag::ARRAY,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            tagged_zero = ValueTag::NUMBER, // number tag with payload 0 = number 0.0
        ));
    }

    /// TypedArray constructor from ArrayBuffer: new Int8Array(buffer, byteOffset?, length?).
    /// Creates an array-backed TypedArray by copying elements from the buffer data region.
    /// Each element in the buffer is stored as a 4-byte tagged runtime value.
    /// Args: (buffer, byteOffset_tagged, length_tagged) — byteOffset and length are tagged numbers.
    pub(crate) fn emit_typed_array_ctor_from_buffer(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $typed_array_ctor_from_buffer (param $buf i32) (param $byte_off_tagged i32) (param $len_tagged i32) (result i32)
    (local $buf_ptr i32)
    (local $buf_len i32)
    (local $byte_off i32)
    (local $elem_count i32)
    (local $num_words i32) (local $elem_off i32)
    (local $size i32) (local $ptr i32) (local $i i32)
    (local $src_elem i32)
    ;; Get buffer ptr and length
    (local.set $buf_ptr (i32.and (local.get $buf) (i32.const {heap_mask})))
    (local.set $buf_len (i32.load (local.get $buf_ptr)))
    ;; Decode byteOffset: if undefined/zero -> 0, else tagged number
    (local.set $byte_off
      (if (result i32) (i32.eqz (local.get $byte_off_tagged))
        (then (i32.const 0))
        (else (i32.shr_s (local.get $byte_off_tagged) (i32.const {num_shift})))))
    ;; Clamp byteOffset to buffer length
    (if (i32.gt_u (local.get $byte_off) (local.get $buf_len))
      (then (local.set $byte_off (local.get $buf_len))))
    ;; Decode length: if undefined/zero -> remaining elements, else tagged number
    (local.set $elem_count
      (if (result i32) (i32.eqz (local.get $len_tagged))
        (then
          (i32.shr_u (i32.sub (local.get $buf_len) (local.get $byte_off)) (i32.const 2)))
        (else (i32.shr_s (local.get $len_tagged) (i32.const {num_shift})))))
    ;; Presence word count = ceil(elem_count / 32)
    (local.set $num_words
      (i32.shr_u (i32.add (local.get $elem_count) (i32.const 31)) (i32.const 5)))
    (local.set $elem_off (i32.const {array_header}))
    (if (i32.gt_u (local.get $num_words) (i32.const 1))
      (then
        (local.set $elem_off
          (i32.add
            (i32.const {array_header})
            (i32.shl (i32.sub (local.get $num_words) (i32.const 1)) (i32.const 2))))))
    ;; size = elem_off + elem_count * 4
    (local.set $size
      (i32.add (local.get $elem_off) (i32.shl (local.get $elem_count) (i32.const 2))))
    (local.set $ptr (call $alloc_heap (local.get $size)))
    ;; Header fields
    (i32.store (local.get $ptr) (local.get $elem_count))
    (i32.store (i32.add (local.get $ptr) (i32.const 4)) (local.get $elem_count))
    (i32.store (i32.add (local.get $ptr) (i32.const 8)) (local.get $num_words))
    (i32.store (i32.add (local.get $ptr) (i32.const 12)) (local.get $elem_off))
    ;; Presence bits: all ones
    (local.set $i (i32.const 0))
    (block $pres_done
      (loop $pres_loop
        (br_if $pres_done (i32.ge_u (local.get $i) (local.get $num_words)))
        (i32.store
          (i32.add
            (local.get $ptr)
            (i32.add (i32.const {presence_words_offset}) (i32.shl (local.get $i) (i32.const 2))))
          (i32.const -1))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $pres_loop)))
    ;; Copy elements from buffer data region to typed array elements
    (local.set $i (i32.const 0))
    (block $copy_done
      (loop $copy_loop
        (br_if $copy_done (i32.ge_u (local.get $i) (local.get $elem_count)))
        (local.set $src_elem
          (i32.load
            (i32.add
              (local.get $buf_ptr)
              (i32.add
                (i32.const {array_header})
                (i32.add (local.get $byte_off) (i32.shl (local.get $i) (i32.const 2)))))))
        (i32.store
          (i32.add
            (local.get $ptr)
            (i32.add (local.get $elem_off) (i32.shl (local.get $i) (i32.const 2))))
          (local.get $src_elem))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $copy_loop)))
    (i32.or (local.get $ptr) (i32.const {array_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            num_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            array_tag = ValueTag::ARRAY,
        ));
    }

    pub(crate) fn emit_typed_array_set(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $typed_array_set (param $target i32) (param $source i32) (param $offset i32) (result i32)
    (local $target_base i32)
    (local $target_len i32)
    (local $source_len_value i32)
    (local $source_len i32)
    (local $offset_raw i32)
    (local $i i32)
    (local $target_index i32)
    (local $elem i32)
    (if
      (i32.ne
        (i32.and (local.get $target) (i32.const {tag_mask}))
        (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $target_base (i32.and (local.get $target) (i32.const {heap_mask})))
    (local.set $target_len (i32.load (local.get $target_base)))
    (local.set $offset_raw (i32.const {zero}))
    (if
      (i32.ne
        (i32.and (local.get $offset) (i32.const {tag_mask}))
        (i32.const {undefined}))
      (then
        (local.set $offset_raw
          (i32.shr_s (local.get $offset) (i32.const {number_shift})))))
    (if (i32.lt_s (local.get $offset_raw) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
    (local.set $source_len_value (call $get_length (local.get $source)))
    (local.set $source_len (i32.const {zero}))
    (if
      (i32.eq
        (i32.and (local.get $source_len_value) (i32.const {tag_mask}))
        (i32.const {number_tag}))
      (then
        (local.set $source_len
          (i32.shr_s (local.get $source_len_value) (i32.const {number_shift})))))
    (block $done
      (loop $copy
        (br_if $done (i32.ge_u (local.get $i) (local.get $source_len)))
        (local.set $target_index (i32.add (local.get $offset_raw) (local.get $i)))
        (br_if $done (i32.ge_u (local.get $target_index) (local.get $target_len)))
        (local.set $elem
          (call $index
            (local.get $source)
            (i32.or
              (i32.shl (local.get $i) (i32.const {number_shift}))
              (i32.const {number_tag}))))
        (i32.store
          (i32.add
            (local.get $target_base)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $target_index) (i32.const {elem_shift}))))
          (local.get $elem))
        (i32.store
          (i32.add (local.get $target_base) (i32.const {presence_words_offset}))
          (i32.or
            (i32.load (i32.add (local.get $target_base) (i32.const {presence_words_offset})))
            (i32.shl (i32.const 1) (local.get $target_index))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy)))
    (i32.const {undefined}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }
}
