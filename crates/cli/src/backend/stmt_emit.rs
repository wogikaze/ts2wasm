use super::emitter::WatEmitter;
use crate::ir::lowered::LoweredStmt;
use crate::wasm_ident;

impl WatEmitter<'_> {
    pub(super) fn emit_top_level_statements(&self, wat: &mut String, indent: usize) {
        for statement in self.program {
            if matches!(statement, LoweredStmt::Function { .. }) {
                continue;
            }
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
            LoweredStmt::Let(name, expr) | LoweredStmt::Assign(name, expr) => {
                self.emit_expr(wat, expr, indent);
                wat.push_str(&format!("{pad}(local.set ${})\n", wasm_ident(name)));
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
            LoweredStmt::Function { .. } => {}
        }
    }
}
