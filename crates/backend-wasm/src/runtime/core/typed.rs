// ---------------------------------------------------------------------------
// Typed WasmIR builders for the Core runtime domain.
//
// Each function constructs a `WasmFunction` using `WasmInstr` variants instead
// of raw WAT string assembly.  This is a migration demonstration: the typed
// path is side-by-side with the existing raw-WAT `emit.rs` functions.
//
// Escape hatches (WasmInstr::Raw) are used where a typed variant does not yet
// exist — see doc comments per function.
//
// Each builder has a corresponding parity test that:
//   1. Emits via `WatWriter::emit_function()`
//   2. Validates the WAT with `wat2wasm`
//   3. Checks the output is non-empty and contains the function name
// ---------------------------------------------------------------------------

use ts2wasm_backend_core::wasm_ir::{WasmFunction, WasmInstr, WasmValType};
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

// ---------------------------------------------------------------------------
// Builders
// ---------------------------------------------------------------------------

/// Build the `$bitwise_and` function.
///
/// Generated WAT (flat instruction format):
/// ```wat
/// (func $bitwise_and (param i32) (param i32) (result i32)
///   (local.get 0)
///   (call $bitwise_to_i32)
///   (local.get 1)
///   (call $bitwise_to_i32)
///   (i32.and)
///   (call $number_from_i32))
/// ```
///
/// Remaining raw escape hatches: none.
pub fn build_bitwise_and() -> WasmFunction {
    WasmFunction::new("$bitwise_and")
        .param(WasmValType::I32)
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .body(vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$bitwise_to_i32".to_owned()),
            WasmInstr::LocalGet(1),
            WasmInstr::Call("$bitwise_to_i32".to_owned()),
            WasmInstr::I32And,
            WasmInstr::Call("$number_from_i32".to_owned()),
        ])
}

/// Build the `$bitwise_or` function.
///
/// Remaining raw escape hatches: none.
pub fn build_bitwise_or() -> WasmFunction {
    WasmFunction::new("$bitwise_or")
        .param(WasmValType::I32)
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .body(vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$bitwise_to_i32".to_owned()),
            WasmInstr::LocalGet(1),
            WasmInstr::Call("$bitwise_to_i32".to_owned()),
            WasmInstr::I32Or,
            WasmInstr::Call("$number_from_i32".to_owned()),
        ])
}

/// Build the `$bitwise_xor` function.
///
/// Remaining raw escape hatches: none.
pub fn build_bitwise_xor() -> WasmFunction {
    WasmFunction::new("$bitwise_xor")
        .param(WasmValType::I32)
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .body(vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$bitwise_to_i32".to_owned()),
            WasmInstr::LocalGet(1),
            WasmInstr::Call("$bitwise_to_i32".to_owned()),
            WasmInstr::I32Xor,
            WasmInstr::Call("$number_from_i32".to_owned()),
        ])
}

/// Build the `$is_string` function.
///
/// Checks whether a value's tag equals STRING.
/// Uses `WasmInstr::Raw` for the inline `(i32.eq (i32.and ...) ...)` pattern
/// since WasmInstr has no typed variant for inline-equality-with-and.
///
/// Remaining raw escape hatches: `WasmInstr::Raw` for the combined
/// and-eq expression (no single-instruction WasmInstr for and+eq with
/// hardcoded constants).
pub fn build_is_string() -> WasmFunction {
    let tag_mask = ValueTag::TAG_MASK as i32;
    let string_tag = ValueTag::STRING as i32;
    WasmFunction::new("$is_string")
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .body(vec![
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(tag_mask),
            WasmInstr::I32And,
            WasmInstr::I32Const(string_tag),
            WasmInstr::I32Eq,
        ])
}

/// Build the `$symbol_key_for` function.
///
/// Simply returns its argument (identity function).
///
/// Remaining raw escape hatches: none.
pub fn build_symbol_key_for() -> WasmFunction {
    WasmFunction::new("$symbol_key_for")
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .body(vec![WasmInstr::LocalGet(0), WasmInstr::Return])
}

/// Build the `$symbol_for` function.
///
/// `str_symbol_open` is the tagged value for "Symbol(".
/// `str_close_paren` is the tagged value for ")".
///
/// Remaining raw escape hatches: none.
pub fn build_symbol_for(str_symbol_open: i32, str_close_paren: i32) -> WasmFunction {
    WasmFunction::new("$symbol_for")
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .body(vec![
            WasmInstr::I32Const(str_symbol_open),
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$concat".to_owned()),
            WasmInstr::I32Const(str_close_paren),
            WasmInstr::Call("$concat".to_owned()),
            WasmInstr::Return,
        ])
}

