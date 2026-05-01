#[path = "runtime_builtins_host_json_parse.rs"]
mod runtime_builtins_host_json_parse;
use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

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

    pub(super) fn emit_math_pow(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_pow (param $base i32) (param $exp i32) (result i32)
    (local $base_n i32)
    (local $exp_n i32)
    (local $result i32)
    (local $i i32)
    (local.set $base_n (call $number_to_i32 (local.get $base)))
    (local.set $exp_n (call $number_to_i32 (local.get $exp)))
    ;; Simplified integer pow: base^exp
    ;; Handle special cases: exp = 0 returns 1, exp < 0 returns undefined (not supported for integers)
    (if (i32.lt_s (local.get $exp_n) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
    (if (i32.eq (local.get $exp_n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const 1) (i32.const {number_shift})) (i32.const {number_tag})))))
    (local.set $result (i32.const 1))
    (local.set $i (local.get $exp_n))
    (block $pow_break
      (loop $pow_loop
        (if (i32.eq (local.get $i) (i32.const {zero}))
          (then (br $pow_break)))
        (local.set $result (i32.mul (local.get $result) (local.get $base_n)))
        (local.set $i (i32.sub (local.get $i) (i32.const 1)))
        (br $pow_loop)))
    (call $number_from_i32 (local.get $result)))
