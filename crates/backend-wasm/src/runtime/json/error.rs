use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::consts::RuntimeString;

impl WatEmitter<'_> {
    /// Emit the `$json_parse_syntax_error` WAT function.
    pub(crate) fn emit_json_parse_syntax_error(&self, wat: &mut String, syntax_error: i32) {
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
}
