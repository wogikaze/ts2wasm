use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Emit a shared helper function used by both $regexp_test and $regexp_match.
    /// Checks whether a single input byte matches a pattern atom (literal, dot,
    /// digit, word, whitespace).  Returns 1 (match) or 0 (no match).
    pub(super) fn emit_regexp_match_inner(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $regexp_match_inner (param $kind i32) (param $lit_val i32) (param $byte i32) (result i32)
    (if (i32.eq (local.get $kind) (i32.const 0))
      (then (return (i32.eq (local.get $lit_val) (local.get $byte)))))
    (if (i32.eq (local.get $kind) (i32.const 1))
      (then (return (i32.ne (local.get $byte) (i32.const 0x0A)))))
    (if (i32.eq (local.get $kind) (i32.const 2))
      (then (return
        (i32.and
          (i32.ge_u (local.get $byte) (i32.const 0x30))
          (i32.le_u (local.get $byte) (i32.const 0x39))))))
    (if (i32.eq (local.get $kind) (i32.const 3))
      (then (return
        (i32.or
          (i32.and
            (i32.ge_u (local.get $byte) (i32.const 0x30))
            (i32.le_u (local.get $byte) (i32.const 0x39)))
          (i32.or
            (i32.and
              (i32.ge_u (local.get $byte) (i32.const 0x41))
              (i32.le_u (local.get $byte) (i32.const 0x5A)))
            (i32.or
              (i32.and
                (i32.ge_u (local.get $byte) (i32.const 0x61))
                (i32.le_u (local.get $byte) (i32.const 0x7A)))
              (i32.eq (local.get $byte) (i32.const 0x5F))))))))
    (return
      (i32.or
        (i32.eq (local.get $byte) (i32.const 0x20))
        (i32.or
          (i32.eq (local.get $byte) (i32.const 0x09))
          (i32.or
            (i32.eq (local.get $byte) (i32.const 0x0A))
            (i32.or
              (i32.eq (local.get $byte) (i32.const 0x0D))
              (i32.eq (local.get $byte) (i32.const 0x0C))))))))
