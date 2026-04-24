use std::collections::{BTreeSet, HashMap};

use crate::ir::lowered::{
    FuncId, FunctionCallKind, LoweredBinaryOp, LoweredExpr, LoweredProgram, LoweredStmt,
    LoweredUnaryOp,
};
use crate::runtime::consts::RuntimeString;
use crate::runtime::layout::Layout;
use crate::runtime::value::ValueTag;
use crate::{DiagCode, Diagnostic};

use super::runtime_fn::{Capability, HostImport, RuntimeFn};

pub(crate) fn emit_wat(program: &LoweredProgram) -> Result<String, Diagnostic> {
    WatEmitter::new(program).emit()
}

pub(super) struct WatEmitter<'a> {
    pub(super) program: &'a LoweredProgram,
    pub(super) strings: HashMap<String, u32>,
    pub(super) string_data: Vec<(u32, String)>,
    pub(super) next_data_offset: u32,
    pub(super) required_runtime: BTreeSet<RuntimeFn>,
}

impl<'a> WatEmitter<'a> {
    pub(super) fn new(program: &'a LoweredProgram) -> Self {
        let mut emitter = Self {
            program,
            strings: HashMap::new(),
            string_data: Vec::new(),
            next_data_offset: Layout::DATA_START,
            required_runtime: BTreeSet::new(),
        };
        for value in [
            RuntimeString::UNDEFINED,
            RuntimeString::NULL,
            RuntimeString::FALSE,
            RuntimeString::TRUE,
            RuntimeString::NEWLINE,
        ] {
            emitter.intern_string(value);
        }
        emitter.collect_program_strings(&program.top_level_statements);
        for function in &program.functions {
            emitter.collect_program_strings(&function.body);
        }
        emitter.collect_required_runtime(program);
        emitter
    }

    fn emit(self) -> Result<String, Diagnostic> {
        self.validate_memory_layout()?;
        let mut wat = String::new();
        wat.push_str("(module\n");
        if self.requires_host_import(HostImport::FdWrite) {
            wat.push_str("  (import \"wasi_snapshot_preview1\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))\n");
        }
        let _requires_stdout = self.requires_capability(Capability::StdoutWrite);
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
        Ok(wat)
    }

    fn validate_memory_layout(&self) -> Result<(), Diagnostic> {
        if self.next_data_offset > Layout::SCRATCH_OFFSET {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "static data segment ({}) overlaps scratch buffer ({})",
                    self.next_data_offset,
                    Layout::SCRATCH_OFFSET
                ),
                span: None,
            });
        }
        let scratch_end = Layout::SCRATCH_OFFSET
            .checked_add(Layout::SCRATCH_SIZE)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "scratch range overflow while validating memory layout".to_owned(),
                span: None,
            })?;
        if scratch_end > Layout::HEAP_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "scratch range [{}..{}) overlaps heap start ({})",
                    Layout::SCRATCH_OFFSET,
                    scratch_end,
                    Layout::HEAP_START
                ),
                span: None,
            });
        }
        if Layout::SCRATCH_OFFSET >= Layout::HEAP_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "scratch buffer ({}) must be below heap start ({})",
                    Layout::SCRATCH_OFFSET,
                    Layout::HEAP_START
                ),
                span: None,
            });
        }
        Ok(())
    }

    fn requires_host_import(&self, import: HostImport) -> bool {
        self.required_runtime
            .iter()
            .any(|runtime_fn| runtime_fn.spec().imports.contains(&import))
    }

    fn requires_capability(&self, capability: Capability) -> bool {
        self.required_runtime
            .iter()
            .any(|runtime_fn| runtime_fn.spec().capability.contains(&capability))
    }

    fn add_required_runtime(&mut self, runtime_fn: RuntimeFn) {
        if !self.required_runtime.insert(runtime_fn) {
            return;
        }
        for dep in runtime_fn.spec().deps {
            self.add_required_runtime(*dep);
        }
    }

    fn collect_required_runtime(&mut self, program: &LoweredProgram) {
        self.collect_required_runtime_stmts(&program.top_level_statements);
        for function in &program.functions {
            self.collect_required_runtime_stmts(&function.body);
        }
    }

    fn collect_required_runtime_stmts(&mut self, statements: &[LoweredStmt]) {
        for statement in statements {
            match statement {
                LoweredStmt::Let(_, expr)
                | LoweredStmt::Assign(_, expr)
                | LoweredStmt::Expr(expr)
                | LoweredStmt::Return(expr) => self.collect_required_runtime_expr(expr),
                LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,
                } => {
                    self.collect_required_runtime_expr(condition);
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                    self.collect_required_runtime_stmts(then_body);
                    self.collect_required_runtime_stmts(else_body);
                }
                LoweredStmt::While { condition, body } => {
                    self.collect_required_runtime_expr(condition);
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                    self.collect_required_runtime_stmts(body);
                }
            }
        }
    }

    fn collect_required_runtime_expr(&mut self, expr: &LoweredExpr) {
        match expr {
            LoweredExpr::Unary { op, expr } => {
                self.collect_required_runtime_expr(expr);
                match op {
                    LoweredUnaryOp::Not => self.add_required_runtime(RuntimeFn::Not),
                }
            }
            LoweredExpr::Binary { left, op, right } => {
                self.collect_required_runtime_expr(left);
                self.collect_required_runtime_expr(right);
                match op {
                    LoweredBinaryOp::Add => self.add_required_runtime(RuntimeFn::Add),
                    LoweredBinaryOp::Subtract => self.add_required_runtime(RuntimeFn::Sub),
                    LoweredBinaryOp::Less => self.add_required_runtime(RuntimeFn::Less),
                    LoweredBinaryOp::StrictEqual => {
                        self.add_required_runtime(RuntimeFn::StrictEqual)
                    }
                }
            }
            LoweredExpr::Call { kind, args } => {
                for arg in args {
                    self.collect_required_runtime_expr(arg);
                }
                if let FunctionCallKind::Builtin(builtin) = kind {
                    self.add_required_runtime(RuntimeFn::from_builtin(*builtin));
                }
            }
            LoweredExpr::Number(_)
            | LoweredExpr::String(_)
            | LoweredExpr::Bool(_)
            | LoweredExpr::Null
            | LoweredExpr::Undefined
            | LoweredExpr::Local(_) => {}
        }
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
            | LoweredStmt::Expr(expr)
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
            | LoweredExpr::Local(_) => {}
        }
    }

    fn emit_functions(&self, wat: &mut String) {
        for function in &self.program.functions {
            wat.push_str(&format!("  (func ${} ", function_symbol(function.id)));
            for _ in &function.params {
                wat.push_str("(param i32) ");
            }
            wat.push_str("(result i32)\n");
            for _ in &function.locals {
                wat.push_str("    (local i32)\n");
            }
            self.emit_statements(wat, &function.body, 4);
            wat.push_str(&format!("    (i32.const {})\n", ValueTag::UNDEFINED));
            wat.push_str("  )\n");
        }
    }

    fn emit_start(&self, wat: &mut String) {
        wat.push_str("  (func $_start (export \"_start\")\n");
        for _ in &self.program.top_level_locals {
            wat.push_str("    (local i32)\n");
        }
        self.emit_top_level_statements(wat, 4);
        wat.push_str("  )\n");
    }
}

pub(super) fn function_symbol(id: FuncId) -> String {
    format!("func_{}", id.0)
}
