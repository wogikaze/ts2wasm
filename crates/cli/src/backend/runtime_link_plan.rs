use std::collections::{BTreeMap, BTreeSet};

use ts2wasm_ir::lowered::{
    FunctionCallKind, LoweredBinaryOp, LoweredExpr, LoweredProgram, LoweredStmt, LoweredUnaryOp,
};

use super::runtime_fn::{Capability, HostAbi, HostImport, RuntimeFn, RuntimeGlobal};

#[derive(Debug, Clone)]
pub(crate) struct RuntimeLinkPlan {
    required_runtime: BTreeSet<RuntimeFn>,
    required_globals: BTreeSet<RuntimeGlobal>,
    required_imports: BTreeSet<HostImport>,
    required_capabilities: BTreeSet<Capability>,
    required_runtime_strings: BTreeSet<&'static str>,
    manifest_target: &'static str,
    capability_reasons: BTreeMap<String, Vec<String>>,
}

impl Default for RuntimeLinkPlan {
    fn default() -> Self {
        Self {
            required_runtime: BTreeSet::new(),
            required_globals: BTreeSet::new(),
            required_imports: BTreeSet::new(),
            required_capabilities: BTreeSet::new(),
            required_runtime_strings: BTreeSet::new(),
            manifest_target: "wasm32-wasi-p1",
            capability_reasons: BTreeMap::new(),
        }
    }
}

