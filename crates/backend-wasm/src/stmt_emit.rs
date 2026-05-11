use super::emitter::LocalFrame;
use super::emitter::WatEmitter;
use super::expr_emit::{
    CLOSURE_CAPTURE_COUNT_OFFSET, CLOSURE_CAPTURE_SLOTS_OFFSET, CLOSURE_CODE_ID_OFFSET,
    CLOSURE_ENV_FLAGS_OFFSET, CLOSURE_SENTINEL,
};
use super::runtime_fn::RuntimeFn;
use super::wat_writer::WatWriter;
use std::cell::RefCell;
use ts2wasm_ir::lowered::FuncId;
use ts2wasm_ir::lowered::LocalId;
use ts2wasm_ir::lowered::LoweredExpr;
use ts2wasm_ir::lowered::LoweredStmt;
use ts2wasm_runtime_abi::Layout;
use ts2wasm_runtime_abi::ValueTag;

thread_local! {
    static LABEL_COUNTER: RefCell<usize> = const { RefCell::new(0) };
}

fn gen_label(prefix: &str) -> String {
    LABEL_COUNTER.with(|c| {
        let mut counter = c.borrow_mut();
        let num = *counter;
        *counter += 1;
        format!("{}_{}", prefix, num)
    })
}

#[derive(Default)]
pub(crate) struct LoopContext {
    frames: Vec<ControlFrame>,
    /// Stack of try-finally exit labels. When inside a try with finally,
    /// Return should br to this label instead of doing a direct wasm return.
    try_finally_exit_labels: Vec<String>,
}

struct ControlFrame {
    name: Option<String>,
    exit_label: String,
    continue_label: Option<String>,
}

impl LoopContext {
    fn push(&mut self, frame: ControlFrame) {
        self.frames.push(frame);
    }

    fn pop(&mut self) {
        self.frames.pop();
    }

    fn break_label(&self, name: Option<&str>) -> Option<&str> {
        match name {
            Some(name) => self
                .frames
                .iter()
                .rev()
                .find(|frame| frame.name.as_deref() == Some(name))
                .map(|frame| frame.exit_label.as_str()),
            None => self.frames.last().map(|frame| frame.exit_label.as_str()),
        }
    }

    fn continue_label(&self, name: Option<&str>) -> Option<&str> {
        self.frames.iter().rev().find_map(|frame| {
            if name.is_none() || frame.name.as_deref() == name {
                frame.continue_label.as_deref()
            } else {
                None
            }
        })
    }

    fn push_try_finally(&mut self, label: String) {
        self.try_finally_exit_labels.push(label);
    }

    fn pop_try_finally(&mut self) {
        self.try_finally_exit_labels.pop();
    }

    fn has_try_finally(&self) -> bool {
        !self.try_finally_exit_labels.is_empty()
    }

    fn try_finally_exit(&self) -> Option<&str> {
        self.try_finally_exit_labels.last().map(|s| s.as_str())
    }
}

