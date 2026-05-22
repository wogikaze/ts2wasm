//! Constant folding pass for MIR.
//!
//! Folds `Phi` and `Select` nodes into constants when all inputs are known
//! constant values. This pass runs after DCE in the MIR pipeline.
//!
//! ## Phi folding
//!
//! If all incoming values to a Phi node are the same constant (e.g., all are
//! `Number(42)`), the Phi is replaced with that constant.
//!
//! ## Select folding
//!
//! - If the condition is `Bool(true)`, fold to the true branch.
//! - If the condition is `Bool(false)`, fold to the false branch.
//! - If both branches are the same constant, fold unconditionally.

use super::types::{MirExpr, MirFunction, MirProgram, MirStmt};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Run constant folding on all functions in a `MirProgram`.
///
/// Returns `true` if any changes were made.
pub fn run_constant_folding(program: &mut MirProgram) -> bool {
    let mut changed = false;

    for func in &mut program.functions {
        changed |= fold_constants_in_stmts(&mut func.body);
    }

    changed |= fold_constants_in_stmts(&mut program.top_level_statements);

    changed
}

// ---------------------------------------------------------------------------
// Statement walker
// ---------------------------------------------------------------------------

/// Walk through statements and fold constants in expressions.
fn fold_constants_in_stmts(stmts: &mut Vec<MirStmt>) -> bool {
    let mut changed = false;
    for stmt in stmts.iter_mut() {
        match stmt {
            MirStmt::Let(_, expr, _) | MirStmt::Assign(_, expr, _) => {
                changed |= fold_constants_in_expr(expr);
            }
            MirStmt::Expr(expr, _) | MirStmt::Yield(expr, _) => {
                changed |= fold_constants_in_expr(expr);
            }
            MirStmt::Return(expr, _) | MirStmt::Throw(expr, _) => {
                changed |= fold_constants_in_expr(expr);
            }
            MirStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                changed |= fold_constants_in_expr(condition);
                changed |= fold_constants_in_stmts(then_body);
                changed |= fold_constants_in_stmts(else_body);
            }
            MirStmt::While {
                condition, body, ..
            } => {
                changed |= fold_constants_in_expr(condition);
                changed |= fold_constants_in_stmts(body);
            }
            MirStmt::DoWhile {
                body, condition, ..
            } => {
                changed |= fold_constants_in_stmts(body);
                changed |= fold_constants_in_expr(condition);
            }
            MirStmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                if let Some(init_stmt) = init {
                    fold_constants_in_stmts(std::slice::from_mut(init_stmt.as_mut()));
                }
                if let Some(cond) = condition {
                    changed |= fold_constants_in_expr(cond);
                }
                if let Some(upd) = update {
                    changed |= fold_constants_in_expr(upd);
                }
                changed |= fold_constants_in_stmts(body);
            }
            MirStmt::ForIn { iter, body, .. }
            | MirStmt::ForOf { iter, body, .. }
            | MirStmt::ForAwaitOfLower { iter, body, .. } => {
                changed |= fold_constants_in_expr(iter);
                changed |= fold_constants_in_stmts(body);
            }
            MirStmt::Block(children, _) => {
                changed |= fold_constants_in_stmts(children);
            }
            MirStmt::TryFinally {
                try_body,
                finally_body,
                ..
            } => {
                changed |= fold_constants_in_stmts(try_body);
                changed |= fold_constants_in_stmts(finally_body);
            }
            MirStmt::TryCatch {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                changed |= fold_constants_in_stmts(try_body);
                if let Some(body) = catch_body {
                    changed |= fold_constants_in_stmts(body);
                }
                if let Some(body) = finally_body {
                    changed |= fold_constants_in_stmts(body);
                }
            }
            MirStmt::Switch { expr, cases, .. } => {
                changed |= fold_constants_in_expr(expr);
                for (cond, body) in cases {
                    if let Some(cond_expr) = cond {
                        changed |= fold_constants_in_expr(cond_expr);
                    }
                    changed |= fold_constants_in_stmts(body);
                }
            }
            MirStmt::Labeled { body, .. } => {
                changed |= fold_constants_in_stmts(std::slice::from_mut(body.as_mut()));
            }
            MirStmt::Export { expr, .. } | MirStmt::ModuleExportsAssign { expr, .. } => {
                changed |= fold_constants_in_expr(expr);
            }
            MirStmt::Break { .. }
            | MirStmt::Continue { .. }
            | MirStmt::ModuleExportsUpdate { .. }
            | MirStmt::ClassDecl { .. } => {}
        }
    }
    changed
}

