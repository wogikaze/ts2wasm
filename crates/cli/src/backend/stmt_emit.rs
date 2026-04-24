use super::emitter::WatEmitter;
use crate::ir::lowered::LocalId;
use crate::ir::lowered::LoweredStmt;

impl WatEmitter<'_> {
    pub(super) fn emit_top_level_statements(&self, wat: &mut String, indent: usize) {
        for statement in &self.program.top_level_statements {
            self.emit_statement(wat, statement, indent);
        }
    }

    pub(super) fn emit_statements(
        &self,
        wat: &mut String,
        statements: &[LoweredStmt],
        indent: usize,
    ) {
        for statement in statements {
            self.emit_statement(wat, statement, indent);
        }
    }

    fn emit_statement(&self, wat: &mut String, statement: &LoweredStmt, indent: usize) {
        let pad = " ".repeat(indent);
        match statement {
            LoweredStmt::Let(local_id, expr) | LoweredStmt::Assign(local_id, expr) => {
                self.emit_expr(wat, expr, indent);
                wat.push_str(&format!("{pad}(local.set {})\n", local_index(*local_id)));
            }
            LoweredStmt::ConsoleLog(expr) => {
                self.emit_expr(wat, expr, indent);
                wat.push_str(&format!("{pad}(call $log)\n"));
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.emit_expr(wat, condition, indent);
                wat.push_str(&format!("{pad}(call $truthy_bool)\n"));
                wat.push_str(&format!("{pad}(if\n"));
                wat.push_str(&format!("{pad}  (then\n"));
                self.emit_statements(wat, then_body, indent + 4);
                wat.push_str(&format!("{pad}  )\n"));
                if !else_body.is_empty() {
                    wat.push_str(&format!("{pad}  (else\n"));
                    self.emit_statements(wat, else_body, indent + 4);
                    wat.push_str(&format!("{pad}  )\n"));
                }
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::While { condition, body } => {
                wat.push_str(&format!("{pad}(block $while_exit\n"));
                wat.push_str(&format!("{pad}  (loop $while_loop\n"));
                self.emit_expr(wat, condition, indent + 4);
                wat.push_str(&format!("{pad}    (call $truthy_bool)\n"));
                wat.push_str(&format!("{pad}    (i32.eqz)\n"));
                wat.push_str(&format!("{pad}    (br_if $while_exit)\n"));
                self.emit_statements(wat, body, indent + 4);
                wat.push_str(&format!("{pad}    (br $while_loop)\n"));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredStmt::Return(expr) => {
                self.emit_expr(wat, expr, indent);
                wat.push_str(&format!("{pad}(return)\n"));
            }
        }
    }
}

fn local_index(id: LocalId) -> usize {
    id.0
}
