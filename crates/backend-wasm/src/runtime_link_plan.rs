use std::collections::BTreeSet;

use ts2wasm_ir::lowered::{
    ClosureRepresentation, FunctionCallKind, LoweredBinaryOp, LoweredExpr, LoweredLogicalAssignOp,
    LoweredProgram, LoweredStmt, LoweredUnaryOp, Validated,
};
use ts2wasm_runtime_abi::ValueTag;

// Re-export catalog types so existing `super::runtime_link_plan::RuntimeLinkPlan`
// import paths continue to work.
use ts2wasm_runtime_catalog::{
    Capability, GLOBALS_EXCEPTION_RUNTIME, HostAbi, HostImport, RuntimeFn, RuntimeGlobal,
    runtime_fn_from_name,
};
pub use ts2wasm_runtime_catalog::{
    LinkPlanSnapshot, RuntimeLinkPlan, ValidatedRuntimeLinkPlan, emit_link_plan_snapshot,
    validate_runtime_link_plan,
};

/// Build a RuntimeLinkPlan from a lowered program by walking the full IR.
pub fn build_runtime_link_plan(program: &LoweredProgram) -> RuntimeLinkPlan {
    let mut plan = RuntimeLinkPlan::default();
    collect_required_runtime_stmts(&mut plan, &program.top_level_statements);
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
        collect_required_runtime_stmts(&mut plan, &function.body);
    }
    for module in &program.modules {
        collect_required_runtime_stmts(&mut plan, &module.statements);
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

/// Build a validated RuntimeLinkPlan from a lowered program.
/// Returns `Ok(ValidatedRuntimeLinkPlan)` on success.
pub fn build_validated_runtime_link_plan(
    program: &LoweredProgram,
) -> Result<ValidatedRuntimeLinkPlan, String> {
    let plan = build_runtime_link_plan(program);
    validate_runtime_link_plan(plan)
}

/// Generate a JSON snapshot of the RuntimeLinkPlan for a given lowered program.
/// Used by fixture-based linker structure tests.
pub fn emit_link_plan_snapshot_json(program: &Validated<LoweredProgram>) -> String {
    let plan = build_runtime_link_plan(program.as_ref());
    emit_link_plan_snapshot(&plan)
}

fn collect_required_runtime_stmts(plan: &mut RuntimeLinkPlan, statements: &[LoweredStmt]) {
    for statement in statements {
        match statement {
            LoweredStmt::Block(statements, _) => {
                collect_required_runtime_stmts(plan, statements);
            }
            LoweredStmt::Let(_, expr, _)
            | LoweredStmt::Assign(_, expr, _)
            | LoweredStmt::Expr(expr, _)
            | LoweredStmt::Return(expr, _)
            | LoweredStmt::Throw(expr, _) => {
                plan.add_required_globals(GLOBALS_EXCEPTION_RUNTIME);
                collect_required_runtime_expr(plan, expr);
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                collect_required_runtime_expr(plan, condition);
                plan.add_required_runtime(RuntimeFn::TruthyBool);
                collect_required_runtime_stmts(plan, then_body);
                collect_required_runtime_stmts(plan, else_body);
            }
            LoweredStmt::While {
                condition, body, ..
            } => {
                collect_required_runtime_expr(plan, condition);
                plan.add_required_runtime(RuntimeFn::TruthyBool);
                collect_required_runtime_stmts(plan, body);
            }
            LoweredStmt::TryCatch {
                try_body,
                catch_var: _,
                catch_body,
                finally_body,
                ..
            } => {
                plan.add_required_globals(GLOBALS_EXCEPTION_RUNTIME);
                collect_required_runtime_stmts(plan, try_body);
                if let Some(body) = catch_body {
                    collect_required_runtime_stmts(plan, body);
                }
                if let Some(body) = finally_body {
                    collect_required_runtime_stmts(plan, body);
                }
            }
            LoweredStmt::Switch { expr, cases, .. } => {
                collect_required_runtime_expr(plan, expr);
                for (cond, case_body) in cases {
                    if let Some(cond_expr) = cond {
                        collect_required_runtime_expr(plan, cond_expr);
                        plan.add_required_runtime(RuntimeFn::StrictEqual);
                    }
                    collect_required_runtime_stmts(plan, case_body);
                }
            }
            LoweredStmt::DoWhile {
                body, condition, ..
            } => {
                collect_required_runtime_expr(plan, condition);
                plan.add_required_runtime(RuntimeFn::TruthyBool);
                collect_required_runtime_stmts(plan, body);
            }
            LoweredStmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                if let Some(stmt) = init {
                    collect_required_runtime_stmts(plan, &[stmt.as_ref().clone()]);
                }
                if let Some(expr) = condition {
                    collect_required_runtime_expr(plan, expr);
                    plan.add_required_runtime(RuntimeFn::TruthyBool);
                }
                if let Some(expr) = update {
                    collect_required_runtime_expr(plan, expr);
                }
                collect_required_runtime_stmts(plan, body);
            }
            LoweredStmt::ForIn {
                var: _, iter, body, ..
            } => {
                collect_required_runtime_expr(plan, iter);
                plan.add_required_runtime(RuntimeFn::ObjectKeys);
                plan.add_required_runtime(RuntimeFn::GetLength);
                plan.add_required_runtime(RuntimeFn::Less);
                plan.add_required_runtime(RuntimeFn::TruthyBool);
                plan.add_required_runtime(RuntimeFn::ArrayGet);
                plan.add_required_runtime(RuntimeFn::Add);
                collect_required_runtime_stmts(plan, body);
            }
            LoweredStmt::ForOf {
                var: _, iter, body, ..
            } => {
                collect_required_runtime_expr(plan, iter);
                plan.add_required_runtime(RuntimeFn::GetLength);
                plan.add_required_runtime(RuntimeFn::Less);
                plan.add_required_runtime(RuntimeFn::TruthyBool);
                plan.add_required_runtime(RuntimeFn::ArrayGet);
                plan.add_required_runtime(RuntimeFn::Add);
                collect_required_runtime_stmts(plan, body);
            }
            LoweredStmt::Labeled { body, .. } => {
                collect_required_runtime_stmts(plan, std::slice::from_ref(body.as_ref()));
            }
            LoweredStmt::Break { .. } | LoweredStmt::Continue { .. } => {}
            LoweredStmt::Export { expr, .. } => {
                collect_required_runtime_expr(plan, expr);
                plan.add_required_runtime(RuntimeFn::ModuleExportsSet);
            }
            LoweredStmt::ModuleExportsAssign { expr, .. } => {
                collect_required_runtime_expr(plan, expr);
                plan.add_required_runtime(RuntimeFn::ModuleExportsAssign);
            }
            LoweredStmt::ClassDecl { methods, .. } => {
                plan.add_required_runtime(RuntimeFn::AllocHeap);
                if !methods.is_empty() {
                    plan.add_required_runtime(RuntimeFn::PropertySet);
                }
            }
        }
    }
}

