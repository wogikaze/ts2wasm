use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use ts2wasm_ir::lowered::{
    ClosureRepresentation, FunctionCallKind, LoweredBinaryOp, LoweredExpr, LoweredLogicalAssignOp,
    LoweredProgram, LoweredStmt, LoweredUnaryOp,
};
use ts2wasm_ir::RuntimeIntrinsic;
use ts2wasm_runtime_abi::ValueTag;

use super::runtime_fn::{
    Capability, GLOBALS_EXCEPTION_RUNTIME, HostAbi, HostImport, RuntimeFn, RuntimeGlobal,
};

#[derive(Debug, Clone)]
pub(crate) struct RuntimeLinkPlan {
    required_runtime: BTreeSet<RuntimeFn>,
    required_globals: BTreeSet<RuntimeGlobal>,
    required_imports: BTreeSet<HostImport>,
    required_capabilities: BTreeSet<Capability>,
    required_runtime_strings: BTreeSet<&'static str>,
    /// Maps each runtime string to the RuntimeFn variants that declare it.
    /// Preserves origin information for auditing and conditional interning.
    string_origins: BTreeMap<&'static str, Vec<RuntimeFn>>,
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
            string_origins: BTreeMap::new(),
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
        // emit_top_level_statements unconditionally emits a $exception_pending
        // guard after each top-level statement. Declare the exception globals
        // whenever there are top-level statements so that WAT never references
        // an undeclared global (e.g. after ClassDecl, If, or similar statements
        // that do not themselves select exception globals).
        if !program.top_level_statements.is_empty() {
            plan.add_required_globals(GLOBALS_EXCEPTION_RUNTIME);
        }
        // WASI proc_exit is always required for program termination
        plan.required_imports.insert(HostImport::WasiProcExit);
        for function in &program.functions {
            plan.collect_required_runtime_stmts(&function.body);
        }
        for module in &program.modules {
            plan.collect_required_runtime_stmts(&module.statements);
        }
        if program
            .functions
            .iter()
            .any(|function| function.rest_param_index.is_some())
        {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
        }
        // Async functions need AllocHeap for their state-machine frame.
        if program.functions.iter().any(|function| function.is_async) {
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

    pub(crate) fn string_origins(&self) -> &BTreeMap<&'static str, Vec<RuntimeFn>> {
        &self.string_origins
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

    fn add_required_globals(&mut self, globals: &'static [RuntimeGlobal]) {
        for global in globals {
            self.required_globals.insert(*global);
        }
    }

    fn populate_derived_sets(&mut self) {
        // Recursively add transitive deps from RuntimeSpec (e.g. ArrayEvery needs TruthyBool)
        let mut changed = true;
        while changed {
            changed = false;
            let deps: Vec<RuntimeFn> = self
                .required_runtime
                .iter()
                .flat_map(|rf| rf.spec().deps.iter().copied())
                .collect();
            for dep in deps {
                if self.required_runtime.insert(dep) {
                    changed = true;
                }
            }
        }

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
                match (*capability, *runtime_fn) {
                    (Capability::WasiClockRealtime, RuntimeFn::DateNow) => {
                        capability_reasons_to_add
                            .push((capability.manifest_name().to_owned(), "Date.now".to_owned()));
                    }
                    (Capability::WasiClockRealtime, RuntimeFn::DateNewLive) => {
                        capability_reasons_to_add.push((
                            capability.manifest_name().to_owned(),
                            "new Date()".to_owned(),
                        ));
                    }
                    (Capability::WasiClockRealtime, _) => {}
                    _ => {
                        let reason = format!(
                            "required by runtime function: {}",
                            runtime_fn.manifest_name()
                        );
                        capability_reasons_to_add
                            .push((capability.manifest_name().to_owned(), reason));
                    }
                }
            }
            for value in runtime_fn.spec().runtime_strings {
                self.required_runtime_strings.insert(*value);
                self.string_origins
                    .entry(*value)
                    .or_default()
                    .push(*runtime_fn);
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
                LoweredStmt::Block(statements, _) => {
                    self.collect_required_runtime_stmts(statements);
                }
                LoweredStmt::Let(_, expr, _)
                | LoweredStmt::Assign(_, expr, _)
                | LoweredStmt::Expr(expr, _)
                | LoweredStmt::Return(expr, _)
                | LoweredStmt::Throw(expr, _) => {
                    self.add_required_globals(GLOBALS_EXCEPTION_RUNTIME);
                    self.collect_required_runtime_expr(expr);
                }
                LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    self.collect_required_runtime_expr(condition);
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                    self.collect_required_runtime_stmts(then_body);
                    self.collect_required_runtime_stmts(else_body);
                }
                LoweredStmt::While {
                    condition, body, ..
                } => {
                    self.collect_required_runtime_expr(condition);
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                    self.collect_required_runtime_stmts(body);
                }
                LoweredStmt::TryCatch {
                    try_body,
                    catch_var: _,
                    catch_body,
                    finally_body,
                    ..
                } => {
                    self.add_required_globals(GLOBALS_EXCEPTION_RUNTIME);
                    self.collect_required_runtime_stmts(try_body);
                    if let Some(body) = catch_body {
                        self.collect_required_runtime_stmts(body);
                    }
                    if let Some(body) = finally_body {
                        self.collect_required_runtime_stmts(body);
                    }
                }
                LoweredStmt::Switch { expr, cases, .. } => {
                    self.collect_required_runtime_expr(expr);
                    for (cond, case_body) in cases {
                        if let Some(cond_expr) = cond {
                            self.collect_required_runtime_expr(cond_expr);
                            self.add_required_runtime(RuntimeFn::StrictEqual);
                        }
                        self.collect_required_runtime_stmts(case_body);
                    }
                }
                LoweredStmt::DoWhile {
                    body, condition, ..
                } => {
                    self.collect_required_runtime_expr(condition);
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                    self.collect_required_runtime_stmts(body);
                }
                LoweredStmt::For {
                    init,
                    condition,
                    update,
                    body,
                    ..
                } => {
                    if let Some(stmt) = init {
                        self.collect_required_runtime_stmts(&[stmt.as_ref().clone()]);
                    }
                    if let Some(expr) = condition {
                        self.collect_required_runtime_expr(expr);
                        self.add_required_runtime(RuntimeFn::TruthyBool);
                    }
                    if let Some(expr) = update {
                        self.collect_required_runtime_expr(expr);
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
                LoweredStmt::ModuleExportsAssign { expr, .. } => {
                    self.collect_required_runtime_expr(expr);
                    self.add_required_runtime(RuntimeFn::ModuleExportsAssign);
                }
                LoweredStmt::ClassDecl { methods, .. } => {
                    self.add_required_runtime(RuntimeFn::AllocHeap);
                    if !methods.is_empty() {
                        self.add_required_runtime(RuntimeFn::PropertySet);
                    }
                }
            }
        }
    }