// ---------------------------------------------------------------------------
// Expression folding
// ---------------------------------------------------------------------------

/// Recursively fold constants in a MIR expression.
///
/// Returns `true` if any replacement was made.
fn fold_constants_in_expr(expr: &mut MirExpr) -> bool {
    match expr {
        // Phi folding: if all values are the same constant, fold to that constant.
        MirExpr::Phi { values, span } => {
            if let Some(constant) = try_fold_phi(values) {
                *expr = constant;
                return true;
            }
            // Recurse into values even if Phi didn't fold.
            let mut changed = false;
            for v in values.iter_mut() {
                changed |= fold_constants_in_expr(v);
            }
            changed
        }

        // Select folding: if condition is known bool, fold to appropriate branch.
        MirExpr::Select {
            condition,
            true_value,
            false_value,
            span,
        } => {
            // First, recurse into sub-expressions.
            let cond_changed = fold_constants_in_expr(condition);
            let true_changed = fold_constants_in_expr(true_value);
            let false_changed = fold_constants_in_expr(false_value);

            // Now try folding the Select itself.
            if let Some(constant) = try_fold_select(condition, true_value, false_value) {
                *expr = constant;
                return true;
            }

            cond_changed || true_changed || false_changed
        }

        // Recurse into other compound expressions.
        MirExpr::Unary { expr: inner, .. } => fold_constants_in_expr(inner),

        MirExpr::Binary { left, right, .. } => {
            let l = fold_constants_in_expr(left);
            let r = fold_constants_in_expr(right);
            l || r
        }

        MirExpr::Assign { expr: inner, .. } => fold_constants_in_expr(inner),

        MirExpr::LogicalAssign { expr, .. }
        | MirExpr::LogicalPropertyAssign { expr, .. } => fold_constants_in_expr(expr),

        MirExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            let k = fold_constants_in_expr(key);
            let e = fold_constants_in_expr(expr);
            k || e
        }

        MirExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            let o = fold_constants_in_expr(object);
            let k = fold_constants_in_expr(key);
            let e = fold_constants_in_expr(expr);
            o || k || e
        }

        MirExpr::LogicalMemberAssign { object, expr, .. } => {
            let o = fold_constants_in_expr(object);
            let e = fold_constants_in_expr(expr);
            o || e
        }

        MirExpr::EnvCellNew(inner, _) => fold_constants_in_expr(inner),

        MirExpr::EnvCellSet { expr: inner, .. } => fold_constants_in_expr(inner),

        MirExpr::PropertyIn { obj, .. } => fold_constants_in_expr(obj),

        MirExpr::PropertyInDynamic { obj, key, .. } => {
            let o = fold_constants_in_expr(obj);
            let k = fold_constants_in_expr(key);
            o || k
        }

        MirExpr::Call { args, .. }
        | MirExpr::RuntimeCall { args, .. }
        | MirExpr::New { args, .. } => {
            let mut changed = false;
            for arg in args.iter_mut() {
                changed |= fold_constants_in_expr(arg);
            }
            changed
        }

        MirExpr::OptionalCall { callee, call, .. } => {
            let c = fold_constants_in_expr(callee);
            let ca = fold_constants_in_expr(call);
            c || ca
        }

        MirExpr::ArrayNew { elements, .. } => {
            let mut changed = false;
            for elem in elements.iter_mut() {
                changed |= fold_constants_in_expr(elem);
            }
            changed
        }

        MirExpr::ArrayNewSparse { slots, .. } => {
            let mut changed = false;
            for slot in slots.iter_mut() {
                if let super::types::MirArraySlot::Present(elem) = slot {
                    changed |= fold_constants_in_expr(elem);
                }
            }
            changed
        }

        MirExpr::ArrayGet { arr, index, .. }
        | MirExpr::Index {
            object: arr, index, ..
        } => {
            let a = fold_constants_in_expr(arr);
            let i = fold_constants_in_expr(index);
            a || i
        }

        MirExpr::GetLength(inner, _) => fold_constants_in_expr(inner),

        MirExpr::ObjectNew { props, .. } => {
            let mut changed = false;
            for (_, val) in props.iter_mut() {
                changed |= fold_constants_in_expr(val);
            }
            changed
        }

        MirExpr::ErrorNew { message, cause, .. } => {
            let m = fold_constants_in_expr(message);
            let c = cause.as_mut().map_or(false, |c| fold_constants_in_expr(c));
            m || c
        }

        MirExpr::PropertyGet { obj, .. }
        | MirExpr::OptionalPropertyGet { obj, .. }
        | MirExpr::MethodCall { object: obj, .. }
        | MirExpr::PropertyDelete { object: obj, .. } => fold_constants_in_expr(obj),

        MirExpr::PropertyGetDynamic { obj, key, .. }
        | MirExpr::PropertyDeleteDynamic {
            object: obj, key, ..
        } => {
            let o = fold_constants_in_expr(obj);
            let k = fold_constants_in_expr(key);
            o || k
        }

        MirExpr::OptionalIndex { object, index, .. } => {
            let o = fold_constants_in_expr(object);
            let i = fold_constants_in_expr(index);
            o || i
        }

        MirExpr::PropertySet { object, value, .. }
        | MirExpr::PropertySetDynamic {
            object,
            index: _,
            value,
            ..
        } => {
            let o = fold_constants_in_expr(object);
            let v = fold_constants_in_expr(value);
            o || v
        }

        MirExpr::PromiseGetValue { promise, .. } => fold_constants_in_expr(promise),

        MirExpr::Block { stmts, result, .. } => {
            let s = fold_constants_in_stmts(stmts);
            let r = fold_constants_in_expr(result);
            s || r
        }

        // Literals and leaf expressions — no sub-expressions to fold.
        MirExpr::Number(..)
        | MirExpr::DecimalNumber(..)
        | MirExpr::BigIntLiteral { .. }
        | MirExpr::String(..)
        | MirExpr::Bool(..)
        | MirExpr::Null(_)
        | MirExpr::Undefined(_)
        | MirExpr::Local(..)
        | MirExpr::EnvCellGet(..)
        | MirExpr::This(_)
        | MirExpr::ClassPrototype(..)
        | MirExpr::BuiltinErrorPrototype(..)
        | MirExpr::ModuleLoad { .. }
        | MirExpr::ArrowFn { .. } => false,
    }
}