impl RuntimeLinkPlan {
    /// Return the manifest target string.
    /// Kept for future manifest emission capabilities.
    #[allow(dead_code)]
    pub(crate) const fn manifest_target(&self) -> &'static str {
        self.manifest_target
    }

    pub(crate) fn from_program(program: &LoweredProgram) -> Self {
        let mut plan = Self::default();
        plan.collect_required_runtime_stmts(&program.top_level_statements);
        for function in &program.functions {
            plan.collect_required_runtime_stmts(&function.body);
        }
        // Module cache initialization requires AllocHeap.
        if !program.modules.is_empty() {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
        }
        plan.populate_derived_sets();
        plan
    }

    pub(crate) fn required_runtime_functions(&self) -> &BTreeSet<RuntimeFn> {
        &self.required_runtime
    }

    pub(crate) fn required_imports(&self) -> &BTreeSet<HostImport> {
        &self.required_imports
    }

    pub(crate) fn required_globals(&self) -> &BTreeSet<RuntimeGlobal> {
        &self.required_globals
    }

    pub(crate) fn required_capabilities(&self) -> &BTreeSet<Capability> {
        &self.required_capabilities
    }

    pub(crate) fn required_runtime_strings(&self) -> &BTreeSet<&'static str> {
        &self.required_runtime_strings
    }

    pub(crate) fn capability_reasons(&self) -> &BTreeMap<String, Vec<String>> {
        &self.capability_reasons
    }

    pub(crate) fn add_capability_reason(&mut self, capability_key: String, reason: String) {
        self.capability_reasons
            .entry(capability_key)
            .or_default()
            .push(reason);
    }

    #[cfg(test)]
    pub(crate) fn from_required_runtime_for_tests(required: &[RuntimeFn]) -> Self {
        let mut plan = Self::default();
        for runtime_fn in required {
            plan.add_required_runtime(*runtime_fn);
        }
        plan.populate_derived_sets();
        plan
    }

    fn add_required_runtime(&mut self, runtime_fn: RuntimeFn) {
        if !self.required_runtime.insert(runtime_fn) {
            return;
        }
        for dep in runtime_fn.spec().deps {
            self.add_required_runtime(*dep);
        }
    }

    fn populate_derived_sets(&mut self) {
        // Collect capability reasons first to avoid borrow conflicts
        let mut capability_reasons_to_add: Vec<(String, String)> = Vec::new();

        for runtime_fn in &self.required_runtime {
            for global in runtime_fn.globals() {
                self.required_globals.insert(*global);
            }
            for import in runtime_fn.spec().imports {
                self.required_imports.insert(*import);
            }
            for capability in runtime_fn.spec().capability {
                self.required_capabilities.insert(*capability);
                // Collect capability reason based on the runtime function
                let reason = format!(
                    "required by runtime function: {}",
                    runtime_fn.manifest_name()
                );
                capability_reasons_to_add.push((capability.manifest_name().to_owned(), reason));
            }
            for value in runtime_fn.spec().runtime_strings {
                self.required_runtime_strings.insert(*value);
            }
        }

        // Add collected capability reasons
        for (key, reason) in capability_reasons_to_add {
            self.add_capability_reason(key, reason);
        }

        self.manifest_target = if self
            .required_imports
            .iter()
            .any(|import| matches!(import.spec().abi, HostAbi::NodeShim))
        {
            "wasm32-wasi-p1+node-shim"
        } else {
            "wasm32-wasi-p1"
        };
    }

    fn collect_required_runtime_stmts(&mut self, statements: &[LoweredStmt]) {
        for statement in statements {
            match statement {
                LoweredStmt::Let(_, expr)
                | LoweredStmt::Assign(_, expr)
                | LoweredStmt::Expr(expr)
                | LoweredStmt::Return(expr)
                | LoweredStmt::Throw(expr) => self.collect_required_runtime_expr(expr),
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
                LoweredStmt::TryCatch {
                    try_body,
                    catch_var: _,
                    catch_body,
                    finally_body,
                } => {
                    self.collect_required_runtime_stmts(try_body);
                    if let Some(body) = catch_body {
                        self.collect_required_runtime_stmts(body);
                    }
                    if let Some(body) = finally_body {
                        self.collect_required_runtime_stmts(body);
                    }
                }
                LoweredStmt::Switch { expr, cases } => {
                    self.collect_required_runtime_expr(expr);
                    for (_, case_body) in cases {
                        self.collect_required_runtime_stmts(case_body);
                    }
                }
                LoweredStmt::DoWhile { body, condition } => {
                    self.collect_required_runtime_expr(condition);
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                    self.collect_required_runtime_stmts(body);
                }
                LoweredStmt::For {
                    init,
                    condition,
                    update,
                    body,
                } => {
                    if let Some(stmt) = init {
                        self.collect_required_runtime_stmts(&[stmt.as_ref().clone()]);
                    }
                    if let Some(expr) = condition {
                        self.collect_required_runtime_expr(&expr);
                        self.add_required_runtime(RuntimeFn::TruthyBool);
                    }
                    if let Some(expr) = update {
                        self.collect_required_runtime_expr(&expr);
                    }
                    self.collect_required_runtime_stmts(body);
                }
                LoweredStmt::ForIn {
                    var: _, iter, body, ..
                } => {
                    self.collect_required_runtime_expr(iter);
                    self.add_required_runtime(RuntimeFn::ObjectKeys);
                    self.add_required_runtime(RuntimeFn::GetLength);
                    self.add_required_runtime(RuntimeFn::Less);
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                    self.add_required_runtime(RuntimeFn::ArrayGet);
                    self.add_required_runtime(RuntimeFn::Add);
                    self.collect_required_runtime_stmts(body);
                }
                LoweredStmt::ForOf {
                    var: _, iter, body, ..
                } => {
                    self.collect_required_runtime_expr(iter);
                    self.add_required_runtime(RuntimeFn::GetLength);
                    self.add_required_runtime(RuntimeFn::Less);
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                    self.add_required_runtime(RuntimeFn::ArrayGet);
                    self.add_required_runtime(RuntimeFn::Add);
                    self.collect_required_runtime_stmts(body);
                }
                LoweredStmt::Break | LoweredStmt::Continue => {}
                LoweredStmt::Export { expr, .. } => {
                    self.collect_required_runtime_expr(expr);
                    self.add_required_runtime(RuntimeFn::ModuleExportsSet);
                }
                LoweredStmt::ModuleExportsAssign { expr } => {
                    self.collect_required_runtime_expr(expr);
                    self.add_required_runtime(RuntimeFn::ModuleExportsAssign);
                }
                LoweredStmt::ClassDecl { .. } => {}
            }
        }
    }

    fn collect_required_runtime_expr(&mut self, expr: &LoweredExpr) {
        match expr {
            LoweredExpr::Unary { op, expr } => {
                self.collect_required_runtime_expr(expr);
                match op {
                    LoweredUnaryOp::Not => self.add_required_runtime(RuntimeFn::Not),
                    LoweredUnaryOp::Negate => self.add_required_runtime(RuntimeFn::Negate),
                    LoweredUnaryOp::TypeOf => self.add_required_runtime(RuntimeFn::TypeOf),
                    LoweredUnaryOp::Delete => {
                        // Delete is handled specially, no runtime function needed
                    }
                }
            }
            LoweredExpr::Binary { left, op, right } => {
                self.collect_required_runtime_expr(left);
                self.collect_required_runtime_expr(right);
                match op {
                    LoweredBinaryOp::Add => {
                        if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                            && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        {
                            self.add_required_runtime(RuntimeFn::AddFast);
                        } else {
                            self.add_required_runtime(RuntimeFn::Add);
                        }
                    }
                    LoweredBinaryOp::Subtract => {
                        if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                            && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        {
                            self.add_required_runtime(RuntimeFn::SubFast);
                        } else {
                            self.add_required_runtime(RuntimeFn::Sub);
                        }
                    }
                    LoweredBinaryOp::Less => {
                        if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                            && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        {
                            self.add_required_runtime(RuntimeFn::LessFast);
                        } else {
                            self.add_required_runtime(RuntimeFn::Less);
                        }
                    }
                    LoweredBinaryOp::Greater => {
                        if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                            && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        {
                            self.add_required_runtime(RuntimeFn::GreaterFast);
                        } else {
                            self.add_required_runtime(RuntimeFn::Greater);
                        }
                    }
                    LoweredBinaryOp::StrictEqual => {
                        self.add_required_runtime(RuntimeFn::StrictEqual)
                    }
                    LoweredBinaryOp::EqualEqual => self.add_required_runtime(RuntimeFn::EqualEqual),
                    LoweredBinaryOp::BangEqual => self.add_required_runtime(RuntimeFn::BangEqual),
                    LoweredBinaryOp::StrictNotEqual => {
                        self.add_required_runtime(RuntimeFn::StrictNotEqual)
                    }
                    LoweredBinaryOp::And => self.add_required_runtime(RuntimeFn::And),
                    LoweredBinaryOp::Or => self.add_required_runtime(RuntimeFn::Or),
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
            | LoweredExpr::This
            | LoweredExpr::Local(_)
            | LoweredExpr::ArrowFn { .. } => {}
            LoweredExpr::ArrayNew { elements, .. } => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
                for elem in elements {
                    self.collect_required_runtime_expr(elem);
                }
            }
            LoweredExpr::ArrayGet { arr, index } => {
                self.add_required_runtime(RuntimeFn::ArrayGet);
                self.collect_required_runtime_expr(arr);
                self.collect_required_runtime_expr(index);
            }
            LoweredExpr::Index { object, index } => {
                self.add_required_runtime(RuntimeFn::Index);
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(index);
            }
            LoweredExpr::GetLength(inner) => {
                self.add_required_runtime(RuntimeFn::GetLength);
                self.collect_required_runtime_expr(inner);
            }
            LoweredExpr::ObjectNew { props, .. } => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
                for (_, val) in props {
                    self.collect_required_runtime_expr(val);
                }
            }
            LoweredExpr::PropertyGet { obj, .. } => {
                self.add_required_runtime(RuntimeFn::PropertyGet);
                self.collect_required_runtime_expr(obj);
            }
            LoweredExpr::PropertyGetDynamic { obj, key } => {
                self.add_required_runtime(RuntimeFn::PropertyGet);
                self.collect_required_runtime_expr(obj);
                self.collect_required_runtime_expr(key);
            }
            LoweredExpr::PropertySet { object, value, .. } => {
                self.add_required_runtime(RuntimeFn::PropertySet);
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(value);
            }
            LoweredExpr::PropertySetDynamic { object, key, value } => {
                self.add_required_runtime(RuntimeFn::PropertySet);
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(key);
                self.collect_required_runtime_expr(value);
            }
            LoweredExpr::New { args, .. } => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
                for arg in args {
                    self.collect_required_runtime_expr(arg);
                }
            }
            LoweredExpr::MethodCall { .. } => {}
            LoweredExpr::ModuleLoad { .. } => {
                self.add_required_runtime(RuntimeFn::ModuleRequire);
            }
            LoweredExpr::RuntimeCall { runtime_fn, args } => {
                if let Some(runtime_fn_enum) = super::runtime_fn::runtime_fn_from_name(runtime_fn) {
                    self.add_required_runtime(runtime_fn_enum);
                }
                for arg in args {
                    self.collect_required_runtime_expr(arg);
                }
            }
            LoweredExpr::PropertyDelete { object, key: _ } => {
                self.collect_required_runtime_expr(object);
                self.add_required_runtime(RuntimeFn::PropertyDelete);
            }
            LoweredExpr::PropertyDeleteDynamic { object, key } => {
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(key);
                self.add_required_runtime(RuntimeFn::PropertyDelete);
            }
            LoweredExpr::PropertyIn { obj, key: _ } => {
                self.collect_required_runtime_expr(obj);
                self.add_required_runtime(RuntimeFn::PropertyHas);
            }
            LoweredExpr::PropertyInDynamic { obj, key } => {
                self.collect_required_runtime_expr(obj);
                self.collect_required_runtime_expr(key);
                self.add_required_runtime(RuntimeFn::PropertyHas);
            }
            LoweredExpr::ArrayNew { elements, .. } => {
                for elem in elements {
                    self.collect_required_runtime_expr(elem);
                }
                self.add_required_runtime(RuntimeFn::AllocHeap);
            }
            LoweredExpr::ArrowFn { .. } => {
                // Arrow functions not yet fully implemented (issue #36)
                // No runtime functions needed for placeholder emission
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::backend::emit_wat;
    use crate::backend::runtime_fn::{Capability, HostImport, RuntimeFn, RuntimeGlobal};
    use ts2wasm_ir::lowered::lower_program;

    use super::RuntimeLinkPlan;

    fn lowered(source: &str) -> ts2wasm_ir::lowered::LoweredProgram {
        let program = crate::parse_program(source).expect("parse failed");
        let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&program)
            .expect("builtin resolution failed");
        lower_program(&resolved).expect("lowering failed")
    }

    #[test]
    fn no_console_log_has_no_fd_write_import() {
        let program = lowered("let x = 1 + 2;");
        let plan = RuntimeLinkPlan::from_program(&program);
        let wat = emit_wat(&program).expect("emit failed");

        assert!(!wat.contains("\"fd_write\""));
        assert!(!plan.required_imports().contains(&HostImport::FdWrite));
        assert!(
            !plan
                .required_capabilities()
                .contains(&Capability::StdoutWrite)
        );
    }

    #[test]
    fn console_log_requires_fd_write_and_runtime_strings() {
        let program = lowered("console.log(1);");
        let plan = RuntimeLinkPlan::from_program(&program);
        let wat = emit_wat(&program).expect("emit failed");

        assert!(wat.contains("\"fd_write\""));
        assert!(plan.required_imports().contains(&HostImport::FdWrite));
        assert!(
            plan.required_capabilities()
                .contains(&Capability::StdoutWrite)
        );
        assert!(
            plan.required_runtime_strings()
                .contains(&ts2wasm_runtime_abi::RuntimeString::NEWLINE)
        );
        assert!(
            plan.required_runtime_strings()
                .contains(&ts2wasm_runtime_abi::RuntimeString::UNDEFINED)
        );
    }

    #[test]
    fn runtime_linker_collects_expected_dependencies() {
        let strict_program = lowered("let x = 1 === 2;");
        let strict = RuntimeLinkPlan::from_program(&strict_program);
        let strict_expected: BTreeSet<_> = [
            RuntimeFn::StrictEqual,
            RuntimeFn::IsString,
            RuntimeFn::StringEqual,
        ]
        .into_iter()
        .collect();
        assert!(
            strict_expected
                .iter()
                .all(|runtime_fn| strict.required_runtime_functions().contains(runtime_fn))
        );

        let add_program = lowered("let y = \"x\" + 12;");
        let add = RuntimeLinkPlan::from_program(&add_program);
        let add_expected: BTreeSet<_> = [
            RuntimeFn::Add,
            RuntimeFn::IsString,
            RuntimeFn::Concat,
            RuntimeFn::ValueToStringInto,
            RuntimeFn::Copy,
        ]
        .into_iter()
        .collect();
        assert!(
            add_expected
                .iter()
                .all(|runtime_fn| add.required_runtime_functions().contains(runtime_fn))
        );

        let cond_program = lowered("if (1) { let x = 1; }");
        let cond = RuntimeLinkPlan::from_program(&cond_program);
        assert!(
            cond.required_runtime_functions()
                .contains(&RuntimeFn::TruthyBool)
        );
    }

    #[test]
    fn runtime_strings_are_trimmed_when_runtime_not_needed() {
        let program = lowered("let x = 1;");
        let plan = RuntimeLinkPlan::from_program(&program);
        assert!(
            !plan
                .required_runtime_strings()
                .contains(&ts2wasm_runtime_abi::RuntimeString::UNDEFINED)
        );
        assert!(
            !plan
                .required_runtime_strings()
                .contains(&ts2wasm_runtime_abi::RuntimeString::NEWLINE)
        );
    }

    #[test]
    fn m5_runtime_linker_collects_array_object_and_length_helpers() {
        let array_get_program = lowered("let x = [1][0];");
        let array_get = RuntimeLinkPlan::from_program(&array_get_program);
        let array_expected: BTreeSet<_> = [RuntimeFn::AllocHeap, RuntimeFn::ArrayGet]
            .into_iter()
            .collect();
        assert!(
            array_expected
                .iter()
                .all(|runtime_fn| array_get.required_runtime_functions().contains(runtime_fn))
        );
        assert!(!array_get.required_imports().contains(&HostImport::FdWrite));

        let length_program = lowered("let a = [1, 2]; let b = a.length;");
        let length = RuntimeLinkPlan::from_program(&length_program);
        let length_expected: BTreeSet<_> = [RuntimeFn::AllocHeap, RuntimeFn::GetLength]
            .into_iter()
            .collect();
        assert!(
            length_expected
                .iter()
                .all(|runtime_fn| length.required_runtime_functions().contains(runtime_fn))
        );
        assert!(!length.required_imports().contains(&HostImport::FdWrite));

        let object_program = lowered("let o = { a: 1 }; let v = o.a;");
        let object = RuntimeLinkPlan::from_program(&object_program);
        let object_expected: BTreeSet<_> = [
            RuntimeFn::AllocHeap,
            RuntimeFn::PropertyGet,
            RuntimeFn::MemEqual,
        ]
        .into_iter()
        .collect();
        assert!(
            object_expected
                .iter()
                .all(|runtime_fn| object.required_runtime_functions().contains(runtime_fn))
        );
        assert!(!object.required_imports().contains(&HostImport::FdWrite));
    }

    #[test]
    fn m6_stdin_skeleton_runtime_derives_fd_read_and_stdin_capability() {
        let plan = RuntimeLinkPlan::from_required_runtime_for_tests(&[RuntimeFn::ReadStdinBytes]);
        assert!(plan.required_imports().contains(&HostImport::FdRead));
        assert!(
            plan.required_capabilities()
                .contains(&Capability::StdinRead)
        );
    }

    #[test]
    fn m6_idiom_lowering_populates_fd_read_link_plan_without_execution() {
        let program = lowered("let s = require(\"fs\").readFileSync(0, \"utf8\");");
        let plan = RuntimeLinkPlan::from_program(&program);
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::ReadStdinBytes)
        );
        assert!(plan.required_imports().contains(&HostImport::FdRead));
        assert!(
            plan.required_capabilities()
                .contains(&Capability::StdinRead)
        );
    }

    #[test]
    fn module_runtime_derives_module_globals() {
        let program = lowered("let m = require(\"./mod\");");
        let plan = RuntimeLinkPlan::from_program(&program);
        assert!(
            plan.required_globals()
                .contains(&RuntimeGlobal::ModuleCache)
        );
        assert!(
            plan.required_globals()
                .contains(&RuntimeGlobal::CurrentModuleId)
        );
    }
}