    fn collect_required_runtime_expr(&mut self, expr: &LoweredExpr) {
        match expr {
            LoweredExpr::Unary { op, expr, .. } => {
                self.collect_required_runtime_expr(expr);
                match op {
                    LoweredUnaryOp::Not => self.add_required_runtime(RuntimeFn::Not),
                    LoweredUnaryOp::Plus => self.add_required_runtime(RuntimeFn::EqualEqual),
                    LoweredUnaryOp::Negate => self.add_required_runtime(RuntimeFn::Negate),
                    LoweredUnaryOp::TypeOf => self.add_required_runtime(RuntimeFn::TypeOf),
                    LoweredUnaryOp::Delete => {
                        // Delete is handled specially, no runtime function needed
                    }
                    LoweredUnaryOp::Void => {
                        // Void evaluates inner expr for side effects, no runtime function needed
                    }
                }
            }
            LoweredExpr::Assign { expr, .. } => {
                self.collect_required_runtime_expr(expr);
            }
            LoweredExpr::EnvCellNew(expr, _) => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
                self.collect_required_runtime_expr(expr);
            }
            LoweredExpr::EnvCellGet(_, _) => {}
            LoweredExpr::EnvCellSet { expr, .. } => {
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
            LoweredExpr::LogicalMemberAssign {
                op, object, expr, ..
            } => {
                self.add_required_runtime(RuntimeFn::PropertyGet);
                self.add_required_runtime(RuntimeFn::PropertySet);
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(expr);
                if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                }
            }
            LoweredExpr::LogicalComputedPropertyAssign { op, key, expr, .. } => {
                self.add_required_runtime(RuntimeFn::PropertyGet);
                self.add_required_runtime(RuntimeFn::PropertySet);
                self.add_required_runtime(RuntimeFn::ValueToStringInto);
                self.collect_required_runtime_expr(key);
                self.collect_required_runtime_expr(expr);
                if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                }
            }
            LoweredExpr::LogicalComputedMemberAssign {
                op,
                object,
                key,
                expr,
                ..
            } => {
                self.add_required_runtime(RuntimeFn::PropertyGet);
                self.add_required_runtime(RuntimeFn::PropertySet);
                self.add_required_runtime(RuntimeFn::ValueToStringInto);
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(key);
                self.collect_required_runtime_expr(expr);
                if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                    self.add_required_runtime(RuntimeFn::TruthyBool);
                }
            }
            LoweredExpr::Binary {
                left, op, right, ..
            } => {
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
                    LoweredBinaryOp::Power => self.add_required_runtime(RuntimeFn::MathPow),
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
                    LoweredBinaryOp::BitwiseAnd => self.add_required_runtime(RuntimeFn::BitwiseAnd),
                    LoweredBinaryOp::BitwiseXor => self.add_required_runtime(RuntimeFn::BitwiseXor),
                    LoweredBinaryOp::BitwiseOr => self.add_required_runtime(RuntimeFn::BitwiseOr),
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
                    LoweredBinaryOp::And | LoweredBinaryOp::Or => {
                        self.add_required_runtime(RuntimeFn::TruthyBool)
                    }
                    LoweredBinaryOp::NullishCoalesce => {}
                }
            }
            LoweredExpr::Call { kind, args, .. } => {
                for arg in args {
                    self.collect_required_runtime_expr(arg);
                }
                if let FunctionCallKind::Builtin(builtin) = kind {
                    self.add_required_runtime(RuntimeFn::from_builtin(*builtin));
                }
            }
            LoweredExpr::Number(value, _) => {
                if !ValueTag::can_encode_number(*value) {
                    self.add_required_runtime(RuntimeFn::NumberFromI32);
                }
            }
            LoweredExpr::String(_, _)
            | LoweredExpr::Bool(_, _)
            | LoweredExpr::Null(..)
            | LoweredExpr::Undefined(..)
            | LoweredExpr::This(..)
            | LoweredExpr::Local(_, _) => {}
            LoweredExpr::ArrowFn { representation, .. } => {
                if matches!(representation, ClosureRepresentation::HeapObject) {
                    self.add_required_runtime(RuntimeFn::AllocHeap);
                }
            }
            LoweredExpr::BigIntLiteral { .. } => {
                self.add_required_runtime(RuntimeFn::MakeBigIntLiteral);
            }
            LoweredExpr::ArrayNew { elements, .. } => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
                for elem in elements {
                    self.collect_required_runtime_expr(elem);
                }
            }
            LoweredExpr::ArrayNewSparse { slots, .. } => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
                for slot in slots {
                    if let ts2wasm_ir::lowered::LoweredArraySlot::Present(elem) = slot {
                        self.collect_required_runtime_expr(elem);
                    }
                }
            }
            LoweredExpr::ArrayGet { arr, index, .. } => {
                self.add_required_runtime(RuntimeFn::ArrayGet);
                self.collect_required_runtime_expr(arr);
                self.collect_required_runtime_expr(index);
            }
            LoweredExpr::Index { object, index, .. } => {
                self.add_required_runtime(RuntimeFn::Index);
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(index);
            }
            LoweredExpr::GetLength(inner, _) => {
                self.add_required_runtime(RuntimeFn::GetLength);
                self.collect_required_runtime_expr(inner);
            }
            LoweredExpr::ObjectNew { props, .. } => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
                for (_, val) in props {
                    self.collect_required_runtime_expr(val);
                }
            }
            LoweredExpr::ErrorNew { message, .. } => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
                self.add_required_runtime(RuntimeFn::Concat);
                self.collect_required_runtime_expr(message);
            }
            LoweredExpr::PropertyGet { obj, .. } => {
                self.add_required_runtime(RuntimeFn::PropertyGet);
                self.collect_required_runtime_expr(obj);
            }
            LoweredExpr::OptionalPropertyGet { obj, .. } => {
                self.add_required_runtime(RuntimeFn::PropertyGet);
                self.collect_required_runtime_expr(obj);
            }
            LoweredExpr::PropertyGetDynamic { obj, key, .. } => {
                self.add_required_runtime(RuntimeFn::PropertyGet);
                self.add_required_runtime(RuntimeFn::ValueToStringInto);
                self.collect_required_runtime_expr(obj);
                self.collect_required_runtime_expr(key);
            }
            LoweredExpr::OptionalIndex { object, index, .. } => {
                self.add_required_runtime(RuntimeFn::Index);
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(index);
            }
            LoweredExpr::OptionalCall { callee, call, .. } => {
                self.collect_required_runtime_expr(callee);
                self.collect_required_runtime_expr(call);
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
                ..
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
            #[allow(unreachable_patterns)]
            LoweredExpr::PromiseGetValue { promise, .. } => {
                self.collect_required_runtime_expr(promise);
                self.add_required_runtime(RuntimeFn::TaskResult);
            }
            LoweredExpr::ClassPrototype(_, _) => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
            }
            LoweredExpr::BuiltinErrorPrototype(_, _) => {
                self.add_required_runtime(RuntimeFn::AllocHeap);
            }
            LoweredExpr::Block { stmts, result, .. } => {
                for stmt in stmts {
                    self.collect_required_runtime_stmts(std::slice::from_ref(stmt));
                }
                self.collect_required_runtime_expr(result);
            }
            LoweredExpr::MethodCall { .. } => {}
            LoweredExpr::ModuleLoad { .. } => {
                self.add_required_runtime(RuntimeFn::ModuleRequire);
            }
            LoweredExpr::RuntimeCall { intrinsic, args, ..
            } => {
                if *intrinsic == RuntimeIntrinsic::ArrayPushMany {
                    self.add_required_runtime(RuntimeFn::ArrayPush);
                    self.add_required_runtime(RuntimeFn::ArrayPushGrow);
                    self.add_required_runtime(RuntimeFn::GetLength);
                }
                if *intrinsic == RuntimeIntrinsic::ArrayPushGrow {
                    self.add_required_runtime(RuntimeFn::ArrayPushGrow);
                }
                if *intrinsic == RuntimeIntrinsic::PrivateFieldGet
                    || *intrinsic == RuntimeIntrinsic::PrivateFieldSet
                    || *intrinsic == RuntimeIntrinsic::PrivateBrandCheck
                {
                    self.add_required_runtime(RuntimeFn::PrivateBrandTypeError);
                }
                if let Some(runtime_fn_enum) = super::runtime_fn::runtime_fn_from_name(intrinsic.name()) {
                    self.add_required_runtime(runtime_fn_enum);
                }
                for arg in args {
                    self.collect_required_runtime_expr(arg);
                }
            }
            LoweredExpr::PropertyDelete { object, key: _, .. } => {
                self.collect_required_runtime_expr(object);
                self.add_required_runtime(RuntimeFn::PropertyDelete);
            }
            LoweredExpr::PropertyDeleteDynamic { object, key, .. } => {
                self.collect_required_runtime_expr(object);
                self.collect_required_runtime_expr(key);
                self.add_required_runtime(RuntimeFn::PropertyDelete);
                self.add_required_runtime(RuntimeFn::ValueToStringInto);
            }
            LoweredExpr::PropertyIn { obj, key: _, .. } => {
                self.collect_required_runtime_expr(obj);
                self.add_required_runtime(RuntimeFn::PropertyHas);
            }
            LoweredExpr::PropertyInDynamic { obj, key, .. } => {
                self.collect_required_runtime_expr(obj);
                self.collect_required_runtime_expr(key);
                self.add_required_runtime(RuntimeFn::PropertyHas);
                self.add_required_runtime(RuntimeFn::ValueToStringInto);
            }
        }
    }
}

