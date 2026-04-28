use std::collections::{BTreeMap, BTreeSet};

use ts2wasm_ir::lowered::{
    FunctionCallKind, LoweredBinaryOp, LoweredExpr, LoweredLogicalAssignOp, LoweredProgram,
    LoweredStmt, LoweredUnaryOp,
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
        if program
            .functions
            .iter()
            .any(|function| function.rest_param_index.is_some())
        {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
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
                    for (cond, case_body) in cases {
                        if let Some(cond_expr) = cond {
                            self.collect_required_runtime_expr(cond_expr);
                            self.add_required_runtime(RuntimeFn::StrictEqual);
                        }
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
                LoweredStmt::Labeled { body, .. } => {
                    self.collect_required_runtime_stmts(std::slice::from_ref(body.as_ref()));
                }
                LoweredStmt::Break { .. } | LoweredStmt::Continue { .. } => {}
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
            LoweredExpr::Assign { expr, .. } => {
                self.collect_required_runtime_expr(expr);
            }
            LoweredExpr::LogicalAssign { op, expr, .. } => {
                self.collect_required_runtime_expr(expr);
                if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                }
            }
            LoweredExpr::LogicalPropertyAssign { op, expr, .. } => {
                self.add_required_runtime(RuntimeFn::PropertyGet);
                self.add_required_runtime(RuntimeFn::PropertySet);
                self.collect_required_runtime_expr(expr);
                if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                    self.add_required_runtime(RuntimeFn::TruthyBool);
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
                    LoweredBinaryOp::Multiply => {
                        if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                            && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        {
                            self.add_required_runtime(RuntimeFn::MulFast);
                        } else {
                            self.add_required_runtime(RuntimeFn::Mul);
                        }
                    }
                    LoweredBinaryOp::Divide => {
                        if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                            && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        {
                            self.add_required_runtime(RuntimeFn::DivFast);
                        } else {
                            self.add_required_runtime(RuntimeFn::Div);
                        }
                    }
                    LoweredBinaryOp::Modulo => {
                        if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                            && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        {
                            self.add_required_runtime(RuntimeFn::ModFast);
                        } else {
                            self.add_required_runtime(RuntimeFn::Mod);
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
                    LoweredBinaryOp::LessEqual => {
                        if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                            && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        {
                            self.add_required_runtime(RuntimeFn::LessEqualFast);
                        } else {
                            self.add_required_runtime(RuntimeFn::LessEqual);
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
                    LoweredBinaryOp::GreaterEqual => {
                        if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                            && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        {
                            self.add_required_runtime(RuntimeFn::GreaterEqualFast);
                        } else {
                            self.add_required_runtime(RuntimeFn::GreaterEqual);
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
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
            } => {
                self.add_required_runtime(RuntimeFn::PropertySet);
                self.add_required_runtime(RuntimeFn::ValueToStringInto);
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(index);
                self.collect_required_runtime_expr(value);
            }
            LoweredExpr::New { args, .. } => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
                for arg in args {
                    self.collect_required_runtime_expr(arg);
                }
            }
            LoweredExpr::ClassPrototype(_) => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
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
        }
    }
}
