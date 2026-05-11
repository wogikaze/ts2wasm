use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Emit literal matching, hex value, unicode escape, utf8 helpers.
    pub(crate) fn emit_json_parse_primitives(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
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

  (func $json_parse_unicode_escape_code_unit (param $obj i32) (param $len i32) (param $pos i32) (result i32)
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
    (local.get $code))

  (func $json_utf8_len (param $code i32) (result i32)
    (if (i32.lt_u (local.get $code) (i32.const 128))
      (then (return (i32.const {one}))))
    (if (i32.lt_u (local.get $code) (i32.const 2048))
      (then (return (i32.const 2))))
    (if (i32.lt_u (local.get $code) (i32.const 65536))
      (then (return (i32.const 3))))
    (i32.const 4))

  (func $json_write_utf8_at (param $ptr i32) (param $code i32) (result i32)
    (if (i32.lt_u (local.get $code) (i32.const 128))
      (then
        (i32.store8 (local.get $ptr) (local.get $code))
        (return (i32.const {one}))))
    (if (i32.lt_u (local.get $code) (i32.const 2048))
      (then
        (i32.store8
          (local.get $ptr)
          (i32.or
            (i32.const 192)
            (i32.shr_u (local.get $code) (i32.const 6))))
        (i32.store8
          (i32.add (local.get $ptr) (i32.const {one}))
          (i32.or
            (i32.const 128)
            (i32.and (local.get $code) (i32.const 63))))
        (return (i32.const 2))))
    (if (i32.lt_u (local.get $code) (i32.const 65536))
      (then
        (i32.store8
          (local.get $ptr)
          (i32.or
            (i32.const 224)
            (i32.shr_u (local.get $code) (i32.const 12))))
        (i32.store8
          (i32.add (local.get $ptr) (i32.const {one}))
          (i32.or
            (i32.const 128)
            (i32.and
              (i32.shr_u (local.get $code) (i32.const 6))
              (i32.const 63))))
        (i32.store8
          (i32.add (local.get $ptr) (i32.const 2))
          (i32.or
            (i32.const 128)
            (i32.and (local.get $code) (i32.const 63))))
        (return (i32.const 3))))
    (i32.store8
      (local.get $ptr)
      (i32.or
        (i32.const 240)
        (i32.shr_u (local.get $code) (i32.const 18))))
    (i32.store8
      (i32.add (local.get $ptr) (i32.const {one}))
      (i32.or
        (i32.const 128)
        (i32.and
          (i32.shr_u (local.get $code) (i32.const 12))
          (i32.const 63))))
    (i32.store8
      (i32.add (local.get $ptr) (i32.const 2))
      (i32.or
        (i32.const 128)
        (i32.and
          (i32.shr_u (local.get $code) (i32.const 6))
          (i32.const 63))))
    (i32.store8
      (i32.add (local.get $ptr) (i32.const 3))
      (i32.or
        (i32.const 128)
        (i32.and (local.get $code) (i32.const 63))))
    (i32.const 4))

  (func $json_skip_whitespace (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $ch i32)
    (block $done
      (loop $skip
        (br_if $done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {space})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (if (i32.eq (local.get $ch) (i32.const {tab})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (if (i32.eq (local.get $ch) (i32.const {newline})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (if (i32.eq (local.get $ch) (i32.const {carriage})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (br $done)))
    (local.get $pos))

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
"#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            ten = RuntimeConst::TEN,
            str_header = Layout::STRING_HEADER_SIZE,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            ascii_upper_a = 65,
            ascii_upper_f = 70,
            ascii_lower_a = 97,
            ascii_lower_f = 102,
            quote = 34,
            lbrace = 123,
            rbrace = 125,
            lbracket = 91,
            rbracket = 93,
            space = 32,
            tab = 9,
            newline = 10,
            carriage = 13,
        ));
    }

    /// Emit skip-string and skip-number helpers.
    pub(crate) fn emit_json_skip_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
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
"#,
            one = RuntimeConst::ONE,
            str_header = Layout::STRING_HEADER_SIZE,
            backslash = 92,
            quote = 34,
        ));
    }
}