/// Public snapshot of a RuntimeLinkPlan for use in fixture-based tests.
/// All fields are sorted for deterministic JSON output.
#[derive(Debug, Clone, Serialize)]
pub struct LinkPlanSnapshot {
    pub runtime_functions: Vec<String>,
    pub globals: Vec<String>,
    pub imports: Vec<String>,
    pub capabilities: Vec<String>,
    pub runtime_strings: Vec<String>,
    pub manifest_target: String,
}

/// Generate a JSON snapshot of the RuntimeLinkPlan for a given lowered program.
/// Used by fixture-based linker structure tests.
pub fn emit_link_plan_snapshot_json(program: &LoweredProgram) -> String {
    let plan = RuntimeLinkPlan::from_program(program);
    let snapshot = LinkPlanSnapshot {
        runtime_functions: plan
            .required_runtime
            .iter()
            .map(|rf| rf.manifest_name().to_owned())
            .collect(),
        globals: plan
            .required_globals
            .iter()
            .map(|g| g.symbol().to_owned())
            .collect(),
        imports: plan
            .required_imports
            .iter()
            .map(|i| i.manifest_name().to_owned())
            .collect(),
        capabilities: plan
            .required_capabilities
            .iter()
            .map(|c| c.manifest_name().to_owned())
            .collect(),
        runtime_strings: plan
            .required_runtime_strings
            .iter()
            .copied()
            .map(|s| s.to_owned())
            .collect(),
        manifest_target: plan.manifest_target.to_owned(),
    };
    serde_json::to_string_pretty(&snapshot).expect("LinkPlanSnapshot must serialize to JSON")
}