impl WatEmitter<'_> {
    pub(super) fn emit_top_level_statements(
        &self,
        writer: &mut WatWriter,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        for statement in &self.program.top_level_statements {
            self.emit_statement(
                writer,
                statement,
                indent,
                &mut LoopContext::default(),
                frame,
            );
            let mut buf = String::new();
            self.emit_gc_backend_temp_roots_clear(&mut buf, &pad, frame);
            writer.push_str(buf.as_str());
            writer.line(indent, "(if (global.get $exception_pending)");
            writer.line(indent + 2, "(then");
            writer.line(indent + 4, "(call $wasi_proc_exit (i32.const 1))");
            writer.line(indent + 4, "(unreachable)");
            writer.line(indent + 2, "))");
        }
    }

    pub(super) fn emit_statements(
        &self,
        writer: &mut WatWriter,
        statements: &[LoweredStmt],
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        for statement in statements {
            self.emit_statement(writer, statement, indent, loop_ctx, frame);
            let mut buf = String::new();
            self.emit_gc_backend_temp_roots_clear(&mut buf, &pad, frame);
            writer.push_str(buf.as_str());
        }
    }

    fn emit_statement(
        &self,
        writer: &mut WatWriter,
        statement: &LoweredStmt,
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
    ) {
        self.emit_statement_with_label(writer, statement, indent, loop_ctx, frame, None);
    }

    fn emit_statement_with_label(
        &self,
        writer: &mut WatWriter,
        statement: &LoweredStmt,
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
        bound_label: Option<&str>,
    ) {
        match statement {
            LoweredStmt::Block(statements, _) => {
                self.emit_statements(writer, statements, indent, loop_ctx, frame);
            }
            LoweredStmt::Let(local_id, expr, _) | LoweredStmt::Assign(local_id, expr, _) => {
                let pad = " ".repeat(indent);
                self.emit_expr(writer, expr, indent, frame);
                writer.local_set(indent, local_index(*local_id));
                let mut buf = String::new();
                self.emit_gc_root_mirror(&mut buf, &pad, *local_id, frame);
                writer.push_str(buf.as_str());
            }
            LoweredStmt::Expr(expr, _) => {
                self.emit_expr(writer, expr, indent, frame);
                if self.expr_produces_value(expr) {
                    writer.drop(indent);
                }
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.emit_expr(writer, condition, indent, frame);
                writer.call(indent, RuntimeFn::TruthyBool.symbol());
                writer.r#if(indent);
                writer.then(indent);
                self.emit_statements(writer, then_body, indent + 4, loop_ctx, frame);
                writer.end(indent);
                if !else_body.is_empty() {
                    writer.r#else(indent);
                    self.emit_statements(writer, else_body, indent + 4, loop_ctx, frame);
                    writer.end(indent);
                }
                writer.end(indent);
            }
            LoweredStmt::While {
                condition, body, ..
            } => {
                let exit_label = gen_label("while_exit");
                let loop_label = gen_label("while_loop");
                writer.block(indent, &exit_label);
                writer.r#loop(indent + 2, &loop_label);
                self.emit_expr(writer, condition, indent + 4, frame);
                writer.line_fmt(
                    indent + 4,
                    format_args!("(call {})", RuntimeFn::TruthyBool.symbol()),
                );
                writer.i32_eqz(indent + 4);
                writer.br_if(indent + 4, &exit_label);

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: exit_label.clone(),
                    continue_label: Some(loop_label.clone()),
                });
                self.emit_statements(writer, body, indent + 4, loop_ctx, frame);
                loop_ctx.pop();

                writer.r#br(indent + 4, &loop_label);
                writer.end(indent + 2);
                writer.end(indent);
            }
            LoweredStmt::Return(expr, _) => {
                self.emit_return_statement(writer, expr, indent, loop_ctx, frame);
            }
            LoweredStmt::Throw(expr, _) => {
                self.emit_throw_statement(writer, expr, indent, loop_ctx, frame);
            }
            LoweredStmt::DoWhile {
                body, condition, ..
            } => {
                let exit_label = gen_label("do_exit");
                let loop_label = gen_label("do_loop");
                writer.block(indent, &exit_label);
                writer.r#loop(indent + 2, &loop_label);

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: exit_label.clone(),
                    continue_label: Some(loop_label.clone()),
                });
                self.emit_statements(writer, body, indent + 4, loop_ctx, frame);
                loop_ctx.pop();

                self.emit_expr(writer, condition, indent + 4, frame);
                writer.line_fmt(
                    indent + 4,
                    format_args!("(call {})", RuntimeFn::TruthyBool.symbol()),
                );
                writer.br_if(indent + 4, &loop_label);
                writer.end(indent + 2);
                writer.end(indent);
            }
            LoweredStmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                self.emit_for_statement(
                    writer,
                    init,
                    condition,
                    update,
                    body,
                    indent,
                    loop_ctx,
                    frame,
                    bound_label,
                );
            }
            LoweredStmt::ForIn {
                var,
                iter,
                iter_local,
                index_local,
                len_local,
                body,
                ..
            } => {
                let exit_label = gen_label("for_in_exit");
                let loop_label = gen_label("for_in_loop");

                self.emit_expr(writer, iter, indent, frame);
                writer.call(indent, RuntimeFn::ObjectKeys.symbol());
                writer.local_set(indent, local_index(*iter_local));

                writer.i32_const(indent, ValueTag::encode_number(0));
                writer.local_set(indent, local_index(*index_local));

                writer.local_get(indent, local_index(*iter_local));
                writer.call(indent, RuntimeFn::GetLength.symbol());
                writer.local_set(indent, local_index(*len_local));

                writer.block(indent, &exit_label);
                writer.r#loop(indent + 2, &loop_label);

                writer.local_get(indent + 4, local_index(*index_local));
                writer.local_get(indent + 4, local_index(*len_local));
                writer.call(indent + 4, RuntimeFn::Less.symbol());
                writer.call(indent + 4, RuntimeFn::TruthyBool.symbol());
                writer.i32_eqz(indent + 4);
                writer.br_if(indent + 4, &exit_label);

                writer.local_get(indent + 4, local_index(*iter_local));
                writer.local_get(indent + 4, local_index(*index_local));
                writer.call(indent + 4, RuntimeFn::ArrayGet.symbol());
                writer.local_set(indent + 4, local_index(*var));

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: exit_label.clone(),
                    continue_label: Some(loop_label.clone()),
                });
                self.emit_statements(writer, body, indent + 4, loop_ctx, frame);
                loop_ctx.pop();

                writer.local_get(indent + 4, local_index(*index_local));
                writer.i32_const(indent + 4, ValueTag::encode_number(1));
                writer.call(indent + 4, RuntimeFn::Add.symbol());
                writer.local_set(indent + 4, local_index(*index_local));

                writer.r#br(indent + 4, &loop_label);
                writer.end(indent + 2);
                writer.end(indent);
            }
            LoweredStmt::ForOf {
                var,
                iter,
                iter_local,
                index_local,
                len_local,
                body,
                ..
            } => {
                let exit_label = gen_label("for_of_exit");
                let loop_label = gen_label("for_of_loop");

                self.emit_expr(writer, iter, indent, frame);
                writer.local_set(indent, local_index(*iter_local));

                writer.i32_const(indent, ValueTag::encode_number(0));
                writer.local_set(indent, local_index(*index_local));

                writer.local_get(indent, local_index(*iter_local));
                writer.call(indent, RuntimeFn::GetLength.symbol());
                writer.local_set(indent, local_index(*len_local));

                writer.block(indent, &exit_label);
                writer.r#loop(indent + 2, &loop_label);

                writer.local_get(indent + 4, local_index(*index_local));
                writer.local_get(indent + 4, local_index(*len_local));
                writer.call(indent + 4, RuntimeFn::Less.symbol());
                writer.call(indent + 4, RuntimeFn::TruthyBool.symbol());
                writer.i32_eqz(indent + 4);
                writer.br_if(indent + 4, &exit_label);

                writer.local_get(indent + 4, local_index(*iter_local));
                writer.local_get(indent + 4, local_index(*index_local));
                writer.call(indent + 4, RuntimeFn::ArrayGet.symbol());
                writer.local_set(indent + 4, local_index(*var));

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: exit_label.clone(),
                    continue_label: Some(loop_label.clone()),
                });
                self.emit_statements(writer, body, indent + 4, loop_ctx, frame);
                loop_ctx.pop();

                writer.local_get(indent + 4, local_index(*index_local));
                writer.i32_const(indent + 4, ValueTag::encode_number(1));
                writer.call(indent + 4, RuntimeFn::Add.symbol());
                writer.local_set(indent + 4, local_index(*index_local));

                writer.r#br(indent + 4, &loop_label);
                writer.end(indent + 2);
                writer.end(indent);
            }
            LoweredStmt::Labeled { label, body, .. } => {
                if is_loop_stmt(body) {
                    self.emit_statement_with_label(
                        writer,
                        body,
                        indent,
                        loop_ctx,
                        frame,
                        Some(label.as_str()),
                    );
                } else {
                    let exit_label = gen_label("label_exit");
                    writer.block(indent, &exit_label);
                    loop_ctx.push(ControlFrame {
                        name: Some(label.clone()),
                        exit_label,
                        continue_label: None,
                    });
                    self.emit_statement(writer, body, indent + 2, loop_ctx, frame);
                    loop_ctx.pop();
                    writer.end(indent);
                }
            }
            LoweredStmt::Break { label, .. } => {
                if loop_ctx.has_try_finally() {
                    // Save to CR and br through finally
                    let cr_base = frame.cr_local_base();
                    writer.line_fmt(
                        indent,
                        format_args!("(i32.const 3) (local.set {})", cr_base),
                    ); // cr_status=Break
                    writer.line_fmt(
                        indent,
                        format_args!("(i32.const 0) (local.set {})", cr_base + 2),
                    ); // cr_target
                    if let Some(exit) = loop_ctx.try_finally_exit() {
                        writer.line(indent, &format!("(br ${})", exit));
                    }
                } else if let Some(target) = loop_ctx.break_label(label.as_deref()) {
                    writer.r#br(indent, target);
                } else {
                    writer.line(indent, ";; ERROR: break outside loop");
                }
            }
            LoweredStmt::Continue { label, .. } => {
                if loop_ctx.has_try_finally() {
                    // Save to CR and br through finally
                    let cr_base = frame.cr_local_base();
                    writer.line_fmt(
                        indent,
                        format_args!("(i32.const 4) (local.set {})", cr_base),
                    ); // cr_status=Continue
                    writer.line_fmt(
                        indent,
                        format_args!("(i32.const 0) (local.set {})", cr_base + 2),
                    ); // cr_target
                    if let Some(exit) = loop_ctx.try_finally_exit() {
                        writer.line(indent, &format!("(br ${})", exit));
                    }
                } else if let Some(target) = loop_ctx.continue_label(label.as_deref()) {
                    writer.r#br(indent, target);
                } else {
                    writer.line(indent, ";; ERROR: continue outside loop");
                }
            }
            LoweredStmt::TryCatch {
                try_body,
                catch_var,
                catch_body,
                finally_body,
                ..
            } => {
                self.emit_try_catch_statement(
                    writer,
                    try_body,
                    catch_var,
                    catch_body,
                    finally_body,
                    indent,
                    loop_ctx,
                    frame,
                );
            }
            LoweredStmt::Switch { expr, cases, .. } => {
                self.emit_switch_statement(
                    writer,
                    expr,
                    cases,
                    indent,
                    loop_ctx,
                    frame,
                    bound_label,
                );
            }
            LoweredStmt::Export { name, expr, .. } => {
                let name_ptr = self.string_offset(name) + Layout::STRING_HEADER_SIZE;
                let name_len = name.len() as u32;
                writer.line_fmt(indent, format_args!("(i32.const {name_ptr})"));
                writer.line_fmt(indent, format_args!("(i32.const {name_len})"));
                self.emit_expr(writer, expr, indent, frame);
                writer.call(indent, RuntimeFn::ModuleExportsSet.symbol());
            }
            LoweredStmt::ModuleExportsAssign { expr, .. } => {
                self.emit_expr(writer, expr, indent, frame);
                writer.call(indent, RuntimeFn::ModuleExportsAssign.symbol());
            }
            LoweredStmt::ClassDecl {
                constructor,
                methods,
                ..
            } => {
                self.emit_class_decl_statement(writer, constructor, methods, indent, frame);
            }
        }
    }

    fn emit_return_statement(
        &self,
        writer: &mut WatWriter,
        expr: &LoweredExpr,
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
    ) {
        self.emit_expr(writer, expr, indent, frame);
        // Save value to Completion Record and set status=Return(1), target=TARGET_EMPTY(0).
        let cr_base = frame.cr_local_base();
        writer.line_fmt(indent, format_args!("(local.set {})", cr_base + 1)); // cr_value
        writer.line_fmt(
            indent,
            format_args!("(i32.const 1) (local.set {})", cr_base),
        ); // cr_status=Return
        writer.line_fmt(
            indent,
            format_args!("(i32.const 0) (local.set {})", cr_base + 2),
        ); // cr_target
        if loop_ctx.has_try_finally() {
            // Inside try-finally: br to finally exit instead of direct wasm return.
            // The finally block will restore the CR and then re-branch to propagate.
            if let Some(exit) = loop_ctx.try_finally_exit() {
                writer.line_fmt(indent, format_args!("(local.get {})", cr_base + 1)); // restore for br
                writer.line(indent, &format!("(br ${})", exit));
            }
        } else {
            // Clear any pending exception before returning — a finally body's
            // return may override a preceding throw, and the caller must not see it.
            writer.line_fmt(
                indent,
                format_args!("(i32.const 0) (global.set $exception_pending)"),
            );
            if self.current_is_async.get() {
                // Save CR status/value to async frame before gc pop.
                // Frame: [state(0), cr_status(4), cr_value(8)].
                writer.line_fmt(
                    indent,
                    format_args!(
                        "(i32.store offset=4 (local.get $frame) (local.get {}))",
                        cr_base
                    ),
                );
                writer.line_fmt(
                    indent,
                    format_args!(
                        "(i32.store offset=8 (local.get $frame) (local.get {}))",
                        cr_base + 1
                    ),
                );
            }
            if frame.uses_activation_roots() {
                let mut buf = String::new();
                self.emit_gc_activation_frame_pop(&mut buf, frame, indent);
                writer.push_str(buf.as_str());
            }
            if self.current_is_async.get() {
                writer.line(indent, "(i32.store (local.get $frame) (i32.const 1))");
                writer.line(indent, "(local.get $frame)");
            } else {
                writer.line_fmt(indent, format_args!("(local.get {})", cr_base + 1)); // restore for return
            }
            writer.return_(indent);
        }
    }

    fn emit_throw_statement(
        &self,
        writer: &mut WatWriter,
        expr: &LoweredExpr,
        indent: usize,
        _loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
    ) {
        // Evaluate the thrown value, save to Completion Record,
        // then store to $exception_pending for enclosing try-catch.
        self.emit_expr(writer, expr, indent, frame);
        // Save value to Completion Record and set status=Throw(2), target=TARGET_EMPTY(0).
        let cr_base = frame.cr_local_base();
        writer.line_fmt(indent, format_args!("(local.set {})", cr_base + 1)); // cr_value
        writer.line_fmt(
            indent,
            format_args!("(i32.const 2) (local.set {})", cr_base),
        ); // cr_status=Throw
        writer.line_fmt(
            indent,
            format_args!("(i32.const 0) (local.set {})", cr_base + 2),
        ); // cr_target
        writer.line_fmt(indent, format_args!("(local.get {})", cr_base + 1)); // restore for throw
        if frame.uses_activation_roots() {
            // Stack has the thrown value. Save to local, pop GC frame, then set global.
            writer.local_set(indent, frame.heap_value_tmp());
            let mut buf = String::new();
            self.emit_gc_activation_frame_pop(&mut buf, frame, indent);
            writer.push_str(buf.as_str());
            writer.line_fmt(
                indent,
                format_args!(
                    "(global.set $exception_pending (local.get {}))",
                    frame.heap_value_tmp(),
                ),
            );
        } else {
            // Value is on wasm stack — consume directly.
            writer.line(indent, "(global.set $exception_pending)");
        }
    }

    fn emit_for_statement(
        &self,
        writer: &mut WatWriter,
        init: &Option<Box<LoweredStmt>>,
        condition: &Option<LoweredExpr>,
        update: &Option<LoweredExpr>,
        body: &[LoweredStmt],
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
        bound_label: Option<&str>,
    ) {
        if let Some(i) = init {
            self.emit_statement(writer, i, indent, loop_ctx, frame);
        }

        let exit_label = gen_label("for_exit");
        let loop_label = gen_label("for_loop");
        let continue_label = gen_label("for_continue");

        writer.block(indent, &exit_label);
        writer.r#loop(indent + 2, &loop_label);
        // Continue target: block that ENCLOSES the body so br $for_continue works
        writer.block(indent + 4, &continue_label);

        if let Some(cond) = condition {
            self.emit_expr(writer, cond, indent + 6, frame);
            writer.line_fmt(
                indent + 6,
                format_args!("(call {})", RuntimeFn::TruthyBool.symbol()),
            );
            writer.i32_eqz(indent + 6);
            writer.br_if(indent + 6, &exit_label);
        }

        loop_ctx.push(ControlFrame {
            name: bound_label.map(str::to_owned),
            exit_label: exit_label.clone(),
            continue_label: Some(continue_label.clone()),
        });
        self.emit_statements(writer, body, indent + 6, loop_ctx, frame);
        loop_ctx.pop();

        writer.end(indent + 4); // end $for_continue block

        if let Some(upd) = update {
            self.emit_expr(writer, upd, indent + 4, frame);
            if self.expr_produces_value(upd) {
                writer.drop(indent + 4);
            }
        }

        writer.r#br(indent + 4, &loop_label);
        writer.end(indent + 2); // end loop
        writer.end(indent); // end block
    }

    fn emit_try_catch_statement(
        &self,
        writer: &mut WatWriter,
        try_body: &[LoweredStmt],
        catch_var: &Option<LocalId>,
        catch_body: &Option<Vec<LoweredStmt>>,
        finally_body: &Option<Vec<LoweredStmt>>,
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let try_exit = gen_label("try_exit");
        let catch_entry = gen_label("catch_entry");
        let has_finally = finally_body.is_some();

        // When finally exists, wrap try-catch in a try_catch_done block
        // so Return inside try can br to it (skip catch, go to finally).
        let (_outer_indent, inner_indent, _tc_done) = if has_finally {
            let label = gen_label("try_catch_done");
            loop_ctx.push_try_finally(label.clone());
            writer.block(indent, &label);
            (indent, indent + 2, Some(label))
        } else {
            (indent, indent, None)
        };

        writer.block(inner_indent, &try_exit);
        writer.block(inner_indent + 2, &catch_entry);
        writer.line_fmt(
            inner_indent + 4,
            format_args!(
                "(global.set $exception_handler_depth (i32.add (global.get $exception_handler_depth) (i32.const 1)))",
            ),
        );

        let mut buf = String::new();
        for statement in try_body {
            self.emit_statement(writer, statement, inner_indent + 4, loop_ctx, frame);
            buf.clear();
            self.emit_gc_backend_temp_roots_clear(&mut buf, &format!("{pad}    "), frame);
            writer.push_str(buf.as_str());
            buf.clear();
            writer.line_fmt(
                inner_indent + 4,
                format_args!("(br_if ${} (global.get $exception_pending))", catch_entry),
            );
        }

        writer.line_fmt(
            inner_indent + 4,
            format_args!(
                "(global.set $exception_handler_depth (i32.sub (global.get $exception_handler_depth) (i32.const 1)))",
            ),
        );
        writer.r#br(inner_indent + 4, &try_exit);
        writer.end(inner_indent + 2);

        // Catch block
        writer.line_fmt(
            inner_indent + 2,
            format_args!(
                "(global.set $exception_handler_depth (i32.sub (global.get $exception_handler_depth) (i32.const 1)))",
            ),
        );
        if let Some(body) = catch_body {
            // Reset CR to Normal(0) — the exception was caught.
            let cr_base = frame.cr_local_base();
            writer.line_fmt(
                inner_indent + 2,
                format_args!("(i32.const 0) (local.set {})", cr_base),
            );
            if let Some(var) = catch_var {
                buf.clear();
                writer.line(inner_indent + 2, "(global.get $exception_pending)");
                writer.local_set(inner_indent + 2, local_index(*var));
                self.emit_gc_root_mirror(&mut buf, &format!("{pad}  "), *var, frame);
                writer.push_str(buf.as_str());
            }
            buf.clear();
            writer.line_fmt(
                inner_indent + 2,
                format_args!(
                    "(global.set $exception_pending (i32.const {}))",
                    ValueTag::UNDEFINED
                ),
            );
            self.emit_statements(writer, body, inner_indent + 4, loop_ctx, frame);
        }

        writer.end(inner_indent);

        if has_finally {
            writer.end(indent); // end try_catch_done block
            loop_ctx.pop_try_finally();

            // Finally body with CR save/restore
            let cr_base = frame.cr_local_base();
            let save_base = frame.cr_save_local_base();
            // SAVE pre-finally CR
            writer.line_fmt(
                indent,
                format_args!("(local.set {} (local.get {}))", save_base, cr_base),
            ); // saved_status = cr_status
            writer.line_fmt(
                indent,
                format_args!("(local.set {} (local.get {}))", save_base + 1, cr_base + 1),
            ); // saved_value = cr_value
            // EMIT finally body (may overwrite CR)
            self.emit_statements(
                writer,
                finally_body.as_ref().unwrap(),
                indent,
                loop_ctx,
                frame,
            );
            // RESTORE if finally was Normal
            writer.line_fmt(
                indent,
                format_args!("(if (i32.eq (local.get {}) (i32.const 0))", cr_base),
            );
            writer.line(indent + 2, "(then");
            writer.line_fmt(
                indent + 4,
                format_args!("(local.set {} (local.get {}))", cr_base, save_base),
            );
            writer.line_fmt(
                indent + 4,
                format_args!("(local.set {} (local.get {}))", cr_base + 1, save_base + 1),
            );
            writer.line(indent + 2, ")");
            writer.end(indent); // if

            // CR dispatch: if Return(1), do wasm return
            writer.line_fmt(
                indent,
                format_args!("(if (i32.eq (local.get {}) (i32.const 1))", cr_base),
            );
            writer.line_fmt(
                indent + 2,
                format_args!("(then (local.get {}) (return))", cr_base + 1),
            );
            writer.end(indent); // if

            // CR dispatch: if Throw(2), set $exception_pending
            writer.line_fmt(
                indent,
                format_args!("(if (i32.eq (local.get {}) (i32.const 2))", cr_base),
            );
            writer.line_fmt(
                indent + 2,
                format_args!(
                    "(then (local.get {}) (global.set $exception_pending))",
                    cr_base + 1
                ),
            );
            writer.end(indent); // if

            // CR dispatch: if Break(3), br to enclosing block exit
            if let Some(exit_label) = loop_ctx.break_label(None) {
                writer.line_fmt(
                    indent,
                    format_args!("(if (i32.eq (local.get {}) (i32.const 3))", cr_base),
                );
                writer.line(indent + 2, &format!("(then (br ${}))", exit_label));
                writer.end(indent); // if
            }

            // CR dispatch: if Continue(4), br to loop continue
            if let Some(cont_label) = loop_ctx.continue_label(None) {
                writer.line_fmt(
                    indent,
                    format_args!("(if (i32.eq (local.get {}) (i32.const 4))", cr_base,),
                );
                writer.line(indent + 2, &format!("(then (br ${}))", cont_label));
                writer.end(indent); // if
            }
        }
    }

    fn emit_switch_statement(
        &self,
        writer: &mut WatWriter,
        expr: &LoweredExpr,
        cases: &[(Option<LoweredExpr>, Vec<LoweredStmt>)],
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
        bound_label: Option<&str>,
    ) {
        let switch_exit = gen_label("switch_exit");
        writer.block(indent, &switch_exit);

        if cases.is_empty() {
            self.emit_expr(writer, expr, indent + 2, frame);
            if self.expr_produces_value(expr) {
                writer.drop(indent + 2);
            }
            writer.end(indent);
            return;
        }

        let case_labels = (0..cases.len())
            .map(|_| gen_label("switch_case"))
            .collect::<Vec<_>>();

        for label in case_labels.iter().rev() {
            writer.line(indent + 2, &format!("(block ${label}"));
        }

        self.emit_expr(writer, expr, indent + 4, frame);
        writer.line_fmt(
            indent + 4,
            format_args!("(local.set {})", frame.switch_value_tmp()),
        );

        let default_label = cases
            .iter()
            .position(|(cond, _)| cond.is_none())
            .map(|index| case_labels[index].as_str());

        for ((cond, _), label) in cases.iter().zip(case_labels.iter()) {
            if let Some(c) = cond {
                writer.line_fmt(
                    indent + 4,
                    format_args!("(local.get {})", frame.switch_value_tmp()),
                );
                self.emit_expr(writer, c, indent + 4, frame);
                writer.line_fmt(
                    indent + 4,
                    format_args!("(call {})", RuntimeFn::StrictEqual.symbol()),
                );
                writer.i32_const(indent + 4, ValueTag::TRUE);
                writer.i32_eq(indent + 4);
                writer.br_if(indent + 4, label);
            }
        }

        if let Some(label) = default_label {
            writer.r#br(indent + 4, label);
        } else {
            writer.r#br(indent + 4, &switch_exit);
        }

        loop_ctx.push(ControlFrame {
            name: bound_label.map(str::to_owned),
            exit_label: switch_exit.clone(),
            continue_label: None,
        });
        for ((_, body), label) in cases.iter().zip(case_labels.iter()) {
            writer.line(indent + 2, &format!(") ;; ${label}"));
            self.emit_statements(writer, body, indent + 2, loop_ctx, frame);
        }
        loop_ctx.pop();
        writer.end(indent);
    }

    fn emit_class_decl_statement(
        &self,
        writer: &mut WatWriter,
        constructor: &Option<FuncId>,
        methods: &[(String, FuncId)],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let constructor_id = constructor.expect("constructor is always Some");
        let proto_global = super::emitter::class_prototype_global(constructor_id);
        let mut buf = String::new();
        for (method_name, func_id) in methods {
            // Allocate no-capture closure
            writer.line_fmt(
                indent,
                format_args!(
                    "(local.set {} (call {} (i32.const {})))",
                    frame.heap_base_tmp(),
                    RuntimeFn::AllocHeap.symbol(),
                    CLOSURE_CAPTURE_SLOTS_OFFSET,
                ),
            );
            buf.clear();
            self.emit_gc_root_mirror_index(&mut buf, &pad, frame.heap_base_tmp(), frame);
            writer.push_str(buf.as_str());
            // Store closure sentinel
            writer.line_fmt(
                indent,
                format_args!(
                    "(i32.store (local.get {}) (i32.const {CLOSURE_SENTINEL}))",
                    frame.heap_base_tmp(),
                ),
            );
            // Store func_id
            writer.line_fmt(
                indent,
                format_args!(
                    "(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_CODE_ID_OFFSET})) (i32.const {}))",
                    frame.heap_base_tmp(),
                    func_id.0,
                ),
            );
            // Store capture_count = 0
            writer.line_fmt(
                indent,
                format_args!(
                    "(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_CAPTURE_COUNT_OFFSET})) (i32.const 0))",
                    frame.heap_base_tmp(),
                ),
            );
            // Store env_flags = 0
            writer.line_fmt(
                indent,
                format_args!(
                    "(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_ENV_FLAGS_OFFSET})) (i32.const 0))",
                    frame.heap_base_tmp(),
                ),
            );
            // Tag as object
            writer.line_fmt(
                indent,
                format_args!(
                    "(local.set {} (i32.or (local.get {}) (i32.const {})))",
                    frame.heap_value_tmp(),
                    frame.heap_base_tmp(),
                    ValueTag::OBJECT_TAG,
                ),
            );
            // $property_set(tagged_prototype, key_ptr, key_len, tagged_closure)
            writer.line_fmt(
                indent,
                format_args!(
                    "(i32.or (global.get ${proto_global}) (i32.const {}))",
                    ValueTag::OBJECT_TAG,
                ),
            );
            let key_offset = self.string_offset(method_name) + Layout::STRING_HEADER_SIZE;
            let key_len = self.string_len(method_name);
            writer.line_fmt(indent, format_args!("(i32.const {key_offset})"));
            writer.line_fmt(indent, format_args!("(i32.const {key_len})"));
            writer.line_fmt(
                indent,
                format_args!("(local.get {})", frame.heap_value_tmp()),
            );
            writer.call(indent, RuntimeFn::PropertySet.symbol());
            writer.drop(indent);
        }
    }
}

fn local_index(id: LocalId) -> usize {
    id.0
}

fn is_loop_stmt(statement: &LoweredStmt) -> bool {
    matches!(
        statement,
        LoweredStmt::While { .. }
            | LoweredStmt::DoWhile { .. }
            | LoweredStmt::For { .. }
            | LoweredStmt::ForIn { .. }
            | LoweredStmt::ForOf { .. }
    )
}
