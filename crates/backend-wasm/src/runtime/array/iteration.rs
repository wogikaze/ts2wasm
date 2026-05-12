use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_array_map_value_to_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_value_to_string (param $arr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $mapped_len i32)
    (local $mapped_ptr i32)
    (local $result_ptr i32)
    (if (i32.ne (i32.and (local.get $arr) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
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
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $mapped_len
          (call $value_to_string_into (local.get $elem) (i32.const {scratch_offset})))
        (local.set $mapped_ptr
          (call $alloc_heap
            (i32.add (i32.const {string_header}) (local.get $mapped_len))))
        (i32.store (local.get $mapped_ptr) (local.get $mapped_len))
        (call $copy
          (i32.const {scratch_offset})
          (i32.add (local.get $mapped_ptr) (i32.const {string_header}))
          (local.get $mapped_len))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.or (local.get $mapped_ptr) (i32.const {string_tag})))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            string_tag = ValueTag::STRING,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            string_header = Layout::STRING_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            scratch_offset = Layout::SCRATCH_OFFSET,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_map_unary_plus(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_unary_plus (param $arr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (if (i32.ne (i32.and (local.get $arr) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
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
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (call $primitive_to_number_for_equality (local.get $elem)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
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

    pub(crate) fn emit_array_map_string_split(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_string_split (param $arr i32) (param $sep i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $mapped i32)
    (local $result_ptr i32)
    (if (i32.ne (i32.and (local.get $arr) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
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
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $mapped (call $string_split (local.get $elem) (local.get $sep)))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (local.get $mapped))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
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

    pub(crate) fn emit_array_map_array_like_identity(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_array_like_identity (param $receiver i32) (result i32)
    (local $len_value i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (local.set $len_value (call $get_length (local.get $receiver)))
    (if
      (i32.ne
        (i32.and (local.get $len_value) (i32.const {tag_mask}))
        (i32.const {number_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $len (i32.shr_s (local.get $len_value) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $len) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
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
            (local.get $receiver)
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
            number_tag = ValueTag::NUMBER,
            array_tag = ValueTag::ARRAY,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_map_array_like_double(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_array_like_double (param $receiver i32) (result i32)
    (local $len_value i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (local.set $len_value (call $get_length (local.get $receiver)))
    (if
      (i32.ne
        (i32.and (local.get $len_value) (i32.const {tag_mask}))
        (i32.const {number_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $len (i32.shr_s (local.get $len_value) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $len) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
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
            (local.get $receiver)
            (i32.or
              (i32.shl (local.get $i) (i32.const {number_shift}))
              (i32.const {number_tag}))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (call $mul
            (local.get $elem)
            (i32.or
              (i32.shl (i32.const 2) (i32.const {number_shift}))
              (i32.const {number_tag}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            array_tag = ValueTag::ARRAY,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_sort_numeric(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_sort_numeric (param $arr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $j i32)
    (local $left_addr i32)
    (local $right_addr i32)
    (local $left_value i32)
    (local $right_value i32)
    (local $left_num i32)
    (local $right_num i32)
    (if (i32.ne (i32.and (local.get $arr) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (if (i32.lt_u (local.get $len) (i32.const 2)) (then (return (local.get $arr))))
    (block $outer_done
      (loop $outer_loop
        (br_if $outer_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $j (i32.const {zero}))
        (block $inner_done
          (loop $inner_loop
            (br_if $inner_done
              (i32.ge_u
                (i32.add (local.get $j) (i32.const {one}))
                (local.get $len)))
            (local.set $left_addr
              (i32.add
                (local.get $obj)
                (i32.add
                  (i32.const {array_header})
                  (i32.shl (local.get $j) (i32.const {elem_shift})))))
            (local.set $right_addr
              (i32.add (local.get $left_addr) (i32.const 4)))
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

    pub(crate) fn emit_array_filter(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_filter (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; First pass: count truthy elements
    (local.set $result_len (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $count_done
      (loop $count_loop
        (br_if $count_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (local.set $result_len (i32.add (local.get $result_len) (i32.const {one})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $count_loop)))
    ;; Allocate result array
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $result_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $result_len)) (i32.const 1)))
    ;; Second pass: copy truthy elements
    (local.set $result_len (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $copy_done
      (loop $copy_loop
        (br_if $copy_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then
            (i32.store
              (i32.add
                (local.get $result_ptr)
                (i32.add
                  (i32.const {array_header})
                  (i32.shl (local.get $result_len) (i32.const {elem_shift}))))
              (local.get $elem))
            (local.set $result_len (i32.add (local.get $result_len) (i32.const {one})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy_loop)))
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

    pub(crate) fn emit_array_every(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_every (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {false}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (i32.eqz (call $truthy_bool (local.get $elem)))
          (then (return (i32.const {false}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {true}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_array_some(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_some (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {false}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (return (i32.const {true}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_array_reduce(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_reduce (param $arr i32) (param $callback i32) (param $initial i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (local.get $initial))
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

    pub(crate) fn emit_array_reduce_right(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_reduce_right (param $arr i32) (param $callback i32) (param $initial i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.sub (local.get $len) (i32.const {one})))
    (block $done
      (loop $scan
        (br_if $done (i32.lt_s (local.get $i) (i32.const {zero})))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $scan)))
    (local.get $initial))
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

    pub(crate) fn emit_array_for_each(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_for_each (param $arr i32) (param $callback i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {undefined}))
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

    pub(crate) fn emit_array_map(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map (param $arr i32) (param $callback i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
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
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
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
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_flat(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_flat (param $arr i32) (param $depth i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $elem_tag i32)
    (local $elem_obj i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (local $j i32)
    (local $flat_sub i32)
    (local $flat_obj i32)
    (local $flat_len i32)
    (local $flat_elem i32)
    (local $depth_raw i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $depth
      (select
        (local.get $depth)
        (i32.const {one_tagged})
        (i32.eq (i32.and (local.get $depth) (i32.const {tag_mask})) (i32.const {number_tag}))))
    (local.set $depth_raw (i32.shr_s (local.get $depth) (i32.const {number_shift})))
    (if (i32.le_s (local.get $depth_raw) (i32.const {zero})) (then (return (local.get $arr))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (local.set $result_len (i32.const {zero}))
    (block $count_done
      (loop $count_loop
        (br_if $count_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add (local.get $obj)
              (i32.add (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $elem_tag (i32.and (local.get $elem) (i32.const {tag_mask})))
        (block $count_skip_simple
          (block $count_try_recursion
            (if (i32.ne (local.get $elem_tag) (i32.const {array_tag}))
              (then (br $count_try_recursion)))
            (if (i32.le_s (local.get $depth_raw) (i32.const {zero}))
              (then (br $count_try_recursion)))
            (local.set $flat_sub
              (call $array_flat (local.get $elem) (i32.sub (local.get $depth) (i32.const {one_tagged}))))
            (local.set $result_len
              (i32.add (local.get $result_len)
                (i32.load (i32.and (local.get $flat_sub) (i32.const {heap_mask})))))
            (br $count_skip_simple))
          (local.set $result_len (i32.add (local.get $result_len) (i32.const {one}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $count_loop)))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header})
          (i32.shl (local.get $result_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (block $presence
      (if (i32.eqz (local.get $result_len))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const 0))
          (br $presence)))
      (if (i32.gt_u (local.get $result_len) (i32.const 31))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const -1))
          (br $presence)))
      (i32.store
        (i32.add (local.get $result_ptr) (i32.const 16))
        (i32.sub (i32.shl (i32.const 1) (local.get $result_len)) (i32.const 1))))
    (local.set $i (i32.const {zero}))
    (local.set $result_len (i32.const {zero}))
    (block $copy_done
      (loop $copy_loop
        (br_if $copy_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add (local.get $obj)
              (i32.add (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $elem_tag (i32.and (local.get $elem) (i32.const {tag_mask})))
        (block $copy_skip_simple
          (block $copy_try_recursion
            (if (i32.ne (local.get $elem_tag) (i32.const {array_tag}))
              (then (br $copy_try_recursion)))
            (if (i32.le_s (local.get $depth_raw) (i32.const {zero}))
              (then (br $copy_try_recursion)))
            (local.set $flat_sub
              (call $array_flat (local.get $elem) (i32.sub (local.get $depth) (i32.const {one_tagged}))))
            (local.set $flat_obj (i32.and (local.get $flat_sub) (i32.const {heap_mask})))
            (local.set $flat_len (i32.load (local.get $flat_obj)))
            (local.set $j (i32.const {zero}))
            (block $flat_done
              (loop $flat_loop
                (br_if $flat_done (i32.ge_u (local.get $j) (local.get $flat_len)))
                (local.set $flat_elem
                  (i32.load
                    (i32.add (local.get $flat_obj)
                      (i32.add (i32.const {array_header})
                        (i32.shl (local.get $j) (i32.const {elem_shift}))))))
                (i32.store
                  (i32.add (local.get $result_ptr)
                    (i32.add (i32.const {array_header})
                      (i32.shl (local.get $result_len) (i32.const {elem_shift}))))
                  (local.get $flat_elem))
                (local.set $result_len (i32.add (local.get $result_len) (i32.const {one})))
                (local.set $j (i32.add (local.get $j) (i32.const {one})))
                (br $flat_loop)))
            (br $copy_skip_simple))
          (i32.store
            (i32.add (local.get $result_ptr)
              (i32.add (i32.const {array_header})
                (i32.shl (local.get $result_len) (i32.const {elem_shift}))))
            (local.get $elem))
          (local.set $result_len (i32.add (local.get $result_len) (i32.const {one}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy_loop)))
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
            one_tagged = (RuntimeConst::ONE << ValueTag::NUMBER_SHIFT) | ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }
}
