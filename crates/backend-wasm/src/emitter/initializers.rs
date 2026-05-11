use crate::emitter::{LocalFrame, WatEmitter, module_init_symbol};
use crate::stmt_emit::LoopContext;
use crate::wat_writer::WatWriter;
use ts2wasm_runtime_abi::Layout;

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
            writer.push_str(&format!(
                "    (global.set $current_module_id (i32.const {}))\n",
                module.id
            ));
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
            writer.push_str(&format!(
                "    (global.set $module_cache (call $alloc_heap (i32.const {cache_size})))\n",
            ));
            writer.push_str("    (global.set $current_module_id (i32.const 1))\n");
        }
        buf.clear();
        self.emit_class_prototype_initializers(&mut buf, 4);
        writer.push_str(&buf);
        buf.clear();
        self.emit_builtin_error_prototype_initializers(&mut buf, 4);
        writer.push_str(&buf);
        buf.clear();
        self.emit_module_initializer_calls(&mut buf, 4);
        writer.push_str(&buf);
        if self.module_runtime_enabled() {
            writer.push_str("    (global.set $current_module_id (i32.const 1))\n");
        }
        self.emit_top_level_statements(writer, 4, &frame);
        // Normal termination: call wasi_proc_exit(0) to exit cleanly
        writer.push_str("    (call $wasi_proc_exit (i32.const 0))\n");
        writer.push_str("    (unreachable)\n");
        writer.push_str("  )\n");
    }

    fn emit_module_initializer_calls(&self, wat: &mut String, indent: usize) {
        let pad = " ".repeat(indent);
        for module in &self.program.modules {
            if module.statements.is_empty() {
                continue;
            }
            wat.push_str(&format!("{pad}(call ${})\n", module_init_symbol(module.id)));
        }
    }
}