"#,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
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
    (local $root_holder i32)
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
    (local.set $root_holder (call $alloc_heap (i32.const {root_holder_size})))
    (i32.store (local.get $root_holder) (i32.const {one}))
    (i32.store
      (i32.add (local.get $root_holder) (i32.const {object_proto}))
      (i32.const {zero}))
    (i32.store
      (i32.add (local.get $root_holder) (i32.const {obj_entries}))
      (i32.const {empty_string}))
    (i32.store
      (i32.add
        (i32.add (local.get $root_holder) (i32.const {obj_entries}))
        (i32.const {value_off}))
      (local.get $v))
    (local.set $result_ptr (call $alloc_heap (i32.const {stringify_alloc_size})))
    (local.set $len
      (call $json_stringify_into
        (local.get $v)
        (i32.add (local.get $result_ptr) (i32.const {header}))
        (local.get $gap)
        (local.get $gap_ptr)
        (i32.const {zero})
        (local.get $replacer)
        (i32.const {empty_string})
        (i32.or (local.get $root_holder) (i32.const {object_tag}))
        (i32.const {one})))
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

  (func $json_write_escaped_string (param $v i32) (param $ptr i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $out i32)
    (local $i i32)
    (local $ch i32)
    (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (i32.store8 (local.get $ptr) (i32.const {quote}))
    (local.set $out (i32.const {one}))
    (block $string_done
      (loop $string_loop
        (br_if $string_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $base) (i32.const {header}))
              (local.get $i))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {quote}))
            (i32.eq (local.get $ch) (i32.const {backslash})))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (local.get $ch))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {backspace}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_b}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {form_feed}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_f}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {newline}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_n}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {carriage_return}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_r}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {tab}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_t}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (local.get $ch))
        (local.set $out (i32.add (local.get $out) (i32.const {one})))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $string_loop)))
    (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {quote}))
    (i32.add (local.get $out) (i32.const {one}))
  )

  (func $json_array_index_key (param $i i32) (result i32)
    (local $result_ptr i32)
    (local $len i32)
    (local.set $result_ptr (call $alloc_heap (i32.const {index_key_alloc_size})))
    (local.set $len
      (call $value_to_string_into
        (i32.or
          (i32.shl (local.get $i) (i32.const {number_shift}))
          (i32.const {number_tag}))
        (i32.add (local.get $result_ptr) (i32.const {header}))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))

  (func $json_apply_replacer (param $replacer i32) (param $holder i32) (param $key i32) (param $value i32) (result i32)
    (if
      (i32.and
        (i32.ne (local.get $replacer) (i32.const {undefined}))
        (i32.ne (local.get $replacer) (i32.const {null_tag})))
      (then
        (return
          (call $json_replacer_call
            (local.get $replacer)
            (local.get $holder)
            (local.get $key)
            (local.get $value)))))
    (local.get $value))

  (func $json_stringify_into (param $v i32) (param $ptr i32) (param $gap i32) (param $gap_ptr i32) (param $depth i32) (param $replacer i32) (param $key i32) (param $holder i32) (param $apply_current i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $len i32)
    (local $out i32)
    (local $i i32)
    (local $entry_base i32)
    (local $key_raw i32)
    (local $child_value i32)
    (local $child_len i32)
    (local $emitted_count i32)
    (if (local.get $apply_current)
      (then
        (local.set $v
          (call $json_apply_replacer
            (local.get $replacer)
            (local.get $holder)
            (local.get $key)
            (local.get $v)))))
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
        (return (call $json_write_escaped_string (local.get $v) (local.get $ptr)))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (if (i32.eq (i32.load (i32.and (local.get $v) (i32.const {heap_mask}))) (i32.const {heap_number_sentinel}))
          (then
            (return (call $value_to_string_into (local.get $v) (local.get $ptr)))))))
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
                (i32.add (local.get $depth) (i32.const {one}))
                (local.get $replacer)
                (call $json_array_index_key (local.get $i))
                (local.get $v)
                (i32.const {one})))
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
        (local.set $emitted_count (i32.const {zero}))
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
            (local.set $child_value
              (call $json_apply_replacer
                (local.get $replacer)
                (local.get $v)
                (local.get $key_raw)
                (i32.load (i32.add (local.get $entry_base) (i32.const {value_off})))))
            (if (i32.eq (local.get $child_value) (i32.const {undefined}))
              (then
                (local.set $i (i32.add (local.get $i) (i32.const {one})))
                (br $object_loop)))
            (if (i32.gt_u (local.get $emitted_count) (i32.const {zero}))
              (then
                (i32.store8
                  (i32.add (local.get $ptr) (local.get $out))
                  (i32.const {comma}))
                (local.set $out (i32.add (local.get $out) (i32.const {one})))))
            (if
              (i32.and
                (i32.gt_u (local.get $emitted_count) (i32.const {zero}))
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
            (if
              (i32.and
                (i32.eqz (local.get $emitted_count))
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
            (local.set $child_len
              (call $json_write_escaped_string
                (local.get $key_raw)
                (i32.add (local.get $ptr) (local.get $out))))
            (local.set $out (i32.add (local.get $out) (local.get $child_len)))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {colon}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (if (i32.gt_u (local.get $gap) (i32.const {zero}))
              (then
                (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {space_char}))
                (local.set $out (i32.add (local.get $out) (i32.const {one})))))
            (local.set $child_len
              (call $json_stringify_into
                (local.get $child_value)
                (i32.add (local.get $ptr) (local.get $out))
                (local.get $gap)
                (local.get $gap_ptr)
                (i32.add (local.get $depth) (i32.const {one}))
                (local.get $replacer)
                (local.get $key_raw)
                (local.get $v)
                (i32.const {zero})))
            (if (i32.lt_s (local.get $child_len) (i32.const {zero}))
              (then
                (local.set $i (i32.add (local.get $i) (i32.const {one})))
                (br $object_loop)))
            (local.set $out (i32.add (local.get $out) (local.get $child_len)))
            (local.set $emitted_count (i32.add (local.get $emitted_count) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $object_loop)))
        (if
          (i32.and
            (i32.gt_u (local.get $emitted_count) (i32.const {zero}))
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
            root_holder_size = Layout::OBJECT_HEADER_SIZE + Layout::OBJECT_ENTRY_SIZE,
            index_key_alloc_size = Layout::STRING_HEADER_SIZE + 16,
            array_header = Layout::ARRAY_HEADER_SIZE,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
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
            empty_string = self.string_value(""),
            heap_number_sentinel = -1,
            unsupported = -1,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            max_gap = 10,
            newline = 10,
            space_char = 32,
            backspace = 8,
            form_feed = 12,
            carriage_return = 13,
            tab = 9,
            backslash = 92,
            letter_b = 98,
            letter_f = 102,
            letter_n = 110,
            letter_r = 114,
            letter_t = 116,
            quote = 34,
            colon = 58,
            comma = 44,
            lbrace = 123,
            rbrace = 125,
            lbracket = 91,
            rbracket = 93,
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

    /// Emit $is_nan global function.
    /// isNaN(x): returns true if ToNumber(x) is NaN, false otherwise.
    pub(super) fn emit_is_nan(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $is_nan (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    ;; If number: return false (all our integers are valid)
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (i32.const {false_tag}))))
    ;; If undefined: return true (ToNumber(undefined) = NaN)
    (if (i32.eq (local.get $v) (i32.const {undefined}))
      (then (return (i32.const {true_tag}))))
    ;; If string: try to parse; if parse fails, return true (NaN)
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then
        (return (call $is_nan_string (local.get $v)))))
    ;; null -> 0 (finite), boolean -> 0/1 (finite): return false
    (i32.const {false_tag}))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            undefined = ValueTag::UNDEFINED,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
        ));
        wat.push_str(&format!(
            r##"
  (func $is_nan_string (param $v i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $seen_digit i32)
    (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    ;; Skip leading whitespace
    (block $ws_done
      (loop $ws_loop
        (br_if $ws_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {space}))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one}))) (br $ws_loop)))
        (br $ws_done)))
    ;; Optional sign
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.or (i32.eq (local.get $ch) (i32.const {plus})) (i32.eq (local.get $ch) (i32.const {minus})))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one})))))))
    ;; Check for at least one digit
    (block $digit_check
      (loop $digit_loop
        (br_if $digit_check (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_zero})) (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then
            (local.set $seen_digit (i32.const {one}))
            (br $digit_check)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $digit_loop)))
    (if (result i32) (local.get $seen_digit)
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"##,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            space = 32,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// Emit $parse_int global function.
    pub(super) fn emit_parse_int(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $parse_int (param $s i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $s) (i32.const {tag_mask})))
    ;; If number: return the number value
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (local.get $s))))
    ;; If not a string, return NaN (0 in our model)
    (if (i32.ne (local.get $tag) (i32.const {string_tag}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    ;; Parse string to integer with auto-detected radix
    (call $parse_int_string (local.get $s) (i32.const {zero})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            zero = RuntimeConst::ZERO,
            number_shift = ValueTag::NUMBER_SHIFT,
        ));
        wat.push_str(&format!(
            r##"
  (func $parse_int_string (param $s i32) (param $radix i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $sign i32)
    (local $n i32)
    (local $r i32)
    (local $digit i32)
    (local $seen i32)
    (local.set $base (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (local.set $sign (i32.const {one}))
    ;; 1. Skip leading whitespace
    (block $ws_done
      (loop $ws_loop
        (br_if $ws_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {space}))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one}))) (br $ws_loop)))
        (br $ws_done)))
    ;; 2. Handle sign
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {minus}))
          (then
            (local.set $sign (i32.const -1))
            (local.set $i (i32.add (local.get $i) (i32.const {one}))))
          (if (i32.eq (local.get $ch) (i32.const {plus}))
            (then (local.set $i (i32.add (local.get $i) (i32.const {one}))))))))
    ;; 3. Determine radix
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        ;; Check for 0x prefix
        (if (i32.eq (local.get $ch) (i32.const {ascii_zero}))
          (then
            (if (i32.lt_u (i32.add (local.get $i) (i32.const {one})) (local.get $len))
              (then
                (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (i32.add (local.get $i) (i32.const {one}))))))
                (if (i32.or (i32.eq (local.get $ch) (i32.const {ascii_lower_x})) (i32.eq (local.get $ch) (i32.const {ascii_upper_x})))
                  (then
                    (local.set $r (i32.const 16))
                    (local.set $i (i32.add (local.get $i) (i32.const {two})))))))))))
    (if (i32.eq (local.get $r) (i32.const {zero}))
      (then
        (if (i32.eq (i32.and (local.get $radix) (i32.const {tag_mask})) (i32.const {number_tag}))
          (then
            (local.set $r (i32.shr_s (local.get $radix) (i32.const {number_shift})))))
        ;; If radix is 0 or undefined (0 tagged), detect automatically
        (if (i32.eqz (local.get $r))
          (then (local.set $r (i32.const 10))))))
    ;; Clamp radix
    (if (i32.lt_s (local.get $r) (i32.const 2))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.gt_s (local.get $r) (i32.const 36))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    ;; 4. Parse digits
    (block $parse_done
      (loop $parse_loop
        (br_if $parse_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        ;; Convert char to digit
        (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_zero})) (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then (local.set $digit (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
          (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_lower_a})) (i32.le_u (local.get $ch) (i32.const {ascii_lower_f})))
            (then (local.set $digit (i32.add (i32.sub (local.get $ch) (i32.const {ascii_lower_a})) (i32.const 10))))
            (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_upper_a})) (i32.le_u (local.get $ch) (i32.const {ascii_upper_f})))
              (then (local.set $digit (i32.add (i32.sub (local.get $ch) (i32.const {ascii_upper_a})) (i32.const 10))))
              (br $parse_done))))
        ;; Check digit is valid for radix
        (if (i32.ge_u (local.get $digit) (local.get $r))
          (then (br $parse_done)))
        (local.set $n (i32.add (i32.mul (local.get $n) (local.get $r)) (local.get $digit)))
        (local.set $seen (i32.const {one}))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $parse_loop)))
    ;; If no digits seen, return NaN (modeled as tagged 0)
    (if (i32.eqz (local.get $seen))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or (i32.shl (local.get $n) (i32.const {number_shift})) (i32.const {number_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            two = 2,
            space = 32,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            ascii_lower_x = 120,
            ascii_upper_x = 88,
            ascii_lower_a = 97,
            ascii_lower_f = 102,
            ascii_upper_a = 65,
            ascii_upper_f = 70,
        ));
    }

    /// Emit $parse_float global function.
    pub(super) fn emit_parse_float(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $parse_float (param $s i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $s) (i32.const {tag_mask})))
    ;; If number: return as-is
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (local.get $s))))
    ;; If not a string, return NaN (modeled as tagged 0)
    (if (i32.ne (local.get $tag) (i32.const {string_tag}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    ;; Parse string to float (integer part only in our model)
    (call $parse_float_string (local.get $s)))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
        ));
        wat.push_str(&format!(
            r##"
  (func $parse_float_string (param $s i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $sign i32)
    (local $n i32)
    (local $seen i32)
    (local.set $base (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (local.set $sign (i32.const {one}))
    ;; Skip leading whitespace
    (block $ws_done
      (loop $ws_loop
        (br_if $ws_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {space}))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one}))) (br $ws_loop)))
        (br $ws_done)))
    ;; Handle sign
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {minus}))
          (then
            (local.set $sign (i32.const -1))
            (local.set $i (i32.add (local.get $i) (i32.const {one}))))
          (if (i32.eq (local.get $ch) (i32.const {plus}))
            (then (local.set $i (i32.add (local.get $i) (i32.const {one}))))))))
    ;; Parse integer part
    (block $int_done
      (loop $int_loop
        (br_if $int_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_zero})) (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then
            (local.set $n (i32.add (i32.mul (local.get $n) (i32.const 10)) (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
            (local.set $seen (i32.const {one}))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $int_loop))
          (br $int_done))))
    ;; If no digits seen, return NaN (modeled as tagged 0)
    (if (i32.eqz (local.get $seen))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or (i32.shl (local.get $n) (i32.const {number_shift})) (i32.const {number_tag})))
