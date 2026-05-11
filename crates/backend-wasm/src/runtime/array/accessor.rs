use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_array_slice(&self, wat: &mut String) {
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
        (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (i32.const {zero}))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
    (local.set $result_len (i32.sub (local.get $e_pos) (local.get $s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $result_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $result_len)) (i32.const 1)))
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

    pub(crate) fn emit_array_concat(&self, wat: &mut String) {
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
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $result_len)) (i32.const 1)))
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

    pub(crate) fn emit_array_join(&self, wat: &mut String) {
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

    pub(crate) fn emit_array_index_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_index_of (param $arr i32) (param $search i32) (param $from_idx i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {neg_one_tagged}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Clamp fromIndex: untag number, if < 0 use 0, if >= len return -1
    (local.set $i (i32.shr_s (local.get $from_idx) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $i) (i32.const {zero}))
      (then (local.set $i (i32.const {zero}))))
    (if (i32.ge_u (local.get $i) (local.get $len))
      (then (return (i32.const {neg_one_tagged}))))
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
        (if (i32.eq (call $strict_equal (local.get $elem) (local.get $search)) (i32.const {true_tag}))
          (then
            (return
              (i32.or
                (i32.shl (local.get $i) (i32.const {number_shift}))
                (i32.const {number_tag})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {neg_one_tagged}))
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
            neg_one_tagged = ((-1_i32) << 3) | 4,
            true_tag = ValueTag::TRUE,
        ));
    }

    pub(crate) fn emit_array_includes(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_includes (param $arr i32) (param $search i32) (param $from_idx i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {false}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Clamp fromIndex: untag number, if < 0 use 0, if >= len return false
    (local.set $i (i32.shr_s (local.get $from_idx) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $i) (i32.const {zero}))
      (then (local.set $i (i32.const {zero}))))
    (if (i32.ge_u (local.get $i) (local.get $len))
      (then (return (i32.const {false}))))
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
        (if (i32.eq (call $strict_equal (local.get $elem) (local.get $search)) (i32.const {true_tag}))
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
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
        ));
    }

    pub(crate) fn emit_array_find(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_find (param $arr i32) (result i32)
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
        (if (call $truthy_bool (local.get $elem))
          (then (return (local.get $elem))))
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

    pub(crate) fn emit_array_find_index(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_find_index (param $arr i32) (result i32)
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
        (if (call $truthy_bool (local.get $elem))
          (then (return
            (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (i32.shl (i32.const -1) (i32.const {number_shift})) (i32.const {number})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            number = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_find_last(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_find_last (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (if (i32.eqz (local.get $len)) (then (return (i32.const {undefined}))))
    (local.set $i (i32.sub (local.get $len) (i32.const {one})))
    (block $done
      (loop $scan
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (return (local.get $elem))))
        (if (i32.eqz (local.get $i))
          (then (br $done)))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {undefined}))
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

    pub(crate) fn emit_array_find_last_index(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_find_last_index (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {neg_one_tagged}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (if (i32.eqz (local.get $len)) (then (return (i32.const {neg_one_tagged}))))
    (local.set $i (i32.sub (local.get $len) (i32.const {one})))
    (block $done
      (loop $scan
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (return
            (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number})))))
        (if (i32.eqz (local.get $i))
          (then (br $done)))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {neg_one_tagged}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            number = ValueTag::NUMBER,
            one = RuntimeConst::ONE,
            neg_one_tagged = ((-1_i32) << 3) | 4,
        ));
    }

    pub(crate) fn emit_array_last_index_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_last_index_of (param $arr i32) (param $search i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {neg_one_tagged}))))
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
        (if (i32.eq (call $strict_equal (local.get $elem) (local.get $search)) (i32.const {true_tag}))
          (then
            (return
              (i32.or
                (i32.shl (local.get $i) (i32.const {number_shift}))
                (i32.const {number_tag})))))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {neg_one_tagged}))
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
            neg_one_tagged = ((-1_i32) << 3) | 4,
            true_tag = ValueTag::TRUE,
        ));
    }

    pub(crate) fn emit_array_at(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_at (param $arr i32) (param $idx i32) (result i32)
    (local $arr_tag i32)
    (local $idx_tag i32)
    (local $raw_i i32)
    (local $len i32)
    (local.set $arr_tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $arr_tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $idx_tag (i32.and (local.get $idx) (i32.const {tag_mask})))
    (if (i32.ne (local.get $idx_tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $raw_i (i32.shr_s (local.get $idx) (i32.const {number_shift})))
    ;; Normalize negative index: if i < 0, i = max(0, len + i)
    (if (i32.lt_s (local.get $raw_i) (i32.const {zero}))
      (then
        (local.set $raw_i
          (i32.add
            (i32.load (i32.and (local.get $arr) (i32.const {heap_mask})))
            (local.get $raw_i)))
        ;; Clamp to zero if still negative
        (if (i32.lt_s (local.get $raw_i) (i32.const {zero}))
          (then (return (i32.const {undefined}))))))
    ;; Re-tag the index and delegate to $array_get
    (return
      (call $array_get
        (local.get $arr)
        (i32.or (i32.shl (local.get $raw_i) (i32.const {number_shift})) (i32.const {number_tag})))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            number_tag = ValueTag::NUMBER,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
        ));
    }
}
