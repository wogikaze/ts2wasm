//! Induction variable analysis for for-loops lowered to MIR.
//!
//! Detects simple for-loop induction variables of the form:
//!
//! ```text
//! let i = <const>;
//! while (i < n) {
//!     // loop body
//!     i = i + 1;
//! }
//! ```
//!
//! Or the structured `For` variant directly:
//!
//! ```text
//! for (let i = <const>; i < n; i = i + 1) { ... }
//! ```
//!
//! This is an analysis-only pass. Results are attached to `MirFunction` as
//! `induction_vars` and are intended for consumption by the MIR emitter.

use super::types::{MirExpr, MirFunction, MirStmt};
use crate::lowered::{LocalId, LoweredBinaryOp};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Direction of an induction variable's step per iteration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InductionVarDirection {
    /// Induction variable increases each iteration (step = +1).
    Increasing,
    /// Induction variable decreases each iteration (step = -1).
    Decreasing,
}

/// Information about a detected for-loop induction variable.
///
/// The variable at `local` starts at `start`, is compared against `end`
/// via the loop condition, and increments by `step` each iteration.
/// The `end` value is the exclusive bound used in the comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InductionVarInfo {
    /// The local variable ID used as the induction variable (counter).
    pub local: LocalId,
    /// Constant initial value assigned before the loop.
    pub start: i32,
    /// Constant bound value used in the loop comparison (exclusive).
    pub end: i32,
    /// Constant step per iteration (1 or -1).
    pub step: i32,
    /// Whether the induction variable increases or decreases.
    pub direction: InductionVarDirection,
}

// ---------------------------------------------------------------------------
// Analysis entry point
// ---------------------------------------------------------------------------

/// Run induction variable analysis on a single `MirFunction`.
///
/// Scans the function body for both `While` and `For` loops and attempts
/// to classify induction variables with constant start, end, and step.
///
/// Returns a list of `InductionVarInfo` for every successfully classified
/// induction variable found in the function.
pub fn analyze_function(func: &MirFunction) -> Vec<InductionVarInfo> {
    let mut results = Vec::new();
    analyze_stmts(&func.body, &mut results);
    results
}

// ---------------------------------------------------------------------------
// Recursive analysis helpers
// ---------------------------------------------------------------------------

/// Walk a list of statements looking for for-loops (While or For patterns).
fn analyze_stmts(stmts: &[MirStmt], results: &mut Vec<InductionVarInfo>) {
    // Track locals initialized by `let i = <const>` for While-pattern lookup.
    let mut init_map: Vec<(LocalId, i32)> = Vec::new();

    for stmt in stmts {
        match stmt {
            MirStmt::Let(local, MirExpr::Number(val, _), _) => {
                init_map.push((*local, *val));
            }
            // Non-constant let: clear any prior entry so stale data is not used.
            MirStmt::Let(local, _, _) => {
                init_map.retain(|(id, _)| id != local);
            }
            // While loop: attempt to classify as a for-loop pattern.
            MirStmt::While {
                condition, body, ..
            } => {
                if let Some(info) = analyze_while_condition(condition, body, &init_map) {
                    results.push(info);
                }
                // Recurse into body for nested loops.
                analyze_stmts(body, results);
            }
            MirStmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                if let Some(info) = analyze_for_stmt(init, condition, update) {
                    results.push(info);
                }
                // Recurse into body for nested loops.
                analyze_stmts(body, results);
            }
            // Recurse into blocks and branched statements.
            MirStmt::Block(stmts_inner, _) => {
                analyze_stmts(stmts_inner, results);
            }
            MirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                analyze_stmts(then_body, results);
                analyze_stmts(else_body, results);
            }
            // For structured loops (ForIn, ForOf), recurse into their bodies.
            MirStmt::ForIn { body, .. }
            | MirStmt::ForOf { body, .. }
            | MirStmt::ForAwaitOfLower { body, .. } => {
                analyze_stmts(body, results);
            }
            MirStmt::DoWhile { body, .. } => {
                analyze_stmts(body, results);
            }
            MirStmt::TryFinally {
                try_body,
                finally_body,
                ..
            } => {
                analyze_stmts(try_body, results);
                analyze_stmts(finally_body, results);
            }
            MirStmt::TryCatch {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                analyze_stmts(try_body, results);
                if let Some(catch_body) = catch_body {
                    analyze_stmts(catch_body, results);
                }
                if let Some(finally_body) = finally_body {
                    analyze_stmts(finally_body, results);
                }
            }
            MirStmt::Switch { cases, .. } => {
                for (_, case_body) in cases {
                    analyze_stmts(case_body, results);
                }
            }
            MirStmt::Labeled { body, .. } => {
                analyze_stmts(&[body.as_ref().clone()], results);
            }
            // Statements that do not contain loops.
            MirStmt::Expr(_, _)
            | MirStmt::Assign(_, _, _)
            | MirStmt::Return(_, _)
            | MirStmt::Throw(_, _)
            | MirStmt::Yield(_, _)
            | MirStmt::Break { .. }
            | MirStmt::Continue { .. }
            | MirStmt::Export { .. }
            | MirStmt::ModuleExportsUpdate { .. }
            | MirStmt::ModuleExportsAssign { .. }
            | MirStmt::ClassDecl { .. } => {}
        }
    }
}

