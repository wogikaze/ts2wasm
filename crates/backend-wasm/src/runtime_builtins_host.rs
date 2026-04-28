use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_math_floor(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_floor (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    ;; floor is no-op for encoded integers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_math_ceil(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_ceil (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    ;; ceil is no-op for encoded integers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_math_round(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_round (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    ;; round is no-op for encoded integers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_math_abs(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_abs (param $v i32) (result i32)
    (local $tag i32)
    (local $n i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or (i32.shl (local.get $n) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_math_max(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_max (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.or (i32.ne (local.get $a_tag) (i32.const {number_tag})) (i32.ne (local.get $b_tag) (i32.const {number_tag})))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (i32.shr_s (local.get $a) (i32.const {number_shift})))
    (local.set $b_n (i32.shr_s (local.get $b) (i32.const {number_shift})))
    (if (i32.gt_s (local.get $a_n) (local.get $b_n))
      (then (return (local.get $a))))
    (local.get $b))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_math_min(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_min (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.or (i32.ne (local.get $a_tag) (i32.const {number_tag})) (i32.ne (local.get $b_tag) (i32.const {number_tag})))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (i32.shr_s (local.get $a) (i32.const {number_shift})))
    (local.set $b_n (i32.shr_s (local.get $b) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $a_n) (local.get $b_n))
      (then (return (local.get $a))))
    (local.get $b))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_math_random(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_random (result i32)
    (local $errno i32)
    (local $raw i32)
    (local.set $errno (call $random_get (i32.const {scratch}) (i32.const 4)))
    (if (i32.ne (local.get $errno) (i32.const 0))
      (then unreachable))
    (local.set $raw (i32.load (i32.const {scratch})))
    ;; The current number representation is tagged i32; this is a random
    ;; integer payload until the broader JS double model is available.
    (i32.or
      (i32.shl
        (i32.rem_u (local.get $raw) (i32.const {modulus}))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            scratch = Layout::SCRATCH_OFFSET,
            modulus = 1000,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    // JSON functions (M10)

    pub(super) fn emit_json_stringify(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_stringify (param $v i32) (param $replacer i32) (param $space i32) (result i32)
    (local $result_ptr i32)
    (local $len i32)
    (local $gap i32)
    (local $gap_ptr i32)
    (local $space_base i32)
    (if
      (i32.and
        (i32.ne (local.get $replacer) (i32.const {undefined}))
        (i32.ne (local.get $replacer) (i32.const {null_tag})))
      (then (return (i32.const {undefined}))))
    (if (i32.eq (i32.and (local.get $space) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then
        (local.set $gap (i32.shr_s (local.get $space) (i32.const {number_shift})))
        (if (i32.lt_s (local.get $gap) (i32.const {zero}))
          (then (local.set $gap (i32.const {zero}))))
        (if (i32.gt_s (local.get $gap) (i32.const {max_gap}))
          (then (local.set $gap (i32.const {max_gap}))))))
    (if (i32.eq (i32.and (local.get $space) (i32.const {tag_mask})) (i32.const {string_tag}))
      (then
        (local.set $space_base (i32.and (local.get $space) (i32.const {heap_mask})))
        (local.set $gap (i32.load (local.get $space_base)))
        (if (i32.gt_u (local.get $gap) (i32.const {max_gap}))
          (then (local.set $gap (i32.const {max_gap}))))
        (local.set $gap_ptr (i32.add (local.get $space_base) (i32.const {header})))))
    (local.set $result_ptr (call $alloc_heap (i32.const {stringify_alloc_size})))
    (local.set $len
      (call $json_stringify_into
        (local.get $v)
        (i32.add (local.get $result_ptr) (i32.const {header}))
        (local.get $gap)
        (local.get $gap_ptr)
        (i32.const {zero})))
    (if (i32.lt_s (local.get $len) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))

  (func $json_write_spaces (param $ptr i32) (param $count i32) (result i32)
    (local $i i32)
    (block $spaces_done
      (loop $spaces_loop
        (br_if $spaces_done (i32.ge_u (local.get $i) (local.get $count)))
        (i32.store8
          (i32.add (local.get $ptr) (local.get $i))
          (i32.const {space_char}))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $spaces_loop)))
    (local.get $count))

  (func $json_write_gap_once (param $ptr i32) (param $gap i32) (param $gap_ptr i32) (result i32)
    (if (i32.eqz (local.get $gap_ptr))
      (then (return (call $json_write_spaces (local.get $ptr) (local.get $gap)))))
    (call $copy (local.get $gap_ptr) (local.get $ptr) (local.get $gap))
    (local.get $gap))

  (func $json_write_newline_indent (param $ptr i32) (param $gap i32) (param $gap_ptr i32) (param $depth i32) (result i32)
    (local $len i32)
    (local $i i32)
    (if (i32.eqz (local.get $gap))
      (then (return (i32.const {zero}))))
    (i32.store8 (local.get $ptr) (i32.const {newline}))
    (local.set $len (i32.const {one}))
    (block $indent_done
      (loop $indent_loop
        (br_if $indent_done (i32.ge_u (local.get $i) (local.get $depth)))
        (local.set $len
          (i32.add
            (local.get $len)
            (call $json_write_gap_once
              (i32.add (local.get $ptr) (local.get $len))
              (local.get $gap)
              (local.get $gap_ptr))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $indent_loop)))
    (local.get $len))

  (func $json_stringify_into (param $v i32) (param $ptr i32) (param $gap i32) (param $gap_ptr i32) (param $depth i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $len i32)
    (local $out i32)
    (local $i i32)
    (local $entry_base i32)
    (local $key_raw i32)
    (local $key_base i32)
    (local $key_len i32)
    (local $child_len i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $v) (i32.const {undefined}))
      (then (return (i32.const {unsupported}))))
    (if
      (i32.or
        (i32.eq (local.get $v) (i32.const {null_tag}))
        (i32.or
          (i32.eq (local.get $v) (i32.const {false_tag}))
          (i32.or
            (i32.eq (local.get $v) (i32.const {true_tag}))
            (i32.eq (local.get $tag) (i32.const {number_tag})))))
      (then (return (call $value_to_string_into (local.get $v) (local.get $ptr)))))
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then
        (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $base)))
        (i32.store8 (local.get $ptr) (i32.const {quote}))
        (call $copy
          (i32.add (local.get $base) (i32.const {header}))
          (i32.add (local.get $ptr) (i32.const {one}))
          (local.get $len))
        (i32.store8
          (i32.add (local.get $ptr) (i32.add (local.get $len) (i32.const {one})))
          (i32.const {quote}))
        (return (i32.add (local.get $len) (i32.const {two})))))
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then
        (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $base)))
        (i32.store8 (local.get $ptr) (i32.const {lbracket}))
        (local.set $out (i32.const {one}))
        (if
          (i32.and
            (i32.gt_u (local.get $len) (i32.const {zero}))
            (i32.gt_u (local.get $gap) (i32.const {zero})))
          (then
            (local.set $out
              (i32.add
                (local.get $out)
                (call $json_write_newline_indent
                  (i32.add (local.get $ptr) (local.get $out))
                  (local.get $gap)
                  (local.get $gap_ptr)
                  (i32.add (local.get $depth) (i32.const {one})))))))
        (block $array_done
          (loop $array_loop
            (br_if $array_done (i32.ge_u (local.get $i) (local.get $len)))
            (if (i32.gt_u (local.get $i) (i32.const {zero}))
              (then
                (i32.store8
                  (i32.add (local.get $ptr) (local.get $out))
                  (i32.const {comma}))
                (local.set $out (i32.add (local.get $out) (i32.const {one})))
                (if (i32.gt_u (local.get $gap) (i32.const {zero}))
                  (then
                    (local.set $out
                      (i32.add
                        (local.get $out)
                        (call $json_write_newline_indent
                          (i32.add (local.get $ptr) (local.get $out))
                          (local.get $gap)
                          (local.get $gap_ptr)
                          (i32.add (local.get $depth) (i32.const {one})))))))))
            (local.set $child_len
              (call $json_stringify_into
                (i32.load
                  (i32.add
                    (local.get $base)
                    (i32.add
                      (i32.const {array_header})
                      (i32.shl (local.get $i) (i32.const {elem_shift})))))
                (i32.add (local.get $ptr) (local.get $out))
                (local.get $gap)
                (local.get $gap_ptr)
                (i32.add (local.get $depth) (i32.const {one}))))
            (if (i32.lt_s (local.get $child_len) (i32.const {zero}))
              (then
                (local.set $child_len
                  (call $value_to_string_into
                    (i32.const {null_tag})
                    (i32.add (local.get $ptr) (local.get $out))))))
            (local.set $out (i32.add (local.get $out) (local.get $child_len)))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $array_loop)))
        (if
          (i32.and
            (i32.gt_u (local.get $len) (i32.const {zero}))
            (i32.gt_u (local.get $gap) (i32.const {zero})))
          (then
            (local.set $out
              (i32.add
                (local.get $out)
                (call $json_write_newline_indent
                  (i32.add (local.get $ptr) (local.get $out))
                  (local.get $gap)
                  (local.get $gap_ptr)
                  (local.get $depth))))))
        (i32.store8
          (i32.add (local.get $ptr) (local.get $out))
          (i32.const {rbracket}))
        (return (i32.add (local.get $out) (i32.const {one})))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $base)))
        (i32.store8 (local.get $ptr) (i32.const {lbrace}))
        (local.set $out (i32.const {one}))
        (local.set $i (i32.const {zero}))
        (if
          (i32.and
            (i32.gt_u (local.get $len) (i32.const {zero}))
            (i32.gt_u (local.get $gap) (i32.const {zero})))
          (then
            (local.set $out
              (i32.add
                (local.get $out)
                (call $json_write_newline_indent
                  (i32.add (local.get $ptr) (local.get $out))
                  (local.get $gap)
                  (local.get $gap_ptr)
                  (i32.add (local.get $depth) (i32.const {one})))))))
        (block $object_done
          (loop $object_loop
            (br_if $object_done (i32.ge_u (local.get $i) (local.get $len)))
            (local.set $entry_base
              (i32.add
                (local.get $base)
                (i32.add
                  (i32.const {obj_entries})
                  (i32.shl (local.get $i) (i32.const {entry_shift})))))
            (local.set $key_raw (i32.load (local.get $entry_base)))
            (local.set $key_base (i32.and (local.get $key_raw) (i32.const {heap_mask})))
            (local.set $key_len (i32.load (local.get $key_base)))
            (if (i32.gt_u (local.get $i) (i32.const {zero}))
              (then
                (i32.store8
                  (i32.add (local.get $ptr) (local.get $out))
                  (i32.const {comma}))
                (local.set $out (i32.add (local.get $out) (i32.const {one})))
                (if (i32.gt_u (local.get $gap) (i32.const {zero}))
                  (then
                    (local.set $out
                      (i32.add
                        (local.get $out)
                        (call $json_write_newline_indent
                          (i32.add (local.get $ptr) (local.get $out))
                          (local.get $gap)
                          (local.get $gap_ptr)
                          (i32.add (local.get $depth) (i32.const {one})))))))))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {quote}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (call $copy
              (i32.add (local.get $key_base) (i32.const {header}))
              (i32.add (local.get $ptr) (local.get $out))
              (local.get $key_len))
            (local.set $out (i32.add (local.get $out) (local.get $key_len)))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {quote}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {colon}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (if (i32.gt_u (local.get $gap) (i32.const {zero}))
              (then
                (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {space_char}))
                (local.set $out (i32.add (local.get $out) (i32.const {one})))))
            (local.set $child_len
              (call $json_stringify_into
                (i32.load (i32.add (local.get $entry_base) (i32.const {value_off})))
                (i32.add (local.get $ptr) (local.get $out))
                (local.get $gap)
                (local.get $gap_ptr)
                (i32.add (local.get $depth) (i32.const {one}))))
            (if (i32.lt_s (local.get $child_len) (i32.const {zero}))
              (then (return (i32.const {unsupported}))))
            (local.set $out (i32.add (local.get $out) (local.get $child_len)))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $object_loop)))
        (if
          (i32.and
            (i32.gt_u (local.get $len) (i32.const {zero}))
            (i32.gt_u (local.get $gap) (i32.const {zero})))
          (then
            (local.set $out
              (i32.add
                (local.get $out)
                (call $json_write_newline_indent
                  (i32.add (local.get $ptr) (local.get $out))
                  (local.get $gap)
                  (local.get $gap_ptr)
                  (local.get $depth))))))
        (i32.store8
          (i32.add (local.get $ptr) (local.get $out))
          (i32.const {rbrace}))
        (return (i32.add (local.get $out) (i32.const {one})))))
    (i32.const {unsupported}))
