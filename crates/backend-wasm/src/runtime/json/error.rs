use crate::emitter::WatEmitter;
use crate::emitter::builtin_error_prototype_global;
use ts2wasm_ir::lowered::BuiltinErrorConstructor;
use ts2wasm_runtime_abi::{
    consts::RuntimeString,
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(crate) fn emit_json_parse_syntax_error(&self, wat: &mut String, syntax_error: i32) {
        let message_value = self.string_value(RuntimeString::JSON_PARSE_SYNTAX_ERROR);
        let message_key = self.string_value("message");
        let object_size = Layout::OBJECT_HEADER_SIZE + Layout::OBJECT_ENTRY_SIZE;
        let prototype_global = builtin_error_prototype_global(BuiltinErrorConstructor::SyntaxError);
        wat.push_str(&format!(
            r#"
  (func $json_parse_syntax_error (result i32)
    (local $error_obj i32)
    (if (i32.eqz (global.get $exception_handler_depth))
      (then
        (call $write (i32.const {syntax_error}) (i32.const {syntax_error_len}))
        (unreachable)))
    (local.set $error_obj (call $alloc_heap (i32.const {object_size})))
    (i32.store (local.get $error_obj) (i32.const 1))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {flags_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {prototype_offset}))
      (global.get ${prototype_global}))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {entries_offset}))
      (i32.const {message_key}))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {value_offset}))
      (i32.const {message_value}))
    (global.set $exception_pending (i32.or (local.get $error_obj) (i32.const {object_tag})))
    (i32.const {undefined_tag}))
"#,
            syntax_error = syntax_error,
            syntax_error_len = RuntimeString::JSON_PARSE_SYNTAX_ERROR.len() as i32,
            object_size = object_size,
            flags_offset = Layout::OBJECT_FLAGS_OFFSET,
            prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            entries_offset = Layout::OBJECT_ENTRIES_OFFSET,
            value_offset = Layout::OBJECT_VALUE_OFFSET,
            prototype_global = prototype_global,
            message_key = message_key,
            message_value = message_value,
            object_tag = ValueTag::OBJECT,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }
}
