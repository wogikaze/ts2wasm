use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_string_char_at(&self, wat: &mut String) {
        // UTF-8 helpers are emitted via emit_utf8_helpers() before any string functions.
        wat.push_str(&format!(
            r#"
  (func $string_char_at (param $s i32) (param $idx i32) (result i32)
    (local $obj i32)
    (local $i i32)
    (local $byte_len i32)
    (local $byte_pos i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $i (i32.shr_s (local.get $idx) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $i) (i32.const {zero})) (then (return (i32.const {undefined}))))
    (local.set $byte_pos (call $utf8_cp_to_byte_index (local.get $obj) (local.get $i)))
    (if (i32.ge_u (local.get $byte_pos) (i32.load (local.get $obj)))
      (then (return (i32.const {undefined}))))
    (local.set $byte_len (call $utf8_cp_byte_length (local.get $obj) (local.get $byte_pos)))
    (local.set $obj (call $alloc_heap (i32.add (i32.const {header}) (local.get $byte_len))))
    (i32.store (local.get $obj) (local.get $byte_len))
    (call $copy
      (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.add (i32.const {header}) (local.get $byte_pos)))
      (i32.add (local.get $obj) (i32.const {header}))
      (local.get $byte_len))
    (i32.or (local.get $obj) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_string_at(&self, wat: &mut String) {
        // UTF-8 helpers are emitted via emit_utf8_helpers() before any string functions.
        self.emit_string_code_point_length(wat);
        wat.push_str(&format!(
            r#"
  (func $string_at (param $s i32) (param $idx i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $byte_len i32)
    (local $byte_pos i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (call $utf8_cp_count (local.get $obj)))
    (local.set $i (i32.shr_s (local.get $idx) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $i) (i32.const {zero}))
      (then
        (local.set $i
          (i32.add
            (call $string_code_point_length (local.get $s))
            (local.get $i)))
        (if (i32.lt_s (local.get $i) (i32.const {zero}))
          (then (return (i32.const {undefined}))))))
    (if (i32.ge_u (local.get $i) (local.get $len))
      (then (return (i32.const {undefined}))))
    (local.set $byte_pos (call $utf8_cp_to_byte_index (local.get $obj) (local.get $i)))
    (local.set $byte_len (call $utf8_cp_byte_length (local.get $obj) (local.get $byte_pos)))
    (local.set $obj (call $alloc_heap (i32.add (i32.const {header}) (local.get $byte_len))))
    (i32.store (local.get $obj) (local.get $byte_len))
    (call $copy
      (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.add (i32.const {header}) (local.get $byte_pos)))
      (i32.add (local.get $obj) (i32.const {header}))
      (local.get $byte_len))
    (i32.or (local.get $obj) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_string_substring(&self, wat: &mut String) {
        // UTF-8 helpers are emitted via emit_utf8_helpers() before any string functions.
        self.emit_string_code_point_length(wat);
        wat.push_str(&format!(
            r#"
  (func $string_substring (param $s i32) (param $start i32) (param $end i32) (result i32)
    (local $obj i32)
    (local $byte_len i32)
    (local $cp_len i32)
    (local $s_pos i32)
    (local $e_pos i32)
    (local $byte_s_pos i32)
    (local $byte_e_pos i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (local $b i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $obj)))
    (local.set $cp_len (call $string_code_point_length (local.get $s)))
    (local.set $s_pos (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (local.set $e_pos (i32.shr_s (local.get $end) (i32.const {number_shift})))
    ;; if $end is undefined (ValueTag::UNDEFINED == 0), default to cp_len
    (if (i32.eqz (local.get $end))
      (then (local.set $e_pos (local.get $cp_len))))
    ;; clamp to [0, cp_len]
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero})) (then (local.set $s_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $s_pos) (local.get $cp_len)) (then (local.set $s_pos (local.get $cp_len))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero})) (then (local.set $e_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $e_pos) (local.get $cp_len)) (then (local.set $e_pos (local.get $cp_len))))
    ;; if start >= end, return empty string
    (if (i32.ge_u (local.get $s_pos) (local.get $e_pos))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    ;; Convert s_pos and e_pos from code point indices to byte indices
    (if (i32.eq (local.get $s_pos) (local.get $cp_len))
      (then (local.set $byte_s_pos (local.get $byte_len)))
      (else (local.set $byte_s_pos (call $utf8_cp_to_byte_index (local.get $obj) (local.get $s_pos)))))
    (if (i32.eq (local.get $e_pos) (local.get $cp_len))
      (then (local.set $byte_e_pos (local.get $byte_len)))
      (else (local.set $byte_e_pos (call $utf8_cp_to_byte_index (local.get $obj) (local.get $e_pos)))))
    (local.set $result_len (i32.sub (local.get $byte_e_pos) (local.get $byte_s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $result_len))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.add (i32.const {header}) (local.get $byte_s_pos)))
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

    pub(crate) fn emit_string_substr(&self, wat: &mut String) {
        self.emit_string_code_point_length(wat);
        wat.push_str(&format!(
            r#"
  (func $string_substr (param $s i32) (param $start i32) (param $len i32) (result i32)
    (local $obj i32)
    (local $byte_len i32)
    (local $cp_len i32)
    (local $s_pos i32)
    (local $len_val i32)
    (local $e_pos i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $obj)))
    (local.set $cp_len (call $string_code_point_length (local.get $s)))
    (local.set $s_pos (i32.shr_s (local.get $start) (i32.const {number_shift})))
    ;; Handle negative start: max(cp_len + start, 0)
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero}))
      (then
        (local.set $s_pos (i32.add (local.get $cp_len) (local.get $s_pos)))
        (if (i32.lt_s (local.get $s_pos) (i32.const {zero}))
          (then (local.set $s_pos (i32.const {zero}))))))
    (if (i32.gt_u (local.get $s_pos) (local.get $cp_len))
      (then (local.set $s_pos (local.get $cp_len))))
    ;; If start >= cp_len, return empty string (call substring with 0,0)
    (if (i32.ge_u (local.get $s_pos) (local.get $cp_len))
      (then
        (return (call $string_substring (local.get $s) (i32.const {zero}) (i32.const {zero})))))
    ;; Decode length parameter
    ;; Check if $len has the number tag (tag == 4)
    (if (i32.eq (i32.and (local.get $len) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then
        (local.set $len_val (i32.shr_s (local.get $len) (i32.const {number_shift})))
        ;; If decoded length <= 0, return empty string
        (if (i32.le_s (local.get $len_val) (i32.const {zero}))
          (then
            (return (call $string_substring (local.get $s) (i32.const {zero}) (i32.const {zero}))))))
      (else
        ;; Non-number length
        ;; If undefined (tagged 0): go to end of string (s_pos to end)
        ;; Otherwise: treat as 0 (return empty string)
        (if (i32.eqz (local.get $len))
          (then
            (local.set $len_val (i32.sub (local.get $cp_len) (local.get $s_pos))))
          (else
            (return (call $string_substring (local.get $s) (i32.const {zero}) (i32.const {zero})))))))
    ;; Compute end position and delegate to substring
    (local.set $e_pos (i32.add (local.get $s_pos) (local.get $len_val)))
    (if (i32.gt_u (local.get $e_pos) (local.get $cp_len))
      (then (local.set $e_pos (local.get $cp_len))))
    (return (call $string_substring (local.get $s)
      (i32.shl (local.get $s_pos) (i32.const {number_shift}))
      (i32.shl (local.get $e_pos) (i32.const {number_shift})))))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_string_slice(&self, wat: &mut String) {
        // ES slice: negative indices count from end, defaults applied
        // UTF-8 helpers are emitted via emit_utf8_helpers() before any string functions.
        self.emit_string_code_point_length(wat);
        wat.push_str(&format!(
            r#"
  (func $string_slice (param $s i32) (param $start i32) (param $end i32) (result i32)
    (local $obj i32)
    (local $byte_len i32)
    (local $cp_len i32)
    (local $s_pos i32)
    (local $e_pos i32)
    (local $byte_s_pos i32)
    (local $byte_e_pos i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $obj)))
    (local.set $cp_len (call $string_code_point_length (local.get $s)))
    (local.set $s_pos (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (local.set $e_pos (i32.shr_s (local.get $end) (i32.const {number_shift})))
    ;; if $end is undefined (ValueTag::UNDEFINED == 0), default to cp_len
    (if (i32.eqz (local.get $end))
      (then (local.set $e_pos (local.get $cp_len))))
    ;; handle negative indices using code point length
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero}))
      (then (local.set $s_pos (i32.add (local.get $cp_len) (local.get $s_pos)))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero}))
      (then (local.set $e_pos (i32.add (local.get $cp_len) (local.get $e_pos)))))
    ;; clamp to [0, cp_len]
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero})) (then (local.set $s_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $s_pos) (local.get $cp_len)) (then (local.set $s_pos (local.get $cp_len))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero})) (then (local.set $e_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $e_pos) (local.get $cp_len)) (then (local.set $e_pos (local.get $cp_len))))
    ;; if start >= end, return empty string
    (if (i32.ge_u (local.get $s_pos) (local.get $e_pos))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    ;; Convert s_pos and e_pos from code point indices to byte indices
    (if (i32.eq (local.get $s_pos) (local.get $cp_len))
      (then (local.set $byte_s_pos (local.get $byte_len)))
      (else (local.set $byte_s_pos (call $utf8_cp_to_byte_index (local.get $obj) (local.get $s_pos)))))
    (if (i32.eq (local.get $e_pos) (local.get $cp_len))
      (then (local.set $byte_e_pos (local.get $byte_len)))
      (else (local.set $byte_e_pos (call $utf8_cp_to_byte_index (local.get $obj) (local.get $e_pos)))))
    (local.set $result_len (i32.sub (local.get $byte_e_pos) (local.get $byte_s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $result_len))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.add (i32.const {header}) (local.get $byte_s_pos)))
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

    pub(crate) fn emit_string_index_of(&self, wat: &mut String) {
        self.emit_utf8_byte_to_cp_index(wat);
        wat.push_str(&format!(
            r#"
  (func $string_index_of (param $haystack i32) (param $needle i32) (param $position i32) (result i32)
    (local $h_obj i32)
    (local $h_len i32)
    (local $n_data i32)
    (local $n_len i32)
    (local $i i32)
    (if (i32.eqz (call $is_string (local.get $haystack))) (then (return (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (local.set $h_obj (i32.and (local.get $haystack) (i32.const {heap_mask})))
    (local.set $h_len (i32.load (local.get $h_obj)))
    ;; Coerce needle to string; use scratch buffer for non-string values
    (if (call $is_string (local.get $needle))
      (then
        (local.set $n_data (i32.add (i32.and (local.get $needle) (i32.const {heap_mask})) (i32.const {header})))
        (local.set $n_len (i32.load (i32.and (local.get $needle) (i32.const {heap_mask})))))
      (else
        (local.set $n_data (i32.const {scratch}))
        (local.set $n_len (call $value_to_string_into (local.get $needle) (local.get $n_data)))))
    ;; Decode position: undefined (0) -> 0, otherwise decode tagged number
    (if (i32.eq (local.get $position) (i32.const {undefined}))
      (then (local.set $i (i32.const {zero})))
      (else (local.set $i (i32.shr_s (local.get $position) (i32.const {shift})))))
    ;; Clamp position to [0, h_len]
    (if (i32.lt_s (local.get $i) (i32.const {zero})) (then (local.set $i (i32.const {zero}))))
    (if (i32.gt_u (local.get $i) (local.get $h_len)) (then (local.set $i (local.get $h_len))))
    ;; Empty needle: return clamped position as code point index

    (if (i32.eqz (local.get $n_len)) (then (return (i32.or (i32.shl (call $utf8_byte_to_cp_index (local.get $haystack) (local.get $i)) (i32.const {number_shift})) (i32.const {number_tag})))))
    ;; Needle longer than haystack: not found
    (if (i32.gt_u (local.get $n_len) (local.get $h_len)) (then (return (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (block $not_found
      (loop $search
        (br_if $not_found (i32.gt_u (local.get $i) (i32.sub (local.get $h_len) (local.get $n_len))))
        (if (call $mem_equal
              (i32.add (i32.add (local.get $h_obj) (i32.const {header})) (local.get $i))
              (local.get $n_data)
              (local.get $n_len))
          (then (return (i32.or (i32.shl (call $utf8_byte_to_cp_index (local.get $haystack) (local.get $i)) (i32.const {number_shift})) (i32.const {number_tag})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $search)))
    (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            neg_one = -1i32,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            shift = ValueTag::NUMBER_SHIFT,
            scratch = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(crate) fn emit_string_last_index_of(&self, wat: &mut String) {
        self.emit_utf8_byte_to_cp_index(wat);
        wat.push_str(&format!(
            r#"
  (func $string_last_index_of (param $haystack i32) (param $needle i32) (param $position i32) (result i32)
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
    ;; Decode position: undefined (0) -> h_len (search entire string)

    (if (i32.eq (local.get $position) (i32.const {undefined}))
      (then (local.set $i (local.get $h_len)))
      (else (local.set $i (i32.shr_s (local.get $position) (i32.const {shift})))))
    ;; Clamp position to [0, h_len]
    (if (i32.lt_s (local.get $i) (i32.const {zero})) (then (local.set $i (i32.const {zero}))))
    (if (i32.gt_u (local.get $i) (local.get $h_len)) (then (local.set $i (local.get $h_len))))
    ;; Empty needle: return clamped position as code point index
    (if (i32.eqz (local.get $n_len)) (then (return (i32.or (i32.shl (call $utf8_byte_to_cp_index (local.get $haystack) (local.get $i)) (i32.const {number_shift})) (i32.const {number_tag})))))
    ;; Needle longer than haystack: not found
    (if (i32.gt_u (local.get $n_len) (local.get $h_len)) (then (return (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    ;; Start from min(position, h_len - n_len)
    (if (i32.gt_u (local.get $i) (i32.sub (local.get $h_len) (local.get $n_len)))
      (then (local.set $i (i32.sub (local.get $h_len) (local.get $n_len)))))
    (block $not_found
      (loop $search
        (br_if $not_found (i32.lt_s (local.get $i) (i32.const {zero})))
        (if (call $mem_equal
              (i32.add (i32.add (local.get $h_obj) (i32.const {header})) (local.get $i))
              (i32.add (local.get $n_obj) (i32.const {header}))
              (local.get $n_len))
          (then (return (i32.or (i32.shl (call $utf8_byte_to_cp_index (local.get $haystack) (local.get $i)) (i32.const {number_shift})) (i32.const {number_tag})))))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $search)))
    (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            neg_one = -1i32,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    pub(crate) fn emit_string_locale_compare(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_locale_compare (param $a i32) (param $b i32) (result i32)
    (local $a_obj i32)
    (local $b_obj i32)
    (local $a_len i32)
    (local $b_len i32)
    (local $min_len i32)
    (local $i i32)
    (local $a_byte i32)
    (local $b_byte i32)
    (if (i32.eqz (call $is_string (local.get $a))) (then (return (i32.const {zero_number}))))
    (if (i32.eqz (call $is_string (local.get $b))) (then (return (i32.const {zero_number}))))
    (local.set $a_obj (i32.and (local.get $a) (i32.const {heap_mask})))
    (local.set $b_obj (i32.and (local.get $b) (i32.const {heap_mask})))
    (local.set $a_len (i32.load (local.get $a_obj)))
    (local.set $b_len (i32.load (local.get $b_obj)))
    (local.set $min_len (local.get $a_len))
    (if (i32.lt_u (local.get $b_len) (local.get $min_len))
      (then (local.set $min_len (local.get $b_len))))
    (block $done
      (loop $compare
        (br_if $done (i32.ge_u (local.get $i) (local.get $min_len)))
        (local.set $a_byte (i32.load8_u (i32.add (i32.add (local.get $a_obj) (i32.const {header})) (local.get $i))))
        (local.set $b_byte (i32.load8_u (i32.add (i32.add (local.get $b_obj) (i32.const {header})) (local.get $i))))
        (if (i32.lt_u (local.get $a_byte) (local.get $b_byte))
          (then (return (i32.const {neg_one}))))
        (if (i32.gt_u (local.get $a_byte) (local.get $b_byte))
          (then (return (i32.const {one}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one_raw})))
        (br $compare)))
    (if (i32.lt_u (local.get $a_len) (local.get $b_len))
      (then (return (i32.const {neg_one}))))
    (if (i32.gt_u (local.get $a_len) (local.get $b_len))
      (then (return (i32.const {one}))))
    (i32.const {zero_number}))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero_number = ValueTag::NUMBER, // tagged 0: (0 << 3) | 4 = 4
            one = (1 << ValueTag::NUMBER_SHIFT) | ValueTag::NUMBER, // tagged 1: (1 << 3) | 4 = 12
            neg_one = ((-1i32) << ValueTag::NUMBER_SHIFT) | ValueTag::NUMBER, // tagged -1
            one_raw = RuntimeConst::ONE, // raw 1 for i32.add counter
        ));
    }

    pub(crate) fn emit_string_includes(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_includes (param $haystack i32) (param $needle i32) (param $position i32) (result i32)
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
    ;; Decode position from tagged value (number: (payload << 3) | 4)
    ;; If position is undefined (0), use 0 as start
    (if (i32.eq (local.get $position) (i32.const {undefined}))
      (then (local.set $i (i32.const {zero})))
      (else (local.set $i (i32.shr_s (local.get $position) (i32.const {shift})))))
    ;; Clamp position to [0, h_len] per spec
    (if (i32.lt_s (local.get $i) (i32.const {zero})) (then (local.set $i (i32.const {zero}))))
    (if (i32.gt_u (local.get $i) (local.get $h_len)) (then (local.set $i (local.get $h_len))))
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
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    pub(crate) fn emit_string_split(&self, wat: &mut String) {
        // Emit helpers needed for RegExp separator support
        self.emit_regexp_match_from(wat);
        self.emit_string_split_regexp(wat);
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
    (local $first_byte i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (if (i32.eqz (call $is_string (local.get $sep)))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.const 4))))
        (i32.store (local.get $result_ptr) (i32.const 1))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (i32.const 1))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const 1))
        (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.const 0))) (local.get $s))
        (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
    ;; Check if separator is a RegExp pattern (starts with '/')
    (local.set $first_byte
      (i32.load8_u
        (i32.add (i32.and (local.get $sep) (i32.const {heap_mask})) (i32.const {str_header}))))
    (if (i32.eq (local.get $first_byte) (i32.const {slash}))
      (then (return (call $string_split_regexp (local.get $s) (local.get $sep)))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $sep_obj (i32.and (local.get $sep) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $sep_len (i32.load (local.get $sep_obj)))
    (if (i32.eqz (local.get $sep_len))
      (then
        ;; Empty separator: split into individual bytes
        (local.set $count (local.get $s_len))
        (local.set $result_ptr
          (call $alloc_heap
            (i32.add (i32.const {array_header})
              (i32.shl (local.get $count) (i32.const {elem_shift})))))
        (i32.store (local.get $result_ptr) (local.get $count))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $count))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
        (i32.store
          (i32.add (local.get $result_ptr) (i32.const 16))
          (i32.sub (i32.shl (i32.const 1) (local.get $count)) (i32.const 1)))
        (local.set $i (i32.const {zero}))
        (block $empty_sep_done
          (loop $empty_sep_loop
            (br_if $empty_sep_done (i32.ge_u (local.get $i) (local.get $s_len)))
            (local.set $part_ptr
              (call $alloc_heap (i32.add (i32.const {str_header}) (i32.const 1))))
            (i32.store (local.get $part_ptr) (i32.const 1))
            (i32.store8
              (i32.add (local.get $part_ptr) (i32.const {str_header}))
              (i32.load8_u
                (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $i))))
            (local.set $part_value (i32.or (local.get $part_ptr) (i32.const {string_tag})))
            (i32.store
              (i32.add (local.get $result_ptr)
                (i32.add (i32.const {array_header})
                  (i32.shl (local.get $i) (i32.const {elem_shift}))))
              (local.get $part_value))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $empty_sep_loop)))
        (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
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
            slash = b'/' as i32,
        ));
    }

    /// Split string by RegExp pattern separator (e.g., "/,/" splits on commas).
    /// Uses $regexp_match_from which stores match_end at SCRATCH_OFFSET.
    pub(crate) fn emit_string_split_regexp(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_split_regexp (param $input i32) (param $pattern i32) (result i32)
    (local $i_obj i32)
    (local $i_len i32)
    (local $count i32)
    (local $search_pos i32)
    (local $match_str i32)
    (local $match_end i32)
    (local $match_len i32)
    (local $result_ptr i32)
    (local $part_start i32)
    (local $result_idx i32)
    (local $part_ptr i32)
    (local $part_len i32)
    (local $part_value i32)
    (if (i32.eqz (call $is_string (local.get $input))) (then (return (i32.const {null_tag}))))
    (if (i32.eqz (call $is_string (local.get $pattern))) (then (return (i32.const {null_tag}))))
    (local.set $i_obj (i32.and (local.get $input) (i32.const {heap_mask})))
    (local.set $i_len (i32.load (local.get $i_obj)))
    ;; First pass: count parts (at minimum 1)
    (local.set $count (i32.const {one}))
    (local.set $search_pos (i32.const {zero}))
    (block $count_done
      (loop $count_loop
        (local.set $match_str (call $regexp_match_from (local.get $pattern) (local.get $input) (local.get $search_pos)))
        (if (i32.eqz (call $is_string (local.get $match_str))) (then (br $count_done)))
        (local.set $match_end (i32.load (i32.const {scratch})))
        (local.set $count (i32.add (local.get $count) (i32.const {one})))
        (local.set $search_pos (local.get $match_end))
        (local.set $match_len (i32.load (i32.and (local.get $match_str) (i32.const {heap_mask}))))
        (if (i32.eqz (local.get $match_len))
          (then (local.set $search_pos (i32.add (local.get $search_pos) (i32.const {one})))))
        (br $count_loop)))
    ;; Allocate result array
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header})
          (i32.shl (local.get $count) (i32.const {elem_shift})))))
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
    (local.set $search_pos (i32.const {zero}))
    (block $extract_done
      (loop $extract_loop
        (br_if $extract_done
          (i32.ge_u (local.get $result_idx) (i32.sub (local.get $count) (i32.const {one}))))
        (local.set $match_str (call $regexp_match_from (local.get $pattern) (local.get $input) (local.get $search_pos)))
        (local.set $match_end (i32.load (i32.const {scratch})))
        (local.set $match_len (i32.load (i32.and (local.get $match_str) (i32.const {heap_mask}))))
        ;; Extract part: input[part_start .. match_end-match_len]
        (local.set $part_len
          (i32.sub (i32.sub (local.get $match_end) (local.get $match_len)) (local.get $part_start)))
        (local.set $part_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $part_len))))
        (i32.store (local.get $part_ptr) (local.get $part_len))
        (call $copy
          (i32.add (i32.add (local.get $i_obj) (i32.const {str_header})) (local.get $part_start))
          (i32.add (local.get $part_ptr) (i32.const {str_header}))
          (local.get $part_len))
        (local.set $part_value (i32.or (local.get $part_ptr) (i32.const {string_tag})))
        (i32.store
          (i32.add (local.get $result_ptr)
            (i32.add (i32.const {array_header})
              (i32.shl (local.get $result_idx) (i32.const {elem_shift}))))
          (local.get $part_value))
        (local.set $result_idx (i32.add (local.get $result_idx) (i32.const {one})))
        (local.set $part_start (local.get $match_end))
        (local.set $search_pos (local.get $match_end))
        (if (i32.eqz (local.get $match_len))
          (then (local.set $search_pos (i32.add (local.get $search_pos) (i32.const {one})))))
        (br $extract_loop)))
    ;; Final part: input[part_start .. end]
    (local.set $part_len (i32.sub (local.get $i_len) (local.get $part_start)))
    (local.set $part_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $part_len))))
    (i32.store (local.get $part_ptr) (local.get $part_len))
    (call $copy
      (i32.add (i32.add (local.get $i_obj) (i32.const {str_header})) (local.get $part_start))
      (i32.add (local.get $part_ptr) (i32.const {str_header}))
      (local.get $part_len))
    (local.set $part_value (i32.or (local.get $part_ptr) (i32.const {string_tag})))
    (i32.store
      (i32.add (local.get $result_ptr)
        (i32.add (i32.const {array_header})
          (i32.shl (local.get $result_idx) (i32.const {elem_shift}))))
      (local.get $part_value))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            null_tag = ValueTag::NULL,
            array_header = Layout::ARRAY_HEADER_SIZE,
            str_header = Layout::STRING_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            scratch = Layout::SCRATCH_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_string_replace(&self, wat: &mut String) {
        // Emit $regexp_match_from helper first (needed for regexp replace)
        self.emit_regexp_match_from(wat);
        // Emit $string_expand_dollar helper
        self.emit_string_expand_dollar(wat);
        // Emit $string_length helper
        self.emit_string_length(wat);
        // Emit $string_replace_regexp helper
        self.emit_string_replace_regexp(wat);
        wat.push_str(&format!(
            r#"
  (func $string_replace (param $s i32) (param $search i32) (param $replace i32) (result i32)
    (local $s_obj i32)
    (local $search_obj i32)
    (local $replace_obj i32)
    (local $s_len i32)
    (local $search_len i32)
    (local $replace_len i32)
    (local $pos i32)
    (local $pre_len i32)
    (local $post_len i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (local $first_byte i32)
    (local $is_regexp i32)
    (local $result i32)
    ;; Guard: $s must be string
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (local.get $s))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    ;; Check if search is a RegExp pattern (starts with '/')
    (if (call $is_string (local.get $search))
      (then
        (local.set $search_obj (i32.and (local.get $search) (i32.const {heap_mask})))
        (local.set $search_len (i32.load (local.get $search_obj)))
        (local.set $first_byte
          (i32.load8_u (i32.add (local.get $search_obj) (i32.const {str_header}))))
        (if (i32.eq (local.get $first_byte) (i32.const {slash}))
          (then (local.set $is_regexp (i32.const 1)))
          (else (local.set $is_regexp (i32.const 0)))))
      (else
        (local.set $is_regexp (i32.const 0))))
    (if (local.get $is_regexp)
      (then
        ;; === RegExp path ===
        (local.set $result (call $string_replace_regexp
          (local.get $s) (local.get $search) (local.get $replace))))
      (else
        ;; === String path ===
        (if (i32.eqz (call $is_string (local.get $search)))
          (then (local.set $result (local.get $s)))
          (else
            (if (i32.eqz (call $is_string (local.get $replace)))
              (then (local.set $result (local.get $s)))
              (else
                (local.set $search_obj (i32.and (local.get $search) (i32.const {heap_mask})))
                (local.set $replace_obj (i32.and (local.get $replace) (i32.const {heap_mask})))
                (local.set $search_len (i32.load (local.get $search_obj)))
                (local.set $replace_len (i32.load (local.get $replace_obj)))
                (if (i32.gt_u (local.get $search_len) (local.get $s_len))
                  (then (local.set $result (local.get $s)))
                  (else
                    ;; Search loop for first occurrence
                    (local.set $pos (i32.const {zero}))
                    (block $not_found
                      (loop $search
                        (br_if $not_found (i32.gt_u (local.get $pos) (i32.sub (local.get $s_len) (local.get $search_len))))
                        (if (call $mem_equal
                              (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $pos))
                              (i32.add (local.get $search_obj) (i32.const {str_header}))
                              (local.get $search_len))
                          (then (br $not_found)))
                        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
                        (br $search)))
                    (if (i32.gt_u (local.get $pos) (i32.sub (local.get $s_len) (local.get $search_len)))
                      (then (local.set $result (local.get $s)))
                      (else
                        (local.set $pre_len (local.get $pos))
                        (local.set $post_len (i32.sub (i32.sub (local.get $s_len) (local.get $pos)) (local.get $search_len)))
                        (local.set $result_len (i32.add (i32.add (local.get $pre_len) (local.get $replace_len)) (local.get $post_len)))
                        (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $result_len))))
                        (i32.store (local.get $result_ptr) (local.get $result_len))
                        (call $copy
                          (i32.add (local.get $s_obj) (i32.const {str_header}))
                          (i32.add (local.get $result_ptr) (i32.const {str_header}))
                          (local.get $pre_len))
                        (call $copy
                          (i32.add (local.get $replace_obj) (i32.const {str_header}))
                          (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (local.get $pre_len))
                          (local.get $replace_len))
                        (call $copy
                          (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (i32.add (local.get $pos) (local.get $search_len)))
                          (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (i32.add (local.get $pre_len) (local.get $replace_len)))
                          (local.get $post_len))
                        (local.set $result (i32.or (local.get $result_ptr) (i32.const {string_tag})))
                      )
                    )
                  )
                )
              )
            )
          )
        )
      )
    )
    (local.get $result))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
            str_header = Layout::STRING_HEADER_SIZE,
            slash = b'/' as i32,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_string_replace_all(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  ;; Replace all occurrences of $search in $s with $replace
  (func $string_replace_all (param $s i32) (param $search i32) (param $replace i32) (result i32)
    (local $prev i32)
    (local $curr i32)
    (local $s_obj i32)
    (local $search_obj i32)
    (local $replace_obj i32)
    (local $s_len i32)
    (local $search_len i32)
    (local $replace_len i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (local $i i32)
    (local $dst i32)
    (local $delimiter i32)
    (local $has_g i32)
    (if (i32.eqz (call $is_string (local.get $s)))
      (then (return (local.get $s))))
    (if (i32.eqz (call $is_string (local.get $search)))
      (then (return (local.get $s))))
    (if (i32.eqz (call $is_string (local.get $replace)))
      (then (return (local.get $s))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $search_obj (i32.and (local.get $search) (i32.const {heap_mask})))
    (local.set $replace_obj (i32.and (local.get $replace) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $search_len (i32.load (local.get $search_obj)))
    (local.set $replace_len (i32.load (local.get $replace_obj)))
    ;; RegExp global-flag check: throw TypeError for non-global RegExp search arg
    (if (i32.gt_u (local.get $search_len) (i32.const {one}))
      (then
        (if (i32.eq
              (i32.load8_u (i32.add (local.get $search_obj) (i32.const {str_header})))
              (i32.const 0x2F))
          (then
            (local.set $delimiter (i32.sub (local.get $search_len) (i32.const {one})))
            (block $find_delim
              (loop $find_loop
                (br_if $find_delim
                  (i32.eq
                    (i32.load8_u
                      (i32.add (i32.add (local.get $search_obj) (i32.const {str_header}))
                        (local.get $delimiter)))
                    (i32.const 0x2F)))
                (local.set $delimiter (i32.sub (local.get $delimiter) (i32.const {one})))
                (br $find_loop)))
            (local.set $has_g (i32.const {zero}))
            (local.set $i (i32.add (local.get $delimiter) (i32.const {one})))
            (block $find_g
              (loop $scan_flags
                (br_if $find_g (i32.ge_u (local.get $i) (local.get $search_len)))
                (if (i32.eq
                      (i32.load8_u
                        (i32.add (i32.add (local.get $search_obj) (i32.const {str_header}))
                          (local.get $i)))
                      (i32.const 0x67))
                  (then (local.set $has_g (i32.const {one}))))
                (local.set $i (i32.add (local.get $i) (i32.const {one})))
                (br $scan_flags)))
            (if (i32.eqz (local.get $has_g))
              (then (return (local.get $s))))))))
    (if (i32.eqz (local.get $search_len))
      (then
        (local.set $result_len
          (i32.add
            (local.get $s_len)
            (i32.mul (local.get $replace_len) (i32.add (local.get $s_len) (i32.const {one})))))
        (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $result_len))))
        (i32.store (local.get $result_ptr) (local.get $result_len))
        (local.set $dst (i32.add (local.get $result_ptr) (i32.const {str_header})))
        (call $copy
          (i32.add (local.get $replace_obj) (i32.const {str_header}))
          (local.get $dst)
          (local.get $replace_len))
        (local.set $dst (i32.add (local.get $dst) (local.get $replace_len)))
        (local.set $i (i32.const {zero}))
        (block $done_empty_search
          (loop $empty_search_loop
            (br_if $done_empty_search (i32.ge_u (local.get $i) (local.get $s_len)))
            (i32.store8
              (local.get $dst)
              (i32.load8_u (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $i))))
            (local.set $dst (i32.add (local.get $dst) (i32.const {one})))
            (call $copy
              (i32.add (local.get $replace_obj) (i32.const {str_header}))
              (local.get $dst)
              (local.get $replace_len))
            (local.set $dst (i32.add (local.get $dst) (local.get $replace_len)))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $empty_search_loop)))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    (local.set $prev (local.get $s))
    (loop $replace_loop
      (local.set $curr (call $string_replace (local.get $prev) (local.get $search) (local.get $replace)))
      (if (i32.eq (local.get $curr) (local.get $prev))
        (then (return (local.get $curr))))
      (local.set $prev (local.get $curr))
      (br $replace_loop))
    (local.get $prev))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
            str_header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    /// Helper: return the byte length of a string value (raw i32, not tagged)
    pub(crate) fn emit_string_length(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"  (func $string_length (param $s i32) (result i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {zero}))))
    (i32.load (i32.and (local.get $s) (i32.const {heap_mask}))))"#,
            heap_mask = ValueTag::HEAP_MASK,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_string_code_point_length(&self, wat: &mut String) {
        if wat.contains("$string_code_point_length") {
            return;
        }
        wat.push_str(&format!(
            r#"
  (func $string_code_point_length (param $s i32) (result i32)
    (local $ptr i32)
    (local $len i32)
    (local $i i32)
    (local $count i32)
    (local $b i32)
    (local.set $ptr (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.const {header})))
    (local.set $len (i32.load (i32.and (local.get $s) (i32.const {heap_mask}))))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $b (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
        (if (i32.ne (i32.and (local.get $b) (i32.const 0xC0)) (i32.const 0x80))
          (then (local.set $count (i32.add (local.get $count) (i32.const {one})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (local.get $count))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_string_expand_dollar(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  ;; Helper: expand dollar patterns in replacement string
  ;; $replace - replacement template string
  ;; $match - matched text
  ;; $pre - text before match
  ;; $post - text after match
  ;; $result - pre-allocated buffer pointer (without header) where output is written
  ;; returns bytes written
  (func $string_expand_dollar (param $replace i32) (param $match i32) (param $pre i32) (param $post i32) (param $result i32) (result i32)
    (local $r_obj i32)
    (local $r_len i32)
    (local $i i32)
    (local $dst i32)
    (local $ch i32)
    (local $next_ch i32)
    (local $expand_obj i32)
    (local $expand_len i32)
    (if (i32.eqz (call $is_string (local.get $replace)))
      (then (return (i32.const {zero}))))
    (local.set $r_obj (i32.and (local.get $replace) (i32.const {heap_mask})))
    (local.set $r_len (i32.load (local.get $r_obj)))
    (local.set $dst (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $r_len)))
        (local.set $ch (i32.load8_u (i32.add (i32.add (local.get $r_obj) (i32.const {str_header})) (local.get $i))))
        (if (i32.eq (local.get $ch) (i32.const 0x24))
          (then
            ;; Found '$', check next char
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (if (i32.ge_u (local.get $i) (local.get $r_len))
              (then
                ;; '$' at end: copy literal '$'
                (i32.store8 (i32.add (local.get $result) (local.get $dst)) (i32.const 0x24))
                (local.set $dst (i32.add (local.get $dst) (i32.const {one})))
                (br $done)))
            (local.set $next_ch (i32.load8_u (i32.add (i32.add (local.get $r_obj) (i32.const {str_header})) (local.get $i))))
            ;; Flat dispatch for dollar patterns using block/br (no nested if-else)
            (block $case_done
              ;; $$ → literal $
              (if (i32.eq (local.get $next_ch) (i32.const 0x24))
                (then
                  (i32.store8 (i32.add (local.get $result) (local.get $dst)) (i32.const 0x24))
                  (local.set $dst (i32.add (local.get $dst) (i32.const {one})))
                  (local.set $i (i32.add (local.get $i) (i32.const {one})))
                  (br $case_done)))
              ;; $& → match text
              (if (i32.eq (local.get $next_ch) (i32.const 0x26))
                (then
                  (if (call $is_string (local.get $match))
                    (then
                      (local.set $expand_obj (i32.and (local.get $match) (i32.const {heap_mask})))
                      (local.set $expand_len (i32.load (local.get $expand_obj)))
                      (call $copy
                        (i32.add (local.get $expand_obj) (i32.const {str_header}))
                        (i32.add (local.get $result) (local.get $dst))
                        (local.get $expand_len))
                      (local.set $dst (i32.add (local.get $dst) (local.get $expand_len)))))
                  (local.set $i (i32.add (local.get $i) (i32.const {one})))
                  (br $case_done)))
              ;; $` → text before match
              (if (i32.eq (local.get $next_ch) (i32.const 0x60))
                (then
                  (if (call $is_string (local.get $pre))
                    (then
                      (local.set $expand_obj (i32.and (local.get $pre) (i32.const {heap_mask})))
                      (local.set $expand_len (i32.load (local.get $expand_obj)))
                      (call $copy
                        (i32.add (local.get $expand_obj) (i32.const {str_header}))
                        (i32.add (local.get $result) (local.get $dst))
                        (local.get $expand_len))
                      (local.set $dst (i32.add (local.get $dst) (local.get $expand_len)))))
                  (local.set $i (i32.add (local.get $i) (i32.const {one})))
                  (br $case_done)))
              ;; $' → text after match
              (if (i32.eq (local.get $next_ch) (i32.const 0x27))
                (then
                  (if (call $is_string (local.get $post))
                    (then
                      (local.set $expand_obj (i32.and (local.get $post) (i32.const {heap_mask})))
                      (local.set $expand_len (i32.load (local.get $expand_obj)))
                      (call $copy
                        (i32.add (local.get $expand_obj) (i32.const {str_header}))
                        (i32.add (local.get $result) (local.get $dst))
                        (local.get $expand_len))
                      (local.set $dst (i32.add (local.get $dst) (local.get $expand_len)))))
                  (local.set $i (i32.add (local.get $i) (i32.const {one})))
                  (br $case_done)))
              ;; Default: copy '$' and the char literally
              (i32.store8 (i32.add (local.get $result) (local.get $dst)) (i32.const 0x24))
              (local.set $dst (i32.add (local.get $dst) (i32.const {one})))
              (i32.store8 (i32.add (local.get $result) (local.get $dst)) (local.get $next_ch))
              (local.set $dst (i32.add (local.get $dst) (i32.const {one})))
              (local.set $i (i32.add (local.get $i) (i32.const {one})))
            )
          )
          (else
            ;; Not '$', copy character literally
            (i32.store8 (i32.add (local.get $result) (local.get $dst)) (local.get $ch))
            (local.set $dst (i32.add (local.get $dst) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one}))))
          )
        (br $scan)))
    (local.get $dst))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_string_replace_regexp(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  ;; Internal: Replace using RegExp pattern
  ;; $pattern - RegExp pattern string (e.g., "/abc/g")
  ;; $input - input string
  ;; $replace - replacement string
  (func $string_replace_regexp (param $input i32) (param $pattern i32) (param $replace i32) (result i32)
    (local $i_obj i32)
    (local $i_len i32)
    (local $p_obj i32)
    (local $p_len i32)
    (local $delimiter i32)
    (local $is_global i32)
    (local $search_pos i32)
    (local $match_str i32)
    (local $match_end i32)
    (local $match_len i32)
    (local $result_ptr i32)
    (local $result_len i32)
    (local $dst i32)
    (local $expanded_len i32)
    (local $pre_str i32)
    (local $post_str i32)
    (local $seg_len i32)
    (local $prev_end i32)
    (local $replace_obj i32)
    (local $replace_len i32)
    (local $expand_needed i32)
    (local $match_start i32)
    (if (i32.eqz (call $is_string (local.get $input)))
      (then (return (local.get $input))))
    (if (i32.eqz (call $is_string (local.get $pattern)))
      (then (return (local.get $input))))
    (if (i32.eqz (call $is_string (local.get $replace)))
      (then (return (local.get $input))))
    (local.set $i_obj (i32.and (local.get $input) (i32.const {heap_mask})))
    (local.set $i_len (i32.load (local.get $i_obj)))
    (local.set $p_obj (i32.and (local.get $pattern) (i32.const {heap_mask})))
    (local.set $p_len (i32.load (local.get $p_obj)))
    (local.set $replace_obj (i32.and (local.get $replace) (i32.const {heap_mask})))
    (local.set $replace_len (i32.load (local.get $replace_obj)))
    (if (i32.lt_u (local.get $p_len) (i32.const 2))
      (then (return (local.get $input))))
    ;; Find closing slash delimiter
    (local.set $delimiter (i32.sub (local.get $p_len) (i32.const {one})))
    (block $find_delim
      (loop $find_loop
        (br_if $find_delim
          (i32.eq
            (i32.load8_u (i32.add (i32.add (local.get $p_obj) (i32.const {str_header})) (local.get $delimiter)))
            (i32.const {slash})))
        (if (i32.eqz (local.get $delimiter))
          (then (return (local.get $input))))
        (local.set $delimiter (i32.sub (local.get $delimiter) (i32.const {one})))
        (br $find_loop)))
    ;; Check for 'g' flag (after the closing slash)
    (local.set $is_global (i32.const {zero}))
    (block $check_flags
      (local.set $p_len (i32.add (local.get $delimiter) (i32.const {one})))
      (loop $flag_loop
        (br_if $check_flags (i32.ge_u (local.get $p_len) (i32.load (i32.and (local.get $pattern) (i32.const {heap_mask})))))
        (if (i32.eq
              (i32.load8_u (i32.add (i32.add (local.get $p_obj) (i32.const {str_header})) (local.get $p_len)))
              (i32.const 0x67))
          (then (local.set $is_global (i32.const 1))))
        (local.set $p_len (i32.add (local.get $p_len) (i32.const {one})))
        (br $flag_loop)))
    ;; Restore p_len
    (local.set $p_len (i32.load (i32.and (local.get $pattern) (i32.const {heap_mask}))))
    ;; Check if replace contains dollar patterns
    (local.set $expand_needed (i32.const {zero}))
    (block $check_dollar_done
      (local.set $p_len (i32.const {zero})) ;; reuse as scan index
      (loop $check_dollar
        (br_if $check_dollar_done (i32.ge_u (local.get $p_len) (local.get $replace_len)))
        (if (i32.eq
              (i32.load8_u (i32.add (local.get $replace_obj) (i32.add (i32.const {str_header}) (local.get $p_len))))
              (i32.const 0x24))
          (then
            (local.set $expand_needed (i32.const 1))
            (br $check_dollar_done)))
        (local.set $p_len (i32.add (local.get $p_len) (i32.const {one})))
        (br $check_dollar)))
    ;; Restore p_len
    (local.set $p_len (i32.load (i32.and (local.get $pattern) (i32.const {heap_mask}))))
    (if (result i32) (local.get $is_global)
      (then
        ;; === Global regexp replace ===
        ;; First pass: count total result length
        (local.set $result_len (i32.const {zero}))
        (local.set $prev_end (i32.const {zero}))
        (local.set $search_pos (i32.const {zero}))
        (block $count_done
          (loop $count_loop
            (local.set $match_str (call $regexp_match_from (local.get $pattern) (local.get $input) (local.get $search_pos)))
            (if (i32.eqz (call $is_string (local.get $match_str)))
              (then (br $count_done)))
            ;; Read match_end from scratch (stored by regexp_match_from)
            (local.set $match_end (i32.load (i32.const {scratch})))
            (local.set $match_len (i32.load (i32.and (local.get $match_str) (i32.const {heap_mask}))))
            (local.set $match_start (i32.sub (local.get $match_end) (local.get $match_len)))
            ;; Add prefix length: prev_end to match_start
            (local.set $result_len (i32.add (local.get $result_len) (i32.sub (local.get $match_start) (local.get $prev_end))))
            ;; Add replacement length
            (if (local.get $expand_needed)
              (then
                ;; Compute exact expanded replacement length
                ;; We need pre and post strings for dollar expansion
                ;; For counting pass, just use replace_len as estimate (close enough)
                (local.set $result_len (i32.add (local.get $result_len) (local.get $replace_len)))
                )
              (else
                (local.set $result_len (i32.add (local.get $result_len) (local.get $replace_len)))))
            (local.set $prev_end (local.get $match_end))
            ;; Advance past match; for zero-length, advance by 1
            (local.set $search_pos (local.get $match_end))
            (if (i32.eq (local.get $match_len) (i32.const {zero}))
              (then (local.set $search_pos (i32.add (local.get $search_pos) (i32.const {one})))))
            (br $count_loop)))
        ;; Add remaining suffix
        (local.set $result_len (i32.add (local.get $result_len) (i32.sub (local.get $i_len) (local.get $prev_end))))
        ;; Allocate result buffer
        (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $result_len))))
        (i32.store (local.get $result_ptr) (local.get $result_len))
        ;; Second pass: build result
        (local.set $dst (i32.const {zero}))
        (local.set $prev_end (i32.const {zero}))
        (local.set $search_pos (i32.const {zero}))
        (block $build_done
          (loop $build_loop
            (local.set $match_str (call $regexp_match_from (local.get $pattern) (local.get $input) (local.get $search_pos)))
            (if (i32.eqz (call $is_string (local.get $match_str)))
              (then (br $build_done)))
            (local.set $match_end (i32.load (i32.const {scratch})))
            (local.set $match_len (i32.load (i32.and (local.get $match_str) (i32.const {heap_mask}))))
            (local.set $match_start (i32.sub (local.get $match_end) (local.get $match_len)))
            ;; Copy prefix: input[prev_end .. match_start) → result[dst ..]
            (local.set $seg_len (i32.sub (local.get $match_start) (local.get $prev_end)))
            (if (i32.gt_u (local.get $seg_len) (i32.const {zero}))
              (then
                (call $copy
                  (i32.add (i32.add (local.get $i_obj) (i32.const {str_header})) (local.get $prev_end))
                  (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (local.get $dst))
                  (local.get $seg_len))
                (local.set $dst (i32.add (local.get $dst) (local.get $seg_len)))))
            ;; Build replacement
            (if (local.get $expand_needed)
              (then
                ;; Create pre and post strings for dollar expansion
                (local.set $pre_str (call $string_substring
                  (local.get $input)
                  (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag}))
                  (i32.or (i32.shl (local.get $match_start) (i32.const {number_shift})) (i32.const {number_tag}))))
                (local.set $post_str (call $string_substring
                  (local.get $input)
                  (i32.or (i32.shl (local.get $match_end) (i32.const {number_shift})) (i32.const {number_tag}))
                  (i32.or (i32.shl (local.get $i_len) (i32.const {number_shift})) (i32.const {number_tag}))))
                ;; Render expanded replacement to scratch buffer
                (local.set $expanded_len
                  (call $string_expand_dollar
                    (local.get $replace) (local.get $match_str) (local.get $pre_str) (local.get $post_str)
                    (i32.const {scratch})))
                ;; Copy expanded replacement from scratch to result
                (call $copy
                  (i32.const {scratch})
                  (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (local.get $dst))
                  (local.get $expanded_len))
                (local.set $dst (i32.add (local.get $dst) (local.get $expanded_len))))
              (else
                ;; Plain string replacement
                (call $copy
                  (i32.add (local.get $replace_obj) (i32.const {str_header}))
                  (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (local.get $dst))
                  (local.get $replace_len))
                (local.set $dst (i32.add (local.get $dst) (local.get $replace_len)))))
            (local.set $prev_end (local.get $match_end))
            (local.set $search_pos (local.get $match_end))
            (if (i32.eq (local.get $match_len) (i32.const {zero}))
              (then (local.set $search_pos (i32.add (local.get $search_pos) (i32.const {one})))))
            (br $build_loop)))
        ;; Copy remaining suffix
        (local.set $seg_len (i32.sub (local.get $i_len) (local.get $prev_end)))
        (if (i32.gt_u (local.get $seg_len) (i32.const {zero}))
          (then
            (call $copy
              (i32.add (i32.add (local.get $i_obj) (i32.const {str_header})) (local.get $prev_end))
              (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (local.get $dst))
              (local.get $seg_len))))
        (i32.or (local.get $result_ptr) (i32.const {string_tag})))
      (else
        ;; === Non-global regexp replace ===
        ;; Find first match from position 0
        (local.set $match_str (call $regexp_match_from (local.get $pattern) (local.get $input) (i32.const {zero})))
        (if (i32.eqz (call $is_string (local.get $match_str)))
          (then (return (local.get $input))))
        (local.set $match_end (i32.load (i32.const {scratch}))) ;; stored by regexp_match_from
        (local.set $match_len (i32.load (i32.and (local.get $match_str) (i32.const {heap_mask}))))
        (local.set $match_start (i32.sub (local.get $match_end) (local.get $match_len)))
        (if (result i32) (local.get $expand_needed)
          (then
            ;; Dollar expansion: render to scratch first to measure
            (local.set $pre_str (call $string_substring
              (local.get $input)
              (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag}))
              (i32.or (i32.shl (local.get $match_start) (i32.const {number_shift})) (i32.const {number_tag}))))
            (local.set $post_str (call $string_substring
              (local.get $input)
              (i32.or (i32.shl (local.get $match_end) (i32.const {number_shift})) (i32.const {number_tag}))
              (i32.or (i32.shl (local.get $i_len) (i32.const {number_shift})) (i32.const {number_tag}))))
            ;; Render expanded replacement to scratch
            (local.set $expanded_len
              (call $string_expand_dollar
                (local.get $replace) (local.get $match_str) (local.get $pre_str) (local.get $post_str)
                (i32.const {scratch})))
            ;; Compute result: prefix + expanded_replacement + suffix
            (local.set $result_len (i32.add
              (local.get $match_start)
              (i32.add (local.get $expanded_len) (i32.sub (local.get $i_len) (local.get $match_end)))))
            (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $result_len))))
            (i32.store (local.get $result_ptr) (local.get $result_len))
            ;; Copy prefix
            (call $copy
              (i32.add (local.get $i_obj) (i32.const {str_header}))
              (i32.add (local.get $result_ptr) (i32.const {str_header}))
              (local.get $match_start))
            ;; Copy expanded replacement from scratch
            (call $copy
              (i32.const {scratch})
              (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (local.get $match_start))
              (local.get $expanded_len))
            ;; Copy suffix
            (call $copy
              (i32.add (i32.add (local.get $i_obj) (i32.const {str_header})) (local.get $match_end))
              (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (i32.add (local.get $match_start) (local.get $expanded_len)))
              (i32.sub (local.get $i_len) (local.get $match_end)))
            (i32.or (local.get $result_ptr) (i32.const {string_tag})))
          (else
            ;; Plain string replacement (no dollar patterns)
            (local.set $result_len (i32.add
              (local.get $match_start)
              (i32.add (local.get $replace_len) (i32.sub (local.get $i_len) (local.get $match_end)))))
            (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $result_len))))
            (i32.store (local.get $result_ptr) (local.get $result_len))
            (call $copy
              (i32.add (local.get $i_obj) (i32.const {str_header}))
              (i32.add (local.get $result_ptr) (i32.const {str_header}))
              (local.get $match_start))
            (call $copy
              (i32.add (local.get $replace_obj) (i32.const {str_header}))
              (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (local.get $match_start))
              (local.get $replace_len))
            (call $copy
              (i32.add (i32.add (local.get $i_obj) (i32.const {str_header})) (local.get $match_end))
              (i32.add (i32.add (local.get $result_ptr) (i32.const {str_header})) (i32.add (local.get $match_start) (local.get $replace_len)))
              (i32.sub (local.get $i_len) (local.get $match_end)))
            (i32.or (local.get $result_ptr) (i32.const {string_tag})))))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
            str_header = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
            slash = b'/' as i32,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(crate) fn emit_string_trim(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_trim (param $s i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $start i32)
    (local $end i32)
    (local $ch i32)
    (local $ch2 i32)
    (local $ch3 i32)
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
        ;; Check ASCII whitespace (0x09-0x0D, 0x20)
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $start (i32.add (local.get $start) (i32.const {one})))
            (br $trim_leading)))
        ;; Check 0xC2 prefix: U+0085 (NEL), U+00A0 (NBSP) — 2-byte UTF-8
        (if (i32.eq (local.get $ch) (i32.const 0xC2))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 2)) (local.get $end))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (if (i32.or
                      (i32.eq (local.get $ch2) (i32.const 0x85))
                      (i32.eq (local.get $ch2) (i32.const 0xA0)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 2)))
                    (br $trim_leading)))))))
        ;; Check 0xE1 prefix: U+1680 (OGHAM SPACE MARK) — 3-byte UTF-8
        (if (i32.eq (local.get $ch) (i32.const 0xE1))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 3)) (local.get $end))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (local.set $ch3
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const 2)))))
                (if (i32.and
                      (i32.eq (local.get $ch2) (i32.const 0x9A))
                      (i32.eq (local.get $ch3) (i32.const 0x80)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))))))
        ;; Check 0xE2 prefix: U+2000-U+200A, U+2028, U+2029, U+202F, U+205F — 3-byte UTF-8
        (if (i32.eq (local.get $ch) (i32.const 0xE2))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 3)) (local.get $end))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (local.set $ch3
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const 2)))))
                (if (i32.and (i32.eq (local.get $ch2) (i32.const 0x80))
                      (i32.or
                        (i32.or
                          (i32.or
                            (i32.and
                              (i32.ge_u (local.get $ch3) (i32.const 0x80))
                              (i32.le_u (local.get $ch3) (i32.const 0x8A)))
                            (i32.eq (local.get $ch3) (i32.const 0xA8)))
                          (i32.eq (local.get $ch3) (i32.const 0xA9)))
                        (i32.eq (local.get $ch3) (i32.const 0xAF))))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))
                (if (i32.and
                      (i32.eq (local.get $ch2) (i32.const 0x81))
                      (i32.eq (local.get $ch3) (i32.const 0x9F)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))))))
        ;; Check 0xE3 prefix: U+3000 (IDEOGRAPHIC SPACE) — 3-byte UTF-8
        (if (i32.eq (local.get $ch) (i32.const 0xE3))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 3)) (local.get $end))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (local.set $ch3
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const 2)))))
                (if (i32.and
                      (i32.eq (local.get $ch2) (i32.const 0x80))
                      (i32.eq (local.get $ch3) (i32.const 0x80)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))))))
        ;; Check 0xEF prefix: U+FEFF (BOM / ZWNBSP) — 3-byte UTF-8
        (if (i32.eq (local.get $ch) (i32.const 0xEF))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 3)) (local.get $end))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (local.set $ch3
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const 2)))))
                (if (i32.and
                      (i32.eq (local.get $ch2) (i32.const 0xBB))
                      (i32.eq (local.get $ch3) (i32.const 0xBF)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))))))
        (br $trim_leading_done)))
    (block $trim_trailing_done
      (loop $trim_trailing
        (br_if $trim_trailing_done (i32.le_u (local.get $end) (local.get $start)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {header}))
              (i32.sub (local.get $end) (i32.const {one})))))
        ;; Check ASCII whitespace (0x09-0x0D, 0x20)
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $end (i32.sub (local.get $end) (i32.const {one})))
            (br $trim_trailing)))
        ;; Check 2-byte trailing: prev byte 0xC2 + current 0x85 or 0xA0
        (if (i32.and
              (i32.ge_u (local.get $end) (i32.const 2))
              (i32.or
                (i32.eq (local.get $ch) (i32.const 0x85))
                (i32.eq (local.get $ch) (i32.const 0xA0))))
          (then
            (local.set $ch2
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {header}))
                  (i32.sub (local.get $end) (i32.const 2)))))
            (if (i32.eq (local.get $ch2) (i32.const 0xC2))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 2)))
                (br $trim_trailing)))))
        ;; Check 3-byte trailing: load bytes at end-3, end-2, end-1
        (if (i32.ge_u (local.get $end) (i32.const 3))
          (then
            (local.set $ch2
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {header}))
                  (i32.sub (local.get $end) (i32.const 2)))))
            (local.set $ch3
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {header}))
                  (i32.sub (local.get $end) (i32.const 3)))))
            ;; 0xE1 0x9A 0x80 (U+1680)
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xE1))
                  (i32.and
                    (i32.eq (local.get $ch2) (i32.const 0x9A))
                    (i32.eq (local.get $ch) (i32.const 0x80))))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                (br $trim_trailing)))
            ;; 0xE2 0x80 [0x80-0x8A, 0xA8, 0xA9, 0xAF] (U+2000-U+200A, U+2028, U+2029, U+202F)
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xE2))
                  (i32.eq (local.get $ch2) (i32.const 0x80)))
              (then
                (if (i32.or
                      (i32.or
                        (i32.or
                          (i32.and
                            (i32.ge_u (local.get $ch) (i32.const 0x80))
                            (i32.le_u (local.get $ch) (i32.const 0x8A)))
                          (i32.eq (local.get $ch) (i32.const 0xA8)))
                        (i32.eq (local.get $ch) (i32.const 0xA9)))
                      (i32.eq (local.get $ch) (i32.const 0xAF)))
                  (then
                    (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                    (br $trim_trailing)))))
            ;; 0xE2 0x81 0x9F (U+205F)
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xE2))
                  (i32.and
                    (i32.eq (local.get $ch2) (i32.const 0x81))
                    (i32.eq (local.get $ch) (i32.const 0x9F))))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                (br $trim_trailing)))
            ;; 0xE3 0x80 0x80 (U+3000)
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xE3))
                  (i32.and
                    (i32.eq (local.get $ch2) (i32.const 0x80))
                    (i32.eq (local.get $ch) (i32.const 0x80))))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                (br $trim_trailing)))
            ;; 0xEF 0xBB 0xBF (U+FEFF)
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xEF))
                  (i32.and
                    (i32.eq (local.get $ch2) (i32.const 0xBB))
                    (i32.eq (local.get $ch) (i32.const 0xBF))))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                (br $trim_trailing)))))
        (br $trim_trailing_done)))
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

    pub(crate) fn emit_string_trim_start(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_trim_start (param $s i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $start i32)
    (local $ch i32)
    (local $ch2 i32)
    (local $ch3 i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (block $trim_leading_done
      (loop $trim_leading
        (br_if $trim_leading_done (i32.ge_u (local.get $start) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {header}))
              (local.get $start))))
        ;; Check ASCII whitespace (0x09-0x0D, 0x20)
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $start (i32.add (local.get $start) (i32.const {one})))
            (br $trim_leading)))
        ;; Check 0xC2 prefix: U+0085 (NEL), U+00A0 (NBSP)
        (if (i32.eq (local.get $ch) (i32.const 0xC2))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 2)) (local.get $len))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (if (i32.or
                      (i32.eq (local.get $ch2) (i32.const 0x85))
                      (i32.eq (local.get $ch2) (i32.const 0xA0)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 2)))
                    (br $trim_leading)))))))
        ;; Check 0xE1 prefix: U+1680
        (if (i32.eq (local.get $ch) (i32.const 0xE1))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 3)) (local.get $len))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (local.set $ch3
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const 2)))))
                (if (i32.and
                      (i32.eq (local.get $ch2) (i32.const 0x9A))
                      (i32.eq (local.get $ch3) (i32.const 0x80)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))))))
        ;; Check 0xE2 prefix: U+2000-U+200A, U+2028, U+2029, U+202F, U+205F
        (if (i32.eq (local.get $ch) (i32.const 0xE2))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 3)) (local.get $len))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (local.set $ch3
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const 2)))))
                (if (i32.and (i32.eq (local.get $ch2) (i32.const 0x80))
                      (i32.or
                        (i32.or
                          (i32.or
                            (i32.and
                              (i32.ge_u (local.get $ch3) (i32.const 0x80))
                              (i32.le_u (local.get $ch3) (i32.const 0x8A)))
                            (i32.eq (local.get $ch3) (i32.const 0xA8)))
                          (i32.eq (local.get $ch3) (i32.const 0xA9)))
                        (i32.eq (local.get $ch3) (i32.const 0xAF))))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))
                (if (i32.and
                      (i32.eq (local.get $ch2) (i32.const 0x81))
                      (i32.eq (local.get $ch3) (i32.const 0x9F)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))))))
        ;; Check 0xE3 prefix: U+3000
        (if (i32.eq (local.get $ch) (i32.const 0xE3))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 3)) (local.get $len))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (local.set $ch3
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const 2)))))
                (if (i32.and
                      (i32.eq (local.get $ch2) (i32.const 0x80))
                      (i32.eq (local.get $ch3) (i32.const 0x80)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))))))
        ;; Check 0xEF prefix: U+FEFF
        (if (i32.eq (local.get $ch) (i32.const 0xEF))
          (then
            (if (i32.lt_u (i32.add (local.get $start) (i32.const 3)) (local.get $len))
              (then
                (local.set $ch2
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const {one})))))
                (local.set $ch3
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {header}))
                      (i32.add (local.get $start) (i32.const 2)))))
                (if (i32.and
                      (i32.eq (local.get $ch2) (i32.const 0xBB))
                      (i32.eq (local.get $ch3) (i32.const 0xBF)))
                  (then
                    (local.set $start (i32.add (local.get $start) (i32.const 3)))
                    (br $trim_leading)))))))
        (br $trim_leading_done)))
    (if (i32.eqz (local.get $start)) (then (return (local.get $s))))
    (local.set $result_len (i32.sub (local.get $len) (local.get $start)))
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

    pub(crate) fn emit_string_trim_end(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_trim_end (param $s i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $end i32)
    (local $ch i32)
    (local $ch2 i32)
    (local $ch3 i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $end (local.get $len))
    (block $trim_trailing_done
      (loop $trim_trailing
        (br_if $trim_trailing_done (i32.le_u (local.get $end) (i32.const {zero})))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {header}))
              (i32.sub (local.get $end) (i32.const {one})))))
        ;; Check ASCII whitespace (0x09-0x0D, 0x20)
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $end (i32.sub (local.get $end) (i32.const {one})))
            (br $trim_trailing)))
        ;; Check 2-byte trailing: prev byte 0xC2 + current 0x85 or 0xA0
        (if (i32.and
              (i32.ge_u (local.get $end) (i32.const 2))
              (i32.or
                (i32.eq (local.get $ch) (i32.const 0x85))
                (i32.eq (local.get $ch) (i32.const 0xA0))))
          (then
            (local.set $ch2
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {header}))
                  (i32.sub (local.get $end) (i32.const 2)))))
            (if (i32.eq (local.get $ch2) (i32.const 0xC2))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 2)))
                (br $trim_trailing)))))
        ;; Check 3-byte trailing: load bytes at end-3, end-2, end-1
        (if (i32.ge_u (local.get $end) (i32.const 3))
          (then
            (local.set $ch2
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {header}))
                  (i32.sub (local.get $end) (i32.const 2)))))
            (local.set $ch3
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {header}))
                  (i32.sub (local.get $end) (i32.const 3)))))
            ;; 0xE1 0x9A 0x80 (U+1680)
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xE1))
                  (i32.and
                    (i32.eq (local.get $ch2) (i32.const 0x9A))
                    (i32.eq (local.get $ch) (i32.const 0x80))))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                (br $trim_trailing)))
            ;; 0xE2 0x80 [0x80-0x8A, 0xA8, 0xA9, 0xAF]
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xE2))
                  (i32.eq (local.get $ch2) (i32.const 0x80)))
              (then
                (if (i32.or
                      (i32.or
                        (i32.or
                          (i32.and
                            (i32.ge_u (local.get $ch) (i32.const 0x80))
                            (i32.le_u (local.get $ch) (i32.const 0x8A)))
                          (i32.eq (local.get $ch) (i32.const 0xA8)))
                        (i32.eq (local.get $ch) (i32.const 0xA9)))
                      (i32.eq (local.get $ch) (i32.const 0xAF)))
                  (then
                    (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                    (br $trim_trailing)))))
            ;; 0xE2 0x81 0x9F (U+205F)
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xE2))
                  (i32.and
                    (i32.eq (local.get $ch2) (i32.const 0x81))
                    (i32.eq (local.get $ch) (i32.const 0x9F))))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                (br $trim_trailing)))
            ;; 0xE3 0x80 0x80 (U+3000)
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xE3))
                  (i32.and
                    (i32.eq (local.get $ch2) (i32.const 0x80))
                    (i32.eq (local.get $ch) (i32.const 0x80))))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                (br $trim_trailing)))
            ;; 0xEF 0xBB 0xBF (U+FEFF)
            (if (i32.and
                  (i32.eq (local.get $ch3) (i32.const 0xEF))
                  (i32.and
                    (i32.eq (local.get $ch2) (i32.const 0xBB))
                    (i32.eq (local.get $ch) (i32.const 0xBF))))
              (then
                (local.set $end (i32.sub (local.get $end) (i32.const 3)))
                (br $trim_trailing)))))
        (br $trim_trailing_done)))
    (if (i32.eq (local.get $end) (local.get $len)) (then (return (local.get $s))))
    (local.set $result_len (local.get $end))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $result_len))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add (local.get $obj) (i32.const {header}))
      (i32.add (local.get $result_ptr) (i32.const {header}))
      (local.get $result_len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            ascii_tab = 9,
            ascii_cr = 13,
            ascii_space = 32,
        ));
    }

    pub(crate) fn emit_string_starts_with(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_starts_with (param $s i32) (param $search i32) (param $position i32) (result i32)
    (local $s_obj i32)
    (local $search_obj i32)
    (local $s_len i32)
    (local $search_len i32)
    (local $start i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {false_tag}))))
    (if (i32.eqz (call $is_string (local.get $search))) (then (return (i32.const {false_tag}))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $search_obj (i32.and (local.get $search) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $search_len (i32.load (local.get $search_obj)))
    ;; Decode position from tagged value
    (if (i32.eq (local.get $position) (i32.const {undefined}))
      (then (local.set $start (i32.const {zero})))
      (else (local.set $start (i32.shr_s (local.get $position) (i32.const {shift})))))
    ;; Clamp position to [0, len]
    (if (i32.lt_s (local.get $start) (i32.const {zero})) (then (local.set $start (i32.const {zero}))))
    (if (i32.gt_u (local.get $start) (local.get $s_len)) (then (local.set $start (local.get $s_len))))
    ;; Search longer than remaining → false
    (if (i32.gt_u (local.get $search_len) (i32.sub (local.get $s_len) (local.get $start)))
      (then (return (i32.const {false_tag}))))
    ;; Empty search string → true
    (if (i32.eqz (local.get $search_len)) (then (return (i32.const {true_tag}))))
    ;; Check prefix: return tagged bool
    (if (result i32)
      (call $mem_equal
        (i32.add (i32.add (local.get $s_obj) (i32.const {header})) (local.get $start))
        (i32.add (local.get $search_obj) (i32.const {header}))
        (local.get $search_len))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
            shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    pub(crate) fn emit_string_ends_with(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_ends_with (param $s i32) (param $search i32) (param $end_position i32) (result i32)
    (local $s_obj i32)
    (local $search_obj i32)
    (local $s_len i32)
    (local $search_len i32)
    (local $end i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {false_tag}))))
    (if (i32.eqz (call $is_string (local.get $search))) (then (return (i32.const {false_tag}))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $search_obj (i32.and (local.get $search) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $search_len (i32.load (local.get $search_obj)))
    ;; Decode end_position from tagged value; default to len
    (if (i32.eq (local.get $end_position) (i32.const {undefined}))
      (then (local.set $end (local.get $s_len)))
      (else (local.set $end (i32.shr_s (local.get $end_position) (i32.const {shift})))))
    ;; Clamp end to [0, len]
    (if (i32.lt_s (local.get $end) (i32.const {zero})) (then (local.set $end (i32.const {zero}))))
    (if (i32.gt_u (local.get $end) (local.get $s_len)) (then (local.set $end (local.get $s_len))))
    ;; Search longer than available → false
    (if (i32.gt_u (local.get $search_len) (local.get $end))
      (then (return (i32.const {false_tag}))))
    ;; Empty search string → true
    (if (i32.eqz (local.get $search_len)) (then (return (i32.const {true_tag}))))
    ;; Check suffix at position (end - search_len)
    (if (result i32)
      (call $mem_equal
        (i32.add (i32.add (local.get $s_obj) (i32.const {header})) (i32.sub (local.get $end) (local.get $search_len)))
        (i32.add (local.get $search_obj) (i32.const {header}))
        (local.get $search_len))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
            shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    pub(crate) fn emit_string_to_upper_case(&self, wat: &mut String) {
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

    pub(crate) fn emit_string_to_lower_case(&self, wat: &mut String) {
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

    pub(crate) fn emit_string_char_code_at(&self, wat: &mut String) {
        // UTF-8 helpers are emitted via emit_utf8_helpers() before any string functions.
        self.emit_utf8_decode_cp_at_byte(wat);
        self.emit_string_code_point_length(wat);
        wat.push_str(&format!(
            r#"
  (func $string_char_code_at (param $s i32) (param $index i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $idx i32)
    (local $byte_pos i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (call $utf8_cp_count (local.get $obj)))
    (local.set $idx (i32.shr_s (local.get $index) (i32.const {number_shift})))
    ;; Handle negative index using CP length
    (if (i32.lt_s (local.get $idx) (i32.const {zero}))
      (then (local.set $idx (i32.add (local.get $len) (local.get $idx)))))
    ;; Clamp to [0, len)
    (if (i32.lt_s (local.get $idx) (i32.const {zero})) (then (local.set $idx (i32.const {zero}))))
    (if (i32.ge_u (local.get $idx) (local.get $len)) (then (return (i32.const {undefined}))))
    ;; Convert code point index to byte index and decode
    (local.set $byte_pos (call $utf8_cp_to_byte_index (local.get $obj) (local.get $idx)))
    (return (call $utf8_decode_cp_at_byte (local.get $obj) (local.get $byte_pos))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_string_code_point_at(&self, wat: &mut String) {
        // In UTF-8 encoding, codePointAt is equivalent to charCodeAt
        // since we decode full code points (not UTF-16 surrogates).
        self.emit_utf8_decode_cp_at_byte(wat);
        self.emit_string_code_point_length(wat);
        wat.push_str(&format!(
            r#"
  (func $string_code_point_at (param $s i32) (param $index i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $idx i32)
    (local $byte_pos i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (call $utf8_cp_count (local.get $obj)))
    (local.set $idx (i32.shr_s (local.get $index) (i32.const {number_shift})))
    ;; Handle negative index using CP length
    (if (i32.lt_s (local.get $idx) (i32.const {zero}))
      (then (local.set $idx (i32.add (local.get $len) (local.get $idx)))))
    ;; Clamp to [0, len)
    (if (i32.lt_s (local.get $idx) (i32.const {zero})) (then (local.set $idx (i32.const {zero}))))
    (if (i32.ge_u (local.get $idx) (local.get $len)) (then (return (i32.const {undefined}))))
    ;; Convert code point index to byte index and decode
    (local.set $byte_pos (call $utf8_cp_to_byte_index (local.get $obj) (local.get $idx)))
    (return (call $utf8_decode_cp_at_byte (local.get $obj) (local.get $byte_pos))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_string_from_char_code(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_from_char_code (param $code i32) (result i32)
    (local $cp i32)
    (local $byte_len i32)
    (local $ptr i32)
    (local $addr i32)
    (local.set $cp (i32.shr_s (local.get $code) (i32.const {number_shift})))
    ;; Clamp to valid Unicode BMP range (0-65535)
    (if (i32.lt_s (local.get $cp) (i32.const {zero})) (then (local.set $cp (i32.const {zero}))))
    (if (i32.gt_u (local.get $cp) (i32.const 65535)) (then (local.set $cp (i32.const 65535))))
    ;; Determine UTF-8 byte length
    (if (i32.lt_u (local.get $cp) (i32.const 128)) (then (local.set $byte_len (i32.const 1))))
    (if (i32.ge_u (local.get $cp) (i32.const 128)) (then (local.set $byte_len (i32.const 2))))
    (if (i32.ge_u (local.get $cp) (i32.const 2048)) (then (local.set $byte_len (i32.const 3))))
    ;; Allocate: HEADER + byte_len
    (local.set $ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $byte_len))))
    (local.set $addr (i32.add (local.get $ptr) (i32.const {header})))
    ;; Store byte length for the UTF-8 payload.
    (i32.store (local.get $ptr) (local.get $byte_len))
    ;; 1-byte UTF-8: 0xxxxxxx
    (if (i32.eq (local.get $byte_len) (i32.const 1))
      (then (i32.store8 (local.get $addr) (local.get $cp))))
    ;; 2-byte UTF-8: 110xxxxx 10xxxxxx
    (if (i32.eq (local.get $byte_len) (i32.const 2))
      (then
        (i32.store8 (local.get $addr) (i32.or (i32.const 192) (i32.shr_u (local.get $cp) (i32.const 6))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.or (i32.const 128) (i32.and (local.get $cp) (i32.const 63))))))
    ;; 3-byte UTF-8: 1110xxxx 10xxxxxx 10xxxxxx
    (if (i32.eq (local.get $byte_len) (i32.const 3))
      (then
        (i32.store8 (local.get $addr) (i32.or (i32.const 224) (i32.shr_u (local.get $cp) (i32.const 12))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.or (i32.const 128) (i32.and (i32.shr_u (local.get $cp) (i32.const 6)) (i32.const 63))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.or (i32.const 128) (i32.and (local.get $cp) (i32.const 63))))))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_string_from_code_point(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_from_code_point (param $code i32) (result i32)
    (local $cp i32)
    (local $byte_len i32)
    (local $ptr i32)
    (local $addr i32)
    (local.set $cp (i32.shr_s (local.get $code) (i32.const {number_shift})))
    ;; Clamp to valid Unicode range (0-1114111)
    (if (i32.lt_s (local.get $cp) (i32.const {zero})) (then (local.set $cp (i32.const {zero}))))
    (if (i32.gt_u (local.get $cp) (i32.const 1114111)) (then (local.set $cp (i32.const 1114111))))
    ;; Determine UTF-8 byte length
    (if (i32.lt_u (local.get $cp) (i32.const 128)) (then (local.set $byte_len (i32.const 1))))
    (if (i32.ge_u (local.get $cp) (i32.const 128)) (then (local.set $byte_len (i32.const 2))))
    (if (i32.ge_u (local.get $cp) (i32.const 2048)) (then (local.set $byte_len (i32.const 3))))
    (if (i32.ge_u (local.get $cp) (i32.const 65536)) (then (local.set $byte_len (i32.const 4))))
    ;; Allocate: HEADER + byte_len
    (local.set $ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $byte_len))))
    (local.set $addr (i32.add (local.get $ptr) (i32.const {header})))
    ;; Store byte length for the UTF-8 payload.
    (i32.store (local.get $ptr) (local.get $byte_len))
    ;; 1-byte UTF-8: 0xxxxxxx
    (if (i32.eq (local.get $byte_len) (i32.const 1))
      (then (i32.store8 (local.get $addr) (local.get $cp))))
    ;; 2-byte UTF-8: 110xxxxx 10xxxxxx
    (if (i32.eq (local.get $byte_len) (i32.const 2))
      (then
        (i32.store8 (local.get $addr) (i32.or (i32.const 192) (i32.shr_u (local.get $cp) (i32.const 6))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.or (i32.const 128) (i32.and (local.get $cp) (i32.const 63))))))
    ;; 3-byte UTF-8: 1110xxxx 10xxxxxx 10xxxxxx
    (if (i32.eq (local.get $byte_len) (i32.const 3))
      (then
        (i32.store8 (local.get $addr) (i32.or (i32.const 224) (i32.shr_u (local.get $cp) (i32.const 12))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.or (i32.const 128) (i32.and (i32.shr_u (local.get $cp) (i32.const 6)) (i32.const 63))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.or (i32.const 128) (i32.and (local.get $cp) (i32.const 63))))))
    ;; 4-byte UTF-8: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
    (if (i32.eq (local.get $byte_len) (i32.const 4))
      (then
        (i32.store8 (local.get $addr) (i32.or (i32.const 240) (i32.shr_u (local.get $cp) (i32.const 18))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.or (i32.const 128) (i32.and (i32.shr_u (local.get $cp) (i32.const 12)) (i32.const 63))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.or (i32.const 128) (i32.and (i32.shr_u (local.get $cp) (i32.const 6)) (i32.const 63))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 3)) (i32.or (i32.const 128) (i32.and (local.get $cp) (i32.const 63))))))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_string_pad_start(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_pad_start (param $s i32) (param $target_len i32) (param $fill i32) (result i32)
    (local $s_obj i32)
    (local $f_obj i32)
    (local $s_len i32)
    (local $f_len i32)
    (local $max_len i32)
    (local $fill_needed i32)
    (local $result_ptr i32)
    (local $i i32)
    (local $j i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $max_len (i32.shr_s (local.get $target_len) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $max_len) (i32.const {zero})) (then (local.set $max_len (i32.const {zero}))))
    (if (i32.le_u (local.get $max_len) (local.get $s_len)) (then (return (local.get $s))))
    ;; Default fill string to space if not a string
    (if (i32.eqz (call $is_string (local.get $fill)))
      (then
        (local.set $fill_needed (i32.sub (local.get $max_len) (local.get $s_len)))
        (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $max_len))))
        (i32.store (local.get $result_ptr) (local.get $max_len))
        (block $pad_spaces_done
          (loop $pad_spaces
            (br_if $pad_spaces_done (i32.ge_u (local.get $i) (local.get $fill_needed)))
            (i32.store8 (i32.add (i32.add (local.get $result_ptr) (i32.const {header})) (local.get $i)) (i32.const {ascii_space}))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $pad_spaces)))
        (call $copy
          (i32.add (i32.add (local.get $s_obj) (i32.const {header})) (i32.const {zero}))
          (i32.add (i32.add (local.get $result_ptr) (i32.const {header})) (local.get $fill_needed))
          (local.get $s_len))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    (local.set $f_obj (i32.and (local.get $fill) (i32.const {heap_mask})))
    (local.set $f_len (i32.load (local.get $f_obj)))
    (local.set $fill_needed (i32.sub (local.get $max_len) (local.get $s_len)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $max_len))))
    (i32.store (local.get $result_ptr) (local.get $max_len))
    ;; Fill with repeated fill string
    (block $pad_done
      (loop $pad_loop
        (br_if $pad_done (i32.ge_u (local.get $i) (local.get $fill_needed)))
        (i32.store8
          (i32.add (i32.add (local.get $result_ptr) (i32.const {header})) (local.get $i))
          (i32.load8_u (i32.add (i32.add (local.get $f_obj) (i32.const {header})) (local.get $j))))
        (local.set $j (i32.add (local.get $j) (i32.const {one})))
        (if (i32.ge_u (local.get $j) (local.get $f_len)) (then (local.set $j (i32.const {zero}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $pad_loop)))
    ;; Copy original string after padding
    (call $copy
      (i32.add (i32.add (local.get $s_obj) (i32.const {header})) (i32.const {zero}))
      (i32.add (i32.add (local.get $result_ptr) (i32.const {header})) (local.get $fill_needed))
      (local.get $s_len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            ascii_space = 32,
        ));
    }

    pub(crate) fn emit_string_pad_end(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_pad_end (param $s i32) (param $target_len i32) (param $fill i32) (result i32)
    (local $s_obj i32)
    (local $f_obj i32)
    (local $s_len i32)
    (local $f_len i32)
    (local $max_len i32)
    (local $fill_needed i32)
    (local $result_ptr i32)
    (local $i i32)
    (local $j i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $max_len (i32.shr_s (local.get $target_len) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $max_len) (i32.const {zero})) (then (local.set $max_len (i32.const {zero}))))
    (if (i32.le_u (local.get $max_len) (local.get $s_len)) (then (return (local.get $s))))
    ;; Default fill string to space if not a string
    (if (i32.eqz (call $is_string (local.get $fill)))
      (then
        (local.set $fill_needed (i32.sub (local.get $max_len) (local.get $s_len)))
        (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $max_len))))
        (i32.store (local.get $result_ptr) (local.get $max_len))
        ;; Copy original string first
        (call $copy
          (i32.add (i32.add (local.get $s_obj) (i32.const {header})) (i32.const {zero}))
          (i32.add (local.get $result_ptr) (i32.const {header}))
          (local.get $s_len))
        ;; Fill remaining with spaces
        (block $pad_spaces_done
          (loop $pad_spaces
            (br_if $pad_spaces_done (i32.ge_u (local.get $i) (local.get $fill_needed)))
            (i32.store8 (i32.add (i32.add (local.get $result_ptr) (i32.const {header})) (i32.add (local.get $s_len) (local.get $i))) (i32.const {ascii_space}))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $pad_spaces)))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    (local.set $f_obj (i32.and (local.get $fill) (i32.const {heap_mask})))
    (local.set $f_len (i32.load (local.get $f_obj)))
    (local.set $fill_needed (i32.sub (local.get $max_len) (local.get $s_len)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $max_len))))
    (i32.store (local.get $result_ptr) (local.get $max_len))
    ;; Copy original string first
    (call $copy
      (i32.add (i32.add (local.get $s_obj) (i32.const {header})) (i32.const {zero}))
      (i32.add (local.get $result_ptr) (i32.const {header}))
      (local.get $s_len))
    ;; Fill remaining with repeated fill string
    (block $pad_done
      (loop $pad_loop
        (br_if $pad_done (i32.ge_u (local.get $i) (local.get $fill_needed)))
        (i32.store8
          (i32.add (i32.add (local.get $result_ptr) (i32.const {header})) (i32.add (local.get $s_len) (local.get $i)))
          (i32.load8_u (i32.add (i32.add (local.get $f_obj) (i32.const {header})) (local.get $j))))
        (local.set $j (i32.add (local.get $j) (i32.const {one})))
        (if (i32.ge_u (local.get $j) (local.get $f_len)) (then (local.set $j (i32.const {zero}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $pad_loop)))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            ascii_space = 32,
        ));
    }

    pub(crate) fn emit_string_repeat(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_repeat (param $s i32) (param $count i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $cnt i32)
    (local $total_len i32)
    (local $result_ptr i32)
    (local $i i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $cnt (i32.shr_s (local.get $count) (i32.const {number_shift})))
    (if (i32.le_s (local.get $cnt) (i32.const {zero}))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    ;; Guard against overflow: if len > 0 && cnt > i32::MAX / len, return empty string
    (if (i32.and
          (i32.gt_u (local.get $len) (i32.const {zero}))
          (i32.gt_u (local.get $cnt) (i32.div_u (i32.const {i32_max}) (local.get $len))))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    (local.set $total_len (i32.mul (local.get $len) (local.get $cnt)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $total_len))))
    (i32.store (local.get $result_ptr) (local.get $total_len))
    (block $repeat_done
      (loop $repeat_loop
        (br_if $repeat_done (i32.ge_u (local.get $i) (local.get $cnt)))
        (call $copy
          (i32.add (local.get $obj) (i32.const {header}))
          (i32.add (i32.add (local.get $result_ptr) (i32.const {header})) (i32.mul (local.get $i) (local.get $len)))
          (local.get $len))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $repeat_loop)))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            i32_max = i32::MAX,
        ));
    }

    pub(crate) fn emit_string_match(&self, wat: &mut String) {
        wat.push_str(
            r#"  (func $string_match (param $str i32) (param $pattern i32) (result i32)
    (return (call $regexp_match (local.get $pattern) (local.get $str))))"#,
        );
    }

    pub(crate) fn emit_string_search(&self, wat: &mut String) {
        wat.push_str(
            r#"  (func $string_search (param $str i32) (param $pattern i32) (result i32)
    (return (call $regexp_search (local.get $pattern) (local.get $str))))"#,
        );
    }

    pub(crate) fn emit_string_is_well_formed(&self, wat: &mut String) {
        // Strings are byte-level (no UTF-16 surrogates), so all strings are well-formed.
        // String.isWellFormed returns boolean per spec.
        wat.push_str(&format!(
            r#"  (func $string_is_well_formed (param $s i32) (result i32)
    (i32.const {true_tag}))"#,
            true_tag = ValueTag::TRUE,
        ));
    }

    pub(crate) fn emit_string_to_well_formed(&self, wat: &mut String) {
        // Strings are byte-level (no UTF-16 surrogates), so no replacement needed.
        wat.push_str(
            r#"  (func $string_to_well_formed (param $s i32) (result i32)
    (local.get $s))"#,
        );
    }

    pub(crate) fn emit_string_to_locale_string(&self, wat: &mut String) {
        // String.prototype.toLocaleString returns the string value itself.
        wat.push_str(&format!(
            r#"  (func $string_to_locale_string (param $s i32) (result i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.get $s))"#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_string_raw(&self, wat: &mut String) {
        let empty = self.string_value("");
        let raw_key_ptr = self.string_offset("raw") + Layout::STRING_HEADER_SIZE;
        let raw_key_len = self.string_len("raw");
        wat.push_str(&format!(
            r#"  (func $string_raw (param $template i32) (param $sub0 i32) (param $sub1 i32) (result i32)
    (local $raw i32)
    (local $seg0 i32)
    (local $seg1 i32)
    (local $joined i32)
    (local.set $raw
      (call $property_get
        (local.get $template)
        (i32.const {raw_key_ptr})
        (i32.const {raw_key_len})))
    (local.set $seg0
      (call $array_get
        (local.get $raw)
        (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag}))))
    (if (i32.eqz (call $is_string (local.get $seg0)))
      (then (return (i32.const {empty}))))
    (local.set $seg1
      (call $array_get
        (local.get $raw)
        (i32.or (i32.shl (i32.const {one}) (i32.const {number_shift})) (i32.const {number_tag}))))
    (if (i32.eqz (call $is_string (local.get $seg1)))
      (then (return (local.get $seg0))))
    (local.set $joined (call $concat (local.get $seg0) (local.get $sub0)))
    (call $concat (local.get $joined) (local.get $seg1)))"#,
            empty = empty,
            raw_key_ptr = raw_key_ptr,
            raw_key_len = raw_key_len,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }
}