fn collect_required_runtime_expr(plan: &mut RuntimeLinkPlan, expr: &LoweredExpr) {
    match expr {
        LoweredExpr::Unary { op, expr, .. } => {
            collect_required_runtime_expr(plan, expr);
            match op {
                LoweredUnaryOp::Not => plan.add_required_runtime(RuntimeFn::Not),
                LoweredUnaryOp::Plus => plan.add_required_runtime(RuntimeFn::EqualEqual),
                LoweredUnaryOp::Negate => plan.add_required_runtime(RuntimeFn::Negate),
                LoweredUnaryOp::TypeOf => plan.add_required_runtime(RuntimeFn::TypeOf),
                LoweredUnaryOp::Delete => {
                    // Delete is handled specially, no runtime function needed
                }
                LoweredUnaryOp::Void => {
                    // Void evaluates inner expr for side effects, no runtime function needed
                }
            }
        }
        LoweredExpr::Assign { expr, .. } => {
            collect_required_runtime_expr(plan, expr);
        }
        LoweredExpr::EnvCellNew(expr, _) => {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
            collect_required_runtime_expr(plan, expr);
        }
        LoweredExpr::EnvCellGet(_, _) => {}
        LoweredExpr::EnvCellSet { expr, .. } => {
            collect_required_runtime_expr(plan, expr);
        }
        LoweredExpr::LogicalAssign { op, expr, .. } => {
            collect_required_runtime_expr(plan, expr);
            if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                plan.add_required_runtime(RuntimeFn::TruthyBool);
            }
        }
        LoweredExpr::LogicalPropertyAssign { op, expr, .. } => {
            plan.add_required_runtime(RuntimeFn::PropertyGet);
            plan.add_required_runtime(RuntimeFn::PropertySet);
            collect_required_runtime_expr(plan, expr);
            if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                plan.add_required_runtime(RuntimeFn::TruthyBool);
            }
        }
        LoweredExpr::LogicalMemberAssign {
            op, object, expr, ..
        } => {
            plan.add_required_runtime(RuntimeFn::PropertyGet);
            plan.add_required_runtime(RuntimeFn::PropertySet);
            collect_required_runtime_expr(plan, object);
            collect_required_runtime_expr(plan, expr);
            if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                plan.add_required_runtime(RuntimeFn::TruthyBool);
            }
        }
        LoweredExpr::LogicalComputedPropertyAssign { op, key, expr, .. } => {
            plan.add_required_runtime(RuntimeFn::PropertyGet);
            plan.add_required_runtime(RuntimeFn::PropertySet);
            plan.add_required_runtime(RuntimeFn::ValueToStringInto);
            collect_required_runtime_expr(plan, key);
            collect_required_runtime_expr(plan, expr);
            if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                plan.add_required_runtime(RuntimeFn::TruthyBool);
            }
        }
        LoweredExpr::LogicalComputedMemberAssign {
            op,
            object,
            key,
            expr,
            ..
        } => {
            plan.add_required_runtime(RuntimeFn::PropertyGet);
            plan.add_required_runtime(RuntimeFn::PropertySet);
            plan.add_required_runtime(RuntimeFn::ValueToStringInto);
            collect_required_runtime_expr(plan, object);
            collect_required_runtime_expr(plan, key);
            collect_required_runtime_expr(plan, expr);
            if matches!(op, LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or) {
                plan.add_required_runtime(RuntimeFn::TruthyBool);
            }
        }
        LoweredExpr::Binary {
            left, op, right, ..
        } => {
            collect_required_runtime_expr(plan, left);
            collect_required_runtime_expr(plan, right);
            match op {
                LoweredBinaryOp::Add => {
                    if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                    {
                        plan.add_required_runtime(RuntimeFn::AddFast);
                    } else {
                        plan.add_required_runtime(RuntimeFn::Add);
                    }
                }
                LoweredBinaryOp::Subtract => {
                    if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                    {
                        plan.add_required_runtime(RuntimeFn::SubFast);
                    } else {
                        plan.add_required_runtime(RuntimeFn::Sub);
                    }
                }
                LoweredBinaryOp::Multiply => {
                    if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                    {
                        plan.add_required_runtime(RuntimeFn::MulFast);
                    } else {
                        plan.add_required_runtime(RuntimeFn::Mul);
                    }
                }
                LoweredBinaryOp::Power => plan.add_required_runtime(RuntimeFn::MathPow),
                LoweredBinaryOp::Divide => {
                    if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                    {
                        plan.add_required_runtime(RuntimeFn::DivFast);
                    } else {
                        plan.add_required_runtime(RuntimeFn::Div);
                    }
                }
                LoweredBinaryOp::Modulo => {
                    if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                    {
                        plan.add_required_runtime(RuntimeFn::ModFast);
                    } else {
                        plan.add_required_runtime(RuntimeFn::Mod);
                    }
                }
                LoweredBinaryOp::BitwiseAnd => plan.add_required_runtime(RuntimeFn::BitwiseAnd),
                LoweredBinaryOp::BitwiseXor => plan.add_required_runtime(RuntimeFn::BitwiseXor),
                LoweredBinaryOp::BitwiseOr => plan.add_required_runtime(RuntimeFn::BitwiseOr),
                LoweredBinaryOp::Less => {
                    if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                    {
                        plan.add_required_runtime(RuntimeFn::LessFast);
                    } else {
                        plan.add_required_runtime(RuntimeFn::Less);
                    }
                }
                LoweredBinaryOp::LessEqual => {
                    if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                    {
                        plan.add_required_runtime(RuntimeFn::LessEqualFast);
                    } else {
                        plan.add_required_runtime(RuntimeFn::LessEqual);
                    }
                }
                LoweredBinaryOp::Greater => {
                    if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                    {
                        plan.add_required_runtime(RuntimeFn::GreaterFast);
                    } else {
                        plan.add_required_runtime(RuntimeFn::Greater);
                    }
                }
                LoweredBinaryOp::GreaterEqual => {
                    if left.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                        && right.inferred_type() == ts2wasm_ir::lowered::InferredType::Number
                    {
                        plan.add_required_runtime(RuntimeFn::GreaterEqualFast);
                    } else {
                        plan.add_required_runtime(RuntimeFn::GreaterEqual);
                    }
                }
                LoweredBinaryOp::StrictEqual => plan.add_required_runtime(RuntimeFn::StrictEqual),
                LoweredBinaryOp::EqualEqual => plan.add_required_runtime(RuntimeFn::EqualEqual),
                LoweredBinaryOp::BangEqual => plan.add_required_runtime(RuntimeFn::BangEqual),
                LoweredBinaryOp::StrictNotEqual => {
                    plan.add_required_runtime(RuntimeFn::StrictNotEqual)
                }
                LoweredBinaryOp::And | LoweredBinaryOp::Or => {
                    plan.add_required_runtime(RuntimeFn::TruthyBool)
                }
                LoweredBinaryOp::NullishCoalesce => {}
            }
        }
        LoweredExpr::Call { kind, args, .. } => {
            for arg in args {
                collect_required_runtime_expr(plan, arg);
            }
            if let FunctionCallKind::Builtin(builtin) = kind {
                plan.add_required_runtime(crate::runtime_fn::runtime_fn_from_builtin(*builtin));
            }
        }
        LoweredExpr::Number(value, _) => {
            if !ValueTag::can_encode_number(*value) {
                plan.add_required_runtime(RuntimeFn::NumberFromI32);
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
                plan.add_required_runtime(RuntimeFn::AllocHeap);
            }
        }
        LoweredExpr::BigIntLiteral { .. } => {
            plan.add_required_runtime(RuntimeFn::MakeBigIntLiteral);
        }
        LoweredExpr::ArrayNew { elements, .. } => {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
            for elem in elements {
                collect_required_runtime_expr(plan, elem);
            }
        }
        LoweredExpr::ArrayNewSparse { slots, .. } => {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
            for slot in slots {
                if let ts2wasm_ir::lowered::LoweredArraySlot::Present(elem) = slot {
                    collect_required_runtime_expr(plan, elem);
                }
            }
        }
        LoweredExpr::ArrayGet { arr, index, .. } => {
            plan.add_required_runtime(RuntimeFn::ArrayGet);
            collect_required_runtime_expr(plan, arr);
            collect_required_runtime_expr(plan, index);
        }
        LoweredExpr::Index { object, index, .. } => {
            plan.add_required_runtime(RuntimeFn::Index);
            collect_required_runtime_expr(plan, object);
            collect_required_runtime_expr(plan, index);
        }
        LoweredExpr::GetLength(inner, _) => {
            plan.add_required_runtime(RuntimeFn::GetLength);
            collect_required_runtime_expr(plan, inner);
        }
        LoweredExpr::ObjectNew { props, .. } => {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
            for (_, val) in props {
                collect_required_runtime_expr(plan, val);
            }
        }
        LoweredExpr::ErrorNew { message, .. } => {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
            plan.add_required_runtime(RuntimeFn::Concat);
            collect_required_runtime_expr(plan, message);
        }
        LoweredExpr::PropertyGet { obj, .. } => {
            plan.add_required_runtime(RuntimeFn::PropertyGet);
            collect_required_runtime_expr(plan, obj);
        }
        LoweredExpr::OptionalPropertyGet { obj, .. } => {
            plan.add_required_runtime(RuntimeFn::PropertyGet);
            collect_required_runtime_expr(plan, obj);
        }
        LoweredExpr::PropertyGetDynamic { obj, key, .. } => {
            plan.add_required_runtime(RuntimeFn::PropertyGet);
            plan.add_required_runtime(RuntimeFn::ValueToStringInto);
            collect_required_runtime_expr(plan, obj);
            collect_required_runtime_expr(plan, key);
        }
        LoweredExpr::OptionalIndex { object, index, .. } => {
            plan.add_required_runtime(RuntimeFn::Index);
            collect_required_runtime_expr(plan, object);
            collect_required_runtime_expr(plan, index);
        }
        LoweredExpr::OptionalCall { callee, call, .. } => {
            collect_required_runtime_expr(plan, callee);
            collect_required_runtime_expr(plan, call);
        }
        LoweredExpr::PropertySet { object, value, .. } => {
            plan.add_required_runtime(RuntimeFn::PropertySet);
            collect_required_runtime_expr(plan, object);
            collect_required_runtime_expr(plan, value);
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => {
            plan.add_required_runtime(RuntimeFn::PropertySet);
            plan.add_required_runtime(RuntimeFn::ValueToStringInto);
            collect_required_runtime_expr(plan, object);
            collect_required_runtime_expr(plan, index);
            collect_required_runtime_expr(plan, value);
        }
        LoweredExpr::New { args, .. } => {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
            for arg in args {
                collect_required_runtime_expr(plan, arg);
            }
        }
        #[allow(unreachable_patterns)]
        LoweredExpr::PromiseGetValue { promise, .. } => {
            collect_required_runtime_expr(plan, promise);
            plan.add_required_runtime(RuntimeFn::TaskResult);
        }
        LoweredExpr::ClassPrototype(_, _) => {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
        }
        LoweredExpr::BuiltinErrorPrototype(_, _) => {
            plan.add_required_runtime(RuntimeFn::AllocHeap);
        }
        LoweredExpr::Block { stmts, result, .. } => {
            for stmt in stmts {
                collect_required_runtime_stmts(plan, std::slice::from_ref(stmt));
            }
            collect_required_runtime_expr(plan, result);
        }
        LoweredExpr::MethodCall { .. } => {}
        LoweredExpr::ModuleLoad { .. } => {
            plan.add_required_runtime(RuntimeFn::ModuleRequire);
        }
        LoweredExpr::RuntimeCall {
            intrinsic, args, ..
        } => {
            if *intrinsic == RuntimeFn::ArrayPushMany {
                plan.add_required_runtime(RuntimeFn::ArrayPush);
                plan.add_required_runtime(RuntimeFn::ArrayPushGrow);
                plan.add_required_runtime(RuntimeFn::GetLength);
            }
            if *intrinsic == RuntimeFn::ArrayPushGrow {
                plan.add_required_runtime(RuntimeFn::ArrayPushGrow);
            }
            if *intrinsic == RuntimeFn::PrivateFieldGet
                || *intrinsic == RuntimeFn::PrivateFieldSet
                || *intrinsic == RuntimeFn::PrivateBrandCheck
            {
                plan.add_required_runtime(RuntimeFn::PrivateBrandTypeError);
            }
            if let Some(runtime_fn_enum) = runtime_fn_from_name(&format!("{:?}", intrinsic)) {
                plan.add_required_runtime(runtime_fn_enum);
            }
            for arg in args {
                collect_required_runtime_expr(plan, arg);
            }
        }
        LoweredExpr::PropertyDelete { object, key: _, .. } => {
            collect_required_runtime_expr(plan, object);
            plan.add_required_runtime(RuntimeFn::PropertyDelete);
        }
        LoweredExpr::PropertyDeleteDynamic { object, key, .. } => {
            collect_required_runtime_expr(plan, object);
            collect_required_runtime_expr(plan, key);
            plan.add_required_runtime(RuntimeFn::PropertyDelete);
            plan.add_required_runtime(RuntimeFn::ValueToStringInto);
        }
        LoweredExpr::PropertyIn { obj, key: _, .. } => {
            collect_required_runtime_expr(plan, obj);
            plan.add_required_runtime(RuntimeFn::PropertyHas);
        }
        LoweredExpr::PropertyInDynamic { obj, key, .. } => {
            collect_required_runtime_expr(plan, obj);
            collect_required_runtime_expr(plan, key);
            plan.add_required_runtime(RuntimeFn::PropertyHas);
            plan.add_required_runtime(RuntimeFn::ValueToStringInto);
        }
    }
}