"#,
        );
    }

    pub(super) fn emit_regexp_test(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $regexp_test (param $pattern i32) (param $input i32) (result i32)
    (local $p_obj i32)
    (local $i_obj i32)
    (local $p_len i32)
    (local $i_len i32)
    (local $delimiter i32)
    (local $pos i32)
    (local $p i32)
    (local $i i32)
    (local $ch i32)
    (local $kind i32)
    (local $lit_val i32)
    (local $quant i32)
    (local $p_header i32)
    (local $i_header i32)
    (if (i32.eqz (call $is_string (local.get $pattern))) (then (return (i32.const {false_tag}))))
    (if (i32.eqz (call $is_string (local.get $input))) (then (return (i32.const {false_tag}))))
    (local.set $p_obj (i32.and (local.get $pattern) (i32.const {heap_mask})))
    (local.set $i_obj (i32.and (local.get $input) (i32.const {heap_mask})))
    (local.set $p_header (i32.add (local.get $p_obj) (i32.const {header})))
    (local.set $i_header (i32.add (local.get $i_obj) (i32.const {header})))
    (local.set $p_len (i32.load (local.get $p_obj)))
    (local.set $i_len (i32.load (local.get $i_obj)))
    (if (i32.lt_u (local.get $p_len) (i32.const 2)) (then (return (i32.const {false_tag}))))
    (if
      (i32.ne
        (i32.load8_u (local.get $p_header))
        (i32.const {slash}))
      (then (return (i32.const {false_tag}))))
    (local.set $delimiter (i32.sub (local.get $p_len) (i32.const {one})))
    (block $delimiter_found
      (loop $find_delimiter
        (br_if $delimiter_found
          (i32.eq
            (i32.load8_u
              (i32.add (local.get $p_header) (local.get $delimiter)))
            (i32.const {slash})))
        (if (i32.eqz (local.get $delimiter)) (then (return (i32.const {false_tag}))))
        (local.set $delimiter (i32.sub (local.get $delimiter) (i32.const {one})))
        (br $find_delimiter)))
    (if (i32.eq (local.get $delimiter) (i32.const {one}))
      (then (return (i32.const {true_tag}))))
    (block $not_found
      (loop $search
        (br_if $not_found
          (i32.gt_u (local.get $pos) (local.get $i_len)))
        (local.set $p (i32.const {one}))
        (local.set $i (local.get $pos))
        (block $match_fail
          (loop $match_loop
            (if (i32.ge_u (local.get $p) (local.get $delimiter))
              (then (return (i32.const {true_tag}))))
            (local.set $ch
              (i32.load8_u
                (i32.add (local.get $p_header) (local.get $p))))
            (local.set $p (i32.add (local.get $p) (i32.const {one})))
            (if (i32.eq (local.get $ch) (i32.const 0x2E))
              (then
                (local.set $kind (i32.const 1))
                (local.set $lit_val (i32.const 0)))
              (else
                (if (i32.eq (local.get $ch) (i32.const 0x5C))
                  (then
                    (local.set $ch
                      (i32.load8_u
                        (i32.add (local.get $p_header) (local.get $p))))
                    (local.set $p (i32.add (local.get $p) (i32.const {one})))
                    (if (i32.eq (local.get $ch) (i32.const 0x64))
                      (then (local.set $kind (i32.const 2)))
                      (else
                        (if (i32.eq (local.get $ch) (i32.const 0x77))
                          (then (local.set $kind (i32.const 3)))
                          (else
                            (local.set $kind (i32.const 4))))))
                    (local.set $lit_val (i32.const 0)))
                  (else
                    (local.set $kind (i32.const 0))
                    (local.set $lit_val (local.get $ch))))))
            (local.set $quant (i32.const 0))
            (if (i32.lt_u (local.get $p) (local.get $delimiter))
              (then
                (local.set $ch
                  (i32.load8_u
                    (i32.add (local.get $p_header) (local.get $p))))
                (if (i32.eq (local.get $ch) (i32.const 0x2B))
                  (then
                    (local.set $quant (i32.const 1))
                    (local.set $p (i32.add (local.get $p) (i32.const {one}))))
                  (else
                    (if (i32.eq (local.get $ch) (i32.const 0x2A))
                      (then
                        (local.set $quant (i32.const 2))
                        (local.set $p (i32.add (local.get $p) (i32.const {one}))))
                      (else
                        (if (i32.eq (local.get $ch) (i32.const 0x3F))
                          (then
                            (local.set $quant (i32.const 3))
                            (local.set $p (i32.add (local.get $p) (i32.const {one})))))))))))
            (if (i32.ge_u (local.get $i) (local.get $i_len))
              (then
                (if (i32.eq (local.get $quant) (i32.const 0))
                  (then (br $match_fail)))
                (if (i32.eq (local.get $quant) (i32.const 1))
                  (then (br $match_fail)))
                (if (i32.ge_u (local.get $p) (local.get $delimiter))
                  (then (return (i32.const {true_tag}))))
                (br $match_loop)))
            (if (i32.eq (local.get $quant) (i32.const 0))
              (then
                (local.set $ch
                  (i32.load8_u
                    (i32.add (local.get $i_header) (local.get $i))))
                (local.set $ch
                  (call $regexp_match_inner
                    (local.get $kind) (local.get $lit_val) (local.get $ch)))
                (if (i32.eqz (local.get $ch))
                  (then (br $match_fail)))
                (local.set $i (i32.add (local.get $i) (i32.const {one}))))
              (else
                (if (i32.eq (local.get $quant) (i32.const 3))
                  (then
                    (if (i32.lt_u (local.get $i) (local.get $i_len))
                      (then
                        (local.set $ch
                          (i32.load8_u
                            (i32.add (local.get $i_header) (local.get $i))))
                        (local.set $ch
                          (call $regexp_match_inner
                            (local.get $kind) (local.get $lit_val) (local.get $ch)))
                        (if (i32.ne (local.get $ch) (i32.const 0))
                          (then
                            (local.set $i (i32.add (local.get $i) (i32.const {one}))))))))
                  (else
                    (if (i32.eq (local.get $quant) (i32.const 1))
                      (then
                        (local.set $ch
                          (i32.load8_u
                            (i32.add (local.get $i_header) (local.get $i))))
                        (local.set $ch
                          (call $regexp_match_inner
                            (local.get $kind) (local.get $lit_val) (local.get $ch)))
                        (if (i32.eqz (local.get $ch))
                          (then (br $match_fail)))
                        (local.set $i (i32.add (local.get $i) (i32.const {one})))
                        (block $plus_loop_exit
                          (loop $plus_loop
                            (if (i32.ge_u (local.get $i) (local.get $i_len))
                              (then (br $plus_loop_exit)))
                            (local.set $ch
                              (i32.load8_u
                                (i32.add (local.get $i_header) (local.get $i))))
                            (local.set $ch
                              (call $regexp_match_inner
                                (local.get $kind) (local.get $lit_val) (local.get $ch)))
                            (if (i32.eqz (local.get $ch))
                              (then (br $plus_loop_exit)))
                            (local.set $i (i32.add (local.get $i) (i32.const {one})))
                            (br $plus_loop))))
                      (else
                        (block $star_loop_exit
                          (loop $star_loop
                            (if (i32.ge_u (local.get $i) (local.get $i_len))
                              (then (br $star_loop_exit)))
                            (local.set $ch
                              (i32.load8_u
                                (i32.add (local.get $i_header) (local.get $i))))
                            (local.set $ch
                              (call $regexp_match_inner
                                (local.get $kind) (local.get $lit_val) (local.get $ch)))
                            (if (i32.eqz (local.get $ch))
                              (then (br $star_loop_exit)))
                            (local.set $i (i32.add (local.get $i) (i32.const {one})))
                            (br $star_loop)))))))))
            (br $match_loop)))
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
    (local $pos i32)
    (local $p i32)
    (local $i i32)
    (local $ch i32)
    (local $kind i32)
    (local $lit_val i32)
    (local $quant i32)
    (local $p_header i32)
    (local $i_header i32)
    (if (i32.eqz (call $is_string (local.get $pattern))) (then (return (i32.const {null_tag}))))
    (if (i32.eqz (call $is_string (local.get $input))) (then (return (i32.const {null_tag}))))
    (local.set $p_obj (i32.and (local.get $pattern) (i32.const {heap_mask})))
    (local.set $i_obj (i32.and (local.get $input) (i32.const {heap_mask})))
    (local.set $p_header (i32.add (local.get $p_obj) (i32.const {header})))
    (local.set $i_header (i32.add (local.get $i_obj) (i32.const {header})))
    (local.set $p_len (i32.load (local.get $p_obj)))
    (local.set $i_len (i32.load (local.get $i_obj)))
    (if (i32.lt_u (local.get $p_len) (i32.const 2)) (then (return (i32.const {null_tag}))))
    (if
      (i32.ne
        (i32.load8_u (local.get $p_header))
        (i32.const {slash}))
      (then (return (i32.const {null_tag}))))
    (local.set $delimiter (i32.sub (local.get $p_len) (i32.const {one})))
    (block $delimiter_found
      (loop $find_delimiter
        (br_if $delimiter_found
          (i32.eq
            (i32.load8_u
              (i32.add (local.get $p_header) (local.get $delimiter)))
            (i32.const {slash})))
        (if (i32.eqz (local.get $delimiter)) (then (return (i32.const {null_tag}))))
        (local.set $delimiter (i32.sub (local.get $delimiter) (i32.const {one})))
        (br $find_delimiter)))
    (block $not_found
      (loop $search
        (br_if $not_found
          (i32.gt_u (local.get $pos) (local.get $i_len)))
        (local.set $p (i32.const {one}))
        (local.set $i (local.get $pos))
        (block $match_fail
          (loop $match_loop
            (if (i32.ge_u (local.get $p) (local.get $delimiter))
              (then
                (return
                  (call $string_substring
                    (local.get $input)
                    (i32.or
                      (i32.shl (local.get $pos) (i32.const {number_shift}))
                      (i32.const {number_tag}))
                    (i32.or
                      (i32.shl (local.get $i) (i32.const {number_shift}))
                      (i32.const {number_tag}))))))
            (local.set $ch
              (i32.load8_u
                (i32.add (local.get $p_header) (local.get $p))))
            (local.set $p (i32.add (local.get $p) (i32.const {one})))
            (if (i32.eq (local.get $ch) (i32.const 0x2E))
              (then
                (local.set $kind (i32.const 1))
                (local.set $lit_val (i32.const 0)))
              (else
                (if (i32.eq (local.get $ch) (i32.const 0x5C))
                  (then
                    (local.set $ch
                      (i32.load8_u
                        (i32.add (local.get $p_header) (local.get $p))))
                    (local.set $p (i32.add (local.get $p) (i32.const {one})))
                    (if (i32.eq (local.get $ch) (i32.const 0x64))
                      (then (local.set $kind (i32.const 2)))
                      (else
                        (if (i32.eq (local.get $ch) (i32.const 0x77))
                          (then (local.set $kind (i32.const 3)))
                          (else
                            (local.set $kind (i32.const 4))))))
                    (local.set $lit_val (i32.const 0)))
                  (else
                    (local.set $kind (i32.const 0))
                    (local.set $lit_val (local.get $ch))))))
            (local.set $quant (i32.const 0))
            (if (i32.lt_u (local.get $p) (local.get $delimiter))
              (then
                (local.set $ch
                  (i32.load8_u
                    (i32.add (local.get $p_header) (local.get $p))))
                (if (i32.eq (local.get $ch) (i32.const 0x2B))
                  (then
                    (local.set $quant (i32.const 1))
                    (local.set $p (i32.add (local.get $p) (i32.const {one}))))
                  (else
                    (if (i32.eq (local.get $ch) (i32.const 0x2A))
                      (then
                        (local.set $quant (i32.const 2))
                        (local.set $p (i32.add (local.get $p) (i32.const {one}))))
                      (else
                        (if (i32.eq (local.get $ch) (i32.const 0x3F))
                          (then
                            (local.set $quant (i32.const 3))
                            (local.set $p (i32.add (local.get $p) (i32.const {one})))))))))))
            (if (i32.ge_u (local.get $i) (local.get $i_len))
              (then
                (if (i32.eq (local.get $quant) (i32.const 0))
                  (then (br $match_fail)))
                (if (i32.eq (local.get $quant) (i32.const 1))
                  (then (br $match_fail)))
                (if (i32.ge_u (local.get $p) (local.get $delimiter))
                  (then
                    (return
                      (call $string_substring
                        (local.get $input)
                        (i32.or
                          (i32.shl (local.get $pos) (i32.const {number_shift}))
                          (i32.const {number_tag}))
                        (i32.or
                          (i32.shl (local.get $i) (i32.const {number_shift}))
                          (i32.const {number_tag}))))))
                (br $match_loop)))
            (if (i32.eq (local.get $quant) (i32.const 0))
              (then
                (local.set $ch
                  (i32.load8_u
                    (i32.add (local.get $i_header) (local.get $i))))
                (local.set $ch
                  (call $regexp_match_inner
                    (local.get $kind) (local.get $lit_val) (local.get $ch)))
                (if (i32.eqz (local.get $ch))
                  (then (br $match_fail)))
                (local.set $i (i32.add (local.get $i) (i32.const {one}))))
              (else
                (if (i32.eq (local.get $quant) (i32.const 3))
                  (then
                    (if (i32.lt_u (local.get $i) (local.get $i_len))
                      (then
                        (local.set $ch
                          (i32.load8_u
                            (i32.add (local.get $i_header) (local.get $i))))
                        (local.set $ch
                          (call $regexp_match_inner
                            (local.get $kind) (local.get $lit_val) (local.get $ch)))
                        (if (i32.ne (local.get $ch) (i32.const 0))
                          (then
                            (local.set $i (i32.add (local.get $i) (i32.const {one}))))))))
                  (else
                    (if (i32.eq (local.get $quant) (i32.const 1))
                      (then
                        (local.set $ch
                          (i32.load8_u
                            (i32.add (local.get $i_header) (local.get $i))))
                        (local.set $ch
                          (call $regexp_match_inner
                            (local.get $kind) (local.get $lit_val) (local.get $ch)))
                        (if (i32.eqz (local.get $ch))
                          (then (br $match_fail)))
                        (local.set $i (i32.add (local.get $i) (i32.const {one})))
                        (block $plus_loop_exit
                          (loop $plus_loop
                            (if (i32.ge_u (local.get $i) (local.get $i_len))
                              (then (br $plus_loop_exit)))
                            (local.set $ch
                              (i32.load8_u
                                (i32.add (local.get $i_header) (local.get $i))))
                            (local.set $ch
                              (call $regexp_match_inner
                                (local.get $kind) (local.get $lit_val) (local.get $ch)))
                            (if (i32.eqz (local.get $ch))
                              (then (br $plus_loop_exit)))
                            (local.set $i (i32.add (local.get $i) (i32.const {one})))
                            (br $plus_loop))))
                      (else
                        (block $star_loop_exit
                          (loop $star_loop
                            (if (i32.ge_u (local.get $i) (local.get $i_len))
                              (then (br $star_loop_exit)))
                            (local.set $ch
                              (i32.load8_u
                                (i32.add (local.get $i_header) (local.get $i))))
                            (local.set $ch
                              (call $regexp_match_inner
                                (local.get $kind) (local.get $lit_val) (local.get $ch)))
                            (if (i32.eqz (local.get $ch))
                              (then (br $star_loop_exit)))
                            (local.set $i (i32.add (local.get $i) (i32.const {one})))
                            (br $star_loop)))))))))
            (br $match_loop)))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $search)))
    (i32.const {null_tag}))
"#,
            null_tag = ValueTag::NULL,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            slash = b'/' as i32,
            one = RuntimeConst::ONE,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }
}
