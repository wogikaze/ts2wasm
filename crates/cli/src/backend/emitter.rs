use std::cell::RefCell;
use std::collections::HashMap;

use crate::ir::lowered::{FuncId, LoweredExpr, LoweredProgram, LoweredStmt};
use crate::runtime::layout::Layout;
use crate::runtime::value::ValueTag;
use crate::{DiagCode, Diagnostic};

use super::runtime_fn::RuntimeGlobal;
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
    heap_builder_temps: RefCell<Option<HeapBuilderTemps>>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct HeapBuilderTemps {
    pub(super) base_local: usize,
    pub(super) value_local: usize,
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
            heap_builder_temps: RefCell::new(None),
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
        // Emit all required imports from catalog (single source of truth)
        self.emit_imports_from_catalog(&mut wat);
        wat.push_str("  (memory (export \"memory\") 1)\n");
        wat.push_str(&format!(
            "  (global $heap (mut i32) (i32.const {}))\n",
            Layout::HEAP_START,
        ));
        self.emit_required_globals(&mut wat);
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

    fn emit_imports_from_catalog(&self, wat: &mut String) {
        // Emit all required imports using catalog as single source of truth
        for import in self.link_plan.required_imports() {
            let spec = import.spec();
            // Build complete WAT function signature from catalog
            let mut sig = String::new();
            if !spec.params.is_empty() {
                sig.push(' ');
                sig.push('(');
                sig.push_str(spec.params);
                sig.push(')');
            }
            if !spec.result.is_empty() {
                sig.push(' ');
                sig.push('(');
                sig.push_str(spec.result);
                sig.push(')');
            }
            wat.push_str(&format!(
                "  (import \"{}\" \"{}\" (func {}{}))\n",
                spec.module, spec.name, spec.wat_symbol, sig
            ));
        }
    }

    fn emit_required_globals(&self, wat: &mut String) {
        for global in self.link_plan.required_globals() {
            wat.push_str(&format!(
                "  (global {} (mut i32) (i32.const {}))\n",
                global.symbol(),
                global.initial_value(),
            ));
        }
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
            // Two backend-owned temporaries for ArrayNew/ObjectNew construction.
            wat.push_str("    (local i32)\n");
            wat.push_str("    (local i32)\n");
            let temp_base = function.params.len() + function.locals.len();
            self.set_heap_builder_temps(HeapBuilderTemps {
                base_local: temp_base,
                value_local: temp_base + 1,
            });
            let mut loop_ctx = super::stmt_emit::LoopContext::Root;
            self.emit_statements(wat, &function.body, 4, &mut loop_ctx);
            self.clear_heap_builder_temps();
            wat.push_str(&format!("    (i32.const {})\n", ValueTag::UNDEFINED));
            wat.push_str("  )\n");
        }
    }

    fn emit_start(&self, wat: &mut String) {
        wat.push_str("  (func $_start (export \"_start\")\n");
        let extra_locals = if self.module_runtime_enabled() { 1 } else { 0 };
        let top_level_local_count = self.program.top_level_locals.len() + extra_locals;
        for _ in 0..top_level_local_count + 2 {
            wat.push_str("    (local i32)\n");
        }
        self.set_heap_builder_temps(HeapBuilderTemps {
            base_local: top_level_local_count,
            value_local: top_level_local_count + 1,
        });
        if self.module_runtime_enabled() {
            let cache_size = Layout::MODULE_CACHE_MAX as u32 * Layout::MODULE_CACHE_ENTRY_SIZE;
            wat.push_str(&format!(
                "    (global.set $module_cache (call $alloc_heap (i32.const {cache_size})))\n",
            ));
            wat.push_str("    (global.set $current_module_id (i32.const 0))\n");
        }
        self.emit_top_level_statements(wat, 4);
        self.clear_heap_builder_temps();
        wat.push_str("  )\n");
    }

    pub(super) fn heap_builder_temps(&self) -> HeapBuilderTemps {
        self.heap_builder_temps
            .borrow()
            .as_ref()
            .copied()
            // Safe fallback for defensive robustness; normal paths set scope temps.
            .unwrap_or(HeapBuilderTemps {
                base_local: 0,
                value_local: 1,
            })
    }

    fn set_heap_builder_temps(&self, temps: HeapBuilderTemps) {
        *self.heap_builder_temps.borrow_mut() = Some(temps);
    }

    fn clear_heap_builder_temps(&self) {
        *self.heap_builder_temps.borrow_mut() = None;
    }

    fn module_runtime_enabled(&self) -> bool {
        self.link_plan
            .required_globals()
            .contains(&RuntimeGlobal::ModuleCache)
    }
}

pub(super) fn function_symbol(id: FuncId) -> String {
    format!("func_{}", id.0)
}
