use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_object_keys(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_keys (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $flags i32)
    (local $i i32)
    (local $write_i i32)
    (local $entry_base i32)
    (local $key i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    ;; Allocate result array (max size = count)
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    ;; Initialize array header fields (required for $array_get presence check)
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_capacity_offset})) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_word_count_offset})) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_elements_offset_offset})) (i32.const {array_header}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const 0))
    (local.set $write_i (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $keys_done
      (loop $keys_loop
        (br_if $keys_done (i32.ge_u (local.get $i) (local.get $count)))
        ;; Check if property i is non-enumerable (bit (non_enum_shift + i) in flags)
        (if (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
          (then
            ;; Enumerable: copy key to result array
            (local.set $entry_base
              (i32.add (local.get $base)
                (i32.add (i32.const {obj_header})
                  (i32.shl (local.get $i) (i32.const {entry_shift})))))
            (local.set $key (i32.load (local.get $entry_base)))
            (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $write_i) (i32.const {elem_shift})))) (local.get $key))
            (local.set $write_i (i32.add (local.get $write_i) (i32.const {one}))))
        )
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $keys_loop)))
    ;; Compute presence mask = (1 << write_i) - 1 (or -1 if write_i >= 32)
    (if (i32.ge_u (local.get $write_i) (i32.const 32))
      (then
        (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const -1)))
      (else
        (local.set $i (i32.shl (i32.const 1) (local.get $write_i)))
        (local.set $i (i32.sub (local.get $i) (i32.const 1)))
        (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (local.get $i))))
    ;; Update array length to actual enumerable count
    (i32.store (local.get $result_ptr) (local.get $write_i))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_capacity_offset})) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_word_count_offset})) (i32.const {one}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_elements_offset_offset})) (i32.const {array_header}))
    (block $presence
      (if (i32.eqz (local.get $write_i))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const 0))
          (br $presence)))
      (if (i32.gt_u (local.get $write_i) (i32.const 31))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const -1))
          (br $presence)))
      (i32.store
        (i32.add (local.get $result_ptr) (i32.const {presence_words_offset}))
        (i32.sub (i32.shl (i32.const 1) (local.get $write_i)) (i32.const 1))))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            non_enum_shift = Layout::OBJECT_NON_ENUM_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_capacity_offset = Layout::ARRAY_CAPACITY_OFFSET,
            presence_word_count_offset = Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            array_elements_offset_offset = Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            array_tag = ValueTag::ARRAY,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_object_spread(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_spread (param $target i32) (param $source i32) (result i32)
    (local $keys i32)
    (local $keys_tag i32)
    (local $keys_base i32)
    (local $count i32)
    (local $i i32)
    (local $key_raw i32)
    (local $key_base i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local $value i32)
    (local.set $keys (call $object_keys (local.get $source)))
    (local.set $keys_tag (i32.and (local.get $keys) (i32.const {tag_mask})))
    (if (i32.ne (local.get $keys_tag) (i32.const {array_tag}))
      (then (return (local.get $target))))
    (local.set $keys_base (i32.and (local.get $keys) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $keys_base)))
    (local.set $i (i32.const {zero}))
    (block $spread_done
      (loop $spread_loop
        (br_if $spread_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $key_raw
          (i32.load
            (i32.add
              (local.get $keys_base)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $key_base (i32.and (local.get $key_raw) (i32.const {heap_mask})))
        (local.set $key_ptr (i32.add (local.get $key_base) (i32.const {str_header})))
        (local.set $key_len (i32.load (local.get $key_base)))
        (local.set $value
          (call $property_get
            (local.get $source)
            (local.get $key_ptr)
            (local.get $key_len)))
        (drop
          (call $property_set
            (local.get $target)
            (local.get $key_ptr)
            (local.get $key_len)
            (local.get $value)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $spread_loop)))
    (local.get $target))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_object_values(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_values (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $value i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    ;; Allocate result array
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_capacity_offset})) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_word_count_offset})) (i32.const {one}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_elements_offset_offset})) (i32.const {array_header}))
    (block $values_presence
      (if (i32.eqz (local.get $count))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const 0))
          (br $values_presence)))
      (if (i32.gt_u (local.get $count) (i32.const 31))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const -1))
          (br $values_presence)))
      (i32.store
        (i32.add (local.get $result_ptr) (i32.const {presence_words_offset}))
        (i32.sub (i32.shl (i32.const 1) (local.get $count)) (i32.const 1))))
    ;; Extract all values
    (local.set $i (i32.const {zero}))
    (block $values_done
      (loop $values_loop
        (br_if $values_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $value (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))
        ;; Store value in result array
        (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift})))) (local.get $value))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $values_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_capacity_offset = Layout::ARRAY_CAPACITY_OFFSET,
            presence_word_count_offset = Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            array_elements_offset_offset = Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            array_tag = ValueTag::ARRAY,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_object_entries(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_entries (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $key i32)
    (local $value i32)
    (local $result_ptr i32)
    (local $pair_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    ;; Allocate result array (count entries)
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_capacity_offset})) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_word_count_offset})) (i32.const {one}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_elements_offset_offset})) (i32.const {array_header}))
    (block $entries_presence
      (if (i32.eqz (local.get $count))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const 0))
          (br $entries_presence)))
      (if (i32.gt_u (local.get $count) (i32.const 31))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const -1))
          (br $entries_presence)))
      (i32.store
        (i32.add (local.get $result_ptr) (i32.const {presence_words_offset}))
        (i32.sub (i32.shl (i32.const 1) (local.get $count)) (i32.const 1))))
    ;; Extract all [key, value] pairs
    (local.set $i (i32.const {zero}))
    (block $entries_done
      (loop $entries_loop
        (br_if $entries_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $key (i32.load (local.get $entry_base)))
        (local.set $value (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))
        ;; Allocate 2-element pair array
        (local.set $pair_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.const {pair_size}))))
        (i32.store (local.get $pair_ptr) (i32.const {two}))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_capacity_offset})) (i32.const {two}))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const {presence_word_count_offset})) (i32.const {one}))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_elements_offset_offset})) (i32.const {array_header}))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const {presence_words_offset})) (i32.const 3))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_header})) (local.get $key))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_header_plus_4})) (local.get $value))
        ;; Store pair in result array
        (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift})))) (i32.or (local.get $pair_ptr) (i32.const {array_tag})))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $entries_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_capacity_offset = Layout::ARRAY_CAPACITY_OFFSET,
            presence_word_count_offset = Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            array_elements_offset_offset = Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            pair_size = 8,
            array_header_plus_4 = Layout::ARRAY_HEADER_SIZE + 4,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            two = 2,
            array_tag = ValueTag::ARRAY,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_object_has_own_property(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_has_own_property (param $obj i32) (param $key i32) (result i32)
    (local $key_len i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    ;; own-property-only scan (no prototype walk)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {false}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (local.get $count))
    (block $not_found
      (loop $scan
        (br_if $not_found (i32.eq (local.get $i) (i32.const {zero})))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $pk_raw (i32.load (local.get $entry_base)))
        (local.set $pk_ptr
          (i32.add (i32.and (local.get $pk_raw) (i32.const {heap_mask})) (i32.const {str_header})))
        (local.set $pk_len
          (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask}))))
        (if (i32.eq (local.get $key_len) (local.get $pk_len))
          (then
            (if (call $mem_equal
                  (i32.const {scratch_offset}) (local.get $pk_ptr) (local.get $key_len))
              (then (return (i32.const {true}))))))
        (br $scan)))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            scratch_offset = Layout::SCRATCH_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            false = ValueTag::FALSE,
            true = ValueTag::TRUE,
        ));
    }

    pub(crate) fn emit_object_has_own(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $object_has_own (param $obj i32) (param $prop i32) (result i32)
    (return (call $object_has_own_property (local.get $obj) (local.get $prop))))
