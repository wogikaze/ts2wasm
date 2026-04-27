use std::collections::{BTreeMap, HashMap};

use crate::{DiagCode, Diagnostic};
use ts2wasm_ir::lowered::{ClassPrototypeRef, FuncId, LoweredExpr, LoweredProgram, LoweredStmt};
use ts2wasm_runtime_abi::Layout;
use ts2wasm_runtime_abi::ValueTag;

use super::runtime_fn::RuntimeGlobal;
use super::runtime_link_plan::RuntimeLinkPlan;
use super::wat_writer::WatModuleBuilder;

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

#[derive(Debug, Clone, Copy)]
pub(super) struct LocalFrame {
    pub(super) user_local_count: usize,
    pub(super) backend_base: usize,
}

impl LocalFrame {
    pub(super) fn new(user_local_count: usize) -> Self {
        Self {
            user_local_count,
            backend_base: user_local_count,
        }
    }

    pub(super) const fn total_local_count(self) -> usize {
        self.user_local_count + self.backend_local_count()
    }

    pub(super) const fn backend_local_count(self) -> usize {
        3
    }

    pub(super) const fn heap_base_tmp(self) -> usize {
        self.backend_base
    }

    pub(super) const fn heap_value_tmp(self) -> usize {
        self.backend_base + 1
    }

    pub(super) const fn switch_value_tmp(self) -> usize {
        self.backend_base + 2
    }
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