// ---------------------------------------------------------------------------
// Folding helpers
// ---------------------------------------------------------------------------

/// Try to fold a Phi node. If all values are the same constant, return that
/// constant. Otherwise return `None`.
fn try_fold_phi(values: &[MirExpr]) -> Option<MirExpr> {
    if values.is_empty() {
        return None;
    }

    let first = &values[0];
    if !is_constant_expr(first) {
        return None;
    }

    for v in &values[1..] {
        if !exprs_equal_as_constants(first, v) {
            return None;
        }
    }

    Some(first.clone())
}

/// Try to fold a Select node.
///
/// Cases:
/// 1. Condition is `Bool(true)` -> fold to true_value.
/// 2. Condition is `Bool(false)` -> fold to false_value.
/// 3. Both branches are the same constant -> fold unconditionally.
fn try_fold_select(
    condition: &MirExpr,
    true_value: &MirExpr,
    false_value: &MirExpr,
) -> Option<MirExpr> {
    // Case 1: condition is Bool(true)
    if let MirExpr::Bool(true, _) = condition {
        return Some(true_value.clone());
    }

    // Case 2: condition is Bool(false)
    if let MirExpr::Bool(false, _) = condition {
        return Some(false_value.clone());
    }

    // Case 3: both branches are the same constant
    if exprs_equal_as_constants(true_value, false_value) {
        return Some(true_value.clone());
    }

    None
}