"#,
        );
    }

    pub(crate) fn emit_object_get_own_property_descriptor(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_get_own_property_descriptor (param $obj i32) (param $key i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $entry_key_raw i32)
    (local $entry_key_ptr i32)
    (local $entry_key_len i32)
    (local $entry_value i32)
    (local $desc i32)
    (local $key_len i32)
    (local $flags i32)
    (local $prop_offset i32)
    (if (i32.ne (i32.and (local.get $obj) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $key_len (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (block $desc_done (result i32)
      (local.set $i (local.get $count))
      (loop $desc_loop
        (if (i32.eq (local.get $i) (i32.const {zero}))
          (then (br $desc_done (i32.const {undefined}))))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $entry_key_raw (i32.load (local.get $entry_base)))
        (local.set $entry_key_ptr
          (i32.add (i32.and (local.get $entry_key_raw) (i32.const {heap_mask})) (i32.const {str_header})))
        (local.set $entry_key_len
          (i32.load (i32.and (local.get $entry_key_raw) (i32.const {heap_mask}))))
        (if (i32.eq (local.get $key_len) (local.get $entry_key_len))
          (then
            (if (call $mem_equal (i32.const {scratch_offset}) (local.get $entry_key_ptr) (local.get $key_len))
              (then
                (local.set $entry_value (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))
                (local.set $desc (call $alloc_heap (i32.const {collection_size})))
                (i32.store (local.get $desc) (i32.const {zero}))
                (i32.store (i32.add (local.get $desc) (i32.const {obj_proto})) (i32.const {zero}))
                ;; Use scratch + 64 for property name strings to avoid clobbering key
                (local.set $prop_offset (i32.add (i32.const {scratch_offset}) (i32.const 64)))
                ;; Check if this is an accessor descriptor
                (if (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {accessor_shift}))))
                  (then
                    ;; Accessor descriptor: return get/set/enumerable/configurable
                    ;; Write "get" with the stored getter value
                    (i32.store8 (local.get $prop_offset) (i32.const 103))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 116))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 3)
                        (local.get $entry_value)))
                    ;; Write "set": always undefined
                    (i32.store8 (local.get $prop_offset) (i32.const 115))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 116))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 3)
                        (i32.const {undefined})))
                    ;; Write "enumerable"
                    (i32.store8 (local.get $prop_offset) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 110))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 109))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 10)
                        (if (result i32)
                          (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
                          (then (i32.const {true}))
                          (else (i32.const {false})))))
                    ;; Write "configurable"
                    (i32.store8 (local.get $prop_offset) (i32.const 99))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 111))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 110))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 102))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 105))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 103))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 10)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 11)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 12)
                        (if (result i32)
                          (i32.and
                            (i32.eqz (i32.and (local.get $flags) (i32.const {frozen_flag})))
                            (i32.and
                              (i32.eqz (i32.and (local.get $flags) (i32.const {sealed_flag})))
                              (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift})))))
                            )
                          )
                          (then (i32.const {true}))
                          (else (i32.const {false})))))
                  (else
                    ;; Data descriptor: return value/writable/enumerable/configurable
                    ;; Write "value"
                    (i32.store8 (local.get $prop_offset) (i32.const 118))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 5)
                        (local.get $entry_value)))
                    ;; Write "writable"
                    (i32.store8 (local.get $prop_offset) (i32.const 119))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 105))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 116))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 8)
                        (if (result i32)
                          (i32.and
                            (i32.eqz (i32.and (local.get $flags) (i32.const {frozen_flag})))
                            (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_writable_shift})))))
                          )
                          (then (i32.const {true}))
                          (else (i32.const {false})))))
                    ;; Write "enumerable"
                    (i32.store8 (local.get $prop_offset) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 110))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 109))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 10)
                        (if (result i32)
                          (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
                          (then (i32.const {true}))
                          (else (i32.const {false})))))
                    ;; Write "configurable"
                    (i32.store8 (local.get $prop_offset) (i32.const 99))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 111))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 110))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 102))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 105))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 103))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 10)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 11)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 12)
                        (if (result i32)
                          (i32.and
                            (i32.eqz (i32.and (local.get $flags) (i32.const {frozen_flag})))
                            (i32.and
                              (i32.eqz (i32.and (local.get $flags) (i32.const {sealed_flag})))
                              (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift})))))
                            )
                          )
                          (then (i32.const {true}))
                          (else (i32.const {false})))))
                    ;; Write "get": undefined
                    (i32.store8 (local.get $prop_offset) (i32.const 103))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 116))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 3)
                        (i32.const {undefined})))
                    ;; Write "set": undefined
                    (i32.store8 (local.get $prop_offset) (i32.const 115))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 116))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 3)
                        (i32.const {undefined})))))
                (br $desc_done (i32.or (local.get $desc) (i32.const {object_tag})))))))
        (br $desc_loop))
      (i32.const {undefined})))
            "#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            frozen_flag = Layout::OBJECT_FLAG_FROZEN,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            non_enum_shift = Layout::OBJECT_NON_ENUM_SHIFT,
            non_writable_shift = Layout::OBJECT_NON_WRITABLE_SHIFT,
            non_configurable_shift = Layout::OBJECT_NON_CONFIGURABLE_SHIFT,
            accessor_shift = Layout::OBJECT_ACCESSOR_PROP_SHIFT,
            collection_size =
                (Layout::OBJECT_HEADER_SIZE + (32 * Layout::OBJECT_ENTRY_SIZE)) as i32,
            scratch_offset = Layout::SCRATCH_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_object_get_prototype_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_get_prototype_of (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $proto i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $proto (i32.load (i32.add (local.get $base) (i32.const {obj_proto}))))
    (if (i32.eqz (local.get $proto))
      (then (return (i32.const {null}))))
    (i32.or (local.get $proto) (i32.const {object_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            undefined = ValueTag::UNDEFINED,
            null = ValueTag::NULL,
        ));
    }

    pub(crate) fn emit_object_set_prototype_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_set_prototype_of (param $obj i32) (param $proto i32) (result i32)
    (local $obj_tag i32)
    (local $proto_tag i32)
    (local $base i32)
    (local $proto_ptr i32)
    (local.set $obj_tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $obj_tag) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))

    (local.set $proto_ptr (i32.const 0))
    (if (i32.ne (local.get $proto) (i32.const {null}))
      (then
        (local.set $proto_tag (i32.and (local.get $proto) (i32.const {tag_mask})))
        (if (i32.ne (local.get $proto_tag) (i32.const {object_tag}))
          (then (return (i32.const {undefined}))))
        (local.set $proto_ptr (i32.and (local.get $proto) (i32.const {heap_mask})))))

    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (if (i32.eq (local.get $base) (local.get $proto_ptr))
      (then (return (i32.const {undefined}))))
    (i32.store
      (i32.add (local.get $base) (i32.const {obj_proto}))
      (local.get $proto_ptr))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            undefined = ValueTag::UNDEFINED,
            null = ValueTag::NULL,
        ));
    }

    pub(crate) fn emit_object_freeze(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_freeze (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (i32.store (i32.add (local.get $base) (i32.const {obj_flags}))
      (i32.or (local.get $flags) (i32.const {frozen_flag})))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            frozen_flag = Layout::OBJECT_FLAG_FROZEN,
        ));
    }

    pub(crate) fn emit_object_seal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_seal (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (i32.store (i32.add (local.get $base) (i32.const {obj_flags}))
      (i32.or (local.get $flags) (i32.const {sealed_flag})))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
        ));
    }

    pub(crate) fn emit_object_prevent_extensions(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_prevent_extensions (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (i32.store (i32.add (local.get $base) (i32.const {obj_flags}))
      (i32.or (local.get $flags) (i32.const {sealed_flag})))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
        ));
    }

    pub(crate) fn emit_object_is_extensible(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_is_extensible (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {false_val}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (if (i32.and (local.get $flags) (i32.const {sealed_flag}))
      (then (return (i32.const {false_val}))))
    (i32.const {true_val}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
            true_val = ValueTag::TRUE,
            false_val = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_object_is_sealed(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_is_sealed (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {true_val}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (if (i32.and (local.get $flags) (i32.const {sealed_flag}))
      (then (return (i32.const {true_val}))))
    (i32.const {false_val}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
            true_val = ValueTag::TRUE,
            false_val = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_object_is_frozen(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_is_frozen (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {true_val}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (if (i32.and (local.get $flags) (i32.const {frozen_flag}))
      (then (return (i32.const {true_val}))))
    (i32.const {false_val}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            frozen_flag = Layout::OBJECT_FLAG_FROZEN,
            true_val = ValueTag::TRUE,
            false_val = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_object_define_property(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_define_property (param $obj i32) (param $key i32) (param $desc i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $key_len i32)
    (local $desc_off i32)
    (local $value i32)
    (local $get i32)
    (local $set i32)
    (local $writable i32)
    (local $enumerable i32)
    (local $configurable i32)
    (local $has_value i32)
    (local $has_get i32)
    (local $i i32)
    (local $count i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $key_len (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (local.set $desc_off (i32.add (i32.const {scratch_offset}) (i32.const 128)))
    (i32.store8 (local.get $desc_off) (i32.const 103))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 101))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 116))
    (local.set $get (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 3)))
    (local.set $has_get (i32.ne (local.get $get) (i32.const {undefined})))
    (if (local.get $has_get)
      (then
        (drop (call $property_set (local.get $obj) (i32.const {scratch_offset}) (local.get $key_len) (local.get $get)))
        (i32.store8 (local.get $desc_off) (i32.const 101))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 110))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 117))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 109))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 101))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 114))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 97))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 98))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 8)) (i32.const 108))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 9)) (i32.const 101))
        (local.set $enumerable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 10)))
        (i32.store8 (local.get $desc_off) (i32.const 99))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 111))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 110))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 102))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 105))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 103))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 117))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 114))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 8)) (i32.const 97))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 9)) (i32.const 98))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 10)) (i32.const 108))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 11)) (i32.const 101))
        (local.set $configurable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 12)))
        (local.set $count (i32.load (local.get $base)))
        (local.set $i (i32.const {zero}))
        (block $find_acc_done
          (loop $find_acc_loop
            (br_if $find_acc_done (i32.ge_u (local.get $i) (local.get $count)))
            (local.set $entry_base (i32.add (local.get $base) (i32.add (i32.const {obj_header}) (i32.shl (local.get $i) (i32.const {entry_shift})))))
            (local.set $pk_raw (i32.load (local.get $entry_base)))
            (local.set $pk_ptr (i32.add (i32.and (local.get $pk_raw) (i32.const {heap_mask})) (i32.const {str_header})))
            (local.set $pk_len (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask}))))
            (if (i32.eq (local.get $key_len) (local.get $pk_len))
              (then
                (if (call $mem_equal (i32.const {scratch_offset}) (local.get $pk_ptr) (local.get $key_len))
                  (then
                    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
                    (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {accessor_shift})))))
                    (if (i32.eq (local.get $enumerable) (i32.const {false}))
                      (then (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift}))))))
                      (else (if (i32.eq (local.get $enumerable) (i32.const {true}))
                        (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift}))) (i32.const -1))))))))
                    (if (i32.eq (local.get $configurable) (i32.const {false}))
                      (then (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift}))))))
                      (else (if (i32.eq (local.get $configurable) (i32.const {true}))
                        (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift}))) (i32.const -1))))))))
                    (i32.store (i32.add (local.get $base) (i32.const {obj_flags})) (local.get $flags))
                    (br $find_acc_done)))))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $find_acc_loop))
        (return (local.get $obj)))))
    (i32.store8 (local.get $desc_off) (i32.const 118))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 97))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 108))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 117))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 101))
    (local.set $value (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 5)))
    (if (i32.ne (local.get $value) (i32.const {undefined}))
      (then (drop (call $property_set (local.get $obj) (i32.const {scratch_offset}) (local.get $key_len) (local.get $value)))))
    (i32.store8 (local.get $desc_off) (i32.const 119))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 114))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 105))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 116))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 97))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 98))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 108))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 101))
    (local.set $writable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 8)))
    (i32.store8 (local.get $desc_off) (i32.const 101))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 110))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 117))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 109))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 101))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 114))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 97))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 98))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 8)) (i32.const 108))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 9)) (i32.const 101))
    (local.set $enumerable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 10)))
    (i32.store8 (local.get $desc_off) (i32.const 99))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 111))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 110))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 102))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 105))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 103))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 117))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 114))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 8)) (i32.const 97))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 9)) (i32.const 98))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 10)) (i32.const 108))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 11)) (i32.const 101))
    (local.set $configurable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 12)))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (block $find_entry_done
      (loop $find_entry_loop
        (br_if $find_entry_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base (i32.add (local.get $base) (i32.add (i32.const {obj_header}) (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $pk_raw (i32.load (local.get $entry_base)))
        (local.set $pk_ptr (i32.add (i32.and (local.get $pk_raw) (i32.const {heap_mask})) (i32.const {str_header})))
        (local.set $pk_len (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask}))))
        (if (i32.eq (local.get $key_len) (local.get $pk_len))
          (then
            (if (call $mem_equal (i32.const {scratch_offset}) (local.get $pk_ptr) (local.get $key_len))
              (then
                (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
                (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {accessor_shift}))) (i32.const -1))))
                (if (i32.eq (local.get $writable) (i32.const {false}))
                  (then (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_writable_shift}))))))
                  (else (if (i32.eq (local.get $writable) (i32.const {true}))
                    (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_writable_shift}))) (i32.const -1))))))))
                (if (i32.eq (local.get $enumerable) (i32.const {false}))
                  (then (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift}))))))
                  (else (if (i32.eq (local.get $enumerable) (i32.const {true}))
                    (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift}))) (i32.const -1))))))))
                (if (i32.eq (local.get $configurable) (i32.const {false}))
                  (then (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift}))))))
                  (else (if (i32.eq (local.get $configurable) (i32.const {true}))
                    (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift}))) (i32.const -1))))))))
                (i32.store (i32.add (local.get $base) (i32.const {obj_flags})) (local.get $flags))
                (br $find_entry_done)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $find_entry_loop))
    )
    (local.get $obj))