"#,
            header = Layout::STRING_HEADER_SIZE,
            stringify_alloc_size = Layout::STRING_HEADER_SIZE + 1024,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            obj_entries = Layout::OBJECT_ENTRIES_OFFSET,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            unsupported = -1,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            two = 2,
            max_gap = 10,
            newline = 10,
            space_char = 32,
            quote = 34,
            colon = 58,
            comma = 44,
            lbrace = 123,
            rbrace = 125,
            lbracket = 91,
            rbracket = 93,
        ));
    }

    pub(super) fn emit_json_parse(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_parse (param $s i32) (result i32)
    (local $s_obj i32)
    (local $s_len i32)
    (local $pos i32)
    (local $ch i32)
    (local $value i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $pos (call $json_skip_whitespace (local.get $s_obj) (local.get $s_len) (i32.const {zero})))
    (if (i32.ge_u (local.get $pos) (local.get $s_len))
      (then (unreachable)))
    (local.set $ch
      (i32.load8_u
        (i32.add
          (i32.add (local.get $s_obj) (i32.const {str_header}))
          (local.get $pos))))
    (block $parsed_value
      (if (i32.eq (local.get $ch) (i32.const {lbrace}))
        (then
          (local.set $value (call $json_parse_object (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.eq (local.get $value) (i32.const {undefined}))
            (then (unreachable)))
          (local.set $pos (call $json_skip_container (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (br $parsed_value)))
      (if (i32.eq (local.get $ch) (i32.const {lbracket}))
        (then
          (local.set $value (call $json_parse_array (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.eq (local.get $value) (i32.const {undefined}))
            (then (unreachable)))
          (local.set $pos (call $json_skip_container (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (br $parsed_value)))
      (if (i32.eq (local.get $ch) (i32.const {quote}))
        (then
          (local.set $value (call $json_parse_string (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.eq (local.get $value) (i32.const {undefined}))
            (then (unreachable)))
          (local.set $pos (call $json_skip_string (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.gt_u (local.get $pos) (local.get $s_len))
            (then (return (i32.const {undefined}))))
          (br $parsed_value)))
      (if
        (call $json_match_literal4
          (local.get $s_obj)
          (local.get $s_len)
          (local.get $pos)
          (i32.const {ascii_n})
          (i32.const {ascii_u})
          (i32.const {ascii_l})
          (i32.const {ascii_l}))
        (then
          (local.set $value (i32.const {null_tag}))
          (local.set $pos (i32.add (local.get $pos) (i32.const 4)))
          (br $parsed_value)))
      (if
        (call $json_match_literal4
          (local.get $s_obj)
          (local.get $s_len)
          (local.get $pos)
          (i32.const {ascii_t})
          (i32.const {ascii_r})
          (i32.const {ascii_u})
          (i32.const {ascii_e}))
        (then
          (local.set $value (i32.const {true_tag}))
          (local.set $pos (i32.add (local.get $pos) (i32.const 4)))
          (br $parsed_value)))
      (if
        (call $json_match_literal5
          (local.get $s_obj)
          (local.get $s_len)
          (local.get $pos)
          (i32.const {ascii_f})
          (i32.const {ascii_a})
          (i32.const {ascii_l})
          (i32.const {ascii_s})
          (i32.const {ascii_e}))
        (then
          (local.set $value (i32.const {false_tag}))
          (local.set $pos (i32.add (local.get $pos) (i32.const 5)))
          (br $parsed_value)))
      (local.set $value (call $json_parse_number_value (local.get $s_obj) (local.get $s_len) (local.get $pos)))
      (if (i32.eq (local.get $value) (i32.const {undefined}))
        (then (unreachable)))
      (local.set $pos (call $json_skip_number (local.get $s_obj) (local.get $s_len) (local.get $pos)))
      (if (i32.gt_u (local.get $pos) (local.get $s_len))
        (then (return (i32.const {undefined}))))
      (br $parsed_value))
    (local.set $pos (call $json_skip_whitespace (local.get $s_obj) (local.get $s_len) (local.get $pos)))
    (if (i32.ne (local.get $pos) (local.get $s_len))
      (then (unreachable)))
    (local.get $value))

  (func $json_match_literal4 (param $obj i32) (param $len i32) (param $pos i32) (param $c0 i32) (param $c1 i32) (param $c2 i32) (param $c3 i32) (result i32)
    (if (i32.gt_u (i32.add (local.get $pos) (i32.const 4)) (local.get $len))
      (then (return (i32.const {zero}))))
    (i32.and
      (i32.and
        (i32.eq
          (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (local.get $pos)))
          (local.get $c0))
        (i32.eq
          (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $pos) (i32.const {one}))))
          (local.get $c1)))
      (i32.and
        (i32.eq
          (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $pos) (i32.const 2))))
          (local.get $c2))
        (i32.eq
          (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $pos) (i32.const 3))))
          (local.get $c3)))))

  (func $json_match_literal5 (param $obj i32) (param $len i32) (param $pos i32) (param $c0 i32) (param $c1 i32) (param $c2 i32) (param $c3 i32) (param $c4 i32) (result i32)
    (if (i32.gt_u (i32.add (local.get $pos) (i32.const 5)) (local.get $len))
      (then (return (i32.const {zero}))))
    (i32.and
      (call $json_match_literal4
        (local.get $obj)
        (local.get $len)
        (local.get $pos)
        (local.get $c0)
        (local.get $c1)
        (local.get $c2)
        (local.get $c3))
      (i32.eq
        (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $pos) (i32.const 4))))
        (local.get $c4))))

  (func $json_hex_value (param $ch i32) (result i32)
    (if
      (i32.and
        (i32.ge_u (local.get $ch) (i32.const {ascii_zero}))
        (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
      (then
        (return (i32.sub (local.get $ch) (i32.const {ascii_zero})))))
    (if
      (i32.and
        (i32.ge_u (local.get $ch) (i32.const {ascii_upper_a}))
        (i32.le_u (local.get $ch) (i32.const {ascii_upper_f})))
      (then
        (return
          (i32.add
            (i32.sub (local.get $ch) (i32.const {ascii_upper_a}))
            (i32.const {ten})))))
    (if
      (i32.and
        (i32.ge_u (local.get $ch) (i32.const {ascii_lower_a}))
        (i32.le_u (local.get $ch) (i32.const {ascii_lower_f})))
      (then
        (return
          (i32.add
            (i32.sub (local.get $ch) (i32.const {ascii_lower_a}))
            (i32.const {ten})))))
    (i32.const -1))

  (func $json_parse_unicode_escape_byte (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $d0 i32)
    (local $d1 i32)
    (local $d2 i32)
    (local $d3 i32)
    (local $code i32)
    (if (i32.gt_u (i32.add (local.get $pos) (i32.const 4)) (local.get $len))
      (then (return (i32.const -1))))
    (local.set $d0
      (call $json_hex_value
        (i32.load8_u
          (i32.add
            (i32.add (local.get $obj) (i32.const {str_header}))
            (local.get $pos)))))
    (local.set $d1
      (call $json_hex_value
        (i32.load8_u
          (i32.add
            (i32.add (local.get $obj) (i32.const {str_header}))
            (i32.add (local.get $pos) (i32.const {one}))))))
    (local.set $d2
      (call $json_hex_value
        (i32.load8_u
          (i32.add
            (i32.add (local.get $obj) (i32.const {str_header}))
            (i32.add (local.get $pos) (i32.const 2))))))
    (local.set $d3
      (call $json_hex_value
        (i32.load8_u
          (i32.add
            (i32.add (local.get $obj) (i32.const {str_header}))
            (i32.add (local.get $pos) (i32.const 3))))))
    (if
      (i32.or
        (i32.or
          (i32.lt_s (local.get $d0) (i32.const {zero}))
          (i32.lt_s (local.get $d1) (i32.const {zero})))
        (i32.or
          (i32.lt_s (local.get $d2) (i32.const {zero}))
          (i32.lt_s (local.get $d3) (i32.const {zero}))))
      (then (return (i32.const -1))))
    (local.set $code
      (i32.add
        (i32.add
          (i32.shl (local.get $d0) (i32.const 12))
          (i32.shl (local.get $d1) (i32.const 8)))
        (i32.add
          (i32.shl (local.get $d2) (i32.const 4))
          (local.get $d3))))
    (if (i32.gt_u (local.get $code) (i32.const {ascii_max}))
      (then (return (i32.const -1))))
    (local.get $code))

  (func $json_parse_string (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $start i32)
    (local $out_len i32)
    (local $out_pos i32)
    (local $result_ptr i32)
    (local $ch i32)
    (local $store_ch i32)
    (local.set $start (i32.add (local.get $pos) (i32.const {one})))
    (local.set $pos (local.get $start))
    (block $found
      (loop $scan
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (br_if $found (i32.eq (local.get $ch) (i32.const {quote})))
        (if (i32.eq (local.get $ch) (i32.const {backslash}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (if (i32.ge_u (local.get $pos) (local.get $len))
              (then (return (i32.const {undefined}))))
            (local.set $ch
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {str_header}))
                  (local.get $pos))))
            (if (i32.eq (local.get $ch) (i32.const {ascii_u}))
              (then
                (if
                  (i32.lt_s
                    (call $json_parse_unicode_escape_byte
                      (local.get $obj)
                      (local.get $len)
                      (i32.add (local.get $pos) (i32.const {one})))
                    (i32.const {zero}))
                  (then (return (i32.const {undefined}))))
                (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
              (else
                (if
                  (i32.eqz
                    (i32.or
                      (i32.or
                        (i32.or
                          (i32.eq (local.get $ch) (i32.const {quote}))
                          (i32.eq (local.get $ch) (i32.const {backslash})))
                        (i32.or
                          (i32.eq (local.get $ch) (i32.const {slash}))
                          (i32.eq (local.get $ch) (i32.const {ascii_b}))))
                      (i32.or
                        (i32.or
                          (i32.eq (local.get $ch) (i32.const {ascii_f}))
                          (i32.eq (local.get $ch) (i32.const {ascii_n})))
                        (i32.or
                          (i32.eq (local.get $ch) (i32.const {ascii_r}))
                          (i32.eq (local.get $ch) (i32.const {ascii_t}))))))
                  (then (return (i32.const {undefined}))))))))
        (local.set $out_len (i32.add (local.get $out_len) (i32.const {one})))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $scan)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $out_len))))
    (i32.store (local.get $result_ptr) (local.get $out_len))
    (local.set $pos (local.get $start))
    (block $copy_done
      (loop $copy_loop
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (br_if $copy_done (i32.eq (local.get $ch) (i32.const {quote})))
        (local.set $store_ch (local.get $ch))
        (if (i32.eq (local.get $ch) (i32.const {backslash}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (local.set $ch
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {str_header}))
                  (local.get $pos))))
            (local.set $store_ch (local.get $ch))
            (if (i32.eq (local.get $ch) (i32.const {ascii_u}))
              (then
                (local.set $store_ch
                  (call $json_parse_unicode_escape_byte
                    (local.get $obj)
                    (local.get $len)
                    (i32.add (local.get $pos) (i32.const {one}))))
                (if (i32.lt_s (local.get $store_ch) (i32.const {zero}))
                  (then (return (i32.const {undefined}))))
                (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
              (else
                (if (i32.eq (local.get $ch) (i32.const {ascii_b}))
                  (then (local.set $store_ch (i32.const {backspace}))))
                (if (i32.eq (local.get $ch) (i32.const {ascii_f}))
                  (then (local.set $store_ch (i32.const {formfeed}))))
                (if (i32.eq (local.get $ch) (i32.const {ascii_n}))
                  (then (local.set $store_ch (i32.const {newline}))))
                (if (i32.eq (local.get $ch) (i32.const {ascii_r}))
                  (then (local.set $store_ch (i32.const {carriage}))))
                (if (i32.eq (local.get $ch) (i32.const {ascii_t}))
                  (then (local.set $store_ch (i32.const {tab}))))))))
        (i32.store8
          (i32.add
            (i32.add (local.get $result_ptr) (i32.const {str_header}))
            (local.get $out_pos))
          (local.get $store_ch))
        (local.set $out_pos (i32.add (local.get $out_pos) (i32.const {one})))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $copy_loop)))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))

  (func $json_parse_number_value (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $ch i32)
    (local $sign i32)
    (local $n i32)
    (local $saw_digit i32)
    (local $saw_frac_digit i32)
    (local $frac_digits i32)
    (local $exp_sign i32)
    (local $exp i32)
    (local $saw_exp_digit i32)
    (local $scale i32)
    (local.set $sign (i32.const {one}))
    (local.set $exp_sign (i32.const {one}))
    (if (i32.ge_u (local.get $pos) (local.get $len))
      (then (return (i32.const {undefined}))))
    (local.set $ch
      (i32.load8_u
        (i32.add
          (i32.add (local.get $obj) (i32.const {str_header}))
          (local.get $pos))))
    (if (i32.eq (local.get $ch) (i32.const {minus}))
      (then
        (local.set $sign (i32.const -1))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))))
    (block $int_done
      (loop $int_loop
        (br_if $int_done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (br_if $int_done
          (i32.or
            (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
            (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
        (local.set $saw_digit (i32.const {one}))
        (local.set $n
          (i32.add
            (i32.mul (local.get $n) (i32.const {ten}))
            (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $int_loop)))
    (if (i32.eqz (local.get $saw_digit))
      (then (return (i32.const {undefined}))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {dot}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (block $frac_done
              (loop $frac_loop
                (br_if $frac_done (i32.ge_u (local.get $pos) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (local.get $pos))))
                (br_if $frac_done
                  (i32.or
                    (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
                (local.set $saw_frac_digit (i32.const {one}))
                (local.set $frac_digits (i32.add (local.get $frac_digits) (i32.const {one})))
                (local.set $n
                  (i32.add
                    (i32.mul (local.get $n) (i32.const {ten}))
                    (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
                (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
                (br $frac_loop)))
            (if (i32.eqz (local.get $saw_frac_digit))
              (then (return (i32.const {undefined}))))))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_e}))
            (i32.eq (local.get $ch) (i32.const {ascii_E})))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (if (i32.ge_u (local.get $pos) (local.get $len))
              (then (return (i32.const {undefined}))))
            (local.set $ch
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {str_header}))
                  (local.get $pos))))
            (if (i32.eq (local.get $ch) (i32.const {plus}))
              (then
                (local.set $pos (i32.add (local.get $pos) (i32.const {one}))))
              (else
                (if (i32.eq (local.get $ch) (i32.const {minus}))
                  (then
                    (local.set $exp_sign (i32.const -1))
                    (local.set $pos (i32.add (local.get $pos) (i32.const {one})))))))
            (block $exp_done
              (loop $exp_loop
                (br_if $exp_done (i32.ge_u (local.get $pos) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (local.get $pos))))
                (br_if $exp_done
                  (i32.or
                    (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
                (local.set $saw_exp_digit (i32.const {one}))
                (local.set $exp
                  (i32.add
                    (i32.mul (local.get $exp) (i32.const {ten}))
                    (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
                (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
                (br $exp_loop)))
            (if (i32.eqz (local.get $saw_exp_digit))
              (then (return (i32.const {undefined}))))))))
    (local.set $scale
      (i32.sub
        (i32.mul (local.get $exp) (local.get $exp_sign))
        (local.get $frac_digits)))
    (block $scale_up_done
      (loop $scale_up
        (br_if $scale_up_done (i32.le_s (local.get $scale) (i32.const {zero})))
        (local.set $n (i32.mul (local.get $n) (i32.const {ten})))
        (local.set $scale (i32.sub (local.get $scale) (i32.const {one})))
        (br $scale_up)))
    (block $scale_down_done
      (loop $scale_down
        (br_if $scale_down_done (i32.ge_s (local.get $scale) (i32.const {zero})))
        (if (i32.ne (i32.rem_u (local.get $n) (i32.const {ten})) (i32.const {zero}))
          (then (unreachable)))
        (local.set $n (i32.div_u (local.get $n) (i32.const {ten})))
        (local.set $scale (i32.add (local.get $scale) (i32.const {one})))
        (br $scale_down)))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or
      (i32.shl (local.get $n) (i32.const {number_shift}))
      (i32.const {number_tag})))

  (func $json_skip_number (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $ch i32)
    (local $saw_digit i32)
    (local $saw_frac_digit i32)
    (local $saw_exp_digit i32)
    (if (i32.ge_u (local.get $pos) (local.get $len))
      (then (return (i32.add (local.get $len) (i32.const {one})))))
    (local.set $ch
      (i32.load8_u
        (i32.add
          (i32.add (local.get $obj) (i32.const {str_header}))
          (local.get $pos))))
    (if (i32.eq (local.get $ch) (i32.const {minus}))
      (then (local.set $pos (i32.add (local.get $pos) (i32.const {one})))))
    (block $int_done
      (loop $int_loop
        (br_if $int_done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (br_if $int_done
          (i32.or
            (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
            (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
        (local.set $saw_digit (i32.const {one}))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $int_loop)))
    (if (i32.eqz (local.get $saw_digit))
      (then (return (i32.add (local.get $len) (i32.const {one})))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {dot}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (block $frac_done
              (loop $frac_loop
                (br_if $frac_done (i32.ge_u (local.get $pos) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (local.get $pos))))
                (br_if $frac_done
                  (i32.or
                    (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
                (local.set $saw_frac_digit (i32.const {one}))
                (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
                (br $frac_loop)))
            (if (i32.eqz (local.get $saw_frac_digit))
              (then (return (i32.add (local.get $len) (i32.const {one})))))))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_e}))
            (i32.eq (local.get $ch) (i32.const {ascii_E})))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (if (i32.ge_u (local.get $pos) (local.get $len))
              (then (return (i32.add (local.get $len) (i32.const {one})))))
            (local.set $ch
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {str_header}))
                  (local.get $pos))))
            (if
              (i32.or
                (i32.eq (local.get $ch) (i32.const {plus}))
                (i32.eq (local.get $ch) (i32.const {minus})))
              (then (local.set $pos (i32.add (local.get $pos) (i32.const {one})))))
            (block $exp_done
              (loop $exp_loop
                (br_if $exp_done (i32.ge_u (local.get $pos) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (local.get $pos))))
                (br_if $exp_done
                  (i32.or
                    (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
                (local.set $saw_exp_digit (i32.const {one}))
                (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
                (br $exp_loop)))
            (if (i32.eqz (local.get $saw_exp_digit))
              (then (return (i32.add (local.get $len) (i32.const {one})))))))))
    (local.get $pos))

  (func $json_skip_string (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $ch i32)
    (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
    (block $done
      (loop $scan
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.add (local.get $len) (i32.const {one})))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (if (i32.eq (local.get $ch) (i32.const {backslash}))
          (then
            (if (i32.ge_u (local.get $pos) (local.get $len))
              (then (return (i32.add (local.get $len) (i32.const {one})))))
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (br $scan)))
        (br_if $done (i32.eq (local.get $ch) (i32.const {quote})))
        (br $scan)))
    (local.get $pos))

  (func $json_parse_object (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $result_ptr i32)
    (local $count i32)
    (local $ch i32)
    (local $key_obj i32)
    (local $entry_base i32)
    (local $value i32)
    (local $parsed_nested i32)
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {obj_header})
          (i32.shl (local.get $len) (i32.const {entry_shift})))))
    (i32.store (local.get $result_ptr) (i32.const {zero}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const {obj_proto}))
      (i32.const {zero}))
    (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
    (block $object_done
      (loop $object_loop
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (br_if $object_done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {rbrace}))
          (then (return (i32.or (local.get $result_ptr) (i32.const {object_tag})))))
        (if (i32.ne (local.get $ch) (i32.const {quote}))
          (then (return (i32.const {undefined}))))
        (local.set $key_obj (call $json_parse_string (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.eq (local.get $key_obj) (i32.const {undefined}))
          (then (return (i32.const {undefined}))))
        (local.set $pos (call $json_skip_string (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.gt_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.ne (local.get $ch) (i32.const {colon}))
          (then (return (i32.const {undefined}))))
        (local.set $pos
          (call $json_skip_whitespace
            (local.get $obj)
            (local.get $len)
            (i32.add (local.get $pos) (i32.const {one}))))
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (local.set $parsed_nested (i32.const {zero}))
        (if (i32.eq (local.get $ch) (i32.const {lbrace}))
          (then
            (local.set $value (call $json_parse_object (local.get $obj) (local.get $len) (local.get $pos)))
            (if (i32.eq (local.get $value) (i32.const {undefined}))
              (then (return (i32.const {undefined}))))
            (local.set $pos (call $json_skip_container (local.get $obj) (local.get $len) (local.get $pos)))
            (local.set $parsed_nested (i32.const {one}))))
        (if (i32.eq (local.get $ch) (i32.const {lbracket}))
          (then
            (local.set $value (call $json_parse_array (local.get $obj) (local.get $len) (local.get $pos)))
            (if (i32.eq (local.get $value) (i32.const {undefined}))
              (then (return (i32.const {undefined}))))
            (local.set $pos (call $json_skip_container (local.get $obj) (local.get $len) (local.get $pos)))
            (local.set $parsed_nested (i32.const {one}))))
        (if (i32.eqz (local.get $parsed_nested))
          (then
            (if (i32.eq (local.get $ch) (i32.const {quote}))
              (then
                (local.set $value (call $json_parse_string (local.get $obj) (local.get $len) (local.get $pos)))
                (if (i32.eq (local.get $value) (i32.const {undefined}))
                  (then (return (i32.const {undefined}))))
                (local.set $pos (call $json_skip_string (local.get $obj) (local.get $len) (local.get $pos)))
                (if (i32.gt_u (local.get $pos) (local.get $len))
                  (then (return (i32.const {undefined})))))
              (else
                (if
                  (call $json_match_literal4
                    (local.get $obj)
                    (local.get $len)
                    (local.get $pos)
                    (i32.const {ascii_t})
                    (i32.const {ascii_r})
                    (i32.const {ascii_u})
                    (i32.const {ascii_e}))
                  (then
                    (local.set $value (i32.const {true_tag}))
                    (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
                  (else
                    (if
                      (call $json_match_literal5
                        (local.get $obj)
                        (local.get $len)
                        (local.get $pos)
                        (i32.const {ascii_f})
                        (i32.const {ascii_a})
                        (i32.const {ascii_l})
                        (i32.const {ascii_s})
                        (i32.const {ascii_e}))
                      (then
                        (local.set $value (i32.const {false_tag}))
                        (local.set $pos (i32.add (local.get $pos) (i32.const 5))))
                      (else
                        (if
                          (call $json_match_literal4
                            (local.get $obj)
                            (local.get $len)
                            (local.get $pos)
                            (i32.const {ascii_n})
                            (i32.const {ascii_u})
                            (i32.const {ascii_l})
                            (i32.const {ascii_l}))
                          (then
                            (local.set $value (i32.const {null_tag}))
                            (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
                          (else
                            (local.set $value (call $json_parse_number_value (local.get $obj) (local.get $len) (local.get $pos)))
                            (if (i32.eq (local.get $value) (i32.const {undefined}))
                              (then (return (i32.const {undefined}))))
                            (local.set $pos (call $json_skip_number (local.get $obj) (local.get $len) (local.get $pos)))
                            (if (i32.gt_u (local.get $pos) (local.get $len))
                              (then (return (i32.const {undefined}))))))))))))))
        (local.set $entry_base
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {obj_entries})
              (i32.shl (local.get $count) (i32.const {entry_shift})))))
        (i32.store (local.get $entry_base) (local.get $key_obj))
        (i32.store (i32.add (local.get $entry_base) (i32.const {value_off})) (local.get $value))
        (local.set $count (i32.add (local.get $count) (i32.const {one})))
        (i32.store (local.get $result_ptr) (local.get $count))
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {comma}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one}))
            )
            (br $object_loop)))
        (if (i32.eq (local.get $ch) (i32.const {rbrace}))
          (then (return (i32.or (local.get $result_ptr) (i32.const {object_tag})))))
        (return (i32.const {undefined}))))
    (i32.const {undefined}))

  (func $json_parse_array (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $result_ptr i32)
    (local $count i32)
    (local $ch i32)
    (local $value i32)
    (local $parsed_nested i32)
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (i32.const {zero}))
    (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
    (block $array_done
      (loop $array_loop
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (br_if $array_done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {rbracket}))
          (then (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
        (local.set $parsed_nested (i32.const {zero}))
        (if (i32.eq (local.get $ch) (i32.const {lbrace}))
          (then
            (local.set $value (call $json_parse_object (local.get $obj) (local.get $len) (local.get $pos)))
            (if (i32.eq (local.get $value) (i32.const {undefined}))
              (then (return (i32.const {undefined}))))
            (local.set $pos (call $json_skip_container (local.get $obj) (local.get $len) (local.get $pos)))
            (local.set $parsed_nested (i32.const {one}))))
        (if (i32.eq (local.get $ch) (i32.const {lbracket}))
          (then
            (local.set $value (call $json_parse_array (local.get $obj) (local.get $len) (local.get $pos)))
            (if (i32.eq (local.get $value) (i32.const {undefined}))
              (then (return (i32.const {undefined}))))
            (local.set $pos (call $json_skip_container (local.get $obj) (local.get $len) (local.get $pos)))
            (local.set $parsed_nested (i32.const {one}))))
        (if (i32.eqz (local.get $parsed_nested))
          (then
            (if (i32.eq (local.get $ch) (i32.const {quote}))
              (then
                (local.set $value (call $json_parse_string (local.get $obj) (local.get $len) (local.get $pos)))
                (if (i32.eq (local.get $value) (i32.const {undefined}))
                  (then (return (i32.const {undefined}))))
                (local.set $pos (call $json_skip_string (local.get $obj) (local.get $len) (local.get $pos)))
                (if (i32.gt_u (local.get $pos) (local.get $len))
                  (then (return (i32.const {undefined})))))
              (else
                (if
                  (call $json_match_literal4
                    (local.get $obj)
                    (local.get $len)
                    (local.get $pos)
                    (i32.const {ascii_t})
                    (i32.const {ascii_r})
                    (i32.const {ascii_u})
                    (i32.const {ascii_e}))
                  (then
                    (local.set $value (i32.const {true_tag}))
                    (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
                  (else
                    (if
                      (call $json_match_literal5
                        (local.get $obj)
                        (local.get $len)
                        (local.get $pos)
                        (i32.const {ascii_f})
                        (i32.const {ascii_a})
                        (i32.const {ascii_l})
                        (i32.const {ascii_s})
                        (i32.const {ascii_e}))
                      (then
                        (local.set $value (i32.const {false_tag}))
                        (local.set $pos (i32.add (local.get $pos) (i32.const 5))))
                      (else
                        (if
                          (call $json_match_literal4
                            (local.get $obj)
                            (local.get $len)
                            (local.get $pos)
                            (i32.const {ascii_n})
                            (i32.const {ascii_u})
                            (i32.const {ascii_l})
                            (i32.const {ascii_l}))
                          (then
                            (local.set $value (i32.const {null_tag}))
                            (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
                          (else
                            (local.set $value (call $json_parse_number_value (local.get $obj) (local.get $len) (local.get $pos)))
                            (if (i32.eq (local.get $value) (i32.const {undefined}))
                              (then (return (i32.const {undefined}))))
                            (local.set $pos (call $json_skip_number (local.get $obj) (local.get $len) (local.get $pos)))
                            (if (i32.gt_u (local.get $pos) (local.get $len))
                              (then (return (i32.const {undefined}))))))))))))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $count) (i32.const {elem_shift}))))
          (local.get $value))
        (local.set $count (i32.add (local.get $count) (i32.const {one})))
        (i32.store (local.get $result_ptr) (local.get $count))
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {comma}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (br $array_loop)))
        (if (i32.eq (local.get $ch) (i32.const {rbracket}))
          (then (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
        (return (i32.const {undefined}))))
    (i32.const {undefined}))

  (func $json_skip_container (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $ch i32)
    (local $depth i32)
    (block $scan_done
      (loop $scan
        (br_if $scan_done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {quote}))
          (then
            (local.set $pos (call $json_skip_string (local.get $obj) (local.get $len) (local.get $pos)))
            (if (i32.gt_u (local.get $pos) (local.get $len))
              (then (return (local.get $len))))
            (br $scan)))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {lbrace}))
            (i32.eq (local.get $ch) (i32.const {lbracket})))
          (then
            (local.set $depth (i32.add (local.get $depth) (i32.const {one})))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {rbrace}))
            (i32.eq (local.get $ch) (i32.const {rbracket})))
          (then
            (local.set $depth (i32.sub (local.get $depth) (i32.const {one})))
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (if (i32.eqz (local.get $depth))
              (then (return (local.get $pos))))
            (br $scan)))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $scan)))
    (local.get $len))

  (func $json_skip_whitespace (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $ch i32)
    (block $done
      (loop $skip
        (br_if $done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (local.get $pos))))
        ;; Check if whitespace (32=space, 9=tab, 10=newline, 13=carriage return)
        (if (i32.eq (local.get $ch) (i32.const {space})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (if (i32.eq (local.get $ch) (i32.const {tab})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (if (i32.eq (local.get $ch) (i32.const {newline})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (if (i32.eq (local.get $ch) (i32.const {carriage})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (br $done)))
    (local.get $pos))
"#,
            undefined = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            string_tag = ValueTag::STRING,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            array_header = Layout::ARRAY_HEADER_SIZE,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            obj_entries = Layout::OBJECT_ENTRIES_OFFSET,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            ten = RuntimeConst::TEN,
            quote = 34,
            backslash = 92,
            slash = 47,
            colon = 58,
            comma = 44,
            dot = 46,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            lbrace = 123,
            rbrace = 125,
            lbracket = 91,
            rbracket = 93,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            ascii_upper_a = 65,
            ascii_upper_f = 70,
            ascii_a = 97,
            ascii_lower_a = 97,
            ascii_lower_f = 102,
            ascii_max = 127,
            ascii_b = 98,
            ascii_e = 101,
            ascii_E = 69,
            ascii_f = 102,
            ascii_l = 108,
            ascii_n = 110,
            ascii_r = 114,
            ascii_s = 115,
            ascii_t = 116,
            ascii_u = 117,
            backspace = 8,
            formfeed = 12,
            space = 32,
            tab = 9,
            newline = 10,
            carriage = 13,
        ));
    }

    /// Emit `$module_require(id: i32) → i32`.
    pub(super) fn emit_module_require(&self, wat: &mut String) {
        let entry_size = ts2wasm_runtime_abi::Layout::MODULE_CACHE_ENTRY_SIZE;
        wat.push_str(&format!(
            r#"
  (func $module_require (param $id i32) (result i32)
    (local $entry i32)
    (local $loaded i32)
    (local $exports i32)
    (local.set $entry (i32.add (global.get $module_cache) (i32.mul (local.get $id) (i32.const {entry_size}))))
    (local.set $loaded (i32.load (local.get $entry)))
    (if (i32.eqz (local.get $loaded))
      (then
        ;; Initialize an empty exports object once for this module ID.
        (local.set $exports (call $alloc_heap (i32.const {empty_obj_size})))
        (i32.store (local.get $exports) (i32.const {zero}))
        (i32.store (i32.add (local.get $exports) (i32.const {object_proto})) (i32.const {zero}))
        (i32.store (i32.add (local.get $entry) (i32.const {value_offset}))
          (i32.or (local.get $exports) (i32.const {object_tag})))
        (i32.store (local.get $entry) (i32.const {one}))))
    (i32.load (i32.add (local.get $entry) (i32.const {value_offset}))))
"#,
            entry_size = entry_size,
            empty_obj_size = Layout::OBJECT_HEADER_SIZE + (16 * Layout::OBJECT_ENTRY_SIZE),
            value_offset = 4,
            object_tag = ValueTag::OBJECT,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    /// Emit `$module_exports_set`.
    pub(super) fn emit_module_exports_set(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $module_exports_set (param $key_ptr i32) (param $key_len i32) (param $value i32)
    (local $entry i32)
    (local $loaded i32)
    (local $exports i32)
    (local.set $entry
      (i32.add
        (global.get $module_cache)
        (i32.mul (global.get $current_module_id) (i32.const {entry_size}))))
    (local.set $loaded (i32.load (local.get $entry)))
    (if (i32.eqz (local.get $loaded))
      (then
        (local.set $exports (call $alloc_heap (i32.const {empty_obj_size})))
        (i32.store (local.get $exports) (i32.const {zero}))
        (i32.store (i32.add (local.get $exports) (i32.const {object_proto})) (i32.const {zero}))
        (i32.store (i32.add (local.get $entry) (i32.const {value_offset}))
          (i32.or (local.get $exports) (i32.const {object_tag})))
        (i32.store (local.get $entry) (i32.const {one}))))
    (drop
      (call $property_set
        (i32.load (i32.add (local.get $entry) (i32.const {value_offset})))
        (local.get $key_ptr)
        (local.get $key_len)
        (local.get $value))))
"#,
            entry_size = Layout::MODULE_CACHE_ENTRY_SIZE,
            empty_obj_size = Layout::OBJECT_HEADER_SIZE + (16 * Layout::OBJECT_ENTRY_SIZE),
            value_offset = 4,
            object_tag = ValueTag::OBJECT,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    /// Emit `$module_exports_assign`.
    pub(super) fn emit_module_exports_assign(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $module_exports_assign (param $value i32)
    (local $entry i32)
    (local.set $entry
      (i32.add
      (global.get $module_cache)
      (i32.mul (global.get $current_module_id) (i32.const {entry_size}))))
    (i32.store (i32.add (local.get $entry) (i32.const {value_offset})) (local.get $value))
    (i32.store (local.get $entry) (i32.const {one})))
"#,
            entry_size = Layout::MODULE_CACHE_ENTRY_SIZE,
            value_offset = 4,
            one = RuntimeConst::ONE,
        ));
    }

    pub(super) fn emit_fs_read_file_sync(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $fs_read_file_sync (param $path i32) (param $encoding i32) (result i32)
    (call $host_fs_read_file_sync (local.get $path) (local.get $encoding)))
  "#,
        );
    }

    pub(super) fn emit_fs_write_file_sync(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
    (func $fs_write_file_sync (param $path i32) (param $data i32) (result i32)
    (call $host_fs_write_file_sync (local.get $path) (local.get $data))
    (i32.const {undefined}))
  "#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_fs_append_file_sync(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
    (func $fs_append_file_sync (param $path i32) (param $data i32) (result i32)
    (call $host_fs_append_file_sync (local.get $path) (local.get $data))
    (i32.const {undefined}))
  "#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_process_argv(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_argv (result i32)
    (call $host_process_argv))
  "#,
        );
    }

    pub(super) fn emit_process_env(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_env (result i32)
    (call $host_process_env))
  "#,
        );
    }

    pub(super) fn emit_process_exit(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_exit (param $code i32)
    (call $host_process_exit (local.get $code)))
  "#,
        );
    }

    pub(super) fn emit_path_join(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_join (param $a i32) (param $b i32) (result i32)
    (call $host_path_join (local.get $a) (local.get $b)))
  "#,
        );
    }

    pub(super) fn emit_path_resolve(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_resolve (param $path i32) (result i32)
    (call $host_path_resolve (local.get $path)))
  "#,
        );
    }

    pub(super) fn emit_path_basename(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_basename (param $path i32) (result i32)
    (call $host_path_basename (local.get $path)))
  "#,
        );
    }

    pub(super) fn emit_path_dirname(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_dirname (param $path i32) (result i32)
    (call $host_path_dirname (local.get $path)))
  "#,
        );
    }

    pub(super) fn emit_crypto_random_bytes(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $crypto_random_bytes (param $size i32) (result i32)
    (call $host_crypto_random_bytes (local.get $size)))
  "#,
        );
    }
}
