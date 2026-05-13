use crate::emitter::{LocalFrame, WatEmitter, module_init_symbol};
use crate::stmt_emit::LoopContext;
use crate::wasm_ir::WasmInstr;
use crate::wat_writer::WatWriter;
use ts2wasm_runtime_abi::Layout;

// Typed migration boundary for this domain:
// start/module-initializer orchestration now emits straight-line runtime setup,
// module calls, and process-exit instructions through WasmInstr. Function
// headers/locals, GC frame helpers, and statement lowering remain raw WAT escape
// hatches because they still depend on legacy emitter-local layouts.

impl WatEmitter<'_> {
    pub(super) fn emit_module_initializers(&self, writer: &mut WatWriter) {
        for module in &self.program.modules {
            if module.statements.is_empty() {
                continue;
            }
            writer.push_str(&format!("  (func ${}\n", module_init_symbol(module.id)));
            let frame =
                LocalFrame::activation(module.locals_count, self.gc_call_frame_roots_enabled());
            for _ in 0..frame.total_local_count() {
                writer.push_str("    (local i32)\n");
            }
            // Completion Record locals: cr_status, cr_value, cr_target, cr_saved_status, cr_saved_value.
            for _ in 0..LocalFrame::CR_LOCAL_COUNT {
                writer.push_str("    (local i32)\n");
            }
            let mut buf = String::new();
            // Initialize Completion Record status to Normal (0) at function entry.
            let cr_base = frame.cr_local_base();
            writer.line_fmt(4, format_args!("(i32.const 0) (local.set {})", cr_base));
            // Initialize cr_target to TARGET_EMPTY (0).
            writer.line_fmt(4, format_args!("(i32.const 0) (local.set {})", cr_base + 2));
            self.emit_gc_activation_frame_push(&mut buf, &frame, 4);
            writer.push_str(&buf);
            buf.clear();
            emit_typed_instrs(
                &mut buf,
                4,
                &set_current_module_id_instrs(module.id, "module initializer id"),
            );
            writer.push_str(&buf);
            buf.clear();
            let mut loop_ctx = LoopContext::default();
            self.emit_statements(writer, &module.statements, 4, &mut loop_ctx, &frame);
            buf.clear();
            self.emit_gc_activation_frame_pop(&mut buf, &frame, 4);
            writer.push_str(&buf);
            writer.push_str("  )\n");
        }
    }

    pub(super) fn emit_start(&self, writer: &mut WatWriter) {
        writer.push_str("  (func $_start (export \"_start\")\n");
        let extra_locals = if self.module_runtime_enabled() { 1 } else { 0 };
        let frame = LocalFrame::new(
            self.program.top_level_locals.len() + extra_locals,
            self.gc_root_table_enabled().then_some(0),
        );
        for _ in 0..frame.total_local_count() {
            writer.push_str("    (local i32)\n");
        }
        // Completion Record locals: cr_status, cr_value, cr_target, cr_saved_status, cr_saved_value.
        for _ in 0..LocalFrame::CR_LOCAL_COUNT {
            writer.push_str("    (local i32)\n");
        }
        let mut buf = String::new();
        // Initialize Completion Record status to Normal (0) at function entry.
        let cr_base = frame.cr_local_base();
        writer.line_fmt(4, format_args!("(i32.const 0) (local.set {})", cr_base));
        // Initialize cr_target to TARGET_EMPTY (0).
        writer.line_fmt(4, format_args!("(i32.const 0) (local.set {})", cr_base + 2));
        self.emit_gc_root_table_initializer(&mut buf, 4);
        writer.push_str(&buf);
        if self.module_runtime_enabled() {
            let cache_size = Layout::MODULE_CACHE_MAX * Layout::MODULE_CACHE_ENTRY_SIZE;
            emit_typed_instrs(&mut buf, 4, &module_cache_init_instrs(cache_size));
            emit_typed_instrs(
                &mut buf,
                4,
                &set_current_module_id_instrs(1, "root module id"),
            );
            writer.push_str(&buf);
            buf.clear();
        }
        self.emit_class_prototype_initializers(&mut buf, 4);
        writer.push_str(&buf);
        buf.clear();
        self.emit_builtin_error_prototype_initializers(&mut buf, 4);
        writer.push_str(&buf);
        buf.clear();
        emit_typed_instrs(&mut buf, 4, &self.module_initializer_call_instrs());
        writer.push_str(&buf);
        buf.clear();
        if self.module_runtime_enabled() {
            emit_typed_instrs(
                &mut buf,
                4,
                &set_current_module_id_instrs(1, "root module id"),
            );
            writer.push_str(&buf);
            buf.clear();
        }
        self.emit_top_level_statements(writer, 4, &frame);
        // Normal termination: call wasi_proc_exit(0) to exit cleanly
        emit_typed_instrs(&mut buf, 4, &normal_exit_instrs());
        writer.push_str(&buf);
        writer.push_str("  )\n");
    }

    fn module_initializer_call_instrs(&self) -> Vec<WasmInstr> {
        self.program
            .modules
            .iter()
            .filter(|module| !module.statements.is_empty())
            .map(|module| WasmInstr::Call(format!("${}", module_init_symbol(module.id))))
            .collect()
    }
}

