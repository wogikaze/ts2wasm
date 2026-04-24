use std::collections::HashMap;

use crate::runtime::layout::Layout;
use crate::{Expr, Stmt, collect_locals, wasm_ident};

pub(crate) fn emit_wat(program: &[Stmt]) -> String {
    WatEmitter::new(program).emit()
}

pub(super) struct WatEmitter<'a> {
    pub(super) program: &'a [Stmt],
    pub(super) strings: HashMap<String, u32>,
    pub(super) string_data: Vec<(u32, String)>,
    pub(super) next_data_offset: u32,
}

impl<'a> WatEmitter<'a> {
    pub(super) fn new(program: &'a [Stmt]) -> Self {
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
        wat.push_str("  (global $heap (mut i32) (i32.const 2048))\n");
        self.emit_data_segments(&mut wat);
        self.emit_runtime(&mut wat);
        self.emit_functions(&mut wat);
        self.emit_start(&mut wat);
        wat.push_str(")\n");
        wat
    }

    fn collect_program_strings(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.collect_statement_strings(statement);
        }
    }

    fn collect_statement_strings(&mut self, statement: &Stmt) {
        match statement {
            Stmt::Let(_, expr)
            | Stmt::Assign(_, expr)
            | Stmt::ConsoleLog(expr)
            | Stmt::Return(expr) => {
                self.collect_expr_strings(expr);
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_expr_strings(condition);
                self.collect_program_strings(then_body);
                self.collect_program_strings(else_body);
            }
            Stmt::While { condition, body } => {
                self.collect_expr_strings(condition);
                self.collect_program_strings(body);
            }
            Stmt::Function { body, .. } => self.collect_program_strings(body),
        }
    }

    fn collect_expr_strings(&mut self, expr: &Expr) {
        match expr {
            Expr::String(value) => {
                self.intern_string(value);
            }
            Expr::Unary { expr, .. } => self.collect_expr_strings(expr),
            Expr::Binary { left, right, .. } => {
                self.collect_expr_strings(left);
                self.collect_expr_strings(right);
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            Expr::Number(_) | Expr::Bool(_) | Expr::Null | Expr::Undefined | Expr::Ident(_) => {}
        }
    }

    fn emit_functions(&self, wat: &mut String) {
        for statement in self.program {
            if let Stmt::Function { name, params, body } = statement {
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
                wat.push_str("    (i32.const 0)\n");
                wat.push_str("  )\n");
            }
        }
    }

    fn emit_start(&self, wat: &mut String) {
        wat.push_str("  (func $_start (export \"_start\")\n");
        for local in collect_locals(self.program) {
            wat.push_str(&format!("    (local ${} i32)\n", wasm_ident(&local)));
        }
        self.emit_statements(wat, self.program, 4);
        wat.push_str("  )\n");
    }
}
