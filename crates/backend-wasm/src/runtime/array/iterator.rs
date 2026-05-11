use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_array_with(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_with (param $arr i32) (param $idx_tag i32) (param $val i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $idx i32)
    (local $result_ptr i32)
    (local $alloc_size i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $idx (i32.shr_s (local.get $idx_tag) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $idx) (i32.const {zero}))
      (then
        (local.set $idx (i32.add (local.get $len) (local.get $idx)))
        (if (i32.lt_s (local.get $idx) (i32.const {zero}))
          (then (local.set $idx (i32.const {zero}))))))
    (if (i32.ge_s (local.get $idx) (local.get $len))
      (then (local.set $idx (i32.sub (local.get $len) (i32.const {one})))))
    (local.set $alloc_size
      (i32.add
        (i32.const {array_header})
        (i32.shl (local.get $len) (i32.const {elem_shift}))))
    (local.set $result_ptr (call $alloc_heap (local.get $alloc_size)))
    (call $copy (local.get $obj) (local.get $result_ptr) (local.get $alloc_size))
    (i32.store
      (i32.add (local.get $result_ptr)
        (i32.add (i32.const {array_header})
          (i32.shl (local.get $idx) (i32.const {elem_shift}))))
      (local.get $val))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_to_reversed(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_to_reversed (param $arr i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $result_ptr i32)
    (local $alloc_size i32)
    (local $i i32)
    (local $j i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $alloc_size
      (i32.add
        (i32.const {array_header})
        (i32.shl (local.get $len) (i32.const {elem_shift}))))
    (local.set $result_ptr (call $alloc_heap (local.get $alloc_size)))
    (call $copy (local.get $obj) (local.get $result_ptr) (local.get $alloc_size))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (i32.shr_u (local.get $len) (i32.const {one}))))
        (local.set $j (i32.sub (i32.sub (local.get $len) (i32.const {one})) (local.get $i)))
        (local.set $elem
          (i32.load
            (i32.add (local.get $result_ptr)
              (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (i32.store
          (i32.add (local.get $result_ptr)
            (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load
            (i32.add (local.get $result_ptr)
              (i32.add (i32.const {array_header}) (i32.shl (local.get $j) (i32.const {elem_shift}))))))
        (i32.store
          (i32.add (local.get $result_ptr)
            (i32.add (i32.const {array_header}) (i32.shl (local.get $j) (i32.const {elem_shift}))))
          (local.get $elem))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_to_spliced(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_to_spliced (param $arr i32) (param $start_tag i32) (param $delete_count_tag i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $s i32)
    (local $dc i32)
    (local $new_len i32)
    (local $i i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $s (i32.shr_s (local.get $start_tag) (i32.const {number_shift})))
    (local.set $dc (i32.shr_s (local.get $delete_count_tag) (i32.const {number_shift})))
    ;; Clamp start to [0, len]
    (if (i32.lt_s (local.get $s) (i32.const {zero})) (then (local.set $s (i32.const {zero}))))
    (if (i32.gt_u (local.get $s) (local.get $len)) (then (local.set $s (local.get $len))))
    ;; Clamp deleteCount to [0, len - start]
    (if (i32.lt_s (local.get $dc) (i32.const {zero})) (then (local.set $dc (i32.const {zero}))))
    (local.set $new_len (i32.sub (local.get $len) (local.get $s)))
    (if (i32.gt_u (local.get $dc) (local.get $new_len)) (then (local.set $dc (local.get $new_len))))
    (local.set $new_len (i32.sub (local.get $len) (local.get $dc)))
    ;; Allocate new array
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header}) (i32.shl (local.get $new_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $new_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $new_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    ;; Set presence bitmap (dense: bits 0..new_len-1 = 1)
    (block $presence
      (if (i32.eqz (local.get $new_len))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const 0))
          (br $presence)))
      (if (i32.gt_u (local.get $new_len) (i32.const 31))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const -1))
          (br $presence)))
      (i32.store
        (i32.add (local.get $result_ptr) (i32.const 16))
        (i32.sub (i32.shl (i32.const 1) (local.get $new_len)) (i32.const 1))))
    ;; Copy elements 0..start from source
    (local.set $i (i32.const {zero}))
    (block $copy1_done
      (loop $copy1_loop
        (br_if $copy1_done (i32.ge_u (local.get $i) (local.get $s)))
        (i32.store
          (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header})
            (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load
            (i32.add (local.get $obj) (i32.add (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy1_loop)))
    ;; Copy elements start+dc..len from source (shifted left by dc)
    (local.set $i (local.get $s))
    (block $copy2_done
      (loop $copy2_loop
        (br_if $copy2_done (i32.ge_u (local.get $i) (local.get $new_len)))
        (i32.store
          (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header})
            (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load
            (i32.add (local.get $obj) (i32.add (i32.const {array_header})
              (i32.shl (i32.add (local.get $i) (local.get $dc)) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy2_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_to_sorted(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_to_sorted (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $j i32)
    (local $result_ptr i32)
    (local $alloc_size i32)
    (local $left_addr i32)
    (local $right_addr i32)
    (local $left_value i32)
    (local $right_value i32)
    (local $left_num i32)
    (local $right_num i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Allocate and copy the entire array
    (local.set $alloc_size
      (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift}))))
    (local.set $result_ptr (call $alloc_heap (local.get $alloc_size)))
    (call $copy (local.get $obj) (local.get $result_ptr) (local.get $alloc_size))
    (if (i32.lt_u (local.get $len) (i32.const 2))
      (then (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
    ;; Bubble sort the copy (same logic as sort_numeric but on result_ptr)
    (block $outer_done
      (loop $outer_loop
        (br_if $outer_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $j (i32.const {zero}))
        (block $inner_done
          (loop $inner_loop
            (br_if $inner_done
              (i32.ge_u (i32.add (local.get $j) (i32.const {one})) (local.get $len)))
            (local.set $left_addr
              (i32.add (local.get $result_ptr)
                (i32.add (i32.const {array_header})
                  (i32.shl (local.get $j) (i32.const {elem_shift})))))
            (local.set $right_addr (i32.add (local.get $left_addr) (i32.const 4)))
            (local.set $left_value (i32.load (local.get $left_addr)))
            (local.set $right_value (i32.load (local.get $right_addr)))
            (local.set $left_num (call $number_to_i32 (local.get $left_value)))
            (local.set $right_num (call $number_to_i32 (local.get $right_value)))
            (if (i32.gt_s (local.get $left_num) (local.get $right_num))
              (then
                (i32.store (local.get $left_addr) (local.get $right_value))
                (i32.store (local.get $right_addr) (local.get $left_value))))
            (local.set $j (i32.add (local.get $j) (i32.const {one})))
            (br $inner_loop)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $outer_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
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

    pub(crate) fn emit_array_values(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_values (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $alloc_size i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $alloc_size
      (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift}))))
    (local.set $result_ptr (call $alloc_heap (local.get $alloc_size)))
    (call $copy (local.get $obj) (local.get $result_ptr) (local.get $alloc_size))
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

    pub(crate) fn emit_array_keys(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_keys (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Allocate new array: header + len * 4
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    ;; Set presence bitmap (dense: bits 0..len-1 = 1)
    (block $presence
      (if (i32.eqz (local.get $len))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const 0))
          (br $presence)))
      (if (i32.gt_u (local.get $len) (i32.const 31))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const -1))
          (br $presence)))
      (i32.store
        (i32.add (local.get $result_ptr) (i32.const 16))
        (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1))))
    ;; Fill with indices 0, 1, 2, ..., len-1 (tagged as numbers)
    (local.set $i (i32.const {zero}))
    (block $loop_done
      (loop $loop
        (br_if $loop_done (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store
          (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header})
            (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number_tag})))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_entries(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_entries (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $result_ptr i32)
    (local $pair_ptr i32)
    (local $val i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Allocate result array: header + len * 4
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    ;; Set presence bitmap (dense: bits 0..len-1 = 1)
    (block $presence
      (if (i32.eqz (local.get $len))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const 0))
          (br $presence)))
      (if (i32.gt_u (local.get $len) (i32.const 31))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const -1))
          (br $presence)))
      (i32.store
        (i32.add (local.get $result_ptr) (i32.const 16))
        (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1))))
    ;; For each index, create a [index, value] pair array
    (local.set $i (i32.const {zero}))
    (block $loop_done
      (loop $loop
        (br_if $loop_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $val
          (i32.load
            (i32.add (local.get $obj) (i32.add (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        ;; Allocate 2-element pair array
        (local.set $pair_ptr
          (call $alloc_heap
            (i32.add (i32.const {array_header}) (i32.shl (i32.const 2) (i32.const {elem_shift})))))
        (i32.store (local.get $pair_ptr) (i32.const 2))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const 4)) (i32.const 2))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const 8)) (i32.const 1))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const 12)) (i32.const {array_header}))
        ;; Pair presence: 2 elements, bits 0 and 1 set
        (i32.store (i32.add (local.get $pair_ptr) (i32.const 16)) (i32.const 3))
        ;; pair[0] = i (tagged number)
        (i32.store
          (i32.add (local.get $pair_ptr) (i32.add (i32.const {array_header})
            (i32.shl (i32.const 0) (i32.const {elem_shift}))))
          (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number_tag})))
        ;; pair[1] = val
        (i32.store
          (i32.add (local.get $pair_ptr) (i32.add (i32.const {array_header})
            (i32.shl (i32.const 1) (i32.const {elem_shift}))))
          (local.get $val))
        ;; result[i] = pair_ptr | ARRAY_TAG
        (i32.store
          (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header})
            (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.or (local.get $pair_ptr) (i32.const {array_tag})))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_is_array(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_is_array (param $val i32) (result i32)
    (if (result i32)
      (i32.eq (i32.and (local.get $val) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (i32.const {true}))
      (else (i32.const {false})))
  )
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }
}