#[cfg(test)]
mod tests {
    use ts2wasm_ir::builtin::BuiltinId;
    use ts2wasm_ir::lowered::{
        FuncId, FunctionCallKind, LoweredBinaryOp, LoweredExpr, LoweredProgram, LoweredStmt,
        ModuleInfo,
    };
    use ts2wasm_source::Span;

    use super::build_runtime_link_plan;
    use ts2wasm_runtime_catalog::{HostImport, RuntimeFn, RuntimeGlobal};

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

        let plan = build_runtime_link_plan(&program);

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

        let plan = build_runtime_link_plan(&program);

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
                        intrinsic: RuntimeFn::BigIntAdd,
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
                        intrinsic: RuntimeFn::BigIntUnaryMinus,
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
                        intrinsic: RuntimeFn::BigIntMul,
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
                        intrinsic: RuntimeFn::BigIntPow,
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
                        intrinsic: RuntimeFn::BigIntDiv,
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
                        intrinsic: RuntimeFn::BigIntRem,
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
                        intrinsic: RuntimeFn::BigIntMixedArithmeticTypeError,
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
                        intrinsic: RuntimeFn::BigIntBitwiseNot,
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
                        intrinsic: RuntimeFn::BigIntBitwiseAnd,
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
                        intrinsic: RuntimeFn::BigIntBitwiseOr,
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
                        intrinsic: RuntimeFn::BigIntBitwiseXor,
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
                        intrinsic: RuntimeFn::BigIntFromValue,
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
                        intrinsic: RuntimeFn::BigIntAsIntN,
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
                        intrinsic: RuntimeFn::BigIntAsUintN,
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

        let plan = build_runtime_link_plan(&program);

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

        let plan = build_runtime_link_plan(&program);

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
                    intrinsic: RuntimeFn::BigIntToString,
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

        let plan = build_runtime_link_plan(&program);

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

        let plan = build_runtime_link_plan(&program);

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

        let plan = build_runtime_link_plan(&program);

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

        let plan = build_runtime_link_plan(&program);

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
