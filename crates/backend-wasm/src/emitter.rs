use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{DiagCode, Diagnostic};
use ts2wasm_ir::lowered::{
    BuiltinErrorConstructor, ClassPrototypeRef, FuncId, LocalId, LoweredExpr, LoweredProgram,
    LoweredStmt,
};
use ts2wasm_runtime_abi::Layout;
use ts2wasm_runtime_abi::ValueTag;

use super::runtime_fn::{NATIVE_SET_ADD_SENTINEL, RuntimeFn, RuntimeGlobal, StringOrigin};
use super::runtime_link_plan::RuntimeLinkPlan;
use super::wat_writer::{WatModuleBuilder, WatWriter};

pub(crate) fn emit_wat(program: &LoweredProgram) -> Result<String, Diagnostic> {
    WatEmitter::new(program).emit()
}

pub(super) struct WatEmitter<'a> {
    pub(super) program: &'a LoweredProgram,
    pub(super) link_plan: RuntimeLinkPlan,
    pub(super) strings: HashMap<String, u32>,
    pub(super) string_data: Vec<(u32, String, StringOrigin)>,
    pub(super) next_data_offset: u32,
    /// Set of strings that were interned as runtime-originated (from RuntimeFn spec).
    pub(super) runtime_string_set: HashSet<String>,
    class_name_to_ctor: HashMap<String, FuncId>,
    method_counts: HashMap<FuncId, usize>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct LocalFrame {
    pub(super) user_local_count: usize,
    pub(super) backend_base: usize,
    gc_roots: GcRootStorage,
}

#[derive(Debug, Clone, Copy)]
enum GcRootStorage {
    Disabled,
    StaticTable { base_slot: usize },
    ActivationFrame,
}

impl LocalFrame {
    const BACKEND_TEMP_GROUP_SIZE: usize = 3;
    const BACKEND_TEMP_GROUP_COUNT: usize = 4;

    pub(super) fn new(user_local_count: usize, gc_root_base_slot: Option<usize>) -> Self {
        Self {
            user_local_count,
            backend_base: user_local_count,
            gc_roots: gc_root_base_slot
                .map(|base_slot| GcRootStorage::StaticTable { base_slot })
                .unwrap_or(GcRootStorage::Disabled),
        }
    }

    pub(super) fn activation(user_local_count: usize, enabled: bool) -> Self {
        Self {
            user_local_count,
            backend_base: user_local_count,
            gc_roots: if enabled {
                GcRootStorage::ActivationFrame
            } else {
                GcRootStorage::Disabled
            },
        }
    }

    pub(super) const fn total_local_count(self) -> usize {
        self.user_local_count + self.backend_local_count()
    }

    pub(super) const fn backend_local_count(self) -> usize {
        Self::BACKEND_TEMP_GROUP_SIZE * Self::BACKEND_TEMP_GROUP_COUNT
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

    pub(super) fn child_temp_frame(self) -> Self {
        let next_base = self.backend_base + Self::BACKEND_TEMP_GROUP_SIZE;
        let max_base =
            self.user_local_count + self.backend_local_count() - Self::BACKEND_TEMP_GROUP_SIZE;
        Self {
            backend_base: next_base.min(max_base),
            ..self
        }
    }

    pub(super) fn gc_root_slot(self, local_id: LocalId) -> Option<usize> {
        self.gc_root_slot_for_index(local_id.0)
    }

    pub(super) fn gc_root_slot_for_index(self, local_index: usize) -> Option<usize> {
        if local_index >= self.total_local_count() {
            return None;
        }
        match self.gc_roots {
            GcRootStorage::Disabled | GcRootStorage::ActivationFrame => None,
            GcRootStorage::StaticTable { base_slot } => Some(base_slot + local_index),
        }
    }

    pub(super) fn gc_root_slots(self) -> impl Iterator<Item = (usize, usize)> {
        let base_slot = match self.gc_roots {
            GcRootStorage::StaticTable { base_slot } => Some(base_slot),
            GcRootStorage::Disabled | GcRootStorage::ActivationFrame => None,
        };
        base_slot.into_iter().flat_map(move |base| {
            (0..self.total_local_count()).map(move |local| (local, base + local))
        })
    }

    pub(super) fn uses_activation_roots(self) -> bool {
        matches!(self.gc_roots, GcRootStorage::ActivationFrame)
    }
}

impl<'a> WatEmitter<'a> {
    pub(super) fn new(program: &'a LoweredProgram) -> Self {
        let link_plan = RuntimeLinkPlan::from_program(program);
        let mut class_name_to_ctor = HashMap::new();
        let mut method_counts = HashMap::new();
        Self::compute_class_decl_metadata(
            &program.top_level_statements,
            &mut class_name_to_ctor,
            &mut method_counts,
        );
        let mut emitter = Self {
            program,
            link_plan,
            strings: HashMap::new(),
            string_data: Vec::new(),
            runtime_string_set: HashSet::new(),
            next_data_offset: Layout::DATA_START,
            class_name_to_ctor,
            method_counts,
        };
        emitter.intern_required_runtime_strings();
        emitter.collect_program_strings(&program.top_level_statements);
        for function in &program.functions {
            emitter.collect_program_strings(&function.body);
        }
        for module in &program.modules {
            emitter.collect_program_strings(&module.statements);
        }
        emitter
    }

    fn emit(mut self) -> Result<String, Diagnostic> {
        self.validate_memory_layout()?;
        let _required_capabilities = self.link_plan.required_capabilities();
        let mut writer = WatWriter::new();
        writer.open_module();

        // Buffer for methods that still output to &mut String (cross-file API).
        let mut buf = String::new();

        // emit_imports_from_catalog uses WatModuleBuilder internally.
        self.emit_imports_from_catalog(&mut buf);
        writer.push_str(&buf);
        buf.clear();

        writer.line_fmt(
            2,
            format_args!(
                "(memory (export \"memory\") {} {})",
                Layout::MEMORY_MIN_PAGES,
                Layout::MEMORY_MAX_PAGES,
            ),
        );
        writer.line_fmt(
            2,
            format_args!(
                "(global $heap (mut i32) (i32.const {}))",
                Layout::HEAP_START,
            ),
        );

        // emit_required_globals uses WatModuleBuilder internally.
        self.emit_required_globals(&mut buf);
        writer.push_str(&buf);
        buf.clear();

        self.emit_class_prototype_globals(&mut writer);
        self.emit_builtin_error_prototype_globals(&mut writer);

        self.emit_data_segments(&mut buf);
        writer.push_str(&buf);
        buf.clear();

        self.emit_runtime(&mut buf);
        writer.push_str(&buf);
        buf.clear();

        if self
            .link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::SetFromArray)
        {
            self.emit_set_add_dispatcher(&mut writer);
        }
        if self
            .link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::JsonStringify)
        {
            self.emit_json_replacer_dispatcher(&mut writer);
        }