/// Build the `$symbol_new` function.
///
/// `str_symbol_open` is the tagged value for "Symbol(".
/// `str_close_paren` is the tagged value for ")".
/// `str_empty` is the tagged value for "".
/// `undefined_tag` is the ValueTag::UNDEFINED value.
///
/// Remaining raw escape hatches: none.
pub fn build_symbol_new(
    str_symbol_open: i32,
    str_close_paren: i32,
    str_empty: i32,
    undefined_tag: i32,
) -> WasmFunction {
    WasmFunction::new("$symbol_new")
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .body(vec![
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(undefined_tag),
            WasmInstr::I32Eq,
            WasmInstr::If { result_ty: None },
            WasmInstr::Then,
            WasmInstr::I32Const(str_symbol_open),
            WasmInstr::I32Const(str_empty),
            WasmInstr::Call("$concat".to_owned()),
            WasmInstr::I32Const(str_close_paren),
            WasmInstr::Call("$concat".to_owned()),
            WasmInstr::Return,
            WasmInstr::End,
            WasmInstr::End,
            WasmInstr::I32Const(str_symbol_open),
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$concat".to_owned()),
            WasmInstr::I32Const(str_close_paren),
            WasmInstr::Call("$concat".to_owned()),
            WasmInstr::Return,
        ])
}

/// Build the `$error_message` function.
///
/// Converts a value to a string error message.  If the value is `undefined`,
/// returns an empty string.
///
/// Locals: `$len` (index 1), `$ptr` (index 2).
///
/// Remaining raw escape hatches:
///   - `WasmInstr::Raw` for `(i32.store)` — the typed `I32Store` emits
///     combined `align=N offset=N` which this wat2wasm version rejects.
pub fn build_error_message() -> WasmFunction {
    let string_header = Layout::STRING_HEADER_SIZE as i32;
    let scratch = Layout::SCRATCH_OFFSET as i32;
    let undefined_tag = ValueTag::UNDEFINED as i32;
    let string_tag = ValueTag::STRING as i32;
    let zero = RuntimeConst::ZERO as i32;

    WasmFunction::new("$error_message")
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .local(WasmValType::I32) // local 1: $len
        .local(WasmValType::I32) // local 2: $ptr
        .body(vec![
            // if (v == undefined) { alloc empty string, return it }
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(undefined_tag),
            WasmInstr::I32Eq,
            WasmInstr::If { result_ty: None },
            WasmInstr::Then,
            WasmInstr::I32Const(string_header),
            WasmInstr::Call("$alloc_heap".to_owned()),
            WasmInstr::LocalSet(2),
            WasmInstr::LocalGet(2),
            WasmInstr::I32Const(zero),
            WasmInstr::Raw("(i32.store)".to_owned()),
            WasmInstr::LocalGet(2),
            WasmInstr::I32Const(string_tag),
            WasmInstr::I32Or,
            WasmInstr::Return,
            WasmInstr::End,
            WasmInstr::End,
            // $len = value_to_string_into(v, SCRATCH)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(scratch),
            WasmInstr::Call("$value_to_string_into".to_owned()),
            WasmInstr::LocalSet(1),
            // $ptr = alloc_heap(STRING_HEADER_SIZE + $len)
            WasmInstr::I32Const(string_header),
            WasmInstr::LocalGet(1),
            WasmInstr::I32Add,
            WasmInstr::Call("$alloc_heap".to_owned()),
            WasmInstr::LocalSet(2),
            // store length at $ptr
            WasmInstr::LocalGet(2),
            WasmInstr::LocalGet(1),
            WasmInstr::Raw("(i32.store)".to_owned()),
            // copy(SCRATCH, $ptr + STRING_HEADER_SIZE, $len)
            WasmInstr::I32Const(scratch),
            WasmInstr::LocalGet(2),
            WasmInstr::I32Const(string_header),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(1),
            WasmInstr::Call("$copy".to_owned()),
            // return $ptr | STRING_TAG
            WasmInstr::LocalGet(2),
            WasmInstr::I32Const(string_tag),
            WasmInstr::I32Or,
        ])
}

