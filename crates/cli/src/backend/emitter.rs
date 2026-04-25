use std::collections::HashMap;

use crate::ir::lowered::{FuncId, LoweredExpr, LoweredProgram, LoweredStmt};
use crate::runtime::layout::Layout;
use crate::runtime::value::ValueTag;
use crate::{DiagCode, Diagnostic};

use super::RuntimeFn;
use super::runtime_fn::HostImport;
use super::runtime_link_plan::RuntimeLinkPlan;

pub(crate) fn emit_wat(program: &LoweredProgram) -> Result<String, Diagnostic> {
    WatEmitter::new(program).emit()
}

pub(super) struct WatEmitter<'a> {
    pub(super) program: &'a LoweredProgram,
    pub(super) link_plan: RuntimeLinkPlan,
    pub(super) strings: HashMap<String, u32>,
    pub(super) string_data: Vec<(u32, String)>,
    pub(super) next_data_offset: u32,
}

impl<'a> WatEmitter<'a> {
    pub(super) fn new(program: &'a LoweredProgram) -> Self {
        let link_plan = RuntimeLinkPlan::from_program(program);
        let mut emitter = Self {
            program,
            link_plan,
            strings: HashMap::new(),
            string_data: Vec::new(),
            next_data_offset: Layout::DATA_START,
        };
        emitter.intern_required_runtime_strings();
        emitter.collect_program_strings(&program.top_level_statements);
        for function in &program.functions {
            emitter.collect_program_strings(&function.body);
        }
        emitter
    }