        self.emit_functions(&mut writer);

        self.emit_module_initializers(&mut writer);

        self.emit_start(&mut writer);

        writer.close_module();
        Ok(writer.into_string())
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
        if !Layout::HEAP_START.is_multiple_of(Layout::ALIGN) {
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
            // Use the first origin from the RuntimeFn catalog as the representative origin.
            let origin = self
                .link_plan
                .string_origins()
                .get(value)
                .and_then(|origins| origins.first())
                .map(|rf| StringOrigin::Runtime(*rf))
                .unwrap_or(StringOrigin::UserLiteral);
            self.intern_string_with_origin(value, origin);
        }
    }

    fn collect_program_strings(&mut self, statements: &[LoweredStmt]) {
        for statement in statements {
            self.collect_statement_strings(statement);
        }
    }

    fn collect_statement_strings(&mut self, statement: &LoweredStmt) {
        match statement {
            LoweredStmt::Block(statements, _) => {
                self.collect_program_strings(statements);
            }
            LoweredStmt::Let(_, expr, _)
            | LoweredStmt::Assign(_, expr, _)
            | LoweredStmt::Expr(expr, _)
            | LoweredStmt::Return(expr, _)
            | LoweredStmt::Throw(expr, _) => {
                self.collect_expr_strings(expr);
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.collect_expr_strings(condition);
                self.collect_program_strings(then_body);
                self.collect_program_strings(else_body);
            }
            LoweredStmt::While {
                condition, body, ..
            } => {
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
            LoweredStmt::Switch { expr, cases, .. } => {
                self.collect_expr_strings(expr);
                for (cond, body) in cases {
                    if let Some(cond_expr) = cond {
                        self.collect_expr_strings(cond_expr);
                    }
                    self.collect_program_strings(body);
                }
            }
            LoweredStmt::DoWhile {
                body, condition, ..
            } => {
                self.collect_program_strings(body);
                self.collect_expr_strings(condition);
            }
            LoweredStmt::For {
                init,
                condition,
                update,
                body,
                ..
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
            LoweredStmt::Labeled { body, .. } => self.collect_statement_strings(body),
            LoweredStmt::Break { .. } | LoweredStmt::Continue { .. } => {}
            LoweredStmt::Export { name, expr, .. } => {
                self.intern_string(name);
                self.collect_expr_strings(expr);
            }
            LoweredStmt::ModuleExportsAssign { expr, .. } => {
                self.collect_expr_strings(expr);
            }
            LoweredStmt::ClassDecl {
                methods,
                static_methods,
                ..
            } => {
                for (name, _) in methods.iter().chain(static_methods.iter()) {
                    self.intern_string(name);
                }
            }
        }
    }

    fn collect_expr_strings(&mut self, expr: &LoweredExpr) {
        match expr {
            LoweredExpr::String(value, _) => {
                self.intern_string(value);
            }
            LoweredExpr::BigIntLiteral { decimal, .. } => {
                self.intern_string(decimal);
            }
            LoweredExpr::Number(_, _)
            | LoweredExpr::Bool(_, _)
            | LoweredExpr::Null(..)
            | LoweredExpr::Undefined(..)
            | LoweredExpr::This(..)
            | LoweredExpr::Local(_, _)
            | LoweredExpr::ArrowFn { .. } => {}
            LoweredExpr::Unary { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::Assign { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::EnvCellNew(expr, _) => self.collect_expr_strings(expr),
            LoweredExpr::EnvCellGet(_, _) => {}
            LoweredExpr::EnvCellSet { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::LogicalAssign { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::LogicalPropertyAssign { key, expr, .. } => {
                self.intern_string(key);
                self.collect_expr_strings(expr);
            }
            LoweredExpr::LogicalMemberAssign {
                object, key, expr, ..
            } => {
                self.collect_expr_strings(object);
                self.intern_string(key);
                self.collect_expr_strings(expr);
            }
            LoweredExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
                self.collect_expr_strings(key);
                self.collect_expr_strings(expr);
            }
            LoweredExpr::LogicalComputedMemberAssign {
                object, key, expr, ..
            } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(key);
                self.collect_expr_strings(expr);
            }
            LoweredExpr::Binary { left, right, .. } => {
                self.collect_expr_strings(left);
                self.collect_expr_strings(right);
            }
            LoweredExpr::Call { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            LoweredExpr::PropertyDelete { object, key, .. } => {
                self.collect_expr_strings(object);
                self.intern_string(key);
            }
            LoweredExpr::PropertyDeleteDynamic { object, key, .. } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(key);
            }
            LoweredExpr::PropertyIn { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.intern_string(key);
            }
            LoweredExpr::PropertyInDynamic { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.collect_expr_strings(key);
            }
            LoweredExpr::ArrayNew { elements, .. } => {
                for elem in elements {
                    self.collect_expr_strings(elem);
                }
            }
            LoweredExpr::ArrayNewSparse { slots, .. } => {
                for slot in slots {
                    if let ts2wasm_ir::lowered::LoweredArraySlot::Present(elem) = slot {
                        self.collect_expr_strings(elem);
                    }
                }
            }
            LoweredExpr::ArrayGet { arr, index, .. } => {
                self.collect_expr_strings(arr);
                self.collect_expr_strings(index);
            }
            LoweredExpr::Index { object, index, .. } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(index);
            }
            LoweredExpr::GetLength(inner, _) => {
                self.collect_expr_strings(inner);
            }
            LoweredExpr::ObjectNew { props, .. } => {
                for (key, val) in props {
                    self.intern_string(key);
                    self.collect_expr_strings(val);
                }
            }
            LoweredExpr::ErrorNew {
                constructor,
                message,
                ..
            } => {
                self.intern_string("message");
                self.intern_string("stack");
                self.intern_string(builtin_error_stack_prefix(*constructor));
                self.collect_expr_strings(message);
            }
            LoweredExpr::PropertyGet { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.intern_string(key);
            }
            LoweredExpr::OptionalPropertyGet { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.intern_string(key);
            }
            LoweredExpr::PropertyGetDynamic { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.collect_expr_strings(key);
            }
            LoweredExpr::OptionalIndex { object, index, .. } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(index);
            }
            LoweredExpr::OptionalCall { callee, call, .. } => {
                self.collect_expr_strings(callee);
                self.collect_expr_strings(call);
            }
            LoweredExpr::MethodCall { object, .. } => {
                self.collect_expr_strings(object);
            }
            LoweredExpr::PropertySet {
                object, key, value, ..
            } => {
                self.collect_expr_strings(object);
                self.intern_string(key);
                self.collect_expr_strings(value);
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
                ..
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
            LoweredExpr::ClassPrototype(_, _) | LoweredExpr::BuiltinErrorPrototype(_, _) => {}
            LoweredExpr::Block { stmts, result, .. } => {
                for stmt in stmts {
                    self.collect_statement_strings(stmt);
                }
                self.collect_expr_strings(result);
            }
            LoweredExpr::ModuleLoad { .. } => {}
            LoweredExpr::RuntimeCall { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
        }
    }

    fn emit_class_prototype_globals(&self, writer: &mut WatWriter) {
        for constructor in self.class_prototypes().keys() {
            writer.line_fmt(
                2,
                format_args!(
                    "(global ${} (mut i32) (i32.const 0))",
                    class_prototype_global(*constructor),
                ),
            );
        }
    }

    fn emit_builtin_error_prototype_globals(&self, writer: &mut WatWriter) {
        for constructor in self.builtin_error_prototypes() {
            writer.line_fmt(
                2,
                format_args!(
                    "(global ${} (mut i32) (i32.const 0))",
                    builtin_error_prototype_global(constructor),
                ),
            );
        }
    }

    fn emit_class_prototype_initializers(&self, wat: &mut String, indent: usize) {
        let pad = " ".repeat(indent);
        for (constructor, parent) in self.ordered_class_prototypes() {
            let global = class_prototype_global(constructor);
            let method_count = self.method_counts.get(&constructor).copied().unwrap_or(0);
            let size = Layout::OBJECT_HEADER_SIZE + method_count as u32 * Layout::OBJECT_ENTRY_SIZE;
            wat.push_str(&format!(
                "{pad}(if (i32.eqz (global.get ${global}))\n{pad}  (then\n"
            ));
            wat.push_str(&format!(
                "{pad}    (global.set ${global} (call {} (i32.const {size})))\n",
                super::runtime_fn::RuntimeFn::AllocHeap.symbol(),
            ));
            wat.push_str(&format!(
                "{pad}    (i32.store (global.get ${global}) (i32.const 0))\n"
            ));
            wat.push_str(&format!(
                "{pad}    (i32.store (i32.add (global.get ${global}) (i32.const {})) (i32.const 0))\n",
                Layout::OBJECT_FLAGS_OFFSET,
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

    fn emit_builtin_error_prototype_initializers(&self, wat: &mut String, indent: usize) {
        let pad = " ".repeat(indent);
        for constructor in self.builtin_error_prototypes() {
            let global = builtin_error_prototype_global(constructor);
            wat.push_str(&format!(
                "{pad}(if (i32.eqz (global.get ${global}))\n{pad}  (then\n"
            ));
            wat.push_str(&format!(
                "{pad}    (global.set ${global} (call {} (i32.const {})))\n",
                RuntimeFn::AllocHeap.symbol(),
                Layout::OBJECT_HEADER_SIZE,
            ));
            wat.push_str(&format!(
                "{pad}    (i32.store (global.get ${global}) (i32.const 0))\n"
            ));
            wat.push_str(&format!(
                "{pad}    (i32.store (i32.add (global.get ${global}) (i32.const {})) (i32.const 0))\n",
                Layout::OBJECT_FLAGS_OFFSET,
            ));
            let parent_expr = constructor
                .parent()
                .map(|parent| format!("global.get ${}", builtin_error_prototype_global(parent)))
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
        Self::collect_class_decl_prototypes(
            &self.program.top_level_statements,
            &mut prototypes,
            &self.class_name_to_ctor,
        );
        Self::collect_class_prototypes_from_stmts(
            &self.program.top_level_statements,
            &mut prototypes,
        );
        for function in &self.program.functions {
            Self::collect_class_decl_prototypes(
                &function.body,
                &mut prototypes,
                &self.class_name_to_ctor,
            );
            Self::collect_class_prototypes_from_stmts(&function.body, &mut prototypes);
        }
        prototypes
    }

    pub(super) fn builtin_error_prototypes(&self) -> BTreeSet<BuiltinErrorConstructor> {
        let mut prototypes = BTreeSet::new();
        if self
            .link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::BigIntDivisionByZeroRangeError)
        {
            add_builtin_error_prototype_ref(BuiltinErrorConstructor::RangeError, &mut prototypes);
        }
        if self
            .link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::BigIntMixedArithmeticTypeError)
            || self
                .link_plan
                .required_runtime_functions()
                .contains(&RuntimeFn::PrivateBrandTypeError)
        {
            add_builtin_error_prototype_ref(BuiltinErrorConstructor::TypeError, &mut prototypes);
        }
        Self::collect_builtin_error_prototypes_from_stmts(
            &self.program.top_level_statements,
            &mut prototypes,
        );
        for function in &self.program.functions {
            Self::collect_builtin_error_prototypes_from_stmts(&function.body, &mut prototypes);
        }
        prototypes
    }

    fn collect_class_prototypes_from_stmts(
        stmts: &[LoweredStmt],
        prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
    ) {
        for stmt in stmts {
            match stmt {
                LoweredStmt::Block(statements, _) => {
                    Self::collect_class_prototypes_from_stmts(statements, prototypes);
                }
                LoweredStmt::Let(_, expr, _)
                | LoweredStmt::Assign(_, expr, _)
                | LoweredStmt::Expr(expr, _)
                | LoweredStmt::Return(expr, _)
                | LoweredStmt::Throw(expr, _)
                | LoweredStmt::Export { expr, .. }
                | LoweredStmt::ModuleExportsAssign { expr, .. } => {
                    Self::collect_class_prototypes_from_expr(expr, prototypes);
                }
                LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    Self::collect_class_prototypes_from_expr(condition, prototypes);
                    Self::collect_class_prototypes_from_stmts(then_body, prototypes);
                    Self::collect_class_prototypes_from_stmts(else_body, prototypes);
                }
                LoweredStmt::While {
                    condition, body, ..
                }
                | LoweredStmt::DoWhile {
                    body, condition, ..
                } => {
                    Self::collect_class_prototypes_from_expr(condition, prototypes);
                    Self::collect_class_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::TryCatch {
                    try_body,
                    catch_body,
                    finally_body,
                    ..
                } => {
                    Self::collect_class_prototypes_from_stmts(try_body, prototypes);
                    if let Some(body) = catch_body {
                        Self::collect_class_prototypes_from_stmts(body, prototypes);
                    }
                    if let Some(body) = finally_body {
                        Self::collect_class_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::Switch { expr, cases, .. } => {
                    Self::collect_class_prototypes_from_expr(expr, prototypes);
                    for (case_expr, body) in cases {
                        if let Some(case_expr) = case_expr {
                            Self::collect_class_prototypes_from_expr(case_expr, prototypes);
                        }
                        Self::collect_class_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::For {
                    init,
                    condition,
                    update,
                    body,
                    ..
                } => {
                    if let Some(init) = init {
                        Self::collect_class_prototypes_from_stmts(
                            std::slice::from_ref(init.as_ref()),
                            prototypes,
                        );
                    }
                    if let Some(condition) = condition {
                        Self::collect_class_prototypes_from_expr(condition, prototypes);
                    }
                    if let Some(update) = update {
                        Self::collect_class_prototypes_from_expr(update, prototypes);
                    }
                    Self::collect_class_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::ForIn { iter, body, .. } | LoweredStmt::ForOf { iter, body, .. } => {
                    Self::collect_class_prototypes_from_expr(iter, prototypes);
                    Self::collect_class_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::Labeled { body, .. } => Self::collect_class_prototypes_from_stmts(
                    std::slice::from_ref(body.as_ref()),
                    prototypes,
                ),
                LoweredStmt::Break { .. }
                | LoweredStmt::Continue { .. }
                | LoweredStmt::ClassDecl { .. } => {}
            }
        }
    }

    fn collect_builtin_error_prototypes_from_stmts(
        stmts: &[LoweredStmt],
        prototypes: &mut BTreeSet<BuiltinErrorConstructor>,
    ) {
        for stmt in stmts {
            match stmt {
                LoweredStmt::Block(statements, _) => {
                    Self::collect_builtin_error_prototypes_from_stmts(statements, prototypes);
                }
                LoweredStmt::Let(_, expr, _)
                | LoweredStmt::Assign(_, expr, _)
                | LoweredStmt::Expr(expr, _)
                | LoweredStmt::Return(expr, _)
                | LoweredStmt::Throw(expr, _)
                | LoweredStmt::Export { expr, .. }
                | LoweredStmt::ModuleExportsAssign { expr, .. } => {
                    Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
                }
                LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    Self::collect_builtin_error_prototypes_from_expr(condition, prototypes);
                    Self::collect_builtin_error_prototypes_from_stmts(then_body, prototypes);
                    Self::collect_builtin_error_prototypes_from_stmts(else_body, prototypes);
                }
                LoweredStmt::While {
                    condition, body, ..
                }
                | LoweredStmt::DoWhile {
                    body, condition, ..
                } => {
                    Self::collect_builtin_error_prototypes_from_expr(condition, prototypes);
                    Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::TryCatch {
                    try_body,
                    catch_body,
                    finally_body,
                    ..
                } => {
                    Self::collect_builtin_error_prototypes_from_stmts(try_body, prototypes);
                    if let Some(body) = catch_body {
                        Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                    }
                    if let Some(body) = finally_body {
                        Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::Switch { expr, cases, .. } => {
                    Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
                    for (case_expr, body) in cases {
                        if let Some(case_expr) = case_expr {
                            Self::collect_builtin_error_prototypes_from_expr(case_expr, prototypes);
                        }
                        Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::For {
                    init,
                    condition,
                    update,
                    body,
                    ..
                } => {
                    if let Some(stmt) = init {
                        Self::collect_builtin_error_prototypes_from_stmts(
                            std::slice::from_ref(stmt.as_ref()),
                            prototypes,
                        );
                    }
                    if let Some(condition) = condition {
                        Self::collect_builtin_error_prototypes_from_expr(condition, prototypes);
                    }
                    if let Some(update) = update {
                        Self::collect_builtin_error_prototypes_from_expr(update, prototypes);
                    }
                    Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::ForIn { iter, body, .. } | LoweredStmt::ForOf { iter, body, .. } => {
                    Self::collect_builtin_error_prototypes_from_expr(iter, prototypes);
                    Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::Labeled { body, .. } => {
                    Self::collect_builtin_error_prototypes_from_stmts(
                        std::slice::from_ref(body.as_ref()),
                        prototypes,
                    )
                }
                LoweredStmt::Break { .. }
                | LoweredStmt::Continue { .. }
                | LoweredStmt::ClassDecl { .. } => {}
            }
        }
    }

    fn collect_class_prototypes_from_expr(
        expr: &LoweredExpr,
        prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
    ) {
        match expr {
            LoweredExpr::ClassPrototype(prototype, _) => {
                add_class_prototype_ref(prototype, prototypes);
            }
            LoweredExpr::BuiltinErrorPrototype(_, _) | LoweredExpr::ErrorNew { .. } => {}
            LoweredExpr::Block { stmts, result, .. } => {
                Self::collect_class_prototypes_from_stmts(stmts, prototypes);
                Self::collect_class_prototypes_from_expr(result, prototypes);
            }
            LoweredExpr::New {
                prototype, args, ..
            } => {
                add_class_prototype_ref(prototype, prototypes);
                for arg in args {
                    Self::collect_class_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::Unary { expr, .. }
            | LoweredExpr::GetLength(expr, _)
            | LoweredExpr::PropertyGet { obj: expr, .. }
            | LoweredExpr::OptionalPropertyGet { obj: expr, .. }
            | LoweredExpr::MethodCall { object: expr, .. }
            | LoweredExpr::PropertyDelete { object: expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::Binary { left, right, .. } => {
                Self::collect_class_prototypes_from_expr(left, prototypes);
                Self::collect_class_prototypes_from_expr(right, prototypes);
            }
            LoweredExpr::PropertyIn { obj, .. } => {
                Self::collect_class_prototypes_from_expr(obj, prototypes);
            }
            LoweredExpr::PropertyInDynamic { obj, key, .. }
            | LoweredExpr::ArrayGet {
                arr: obj,
                index: key,
                ..
            }
            | LoweredExpr::Index {
                object: obj,
                index: key,
                ..
            }
            | LoweredExpr::OptionalIndex {
                object: obj,
                index: key,
                ..
            }
            | LoweredExpr::PropertyGetDynamic { obj, key, .. }
            | LoweredExpr::PropertyDeleteDynamic {
                object: obj, key, ..
            } => {
                Self::collect_class_prototypes_from_expr(obj, prototypes);
                Self::collect_class_prototypes_from_expr(key, prototypes);
            }
            LoweredExpr::OptionalCall { callee, call, .. } => {
                Self::collect_class_prototypes_from_expr(callee, prototypes);
                Self::collect_class_prototypes_from_expr(call, prototypes);
            }
            LoweredExpr::Call { args, .. } | LoweredExpr::RuntimeCall { args, .. } => {
                for arg in args {
                    Self::collect_class_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::ArrayNew { elements, .. } => {
                for elem in elements {
                    Self::collect_class_prototypes_from_expr(elem, prototypes);
                }
            }
            LoweredExpr::ArrayNewSparse { slots, .. } => {
                for slot in slots {
                    if let ts2wasm_ir::lowered::LoweredArraySlot::Present(elem) = slot {
                        Self::collect_class_prototypes_from_expr(elem, prototypes);
                    }
                }
            }
            LoweredExpr::ObjectNew { props, .. } => {
                for (_, value) in props {
                    Self::collect_class_prototypes_from_expr(value, prototypes);
                }
            }
            LoweredExpr::PropertySet { object, value, .. } => {
                Self::collect_class_prototypes_from_expr(object, prototypes);
                Self::collect_class_prototypes_from_expr(value, prototypes);
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
                ..
            } => {
                Self::collect_class_prototypes_from_expr(object, prototypes);
                Self::collect_class_prototypes_from_expr(index, prototypes);
                Self::collect_class_prototypes_from_expr(value, prototypes);
            }
            LoweredExpr::Assign { expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::EnvCellNew(expr, _) => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::EnvCellGet(_, _) => {}
            LoweredExpr::EnvCellSet { expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalAssign { expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalPropertyAssign { expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalMemberAssign { object, expr, .. } => {
                Self::collect_class_prototypes_from_expr(object, prototypes);
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
                Self::collect_class_prototypes_from_expr(key, prototypes);
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalComputedMemberAssign {
                object, key, expr, ..
            } => {
                Self::collect_class_prototypes_from_expr(object, prototypes);
                Self::collect_class_prototypes_from_expr(key, prototypes);
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::Number(_, _)
            | LoweredExpr::String(_, _)
            | LoweredExpr::BigIntLiteral { .. }
            | LoweredExpr::Bool(_, _)
            | LoweredExpr::Null(..)
            | LoweredExpr::Undefined(..)
            | LoweredExpr::Local(_, _)
            | LoweredExpr::ModuleLoad { .. }
            | LoweredExpr::This(..)
            | LoweredExpr::ArrowFn { .. } => {}
        }
    }

    fn collect_builtin_error_prototypes_from_expr(
        expr: &LoweredExpr,
        prototypes: &mut BTreeSet<BuiltinErrorConstructor>,
    ) {
        match expr {
            LoweredExpr::BuiltinErrorPrototype(constructor, _) => {
                add_builtin_error_prototype_ref(*constructor, prototypes);
            }
            LoweredExpr::Block { stmts, result, .. } => {
                Self::collect_builtin_error_prototypes_from_stmts(stmts, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(result, prototypes);
            }
            LoweredExpr::ErrorNew {
                constructor,
                message,
                ..
            } => {
                add_builtin_error_prototype_ref(*constructor, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(message, prototypes);
            }
            LoweredExpr::Unary { expr, .. }
            | LoweredExpr::GetLength(expr, _)
            | LoweredExpr::PropertyGet { obj: expr, .. }
            | LoweredExpr::OptionalPropertyGet { obj: expr, .. }
            | LoweredExpr::MethodCall { object: expr, .. }
            | LoweredExpr::PropertyDelete { object: expr, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::Binary { left, right, .. }
            | LoweredExpr::PropertyGetDynamic {
                obj: left,
                key: right,
                ..
            }
            | LoweredExpr::Index {
                object: left,
                index: right,
                ..
            }
            | LoweredExpr::OptionalIndex {
                object: left,
                index: right,
                ..
            }
            | LoweredExpr::ArrayGet {
                arr: left,
                index: right,
                ..
            }
            | LoweredExpr::PropertyDeleteDynamic {
                object: left,
                key: right,
                ..
            }
            | LoweredExpr::PropertyInDynamic {
                obj: left,
                key: right,
                ..
            } => {
                Self::collect_builtin_error_prototypes_from_expr(left, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(right, prototypes);
            }
            LoweredExpr::OptionalCall { callee, call, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(callee, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(call, prototypes);
            }
            LoweredExpr::PropertySet { object, value, .. }
            | LoweredExpr::PropertySetDynamic { object, value, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(object, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(value, prototypes);
            }
            LoweredExpr::Call { args, .. } | LoweredExpr::RuntimeCall { args, .. } => {
                for arg in args {
                    Self::collect_builtin_error_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::ArrayNew { elements, .. } => {
                for elem in elements {
                    Self::collect_builtin_error_prototypes_from_expr(elem, prototypes);
                }
            }
            LoweredExpr::ArrayNewSparse { slots, .. } => {
                for slot in slots {
                    if let ts2wasm_ir::lowered::LoweredArraySlot::Present(elem) = slot {
                        Self::collect_builtin_error_prototypes_from_expr(elem, prototypes);
                    }
                }
            }
            LoweredExpr::ObjectNew { props, .. } => {
                for (_, value) in props {
                    Self::collect_builtin_error_prototypes_from_expr(value, prototypes);
                }
            }
            LoweredExpr::New { args, .. } => {
                for arg in args {
                    Self::collect_builtin_error_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::Assign { expr, .. }
            | LoweredExpr::EnvCellNew(expr, _)
            | LoweredExpr::EnvCellSet { expr, .. }
            | LoweredExpr::LogicalAssign { expr, .. }
            | LoweredExpr::LogicalPropertyAssign { expr, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::EnvCellGet(_, _) => {}
            LoweredExpr::LogicalMemberAssign { object, expr, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(object, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(key, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalComputedMemberAssign {
                object, key, expr, ..
            } => {
                Self::collect_builtin_error_prototypes_from_expr(object, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(key, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::PropertyIn { obj, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(obj, prototypes);
            }
            LoweredExpr::Number(_, _)
            | LoweredExpr::String(_, _)
            | LoweredExpr::BigIntLiteral { .. }
            | LoweredExpr::Bool(_, _)
            | LoweredExpr::Null(..)
            | LoweredExpr::Undefined(..)
            | LoweredExpr::Local(_, _)
            | LoweredExpr::ClassPrototype(_, _)
            | LoweredExpr::ModuleLoad { .. }
            | LoweredExpr::This(..)
            | LoweredExpr::ArrowFn { .. } => {}
        }
    }

    fn collect_class_decl_prototypes(
        stmts: &[LoweredStmt],
        prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
        class_name_to_ctor: &HashMap<String, FuncId>,
    ) {
        for stmt in stmts {
            match stmt {
                LoweredStmt::ClassDecl {
                    constructor: Some(ctor_id),
                    extends,
                    ..
                } => {
                    let parent = extends
                        .as_ref()
                        .and_then(|name| class_name_to_ctor.get(name))
                        .copied();
                    prototypes.entry(*ctor_id).or_insert(parent);
                    if let Some(parent_id) = parent {
                        prototypes.entry(parent_id).or_insert(None);
                    }
                }
                LoweredStmt::Block(statements, _) => {
                    Self::collect_class_decl_prototypes(statements, prototypes, class_name_to_ctor);
                }
                _ => {}
            }
        }
    }

    fn compute_class_decl_metadata(
        stmts: &[LoweredStmt],
        class_name_to_ctor: &mut HashMap<String, FuncId>,
        method_counts: &mut HashMap<FuncId, usize>,
    ) {
        for stmt in stmts {
            match stmt {
                LoweredStmt::ClassDecl {
                    name,
                    constructor: Some(ctor_id),
                    methods,
                    ..
                } => {
                    class_name_to_ctor.insert(name.clone(), *ctor_id);
                    method_counts.insert(*ctor_id, methods.len());
                }
                LoweredStmt::Block(statements, _) => {
                    Self::compute_class_decl_metadata(
                        statements,
                        class_name_to_ctor,
                        method_counts,
                    );
                }
                _ => {}
            }
        }
    }

    fn emit_functions(&self, writer: &mut WatWriter) {
        let mut buf = String::new();
        for function in &self.program.functions {
            writer.push_str(&format!("  (func ${} ", function_symbol(function.id)));
            for _ in &function.params {
                writer.push_str("(param i32) ");
            }
            writer.push_str("(result i32)\n");
            for _ in &function.locals {
                writer.push_str("    (local i32)\n");
            }
            let frame = LocalFrame::activation(
                function.params.len() + function.locals.len(),
                self.gc_call_frame_roots_enabled(),
            );
            // Backend-owned temporaries for heap construction and switch dispatch.
            for _ in 0..frame.backend_local_count() {
                writer.push_str("    (local i32)\n");
            }
            buf.clear();
            self.emit_gc_activation_frame_push(&mut buf, &frame, 4);
            self.emit_gc_root_param_initializer(&mut buf, &frame, 4);
            writer.push_str(&buf);
            buf.clear();
            let mut loop_ctx = super::stmt_emit::LoopContext::default();
            self.emit_statements(writer, &function.body, 4, &mut loop_ctx, &frame);
            buf.clear();
            self.emit_gc_activation_frame_pop(&mut buf, &frame, 4);
            writer.push_str(&buf);
            writer.push_str(&format!("    (i32.const {})\n", ValueTag::UNDEFINED));
            writer.push_str("  )\n");
        }
    }

    fn emit_json_replacer_dispatcher(&self, writer: &mut WatWriter) {
        writer.push_str(
            "  (func $json_replacer_call (param $callback i32) (param $holder i32) (param $key i32) (param $value i32) (result i32)\n",
        );
        writer.line(4, "(local $id i32)");
        writer.push_str(&format!(
            "    (if (i32.ne (i32.and (local.get $callback) (i32.const {})) (i32.const {}))\n      (then (return (local.get $value))))\n",
            ValueTag::TAG_MASK,
            ValueTag::NUMBER,
        ));
        writer.line_fmt(
            4,
            format_args!(
                "(local.set $id (i32.shr_s (local.get $callback) (i32.const {})))",
                ValueTag::NUMBER_SHIFT,
            ),
        );

        for function in &self.program.functions {
            writer.line_fmt(
                4,
                format_args!(
                    "(if (i32.eq (local.get $id) (i32.const {})))",
                    function.id.0
                ),
            );
            writer.then(4);
            let mut supplied = 0usize;
            if function.uses_receiver {
                writer.line(8, "(local.get $holder)");
                supplied += 1;
            }
            if supplied < function.params.len() {
                writer.line(8, "(local.get $key)");
                supplied += 1;
            }
            if supplied < function.params.len() {
                writer.line(8, "(local.get $value)");
                supplied += 1;
            }
            for _ in supplied..function.params.len() {
                writer.line_fmt(8, format_args!("(i32.const {})", ValueTag::UNDEFINED));
            }
            writer.line_fmt(
                8,
                format_args!("(return (call ${}))))", function_symbol(function.id)),
            );
        }

        writer.line(4, "(local.get $value))");
    }

    fn emit_set_add_dispatcher(&self, writer: &mut WatWriter) {
        writer.push_str(
            "  (func $set_add_dispatch (param $set i32) (param $value i32) (result i32)\n",
        );
        writer.line(4, "(local $callback i32)");
        writer.line(4, "(local $id i32)");
        writer.line(4, "(local.set $callback (global.get $set_prototype_add))");
        writer.line_fmt(
            4,
            format_args!(
                "(if (i32.eq (local.get $callback) (i32.const {native}))",
                native = NATIVE_SET_ADD_SENTINEL,
            ),
        );
        writer.line(
            6,
            "(then (return (call $set_add (local.get $set) (local.get $value))))",
        );
        writer.line_fmt(
            4,
            format_args!(
                "(if (i32.ne (i32.and (local.get $callback) (i32.const {tag_mask})) (i32.const {number_tag}))",
                tag_mask = ValueTag::TAG_MASK,
                number_tag = ValueTag::NUMBER,
            ),
        );
        writer.line_fmt(
            6,
            format_args!(
                "(then (return (i32.const {undefined})))",
                undefined = ValueTag::UNDEFINED
            ),
        );
        writer.line_fmt(
            4,
            format_args!(
                "(local.set $id (i32.shr_s (local.get $callback) (i32.const {number_shift})))",
                number_shift = ValueTag::NUMBER_SHIFT,
            ),
        );

        for function in &self.program.functions {
            writer.line_fmt(
                4,
                format_args!(
                    "(if (i32.eq (local.get $id) (i32.const {})))",
                    function.id.0
                ),
            );
            writer.then(4);
            let mut supplied = 0usize;
            if function.uses_receiver {
                writer.line(8, "(local.get $set)");
                supplied += 1;
            }
            if supplied < function.params.len() {
                writer.line(8, "(local.get $value)");
                supplied += 1;
            }
            for _ in supplied..function.params.len() {
                writer.line_fmt(8, format_args!("(i32.const {})", ValueTag::UNDEFINED));
            }
            writer.line_fmt(
                8,
                format_args!("(return (call ${}))))", function_symbol(function.id)),
            );
        }

        writer.line(4, "(call $set_add (local.get $set) (local.get $value))");
    }

    fn emit_module_initializers(&self, writer: &mut WatWriter) {
        for module in &self.program.modules {
            if module.statements.is_empty() {
                continue;
            }
            writer.push_str(&format!("  (func ${}\n", module_init_symbol(module.id)));
            let frame =
                LocalFrame::activation(module.locals_count, self.gc_call_frame_roots_enabled());
            for _ in 0..frame.total_local_count() {
                writer.push_str("    (local i32)\n");
            }
            let mut buf = String::new();
            self.emit_gc_activation_frame_push(&mut buf, &frame, 4);
            writer.push_str(&buf);
            buf.clear();
            writer.push_str(&format!(
                "    (global.set $current_module_id (i32.const {}))\n",
                module.id
            ));
            let mut loop_ctx = super::stmt_emit::LoopContext::default();
            self.emit_statements(writer, &module.statements, 4, &mut loop_ctx, &frame);
            buf.clear();
            self.emit_gc_activation_frame_pop(&mut buf, &frame, 4);
            writer.push_str(&buf);
            writer.push_str("  )\n");
        }
    }

    fn emit_start(&self, writer: &mut WatWriter) {
        writer.push_str("  (func $_start (export \"_start\")\n");
        let extra_locals = if self.module_runtime_enabled() { 1 } else { 0 };
        let frame = LocalFrame::new(
            self.program.top_level_locals.len() + extra_locals,
            self.gc_root_table_enabled().then_some(0),
        );
        for _ in 0..frame.total_local_count() {
            writer.push_str("    (local i32)\n");
        }
        let mut buf = String::new();
        self.emit_gc_root_table_initializer(&mut buf, 4);
        writer.push_str(&buf);
        if self.module_runtime_enabled() {
            let cache_size = Layout::MODULE_CACHE_MAX * Layout::MODULE_CACHE_ENTRY_SIZE;
            writer.push_str(&format!(
                "    (global.set $module_cache (call $alloc_heap (i32.const {cache_size})))\n",
            ));
            writer.push_str("    (global.set $current_module_id (i32.const 1))\n");
        }
        buf.clear();
        self.emit_class_prototype_initializers(&mut buf, 4);
        writer.push_str(&buf);
        buf.clear();
        self.emit_builtin_error_prototype_initializers(&mut buf, 4);
        writer.push_str(&buf);
        buf.clear();
        self.emit_module_initializer_calls(&mut buf, 4);
        writer.push_str(&buf);
        if self.module_runtime_enabled() {
            writer.push_str("    (global.set $current_module_id (i32.const 1))\n");
        }
        self.emit_top_level_statements(writer, 4, &frame);
        writer.push_str("  )\n");
    }

    fn emit_module_initializer_calls(&self, wat: &mut String, indent: usize) {
        let pad = " ".repeat(indent);
        for module in &self.program.modules {
            if module.statements.is_empty() {
                continue;
            }
            wat.push_str(&format!("{pad}(call ${})\n", module_init_symbol(module.id)));
        }
    }

    fn emit_gc_root_table_initializer(&self, wat: &mut String, indent: usize) {
        let root_count = self.gc_root_slot_count();
        if (root_count == 0 && !self.gc_call_frame_roots_enabled()) || !self.gc_root_table_enabled()
        {
            return;
        }
        let pad = " ".repeat(indent);
        let static_root_bytes = root_count * std::mem::size_of::<u32>();
        let call_frame_root_bytes = if self.gc_call_frame_roots_enabled() {
            Layout::GC_CALL_FRAME_ROOT_STACK_BYTES as usize
        } else {
            0
        };
        let root_bytes = static_root_bytes + call_frame_root_bytes;
        wat.push_str(&format!(
            "{pad}(global.set $gc_root_count (i32.const {root_count}))\n",
        ));
        wat.push_str(&format!(
            "{pad}(global.set $gc_root_base (call {} (i32.const {root_bytes})))\n",
            RuntimeFn::AllocHeap.symbol(),
        ));
        if self.gc_call_frame_roots_enabled() {
            wat.push_str(&format!(
                "{pad}(global.set $gc_call_frame_base (i32.add (global.get $gc_root_base) (i32.const {static_root_bytes})))\n",
            ));
            wat.push_str(&format!(
                "{pad}(global.set $gc_call_frame_top (global.get $gc_call_frame_base))\n",
            ));
            wat.push_str(&format!(
                "{pad}(global.set $gc_call_frame_limit (i32.add (global.get $gc_call_frame_base) (i32.const {call_frame_root_bytes})))\n",
            ));
            wat.push_str(&format!(
                "{pad}(global.set $gc_call_frame_current (i32.const 0))\n"
            ));
        }
    }

    fn emit_gc_root_param_initializer(&self, wat: &mut String, frame: &LocalFrame, indent: usize) {
        let pad = " ".repeat(indent);
        for (local, slot) in frame.gc_root_slots() {
            let offset = slot * std::mem::size_of::<u32>();
            wat.push_str(&format!(
                "{pad}(i32.store (i32.add (global.get $gc_root_base) (i32.const {offset})) (local.get {local}))\n",
            ));
        }
        if frame.uses_activation_roots() {
            for local in 0..frame.total_local_count() {
                let offset =
                    Layout::GC_CALL_FRAME_HEADER_SIZE as usize + local * std::mem::size_of::<u32>();
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const {offset})) (local.get {local}))\n",
                ));
            }
        }
    }

    pub(super) fn emit_gc_root_mirror(
        &self,
        wat: &mut String,
        pad: &str,
        local_id: LocalId,
        frame: &LocalFrame,
    ) {
        if frame.uses_activation_roots() {
            self.emit_gc_activation_root_mirror_slot(wat, pad, local_id.0, frame);
            return;
        }
        let Some(slot) = frame.gc_root_slot(local_id) else {
            return;
        };
        self.emit_gc_root_mirror_slot(wat, pad, local_id.0, slot);
    }

    pub(super) fn emit_gc_root_mirror_index(
        &self,
        wat: &mut String,
        pad: &str,
        local_index: usize,
        frame: &LocalFrame,
    ) {
        if frame.uses_activation_roots() {
            self.emit_gc_activation_root_mirror_slot(wat, pad, local_index, frame);
            return;
        }
        let Some(slot) = frame.gc_root_slot_for_index(local_index) else {
            return;
        };
        self.emit_gc_root_mirror_slot(wat, pad, local_index, slot);
    }

    pub(super) fn emit_gc_backend_temp_roots_clear(
        &self,
        wat: &mut String,
        pad: &str,
        frame: &LocalFrame,
    ) {
        for local_index in frame.backend_base..frame.total_local_count() {
            wat.push_str(&format!(
                "{pad}(local.set {local_index} (i32.const {}))\n",
                ValueTag::UNDEFINED,
            ));
            self.emit_gc_root_mirror_index(wat, pad, local_index, frame);
        }
    }

    fn emit_gc_root_mirror_slot(
        &self,
        wat: &mut String,
        pad: &str,
        local_index: usize,
        slot: usize,
    ) {
        let offset = slot * std::mem::size_of::<u32>();
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (global.get $gc_root_base) (i32.const {offset})) (local.get {}))\n",
            local_index,
        ));
    }

    fn emit_gc_activation_root_mirror_slot(
        &self,
        wat: &mut String,
        pad: &str,
        local_index: usize,
        frame: &LocalFrame,
    ) {
        if local_index >= frame.total_local_count() {
            return;
        }
        let offset =
            Layout::GC_CALL_FRAME_HEADER_SIZE as usize + local_index * std::mem::size_of::<u32>();
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const {offset})) (local.get {}))\n",
            local_index,
        ));
    }

    fn emit_gc_activation_frame_push(&self, wat: &mut String, frame: &LocalFrame, indent: usize) {
        if !frame.uses_activation_roots() {
            return;
        }
        let pad = " ".repeat(indent);
        let frame_bytes = Layout::GC_CALL_FRAME_HEADER_SIZE as usize
            + frame.total_local_count() * std::mem::size_of::<u32>();
        wat.push_str(&format!(
            "{pad}(if (i32.gt_u (i32.add (global.get $gc_call_frame_top) (i32.const {frame_bytes})) (global.get $gc_call_frame_limit))\n",
        ));
        wat.push_str(&format!("{pad}  (then (unreachable)))\n"));
        wat.push_str(&format!(
            "{pad}(i32.store (global.get $gc_call_frame_top) (global.get $gc_call_frame_current))\n",
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (global.get $gc_call_frame_top) (i32.const 4)) (i32.const {}))\n",
            frame.total_local_count(),
        ));
        wat.push_str(&format!(
            "{pad}(global.set $gc_call_frame_current (global.get $gc_call_frame_top))\n",
        ));
        wat.push_str(&format!(
            "{pad}(global.set $gc_call_frame_top (i32.add (global.get $gc_call_frame_top) (i32.const {frame_bytes})))\n",
        ));
    }

    pub(super) fn emit_gc_activation_frame_pop(
        &self,
        wat: &mut String,
        frame: &LocalFrame,
        indent: usize,
    ) {
        if !frame.uses_activation_roots() {
            return;
        }
        let pad = " ".repeat(indent);
        wat.push_str(&format!(
            "{pad}(global.set $gc_call_frame_top (global.get $gc_call_frame_current))\n",
        ));
        wat.push_str(&format!(
            "{pad}(global.set $gc_call_frame_current (i32.load (global.get $gc_call_frame_current)))\n",
        ));
    }

    fn gc_root_slot_count(&self) -> usize {
        self.program.top_level_locals.len()
            + if self.module_runtime_enabled() { 1 } else { 0 }
            + LocalFrame::new(0, None).backend_local_count()
    }

    fn module_runtime_enabled(&self) -> bool {
        self.link_plan
            .required_globals()
            .contains(&RuntimeGlobal::ModuleCache)
    }

    fn gc_root_table_enabled(&self) -> bool {
        self.link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::AllocHeap)
    }

    fn gc_call_frame_roots_enabled(&self) -> bool {
        self.gc_root_table_enabled() && !self.program.functions.is_empty()
    }
}

pub(super) fn function_symbol(id: FuncId) -> String {
    format!("func_{}", id.0)
}

pub(super) fn module_init_symbol(module_id: usize) -> String {
    format!("module_init_{module_id}")
}

pub(super) fn class_prototype_global(id: FuncId) -> String {
    format!("class_proto_{}", id.0)
}

pub(super) fn builtin_error_prototype_global(constructor: BuiltinErrorConstructor) -> &'static str {
    match constructor {
        BuiltinErrorConstructor::Error => "error_proto_error",
        BuiltinErrorConstructor::RangeError => "error_proto_range_error",
        BuiltinErrorConstructor::TypeError => "error_proto_type_error",
        BuiltinErrorConstructor::ReferenceError => "error_proto_reference_error",
        BuiltinErrorConstructor::SyntaxError => "error_proto_syntax_error",
    }
}

pub(super) fn builtin_error_stack_prefix(constructor: BuiltinErrorConstructor) -> &'static str {
    match constructor {
        BuiltinErrorConstructor::Error => "Error: ",
        BuiltinErrorConstructor::RangeError => "RangeError: ",
        BuiltinErrorConstructor::TypeError => "TypeError: ",
        BuiltinErrorConstructor::ReferenceError => "ReferenceError: ",
        BuiltinErrorConstructor::SyntaxError => "SyntaxError: ",
    }
}

fn add_builtin_error_prototype_ref(
    constructor: BuiltinErrorConstructor,
    prototypes: &mut BTreeSet<BuiltinErrorConstructor>,
) {
    let mut current = Some(constructor);
    while let Some(constructor) = current {
        prototypes.insert(constructor);
        current = constructor.parent();
    }
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