#[cfg(test)]
mod tests {
    use ts2wasm_frontend::Span;
    use ts2wasm_ir::builtin::BuiltinId;
    use ts2wasm_ir::lowered::{
        FuncId, FunctionCallKind, LoweredBinaryOp, LoweredExpr, LoweredProgram, LoweredStmt,
        ModuleInfo,
    };

    use super::{HostImport, RuntimeFn, RuntimeGlobal, RuntimeLinkPlan};

    #[test]
    fn empty_module_metadata_does_not_select_es_module_export_helpers() {
        let program = LoweredProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![ModuleInfo {
                id: 1,
                specifier: "./dep".to_owned(),
                statements: vec![],
                locals_count: 0,
            }],
        };

        let plan = RuntimeLinkPlan::from_program(&program);

        assert!(
            !plan
                .required_runtime_functions()
                .contains(&RuntimeFn::ModuleRequire)
        );
        assert!(
            !plan
                .required_runtime_functions()
                .contains(&RuntimeFn::ModuleExportsSet)
        );
        assert!(
            !plan
                .required_runtime_functions()
                .contains(&RuntimeFn::ModuleExportsAssign)
        );
    }

    #[test]
    fn explicit_module_export_statement_selects_es_module_export_helpers() {
        let program = LoweredProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![ModuleInfo {
                id: 1,
                specifier: "./dep".to_owned(),
                statements: vec![LoweredStmt::Export {
                    name: "value".to_owned(),
                    expr: LoweredExpr::Number(1, Span::generated("test")),
                    span: Span::generated("test"),
                }],
                locals_count: 0,
            }],
        };

        let plan = RuntimeLinkPlan::from_program(&program);

        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::ModuleExportsSet)
        );
        assert!(
            !plan
                .required_runtime_functions()
                .contains(&RuntimeFn::ModuleRequire)
        );
    }

    #[test]
    fn bigint_runtime_arithmetic_selects_helper_deps() {
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntAdd,
                        args: vec![
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntUnaryMinus,
                        args: vec![LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(0),
                            Span::generated("test"),
                        )],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntMul,
                        args: vec![
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntPow,
                        args: vec![
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntDiv,
                        args: vec![
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntRem,
                        args: vec![
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntMixedArithmeticTypeError,
                        args: vec![
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                            LoweredExpr::Number(2, Span::generated("test")),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntBitwiseNot,
                        args: vec![LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(0),
                            Span::generated("test"),
                        )],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntBitwiseAnd,
                        args: vec![
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntBitwiseOr,
                        args: vec![
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntBitwiseXor,
                        args: vec![
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntFromValue,
                        args: vec![LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(0),
                            Span::generated("test"),
                        )],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntAsIntN,
                        args: vec![
                            LoweredExpr::Number(8, Span::generated("test")),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntAsUintN,
                        args: vec![
                            LoweredExpr::Number(8, Span::generated("test")),
                            LoweredExpr::Local(
                                ts2wasm_ir::lowered::LocalId(0),
                                Span::generated("test"),
                            ),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
            ],
            top_level_locals: vec![ts2wasm_ir::lowered::LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let plan = RuntimeLinkPlan::from_program(&program);

        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntAdd)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntUnaryMinus)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntMul)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntPow)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntDiv)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntRem)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntDivisionByZeroRangeError)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntMixedArithmeticTypeError)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntBitwiseNot)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntBitwiseAnd)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntBitwiseOr)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntBitwiseXor)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntFromValue)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntAsIntN)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntAsUintN)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::MakeBigIntLiteral)
        );
        assert!(
            plan.required_imports().contains(&HostImport::FdWrite),
            "dynamic BigInt arithmetic error helpers must declare their uncaught diagnostic import"
        );
    }

    #[test]
    fn bigint_runtime_comparison_selects_helper_deps() {
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(0),
                            Span::generated("test"),
                        )),
                        op: LoweredBinaryOp::StrictEqual,
                        right: Box::new(LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(1),
                            Span::generated("test"),
                        )),
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(0),
                            Span::generated("test"),
                        )),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(1),
                            Span::generated("test"),
                        )),
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
            ],
            top_level_locals: vec![
                ts2wasm_ir::lowered::LocalId(0),
                ts2wasm_ir::lowered::LocalId(1),
            ],
            functions: vec![],
            modules: vec![],
        };

        let plan = RuntimeLinkPlan::from_program(&program);

        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::StrictEqual)
        );
        assert!(plan.required_runtime_functions().contains(&RuntimeFn::Less));
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntCompare)
        );
    }

    #[test]
    fn bigint_builtin_string_conversion_selects_helper_deps_without_imports() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::BigIntToString,
                    args: vec![LoweredExpr::BigIntLiteral {
                        decimal: "10".to_owned(),
                        sign: 1,
                        limb_low: 10,
                        limb_high: 0,
                        span: Span::generated("test"),
                    }],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let plan = RuntimeLinkPlan::from_program(&program);

        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::BigIntToString)
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::MakeBigIntLiteral)
        );
        assert!(
            plan.required_imports()
                .iter()
                .all(|i| matches!(i, HostImport::WasiProcExit)),
            "BigInt string conversion must remain standalone"
        );
    }

    #[test]
    fn class_decl_at_top_level_selects_exception_globals() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::ClassDecl {
                name: "Foo".to_owned(),
                extends: None,
                constructor: Some(FuncId(0)),
                methods: vec![("bar".to_owned(), FuncId(1))],
                static_methods: vec![],
                private_fields: vec![],
                span: Span::generated("test"),
            }],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let plan = RuntimeLinkPlan::from_program(&program);

        assert!(
            plan.required_globals()
                .contains(&RuntimeGlobal::ExceptionPending),
            "ClassDecl at top level must select ExceptionPending global"
        );
        assert!(
            plan.required_globals()
                .contains(&RuntimeGlobal::ExceptionHandlerDepth),
            "ClassDecl at top level must select ExceptionHandlerDepth global"
        );
    }

    #[test]
    fn no_console_log_no_log_write_runtime_strings() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Number(42, Span::generated("test")),
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let plan = RuntimeLinkPlan::from_program(&program);

        assert!(
            !plan.required_runtime_functions().contains(&RuntimeFn::Log),
            "no console.log → Log must not be selected"
        );
        assert!(
            !plan
                .required_runtime_functions()
                .contains(&RuntimeFn::Write),
            "no console.log → Write must not be selected"
        );
        assert!(
            !plan
                .required_runtime_functions()
                .contains(&RuntimeFn::ValueToStringInto),
            "no console.log → ValueToStringInto must not be selected"
        );
        // Verify no runtime strings for Log/Write/VTS
        let log_related: Vec<&str> = plan
            .required_runtime_strings()
            .iter()
            .copied()
            .filter(|s| {
                *s == "\n" || *s == "undefined" || *s == "null" || *s == "false" || *s == "true"
            })
            .collect();
        assert!(
            log_related.is_empty(),
            "no console.log → expected zero Log/Write/VTS runtime strings, got: {:?}",
            log_related
        );
        // Verify string_origins are also empty for these strings
        let origin_keys: Vec<&&str> = plan.string_origins().keys().collect();
        let log_origin_keys: Vec<&&str> = origin_keys
            .into_iter()
            .filter(|s| {
                **s == "\n"
                    || **s == "undefined"
                    || **s == "null"
                    || **s == "false"
                    || **s == "true"
            })
            .collect();
        assert!(
            log_origin_keys.is_empty(),
            "no console.log → expected zero Log/Write/VTS origins, got: {:?}",
            log_origin_keys
        );
    }

    #[test]
    fn console_log_selects_log_write_runtime_strings() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args: vec![LoweredExpr::Number(42, Span::generated("test"))],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let plan = RuntimeLinkPlan::from_program(&program);

        assert!(
            plan.required_runtime_functions().contains(&RuntimeFn::Log),
            "console.log → Log must be selected"
        );
        assert!(
            plan.required_runtime_functions()
                .contains(&RuntimeFn::ValueToStringInto),
            "console.log → ValueToStringInto (transitive dep of Log) must be selected"
        );

        // Log declares "\n", ValueToStringInto declares "undefined"/"null"/"false"/"true"
        assert!(
            plan.required_runtime_strings().contains("\n"),
            "console.log → newline runtime string must be interned"
        );
        assert!(
            plan.required_runtime_strings().contains("undefined"),
            "console.log → 'undefined' runtime string must be interned (via ValueToStringInto)"
        );
        assert!(
            plan.required_runtime_strings().contains("null"),
            "console.log → 'null' runtime string must be interned"
        );
        assert!(
            plan.required_runtime_strings().contains("false"),
            "console.log → 'false' runtime string must be interned"
        );
        assert!(
            plan.required_runtime_strings().contains("true"),
            "console.log → 'true' runtime string must be interned"
        );

        // Verify string_origins tracks origins
        let origins = plan.string_origins();
        assert!(
            origins.contains_key("\n"),
            "string_origins must contain newline"
        );
        assert!(
            origins.contains_key("undefined"),
            "string_origins must contain 'undefined'"
        );
        // Verify the RuntimeFn origin for "\n" includes Log
        let newline_origins = origins.get("\n").unwrap();
        assert!(
            newline_origins.contains(&RuntimeFn::Log),
            "'\\n' must originate from Log"
        );
        // Verify the RuntimeFn origin for "undefined" includes ValueToStringInto
        let undefined_origins = origins.get("undefined").unwrap();
        assert!(
            undefined_origins.contains(&RuntimeFn::ValueToStringInto),
            "'undefined' must originate from ValueToStringInto"
        );
    }
}
