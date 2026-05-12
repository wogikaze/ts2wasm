use crate::wasm_ir::{WasmFunction, WasmInstr, WasmValType};
use crate::wat_writer::WatWriter;
use ts2wasm_runtime_abi::ValueTag;

use super::emitter::WatEmitter;

// ---------------------------------------------------------------------------
// Typed WasmIR builders
//
// These free functions construct WasmFunction objects using typed WasmInstr
// variants.  They replace the previous raw WAT string assembly.
// ---------------------------------------------------------------------------

/// Build the typed WasmFunction for `$task_poll`.
///
/// Reads `frame[0]` (state), returns 1 if DONE, 0 if PENDING.
/// Await lowering calls task_poll on the promise task frame before task_result.
/// Frame layout: [state: i32, return_value: i32, locals...]
fn build_task_poll_fn() -> WasmFunction {
    WasmFunction::new("task_poll")
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .body(vec![
            WasmInstr::LocalGet(0),
            WasmInstr::I32Load {
                align: 2,
                offset: 0,
            },
        ])
}

/// Build the typed WasmFunction for `$task_result`.
///
/// Frame layout: [state, cr_status, cr_value].
/// Reads `frame[4]` (cr_status): 0=Normal, 1=Return, 2=Throw.
/// If Throw(2): sets `$exception_pending` to `frame[8]` (cr_value),
/// returns UNDEFINED. Otherwise: returns `frame[8]` (cr_value).
fn build_task_result_fn() -> WasmFunction {
    WasmFunction::new("task_result")
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .local(WasmValType::I32) // cr_status at local index 1
        .local(WasmValType::I32) // cr_value at local index 2
        .body(vec![
            // cr_status = load(frame + 4)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Load {
                align: 2,
                offset: 4,
            },
            WasmInstr::LocalSet(1),
            // cr_value = load(frame + 8)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Load {
                align: 2,
                offset: 8,
            },
            WasmInstr::LocalSet(2),
            // if cr_status == 2 (Throw): set $exception_pending, return UNDEFINED
            WasmInstr::LocalGet(1),
            WasmInstr::I32Const(2),
            WasmInstr::I32Eq,
            WasmInstr::If { result_ty: None },
            WasmInstr::Then,
            WasmInstr::LocalGet(2),
            WasmInstr::GlobalSet("$exception_pending".to_owned()),
            WasmInstr::I32Const(ValueTag::UNDEFINED),
            WasmInstr::Return,
            WasmInstr::End, // close (then
            WasmInstr::End, // close (if
            // After the if: return cr_value
            WasmInstr::LocalGet(2),
            WasmInstr::Return,
        ])
}

/// Build the typed WasmFunction for `$task_drop`.
///
/// Frees the frame allocation by calling `$free` (from alloc_heap).
fn build_task_drop_fn() -> WasmFunction {
    WasmFunction::new("task_drop")
        .param(WasmValType::I32)
        .body(vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$free".to_owned()),
        ])
}

// ---------------------------------------------------------------------------
// WatEmitter dispatch — delegates to typed builders
// ---------------------------------------------------------------------------

impl WatEmitter<'_> {
    /// Emit `$task_poll` using the typed WasmIR builder.
    pub(super) fn emit_task_poll(&self, wat: &mut String) {
        let f = build_task_poll_fn();
        let mut w = WatWriter::new();
        w.emit_function(&f);
        wat.push_str(&w.into_string());
    }

    /// Emit `$task_result` using the typed WasmIR builder.
    pub(super) fn emit_task_result(&self, wat: &mut String) {
        let f = build_task_result_fn();
        let mut w = WatWriter::new();
        w.emit_function(&f);
        wat.push_str(&w.into_string());
    }

    /// Emit `$task_drop` using the typed WasmIR builder.
    pub(super) fn emit_task_drop(&self, wat: &mut String) {
        let f = build_task_drop_fn();
        let mut w = WatWriter::new();
        w.emit_function(&f);
        wat.push_str(&w.into_string());
    }
}

