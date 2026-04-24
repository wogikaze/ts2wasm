use std::collections::HashMap;

use crate::ir::lowered::{LoweredExpr, LoweredStmt};
use crate::runtime::layout::Layout;
use crate::runtime::value::ValueTag;
use crate::wasm_ident;

pub(crate) fn emit_wat(program: &[LoweredStmt]) -> String {
    WatEmitter::new(program).emit()
}

pub(super) struct WatEmitter<'a> {
    pub(super) program: &'a [LoweredStmt],
    pub(super) strings: HashMap<String, u32>,
    pub(super) string_data: Vec<(u32, String)>,
    pub(super) next_data_offset: u32,
}

impl<'a> WatEmitter<'a> {
    pub(super) fn new(program: &'a [LoweredStmt]) -> Self {
        let mut emitter = Self {
            program,
            strings: HashMap::new(),
            string_data: Vec::new(),
            next_data_offset: Layout::DATA_START,
        };
        for value in ["undefined", "null", "false", "true", "\n"] {
            emitter.intern_string(value);
        }
        emitter.collect_program_strings(program);
        emitter
    }

    fn emit(self) -> String {
        let mut wat = String::new();
        wat.push_str("(module\n");
        wat.push_str("  (import \"wasi_snapshot_preview1\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))\n");
        wat.push_str("  (memory (export \"memory\") 1)\n");
        wat.push_str(&format!(
            "  (global $heap (mut i32) (i32.const {}))\n",
            Layout::HEAP_START,
        ));
        self.emit_data_segments(&mut wat);
        self.emit_runtime(&mut wat);
        self.emit_functions(&mut wat);
        self.emit_start(&mut wat);
        wat.push_str(")\n");
        wat
    }

    fn collect_program_strings(&mut self, statements: &[LoweredStmt]) {
        for statement in statements {
            self.collect_statement_strings(statement);
        }
    }

    fn collect_statement_strings(&mut self, statement: &LoweredStmt) {
        match statement {
            LoweredStmt::Let(_, expr)
            | LoweredStmt::Assign(_, expr)
            | LoweredStmt::ConsoleLog(expr)
            | LoweredStmt::Return(expr) => {
                self.collect_expr_strings(expr);
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_expr_strings(condition);
                self.collect_program_strings(then_body);
                self.collect_program_strings(else_body);
            }
            LoweredStmt::While { condition, body } => {
                self.collect_expr_strings(condition);
                self.collect_program_strings(body);
            }
            LoweredStmt::Function { body, .. } => self.collect_program_strings(body),
        }
    }

    fn collect_expr_strings(&mut self, expr: &LoweredExpr) {
        match expr {
            LoweredExpr::String(value) => {
                self.intern_string(value);
            }
            LoweredExpr::Unary { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::Binary { left, right, .. } => {
                self.collect_expr_strings(left);
                self.collect_expr_strings(right);
            }
            LoweredExpr::Call { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            LoweredExpr::Number(_)
            | LoweredExpr::Bool(_)
            | LoweredExpr::Null
            | LoweredExpr::Undefined
            | LoweredExpr::Ident(_) => {}
        }
    }

    fn emit_functions(&self, wat: &mut String) {
        for statement in self.program {
            if let LoweredStmt::Function { name, params, body } = statement {
                wat.push_str(&format!("  (func $user_{} ", wasm_ident(name)));
                for param in params {
                    wat.push_str(&format!("(param ${} i32) ", wasm_ident(param)));
                }
                wat.push_str("(result i32)\n");
                for local in collect_locals(body) {
                    if !params.contains(&local) {
                        wat.push_str(&format!("    (local ${} i32)\n", wasm_ident(&local)));
                    }
                }
                self.emit_statements(wat, body, 4);
                wat.push_str(&format!("    (i32.const {})\n", ValueTag::UNDEFINED));
                wat.push_str("  )\n");
            }
        }
    }

    fn emit_start(&self, wat: &mut String) {
        wat.push_str("  (func $_start (export \"_start\")\n");
        for local in collect_start_locals(self.program) {
            wat.push_str(&format!("    (local ${} i32)\n", wasm_ident(&local)));
        }
        self.emit_top_level_statements(wat, 4);
        wat.push_str("  )\n");
    }
}

fn collect_start_locals(statements: &[LoweredStmt]) -> Vec<String> {
    let mut locals = Vec::new();
    for statement in statements {
        if matches!(statement, LoweredStmt::Function { .. }) {
            continue;
        }

        for local in collect_locals(std::slice::from_ref(statement)) {
            if !locals.contains(&local) {
                locals.push(local);
            }
        }
    }
    locals
}

fn collect_locals(statements: &[LoweredStmt]) -> Vec<String> {
    let mut locals = Vec::new();
    for statement in statements {
        match statement {
            LoweredStmt::Let(name, _) => {
                if !locals.contains(name) {
                    locals.push(name.clone());
                }
            }
            LoweredStmt::If {
                then_body,
                else_body,
                ..
            } => {
                for local in collect_locals(then_body)
                    .into_iter()
                    .chain(collect_locals(else_body))
                {
                    if !locals.contains(&local) {
                        locals.push(local);
                    }
                }
            }
            LoweredStmt::While { body, .. } | LoweredStmt::Function { body, .. } => {
                for local in collect_locals(body) {
                    if !locals.contains(&local) {
                        locals.push(local);
                    }
                }
            }
            LoweredStmt::Assign(_, _) | LoweredStmt::ConsoleLog(_) | LoweredStmt::Return(_) => {}
        }
    }
    locals
}