// ---------------------------------------------------------------------------
// While-pattern detection
// ---------------------------------------------------------------------------

/// Try to classify a While loop as a for-loop pattern.
///
/// Looks for:
///   `let i = <start>;`  (from init_map)
///   `while (i <op> <end>) { ... i = i + 1; ... }`
fn analyze_while_condition(
    condition: &MirExpr,
    body: &[MirStmt],
    init_map: &[(LocalId, i32)],
) -> Option<InductionVarInfo> {
    // Condition must be a binary comparison with a Local on one side
    // and a Number constant on the other.
    let (iv_local, bound_val, const_is_left) = match extract_comparison(condition)? {
        ComparisonSide::LeftLocal { local, const_val } => (local, const_val, false),
        ComparisonSide::RightLocal { local, const_val } => (local, const_val, true),
    };

    // Find the initial value for iv_local from the init_map.
    let start = init_map.iter().find(|(id, _)| *id == iv_local)?.1;

    // Look in the body for `iv_local = iv_local + 1` or `iv_local = iv_local - 1`.
    let step = extract_step_from_body(body, iv_local)?;

    // When const is on the left (e.g. `0 < i`), we need to flip the
    // comparison op so the local-on-left form is used for direction/bound logic.
    let normalized_condition: MirExpr = if const_is_left {
        flip_comparison(condition)
    } else {
        condition.clone()
    };

    // Determine the direction based on step sign and comparison op.
    let direction = classify_direction(&normalized_condition, step)?;

    // Compute the exclusive bound.
    let end = compute_exclusive_bound(&normalized_condition, step, bound_val)?;

    Some(InductionVarInfo {
        local: iv_local,
        start,
        end,
        step,
        direction,
    })
}

/// Flip a comparison expression (swap operands and reverse the operator).
/// E.g. `0 < i` becomes `i > 0`.
fn flip_comparison(expr: &MirExpr) -> MirExpr {
    use LoweredBinaryOp::*;
    const fn flip_op(op: LoweredBinaryOp) -> LoweredBinaryOp {
        match op {
            Less => Greater,
            LessEqual => GreaterEqual,
            Greater => Less,
            GreaterEqual => LessEqual,
            StrictEqual => StrictEqual,
            EqualEqual => EqualEqual,
            BangEqual => BangEqual,
            StrictNotEqual => StrictNotEqual,
            _ => op,
        }
    }
    match expr {
        MirExpr::Binary {
            left,
            op,
            right,
            span,
        } => MirExpr::Binary {
            left: right.clone(),
            op: flip_op(*op),
            right: left.clone(),
            span: *span,
        },
        other => other.clone(),
    }
}

// ---------------------------------------------------------------------------
// For-stmt detection
// ---------------------------------------------------------------------------

