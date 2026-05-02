use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_string_char_at(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_char_at (param $s i32) (param $idx i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.shr_s (local.get $idx) (i32.const {number_shift})))
    (if (i32.or (i32.lt_s (local.get $i) (i32.const {zero})) (i32.ge_u (local.get $i) (local.get $len)))
      (then (return (i32.const {undefined}))))
    ;; allocate 1-byte string for char
    (local.set $obj (call $alloc_heap (i32.const {char_size})))
    (i32.store (local.get $obj) (i32.const {one}))
    (i32.store8
      (i32.add (local.get $obj) (i32.const {header}))
      (i32.load8_u
        (i32.add
          (i32.and (local.get $s) (i32.const {heap_mask}))
          (i32.add (i32.const {header}) (local.get $i)))))
    (i32.or (local.get $obj) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            char_size = Layout::STRING_HEADER_SIZE + 1,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(super) fn emit_string_substring(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_substring (param $s i32) (param $start i32) (param $end i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $s_pos i32)
    (local $e_pos i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $s_pos (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (local.set $e_pos (i32.shr_s (local.get $end) (i32.const {number_shift})))
    ;; clamp to [0, len]
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero})) (then (local.set $s_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $s_pos) (local.get $len)) (then (local.set $s_pos (local.get $len))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero})) (then (local.set $e_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $e_pos) (local.get $len)) (then (local.set $e_pos (local.get $len))))
    ;; if start >= end, return empty string
    (if (i32.ge_u (local.get $s_pos) (local.get $e_pos))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    (local.set $result_len (i32.sub (local.get $e_pos) (local.get $s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $result_len))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.add (i32.const {header}) (local.get $s_pos)))
      (i32.add (local.get $result_ptr) (i32.const {header}))
      (local.get $result_len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(super) fn emit_string_slice(&self, wat: &mut String) {
        // ES slice: negative indices count from end, defaults applied
        wat.push_str(&format!(
            r#"
  (func $string_slice (param $s i32) (param $start i32) (param $end i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $s_pos i32)
    (local $e_pos i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $s_pos (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (local.set $e_pos (i32.shr_s (local.get $end) (i32.const {number_shift})))
    ;; handle negative indices
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero}))
      (then (local.set $s_pos (i32.add (local.get $len) (local.get $s_pos)))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero}))
      (then (local.set $e_pos (i32.add (local.get $len) (local.get $e_pos)))))
    ;; clamp to [0, len]
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero})) (then (local.set $s_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $s_pos) (local.get $len)) (then (local.set $s_pos (local.get $len))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero})) (then (local.set $e_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $e_pos) (local.get $len)) (then (local.set $e_pos (local.get $len))))
    ;; if start >= end, return empty string
    (if (i32.ge_u (local.get $s_pos) (local.get $e_pos))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    (local.set $result_len (i32.sub (local.get $e_pos) (local.get $s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $result_len))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.add (i32.const {header}) (local.get $s_pos)))
      (i32.add (local.get $result_ptr) (i32.const {header}))
      (local.get $result_len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(super) fn emit_string_index_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_index_of (param $haystack i32) (param $needle i32) (result i32)
    (local $h_obj i32)
    (local $n_obj i32)
    (local $h_len i32)
    (local $n_len i32)
    (local $i i32)
    (if (i32.eqz (call $is_string (local.get $haystack))) (then (return (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.eqz (call $is_string (local.get $needle))) (then (return (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (local.set $h_obj (i32.and (local.get $haystack) (i32.const {heap_mask})))
    (local.set $n_obj (i32.and (local.get $needle) (i32.const {heap_mask})))
    (local.set $h_len (i32.load (local.get $h_obj)))
    (local.set $n_len (i32.load (local.get $n_obj)))
    (if (i32.eqz (local.get $n_len)) (then (return (i32.const {zero}))))
    (block $not_found
      (loop $search
        (br_if $not_found (i32.gt_u (local.get $i) (i32.sub (local.get $h_len) (local.get $n_len))))
        (if (call $mem_equal
              (i32.add (i32.add (local.get $h_obj) (i32.const {header})) (local.get $i))
              (i32.add (local.get $n_obj) (i32.const {header}))
              (local.get $n_len))
          (then (return (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number_tag})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $search)))
    (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            neg_one = -1i32,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(super) fn emit_string_includes(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_includes (param $haystack i32) (param $needle i32) (result i32)
    (local $h_obj i32)
    (local $n_obj i32)
    (local $h_len i32)
    (local $n_len i32)
    (local $i i32)
    (if (i32.eqz (call $is_string (local.get $haystack))) (then (return (i32.const {false_tag}))))
    (if (i32.eqz (call $is_string (local.get $needle))) (then (return (i32.const {false_tag}))))
    (local.set $h_obj (i32.and (local.get $haystack) (i32.const {heap_mask})))
    (local.set $n_obj (i32.and (local.get $needle) (i32.const {heap_mask})))
    (local.set $h_len (i32.load (local.get $h_obj)))
    (local.set $n_len (i32.load (local.get $n_obj)))
    (if (i32.eqz (local.get $n_len)) (then (return (i32.const {true_tag}))))
    (block $not_found
      (loop $search
        (br_if $not_found (i32.gt_u (local.get $i) (i32.sub (local.get $h_len) (local.get $n_len))))
        (if (call $mem_equal
              (i32.add (i32.add (local.get $h_obj) (i32.const {header})) (local.get $i))
              (i32.add (local.get $n_obj) (i32.const {header}))
              (local.get $n_len))
          (then (return (i32.const {true_tag}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $search)))
    (i32.const {false_tag}))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            one = RuntimeConst::ONE,
        ));
    }

    pub(super) fn emit_string_split(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_split (param $s i32) (param $sep i32) (result i32)
    (local $s_obj i32)
    (local $sep_obj i32)
    (local $s_len i32)
    (local $sep_len i32)
    (local $count i32)
    (local $i i32)
    (local $j i32)
    (local $part_start i32)
    (local $result_ptr i32)
    (local $part_ptr i32)
    (local $part_len i32)
    (local $result_idx i32)
    (local $part_value i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (if (i32.eqz (call $is_string (local.get $sep))) (then (return (i32.const {undefined}))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $sep_obj (i32.and (local.get $sep) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $sep_len (i32.load (local.get $sep_obj)))
    (if (i32.eqz (local.get $sep_len)) (then (return (i32.const {undefined}))))
    ;; First pass: count splits (count occurrences of sep + 1)
    (local.set $count (i32.const {one}))
    (local.set $i (i32.const {zero}))
    (block $count_done
      (loop $count_loop
        (br_if $count_done (i32.gt_u (local.get $i) (i32.sub (local.get $s_len) (local.get $sep_len))))
        (if (call $mem_equal
              (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $i))
              (i32.add (local.get $sep_obj) (i32.const {str_header}))
              (local.get $sep_len))
          (then
            (local.set $count (i32.add (local.get $count) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (local.get $sep_len)))
            (br $count_loop)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $count_loop)))
    ;; Allocate result array
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $count))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $count)) (i32.const 1)))
    ;; Second pass: extract parts
    (local.set $result_idx (i32.const {zero}))
    (local.set $part_start (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $split_done
      (loop $split_loop
        (br_if $split_done (i32.ge_u (local.get $i) (local.get $s_len)))
        (if (i32.le_u (i32.add (local.get $i) (local.get $sep_len)) (local.get $s_len))
          (then
            (if (call $mem_equal
                  (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $i))
                  (i32.add (local.get $sep_obj) (i32.const {str_header}))
                  (local.get $sep_len))
              (then
                ;; Found separator: extract part from part_start to i
                (local.set $part_len (i32.sub (local.get $i) (local.get $part_start)))
                (local.set $part_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $part_len))))
                (i32.store (local.get $part_ptr) (local.get $part_len))
                (call $copy
                  (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $part_start))
                  (i32.add (local.get $part_ptr) (i32.const {str_header}))
                  (local.get $part_len))
                (local.set $part_value (i32.or (local.get $part_ptr) (i32.const {string_tag})))
                (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $result_idx) (i32.const {elem_shift})))) (local.get $part_value))
                (local.set $result_idx (i32.add (local.get $result_idx) (i32.const {one})))
                (local.set $i (i32.add (local.get $i) (local.get $sep_len)))
                (local.set $part_start (local.get $i))
                (br $split_loop)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $split_loop)))
    ;; Handle final part
    (local.set $part_len (i32.sub (local.get $s_len) (local.get $part_start)))
    (local.set $part_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $part_len))))
    (i32.store (local.get $part_ptr) (local.get $part_len))
    (call $copy
      (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $part_start))
      (i32.add (local.get $part_ptr) (i32.const {str_header}))
      (local.get $part_len))
    (local.set $part_value (i32.or (local.get $part_ptr) (i32.const {string_tag})))
    (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $result_idx) (i32.const {elem_shift})))) (local.get $part_value))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            array_header = Layout::ARRAY_HEADER_SIZE,
            str_header = Layout::STRING_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(super) fn emit_string_trim(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_trim (param $s i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $start i32)
    (local $end i32)
    (local $ch i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $end (local.get $len))
    (block $trim_leading_done
      (loop $trim_leading
        (br_if $trim_leading_done (i32.ge_u (local.get $start) (local.get $end)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {header}))
              (local.get $start))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $start (i32.add (local.get $start) (i32.const {one})))
            (br $trim_leading))
          (else (br $trim_leading_done)))))
    (block $trim_trailing_done
      (loop $trim_trailing
        (br_if $trim_trailing_done (i32.le_u (local.get $end) (local.get $start)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {header}))
              (i32.sub (local.get $end) (i32.const {one})))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $end (i32.sub (local.get $end) (i32.const {one})))
            (br $trim_trailing))
          (else (br $trim_trailing_done)))))
    (if
      (i32.and
        (i32.eqz (local.get $start))
        (i32.eq (local.get $end) (local.get $len)))
      (then (return (local.get $s))))
    (local.set $result_len (i32.sub (local.get $end) (local.get $start)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $result_len))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add
        (i32.add (local.get $obj) (i32.const {header}))
        (local.get $start))
      (i32.add (local.get $result_ptr) (i32.const {header}))
      (local.get $result_len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            one = RuntimeConst::ONE,
            ascii_tab = 9,
            ascii_cr = 13,
            ascii_space = 32,
        ));
    }

    pub(super) fn emit_string_to_upper_case(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_to_upper_case (param $s i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $len))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {header}))
              (local.get $i))))
        (if
          (i32.and
            (i32.ge_u (local.get $ch) (i32.const {ascii_a}))
            (i32.le_u (local.get $ch) (i32.const {ascii_z})))
          (then
            (local.set $ch (i32.sub (local.get $ch) (i32.const {ascii_case_delta})))))
        (i32.store8
          (i32.add
            (i32.add (local.get $result_ptr) (i32.const {header}))
            (local.get $i))
          (local.get $ch))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            one = RuntimeConst::ONE,
            ascii_a = 97,
            ascii_z = 122,
            ascii_case_delta = 32,
        ));
    }

    pub(super) fn emit_string_to_lower_case(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_to_lower_case (param $s i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $len))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {header}))
              (local.get $i))))
        (if
          (i32.and
            (i32.ge_u (local.get $ch) (i32.const {ascii_a_upper}))
            (i32.le_u (local.get $ch) (i32.const {ascii_z_upper})))
          (then
            (local.set $ch (i32.add (local.get $ch) (i32.const {ascii_case_delta})))))
        (i32.store8
          (i32.add
            (i32.add (local.get $result_ptr) (i32.const {header}))
            (local.get $i))
          (local.get $ch))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            one = RuntimeConst::ONE,
            ascii_a_upper = 65,
            ascii_z_upper = 90,
            ascii_case_delta = 32,
        ));
    }

    pub(super) fn emit_string_char_code_at(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_char_code_at (param $s i32) (param $index i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $idx i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $idx (i32.shr_s (local.get $index) (i32.const {number_shift})))
    ;; Handle negative index
    (if (i32.lt_s (local.get $idx) (i32.const {zero}))
      (then (local.set $idx (i32.add (local.get $len) (local.get $idx)))))
    ;; Clamp to [0, len)
    (if (i32.lt_s (local.get $idx) (i32.const {zero})) (then (local.set $idx (i32.const {zero}))))
    (if (i32.ge_u (local.get $idx) (local.get $len)) (then (return (i32.const {undefined}))))
    ;; Get character code
    (i32.or (i32.shl (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {string_header})) (local.get $idx))) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
            string_header = Layout::STRING_HEADER_SIZE,
        ));
    }

    pub(super) fn emit_string_from_char_code(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_from_char_code (param $code i32) (result i32)
    (local $code_num i32)
    (local $result_ptr i32)
    (local.set $code_num (i32.shr_s (local.get $code) (i32.const {number_shift})))
    ;; Clamp to valid Unicode range (0-65535)
    (if (i32.lt_s (local.get $code_num) (i32.const {zero})) (then (local.set $code_num (i32.const {zero}))))
    (if (i32.gt_u (local.get $code_num) (i32.const 65535)) (then (local.set $code_num (i32.const 65535))))
    ;; Allocate single-character string
    (local.set $result_ptr (call $alloc_heap (i32.const {single_char_size})))
    (i32.store (local.get $result_ptr) (i32.const {one}))
    (i32.store8 (i32.add (local.get $result_ptr) (i32.const {string_header})) (local.get $code_num))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            single_char_size = Layout::STRING_HEADER_SIZE + 1,
            string_header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    // Array methods (M10)
}