"#,
tag_mask = ValueTag::TAG_MASK, object_tag = ValueTag::OBJECT, heap_mask = ValueTag::HEAP_MASK, scratch_offset = Layout::SCRATCH_OFFSET, obj_header = Layout::OBJECT_HEADER_SIZE, obj_flags = Layout::OBJECT_FLAGS_OFFSET, entry_shift = Layout::OBJECT_ENTRY_SHIFT, str_header = Layout::STRING_HEADER_SIZE, non_enum_shift = Layout::OBJECT_NON_ENUM_SHIFT, non_writable_shift = Layout::OBJECT_NON_WRITABLE_SHIFT, non_configurable_shift = Layout::OBJECT_NON_CONFIGURABLE_SHIFT, accessor_shift = Layout::OBJECT_ACCESSOR_PROP_SHIFT, zero = RuntimeConst::ZERO, one = RuntimeConst::ONE, undefined = ValueTag::UNDEFINED, false = ValueTag::FALSE, true = ValueTag::TRUE));
    }

    pub(crate) fn emit_object_assign(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_assign (param $target i32) (param $source i32) (result i32)
    (local $keys i32)
    (local $keys_tag i32)
    (local $keys_base i32)
    (local $count i32)
    (local $i i32)
    (local $key_raw i32)
    (local $key_base i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local $value i32)
    (local.set $keys (call $object_keys (local.get $source)))
    (local.set $keys_tag (i32.and (local.get $keys) (i32.const {tag_mask})))
    (if (i32.ne (local.get $keys_tag) (i32.const {array_tag}))
      (then (return (local.get $target))))
    (local.set $keys_base (i32.and (local.get $keys) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $keys_base)))
    (local.set $i (i32.const {zero}))
    (block $assign_done
      (loop $assign_loop
        (br_if $assign_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $key_raw
          (i32.load
            (i32.add
              (local.get $keys_base)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $key_base (i32.and (local.get $key_raw) (i32.const {heap_mask})))
        (local.set $key_ptr (i32.add (local.get $key_base) (i32.const {str_header})))
        (local.set $key_len (i32.load (local.get $key_base)))
        (local.set $value
          (call $property_get
            (local.get $source)
            (local.get $key_ptr)
            (local.get $key_len)))
        (drop
          (call $property_set
            (local.get $target)
            (local.get $key_ptr)
            (local.get $key_len)
            (local.get $value)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $assign_loop)))
    (local.get $target))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_object_create(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_create (param $proto i32) (result i32)
    (local $obj i32)
    (local $proto_ptr i32)
    (local $proto_tag i32)
    ;; Allocate object with 0 initial entries
    (local.set $obj (call $alloc_heap (i32.const {obj_header})))
    (i32.store (local.get $obj) (i32.const {zero}))
    (i32.store (i32.add (local.get $obj) (i32.const {obj_flags})) (i32.const {zero}))
    ;; Set prototype
    (local.set $proto_ptr (i32.const 0))
    (if (i32.ne (local.get $proto) (i32.const {null}))
      (then
        (local.set $proto_tag (i32.and (local.get $proto) (i32.const {tag_mask})))
        (if (i32.eq (local.get $proto_tag) (i32.const {object_tag}))
          (then
            (local.set $proto_ptr (i32.and (local.get $proto) (i32.const {heap_mask})))))))
    (i32.store (i32.add (local.get $obj) (i32.const {obj_proto})) (local.get $proto_ptr))
    (i32.or (local.get $obj) (i32.const {object_tag})))
"#,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            zero = RuntimeConst::ZERO,
            null = ValueTag::NULL,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
        ));
    }

    pub(crate) fn emit_object_is(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_is (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_is_number i32)
    (local $b_is_number i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.and (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then
        (if (i32.eq (call $bigint_compare (local.get $a) (local.get $b)) (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if (i32.or (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then (return (i32.const {false_tag}))))
    (if (i32.and (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $string_equal (local.get $a) (local.get $b)))))
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (i32.const {false_tag}))))
    (local.set $a_is_number (i32.eq (local.get $a_tag) (i32.const {number_tag})))
    (local.set $b_is_number (i32.eq (local.get $b_tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $a_tag) (i32.const {object_tag}))
      (then
        (local.set $a_is_number
          (i32.eq
            (i32.load (i32.and (local.get $a) (i32.const {heap_mask})))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eq (local.get $b_tag) (i32.const {object_tag}))
      (then
        (local.set $b_is_number
          (i32.eq
            (i32.load (i32.and (local.get $b) (i32.const {heap_mask})))
            (i32.const {heap_number_sentinel})))))
    (if (i32.and (local.get $a_is_number) (local.get $b_is_number))
      (then
        ;; SameValue differs from strict equality for NaN and signed zero.
        ;; The current number model does not encode either distinction, so
        ;; numeric SameValue reduces to payload equality here.
        (return
          (if (result i32)
            (i32.eq (call $number_to_i32 (local.get $a)) (call $number_to_i32 (local.get $b)))
            (then (i32.const {true_tag}))
            (else (i32.const {false_tag}))))))
    (if (result i32) (i32.eq (local.get $a) (local.get $b))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            zero = RuntimeConst::ZERO,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }
}