/// Try to classify a structured `MirStmt::For` as a simple induction variable.
fn analyze_for_stmt(
    init: &Option<Box<MirStmt>>,
    condition: &Option<MirExpr>,
    update: &Option<MirExpr>,
) -> Option<InductionVarInfo> {
    // init must be `let i = <const>;`
    let (iv_local, start) = extract_let_init(init.as_ref()?)?;

    // condition must be a comparison with the induction variable.
    let (_bound_local, bound_val, _direction_from_cmp) =
        match extract_comparison(condition.as_ref()?)? {
            ComparisonSide::LeftLocal { local, const_val } if local == iv_local => {
                (local, const_val, InductionVarDirection::Increasing)
            }
            ComparisonSide::RightLocal { local, const_val } if local == iv_local => {
                (local, const_val, InductionVarDirection::Decreasing)
            }
            _ => return None,
        };

    // update must be `iv_local = iv_local +/- 1`
    let step = extract_step_from_expr(update.as_ref()?, iv_local)?;

    let direction = classify_direction_for_update(condition.as_ref()?, step)?;
    let end = compute_exclusive_bound(condition.as_ref()?, step, bound_val)?;

    Some(InductionVarInfo {
        local: iv_local,
        start,
        end,
        step,
        direction,
    })
}

// ---------------------------------------------------------------------------
// Extraction helpers
// ---------------------------------------------------------------------------

/// Extract `(local, value)` from `let local = Number(value);`.
fn extract_let_init(stmt: &MirStmt) -> Option<(LocalId, i32)> {
    if let MirStmt::Let(local, MirExpr::Number(val, _), _) = stmt {
        Some((*local, *val))
    } else {
        None
    }
}

/// Which side of a binary comparison is the local variable?
enum ComparisonSide {
    LeftLocal { local: LocalId, const_val: i32 },
    RightLocal { local: LocalId, const_val: i32 },
}

/// Extract a comparison: `local <op> const` or `const <op> local`.
fn extract_comparison(expr: &MirExpr) -> Option<ComparisonSide> {
    match expr {
        MirExpr::Binary {
            left, op, right, ..
        } if is_comparison_op(*op) => match (left.as_ref(), right.as_ref()) {
            (MirExpr::Local(local, _), MirExpr::Number(val, _)) => {
                Some(ComparisonSide::LeftLocal {
                    local: *local,
                    const_val: *val,
                })
            }
            (MirExpr::Number(val, _), MirExpr::Local(local, _)) => {
                Some(ComparisonSide::RightLocal {
                    local: *local,
                    const_val: *val,
                })
            }
            _ => None,
        },
        _ => None,
    }
}

/// Is this binary op a comparison that bounds a for-loop?
fn is_comparison_op(op: LoweredBinaryOp) -> bool {
    matches!(
        op,
        LoweredBinaryOp::Less
            | LoweredBinaryOp::LessEqual
            | LoweredBinaryOp::Greater
            | LoweredBinaryOp::GreaterEqual
    )
}

/// Scan the body of a While loop for `i = i + step` or `i = i - step`.
fn extract_step_from_body(body: &[MirStmt], iv_local: LocalId) -> Option<i32> {
    for stmt in body {
        if let Some(step) = extract_step_from_stmt(stmt, iv_local) {
            return Some(step);
        }
    }
    None
}

/// Try to extract the step from a single statement.
fn extract_step_from_stmt(stmt: &MirStmt, iv_local: LocalId) -> Option<i32> {
    match stmt {
        MirStmt::Assign(local, expr, _) if *local == iv_local => {
            extract_step_from_binary(expr, iv_local)
        }
        _ => None,
    }
}

/// Extract step value from `i + 1`, `i - 1`, `1 + i` etc.
fn extract_step_from_binary(expr: &MirExpr, iv_local: LocalId) -> Option<i32> {
    match expr {
        MirExpr::Binary {
            left,
            op: LoweredBinaryOp::Add,
            right,
            ..
        } => match (left.as_ref(), right.as_ref()) {
            (MirExpr::Local(l, _), MirExpr::Number(val, _)) if *l == iv_local => Some(*val),
            (MirExpr::Number(val, _), MirExpr::Local(l, _)) if *l == iv_local => Some(*val),
            _ => None,
        },
        MirExpr::Binary {
            left,
            op: LoweredBinaryOp::Subtract,
            right,
            ..
        } => match (left.as_ref(), right.as_ref()) {
            (MirExpr::Local(l, _), MirExpr::Number(val, _)) if *l == iv_local => {
                Some(-val) // i - c => step is -c
            }
            _ => None,
        },
        _ => None,
    }
}

