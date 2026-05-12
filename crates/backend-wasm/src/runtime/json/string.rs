use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Emit the $json_parse_string WAT function.
    pub(crate) fn emit_json_parse_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_parse_string (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $start i32)
    (local $out_len i32)
    (local $out_pos i32)
    (local $result_ptr i32)
    (local $ch i32)
    (local $store_ch i32)
    (local $code i32)
    (local $low_code i32)
    (local $unicode_advance i32)
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
        (if (i32.lt_u (local.get $ch) (i32.const {space}))
          (then (return (i32.const {undefined}))))
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
                (local.set $code
                  (call $json_parse_unicode_escape_code_unit
                    (local.get $obj)
                    (local.get $len)
                    (i32.add (local.get $pos) (i32.const {one}))))
                (if
                  (i32.lt_s (local.get $code) (i32.const {zero}))
                  (then (return (i32.const {undefined}))))
                (local.set $unicode_advance (i32.const 5))
                (if
                  (i32.and
                    (i32.ge_u (local.get $code) (i32.const {high_surrogate_start}))
                    (i32.le_u (local.get $code) (i32.const {high_surrogate_end})))
                  (then
                    (if
                      (i32.and
                        (i32.le_u (i32.add (local.get $pos) (i32.const 11)) (local.get $len))
                        (i32.and
                          (i32.eq
                            (i32.load8_u
                              (i32.add
                                (i32.add (local.get $obj) (i32.const {str_header}))
                                (i32.add (local.get $pos) (i32.const 5))))
                            (i32.const {backslash}))
                          (i32.eq
                            (i32.load8_u
                              (i32.add
                                (i32.add (local.get $obj) (i32.const {str_header}))
                                (i32.add (local.get $pos) (i32.const 6))))
                            (i32.const {ascii_u}))))
                      (then
                        (local.set $low_code
                          (call $json_parse_unicode_escape_code_unit
                            (local.get $obj)
                            (local.get $len)
                            (i32.add (local.get $pos) (i32.const 7))))
                        (if
                          (i32.and
                            (i32.ge_u (local.get $low_code) (i32.const {low_surrogate_start}))
                            (i32.le_u (local.get $low_code) (i32.const {low_surrogate_end})))
                          (then
                            (local.set $code
                              (i32.add
                                (i32.const 65536)
                                (i32.add
                                  (i32.shl
                                    (i32.sub (local.get $code) (i32.const {high_surrogate_start}))
                                    (i32.const 10))
                                  (i32.sub (local.get $low_code) (i32.const {low_surrogate_start})))))
                            (local.set $unicode_advance (i32.const 11))))))
                    (if (i32.eq (local.get $unicode_advance) (i32.const 5))
                      (then (local.set $code (i32.const {replacement_char}))))))
                (if
                  (i32.and
                    (i32.ge_u (local.get $code) (i32.const {low_surrogate_start}))
                    (i32.le_u (local.get $code) (i32.const {low_surrogate_end})))
                  (then (local.set $code (i32.const {replacement_char}))))
                (local.set $out_len
                  (i32.add
                    (local.get $out_len)
                    (call $json_utf8_len (local.get $code))))
                (local.set $pos (i32.add (local.get $pos) (local.get $unicode_advance)))
                (br $scan))
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
                (local.set $code
                  (call $json_parse_unicode_escape_code_unit
                    (local.get $obj)
                    (local.get $len)
                    (i32.add (local.get $pos) (i32.const {one}))))
                (if (i32.lt_s (local.get $code) (i32.const {zero}))
                  (then (return (i32.const {undefined}))))
                (local.set $unicode_advance (i32.const 5))
                (if
                  (i32.and
                    (i32.ge_u (local.get $code) (i32.const {high_surrogate_start}))
                    (i32.le_u (local.get $code) (i32.const {high_surrogate_end})))
                  (then
                    (if
                      (i32.and
                        (i32.le_u (i32.add (local.get $pos) (i32.const 11)) (local.get $len))
                        (i32.and
                          (i32.eq
                            (i32.load8_u
                              (i32.add
                                (i32.add (local.get $obj) (i32.const {str_header}))
                                (i32.add (local.get $pos) (i32.const 5))))
                            (i32.const {backslash}))
                          (i32.eq
                            (i32.load8_u
                              (i32.add
                                (i32.add (local.get $obj) (i32.const {str_header}))
                                (i32.add (local.get $pos) (i32.const 6))))
                            (i32.const {ascii_u}))))
                      (then
                        (local.set $low_code
                          (call $json_parse_unicode_escape_code_unit
                            (local.get $obj)
                            (local.get $len)
                            (i32.add (local.get $pos) (i32.const 7))))
                        (if
                          (i32.and
                            (i32.ge_u (local.get $low_code) (i32.const {low_surrogate_start}))
                            (i32.le_u (local.get $low_code) (i32.const {low_surrogate_end})))
                          (then
                            (local.set $code
                              (i32.add
                                (i32.const 65536)
                                (i32.add
                                  (i32.shl
                                    (i32.sub (local.get $code) (i32.const {high_surrogate_start}))
                                    (i32.const 10))
                                  (i32.sub (local.get $low_code) (i32.const {low_surrogate_start})))))
                            (local.set $unicode_advance (i32.const 11))))))
                    (if (i32.eq (local.get $unicode_advance) (i32.const 5))
                      (then (local.set $code (i32.const {replacement_char}))))))
                (if
                  (i32.and
                    (i32.ge_u (local.get $code) (i32.const {low_surrogate_start}))
                    (i32.le_u (local.get $code) (i32.const {low_surrogate_end})))
                  (then (local.set $code (i32.const {replacement_char}))))
                (local.set $out_pos
                  (i32.add
                    (local.get $out_pos)
                    (call $json_write_utf8_at
                      (i32.add
                        (i32.add (local.get $result_ptr) (i32.const {str_header}))
                        (local.get $out_pos))
                      (local.get $code))))
                (local.set $pos (i32.add (local.get $pos) (local.get $unicode_advance)))
                (br $copy_loop))
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
"#,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
            string_tag = ValueTag::STRING,
            str_header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            quote = 34,
            backslash = 92,
            slash = 47,
            space = 32,
            ascii_u = 117,
            ascii_b = 98,
            ascii_f = 102,
            ascii_n = 110,
            ascii_r = 114,
            ascii_t = 116,
            high_surrogate_start = 0xD800,
            high_surrogate_end = 0xDBFF,
            low_surrogate_start = 0xDC00,
            low_surrogate_end = 0xDFFF,
            replacement_char = 0xFFFD,
            backspace = 8,
            formfeed = 12,
            newline = 10,
            carriage = 13,
            tab = 9,
        ));
    }
}