/// Returns `true` if `expr` is a constant literal expression.
fn is_constant_expr(expr: &MirExpr) -> bool {
    matches!(
        expr,
        MirExpr::Number(..)
            | MirExpr::DecimalNumber(..)
            | MirExpr::BigIntLiteral { .. }
            | MirExpr::String(..)
            | MirExpr::Bool(..)
            | MirExpr::Null(_)
            | MirExpr::Undefined(_)
    )
}

/// Structural equality for constant expressions.
///
/// Two constants are equal if they are the same variant and have the same
/// data payload. This is deliberately stricter than JS semantics (e.g.,
/// `null == undefined` is false here).
fn exprs_equal_as_constants(a: &MirExpr, b: &MirExpr) -> bool {
    match (a, b) {
        (MirExpr::Number(na, _), MirExpr::Number(nb, _)) => na == nb,
        (MirExpr::DecimalNumber(da, _), MirExpr::DecimalNumber(db, _)) => da == db,
        (
            MirExpr::BigIntLiteral {
                decimal: da,
                sign: sa,
                limb_low: lla,
                limb_high: lha,
                ..
            },
            MirExpr::BigIntLiteral {
                decimal: db,
                sign: sb,
                limb_low: llb,
                limb_high: lhb,
                ..
            },
        ) => da == db && sa == sb && lla == llb && lha == lhb,
        (MirExpr::String(sa, _), MirExpr::String(sb, _)) => sa == sb,
        (MirExpr::Bool(ba, _), MirExpr::Bool(bb, _)) => ba == bb,
        (MirExpr::Null(_), MirExpr::Null(_)) => true,
        (MirExpr::Undefined(_), MirExpr::Undefined(_)) => true,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lowered::LocalId;
    use ts2wasm_source::Span;

    fn s() -> Span {
        Span { start: 0, end: 0 }
    }

    // -----------------------------------------------------------------------
    // Phi folding tests
    // -----------------------------------------------------------------------

    #[test]
    fn fold_phi_all_same_number() {
        let values = vec![
            MirExpr::Number(42, s()),
            MirExpr::Number(42, s()),
            MirExpr::Number(42, s()),
        ];
        let result = try_fold_phi(&values);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::Number(42, s()));
    }

    #[test]
    fn fold_phi_all_same_bool() {
        let values = vec![
            MirExpr::Bool(true, s()),
            MirExpr::Bool(true, s()),
        ];
        let result = try_fold_phi(&values);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::Bool(true, s()));
    }

    #[test]
    fn fold_phi_all_same_string() {
        let values = vec![
            MirExpr::String("hello".to_string(), s()),
            MirExpr::String("hello".to_string(), s()),
        ];
        let result = try_fold_phi(&values);
        assert!(result.is_some());
    }

    #[test]
    fn fold_phi_all_same_null() {
        let values = vec![
            MirExpr::Null(s()),
            MirExpr::Null(s()),
        ];
        let result = try_fold_phi(&values);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::Null(s()));
    }

    #[test]
    fn fold_phi_all_same_undefined() {
        let values = vec![
            MirExpr::Undefined(s()),
            MirExpr::Undefined(s()),
        ];
        let result = try_fold_phi(&values);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::Undefined(s()));
    }

    #[test]
    fn fold_phi_different_values_not_folded() {
        let values = vec![
            MirExpr::Number(1, s()),
            MirExpr::Number(2, s()),
        ];
        let result = try_fold_phi(&values);
        assert!(result.is_none());
    }

    #[test]
    fn fold_phi_empty_not_folded() {
        let values = vec![];
        let result = try_fold_phi(&values);
        assert!(result.is_none());
    }

    #[test]
    fn fold_phi_contains_local_not_folded() {
        let values = vec![
            MirExpr::Number(1, s()),
            MirExpr::Local(LocalId(0), s()),
        ];
        let result = try_fold_phi(&values);
        assert!(result.is_none());
    }

    #[test]
    fn fold_phi_single_value() {
        let values = vec![MirExpr::Number(99, s())];
        let result = try_fold_phi(&values);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::Number(99, s()));
    }

    #[test]
    fn fold_phi_different_types_not_folded() {
        let values = vec![
            MirExpr::Number(1, s()),
            MirExpr::Bool(true, s()),
        ];
        let result = try_fold_phi(&values);
        assert!(result.is_none());
    }

    #[test]
    fn fold_phi_null_and_undefined_not_equal() {
        let values = vec![
            MirExpr::Null(s()),
            MirExpr::Undefined(s()),
        ];
        let result = try_fold_phi(&values);
        assert!(result.is_none());
    }

    // -----------------------------------------------------------------------
    // Select folding tests
    // -----------------------------------------------------------------------

    #[test]
    fn fold_select_true_branch() {
        let cond = MirExpr::Bool(true, s());
        let true_val = MirExpr::Number(42, s());
        let false_val = MirExpr::Number(0, s());
        let result = try_fold_select(&cond, &true_val, &false_val);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::Number(42, s()));
    }

    #[test]
    fn fold_select_false_branch() {
        let cond = MirExpr::Bool(false, s());
        let true_val = MirExpr::Number(42, s());
        let false_val = MirExpr::Number(0, s());
        let result = try_fold_select(&cond, &true_val, &false_val);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::Number(0, s()));
    }

    #[test]
    fn fold_select_same_branches() {
        let cond = MirExpr::Local(LocalId(0), s()); // unknown condition
        let true_val = MirExpr::Number(7, s());
        let false_val = MirExpr::Number(7, s());
        let result = try_fold_select(&cond, &true_val, &false_val);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::Number(7, s()));
    }

    #[test]
    fn fold_select_different_branches_unknown_cond() {
        let cond = MirExpr::Local(LocalId(0), s());
        let true_val = MirExpr::Number(1, s());
        let false_val = MirExpr::Number(2, s());
        let result = try_fold_select(&cond, &true_val, &false_val);
        assert!(result.is_none());
    }

    #[test]
    fn fold_select_true_branch_with_complex() {
        let cond = MirExpr::Bool(true, s());
        let true_val = MirExpr::String("yes".to_string(), s());
        let false_val = MirExpr::String("no".to_string(), s());
        let result = try_fold_select(&cond, &true_val, &false_val);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::String("yes".to_string(), s()));
    }

    #[test]
    fn fold_select_both_null() {
        let cond = MirExpr::Local(LocalId(0), s());
        let result = try_fold_select(&cond, &MirExpr::Null(s()), &MirExpr::Null(s()));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), MirExpr::Null(s()));
    }

    #[test]
    fn fold_select_both_undefined() {
        let cond = MirExpr::Local(LocalId(0), s());
        let result = try_fold_select(&cond, &MirExpr::Undefined(s()), &MirExpr::Undefined(s()));
        assert!(result.is_some());
    }

    // -----------------------------------------------------------------------
    // Integration: full expression folding
    // -----------------------------------------------------------------------

    #[test]
    fn fold_phi_in_let() {
        let mut stmts = vec![MirStmt::Let(
            LocalId(0),
            MirExpr::Phi {
                values: vec![
                    MirExpr::Number(10, s()),
                    MirExpr::Number(10, s()),
                ],
                span: s(),
            },
            s(),
        )];
        let changed = fold_constants_in_stmts(&mut stmts);
        assert!(changed);
        match &stmts[0] {
            MirStmt::Let(_, expr, _) => {
                assert_eq!(expr, &MirExpr::Number(10, s()));
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn fold_select_true_in_let() {
        let mut stmts = vec![MirStmt::Let(
            LocalId(0),
            MirExpr::Select {
                condition: Box::new(MirExpr::Bool(true, s())),
                true_value: Box::new(MirExpr::Number(100, s())),
                false_value: Box::new(MirExpr::Number(200, s())),
                span: s(),
            },
            s(),
        )];
        let changed = fold_constants_in_stmts(&mut stmts);
        assert!(changed);
        match &stmts[0] {
            MirStmt::Let(_, expr, _) => {
                assert_eq!(expr, &MirExpr::Number(100, s()));
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn fold_select_false_in_let() {
        let mut stmts = vec![MirStmt::Let(
            LocalId(0),
            MirExpr::Select {
                condition: Box::new(MirExpr::Bool(false, s())),
                true_value: Box::new(MirExpr::Number(100, s())),
                false_value: Box::new(MirExpr::Number(200, s())),
                span: s(),
            },
            s(),
        )];
        let changed = fold_constants_in_stmts(&mut stmts);
        assert!(changed);
        match &stmts[0] {
            MirStmt::Let(_, expr, _) => {
                assert_eq!(expr, &MirExpr::Number(200, s()));
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn fold_no_change_when_not_foldable() {
        let mut stmts = vec![MirStmt::Let(
            LocalId(0),
            MirExpr::Phi {
                values: vec![
                    MirExpr::Number(1, s()),
                    MirExpr::Number(2, s()),
                ],
                span: s(),
            },
            s(),
        )];
        let changed = fold_constants_in_stmts(&mut stmts);
        assert!(!changed);
    }

    #[test]
    fn fold_nested_phi_in_block() {
        // Block with a Phi that should be folded.
        let mut expr = MirExpr::Block {
            stmts: vec![],
            result: Box::new(MirExpr::Phi {
                values: vec![
                    MirExpr::Bool(false, s()),
                    MirExpr::Bool(false, s()),
                ],
                span: s(),
            }),
            span: s(),
        };
        let changed = fold_constants_in_expr(&mut expr);
        assert!(changed);
        match &expr {
            MirExpr::Block { result, .. } => {
                assert_eq!(result.as_ref(), &MirExpr::Bool(false, s()));
            }
            _ => panic!("expected Block"),
        }
    }

    #[test]
    fn run_constant_folding_on_program() {
        let mut func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![MirStmt::Let(
                LocalId(0),
                MirExpr::Select {
                    condition: Box::new(MirExpr::Bool(true, s())),
                    true_value: Box::new(MirExpr::Number(42, s())),
                    false_value: Box::new(MirExpr::Number(0, s())),
                    span: s(),
                },
                s(),
            )],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
            escape_status: vec![],
        };

        let mut program = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![func],
            modules: vec![],
            escape_status: vec![],
        };

        let changed = run_constant_folding(&mut program);
        assert!(changed);
        match &program.functions[0].body[0] {
            MirStmt::Let(_, expr, _) => {
                assert_eq!(expr, &MirExpr::Number(42, s()));
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn run_constant_folding_no_change() {
        let mut func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![MirStmt::Let(
                LocalId(0),
                MirExpr::Number(1, s()),
                s(),
            )],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
            escape_status: vec![],
        };

        let mut program = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![func],
            modules: vec![],
            escape_status: vec![],
        };

        let changed = run_constant_folding(&mut program);
        assert!(!changed);
    }
}