fn emit_typed_instrs(wat: &mut String, indent: usize, instrs: &[WasmInstr]) {
    let mut writer = WatWriter::new();
    writer.emit_instrs(indent, instrs);
    wat.push_str(&writer.into_string());
}

fn set_current_module_id_instrs(module_id: usize, context: &str) -> Vec<WasmInstr> {
    vec![
        WasmInstr::I32Const(checked_i32(module_id, context)),
        WasmInstr::GlobalSet("$current_module_id".to_owned()),
    ]
}

fn module_cache_init_instrs(cache_size: u32) -> Vec<WasmInstr> {
    vec![
        WasmInstr::I32Const(checked_i32(cache_size, "module cache size")),
        WasmInstr::Call("$alloc_heap".to_owned()),
        WasmInstr::GlobalSet("$module_cache".to_owned()),
    ]
}

fn normal_exit_instrs() -> Vec<WasmInstr> {
    vec![
        WasmInstr::I32Const(0),
        WasmInstr::Call("$wasi_proc_exit".to_owned()),
        WasmInstr::Unreachable,
    ]
}

fn checked_i32(value: impl TryInto<i32>, context: &str) -> i32 {
    value
        .try_into()
        .unwrap_or_else(|_| panic!("{context} does not fit in i32"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_runtime_abi::Layout;

    #[test]
    fn start_orchestration_uses_typed_wasm_instrs_without_raw_escape_hatches() {
        let mut instrs = Vec::new();
        instrs.extend(module_cache_init_instrs(
            Layout::MODULE_CACHE_MAX * Layout::MODULE_CACHE_ENTRY_SIZE,
        ));
        instrs.extend(set_current_module_id_instrs(1, "root module id"));
        instrs.extend([
            WasmInstr::Call("$module_init_2".to_owned()),
            WasmInstr::Call("$module_init_1".to_owned()),
        ]);
        instrs.extend(set_current_module_id_instrs(1, "root module id"));
        instrs.extend(normal_exit_instrs());

        assert!(
            instrs
                .iter()
                .all(|instr| !matches!(instr, WasmInstr::Raw(_))),
            "initializer orchestration should not need raw WAT instructions"
        );

        let mut wat = String::new();
        emit_typed_instrs(&mut wat, 4, &instrs);

        assert!(wat.contains("    (i32.const 512)"));
        assert!(wat.contains("    (call $alloc_heap)"));
        assert!(wat.contains("    (global.set $module_cache)"));
        assert!(wat.contains("    (call $module_init_2)"));
        assert!(wat.contains("    (call $module_init_1)"));
        assert!(wat.contains("    (call $wasi_proc_exit)"));
        assert!(wat.contains("    (unreachable)"));
    }

    #[test]
    fn module_id_setting_renders_stack_form() {
        let instrs = set_current_module_id_instrs(7, "test module id");
        assert!(matches!(instrs.as_slice(), [
            WasmInstr::I32Const(7),
            WasmInstr::GlobalSet(name),
        ] if name == "$current_module_id"));

        let mut wat = String::new();
        emit_typed_instrs(&mut wat, 4, &instrs);

        assert_eq!(
            wat,
            "    (i32.const 7)\n    (global.set $current_module_id)\n"
        );
    }
}
