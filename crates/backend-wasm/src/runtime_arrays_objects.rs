use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_array_push(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_push (param $arr i32) (param $val i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; TODO: reallocate if needed; for now assume enough space
    (i32.store (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift})))) (local.get $val))
    (local.set $len (i32.add (local.get $len) (i32.const {one})))
    (i32.store (local.get $obj) (local.get $len))
    (i32.or (i32.shl (local.get $len) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_pop(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_pop (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (if (i32.eqz (local.get $len)) (then (return (i32.const {undefined}))))
    (local.set $len (i32.sub (local.get $len) (i32.const {one})))
    (i32.store (local.get $obj) (local.get $len))
    (i32.load (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift}))))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_slice(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_slice (param $arr i32) (param $start i32) (param $end i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $s_pos i32)
    (local $e_pos i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (local $i i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $s_pos (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (local.set $e_pos (i32.shr_s (local.get $end) (i32.const {number_shift})))
    ;; clamp
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero})) (then (local.set $s_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $s_pos) (local.get $len)) (then (local.set $s_pos (local.get $len))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero})) (then (local.set $e_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $e_pos) (local.get $len)) (then (local.set $e_pos (local.get $len))))
    (if (i32.ge_u (local.get $s_pos) (local.get $e_pos))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {array_header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
    (local.set $result_len (i32.sub (local.get $e_pos) (local.get $s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $result_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $s_pos) (i32.const {elem_shift}))))
      (i32.add (local.get $result_ptr) (i32.const {array_header}))
      (i32.shl (local.get $result_len) (i32.const {elem_shift})))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_concat(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_concat (param $a i32) (param $b i32) (result i32)
    (local $a_obj i32)
    (local $b_obj i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_len i32)
    (local $b_len i32)
    (local $result_ptr i32)
    (local $result_len i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (if (i32.ne (local.get $a_tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.ne (local.get $b_tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $a_obj (i32.and (local.get $a) (i32.const {heap_mask})))
    (local.set $b_obj (i32.and (local.get $b) (i32.const {heap_mask})))
    (local.set $a_len (i32.load (local.get $a_obj)))
    (local.set $b_len (i32.load (local.get $b_obj)))
    (local.set $result_len (i32.add (local.get $a_len) (local.get $b_len)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $result_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add (local.get $a_obj) (i32.const {array_header}))
      (i32.add (local.get $result_ptr) (i32.const {array_header}))
      (i32.shl (local.get $a_len) (i32.const {elem_shift})))
    (call $copy
      (i32.add (local.get $b_obj) (i32.const {array_header}))
      (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $a_len) (i32.const {elem_shift}))))
      (i32.shl (local.get $b_len) (i32.const {elem_shift})))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_join(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_join (param $arr i32) (param $sep i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $sep_obj i32)
    (local $sep_len i32)
    (local $i i32)
    (local $elem i32)
    (local $elem_str_len i32)
    (local $total_len i32)
    (local $result_ptr i32)
    (local $write_pos i32)
    (local $sep_tag i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Validate separator is a string
    (local.set $sep_tag (i32.and (local.get $sep) (i32.const {tag_mask})))
    (if (i32.ne (local.get $sep_tag) (i32.const {string_tag})) (then (return (i32.const {undefined}))))
    (local.set $sep_obj (i32.and (local.get $sep) (i32.const {heap_mask})))
    (local.set $sep_len (i32.load (local.get $sep_obj)))
    ;; First pass: calculate total length
    (local.set $total_len (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $calc_done
      (loop $calc_loop
        (br_if $calc_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        ;; Get length of stringified element
        (local.set $elem_str_len (call $value_to_string_into (local.get $elem) (i32.const {scratch_offset})))
        (local.set $total_len (i32.add (local.get $total_len) (local.get $elem_str_len)))
        ;; Add separator length if not last
        (if (i32.lt_u (local.get $i) (i32.sub (local.get $len) (i32.const {one})))
          (then (local.set $total_len (i32.add (local.get $total_len) (local.get $sep_len)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $calc_loop)))
    ;; Allocate result string
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $total_len))))
    (i32.store (local.get $result_ptr) (local.get $total_len))
    (local.set $write_pos (i32.add (local.get $result_ptr) (i32.const {str_header})))
    ;; Second pass: concatenate
    (local.set $i (i32.const {zero}))
    (block $concat_done
      (loop $concat_loop
        (br_if $concat_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        ;; Stringify element to scratch
        (local.set $elem_str_len (call $value_to_string_into (local.get $elem) (i32.const {scratch_offset})))
        ;; Copy to result
        (call $copy (i32.const {scratch_offset}) (local.get $write_pos) (local.get $elem_str_len))
        (local.set $write_pos (i32.add (local.get $write_pos) (local.get $elem_str_len)))
        ;; Add separator if not last
        (if (i32.lt_u (local.get $i) (i32.sub (local.get $len) (i32.const {one})))
          (then
            (call $copy (i32.add (local.get $sep_obj) (i32.const {str_header})) (local.get $write_pos) (local.get $sep_len))
            (local.set $write_pos (i32.add (local.get $write_pos) (local.get $sep_len)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $concat_loop)))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            string_tag = ValueTag::STRING,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            str_header = Layout::STRING_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            scratch_offset = Layout::SCRATCH_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_reverse(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_reverse (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $j i32)
    (local $left_idx i32)
    (local $right_idx i32)
    (local $temp i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Reverse in-place: swap arr[i] with arr[len-1-i]
    (local.set $i (i32.const {zero}))
    (block $reverse_done
      (loop $reverse_loop
        (br_if $reverse_done (i32.ge_u (local.get $i) (i32.shr_u (local.get $len) (i32.const {one}))))
        (local.set $j (i32.sub (local.get $len) (i32.const {one})))
        (local.set $j (i32.sub (local.get $j) (local.get $i)))
        ;; Swap arr[i] and arr[j]
        (local.set $left_idx (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift})))))
        (local.set $right_idx (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $j) (i32.const {elem_shift})))))
        (local.set $temp (i32.load (local.get $left_idx)))
        (i32.store (local.get $left_idx) (i32.load (local.get $right_idx)))
        (i32.store (local.get $right_idx) (local.get $temp))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $reverse_loop)))
    (local.get $arr))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    // Object methods (M10)

    pub(super) fn emit_object_keys(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_keys (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $key i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    ;; Allocate result array
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $count))
    ;; Extract all keys
    (local.set $i (i32.const {zero}))
    (block $keys_done
      (loop $keys_loop
        (br_if $keys_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $key (i32.load (local.get $entry_base)))
        ;; Store key in result array
        (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift})))) (local.get $key))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $keys_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            array_tag = ValueTag::ARRAY,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_object_values(&self, wat: &mut String) {
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

    pub(super) fn emit_object_entries(&self, wat: &mut String) {
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
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            pair_size = 8,  // 2 elements * 4 bytes
            array_header_plus_4 = Layout::ARRAY_HEADER_SIZE + 4,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            two = 2,
            array_tag = ValueTag::ARRAY,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_greater_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $greater_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (i32.ge_s (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_greater_equal_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $greater_equal_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return
          (if (result i32)
            (i32.ge_s
              (i32.shr_s (local.get $a) (i32.const {number_shift}))
              (i32.shr_s (local.get $b) (i32.const {number_shift})))
            (then (i32.const {true_tag}))
            (else (i32.const {false_tag}))))))
    (call $greater_equal (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_object_get_prototype_of(&self, wat: &mut String) {
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

    pub(super) fn emit_object_set_prototype_of(&self, wat: &mut String) {
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

    pub(super) fn emit_instanceof(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $instanceof (param $obj i32) (param $constructor i32) (result i32)
    ;; For now, return false for all instanceof checks
    ;; Full implementation requires prototype chain traversal
    (i32.const {false}))
"#,
            false = ValueTag::FALSE,
        ));
    }

    // Math functions (M10)
}
