use crate::emitter::{LocalFrame, WatEmitter, function_symbol};
use crate::runtime_fn::NATIVE_SET_ADD_SENTINEL;
use crate::stmt_emit::LoopContext;
use crate::wat_writer::WatWriter;
use ts2wasm_runtime_abi::ValueTag;

impl WatEmitter<'_> {
    pub(super) fn emit_functions(&self, writer: &mut WatWriter) {
        let mut buf = String::new();
        for function in &self.program.functions {
            let is_async = function.is_async;
            let user_local_count = function.params.len() + function.locals.len();
            writer.push_str(&format!("  (func ${} ", function_symbol(function.id)));
            for _ in &function.params {
                writer.push_str("(param i32) ");
            }
            writer.push_str("(result i32)\n");
            for _ in &function.locals {
                writer.push_str("    (local i32)\n");
            }
            let frame =
                LocalFrame::activation(user_local_count, self.gc_call_frame_roots_enabled());
            // Backend-owned temporaries for heap construction and switch dispatch.
            for _ in 0..frame.backend_local_count() {
                writer.push_str("    (local i32)\n");
            }
            // Completion Record locals: cr_status, cr_value, cr_target, cr_saved_status, cr_saved_value.
            for _ in 0..LocalFrame::CR_LOCAL_COUNT {
                writer.push_str("    (local i32)\n");
            }
            if is_async {
                writer.push_str("    (local $frame i32)\n");
            }
            buf.clear();
            if is_async {
                // Allocate state-machine frame: [state, return_value, saved_locals...]
                // For now, only allocate state + return_value (2 slots = 8 bytes)
                // Frame layout: [state(0), cr_status(4), cr_value(8)] = 12 bytes.
                buf.push_str("    (local.set $frame (call $alloc_heap (i32.const 12)))\n");
                buf.push_str("    (i32.store (local.get $frame) (i32.const 0))\n");
                self.emit_gc_activation_frame_push(&mut buf, &frame, 4);
                self.emit_gc_root_param_initializer(&mut buf, &frame, 4);
                buf.push('\n');
            } else {
                self.emit_gc_activation_frame_push(&mut buf, &frame, 4);
                self.emit_gc_root_param_initializer(&mut buf, &frame, 4);
            }
            writer.push_str(&buf);
            buf.clear();
            // Initialize Completion Record status to Normal (0) at function entry.
            let cr_base = frame.cr_local_base();
            writer.line_fmt(4, format_args!("(i32.const 0) (local.set {})", cr_base));
            // Initialize cr_target to TARGET_EMPTY (0).
            writer.line_fmt(4, format_args!("(i32.const 0) (local.set {})", cr_base + 2));
            self.current_is_async.set(is_async);
            let mut loop_ctx = LoopContext::default();
            self.emit_statements(writer, &function.body, 4, &mut loop_ctx, &frame);
            buf.clear();
            if is_async {
                // Close async state=0: gc frame pop
                self.emit_gc_activation_frame_pop(&mut buf, &frame, 4);
                writer.push_str(&buf);
                buf.clear();
                // Async epilogue: check $exception_pending for uncaught throws.
                // If set: store Throw(2) + thrown value to frame[4]/frame[8].
                // If clear: store Normal(0) + UNDEFINED to frame[4]/frame[8].
                // Frame layout: [state(0), cr_status(4), cr_value(8)].
                buf.push_str(&format!(
                    "    (if (global.get $exception_pending)\n\
                     (then\n\
                       (i32.store offset=4 (local.get $frame) (i32.const 2))\n\
                       (i32.store offset=8 (local.get $frame) (global.get $exception_pending))\n\
                       (i32.const 0) (global.set $exception_pending)\n\
                     )\n\
                     (else\n\
                       (i32.store offset=4 (local.get $frame) (i32.const 0))\n\
                       (i32.store offset=8 (local.get $frame) (i32.const {}))\n\
                     )\n\
                   )\n",
                    ValueTag::UNDEFINED,
                ));
                buf.push_str("    (i32.store (local.get $frame) (i32.const 1))\n");
                buf.push_str("    (local.get $frame)\n");
                writer.push_str(&buf);
            } else {
                self.emit_gc_activation_frame_pop(&mut buf, &frame, 4);
                writer.push_str(&buf);
                writer.push_str(&format!("    (i32.const {})\n", ValueTag::UNDEFINED));
            }
            writer.push_str("  )\n");
        }
    }

    pub(super) fn emit_json_replacer_dispatcher(&self, writer: &mut WatWriter) {
        writer.push_str(
            "  (func $json_replacer_call (param $callback i32) (param $holder i32) (param $key i32) (param $value i32) (result i32)\n",
        );
        writer.line(4, "(local $id i32)");
        writer.push_str(&format!(
            "    (if (i32.ne (i32.and (local.get $callback) (i32.const {})) (i32.const {}))\n      (then (return (local.get $value))))\n",
            ValueTag::TAG_MASK,
            ValueTag::NUMBER,
        ));
        writer.line_fmt(
            4,
            format_args!(
                "(local.set $id (i32.shr_s (local.get $callback) (i32.const {})))",
                ValueTag::NUMBER_SHIFT,
            ),
        );

        for function in &self.program.functions {
            writer.line_fmt(
                4,
                format_args!(
                    "(if (i32.eq (local.get $id) (i32.const {})))",
                    function.id.0
                ),
            );
            writer.then(4);
            let mut supplied = 0usize;
            if function.uses_receiver {
                writer.line(8, "(local.get $holder)");
                supplied += 1;
            }
            if supplied < function.params.len() {
                writer.line(8, "(local.get $key)");
                supplied += 1;
            }
            if supplied < function.params.len() {
                writer.line(8, "(local.get $value)");
                supplied += 1;
            }
            for _ in supplied..function.params.len() {
                writer.line_fmt(8, format_args!("(i32.const {})", ValueTag::UNDEFINED));
            }
            writer.line_fmt(
                8,
                format_args!("(return (call ${}))))", function_symbol(function.id)),
            );
        }

        writer.line(4, "(local.get $value))");
    }

    pub(super) fn emit_set_add_dispatcher(&self, writer: &mut WatWriter) {
        writer.push_str(
            "  (func $set_add_dispatch (param $set i32) (param $value i32) (result i32)\n",
        );
        writer.line(4, "(local $callback i32)");
        writer.line(4, "(local $id i32)");
        writer.line(4, "(local.set $callback (global.get $set_prototype_add))");
        writer.line_fmt(
            4,
            format_args!(
                "(if (i32.eq (local.get $callback) (i32.const {native}))",
                native = NATIVE_SET_ADD_SENTINEL,
            ),
        );
        writer.line(
            6,
            "(then (return (call $set_add (local.get $set) (local.get $value))))",
        );
        writer.line_fmt(
            4,
            format_args!(
                "(if (i32.ne (i32.and (local.get $callback) (i32.const {tag_mask})) (i32.const {number_tag}))",
                tag_mask = ValueTag::TAG_MASK,
                number_tag = ValueTag::NUMBER,
            ),
        );
        writer.line_fmt(
            6,
            format_args!(
                "(then (return (i32.const {undefined})))",
                undefined = ValueTag::UNDEFINED
            ),
        );
        writer.line_fmt(
            4,
            format_args!(
                "(local.set $id (i32.shr_s (local.get $callback) (i32.const {number_shift})))",
                number_shift = ValueTag::NUMBER_SHIFT,
            ),
        );

        for function in &self.program.functions {
            writer.line_fmt(
                4,
                format_args!(
                    "(if (i32.eq (local.get $id) (i32.const {})))",
                    function.id.0
                ),
            );
            writer.then(4);
            let mut supplied = 0usize;
            if function.uses_receiver {
                writer.line(8, "(local.get $set)");
                supplied += 1;
            }
            if supplied < function.params.len() {
                writer.line(8, "(local.get $value)");
                supplied += 1;
            }
            for _ in supplied..function.params.len() {
                writer.line_fmt(8, format_args!("(i32.const {})", ValueTag::UNDEFINED));
            }
            writer.line_fmt(
                8,
                format_args!("(return (call ${}))))", function_symbol(function.id)),
            );
        }

        writer.line(4, "(call $set_add (local.get $set) (local.get $value))");
    }
}