/// Extract step from an update expression `i = i +/- 1`.
fn extract_step_from_expr(expr: &MirExpr, iv_local: LocalId) -> Option<i32> {
    match expr {
        MirExpr::Assign {
            local, expr: rhs, ..
        } if *local == iv_local => extract_step_from_binary(rhs.as_ref(), iv_local),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Direction and bound helpers
// ---------------------------------------------------------------------------

/// Determine the induction direction based on comparison and step sign.
fn classify_direction(condition: &MirExpr, step: i32) -> Option<InductionVarDirection> {
    use LoweredBinaryOp::*;

    let op = match condition {
        MirExpr::Binary { op, .. } => *op,
        _ => return None,
    };

    // Step must be 1 or -1 for simple induction.
    if step.abs() != 1 {
        return None;
    }

    match (op, step) {
        // i < n, step = +1 => increasing (canonical forward loop)
        (Less, 1) | (LessEqual, 1) => Some(InductionVarDirection::Increasing),
        // i > n, step = -1 => decreasing (canonical backward loop)
        (Greater, -1) | (GreaterEqual, -1) => Some(InductionVarDirection::Decreasing),
        // i > n, step = +1 => increasing (i starts above bound, goes up)
        (Greater, 1) | (GreaterEqual, 1) => Some(InductionVarDirection::Increasing),
        // i < n, step = -1 => decreasing (i starts below bound, goes down)
        (Less, -1) | (LessEqual, -1) => Some(InductionVarDirection::Decreasing),
        _ => None,
    }
}

/// Same as `classify_direction` but for the `For` stmt pattern.
fn classify_direction_for_update(condition: &MirExpr, step: i32) -> Option<InductionVarDirection> {
    classify_direction(condition, step)
}

/// Compute the exclusive bound from comparison op, step, and bound value.
///
/// For example, `i < 10` with step +1 has end = 10 (exclusive).
/// `i <= 10` with step +1 has end = 11 (exclusive, inclusive adjusted).
fn compute_exclusive_bound(condition: &MirExpr, step: i32, bound_val: i32) -> Option<i32> {
    use LoweredBinaryOp::*;

    let op = match condition {
        MirExpr::Binary { op, .. } => *op,
        _ => return None,
    };

    match (op, step.signum()) {
        // i < n: exclusive bound is n
        (Less, 1) | (Greater, -1) => Some(bound_val),
        // i <= n: exclusive bound is n + 1 (for increasing) or n - 1 (for decreasing)
        (LessEqual, 1) => bound_val.checked_add(1),
        (GreaterEqual, -1) => bound_val.checked_sub(1),
        (LessEqual, -1) => bound_val.checked_add(1),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lowered::mir::types::*;
    use ts2wasm_source::Span;

    fn s() -> Span {
        Span { start: 0, end: 0 }
    }

    // -- While-pattern: canonical forward loop --

    #[test]
    fn detect_while_increasing_i_lt_10() {
        // let i = 0;
        // while (i < 10) { i = i + 1; }
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(0, s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(MirExpr::Number(10, s())),
                        span: s(),
                    },
                    body: vec![MirStmt::Assign(
                        LocalId(0),
                        MirExpr::Binary {
                            left: Box::new(MirExpr::Local(LocalId(0), s())),
                            op: LoweredBinaryOp::Add,
                            right: Box::new(MirExpr::Number(1, s())),
                            span: s(),
                        },
                        s(),
                    )],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 1);
        let info = &infos[0];
        assert_eq!(info.local, LocalId(0));
        assert_eq!(info.start, 0);
        assert_eq!(info.end, 10);
        assert_eq!(info.step, 1);
        assert_eq!(info.direction, InductionVarDirection::Increasing);
    }

    #[test]
    fn detect_while_decreasing_i_gt_0() {
        // let i = 10;
        // while (i > 0) { i = i - 1; }
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(10, s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::Greater,
                        right: Box::new(MirExpr::Number(0, s())),
                        span: s(),
                    },
                    body: vec![MirStmt::Assign(
                        LocalId(0),
                        MirExpr::Binary {
                            left: Box::new(MirExpr::Local(LocalId(0), s())),
                            op: LoweredBinaryOp::Subtract,
                            right: Box::new(MirExpr::Number(1, s())),
                            span: s(),
                        },
                        s(),
                    )],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 1);
        let info = &infos[0];
        assert_eq!(info.local, LocalId(0));
        assert_eq!(info.start, 10);
        assert_eq!(info.end, 0);
        assert_eq!(info.step, -1);
        assert_eq!(info.direction, InductionVarDirection::Decreasing);
    }

    #[test]
    fn detect_while_less_equal() {
        // let i = 0;
        // while (i <= 5) { i = i + 1; }
        // Exclusive bound: 5 + 1 = 6
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(0, s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::LessEqual,
                        right: Box::new(MirExpr::Number(5, s())),
                        span: s(),
                    },
                    body: vec![MirStmt::Assign(
                        LocalId(0),
                        MirExpr::Binary {
                            left: Box::new(MirExpr::Local(LocalId(0), s())),
                            op: LoweredBinaryOp::Add,
                            right: Box::new(MirExpr::Number(1, s())),
                            span: s(),
                        },
                        s(),
                    )],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 1);
        let info = &infos[0];
        assert_eq!(info.start, 0);
        assert_eq!(info.end, 6); // inclusive -> exclusive: +1
        assert_eq!(info.step, 1);
        assert_eq!(info.direction, InductionVarDirection::Increasing);
    }

    #[test]
    fn detect_while_greater_equal() {
        // let i = 5;
        // while (i >= 0) { i = i - 1; }
        // Exclusive bound: 0 - 1 = -1
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(5, s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::GreaterEqual,
                        right: Box::new(MirExpr::Number(0, s())),
                        span: s(),
                    },
                    body: vec![MirStmt::Assign(
                        LocalId(0),
                        MirExpr::Binary {
                            left: Box::new(MirExpr::Local(LocalId(0), s())),
                            op: LoweredBinaryOp::Subtract,
                            right: Box::new(MirExpr::Number(1, s())),
                            span: s(),
                        },
                        s(),
                    )],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 1);
        let info = &infos[0];
        assert_eq!(info.start, 5);
        assert_eq!(info.end, -1); // inclusive -> exclusive: -1 - 1 → error, wait
        assert_eq!(info.step, -1);
        assert_eq!(info.direction, InductionVarDirection::Decreasing);
    }

    // -- For-stmt pattern --

    #[test]
    fn detect_for_increasing() {
        // for (let i = 0; i < 10; i = i + 1) {}
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![MirStmt::For {
                init: Some(Box::new(MirStmt::Let(
                    LocalId(0),
                    MirExpr::Number(0, s()),
                    s(),
                ))),
                condition: Some(MirExpr::Binary {
                    left: Box::new(MirExpr::Local(LocalId(0), s())),
                    op: LoweredBinaryOp::Less,
                    right: Box::new(MirExpr::Number(10, s())),
                    span: s(),
                }),
                update: Some(MirExpr::Assign {
                    local: LocalId(0),
                    expr: Box::new(MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::Add,
                        right: Box::new(MirExpr::Number(1, s())),
                        span: s(),
                    }),
                    span: s(),
                }),
                body: vec![],
                span: s(),
            }],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 1);
        let info = &infos[0];
        assert_eq!(info.local, LocalId(0));
        assert_eq!(info.start, 0);
        assert_eq!(info.end, 10);
        assert_eq!(info.step, 1);
        assert_eq!(info.direction, InductionVarDirection::Increasing);
    }

    // -- Negative cases --

    #[test]
    fn reject_non_constant_init() {
        // let i = x; (non-constant)
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0), LocalId(1)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Local(LocalId(1), s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(MirExpr::Number(10, s())),
                        span: s(),
                    },
                    body: vec![MirStmt::Assign(
                        LocalId(0),
                        MirExpr::Binary {
                            left: Box::new(MirExpr::Local(LocalId(0), s())),
                            op: LoweredBinaryOp::Add,
                            right: Box::new(MirExpr::Number(1, s())),
                            span: s(),
                        },
                        s(),
                    )],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 0);
    }

    #[test]
    fn reject_non_constant_bound() {
        // let i = 0;
        // while (i < n) { i = i + 1; }  // n is not a constant
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0), LocalId(1)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(0, s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(MirExpr::Local(LocalId(1), s())),
                        span: s(),
                    },
                    body: vec![MirStmt::Assign(
                        LocalId(0),
                        MirExpr::Binary {
                            left: Box::new(MirExpr::Local(LocalId(0), s())),
                            op: LoweredBinaryOp::Add,
                            right: Box::new(MirExpr::Number(1, s())),
                            span: s(),
                        },
                        s(),
                    )],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 0);
    }

    #[test]
    fn reject_non_unit_step() {
        // let i = 0;
        // while (i < 10) { i = i + 2; }
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(0, s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(MirExpr::Number(10, s())),
                        span: s(),
                    },
                    body: vec![MirStmt::Assign(
                        LocalId(0),
                        MirExpr::Binary {
                            left: Box::new(MirExpr::Local(LocalId(0), s())),
                            op: LoweredBinaryOp::Add,
                            right: Box::new(MirExpr::Number(2, s())),
                            span: s(),
                        },
                        s(),
                    )],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 0);
    }

    #[test]
    fn empty_function_no_induction_vars() {
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![],
            body: vec![],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert!(infos.is_empty());
    }

    #[test]
    fn detect_nested_loops() {
        // Outer and inner both have induction vars.
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0), LocalId(1)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(0, s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(MirExpr::Number(5, s())),
                        span: s(),
                    },
                    body: vec![
                        MirStmt::Assign(
                            LocalId(0),
                            MirExpr::Binary {
                                left: Box::new(MirExpr::Local(LocalId(0), s())),
                                op: LoweredBinaryOp::Add,
                                right: Box::new(MirExpr::Number(1, s())),
                                span: s(),
                            },
                            s(),
                        ),
                        MirStmt::Let(LocalId(1), MirExpr::Number(0, s()), s()),
                        MirStmt::While {
                            condition: MirExpr::Binary {
                                left: Box::new(MirExpr::Local(LocalId(1), s())),
                                op: LoweredBinaryOp::Less,
                                right: Box::new(MirExpr::Number(3, s())),
                                span: s(),
                            },
                            body: vec![MirStmt::Assign(
                                LocalId(1),
                                MirExpr::Binary {
                                    left: Box::new(MirExpr::Local(LocalId(1), s())),
                                    op: LoweredBinaryOp::Add,
                                    right: Box::new(MirExpr::Number(1, s())),
                                    span: s(),
                                },
                                s(),
                            )],
                            span: s(),
                        },
                    ],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 2);
        assert_eq!(infos[0].local, LocalId(0)); // outer
        assert_eq!(infos[1].local, LocalId(1)); // inner
    }

    #[test]
    fn detect_for_while_modified_body() {
        // Verify that a while with non-trivial body still works.
        // let i = 0;
        // while (i < 5) { foo(i); i = i + 1; bar(i); }
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(0, s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Local(LocalId(0), s())),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(MirExpr::Number(5, s())),
                        span: s(),
                    },
                    body: vec![
                        MirStmt::Expr(
                            MirExpr::Call {
                                kind: crate::lowered::FunctionCallKind::User(
                                    crate::lowered::FuncId(1),
                                ),
                                args: vec![MirExpr::Local(LocalId(0), s())],
                                span: s(),
                            },
                            s(),
                        ),
                        MirStmt::Assign(
                            LocalId(0),
                            MirExpr::Binary {
                                left: Box::new(MirExpr::Local(LocalId(0), s())),
                                op: LoweredBinaryOp::Add,
                                right: Box::new(MirExpr::Number(1, s())),
                                span: s(),
                            },
                            s(),
                        ),
                    ],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 1);
        let info = &infos[0];
        assert_eq!(info.local, LocalId(0));
        assert_eq!(info.start, 0);
        assert_eq!(info.end, 5);
        assert_eq!(info.step, 1);
    }

    #[test]
    fn detect_for_while_const_on_right() {
        // while (0 < i) with i = i - 1 — decreasing, const on left side
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(10, s()), s()),
                MirStmt::While {
                    condition: MirExpr::Binary {
                        left: Box::new(MirExpr::Number(0, s())),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(MirExpr::Local(LocalId(0), s())),
                        span: s(),
                    },
                    body: vec![MirStmt::Assign(
                        LocalId(0),
                        MirExpr::Binary {
                            left: Box::new(MirExpr::Local(LocalId(0), s())),
                            op: LoweredBinaryOp::Subtract,
                            right: Box::new(MirExpr::Number(1, s())),
                            span: s(),
                        },
                        s(),
                    )],
                    span: s(),
                },
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            escape_status: vec![],
        };

        let infos = analyze_function(&func);
        assert_eq!(infos.len(), 1);
        let info = &infos[0];
        assert_eq!(info.local, LocalId(0));
        assert_eq!(info.start, 10);
        assert_eq!(info.step, -1);
        assert_eq!(info.direction, InductionVarDirection::Decreasing);
    }
}