// ---------------------------------------------------------------------------
// Parity tests
//
// Each test builds the typed WasmFunction for one async helper, emits it
// inside a minimal module via WatWriter, and validates the result with
// wat2wasm (and iwasm where applicable).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm_ir::{WasmExport, WasmGlobal, WasmImport, WasmMemory, WasmModule, WasmValType};
    use std::fs;
    use std::process::Command;
    use ts2wasm_shared::test_helpers::unique_temp_dir;

    /// Helper: emit a WasmFunction as a complete module with required globals,
    /// write to disk, run wat2wasm, assert success.
    fn validate_function_wat2wasm(
        f: &WasmFunction,
        globals: Vec<WasmGlobal>,
        imports: Vec<WasmImport>,
        test_name: &str,
    ) {
        let mut module = WasmModule::new().memory(WasmMemory::exported(1, 2, "memory"));
        for imp in imports {
            module = module.import(imp);
        }
        for g in globals {
            module = module.global(g);
        }
        let module = module
            .function(f.clone())
            .export(WasmExport::func(&f.symbol, &f.symbol));

        let mut w = WatWriter::new();
        w.emit_module(&module);
        let wat = w.into_string();

        let temp_dir = unique_temp_dir(&format!("async-{test_name}"));
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wat_path = temp_dir.join("out.wat");
        let wasm_path = temp_dir.join("out.wasm");
        fs::write(&wat_path, &wat).expect("WAT should be written");

        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wasm_path)
            .output()
            .expect("wat2wasm should run");

        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed for {test_name}\nWAT:\n{wat}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr),
        );

        // Cleanup
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn task_poll_wasm_function_is_valid() {
        let f = build_task_poll_fn();
        assert_eq!(f.symbol, "task_poll");
        assert_eq!(f.params, vec![WasmValType::I32]);
        assert_eq!(f.results, vec![WasmValType::I32]);
        assert!(f.locals.is_empty());
        assert_eq!(f.body.len(), 2);
        validate_function_wat2wasm(&f, vec![], vec![], "task-poll");
    }

    #[test]
    fn task_result_wasm_function_is_valid() {
        let f = build_task_result_fn();
        assert_eq!(f.symbol, "task_result");
        assert_eq!(f.params, vec![WasmValType::I32]);
        assert_eq!(f.results, vec![WasmValType::I32]);
        assert_eq!(f.locals.len(), 2);
        assert_eq!(f.locals[0], WasmValType::I32);
        assert_eq!(f.locals[1], WasmValType::I32);
        // task_result references $exception_pending — provide it in the module
        let exception_global = WasmGlobal::i32_mut("$exception_pending", 0);
        validate_function_wat2wasm(&f, vec![exception_global], vec![], "task-result");
    }

    #[test]
    fn task_drop_wasm_function_is_valid() {
        let f = build_task_drop_fn();
        assert_eq!(f.symbol, "task_drop");
        assert_eq!(f.params, vec![WasmValType::I32]);
        assert!(f.results.is_empty());
        assert!(f.locals.is_empty());
        assert_eq!(f.body.len(), 2);
        // task_drop calls $free -- provide it as an import
        let free_import = WasmImport::func("env", "free", "$free", [WasmValType::I32], []);
        validate_function_wat2wasm(&f, vec![], vec![free_import], "task-drop");
    }

    #[test]
    fn task_poll_wasm_function_body_structure() {
        let f = build_task_poll_fn();
        assert!(matches!(f.body[0], WasmInstr::LocalGet(0)));
        assert!(matches!(
            f.body[1],
            WasmInstr::I32Load {
                align: 2,
                offset: 0
            }
        ));
    }

    #[test]
    fn task_result_wasm_function_has_if_then_return() {
        let f = build_task_result_fn();
        let instrs = &f.body;
        // Find the If after the eq-check
        let if_idx = instrs
            .iter()
            .position(|i| matches!(i, WasmInstr::If { .. }))
            .expect("task_result body should contain If");
        assert!(
            matches!(instrs[if_idx + 1], WasmInstr::Then),
            "If should be followed by Then"
        );
        assert!(
            instrs[if_idx + 2..]
                .iter()
                .any(|i| matches!(i, WasmInstr::GlobalSet(_))),
            "Then body should contain GlobalSet"
        );
        assert!(
            instrs[if_idx + 2..]
                .iter()
                .any(|i| matches!(i, WasmInstr::Return)),
            "Then body should contain Return"
        );
        // After Then, expect End (close then) then End (close if) or two consecutive Ends
        let ends_after_if: usize = instrs[if_idx + 1..]
            .iter()
            .filter(|i| matches!(i, WasmInstr::End))
            .count();
        assert!(
            ends_after_if >= 2,
            "task_result body should have at least 2 End instructions after If (close then + close if), found {ends_after_if}"
        );
    }

    #[test]
    fn task_drop_wasm_function_body_structure() {
        let f = build_task_drop_fn();
        assert!(matches!(f.body[0], WasmInstr::LocalGet(0)));
        assert!(matches!(&f.body[1], WasmInstr::Call(name) if name == "$free"));
    }
}