"##,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            space = 32,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    /// Emit $is_finite global function.
    pub(super) fn emit_is_finite(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $is_finite (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    ;; If number: return true (all our integers are finite)
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (i32.const {true_tag}))))
    ;; If undefined: return false (ToNumber(undefined) = NaN)
    (if (i32.eq (local.get $v) (i32.const {undefined}))
      (then (return (i32.const {false_tag}))))
    ;; If string: try to parse; if parse succeeds and is finite, return true
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then
        (return (call $is_finite_string (local.get $v)))))
    ;; null -> 0 (finite), boolean -> 0/1 (finite): return true
    (i32.const {true_tag}))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            undefined = ValueTag::UNDEFINED,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
        wat.push_str(&format!(
            r##"
  (func $is_finite_string (param $v i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $seen_digit i32)
    (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    ;; Skip leading whitespace
    (block $ws_done
      (loop $ws_loop
        (br_if $ws_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {space}))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one}))) (br $ws_loop)))
        (br $ws_done)))
    ;; Optional sign
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.or (i32.eq (local.get $ch) (i32.const {plus})) (i32.eq (local.get $ch) (i32.const {minus})))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one})))))))
    ;; Check for at least one digit
    (block $digit_check
      (loop $digit_loop
        (br_if $digit_check (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_zero})) (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then
            (local.set $seen_digit (i32.const {one}))
            (br $digit_check)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $digit_loop)))
    (if (result i32) (local.get $seen_digit)
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"##,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            space = 32,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    /// Emit $boolean_coerce global function.
    /// Boolean(x): returns TRUE if truthy (not 0, not undefined, not null, not false, not empty string)
    pub(crate) fn emit_boolean_coerce(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $boolean_coerce (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    ;; false, undefined, null -> 0 -> false
    (if (i32.eq (local.get $v) (i32.const {false_val})) (then (return (i32.const {false_tag}))))
    (if (i32.eq (local.get $v) (i32.const {undefined})) (then (return (i32.const {false_tag}))))
    (if (i32.eq (local.get $v) (i32.const {null_val})) (then (return (i32.const {false_tag}))))
    ;; number 0 -> false
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then
        (if (i32.eqz (i32.shr_s (local.get $v) (i32.const {number_shift})))
          (then (return (i32.const {false_tag})))
          (else (return (i32.const {true_tag}))))))
    ;; empty string -> false (check string length)
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then
        (if (i32.eqz (i32.load (i32.and (local.get $v) (i32.const {heap_mask}))))
          (then (return (i32.const {false_tag})))
          (else (return (i32.const {true_tag}))))))
    ;; Everything else is truthy
    (i32.const {true_tag}))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            string_tag = ValueTag::STRING,
            heap_mask = ValueTag::HEAP_MASK,
            false_val = ValueTag::FALSE,
            undefined = ValueTag::UNDEFINED,
            null_val = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// Emit $number_coerce global function.
    /// Number(x): coerces to number value.
    pub(crate) fn emit_number_coerce(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_coerce (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    ;; Already a number: return as-is
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (local.get $v))))
    ;; Boolean: 0 or 1
    (if (i32.eq (local.get $v) (i32.const {false_val}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.eq (local.get $v) (i32.const {true_val}))
      (then (return (i32.or (i32.shl (i32.const {one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    ;; undefined -> 0 (NaN in spec, but 0 in our model)
    (if (i32.eq (local.get $v) (i32.const {undefined}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    ;; null -> 0
    (if (i32.eq (local.get $v) (i32.const {null_val}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    ;; String: parse integer
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then (return (call $parse_int_string (local.get $v) (i32.const {zero})))))
    ;; Default: 0
    (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            string_tag = ValueTag::STRING,
            false_val = ValueTag::FALSE,
            true_val = ValueTag::TRUE,
            undefined = ValueTag::UNDEFINED,
            null_val = ValueTag::NULL,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    /// Emit $number_is_nan — wrapper around $is_nan for Number.isNaN().
    /// Number.isNaN(x): same semantics as global isNaN(x).
    pub(super) fn emit_number_is_nan(&self, wat: &mut String) {
        wat.push_str(
            r##"
  (func $number_is_nan (param $v i32) (result i32)
    (return (call $is_nan (local.get $v))))
"##,
        );
    }

    /// Emit $number_is_finite — wrapper around $is_finite for Number.isFinite().
    /// Number.isFinite(x): same semantics as global isFinite(x).
    pub(super) fn emit_number_is_finite(&self, wat: &mut String) {
        wat.push_str(
            r##"
  (func $number_is_finite (param $v i32) (result i32)
    (return (call $is_finite (local.get $v))))
"##,
        );
    }

    /// Emit $number_is_integer.
    /// Number.isInteger(x): returns true if x is a number and is an integer.
    pub(super) fn emit_number_is_integer(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_is_integer (param $v i32) (result i32)
    ;; Number.isInteger returns true only if value is a number tag.
    ;; All our numbers are integers (in the tagged model), so check tag.
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then (return (i32.const {true_tag}))))
    (return (i32.const {false_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    /// Emit $number_is_safe_integer.
    /// Number.isSafeInteger(x): returns true if x is a number and is a safe integer.
    pub(super) fn emit_number_is_safe_integer(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_is_safe_integer (param $v i32) (result i32)
    ;; All our numbers are integers in the tagged model.
    ;; MAX_SAFE_INTEGER is 2^53-1 which is larger than our shifted range.
    ;; So all tagged numbers are "safe" in our model.
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then (return (i32.const {true_tag}))))
    (return (i32.const {false_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }
}
