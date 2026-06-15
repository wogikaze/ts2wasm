use crate::emitter::{WatEmitter, builtin_error_prototype_global};
use crate::runtime_fn::RuntimeGlobal;
use ts2wasm_ir::lowered::BuiltinErrorConstructor;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_object_keys(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_key_array_index_or_minus1 (param $key i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $ptr i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $value i32)
    (local.set $tag (i32.and (local.get $key) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {string_tag}))
      (then (return (i32.const -1))))
    (local.set $base (i32.and (local.get $key) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (if (i32.eqz (local.get $len))
      (then (return (i32.const -1))))
    (local.set $ptr (i32.add (local.get $base) (i32.const {str_header})))
    ;; Canonical array-index strings do not have leading zeros except "0".
    (if (i32.and
          (i32.gt_u (local.get $len) (i32.const 1))
          (i32.eq (i32.load8_u (local.get $ptr)) (i32.const 48)))
      (then (return (i32.const -1))))
    (local.set $i (i32.const 0))
    (local.set $value (i32.const 0))
    (block $parse_done
      (loop $parse
        (br_if $parse_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
        (if (i32.or
              (i32.lt_u (local.get $ch) (i32.const 48))
              (i32.gt_u (local.get $ch) (i32.const 57)))
          (then (return (i32.const -1))))
        (local.set $value
          (i32.add
            (i32.mul (local.get $value) (i32.const 10))
            (i32.sub (local.get $ch) (i32.const 48))))
        (if (i32.lt_s (local.get $value) (i32.const 0))
          (then (return (i32.const -1))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $parse)))
    (local.get $value))

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
    (local $candidate_index i32)
    (local $last_index i32)
    (local $best_index i32)
    (local $best_i i32)
    (local $found i32)
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
    ;; OrdinaryOwnPropertyKeys orders array-index keys first, ascending.
    (local.set $last_index (i32.const -1))
    (block $numeric_done
      (loop $numeric_outer
        (local.set $found (i32.const 0))
        (local.set $best_index (i32.const 0))
        (local.set $best_i (i32.const 0))
        (local.set $i (i32.const {zero}))
        (block $numeric_scan_done
          (loop $numeric_scan
            (br_if $numeric_scan_done (i32.ge_u (local.get $i) (local.get $count)))
            (if (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
              (then
                (local.set $entry_base
                  (i32.add (local.get $base)
                    (i32.add (i32.const {obj_header})
                      (i32.shl (local.get $i) (i32.const {entry_shift})))))
                (local.set $key (i32.load (local.get $entry_base)))
                (local.set $candidate_index (call $object_key_array_index_or_minus1 (local.get $key)))
                (if (i32.and
                      (i32.ge_s (local.get $candidate_index) (i32.const 0))
                      (i32.gt_s (local.get $candidate_index) (local.get $last_index)))
                  (then
                    (if (i32.or
                          (i32.eqz (local.get $found))
                          (i32.lt_s (local.get $candidate_index) (local.get $best_index)))
                      (then
                        (local.set $found (i32.const 1))
                        (local.set $best_index (local.get $candidate_index))
                        (local.set $best_i (local.get $i))))))))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $numeric_scan)))
        (if (i32.eqz (local.get $found))
          (then (br $numeric_done)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $best_i) (i32.const {entry_shift})))))
        (local.set $key (i32.load (local.get $entry_base)))
        (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $write_i) (i32.const {elem_shift})))) (local.get $key))
        (local.set $write_i (i32.add (local.get $write_i) (i32.const {one})))
        (local.set $last_index (local.get $best_index))
        (br $numeric_outer)))
    ;; Then emit ordinary string keys in insertion order.
    (local.set $i (i32.const {zero}))
    (block $string_keys_done
      (loop $string_keys_loop
        (br_if $string_keys_done (i32.ge_u (local.get $i) (local.get $count)))
        (if (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
          (then
            (local.set $entry_base
              (i32.add (local.get $base)
                (i32.add (i32.const {obj_header})
                  (i32.shl (local.get $i) (i32.const {entry_shift})))))
            (local.set $key (i32.load (local.get $entry_base)))
            (local.set $candidate_index (call $object_key_array_index_or_minus1 (local.get $key)))
            (if (i32.lt_s (local.get $candidate_index) (i32.const 0))
              (then
                (if
                  (i32.eqz
                    (i32.and
                      (i32.eq (i32.and (local.get $key) (i32.const {tag_mask})) (i32.const {object_tag}))
                      (i32.eq
                        (i32.load (i32.and (local.get $key) (i32.const {heap_mask})))
                        (i32.const {symbol_sentinel}))))
                  (then
                    (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $write_i) (i32.const {elem_shift})))) (local.get $key))
                    (local.set $write_i (i32.add (local.get $write_i) (i32.const {one})))))))
          ))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $string_keys_loop)))
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
            string_tag = ValueTag::STRING,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
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
            symbol_sentinel = Layout::SYMBOL_SENTINEL,
        ));
    }

    pub(crate) fn emit_object_get_own_property_names(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_get_own_property_names (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $flags i32)
    (local $i i32)
    (local $write_i i32)
    (local $entry_base i32)
    (local $key i32)
    (local $result_ptr i32)
    (local $candidate_index i32)
    (local $last_index i32)
    (local $best_index i32)
    (local $best_i i32)
    (local $found i32)
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
    ;; OrdinaryOwnPropertyKeys orders array-index keys first, ascending.
    (local.set $last_index (i32.const -1))
    (block $numeric_done
      (loop $numeric_outer
        (local.set $found (i32.const 0))
        (local.set $best_index (i32.const 0))
        (local.set $best_i (i32.const 0))
        (local.set $i (i32.const {zero}))
        (block $numeric_scan_done
          (loop $numeric_scan
            (br_if $numeric_scan_done (i32.ge_u (local.get $i) (local.get $count)))
            (local.set $entry_base
              (i32.add (local.get $base)
                (i32.add (i32.const {obj_header})
                  (i32.shl (local.get $i) (i32.const {entry_shift})))))
            (local.set $key (i32.load (local.get $entry_base)))
            (local.set $candidate_index (call $object_key_array_index_or_minus1 (local.get $key)))
            (if (i32.and
                  (i32.ge_s (local.get $candidate_index) (i32.const 0))
                  (i32.gt_s (local.get $candidate_index) (local.get $last_index)))
              (then
                (if (i32.or
                      (i32.eqz (local.get $found))
                      (i32.lt_s (local.get $candidate_index) (local.get $best_index)))
                  (then
                    (local.set $found (i32.const 1))
                    (local.set $best_index (local.get $candidate_index))
                    (local.set $best_i (local.get $i))))))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $numeric_scan)))
        (if (i32.eqz (local.get $found))
          (then (br $numeric_done)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $best_i) (i32.const {entry_shift})))))
        (local.set $key (i32.load (local.get $entry_base)))
        (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $write_i) (i32.const {elem_shift})))) (local.get $key))
        (local.set $write_i (i32.add (local.get $write_i) (i32.const {one})))
        (local.set $last_index (local.get $best_index))
        (br $numeric_outer)))
    ;; Then emit ordinary string keys in insertion order (all own, including non-enumerable).
    (local.set $i (i32.const {zero}))
    (block $string_keys_done
      (loop $string_keys_loop
        (br_if $string_keys_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $key (i32.load (local.get $entry_base)))
        (local.set $candidate_index (call $object_key_array_index_or_minus1 (local.get $key)))
        (if (i32.lt_s (local.get $candidate_index) (i32.const 0))
          (then
            (if
              (i32.eqz
                (i32.and
                  (i32.eq (i32.and (local.get $key) (i32.const {tag_mask})) (i32.const {object_tag}))
                  (i32.eq
                    (i32.load (i32.and (local.get $key) (i32.const {heap_mask})))
                    (i32.const {symbol_sentinel}))))
              (then
                (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $write_i) (i32.const {elem_shift})))) (local.get $key))
                (local.set $write_i (i32.add (local.get $write_i) (i32.const {one})))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $string_keys_loop)))
    ;; Compute presence mask = (1 << write_i) - 1 (or -1 if write_i >= 32)
    (if (i32.ge_u (local.get $write_i) (i32.const 32))
      (then
        (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const -1)))
      (else
        (local.set $i (i32.shl (i32.const 1) (local.get $write_i)))
        (local.set $i (i32.sub (local.get $i) (i32.const 1)))
        (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (local.get $i))))
    ;; Update array length to actual count
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
            symbol_sentinel = Layout::SYMBOL_SENTINEL,
        ));
    }

    pub(crate) fn emit_object_get_own_property_symbols(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_get_own_property_symbols (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $write_i i32)
    (local $entry_base i32)
    (local $key i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_capacity_offset})) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_word_count_offset})) (i32.const {one}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_elements_offset_offset})) (i32.const {array_header}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const 0))
    (local.set $write_i (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $symbols_done
      (loop $symbols_loop
        (br_if $symbols_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $key (i32.load (local.get $entry_base)))
        (if
          (i32.and
            (i32.eq (i32.and (local.get $key) (i32.const {tag_mask})) (i32.const {object_tag}))
            (i32.eq
              (i32.load (i32.and (local.get $key) (i32.const {heap_mask})))
              (i32.const {symbol_sentinel})))
          (then
            (i32.store
              (i32.add
                (local.get $result_ptr)
                (i32.add
                  (i32.const {array_header})
                  (i32.shl (local.get $write_i) (i32.const {elem_shift}))))
              (local.get $key))
            (local.set $write_i (i32.add (local.get $write_i) (i32.const {one})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $symbols_loop)))
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
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            array_capacity_offset = Layout::ARRAY_CAPACITY_OFFSET,
            presence_word_count_offset = Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            array_elements_offset_offset = Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            array_tag = ValueTag::ARRAY,
            symbol_sentinel = Layout::SYMBOL_SENTINEL,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
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
    (local $flags i32)
    (local $i i32)
    (local $write_i i32)
    (local $entry_base i32)
    (local $value i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    ;; Allocate result array (max size = count)
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_capacity_offset})) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_word_count_offset})) (i32.const {one}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_elements_offset_offset})) (i32.const {array_header}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const 0))
    (local.set $write_i (i32.const {zero}))
    ;; Extract only enumerable own property values
    (local.set $i (i32.const {zero}))
    (block $values_done
      (loop $values_loop
        (br_if $values_done (i32.ge_u (local.get $i) (local.get $count)))
        (if (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
          (then
            (local.set $entry_base
              (i32.add (local.get $base)
                (i32.add (i32.const {obj_header})
                  (i32.shl (local.get $i) (i32.const {entry_shift})))))
            (local.set $value (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))
            (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $write_i) (i32.const {elem_shift})))) (local.get $value))
            (local.set $write_i (i32.add (local.get $write_i) (i32.const {one})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $values_loop)))
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
    (local $flags i32)
    (local $i i32)
    (local $write_i i32)
    (local $entry_base i32)
    (local $key i32)
    (local $value i32)
    (local $result_ptr i32)
    (local $pair_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    ;; Allocate result array (max size = count)
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_capacity_offset})) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_word_count_offset})) (i32.const {one}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {array_elements_offset_offset})) (i32.const {array_header}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const 0))
    (local.set $write_i (i32.const {zero}))
    ;; Extract only enumerable own property [key, value] pairs
    (local.set $i (i32.const {zero}))
    (block $entries_done
      (loop $entries_loop
        (br_if $entries_done (i32.ge_u (local.get $i) (local.get $count)))
        (if (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
          (then
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
            (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $write_i) (i32.const {elem_shift})))) (i32.or (local.get $pair_ptr) (i32.const {array_tag})))
            (local.set $write_i (i32.add (local.get $write_i) (i32.const {one})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $entries_loop)))
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
    (local $key_arg i32)
    (local $key_len i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (if
      (i32.and
        (i32.eq (i32.and (local.get $key) (i32.const {tag_mask})) (i32.const {object_tag}))
        (i32.eq
          (i32.load (i32.and (local.get $key) (i32.const {heap_mask})))
          (i32.const {symbol_sentinel})))
      (then
        (local.set $key_arg (local.get $key))
        (local.set $key_len (i32.const -1)))
      (else
        (local.set $key_arg (i32.const {scratch_offset}))
        (local.set $key_len (call $value_to_string_into (local.get $key) (local.get $key_arg)))))
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
        (if (i32.eq (local.get $key_len) (i32.const -1))
          (then
            (if (i32.eq (local.get $pk_raw) (local.get $key_arg))
              (then (return (i32.const {true}))))
            (br $scan)))
        (if
          (i32.and
            (i32.eq (i32.and (local.get $pk_raw) (i32.const {tag_mask})) (i32.const {object_tag}))
            (i32.eq
              (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask})))
              (i32.const {symbol_sentinel})))
          (then (br $scan)))
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
            symbol_sentinel = Layout::SYMBOL_SENTINEL,
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

    pub(crate) fn emit_object_from_entries(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_from_entries (param $entries i32) (result i32)
    (local $tag i32)
    (local $entries_base i32)
    (local $count i32)
    (local $i i32)
    (local $result_obj i32)
    (local $pair i32)
    (local $key i32)
    (local $value i32)
    (local $key_len i32)
    (local $proto i32)
    (local.set $tag (i32.and (local.get $entries) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag}))
      (then
        (return (call $object_create (i32.const {null})))))
    (local.set $entries_base (i32.and (local.get $entries) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $entries_base)))
    (local.set $result_obj
      (call $alloc_heap
        (i32.add
          (i32.const {obj_header})
          (i32.shl (local.get $count) (i32.const {entry_shift})))))
    (i32.store (local.get $result_obj) (i32.const {zero}))
    (i32.store (i32.add (local.get $result_obj) (i32.const {obj_flags})) (i32.const {zero}))
    (local.set $proto (i32.and (call $object_prototype) (i32.const {heap_mask})))
    (i32.store (i32.add (local.get $result_obj) (i32.const {obj_proto})) (local.get $proto))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $pair
          (call $array_get
            (local.get $entries)
            (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number_tag}))))
        (local.set $key
          (call $array_get
            (local.get $pair)
            (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag}))))
        (local.set $value
          (call $array_get
            (local.get $pair)
            (i32.or (i32.shl (i32.const {one}) (i32.const {number_shift})) (i32.const {number_tag}))))
        (local.set $key_len (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
        (drop
          (call $property_set
            (i32.or (local.get $result_obj) (i32.const {object_tag}))
            (i32.const {scratch_offset})
            (local.get $key_len)
            (local.get $value)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.or (local.get $result_obj) (i32.const {object_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            scratch_offset = Layout::SCRATCH_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            null = ValueTag::NULL,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_object_get_prototype_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_get_prototype_of (param $obj i32) (result i32)
    (local $tag i32)
    (local $payload i32)
    (local $base i32)
    (local $proto i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    ;; NativeError constructor sentinels (AggregateError, Error, etc.)
    ;; are NUMBER-tagged. Their [[Prototype]] is Error.prototype.
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then
        (local.set $payload (i32.shr_u (local.get $obj) (i32.const {number_shift})))
        (if (i32.and
              (i32.ge_u (local.get $payload) (i32.const {native_error_payload_base}))
              (i32.lt_u (local.get $payload) (i32.const {direct_local_token_payload_base})))
          (then
            (return (i32.or (global.get $error_proto_error) (i32.const {object_tag})))))))
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
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            native_error_payload_base = ValueTag::NATIVE_ERROR_PAYLOAD_BASE,
            direct_local_token_payload_base = ValueTag::DIRECT_LOCAL_TOKEN_PAYLOAD_BASE,
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
    ;; Allocate object with 0 initial entries plus the same append headroom
    ;; used by object literals. `$property_set` appends in-place.
    (local.set $obj (call $alloc_heap (i32.const {obj_create_size})))
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
            obj_create_size = Layout::OBJECT_HEADER_SIZE + 8 * Layout::OBJECT_ENTRY_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            zero = RuntimeConst::ZERO,
            null = ValueTag::NULL,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
        ));
    }

    pub(crate) fn emit_object_to_object(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_to_object (param $value i32) (result i32)
    (local $obj i32)
    ;; If the value is already a heap object, return its handle.
    ;; Otherwise, create a new empty object.
    (if (i32.eq (i32.and (local.get $value) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then
        (return (local.get $value))))
    ;; Create empty object with null prototype
    (local.set $obj (call $alloc_heap (i32.const {obj_create_size})))
    (i32.store (local.get $obj) (i32.const {zero}))
    (i32.store (i32.add (local.get $obj) (i32.const {obj_flags})) (i32.const {zero}))
    (i32.store (i32.add (local.get $obj) (i32.const {obj_proto})) (i32.const {zero}))
    (i32.or (local.get $obj) (i32.const {object_tag})))
"#,
            obj_create_size = Layout::OBJECT_HEADER_SIZE + 8 * Layout::OBJECT_ENTRY_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            zero = RuntimeConst::ZERO,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
        ));
    }

    pub(crate) fn emit_object_prototype(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_prototype (result i32)
    (if (i32.eqz (global.get {object_prototype}))
      (then
        (global.set {object_prototype} (call $alloc_heap (i32.const {obj_header})))
        (i32.store (global.get {object_prototype}) (i32.const {zero}))
        (i32.store (i32.add (global.get {object_prototype}) (i32.const {obj_flags})) (i32.const {zero}))
        (i32.store (i32.add (global.get {object_prototype}) (i32.const {obj_proto})) (i32.const {zero}))))
    (i32.or (global.get {object_prototype}) (i32.const {object_tag})))