    fn emit(mut self) -> Result<String, Diagnostic> {
        self.validate_memory_layout()?;
        let _required_capabilities = self.link_plan.required_capabilities();
        let mut wat = String::new();
        wat.push_str("(module\n");
        // Emit all required imports from catalog (single source of truth)
        self.emit_imports_from_catalog(&mut wat);
        wat.push_str(&format!(
            "  (memory (export \"memory\") {})\n",
            Layout::MEMORY_MIN_PAGES,
        ));
        wat.push_str(&format!(
            "  (global $heap (mut i32) (i32.const {}))\n",
            Layout::HEAP_START,
        ));
        self.emit_required_globals(&mut wat);
        self.emit_class_prototype_globals(&mut wat);
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
        let max_stdin_heap_allocation = Layout::HEAP_START
            .checked_add(Layout::STRING_HEADER_SIZE)
            .and_then(|base| base.checked_add(Layout::STDIN_READ_LIMIT))
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "stdin heap allocation overflow while validating memory layout".to_owned(),
                span: None,
            })?;
        let initial_memory_bytes = Layout::MEMORY_MIN_PAGES
            .checked_mul(Layout::WASM_PAGE_SIZE)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "memory page byte size overflow while validating memory layout".to_owned(),
                span: None,
            })?;
        if max_stdin_heap_allocation > initial_memory_bytes {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "single max stdin heap allocation from HEAP_START ({max_stdin_heap_allocation}) exceeds initial memory bytes ({initial_memory_bytes})"
                ),
                span: None,
            });
        }
        Ok(())
    }

    fn emit_imports_from_catalog(&self, wat: &mut String) {
        let mut writer = WatModuleBuilder::new();
        for import in self.link_plan.required_imports() {
            let spec = import.spec();
            writer.push_import_func(&spec);
        }
        wat.push_str(&writer.into_inner());
    }

    fn emit_required_globals(&self, wat: &mut String) {
        let mut writer = WatModuleBuilder::new();
        for global in self.link_plan.required_globals() {
            writer.push_global_i32(global.symbol(), global.initial_value());
        }
        wat.push_str(&writer.into_inner());
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
            LoweredExpr::Number(_)
            | LoweredExpr::Bool(_)
            | LoweredExpr::Null
            | LoweredExpr::Undefined
            | LoweredExpr::This
            | LoweredExpr::Local(_)
            | LoweredExpr::ArrowFn { .. } => {}
            LoweredExpr::Unary { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::Assign { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::Binary { left, right, .. } => {
                self.collect_expr_strings(left);
                self.collect_expr_strings(right);
            }
            LoweredExpr::Call { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            LoweredExpr::PropertyDelete { object, key } => {
                self.collect_expr_strings(object);
                self.intern_string(key);
            }
            LoweredExpr::PropertyDeleteDynamic { object, key } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(key);
            }
            LoweredExpr::PropertyIn { obj, key } => {
                self.collect_expr_strings(obj);
                self.intern_string(key);
            }
            LoweredExpr::PropertyInDynamic { obj, key } => {
                self.collect_expr_strings(obj);
                self.collect_expr_strings(key);
            }
            LoweredExpr::ArrayNew { elements, .. } => {
                for elem in elements {
                    self.collect_expr_strings(elem);
                }
            }
            LoweredExpr::ArrayGet { arr, index } => {
                self.collect_expr_strings(arr);
                self.collect_expr_strings(index);
            }
            LoweredExpr::Index { object, index } => {
                self.collect_expr_strings(object);
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
            LoweredExpr::PropertyGetDynamic { obj, key } => {
                self.collect_expr_strings(obj);
                self.collect_expr_strings(key);
            }
            LoweredExpr::MethodCall { object, .. } => {
                self.collect_expr_strings(object);
            }
            LoweredExpr::PropertySet { object, key, value } => {
                self.collect_expr_strings(object);
                self.intern_string(key);
                self.collect_expr_strings(value);
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
            } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(index);
                self.collect_expr_strings(value);
            }
            LoweredExpr::New { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            LoweredExpr::ClassPrototype(_) => {}
            LoweredExpr::ModuleLoad { .. } => {}
            LoweredExpr::RuntimeCall { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
        }
    }

    fn emit_class_prototype_globals(&self, wat: &mut String) {
        for constructor in self.class_prototypes().keys() {
            wat.push_str(&format!(
                "  (global ${} (mut i32) (i32.const 0))\n",
                class_prototype_global(*constructor),
            ));
        }
    }

    fn emit_class_prototype_initializers(&self, wat: &mut String, indent: usize) {
        let pad = " ".repeat(indent);
        for (constructor, parent) in self.ordered_class_prototypes() {
            let global = class_prototype_global(constructor);
            wat.push_str(&format!(
                "{pad}(if (i32.eqz (global.get ${global}))\n{pad}  (then\n"
            ));
            wat.push_str(&format!(
                "{pad}    (global.set ${global} (call {} (i32.const {})))\n",
                super::runtime_fn::RuntimeFn::AllocHeap.symbol(),
                Layout::OBJECT_HEADER_SIZE,
            ));
            wat.push_str(&format!(
                "{pad}    (i32.store (global.get ${global}) (i32.const 0))\n"
            ));
            let parent_expr = parent
                .map(|id| format!("global.get ${}", class_prototype_global(id)))
                .unwrap_or_else(|| "i32.const 0".to_owned());
            wat.push_str(&format!(
                "{pad}    (i32.store (i32.add (global.get ${global}) (i32.const {})) ({parent_expr}))\n",
                Layout::OBJECT_PROTOTYPE_OFFSET,
            ));
            wat.push_str(&format!("{pad}  )\n{pad})\n"));
        }
    }

    fn ordered_class_prototypes(&self) -> Vec<(FuncId, Option<FuncId>)> {
        let prototypes = self.class_prototypes();
        let mut ordered = prototypes
            .iter()
            .map(|(constructor, parent)| {
                (
                    *constructor,
                    *parent,
                    class_prototype_depth(*constructor, &prototypes),
                )
            })
            .collect::<Vec<_>>();
        ordered.sort_by_key(|(constructor, _, depth)| (*depth, constructor.0));
        ordered
            .into_iter()
            .map(|(constructor, parent, _)| (constructor, parent))
            .collect()
    }

    pub(super) fn class_prototypes(&self) -> BTreeMap<FuncId, Option<FuncId>> {
        let mut prototypes = BTreeMap::new();
        self.collect_class_prototypes_from_stmts(
            &self.program.top_level_statements,
            &mut prototypes,
        );
        for function in &self.program.functions {
            self.collect_class_prototypes_from_stmts(&function.body, &mut prototypes);
        }
        prototypes
    }

    fn collect_class_prototypes_from_stmts(
        &self,
        stmts: &[LoweredStmt],
        prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
    ) {
        for stmt in stmts {
            match stmt {
                LoweredStmt::Let(_, expr)
                | LoweredStmt::Assign(_, expr)
                | LoweredStmt::Expr(expr)
                | LoweredStmt::Return(expr)
                | LoweredStmt::Throw(expr)
                | LoweredStmt::Export { expr, .. }
                | LoweredStmt::ModuleExportsAssign { expr } => {
                    self.collect_class_prototypes_from_expr(expr, prototypes);
                }
                LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,
                } => {
                    self.collect_class_prototypes_from_expr(condition, prototypes);
                    self.collect_class_prototypes_from_stmts(then_body, prototypes);
                    self.collect_class_prototypes_from_stmts(else_body, prototypes);
                }
                LoweredStmt::While { condition, body }
                | LoweredStmt::DoWhile { body, condition } => {
                    self.collect_class_prototypes_from_expr(condition, prototypes);
                    self.collect_class_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::TryCatch {
                    try_body,
                    catch_body,
                    finally_body,
                    ..
                } => {
                    self.collect_class_prototypes_from_stmts(try_body, prototypes);
                    if let Some(body) = catch_body {
                        self.collect_class_prototypes_from_stmts(body, prototypes);
                    }
                    if let Some(body) = finally_body {
                        self.collect_class_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::Switch { expr, cases } => {
                    self.collect_class_prototypes_from_expr(expr, prototypes);
                    for (case_expr, body) in cases {
                        if let Some(case_expr) = case_expr {
                            self.collect_class_prototypes_from_expr(case_expr, prototypes);
                        }
                        self.collect_class_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::For {
                    init,
                    condition,
                    update,
                    body,
                } => {
                    if let Some(init) = init {
                        self.collect_class_prototypes_from_stmts(
                            std::slice::from_ref(init.as_ref()),
                            prototypes,
                        );
                    }
                    if let Some(condition) = condition {
                        self.collect_class_prototypes_from_expr(condition, prototypes);
                    }
                    if let Some(update) = update {
                        self.collect_class_prototypes_from_expr(update, prototypes);
                    }
                    self.collect_class_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::ForIn { iter, body, .. } | LoweredStmt::ForOf { iter, body, .. } => {
                    self.collect_class_prototypes_from_expr(iter, prototypes);
                    self.collect_class_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::Break | LoweredStmt::Continue | LoweredStmt::ClassDecl { .. } => {}
            }
        }
    }

    fn collect_class_prototypes_from_expr(
        &self,
        expr: &LoweredExpr,
        prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
    ) {
        match expr {
            LoweredExpr::ClassPrototype(prototype) => {
                add_class_prototype_ref(prototype, prototypes);
            }
            LoweredExpr::New {
                prototype, args, ..
            } => {
                add_class_prototype_ref(prototype, prototypes);
                for arg in args {
                    self.collect_class_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::Unary { expr, .. }
            | LoweredExpr::GetLength(expr)
            | LoweredExpr::PropertyGet { obj: expr, .. }
            | LoweredExpr::MethodCall { object: expr, .. }
            | LoweredExpr::PropertyDelete { object: expr, .. } => {
                self.collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::Binary { left, right, .. } => {
                self.collect_class_prototypes_from_expr(left, prototypes);
                self.collect_class_prototypes_from_expr(right, prototypes);
            }
            LoweredExpr::PropertyIn { obj, .. } => {
                self.collect_class_prototypes_from_expr(obj, prototypes);
            }
            LoweredExpr::PropertyInDynamic { obj, key }
            | LoweredExpr::ArrayGet {
                arr: obj,
                index: key,
            }
            | LoweredExpr::Index {
                object: obj,
                index: key,
            }
            | LoweredExpr::PropertyGetDynamic { obj, key }
            | LoweredExpr::PropertyDeleteDynamic { object: obj, key } => {
                self.collect_class_prototypes_from_expr(obj, prototypes);
                self.collect_class_prototypes_from_expr(key, prototypes);
            }
            LoweredExpr::Call { args, .. } | LoweredExpr::RuntimeCall { args, .. } => {
                for arg in args {
                    self.collect_class_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::ArrayNew { elements } => {
                for elem in elements {
                    self.collect_class_prototypes_from_expr(elem, prototypes);
                }
            }
            LoweredExpr::ObjectNew { props } => {
                for (_, value) in props {
                    self.collect_class_prototypes_from_expr(value, prototypes);
                }
            }
            LoweredExpr::PropertySet { object, value, .. } => {
                self.collect_class_prototypes_from_expr(object, prototypes);
                self.collect_class_prototypes_from_expr(value, prototypes);
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
            } => {
                self.collect_class_prototypes_from_expr(object, prototypes);
                self.collect_class_prototypes_from_expr(index, prototypes);
                self.collect_class_prototypes_from_expr(value, prototypes);
            }
            LoweredExpr::Assign { expr, .. } => {
                self.collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::Number(_)
            | LoweredExpr::String(_)
            | LoweredExpr::Bool(_)
            | LoweredExpr::Null
            | LoweredExpr::Undefined
            | LoweredExpr::Local(_)
            | LoweredExpr::ModuleLoad { .. }
            | LoweredExpr::This
            | LoweredExpr::ArrowFn { .. } => {}
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
            let frame = LocalFrame::new(function.params.len() + function.locals.len());
            // Backend-owned temporaries for heap construction and switch dispatch.
            for _ in 0..frame.backend_local_count() {
                wat.push_str("    (local i32)\n");
            }
            let mut loop_ctx = super::stmt_emit::LoopContext::Root;
            self.emit_statements(wat, &function.body, 4, &mut loop_ctx, &frame);
            wat.push_str(&format!("    (i32.const {})\n", ValueTag::UNDEFINED));
            wat.push_str("  )\n");
        }
    }

    fn emit_start(&self, wat: &mut String) {
        wat.push_str("  (func $_start (export \"_start\")\n");
        let extra_locals = if self.module_runtime_enabled() { 1 } else { 0 };
        let frame = LocalFrame::new(self.program.top_level_locals.len() + extra_locals);
        for _ in 0..frame.total_local_count() {
            wat.push_str("    (local i32)\n");
        }
        if self.module_runtime_enabled() {
            let cache_size = Layout::MODULE_CACHE_MAX as u32 * Layout::MODULE_CACHE_ENTRY_SIZE;
            wat.push_str(&format!(
                "    (global.set $module_cache (call $alloc_heap (i32.const {cache_size})))\n",
            ));
            wat.push_str("    (global.set $current_module_id (i32.const 1))\n");
        }
        self.emit_class_prototype_initializers(wat, 4);
        self.emit_top_level_statements(wat, 4, &frame);
        wat.push_str("  )\n");
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

pub(super) fn class_prototype_global(id: FuncId) -> String {
    format!("class_proto_{}", id.0)
}

fn add_class_prototype_ref(
    prototype: &ClassPrototypeRef,
    prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
) {
    let mut current = prototype.constructor;
    for parent in &prototype.parent_constructors {
        prototypes
            .entry(current)
            .and_modify(|existing| {
                if existing.is_none() {
                    *existing = Some(*parent);
                }
            })
            .or_insert(Some(*parent));
        prototypes.entry(*parent).or_insert(None);
        current = *parent;
    }
    prototypes.entry(current).or_insert(None);
}

fn class_prototype_depth(
    constructor: FuncId,
    prototypes: &BTreeMap<FuncId, Option<FuncId>>,
) -> usize {
    let mut depth = 0;
    let mut current = constructor;
    while let Some(Some(parent)) = prototypes.get(&current) {
        depth += 1;
        current = *parent;
        if depth > prototypes.len() {
            break;
        }
    }
    depth
}