    fn emit(self) -> Result<String, Diagnostic> {
        self.validate_memory_layout()?;
        let _required_capabilities = self.link_plan.required_capabilities();
        let mut wat = String::new();
        wat.push_str("(module\n");
        if self.requires_host_import(HostImport::FdRead) {
            wat.push_str("  (import \"wasi_snapshot_preview1\" \"fd_read\" (func $fd_read (param i32 i32 i32 i32) (result i32)))\n");
        }
        if self.requires_host_import(HostImport::FdWrite) {
            wat.push_str("  (import \"wasi_snapshot_preview1\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))\n");
        }
        if self.requires_host_import(HostImport::FsReadFileSync) {
            wat.push_str("  (import \"host\" \"fs.readFileSync\" (func $host_fs_read_file_sync (param i32 i32) (result i32)))\n");
        }
        if self.requires_host_import(HostImport::FsWriteFileSync) {
            wat.push_str("  (import \"host\" \"fs.writeFileSync\" (func $host_fs_write_file_sync (param i32 i32)))\n");
        }
        if self.requires_host_import(HostImport::FsAppendFileSync) {
            wat.push_str("  (import \"host\" \"fs.appendFileSync\" (func $host_fs_append_file_sync (param i32 i32)))\n");
        }
        if self.requires_host_import(HostImport::ProcessArgv) {
            wat.push_str(
                "  (import \"host\" \"process.argv\" (func $host_process_argv (result i32)))\n",
            );
        }
        if self.requires_host_import(HostImport::ProcessEnv) {
            wat.push_str(
                "  (import \"host\" \"process.env\" (func $host_process_env (result i32)))\n",
            );
        }
        if self.requires_host_import(HostImport::ProcessExit) {
            wat.push_str(
                "  (import \"host\" \"process.exit\" (func $host_process_exit (param i32)))\n",
            );
        }
        if self.requires_host_import(HostImport::PathJoin) {
            wat.push_str("  (import \"host\" \"path.join\" (func $host_path_join (param i32 i32) (result i32)))\n");
        }
        if self.requires_host_import(HostImport::PathResolve) {
            wat.push_str("  (import \"host\" \"path.resolve\" (func $host_path_resolve (param i32) (result i32)))\n");
        }
        if self.requires_host_import(HostImport::PathBasename) {
            wat.push_str("  (import \"host\" \"path.basename\" (func $host_path_basename (param i32) (result i32)))\n");
        }
        if self.requires_host_import(HostImport::PathDirname) {
            wat.push_str("  (import \"host\" \"path.dirname\" (func $host_path_dirname (param i32) (result i32)))\n");
        }
        if self.requires_host_import(HostImport::CryptoRandomBytes) {
            wat.push_str("  (import \"host\" \"crypto.randomBytes\" (func $host_crypto_random_bytes (param i32) (result i32)))\n");
        }
        wat.push_str("  (memory (export \"memory\") 1)\n");
        wat.push_str(&format!(
            "  (global $heap (mut i32) (i32.const {}))\n",
            Layout::HEAP_START,
        ));
        if self.module_runtime_enabled() {
            wat.push_str("  (global $module_cache (mut i32) (i32.const 0))\n");
            wat.push_str("  (global $current_module_id (mut i32) (i32.const 0))\n");
        }
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
        if self.next_data_offset < Layout::DATA_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "static data end ({}) is below data start ({})",
                    self.next_data_offset,
                    Layout::DATA_START
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
        if scratch_end > Layout::STDIN_BUFFER_OFFSET {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "scratch range [{}..{}) overlaps stdin buffer ({})",
                    Layout::SCRATCH_OFFSET,
                    scratch_end,
                    Layout::STDIN_BUFFER_OFFSET
                ),
                span: None,
            });
        }
        let stdin_end = Layout::STDIN_BUFFER_OFFSET
            .checked_add(Layout::STDIN_BUFFER_SIZE)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "stdin range overflow while validating memory layout".to_owned(),
                span: None,
            })?;
        if stdin_end > Layout::HEAP_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "stdin range [{}..{}) overlaps heap start ({})",
                    Layout::STDIN_BUFFER_OFFSET,
                    stdin_end,
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
        if Layout::STDIN_BUFFER_OFFSET >= Layout::HEAP_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "stdin buffer ({}) must be below heap start ({})",
                    Layout::STDIN_BUFFER_OFFSET,
                    Layout::HEAP_START
                ),
                span: None,
            });
        }
        let stdin_nread_end =
            Layout::STDIN_NREAD_OFFSET
                .checked_add(4)
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "stdin nread region overflow while validating memory layout"
                        .to_owned(),
                    span: None,
                })?;
        if stdin_nread_end > Layout::STDIN_BUFFER_OFFSET {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "stdin iovec/nread region [{}..{}) overlaps stdin buffer ({})",
                    Layout::STDIN_IOVEC_OFFSET,
                    stdin_nread_end,
                    Layout::STDIN_BUFFER_OFFSET
                ),
                span: None,
            });
        }
        if Layout::HEAP_START % Layout::ALIGN != 0 {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "HEAP_START ({}) must be {}-byte aligned for RawValue heap tags",
                    Layout::HEAP_START,
                    Layout::ALIGN
                ),
                span: None,
            });
        }
        Ok(())
    }

    fn requires_host_import(&self, import: HostImport) -> bool {
        self.link_plan.required_imports().contains(&import)
    }

    fn intern_required_runtime_strings(&mut self) {
        let runtime_strings: Vec<_> = self
            .link_plan
            .required_runtime_strings()
            .iter()
            .copied()
            .collect();
        for value in runtime_strings {
            self.intern_string(value);
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
            | LoweredStmt::Return(expr)
            | LoweredStmt::Throw(expr) => {
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
            LoweredStmt::TryCatch {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                self.collect_program_strings(try_body);
                if let Some(body) = catch_body {
                    self.collect_program_strings(body);
                }
                if let Some(body) = finally_body {
                    self.collect_program_strings(body);
                }
            }
            LoweredStmt::Switch { expr, cases } => {
                self.collect_expr_strings(expr);
                for (cond, body) in cases {
                    if let Some(cond_expr) = cond {
                        self.collect_expr_strings(cond_expr);
                    }
                    self.collect_program_strings(body);
                }
            }
            LoweredStmt::DoWhile { body, condition } => {
                self.collect_program_strings(body);
                self.collect_expr_strings(condition);
            }
            LoweredStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init_stmt) = init {
                    self.collect_statement_strings(init_stmt);
                }
                if let Some(cond) = condition {
                    self.collect_expr_strings(cond);
                }
                if let Some(upd) = update {
                    self.collect_expr_strings(upd);
                }
                self.collect_program_strings(body);
            }
            LoweredStmt::ForIn {
                var: _, iter, body, ..
            } => {
                self.collect_expr_strings(iter);
                self.collect_program_strings(body);
            }
            LoweredStmt::ForOf {
                var: _, iter, body, ..
            } => {
                self.collect_expr_strings(iter);
                self.collect_program_strings(body);
            }
            LoweredStmt::Break | LoweredStmt::Continue => {}
            LoweredStmt::Export { name, expr } => {
                self.intern_string(name);
                self.collect_expr_strings(expr);
            }
            LoweredStmt::ModuleExportsAssign { expr } => {
                self.collect_expr_strings(expr);
            }
            LoweredStmt::ClassDecl { .. } => {
                // Class declarations should not appear in lowered program
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
            LoweredExpr::ArrayNew { elements, .. } => {
                for elem in elements {
                    self.collect_expr_strings(elem);
                }
            }
            LoweredExpr::ArrayGet { arr, index } => {
                self.collect_expr_strings(arr);
                self.collect_expr_strings(index);
            }
            LoweredExpr::GetLength(inner) => {
                self.collect_expr_strings(inner);
            }
            LoweredExpr::ObjectNew { props, .. } => {
                for (key, val) in props {
                    self.intern_string(key);
                    self.collect_expr_strings(val);
                }
            }
            LoweredExpr::PropertyGet { obj, key } => {
                self.collect_expr_strings(obj);
                self.intern_string(key);
            }
            LoweredExpr::MethodCall { object, args, .. } => {
                self.collect_expr_strings(object);
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            LoweredExpr::PropertySet { object, key, value } => {
                self.collect_expr_strings(object);
                self.intern_string(key);
                self.collect_expr_strings(value);
            }
            LoweredExpr::New { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            LoweredExpr::ModuleLoad { .. } => {}
            LoweredExpr::RuntimeCall { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
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
            let mut loop_ctx = super::stmt_emit::LoopContext::Root;
            self.emit_statements(wat, &function.body, 4, &mut loop_ctx);
            wat.push_str(&format!("    (i32.const {})\n", ValueTag::UNDEFINED));
            wat.push_str("  )\n");
        }
    }

    fn emit_start(&self, wat: &mut String) {
        wat.push_str("  (func $_start (export \"_start\")\n");
        let extra_locals = if self.module_runtime_enabled() { 1 } else { 0 };
        for _ in 0..self.program.top_level_locals.len() + extra_locals {
            wat.push_str("    (local i32)\n");
        }
        if self.module_runtime_enabled() {
            let cache_size = Layout::MODULE_CACHE_MAX as u32 * Layout::MODULE_CACHE_ENTRY_SIZE;
            wat.push_str(&format!(
                "    (global.set $module_cache (call $alloc_heap (i32.const {cache_size})))\n",
            ));
            wat.push_str("    (global.set $current_module_id (i32.const 0))\n");
        }
        self.emit_top_level_statements(wat, 4);
        wat.push_str("  )\n");
    }

    fn module_runtime_enabled(&self) -> bool {
        let required = self.link_plan.required_runtime_functions();
        required.contains(&RuntimeFn::ModuleRequire)
            || required.contains(&RuntimeFn::ModuleExportsSet)
            || required.contains(&RuntimeFn::ModuleExportsAssign)
    }
}

pub(super) fn function_symbol(id: FuncId) -> String {
    format!("func_{}", id.0)
}