"#,
            object_prototype = RuntimeGlobal::ObjectPrototypeObject.symbol(),
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            object_tag = ValueTag::OBJECT,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_global_this(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $global_this (result i32)
    (if (i32.eqz (global.get {global_this}))
      (then
        (global.set {global_this} (call $object_create (i32.const {null})))))
    (global.get {global_this}))
"#,
            global_this = RuntimeGlobal::GlobalThisObject.symbol(),
            null = ValueTag::NULL,
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

    pub(crate) fn emit_object_is_prototype_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $is_prototype_of (param $this_proto i32) (param $obj i32) (result i32)
    (local $tag i32)
    (local $proto i32)
    (local $proto_ptr i32)
    (local $this_proto_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {false}))))
    (local.set $this_proto_ptr (i32.and (local.get $this_proto) (i32.const {heap_mask})))
    (local.set $proto (i32.and (local.get $obj) (i32.const {heap_mask})))
    (block $chain_done
      (loop $chain
        (local.set $proto_ptr (i32.load (i32.add (local.get $proto) (i32.const {obj_proto}))))
        (if (i32.eqz (local.get $proto_ptr)) (then (br $chain_done)))
        (if (i32.eq (local.get $proto_ptr) (local.get $this_proto_ptr))
          (then (return (i32.const {true}))))
        (local.set $proto (local.get $proto_ptr))
        (br $chain)))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            false = ValueTag::FALSE,
            true = ValueTag::TRUE,
        ));
    }

    pub(crate) fn emit_object_to_string(&self, wat: &mut String) {
        let str_undefined = self.string_value("[object Undefined]");
        let str_null = self.string_value("[object Null]");
        let str_boolean = self.string_value("[object Boolean]");
        let str_number = self.string_value("[object Number]");
        let str_string = self.string_value("[object String]");
        let str_function = self.string_value("[object Function]");
        let str_array = self.string_value("[object Array]");
        let str_bigint = self.string_value("[object BigInt]");
        let str_symbol = self.string_value("[object Symbol]");
        let str_error = self.string_value("[object Error]");
        let str_object = self.string_value("[object Object]");
        // CLOSURE_SENTINEL = -2, stored at offset 0 of the heap object payload
        const CLOSURE_SENTINEL: i32 = -2;

        // Build [[ErrorData]] internal slot check: Error instances have prototypes
        // matching one of the $error_proto_<Name> globals.
        let error_proto_check: String = {
            let enabled: Vec<BuiltinErrorConstructor> =
                self.builtin_error_prototypes().into_iter().collect();
            if enabled.is_empty() {
                String::new()
            } else {
                let obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET;
                let checks: Vec<String> = enabled
                    .iter()
                    .map(|c| {
                        let global = builtin_error_prototype_global(*c);
                        format!(
                            "(i32.eq (i32.load (i32.add (local.get $base) (i32.const {obj_proto}))) (global.get {global}))"
                        )
                    })
                    .collect();
                format!(
                    ";; [[ErrorData]] — prototype matches an error prototype global
        (if (i32.or\n              {}\n              (i32.const 0))\n          (then (return (i32.const {str_error}))))",
                    checks.join("\n              "),
                    str_error = str_error,
                )
            }
        };
        wat.push_str(&format!(
            r#"
  (func $object_to_string (param $v i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    ;; undefined -> "[object Undefined]"
    (if (i32.eq (local.get $v) (i32.const {undefined_val}))
      (then (return (i32.const {str_undefined}))))
    ;; null -> "[object Null]"
    (if (i32.eq (local.get $v) (i32.const {null_val}))
      (then (return (i32.const {str_null}))))
    ;; boolean -> "[object Boolean]"
    (if (i32.eq (local.get $tag) (i32.const {false_tag}))
      (then (return (i32.const {str_boolean}))))
    (if (i32.eq (local.get $tag) (i32.const {true_tag}))
      (then (return (i32.const {str_boolean}))))
    ;; string -> "[object String]"
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then (return (i32.const {str_string}))))
    ;; ARRAY -> "[object Array]"
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then (return (i32.const {str_array}))))
    ;; number (tag=4) — may be a function token (direct-local or builtin)
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then
        ;; Check builtin function tokens (parseInt / parseFloat)
        (if (i32.or
              (i32.eq (local.get $v) (i32.const {builtin_parse_int}))
              (i32.eq (local.get $v) (i32.const {builtin_parse_float})))
          (then (return (i32.const {str_function}))))
        ;; Check direct-local function tokens
        (if (i32.ge_u
              (i32.shr_u (local.get $v) (i32.const {number_shift}))
              (i32.const {direct_local_payload_base}))
          (then (return (i32.const {str_function}))))
        ;; Plain number -> "[object Number]"
        (return (i32.const {str_number}))))
    ;; OBJECT tag (7) — check subtypes: BigInt, Symbol, HeapClosure, HeapNumber, or plain Object
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
        ;; BigInt -> "[object BigInt]"
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $base) (i32.const {gc_header_size}))
                    (i32.const {gc_flags_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then (return (i32.const {str_bigint}))))
        ;; Symbol -> "[object Symbol]"
        (if (i32.eq (i32.load (local.get $base)) (i32.const {symbol_sentinel}))
          (then (return (i32.const {str_symbol}))))
        ;; HeapClosure -> "[object Function]"
        (if (i32.eq (i32.load (local.get $base)) (i32.const {closure_sentinel}))
          (then (return (i32.const {str_function}))))
        ;; Heap number -> "[object Number]"
        (if (i32.eq (i32.load (local.get $base)) (i32.const {heap_number_sentinel}))
          (then (return (i32.const {str_number}))))
        {error_proto_check}        ;; Plain object -> "[object Object]"
        (return (i32.const {str_object}))))
    ;; Fallback (should not reach here)
    (i32.const {str_object}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            undefined_val = ValueTag::UNDEFINED,
            null_val = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            builtin_parse_int = ValueTag::BUILTIN_PARSE_INT_VALUE,
            builtin_parse_float = ValueTag::BUILTIN_PARSE_FLOAT_VALUE,
            direct_local_payload_base = ValueTag::DIRECT_LOCAL_TOKEN_PAYLOAD_BASE,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_mask = Layout::GC_KIND_MASK,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            symbol_sentinel = Layout::SYMBOL_SENTINEL,
            closure_sentinel = CLOSURE_SENTINEL,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            error_proto_check = error_proto_check,
            str_undefined = str_undefined,
            str_null = str_null,
            str_boolean = str_boolean,
            str_number = str_number,
            str_string = str_string,
            str_function = str_function,
            str_array = str_array,
            str_bigint = str_bigint,
            str_symbol = str_symbol,
            str_object = str_object,
        ));
    }
}