/// Build the `$log` function.
///
/// Writes a value's string representation to stdout with a newline.
/// Uses `WasmInstr::Raw` for the inline GC-kind check (the bitwise
/// `(i32.and (i32.load ...) ...)` sequence to test for BigInt kind)
/// because WasmInstr has no typed variant for nested-load-and-compare.
///
/// Remaining raw escape hatches: `WasmInstr::Raw` for GC kind/flag checks.
pub fn build_log(newline_offset: i32) -> WasmFunction {
    let scratch = Layout::SCRATCH_OFFSET as i32;
    let tag_mask = ValueTag::TAG_MASK as i32;
    let object_tag = ValueTag::OBJECT as i32;
    let heap_mask = ValueTag::HEAP_MASK as i32;
    let gc_header_size = Layout::GC_HEADER_SIZE as i32;
    let gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET as i32;
    let gc_kind_mask = Layout::GC_KIND_MASK as i32;
    let gc_kind_bigint = Layout::GC_KIND_BIGINT as i32;
    let ascii_n = b'n' as i32;
    let one = RuntimeConst::ONE as i32;

    WasmFunction::new("$log")
        .param(WasmValType::I32)
        .local(WasmValType::I32) // local 1: $obj
        .local(WasmValType::I32) // local 2: $len
        .body(vec![
            // $len = value_to_string_into(v, SCRATCH)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(scratch),
            WasmInstr::Call("$value_to_string_into".to_owned()),
            WasmInstr::LocalSet(2),
            // if tag == OBJECT: check BigInt kind
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(tag_mask),
            WasmInstr::I32And,
            WasmInstr::I32Const(object_tag),
            WasmInstr::I32Eq,
            WasmInstr::If { result_ty: None },
            WasmInstr::Then,
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(heap_mask),
            WasmInstr::I32And,
            WasmInstr::LocalSet(1),
            // Load GC flags from header and check BigInt kind
            // (uses WasmInstr::Raw because typed I32Load emits combined
            //  align=N offset=N which this wat2wasm version rejects)
            WasmInstr::LocalGet(1),
            WasmInstr::I32Const(gc_header_size),
            WasmInstr::I32Sub,
            WasmInstr::I32Const(gc_flags_offset),
            WasmInstr::I32Add,
            WasmInstr::Raw("(i32.load)".to_owned()),
            WasmInstr::I32Const(gc_kind_mask),
            WasmInstr::I32And,
            WasmInstr::I32Const(gc_kind_bigint),
            WasmInstr::I32Eq,
            WasmInstr::If { result_ty: None },
            WasmInstr::Then,
            // Append 'n' character for BigInt (i32.store8 not yet typed)
            WasmInstr::I32Const(scratch),
            WasmInstr::LocalGet(2),
            WasmInstr::I32Add,
            WasmInstr::I32Const(ascii_n),
            WasmInstr::Raw("(i32.store8)".to_owned()),
            WasmInstr::LocalGet(2),
            WasmInstr::I32Const(one),
            WasmInstr::I32Add,
            WasmInstr::LocalSet(2),
            WasmInstr::End,
            WasmInstr::End,
            WasmInstr::End,
            WasmInstr::End,
            // write(SCRATCH, $len)
            WasmInstr::I32Const(scratch),
            WasmInstr::LocalGet(2),
            WasmInstr::Call("$write".to_owned()),
            // write(newline_offset, 1)
            WasmInstr::I32Const(newline_offset),
            WasmInstr::I32Const(one),
            WasmInstr::Call("$write".to_owned()),
        ])
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wat_writer::WatWriter;
    use std::process::Command;

    /// Validate that the given WAT text is syntactically valid using wat2wasm.
    fn validate_wat(wat: &str) {
        let mut child = Command::new("wat2wasm")
            .arg("-o")
            .arg("/dev/null")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("wat2wasm not found; install wabt tools");

        use std::io::Write;
        child
            .stdin
            .take()
            .unwrap()
            .write_all(wat.as_bytes())
            .expect("write stdin");

        let output = child.wait_with_output().expect("wait for wat2wasm");
        assert!(
            output.status.success(),
            "wat2wasm validation failed:\n--- WAT ---\n{}\n--- stderr ---\n{}",
            wat,
            String::from_utf8_lossy(&output.stderr),
        );
    }

    /// Emit a WasmFunction into a complete WAT module and validate it.
    /// `stubs` is a list of `(symbol, params, results)` tuples for stub functions
    /// that the tested function calls, so wat2wasm can resolve cross-references.
    fn emit_and_validate_with_stubs(f: &WasmFunction, stubs: &[(&str, &str, &str)]) -> String {
        let mut w = WatWriter::new();
        w.open_module();
        w.line(2, "(memory 1)");
        w.emit_function(f);
        for (symbol, params, results) in stubs {
            w.push_str(&format!(
                "  (func {} (param {}) (result {})\n",
                symbol, params, results
            ));
            if !results.is_empty() {
                w.line(4, "(i32.const 0)");
            }
            w.func_end();
        }
        w.close_module();
        let wat = w.into_string();
        validate_wat(&wat);
        wat
    }

    fn emit_and_validate(f: &WasmFunction) -> String {
        emit_and_validate_with_stubs(f, &[])
    }

    // ---- bitwise operations -------------------------------------------------

    #[test]
    fn typed_bitwise_and_emits_valid_wat() {
        let f = build_bitwise_and();
        let stubs = &[
            ("$bitwise_to_i32", "i32", "i32"),
            ("$number_from_i32", "i32", "i32"),
        ];
        let wat = emit_and_validate_with_stubs(&f, stubs);
        assert!(wat.contains("$bitwise_and"));
        assert!(wat.contains("i32.and"));
        assert!(wat.contains("$bitwise_to_i32"));
        assert!(wat.contains("$number_from_i32"));
    }

    #[test]
    fn typed_bitwise_or_emits_valid_wat() {
        let f = build_bitwise_or();
        let stubs = &[
            ("$bitwise_to_i32", "i32", "i32"),
            ("$number_from_i32", "i32", "i32"),
        ];
        let wat = emit_and_validate_with_stubs(&f, stubs);
        assert!(wat.contains("$bitwise_or"));
        assert!(wat.contains("i32.or"));
    }

    #[test]
    fn typed_bitwise_xor_emits_valid_wat() {
        let f = build_bitwise_xor();
        let stubs = &[
            ("$bitwise_to_i32", "i32", "i32"),
            ("$number_from_i32", "i32", "i32"),
        ];
        let wat = emit_and_validate_with_stubs(&f, stubs);
        assert!(wat.contains("$bitwise_xor"));
        assert!(wat.contains("i32.xor"));
    }

    #[test]
    fn typed_bitwise_functions_are_distinct() {
        let stubs = &[
            ("$bitwise_to_i32", "i32", "i32"),
            ("$number_from_i32", "i32", "i32"),
        ];
        let and = emit_and_validate_with_stubs(&build_bitwise_and(), stubs);
        let or = emit_and_validate_with_stubs(&build_bitwise_or(), stubs);
        let xor = emit_and_validate_with_stubs(&build_bitwise_xor(), stubs);
        assert_ne!(and, or);
        assert_ne!(or, xor);
    }

    // ---- type checks --------------------------------------------------------

    #[test]
    fn typed_is_string_emits_valid_wat() {
        let f = build_is_string();
        let wat = emit_and_validate(&f);
        assert!(wat.contains("$is_string"));
        assert!(wat.contains("i32.and"));
        assert!(wat.contains("i32.eq"));
    }

    // ---- symbol helpers -----------------------------------------------------

    #[test]
    fn typed_symbol_key_for_emits_valid_wat() {
        let f = build_symbol_key_for();
        let wat = emit_and_validate(&f);
        assert!(wat.contains("$symbol_key_for"));
        assert!(wat.contains("local.get 0"));
    }

    #[test]
    fn typed_symbol_for_emits_valid_wat() {
        let open: i32 = 1000;
        let close: i32 = 1004;
        let f = build_symbol_for(open, close);
        let stubs = &[("$concat", "i32 i32", "i32")];
        let wat = emit_and_validate_with_stubs(&f, stubs);
        assert!(wat.contains("$symbol_for"));
        assert!(wat.contains("$concat"));
    }

    #[test]
    fn typed_symbol_new_emits_valid_wat() {
        let open: i32 = 1000;
        let close: i32 = 1004;
        let empty: i32 = 1008;
        let f = build_symbol_new(open, close, empty, ValueTag::UNDEFINED as i32);
        let stubs = &[("$concat", "i32 i32", "i32")];
        let wat = emit_and_validate_with_stubs(&f, stubs);
        assert!(wat.contains("$symbol_new"));
        assert!(wat.contains("if"));
        assert!(wat.contains("then"));
        assert!(wat.contains("$concat"));
    }

    // ---- error message ------------------------------------------------------

    #[test]
    fn typed_error_message_emits_valid_wat() {
        let f = build_error_message();
        let stubs = &[
            ("$alloc_heap", "i32", "i32"),
            ("$value_to_string_into", "i32 i32", "i32"),
            ("$copy", "i32 i32 i32", ""),
        ];
        let wat = emit_and_validate_with_stubs(&f, stubs);
        assert!(wat.contains("$error_message"));
        assert!(wat.contains("$value_to_string_into"));
        assert!(wat.contains("$alloc_heap"));
        assert!(wat.contains("$copy"));
        assert!(wat.contains("i32.store"));
        assert!(wat.contains("i32.or"));
    }

    // ---- log ----------------------------------------------------------------

    #[test]
    fn typed_log_emits_valid_wat() {
        let f = build_log(42);
        let stubs = &[
            ("$value_to_string_into", "i32 i32", "i32"),
            ("$write", "i32 i32", ""),
        ];
        let wat = emit_and_validate_with_stubs(&f, stubs);
        assert!(wat.contains("$log"));
        assert!(wat.contains("$value_to_string_into"));
        assert!(wat.contains("$write"));
        assert!(wat.contains("i32.store8"));
        assert!(wat.contains("i32.load"));
    }

    // ---- multi-function parity ----------------------------------------------

    #[test]
    fn typed_bitwise_triad_in_single_module() {
        let deps = &["$bitwise_to_i32", "$number_from_i32"];
        let mut w = WatWriter::new();
        w.open_module();
        w.emit_function(&build_bitwise_and());
        w.emit_function(&build_bitwise_or());
        w.emit_function(&build_bitwise_xor());
        for dep in deps {
            w.push_str(&format!("  (func {} (param i32) (result i32)\n", dep));
            w.line(4, "(i32.const 0)");
            w.func_end();
        }
        w.close_module();
        let wat = w.into_string();
        validate_wat(&wat);
        assert!(wat.contains("$bitwise_and"));
        assert!(wat.contains("$bitwise_or"));
        assert!(wat.contains("$bitwise_xor"));
    }

    #[test]
    fn typed_symbols_in_single_module() {
        let open: i32 = 1000;
        let close: i32 = 1004;
        let empty: i32 = 1008;
        let mut w = WatWriter::new();
        w.open_module();
        w.emit_function(&build_symbol_key_for());
        w.emit_function(&build_symbol_for(open, close));
        w.emit_function(&build_symbol_new(
            open,
            close,
            empty,
            ValueTag::UNDEFINED as i32,
        ));
        w.push_str("  (func $concat (param i32) (param i32) (result i32)\n");
        w.line(4, "(i32.const 0)");
        w.func_end();
        w.close_module();
        let wat = w.into_string();
        validate_wat(&wat);
        assert!(wat.contains("$symbol_key_for"));
        assert!(wat.contains("$symbol_for"));
        assert!(wat.contains("$symbol_new"));
    }

    // ---- type spec and structure checks -------------------------------------

    #[test]
    fn typed_error_message_has_correct_params_locals() {
        let f = build_error_message();
        assert_eq!(f.params.len(), 1);
        assert_eq!(f.results.len(), 1);
        assert_eq!(f.locals.len(), 2);
    }

    #[test]
    fn typed_log_has_correct_params_locals() {
        let f = build_log(42);
        assert_eq!(f.params.len(), 1);
        assert_eq!(f.results.len(), 0);
        assert_eq!(f.locals.len(), 2);
    }

    #[test]
    fn typed_bitwise_operations_have_correct_params() {
        let f = build_bitwise_and();
        assert_eq!(f.params.len(), 2);
        assert_eq!(f.results.len(), 1);
        assert_eq!(f.locals.len(), 0);
    }

    #[test]
    fn typed_symbol_new_has_if_then_structure() {
        let open: i32 = 1000;
        let close: i32 = 1004;
        let empty: i32 = 1008;
        let f = build_symbol_new(open, close, empty, ValueTag::UNDEFINED as i32);
        // Should contain if/then/end structure
        let mut w = WatWriter::new();
        w.open_module();
        w.emit_function(&f);
        w.close_module();
        let wat = w.into_string();
        assert!(wat.contains("(if"));
        assert!(wat.contains("(then"));
    }
}
