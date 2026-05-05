use super::emitter::LocalFrame;
use super::emitter::WatEmitter;
use super::expr_emit::{
    CLOSURE_CAPTURE_COUNT_OFFSET, CLOSURE_CAPTURE_SLOTS_OFFSET, CLOSURE_CODE_ID_OFFSET,
    CLOSURE_ENV_FLAGS_OFFSET, CLOSURE_SENTINEL,
};
use super::runtime_fn::RuntimeFn;
use std::cell::RefCell;
use ts2wasm_ir::lowered::LocalId;
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
}

impl WatEmitter<'_> {
    pub(super) fn emit_top_level_statements(
        &self,
        wat: &mut String,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        for statement in &self.program.top_level_statements {
            self.emit_statement(wat, statement, indent, &mut LoopContext::default(), frame);
            self.emit_gc_backend_temp_roots_clear(wat, &pad, frame);
            wat.push_str(&format!(
                "{pad}(if (global.get $exception_pending)\n{pad}  (then (unreachable)))\n"
            ));
        }
    }

    pub(super) fn emit_statements(
        &self,
        wat: &mut String,
        statements: &[LoweredStmt],
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        for statement in statements {
            self.emit_statement(wat, statement, indent, loop_ctx, frame);
            self.emit_gc_backend_temp_roots_clear(wat, &pad, frame);
        }
    }

    fn emit_statement(
        &self,
        wat: &mut String,
        statement: &LoweredStmt,
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
    ) {
        self.emit_statement_with_label(wat, statement, indent, loop_ctx, frame, None);
    }

    fn emit_statement_with_label(
        &self,
        wat: &mut String,
        statement: &LoweredStmt,
        indent: usize,
        loop_ctx: &mut LoopContext,
        frame: &LocalFrame,
        bound_label: Option<&str>,
    ) {
        let pad = " ".repeat(indent);
        match statement {
            LoweredStmt::Block(statements) => {
                self.emit_statements(wat, statements, indent, loop_ctx, frame);
            }
            LoweredStmt::Let(local_id, expr) | LoweredStmt::Assign(local_id, expr) => {
                self.emit_expr(wat, expr, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", local_index(*local_id)));
                self.emit_gc_root_mirror(wat, &pad, *local_id, frame);
            }
            LoweredStmt::Expr(expr) => {
                self.emit_expr(wat, expr, indent, frame);
                if self.expr_produces_value(expr) {
                    wat.push_str(&format!("{pad}(drop)\n"));
                }
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.emit_expr(wat, condition, indent, frame);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::TruthyBool.symbol()));
                wat.push_str(&format!("{pad}(if\n"));
                wat.push_str(&format!("{pad}  (then\n"));
                self.emit_statements(wat, then_body, indent + 4, loop_ctx, frame);
                wat.push_str(&format!("{pad}  )\n"));
                if !else_body.is_empty() {
                    wat.push_str(&format!("{pad}  (else\n"));
                    self.emit_statements(wat, else_body, indent + 4, loop_ctx, frame);
                    wat.push_str(&format!("{pad}  )\n"));
                }
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::While { condition, body } => {
                let exit_label = gen_label("while_exit");
                let loop_label = gen_label("while_loop");
                wat.push_str(&format!("{pad}(block ${}\n", exit_label));
                wat.push_str(&format!("{pad}  (loop ${}\n", loop_label));
                self.emit_expr(wat, condition, indent + 4, frame);
                wat.push_str(&format!(
                    "{pad}    (call {})\n",
                    RuntimeFn::TruthyBool.symbol()
                ));
                wat.push_str(&format!("{pad}    (i32.eqz)\n"));
                wat.push_str(&format!("{pad}    (br_if ${})\n", exit_label));

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: exit_label.clone(),
                    continue_label: Some(loop_label.clone()),
                });
                self.emit_statements(wat, body, indent + 4, loop_ctx, frame);
                loop_ctx.pop();

                wat.push_str(&format!("{pad}    (br ${})\n", loop_label));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::Return(expr) => {
                self.emit_expr(wat, expr, indent, frame);
                if frame.uses_activation_roots() {
                    wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp()));
                    self.emit_gc_activation_frame_pop(wat, frame, indent);
                    wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_value_tmp()));
                }
                wat.push_str(&format!("{pad}(return)\n"));
            }
            LoweredStmt::Throw(expr) => {
                // Evaluate the thrown value, store to $exception_pending,
                // then let the enclosing try-catch's br_if catch it.
                self.emit_expr(wat, expr, indent, frame);
                if frame.uses_activation_roots() {
                    // Stack has the thrown value. Save to local, pop GC frame, then set global.
                    wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp()));
                    self.emit_gc_activation_frame_pop(wat, frame, indent);
                    wat.push_str(&format!(
                        "{pad}(global.set $exception_pending (local.get {}))\n",
                        frame.heap_value_tmp(),
                    ));
                } else {
                    // Value is on wasm stack — consume directly.
                    wat.push_str(&format!("{pad}(global.set $exception_pending)\n"));
                }
            }
            LoweredStmt::DoWhile { body, condition } => {
                let exit_label = gen_label("do_exit");
                let loop_label = gen_label("do_loop");
                wat.push_str(&format!("{pad}(block ${}\n", exit_label));
                wat.push_str(&format!("{pad}  (loop ${}\n", loop_label));

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: exit_label.clone(),
                    continue_label: Some(loop_label.clone()),
                });
                self.emit_statements(wat, body, indent + 4, loop_ctx, frame);
                loop_ctx.pop();

                self.emit_expr(wat, condition, indent + 4, frame);
                wat.push_str(&format!(
                    "{pad}    (call {})\n",
                    RuntimeFn::TruthyBool.symbol()
                ));
                wat.push_str(&format!("{pad}    (br_if ${})\n", loop_label));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(i) = init {
                    self.emit_statement(wat, i, indent, loop_ctx, frame);
                }

                let exit_label = gen_label("for_exit");
                let loop_label = gen_label("for_loop");
                let continue_label = gen_label("for_continue");

                wat.push_str(&format!("{pad}(block ${}\n", exit_label));
                wat.push_str(&format!("{pad}  (loop ${}\n", loop_label));

                if let Some(cond) = condition {
                    self.emit_expr(wat, cond, indent + 4, frame);
                    wat.push_str(&format!(
                        "{pad}    (call {})\n",
                        RuntimeFn::TruthyBool.symbol()
                    ));
                    wat.push_str(&format!("{pad}    (i32.eqz)\n"));
                    wat.push_str(&format!("{pad}    (br_if ${})\n", exit_label));
                }

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: exit_label.clone(),
                    continue_label: Some(continue_label.clone()),
                });
                self.emit_statements(wat, body, indent + 4, loop_ctx, frame);
                loop_ctx.pop();

                wat.push_str(&format!("{pad}  (block ${}\n", continue_label));
                if let Some(upd) = update {
                    self.emit_expr(wat, upd, indent + 4, frame);
                    if self.expr_produces_value(upd) {
                        wat.push_str(&format!("{pad}    (drop)\n"));
                    }
                }
                wat.push_str(&format!("{pad}  )\n"));

                wat.push_str(&format!("{pad}    (br ${})\n", loop_label));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::ForIn {
                var,
                iter,
                iter_local,
                index_local,
                len_local,
                body,
            } => {
                let exit_label = gen_label("for_in_exit");
                let loop_label = gen_label("for_in_loop");

                self.emit_expr(wat, iter, indent, frame);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::ObjectKeys.symbol()));
                wat.push_str(&format!("{pad}(local.set {})\n", local_index(*iter_local)));

                wat.push_str(&format!(
                    "{pad}(i32.const {})\n",
                    ValueTag::encode_number(0)
                ));
                wat.push_str(&format!("{pad}(local.set {})\n", local_index(*index_local)));

                wat.push_str(&format!("{pad}(local.get {})\n", local_index(*iter_local)));
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::GetLength.symbol()));
                wat.push_str(&format!("{pad}(local.set {})\n", local_index(*len_local)));

                wat.push_str(&format!("{pad}(block ${}\n", exit_label));
                wat.push_str(&format!("{pad}  (loop ${}\n", loop_label));

                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*index_local)
                ));
                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*len_local)
                ));
                wat.push_str(&format!("{pad}    (call {})\n", RuntimeFn::Less.symbol()));
                wat.push_str(&format!(
                    "{pad}    (call {})\n",
                    RuntimeFn::TruthyBool.symbol()
                ));
                wat.push_str(&format!("{pad}    (i32.eqz)\n"));
                wat.push_str(&format!("{pad}    (br_if ${})\n", exit_label));

                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*iter_local)
                ));
                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*index_local)
                ));
                wat.push_str(&format!(
                    "{pad}    (call {})\n",
                    RuntimeFn::ArrayGet.symbol()
                ));
                wat.push_str(&format!("{pad}    (local.set {})\n", local_index(*var)));

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: exit_label.clone(),
                    continue_label: Some(loop_label.clone()),
                });
                self.emit_statements(wat, body, indent + 4, loop_ctx, frame);
                loop_ctx.pop();

                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*index_local)
                ));
                wat.push_str(&format!(
                    "{pad}    (i32.const {})\n",
                    ValueTag::encode_number(1)
                ));
                wat.push_str(&format!("{pad}    (call {})\n", RuntimeFn::Add.symbol()));
                wat.push_str(&format!(
                    "{pad}    (local.set {})\n",
                    local_index(*index_local)
                ));

                wat.push_str(&format!("{pad}    (br ${})\n", loop_label));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::ForOf {
                var,
                iter,
                iter_local,
                index_local,
                len_local,
                body,
            } => {
                let exit_label = gen_label("for_of_exit");
                let loop_label = gen_label("for_of_loop");

                self.emit_expr(wat, iter, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", local_index(*iter_local)));

                wat.push_str(&format!(
                    "{pad}(i32.const {})\n",
                    ValueTag::encode_number(0)
                ));
                wat.push_str(&format!("{pad}(local.set {})\n", local_index(*index_local)));

                wat.push_str(&format!("{pad}(local.get {})\n", local_index(*iter_local)));
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::GetLength.symbol()));
                wat.push_str(&format!("{pad}(local.set {})\n", local_index(*len_local)));

                wat.push_str(&format!("{pad}(block ${}\n", exit_label));
                wat.push_str(&format!("{pad}  (loop ${}\n", loop_label));

                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*index_local)
                ));
                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*len_local)
                ));
                wat.push_str(&format!("{pad}    (call {})\n", RuntimeFn::Less.symbol()));
                wat.push_str(&format!(
                    "{pad}    (call {})\n",
                    RuntimeFn::TruthyBool.symbol()
                ));
                wat.push_str(&format!("{pad}    (i32.eqz)\n"));
                wat.push_str(&format!("{pad}    (br_if ${})\n", exit_label));

                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*iter_local)
                ));
                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*index_local)
                ));
                wat.push_str(&format!(
                    "{pad}    (call {})\n",
                    RuntimeFn::ArrayGet.symbol()
                ));
                wat.push_str(&format!("{pad}    (local.set {})\n", local_index(*var)));

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: exit_label.clone(),
                    continue_label: Some(loop_label.clone()),
                });
                self.emit_statements(wat, body, indent + 4, loop_ctx, frame);
                loop_ctx.pop();

                wat.push_str(&format!(
                    "{pad}    (local.get {})\n",
                    local_index(*index_local)
                ));
                wat.push_str(&format!(
                    "{pad}    (i32.const {})\n",
                    ValueTag::encode_number(1)
                ));
                wat.push_str(&format!("{pad}    (call {})\n", RuntimeFn::Add.symbol()));
                wat.push_str(&format!(
                    "{pad}    (local.set {})\n",
                    local_index(*index_local)
                ));

                wat.push_str(&format!("{pad}    (br ${})\n", loop_label));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::Labeled { label, body } => {
                if is_loop_stmt(body) {
                    self.emit_statement_with_label(
                        wat,
                        body,
                        indent,
                        loop_ctx,
                        frame,
                        Some(label.as_str()),
                    );
                } else {
                    let exit_label = gen_label("label_exit");
                    wat.push_str(&format!("{pad}(block ${}\n", exit_label));
                    loop_ctx.push(ControlFrame {
                        name: Some(label.clone()),
                        exit_label,
                        continue_label: None,
                    });
                    self.emit_statement(wat, body, indent + 2, loop_ctx, frame);
                    loop_ctx.pop();
                    wat.push_str(&format!("{pad})\n"));
                }
            }
            LoweredStmt::Break { label } => {
                if let Some(target) = loop_ctx.break_label(label.as_deref()) {
                    wat.push_str(&format!("{pad}(br ${target})\n"));
                } else {
                    wat.push_str(&format!("{pad};; ERROR: break outside loop\n"));
                }
            }
            LoweredStmt::Continue { label } => {
                if let Some(target) = loop_ctx.continue_label(label.as_deref()) {
                    wat.push_str(&format!("{pad}(br ${target})\n"));
                } else {
                    wat.push_str(&format!("{pad};; ERROR: continue outside loop\n"));
                }
            }
            LoweredStmt::TryCatch {
                try_body,
                catch_var,
                catch_body,
                finally_body,
            } => {
                // Basic try-catch: wrap in block, execute try, handle catch
                let try_exit = gen_label("try_exit");
                let catch_entry = gen_label("catch_entry");

                wat.push_str(&format!("{pad}(block ${}\n", try_exit));
                wat.push_str(&format!("{pad}  (block ${}\n", catch_entry));
                wat.push_str(&format!(
                    "{pad}    (global.set $exception_handler_depth (i32.add (global.get $exception_handler_depth) (i32.const 1)))\n",
                ));

                for statement in try_body {
                    self.emit_statement(wat, statement, indent + 4, loop_ctx, frame);
                    self.emit_gc_backend_temp_roots_clear(wat, &format!("{pad}    "), frame);
                    wat.push_str(&format!(
                        "{pad}    (br_if ${} (global.get $exception_pending))\n",
                        catch_entry
                    ));
                }

                wat.push_str(&format!(
                    "{pad}    (global.set $exception_handler_depth (i32.sub (global.get $exception_handler_depth) (i32.const 1)))\n",
                ));
                wat.push_str(&format!("{pad}    (br ${})\n", try_exit));
                wat.push_str(&format!("{pad}  )\n"));

                // Catch block (for now, just a placeholder)
                wat.push_str(&format!(
                    "{pad}  (global.set $exception_handler_depth (i32.sub (global.get $exception_handler_depth) (i32.const 1)))\n",
                ));
                if let Some(body) = catch_body {
                    if let Some(var) = catch_var {
                        wat.push_str(&format!("{pad}  (global.get $exception_pending)\n"));
                        wat.push_str(&format!("{pad}  (local.set {})\n", local_index(*var)));
                        self.emit_gc_root_mirror(wat, &format!("{pad}  "), *var, frame);
                    }
                    wat.push_str(&format!(
                        "{pad}  (global.set $exception_pending (i32.const {}))\n",
                        ValueTag::UNDEFINED
                    ));
                    self.emit_statements(wat, body, indent + 4, loop_ctx, frame);
                }

                wat.push_str(&format!("{pad})\n"));

                // Finally block (always executes)
                if let Some(body) = finally_body {
                    self.emit_statements(wat, body, indent + 2, loop_ctx, frame);
                }
            }
            LoweredStmt::Switch { expr, cases } => {
                let switch_exit = gen_label("switch_exit");
                wat.push_str(&format!("{pad}(block ${}\n", switch_exit));

                if cases.is_empty() {
                    self.emit_expr(wat, expr, indent + 2, frame);
                    if self.expr_produces_value(expr) {
                        wat.push_str(&format!("{pad}  (drop)\n"));
                    }
                    wat.push_str(&format!("{pad})\n"));
                    return;
                }

                let case_labels = (0..cases.len())
                    .map(|_| gen_label("switch_case"))
                    .collect::<Vec<_>>();

                for label in case_labels.iter().rev() {
                    wat.push_str(&format!("{pad}  (block ${label}\n"));
                }

                self.emit_expr(wat, expr, indent + 4, frame);
                wat.push_str(&format!(
                    "{pad}    (local.set {})\n",
                    frame.switch_value_tmp()
                ));

                let default_label = cases
                    .iter()
                    .position(|(cond, _)| cond.is_none())
                    .map(|index| case_labels[index].as_str());

                for ((cond, _), label) in cases.iter().zip(case_labels.iter()) {
                    if let Some(c) = cond {
                        wat.push_str(&format!(
                            "{pad}    (local.get {})\n",
                            frame.switch_value_tmp()
                        ));
                        self.emit_expr(wat, c, indent + 4, frame);
                        wat.push_str(&format!(
                            "{pad}    (call {})\n",
                            RuntimeFn::StrictEqual.symbol()
                        ));
                        wat.push_str(&format!("{pad}    (i32.const {})\n", ValueTag::TRUE));
                        wat.push_str(&format!("{pad}    (i32.eq)\n"));
                        wat.push_str(&format!("{pad}    (br_if ${label})\n"));
                    }
                }

                if let Some(label) = default_label {
                    wat.push_str(&format!("{pad}    (br ${label})\n"));
                } else {
                    wat.push_str(&format!("{pad}    (br ${})\n", switch_exit));
                }

                loop_ctx.push(ControlFrame {
                    name: bound_label.map(str::to_owned),
                    exit_label: switch_exit.clone(),
                    continue_label: None,
                });
                for ((_, body), label) in cases.iter().zip(case_labels.iter()) {
                    wat.push_str(&format!("{pad}  ) ;; ${label}\n"));
                    self.emit_statements(wat, body, indent + 2, loop_ctx, frame);
                }
                loop_ctx.pop();
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::Export { name, expr } => {
                let name_ptr = self.string_offset(name) + Layout::STRING_HEADER_SIZE;
                let name_len = name.len() as u32;
                wat.push_str(&format!(
                    "{pad}(i32.const {name_ptr})\n{pad}(i32.const {name_len})\n"
                ));
                self.emit_expr(wat, expr, indent, frame);
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::ModuleExportsSet.symbol(),
                ));
            }
            LoweredStmt::ModuleExportsAssign { expr } => {
                self.emit_expr(wat, expr, indent, frame);
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::ModuleExportsAssign.symbol(),
                ));
            }
            LoweredStmt::ClassDecl {
                constructor,
                methods,
                ..
            } => {
                let constructor_id = constructor.expect("constructor is always Some");
                let proto_global = super::emitter::class_prototype_global(constructor_id);
                for (method_name, func_id) in methods {
                    // Allocate no-capture closure
                    wat.push_str(&format!(
                        "{pad}(local.set {} (call {} (i32.const {})))\n",
                        frame.heap_base_tmp(),
                        RuntimeFn::AllocHeap.symbol(),
                        CLOSURE_CAPTURE_SLOTS_OFFSET,
                    ));
                    self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
                    // Store closure sentinel
                    wat.push_str(&format!(
                        "{pad}(i32.store (local.get {}) (i32.const {CLOSURE_SENTINEL}))\n",
                        frame.heap_base_tmp(),
                    ));
                    // Store func_id
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_CODE_ID_OFFSET})) (i32.const {}))\n",
                        frame.heap_base_tmp(),
                        func_id.0,
                    ));
                    // Store capture_count = 0
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_CAPTURE_COUNT_OFFSET})) (i32.const 0))\n",
                        frame.heap_base_tmp(),
                    ));
                    // Store env_flags = 0
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_ENV_FLAGS_OFFSET})) (i32.const 0))\n",
                        frame.heap_base_tmp(),
                    ));
                    // Tag as object
                    wat.push_str(&format!(
                        "{pad}(local.set {} (i32.or (local.get {}) (i32.const {})))\n",
                        frame.heap_value_tmp(),
                        frame.heap_base_tmp(),
                        ValueTag::OBJECT_TAG,
                    ));
                    // $property_set(tagged_prototype, key_ptr, key_len, tagged_closure)
                    wat.push_str(&format!(
                        "{pad}(i32.or (global.get ${proto_global}) (i32.const {}))\n",
                        ValueTag::OBJECT_TAG,
                    ));
                    let key_offset = self.string_offset(method_name) + Layout::STRING_HEADER_SIZE;
                    let key_len = self.string_len(method_name);
                    wat.push_str(&format!("{pad}(i32.const {key_offset})\n"));
                    wat.push_str(&format!("{pad}(i32.const {key_len})\n"));
                    wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_value_tmp(),));
                    wat.push_str(&format!(
                        "{pad}(call {})\n",
                        RuntimeFn::PropertySet.symbol(),
                    ));
                    wat.push_str(&format!("{pad}(drop)\n"));
                }
            }
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
