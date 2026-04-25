use std::collections::BTreeSet;

use crate::ir::lowered::{
    FunctionCallKind, LoweredBinaryOp, LoweredExpr, LoweredProgram, LoweredStmt, LoweredUnaryOp,
};

use super::runtime_fn::{Capability, HostImport, RuntimeFn};

#[derive(Debug, Default, Clone)]
pub(crate) struct RuntimeLinkPlan {
    required_runtime: BTreeSet<RuntimeFn>,
    required_imports: BTreeSet<HostImport>,
    required_capabilities: BTreeSet<Capability>,
    required_runtime_strings: BTreeSet<&'static str>,
}

impl RuntimeLinkPlan {
    pub(crate) fn from_program(program: &LoweredProgram) -> Self {
        let mut plan = Self::default();
        plan.collect_required_runtime_stmts(&program.top_level_statements);
        for function in &program.functions {
            plan.collect_required_runtime_stmts(&function.body);
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

    pub(crate) fn required_capabilities(&self) -> &BTreeSet<Capability> {
        &self.required_capabilities
    }

    pub(crate) fn required_runtime_strings(&self) -> &BTreeSet<&'static str> {
        &self.required_runtime_strings
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
        for runtime_fn in &self.required_runtime {
            for import in runtime_fn.spec().imports {
                self.required_imports.insert(*import);
            }
            for capability in runtime_fn.spec().capability {
                self.required_capabilities.insert(*capability);
            }
            for value in runtime_fn.spec().runtime_strings {
                self.required_runtime_strings.insert(*value);
            }
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
                LoweredStmt::ForIn { var: _, iter, body } => {
                    self.collect_required_runtime_expr(iter);
                    self.collect_required_runtime_stmts(body);
                }
                LoweredStmt::ForOf { var: _, iter, body } => {
                    self.collect_required_runtime_expr(iter);
                    self.collect_required_runtime_stmts(body);
                }
                LoweredStmt::Break | LoweredStmt::Continue => {}
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
                }
            }
            LoweredExpr::Binary { left, op, right } => {
                self.collect_required_runtime_expr(left);
                self.collect_required_runtime_expr(right);
                match op {
                    LoweredBinaryOp::Add => self.add_required_runtime(RuntimeFn::Add),
                    LoweredBinaryOp::Subtract => self.add_required_runtime(RuntimeFn::Sub),
                    LoweredBinaryOp::Less => self.add_required_runtime(RuntimeFn::Less),
                    LoweredBinaryOp::Greater => self.add_required_runtime(RuntimeFn::Greater),
                    LoweredBinaryOp::StrictEqual => {
                        self.add_required_runtime(RuntimeFn::StrictEqual)
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
            | LoweredExpr::Local(_) => {}
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
            LoweredExpr::MethodCall { .. }
            | LoweredExpr::PropertySet { .. }
            | LoweredExpr::New { .. } => {}
            LoweredExpr::RuntimeCall { runtime_fn, args } => {
                self.add_required_runtime(*runtime_fn);
                for arg in args {
                    self.collect_required_runtime_expr(arg);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::backend::emit_wat;
    use crate::backend::runtime_fn::{Capability, HostImport, RuntimeFn};
    use crate::ir::lowered::lower_program;

    use super::RuntimeLinkPlan;

    fn lowered(source: &str) -> crate::ir::lowered::LoweredProgram {
        let program = crate::parse_program(source).expect("parse failed");
        let resolved = crate::ir::builtin_resolver::resolve_builtins(&program)
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
                .contains(&crate::runtime::consts::RuntimeString::NEWLINE)
        );
        assert!(
            plan.required_runtime_strings()
                .contains(&crate::runtime::consts::RuntimeString::UNDEFINED)
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
                .contains(&crate::runtime::consts::RuntimeString::UNDEFINED)
        );
        assert!(
            !plan
                .required_runtime_strings()
                .contains(&crate::runtime::consts::RuntimeString::NEWLINE)
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
        let plan = RuntimeLinkPlan::from_required_runtime_for_tests(&[RuntimeFn::ReadStdinUtf8]);
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
                .contains(&RuntimeFn::ReadStdinUtf8)
        );
        assert!(plan.required_imports().contains(&HostImport::FdRead));
        assert!(
            plan.required_capabilities()
                .contains(&Capability::StdinRead)
        );
    }
}
