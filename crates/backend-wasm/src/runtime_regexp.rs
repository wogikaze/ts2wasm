use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_regexp_test(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $regexp_test (param $pattern i32) (param $input i32) (result i32)
    (local $p_obj i32)
    (local $i_obj i32)
    (local $p_len i32)
    (local $i_len i32)
    (local $delimiter i32)
    (local $pattern_len i32)
    (local $pos i32)
    (if (i32.eqz (call $is_string (local.get $pattern))) (then (return (i32.const {false_tag}))))
    (if (i32.eqz (call $is_string (local.get $input))) (then (return (i32.const {false_tag}))))
    (local.set $p_obj (i32.and (local.get $pattern) (i32.const {heap_mask})))
    (local.set $i_obj (i32.and (local.get $input) (i32.const {heap_mask})))
    (local.set $p_len (i32.load (local.get $p_obj)))
    (local.set $i_len (i32.load (local.get $i_obj)))
    (if (i32.lt_u (local.get $p_len) (i32.const 2)) (then (return (i32.const {false_tag}))))
    (if
      (i32.ne
        (i32.load8_u (i32.add (local.get $p_obj) (i32.const {header})))
        (i32.const {slash}))
      (then (return (i32.const {false_tag}))))
    (local.set $delimiter (i32.sub (local.get $p_len) (i32.const {one})))
    (block $delimiter_found
      (loop $find_delimiter
        (br_if $delimiter_found
          (i32.eq
            (i32.load8_u
              (i32.add
                (i32.add (local.get $p_obj) (i32.const {header}))
                (local.get $delimiter)))
            (i32.const {slash})))
        (if (i32.eqz (local.get $delimiter)) (then (return (i32.const {false_tag}))))
        (local.set $delimiter (i32.sub (local.get $delimiter) (i32.const {one})))
        (br $find_delimiter)))
    (local.set $pattern_len (i32.sub (local.get $delimiter) (i32.const {one})))
    (if (i32.eqz (local.get $pattern_len)) (then (return (i32.const {true_tag}))))
    (if (i32.gt_u (local.get $pattern_len) (local.get $i_len)) (then (return (i32.const {false_tag}))))
    (block $not_found
      (loop $search
        (br_if $not_found
          (i32.gt_u (local.get $pos) (i32.sub (local.get $i_len) (local.get $pattern_len))))
        (if
          (call $mem_equal
            (i32.add
              (i32.add (local.get $i_obj) (i32.const {header}))
              (local.get $pos))
            (i32.add (i32.add (local.get $p_obj) (i32.const {header})) (i32.const {one}))
            (local.get $pattern_len))
          (then (return (i32.const {true_tag}))))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $search)))
    (i32.const {false_tag}))
"#,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            slash = b'/' as i32,
            one = RuntimeConst::ONE,
        ));
    }

    pub(super) fn emit_regexp_match(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $regexp_match (param $pattern i32) (param $input i32) (result i32)
    (local $p_obj i32)
    (local $i_obj i32)
    (local $p_len i32)
    (local $i_len i32)
    (local $delimiter i32)
    (local $pattern_len i32)
    (local $pos i32)
    (if (i32.eqz (call $is_string (local.get $pattern))) (then (return (i32.const {null_tag}))))
    (if (i32.eqz (call $is_string (local.get $input))) (then (return (i32.const {null_tag}))))
    (local.set $p_obj (i32.and (local.get $pattern) (i32.const {heap_mask})))
    (local.set $i_obj (i32.and (local.get $input) (i32.const {heap_mask})))
    (local.set $p_len (i32.load (local.get $p_obj)))
    (local.set $i_len (i32.load (local.get $i_obj)))
    (if (i32.lt_u (local.get $p_len) (i32.const 2)) (then (return (i32.const {null_tag}))))
    (if
      (i32.ne
        (i32.load8_u (i32.add (local.get $p_obj) (i32.const {header})))
        (i32.const {slash}))
      (then (return (i32.const {null_tag}))))
    (local.set $delimiter (i32.sub (local.get $p_len) (i32.const {one})))
    (block $delimiter_found
      (loop $find_delimiter
        (br_if $delimiter_found
          (i32.eq
            (i32.load8_u
              (i32.add
                (i32.add (local.get $p_obj) (i32.const {header}))
                (local.get $delimiter)))
            (i32.const {slash})))
        (if (i32.eqz (local.get $delimiter)) (then (return (i32.const {null_tag}))))
        (local.set $delimiter (i32.sub (local.get $delimiter) (i32.const {one})))
        (br $find_delimiter)))
    (local.set $pattern_len (i32.sub (local.get $delimiter) (i32.const {one})))
    (if (i32.eqz (local.get $pattern_len))
      (then
        (return
          (call $string_substring
            (local.get $input)
            (i32.const {number_zero})
            (i32.const {number_zero})))))
    (if (i32.gt_u (local.get $pattern_len) (local.get $i_len)) (then (return (i32.const {null_tag}))))
    (block $not_found
      (loop $search
        (br_if $not_found
          (i32.gt_u (local.get $pos) (i32.sub (local.get $i_len) (local.get $pattern_len))))
        (if
          (call $mem_equal
            (i32.add
              (i32.add (local.get $i_obj) (i32.const {header}))
              (local.get $pos))
            (i32.add (i32.add (local.get $p_obj) (i32.const {header})) (i32.const {one}))
            (local.get $pattern_len))
          (then
            (return
              (call $string_substring
                (local.get $input)
                (i32.or
                  (i32.shl (local.get $pos) (i32.const {number_shift}))
                  (i32.const {number_tag}))
                (i32.or
                  (i32.shl
                    (i32.add (local.get $pos) (local.get $pattern_len))
                    (i32.const {number_shift}))
                  (i32.const {number_tag}))))))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $search)))
    (i32.const {null_tag}))
"#,
            null_tag = ValueTag::NULL,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            slash = b'/' as i32,
            one = RuntimeConst::ONE,
            number_zero = ValueTag::encode_number(0),
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }
}
