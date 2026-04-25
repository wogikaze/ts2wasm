use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;
use crate::ir::lowered::LocalId;
use crate::ir::lowered::LoweredStmt;
use std::cell::RefCell;

thread_local! {
    static LABEL_COUNTER: RefCell<usize> = RefCell::new(0);
}

fn gen_label(prefix: &str) -> String {
    LABEL_COUNTER.with(|c| {
        let mut counter = c.borrow_mut();
        let num = *counter;
        *counter += 1;
        format!("{}_{}", prefix, num)
    })
}

pub(crate) enum LoopContext {
    Root,
    Loop {
        exit_label: String,
        continue_label: String,
    },
}

impl WatEmitter<'_> {
    pub(super) fn emit_top_level_statements(&self, wat: &mut String, indent: usize) {
        for statement in &self.program.top_level_statements {
            self.emit_statement(wat, statement, indent, &mut LoopContext::Root);
        }
    }

    pub(super) fn emit_statements(
        &self,
        wat: &mut String,
        statements: &[LoweredStmt],
        indent: usize,
        loop_ctx: &mut LoopContext,
    ) {
        for statement in statements {
            self.emit_statement(wat, statement, indent, loop_ctx);
        }
    }

    fn emit_statement(
        &self,
        wat: &mut String,
        statement: &LoweredStmt,
        indent: usize,
        loop_ctx: &mut LoopContext,
    ) {
        let pad = " ".repeat(indent);
        match statement {
            LoweredStmt::Let(local_id, expr) | LoweredStmt::Assign(local_id, expr) => {
                self.emit_expr(wat, expr, indent);
                wat.push_str(&format!("{pad}(local.set {})\n", local_index(*local_id)));
            }
            LoweredStmt::Expr(expr) => {
                self.emit_expr(wat, expr, indent);
                if self.expr_produces_value(expr) {
                    wat.push_str(&format!("{pad}(drop)\n"));
                }
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.emit_expr(wat, condition, indent);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::TruthyBool.symbol()));
                wat.push_str(&format!("{pad}(if\n"));
                wat.push_str(&format!("{pad}  (then\n"));
                self.emit_statements(wat, then_body, indent + 4, loop_ctx);
                wat.push_str(&format!("{pad}  )\n"));
                if !else_body.is_empty() {
                    wat.push_str(&format!("{pad}  (else\n"));
                    self.emit_statements(wat, else_body, indent + 4, loop_ctx);
                    wat.push_str(&format!("{pad}  )\n"));
                }
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::While { condition, body } => {
                let exit_label = gen_label("while_exit");
                let loop_label = gen_label("while_loop");
                wat.push_str(&format!("{pad}(block ${}\n", exit_label));
                wat.push_str(&format!("{pad}  (loop ${}\n", loop_label));
                self.emit_expr(wat, condition, indent + 4);
                wat.push_str(&format!(
                    "{pad}    (call {})\n",
                    RuntimeFn::TruthyBool.symbol()
                ));
                wat.push_str(&format!("{pad}    (i32.eqz)\n"));
                wat.push_str(&format!("{pad}    (br_if ${})\n", exit_label));

                let mut new_ctx = LoopContext::Loop {
                    exit_label: exit_label.clone(),
                    continue_label: loop_label.clone(),
                };
                self.emit_statements(wat, body, indent + 4, &mut new_ctx);

                wat.push_str(&format!("{pad}    (br ${})\n", loop_label));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::Return(expr) => {
                self.emit_expr(wat, expr, indent);
                wat.push_str(&format!("{pad}(return)\n"));
            }
            LoweredStmt::DoWhile { body, condition } => {
                let exit_label = gen_label("do_exit");
                let loop_label = gen_label("do_loop");
                wat.push_str(&format!("{pad}(block ${}\n", exit_label));
                wat.push_str(&format!("{pad}  (loop ${}\n", loop_label));

                let mut new_ctx = LoopContext::Loop {
                    exit_label: exit_label.clone(),
                    continue_label: loop_label.clone(),
                };
                self.emit_statements(wat, body, indent + 4, &mut new_ctx);

                self.emit_expr(wat, condition, indent + 4);
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
                    self.emit_statement(wat, i, indent, loop_ctx);
                }

                let exit_label = gen_label("for_exit");
                let loop_label = gen_label("for_loop");
                let continue_label = gen_label("for_continue");

                wat.push_str(&format!("{pad}(block ${}\n", exit_label));
                wat.push_str(&format!("{pad}  (loop ${}\n", loop_label));

                if let Some(cond) = condition {
                    self.emit_expr(wat, cond, indent + 4);
                    wat.push_str(&format!(
                        "{pad}    (call {})\n",
                        RuntimeFn::TruthyBool.symbol()
                    ));
                    wat.push_str(&format!("{pad}    (i32.eqz)\n"));
                    wat.push_str(&format!("{pad}    (br_if ${})\n", exit_label));
                }

                let mut new_ctx = LoopContext::Loop {
                    exit_label: exit_label.clone(),
                    continue_label: continue_label.clone(),
                };
                self.emit_statements(wat, body, indent + 4, &mut new_ctx);

                wat.push_str(&format!("{pad}  (block ${}\n", continue_label));
                if let Some(upd) = update {
                    self.emit_expr(wat, upd, indent + 4);
                    if self.expr_produces_value(upd) {
                        wat.push_str(&format!("{pad}    (drop)\n"));
                    }
                }
                wat.push_str(&format!("{pad}  )\n"));

                wat.push_str(&format!("{pad}    (br ${})\n", loop_label));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::ForIn { var, iter, body } => {
                // For-in: iterate object keys
                // Pseudo: for (let k in obj) -> load obj, iterate keys, bind to k, execute body
                // Simplified for now: compile error
                wat.push_str(&format!("{pad};; for-in not yet implemented\n"));
            }
            LoweredStmt::ForOf { var, iter, body } => {
                // For-of: iterate array values
                // Simplified for now: compile error
                wat.push_str(&format!("{pad};; for-of not yet implemented\n"));
            }
            LoweredStmt::Break => match loop_ctx {
                LoopContext::Loop { exit_label, .. } => {
                    wat.push_str(&format!("{pad}(br ${})\n", exit_label));
                }
                LoopContext::Root => {
                    wat.push_str(&format!("{pad};; ERROR: break outside loop\n"));
                }
            },
            LoweredStmt::Continue => match loop_ctx {
                LoopContext::Loop { continue_label, .. } => {
                    wat.push_str(&format!("{pad}(br ${})\n", continue_label));
                }
                LoopContext::Root => {
                    wat.push_str(&format!("{pad};; ERROR: continue outside loop\n"));
                }
            },
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

                self.emit_statements(wat, try_body, indent + 4, loop_ctx);

                wat.push_str(&format!("{pad}    (br ${})\n", try_exit));
                wat.push_str(&format!("{pad}  )\n"));

                // Catch block (for now, just a placeholder)
                if let Some(body) = catch_body {
                    if let Some(var) = catch_var {
                        wat.push_str(&format!("{pad}  (local.set {})\n", local_index(*var)));
                    }
                    self.emit_statements(wat, body, indent + 4, loop_ctx);
                }

                wat.push_str(&format!("{pad})\n"));

                // Finally block (always executes)
                if let Some(body) = finally_body {
                    self.emit_statements(wat, body, indent + 2, loop_ctx);
                }
            }
            LoweredStmt::Switch { expr, cases } => {
                // Switch: for now, convert to if-else chain
                let switch_exit = gen_label("switch_exit");
                wat.push_str(&format!("{pad}(block ${}\n", switch_exit));

                for (cond, body) in cases {
                    if let Some(c) = cond {
                        // Case with condition
                        self.emit_expr(wat, expr, indent + 2);
                        self.emit_expr(wat, c, indent + 2);
                        wat.push_str(&format!("{pad}  (i32.eq)\n"));
                        wat.push_str(&format!("{pad}  (if\n"));
                        wat.push_str(&format!("{pad}    (then\n"));
                        self.emit_statements(wat, body, indent + 6, loop_ctx);
                        wat.push_str(&format!("{pad}      (br ${})\n", switch_exit));
                        wat.push_str(&format!("{pad}    )\n"));
                        wat.push_str(&format!("{pad}  )\n"));
                    } else {
                        // Default case
                        self.emit_statements(wat, body, indent + 2, loop_ctx);
                        wat.push_str(&format!("{pad}  (br ${})\n", switch_exit));
                    }
                }

                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::ClassDecl {
                name: _,
                extends: _,
                constructor: _,
                methods: _,
            } => {
                // Placeholder for class declarations - will be implemented in Phase D
                wat.push_str(&format!("{pad};; TODO: implement class declaration\n"));
            }
        }
    }
}

fn local_index(id: LocalId) -> usize {
    id.0
}
