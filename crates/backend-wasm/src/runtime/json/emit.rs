use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    /// Emit all JSON parse WAT functions.
    pub(crate) fn emit_json_parse(&self, wat: &mut String) {
        let syntax_error = (self.string_offset(RuntimeString::JSON_PARSE_SYNTAX_ERROR)
            + Layout::STRING_HEADER_SIZE) as i32;
        self.emit_json_parse_syntax_error(wat, syntax_error);
        self.emit_json_parse_main(wat, syntax_error);
        self.emit_json_parse_primitives(wat);
        self.emit_json_parse_string(wat);
        self.emit_json_skip_string(wat);
        self.emit_json_parse_fraction_number(wat);
        self.emit_json_parse_number_value(wat);
        self.emit_json_skip_number(wat);
        self.emit_json_parse_object(wat);
        self.emit_json_parse_array(wat);
        self.emit_json_skip_string(wat);
    }

    fn emit_json_parse_syntax_error(&self, wat: &mut String, syntax_error: i32) {
        wat.push_str(&format!(
            r#"
  (func $json_parse_syntax_error
    (call $write (i32.const {json_parse_syntax_error}) (i32.const {json_parse_syntax_error_len}))
    (unreachable))
"#,
            json_parse_syntax_error = syntax_error,
            json_parse_syntax_error_len = RuntimeString::JSON_PARSE_SYNTAX_ERROR.len() as i32,
        ));
    }

    fn emit_json_parse_main(&self, wat: &mut String, _syntax_error: i32) {
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
      (then (call $json_parse_syntax_error)))
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
            (then (call $json_parse_syntax_error)))
          (local.set $pos (call $json_skip_container (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (br $parsed_value)))
      (if (i32.eq (local.get $ch) (i32.const {lbracket}))
        (then
          (local.set $value (call $json_parse_array (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.eq (local.get $value) (i32.const {undefined}))
            (then (call $json_parse_syntax_error)))
          (local.set $pos (call $json_skip_container (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (br $parsed_value)))
      (if (i32.eq (local.get $ch) (i32.const {quote}))
        (then
          (local.set $value (call $json_parse_string (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.eq (local.get $value) (i32.const {undefined}))
            (then (call $json_parse_syntax_error)))
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
        (then (call $json_parse_syntax_error)))
      (local.set $pos (call $json_skip_number (local.get $s_obj) (local.get $s_len) (local.get $pos)))
      (if (i32.gt_u (local.get $pos) (local.get $s_len))
        (then (return (i32.const {undefined}))))
      (br $parsed_value))
    (local.set $pos (call $json_skip_whitespace (local.get $s_obj) (local.get $s_len) (local.get $pos)))
    (if (i32.ne (local.get $pos) (local.get $s_len))
      (then (call $json_parse_syntax_error)))
    (local.get $value))
"#,
            undefined = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            lbrace = 123,
            lbracket = 91,
            quote = 34,
            ascii_n = 110,
            ascii_u = 117,
            ascii_l = 108,
            ascii_t = 116,
            ascii_r = 114,
            ascii_e = 101,
            ascii_f = 102,
            ascii_a = 97,
            ascii_s = 115,
        ));
    }
}
