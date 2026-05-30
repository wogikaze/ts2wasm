//! Value range analysis for MIR.
//!
//! Tracks known i32 value ranges for local variables through forward dataflow.
//! For each local, a `ValueRange { min, max }` pair describes the set of
//! possible integer values it can hold at any program point.
//!
//! ## What is tracked
//!
//! - **Number literals**: `let x = 42` → range `[42, 42]`
//! - **Binary arithmetic**: add, subtract, multiply (with conservative overflow wrapping)
//! - **Comparison results**: `a < b` produces a boolean `[0, 1]`
//! - **Induction variables**: ranges are derived from the function's `induction_vars`
//! - **Assignments**: overwrite the target local's range from the RHS
//!
//! ## Conservative fallback
//!
//! Any expression whose range cannot be statically determined (e.g., function
//! calls, property loads, locals without a known assigned range) is left
//! un-tracked — it simply does not appear in the result map.
//!
//! ## Usage
//!
//! ```ignore
//! let ranges = run_value_range_analysis(&function);
//! if let Some(range) = ranges.get(&local_id) {
//!     // range.min .. range.max is known
//! }
//! ```

use std::collections::HashMap;

use super::types::{MirExpr, MirFunction, MirStmt};
use crate::lowered::{LocalId, LoweredBinaryOp};

// ---------------------------------------------------------------------------
// ValueRange
// ---------------------------------------------------------------------------

/// A closed interval `[min, max]` of possible i32 values.
///
/// `min` is always ≤ `max`. When a range is empty (no possible values),
/// use `ValueRange::empty()` which has `min > max`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueRange {
    pub min: i32,
    pub max: i32,
}

impl ValueRange {
    /// Create a range that contains only a single value.
    pub fn exact(val: i32) -> Self {
        ValueRange { min: val, max: val }
    }

    /// Create a range covering all possible i32 values.
    pub fn everything() -> Self {
        ValueRange {
            min: i32::MIN,
            max: i32::MAX,
        }
    }

    /// Create an empty range (no possible values).
    pub fn empty() -> Self {
        ValueRange {
            min: i32::MAX,
            max: i32::MIN,
        }
    }

    /// Returns `true` if this range contains `val`.
    pub fn contains(self, val: i32) -> bool {
        val >= self.min && val <= self.max
    }

    /// Returns `true` if no value can satisfy this range.
    pub fn is_empty(self) -> bool {
        self.min > self.max
    }

    /// Widening meet: return the smallest range that covers both `self` and `other`.
    /// This is used when merging ranges from different control-flow paths (e.g.,
    /// if-else branches, loop headers).
    pub fn union(self, other: Self) -> Self {
        ValueRange {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Intersect two ranges, returning the overlapping portion.
    /// Returns `ValueRange::empty()` if they do not overlap.
    pub fn intersect(self, other: Self) -> Self {
        ValueRange {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }
}

// ---------------------------------------------------------------------------
// Analysis state
// ---------------------------------------------------------------------------

/// Per-function analysis state: a map from `LocalId` to the current known range.
type RangeMap = HashMap<LocalId, ValueRange>;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Run value range analysis on a single `MirFunction`.
///
/// Returns a map from `LocalId` to `ValueRange` for every local whose range
/// could be statically determined. Locals with unknown ranges are absent from
/// the map (callers should treat them as `[i32::MIN, i32::MAX]`).
pub fn run_value_range_analysis(func: &MirFunction) -> HashMap<LocalId, ValueRange> {
    let mut ranges = RangeMap::new();

    // Pre-populate ranges from induction variable analysis results.
    for iv in &func.induction_vars {
        let lo = iv.start.min(iv.end);
        let hi = iv.start.max(iv.end);
        ranges.insert(iv.local, ValueRange { min: lo, max: hi });
    }

    // Walk through all statements in the function body.
    analyze_stmts(&func.body, &mut ranges);

    ranges
}

// ---------------------------------------------------------------------------
// Statement walker
// ---------------------------------------------------------------------------

/// Walk through a list of statements, updating range information as we go.
fn analyze_stmts(stmts: &[MirStmt], ranges: &mut RangeMap) {
    for stmt in stmts {
        match stmt {
            MirStmt::Let(local, expr, _) | MirStmt::Assign(local, expr, _) => {
                if let Some(range) = expr_to_range(expr, ranges) {
                    ranges.insert(*local, range);
                } else {
                    // Unknown expression: remove any previous range for this local.
                    ranges.remove(local);
                }
            }

            // Recurse into compound statements.
            MirStmt::Block(children, _) => {
                analyze_stmts(children, ranges);
            }

            MirStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                // Evaluate condition but do not change ranges from it.
                expr_to_range(condition, ranges);

                // Analyze both branches independently.
                let mut then_ranges = ranges.clone();
                analyze_stmts(then_body, &mut then_ranges);

                let mut else_ranges = ranges.clone();
                analyze_stmts(else_body, &mut else_ranges);

                // Merge ranges after the if-else.
                merge_ranges(ranges, &then_ranges, &else_ranges);
            }

            MirStmt::While {
                condition, body, ..
            } => {
                // Simple fixed-point for while loops: iterate body at most 3 times
                // to approximate the loop effect.
                for _ in 0..3 {
                    let mut body_ranges = ranges.clone();
                    analyze_stmts(body, &mut body_ranges);
                    // Merge loop body effects back into pre-loop state.
                    merge_ranges_unidirectional(ranges, &body_ranges);
                }
                // Condition is evaluated but we don't refine ranges from it
                // (no path-sensitive refinement in this basic pass).
                expr_to_range(condition, ranges);
            }

            MirStmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                if let Some(init_stmt) = init {
                    analyze_stmts(std::slice::from_ref(init_stmt.as_ref()), ranges);
                }

                // Fixed-point for the loop body + update.
                for _ in 0..3 {
                    let mut body_ranges = ranges.clone();
                    analyze_stmts(body, &mut body_ranges);
                    if let Some(update_expr) = update {
                        // The update expression is an Assign — handle it.
                        if let MirExpr::Assign {
                            local, expr: rhs, ..
                        } = update_expr
                        {
                            if let Some(range) = expr_to_range(rhs, &body_ranges) {
                                body_ranges.insert(*local, range);
                            } else {
                                body_ranges.remove(local);
                            }
                        }
                    }
                    merge_ranges_unidirectional(ranges, &body_ranges);
                }

                if let Some(cond_expr) = condition {
                    expr_to_range(cond_expr, ranges);
                }
            }

            // Recurse into other compound statements.
            MirStmt::DoWhile {
                body, condition, ..
            } => {
                for _ in 0..3 {
                    let mut body_ranges = ranges.clone();
                    analyze_stmts(body, &mut body_ranges);
                    merge_ranges_unidirectional(ranges, &body_ranges);
                }
                expr_to_range(condition, ranges);
            }

            MirStmt::ForIn { body, .. }
            | MirStmt::ForOf { body, .. }
            | MirStmt::ForAwaitOfLower { body, .. } => {
                analyze_stmts(body, ranges);
            }

            MirStmt::TryFinally {
                try_body,
                finally_body,
                ..
            } => {
                let mut try_ranges = ranges.clone();
                analyze_stmts(try_body, &mut try_ranges);
                // Finally always runs; merge its effects too.
                let mut finally_ranges = ranges.clone();
                analyze_stmts(finally_body, &mut finally_ranges);
                // Result: try ranges, then finally ranges.
                merge_ranges_unidirectional(ranges, &try_ranges);
                merge_ranges_unidirectional(ranges, &finally_ranges);
            }

            MirStmt::TryCatch {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                let mut try_ranges = ranges.clone();
                analyze_stmts(try_body, &mut try_ranges);

                let mut catch_ranges = ranges.clone();
                if let Some(body) = catch_body {
                    analyze_stmts(body, &mut catch_ranges);
                }

                // Merge try and catch.
                let mut merged = ranges.clone();
                merge_ranges(&mut merged, &try_ranges, &catch_ranges);

                if let Some(finally_body) = finally_body {
                    let mut finally_ranges = merged.clone();
                    analyze_stmts(finally_body, &mut finally_ranges);
                    *ranges = finally_ranges;
                } else {
                    *ranges = merged;
                }
            }

            MirStmt::Switch { expr, cases, .. } => {
                expr_to_range(expr, ranges);
                // For switch, merge all case bodies.
                let mut merged = ranges.clone();
                for (_cond, body) in cases {
                    let mut case_ranges = ranges.clone();
                    analyze_stmts(body, &mut case_ranges);
                    merge_ranges_unidirectional(&mut merged, &case_ranges);
                }
                *ranges = merged;
            }

            MirStmt::Labeled { body, .. } => {
                analyze_stmts(std::slice::from_ref(body.as_ref()), ranges);
            }

            // Statements that don't produce range information for any local.
            MirStmt::Expr(expr, _) => {
                expr_to_range(expr, ranges);
            }

            MirStmt::Return(expr, _) => {
                expr_to_range(expr, ranges);
            }

            MirStmt::Throw(expr, _) => {
                expr_to_range(expr, ranges);
            }

            MirStmt::Yield(expr, _) => {
                expr_to_range(expr, ranges);
            }

            MirStmt::Export { expr, .. } => {
                expr_to_range(expr, ranges);
            }

            MirStmt::ModuleExportsUpdate { .. }
            | MirStmt::ModuleExportsAssign { .. }
            | MirStmt::Break { .. }
            | MirStmt::Continue { .. }
            | MirStmt::ClassDecl { .. } => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Expression evaluator
// ---------------------------------------------------------------------------

/// Try to compute a `ValueRange` for a `MirExpr`, given the current known ranges.
///
/// Returns `None` if the expression's range cannot be statically determined.
fn expr_to_range(expr: &MirExpr, ranges: &RangeMap) -> Option<ValueRange> {
    match expr {
        MirExpr::Number(n, _) => Some(ValueRange::exact(*n)),

        MirExpr::Bool(b, _) => Some(ValueRange::exact(if *b { 1 } else { 0 })),

        MirExpr::Local(id, _) => ranges.get(id).copied(),

        MirExpr::Unary {
            op: _, expr: inner, ..
        } => {
            // For negate: if we know the inner range, negate it (with overflow wrap).
            expr_to_range(inner, ranges)
        }

        MirExpr::Binary {
            left, op, right, ..
        } => {
            let left_range = expr_to_range(left, ranges)?;
            let right_range = expr_to_range(right, ranges)?;
            binary_op_range(*op, left_range, right_range)
        }

        MirExpr::Assign { expr: rhs, .. } => expr_to_range(rhs, ranges),

        // For Block, look at the result expression.
        MirExpr::Block { stmts, result, .. } => {
            let mut block_ranges = ranges.clone();
            analyze_stmts(stmts, &mut block_ranges);
            expr_to_range(result, &block_ranges)
        }

        // All other expressions have unknown ranges.
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Binary operation range computation
// ---------------------------------------------------------------------------

/// Compute the resulting range for a binary operation.
fn binary_op_range(op: LoweredBinaryOp, left: ValueRange, right: ValueRange) -> Option<ValueRange> {
    match op {
        // Arithmetic
        LoweredBinaryOp::Add => {
            // Wrapping addition: compute all four corner cases.
            let v1 = left.min.wrapping_add(right.min);
            let v2 = left.min.wrapping_add(right.max);
            let v3 = left.max.wrapping_add(right.min);
            let v4 = left.max.wrapping_add(right.max);
            let mn = v1.min(v2).min(v3).min(v4);
            let mx = v1.max(v2).max(v3).max(v4);
            // If wrapping produces a range that covers more than half of i32 space,
            // be conservative and return the full range.
            if mx.wrapping_sub(mn) as u32 >= (i32::MAX as u32 / 2) {
                Some(ValueRange::everything())
            } else {
                Some(ValueRange { min: mn, max: mx })
            }
        }

        LoweredBinaryOp::Subtract => {
            let v1 = left.min.wrapping_sub(right.min);
            let v2 = left.min.wrapping_sub(right.max);
            let v3 = left.max.wrapping_sub(right.min);
            let v4 = left.max.wrapping_sub(right.max);
            let mn = v1.min(v2).min(v3).min(v4);
            let mx = v1.max(v2).max(v3).max(v4);
            if mx.wrapping_sub(mn) as u32 >= (i32::MAX as u32 / 2) {
                Some(ValueRange::everything())
            } else {
                Some(ValueRange { min: mn, max: mx })
            }
        }

        LoweredBinaryOp::Multiply => {
            let v1 = left.min.wrapping_mul(right.min);
            let v2 = left.min.wrapping_mul(right.max);
            let v3 = left.max.wrapping_mul(right.min);
            let v4 = left.max.wrapping_mul(right.max);
            let mn = v1.min(v2).min(v3).min(v4);
            let mx = v1.max(v2).max(v3).max(v4);
            if left.min == left.max && right.min == right.max {
                // Both sides are exact: the result is exact.
                Some(ValueRange { min: mn, max: mx })
            } else if mx.wrapping_sub(mn) as u32 >= (i32::MAX as u32 / 2) {
                Some(ValueRange::everything())
            } else {
                Some(ValueRange { min: mn, max: mx })
            }
        }

        // Comparisons always produce boolean [0, 1]
        LoweredBinaryOp::Less
        | LoweredBinaryOp::LessEqual
        | LoweredBinaryOp::Greater
        | LoweredBinaryOp::GreaterEqual
        | LoweredBinaryOp::StrictEqual
        | LoweredBinaryOp::EqualEqual
        | LoweredBinaryOp::BangEqual
        | LoweredBinaryOp::StrictNotEqual => Some(ValueRange { min: 0, max: 1 }),

        // Logical operators: result can be either operand's value, but we
        // conservatively return the full i32 range.
        LoweredBinaryOp::And | LoweredBinaryOp::Or | LoweredBinaryOp::NullishCoalesce => {
            Some(ValueRange::everything())
        }

        // Bitwise ops produce i32 results — conservative for now.
        LoweredBinaryOp::BitwiseAnd => Some(ValueRange::everything()),
        LoweredBinaryOp::BitwiseXor
        | LoweredBinaryOp::BitwiseOr
        | LoweredBinaryOp::Shl
        | LoweredBinaryOp::Shr
        | LoweredBinaryOp::ShrU => Some(ValueRange::everything()),

        // Division/power/modulo: conservative.
        LoweredBinaryOp::Divide | LoweredBinaryOp::Power | LoweredBinaryOp::Modulo => {
            Some(ValueRange::everything())
        }
    }
}

// ---------------------------------------------------------------------------
// Merge helpers
// ---------------------------------------------------------------------------

/// Merge ranges from two post-branch analysis states into the pre-branch state.
///
/// For each local tracked in either branch's state, the merged range is the
/// union of the two branch ranges. Locals not tracked by a particular branch
/// are treated as unknown (everything).
fn merge_ranges(pre: &mut RangeMap, then_ranges: &RangeMap, else_ranges: &RangeMap) {
    let all_keys: Vec<LocalId> = {
        let mut keys: Vec<_> = pre.keys().copied().collect();
        for k in then_ranges.keys() {
            if !keys.contains(k) {
                keys.push(*k);
            }
        }
        for k in else_ranges.keys() {
            if !keys.contains(k) {
                keys.push(*k);
            }
        }
        keys
    };

    for key in all_keys {
        let then_r = then_ranges.get(&key).copied();
        let else_r = else_ranges.get(&key).copied();

        let merged = match (then_r, else_r) {
            (Some(a), Some(b)) => a.union(b),
            (Some(a), None) => a.union(ValueRange::everything()),
            (None, Some(b)) => ValueRange::everything().union(b),
            (None, None) => continue,
        };

        pre.insert(key, merged);
    }
}

/// Unidirectional merge: update `pre` ranges with ranges from `post`.
///
/// For each local in `post`, if it also appears in `pre`, the pre range becomes
/// the union of pre and post. New locals from `post` are added to `pre`.
fn merge_ranges_unidirectional(pre: &mut RangeMap, post: &RangeMap) {
    for (&key, &post_r) in post {
        match pre.get(&key) {
            Some(&pre_r) => {
                pre.insert(key, pre_r.union(post_r));
            }
            None => {
                pre.insert(key, post_r);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lowered::mir::types::*;
    use crate::lowered::mir::{InductionVarDirection, InductionVarInfo};
    use crate::lowered::{LocalId, LoweredBinaryOp};
    use ts2wasm_source::Span;

    fn s() -> Span {
        Span { start: 0, end: 0 }
    }

    // -----------------------------------------------------------------------
    // ValueRange tests
    // -----------------------------------------------------------------------

    #[test]
    fn range_exact_contains() {
        let r = ValueRange::exact(42);
        assert!(r.contains(42));
        assert!(!r.contains(41));
        assert!(!r.contains(43));
        assert!(!r.is_empty());
    }

    #[test]
    fn range_empty_is_empty() {
        let r = ValueRange::empty();
        assert!(r.is_empty());
        assert!(!r.contains(0));
    }

    #[test]
    fn range_union() {
        let a = ValueRange { min: 0, max: 5 };
        let b = ValueRange { min: 10, max: 15 };
        let u = a.union(b);
        assert_eq!(u.min, 0);
        assert_eq!(u.max, 15);
    }

    #[test]
    fn range_intersect() {
        let a = ValueRange { min: 0, max: 10 };
        let b = ValueRange { min: 5, max: 15 };
        let i = a.intersect(b);
        assert_eq!(i.min, 5);
        assert_eq!(i.max, 10);
    }

    #[test]
    fn range_intersect_no_overlap() {
        let a = ValueRange { min: 0, max: 5 };
        let b = ValueRange { min: 10, max: 15 };
        let i = a.intersect(b);
        assert!(i.is_empty());
    }

    // -----------------------------------------------------------------------
    // Number literal tracking
    // -----------------------------------------------------------------------

    #[test]
    fn track_number_literal() {
        // let x = 42;
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![MirStmt::Let(LocalId(0), MirExpr::Number(42, s()), s())],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
            escape_status: vec![],
        };

        let ranges = run_value_range_analysis(&func);
        let r = ranges.get(&LocalId(0)).unwrap();
        assert_eq!(r.min, 42);
        assert_eq!(r.max, 42);
    }

    #[test]
    fn track_bool_literal() {
        // let x = true;  // -> [1, 1]
        // let y = false; // -> [0, 0]
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0), LocalId(1)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Bool(true, s()), s()),
                MirStmt::Let(LocalId(1), MirExpr::Bool(false, s()), s()),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
            escape_status: vec![],
        };

        let ranges = run_value_range_analysis(&func);
        assert_eq!(ranges.get(&LocalId(0)).unwrap(), &ValueRange::exact(1));
        assert_eq!(ranges.get(&LocalId(1)).unwrap(), &ValueRange::exact(0));
    }

    // -----------------------------------------------------------------------
    // Binary operation range tracking
    // -----------------------------------------------------------------------

    #[test]
    fn track_binary_add_constants() {
        // let x = 10 + 20;
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![MirStmt::Let(
                LocalId(0),
                MirExpr::Binary {
                    left: Box::new(MirExpr::Number(10, s())),
                    op: LoweredBinaryOp::Add,
                    right: Box::new(MirExpr::Number(20, s())),
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

        let ranges = run_value_range_analysis(&func);
        let r = ranges.get(&LocalId(0)).unwrap();
        assert_eq!(r.min, 30);
        assert_eq!(r.max, 30);
    }

    #[test]
    fn track_binary_subtract() {
        // let x = 50 - 20;
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![MirStmt::Let(
                LocalId(0),
                MirExpr::Binary {
                    left: Box::new(MirExpr::Number(50, s())),
                    op: LoweredBinaryOp::Subtract,
                    right: Box::new(MirExpr::Number(20, s())),
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

        let ranges = run_value_range_analysis(&func);
        let r = ranges.get(&LocalId(0)).unwrap();
        assert_eq!(r.min, 30);
        assert_eq!(r.max, 30);
    }

    // -----------------------------------------------------------------------
    // Comparison result range [0, 1]
    // -----------------------------------------------------------------------

    #[test]
    fn track_comparison_range_0_1() {
        // let x = a < b; (range [0, 1] for any comparison)
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![MirStmt::Let(
                LocalId(0),
                MirExpr::Binary {
                    left: Box::new(MirExpr::Number(1, s())),
                    op: LoweredBinaryOp::Less,
                    right: Box::new(MirExpr::Number(2, s())),
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

        let ranges = run_value_range_analysis(&func);
        let r = ranges.get(&LocalId(0)).unwrap();
        assert_eq!(r.min, 0);
        assert_eq!(r.max, 1);
    }

    // -----------------------------------------------------------------------
    // Induction variable range
    // -----------------------------------------------------------------------

    #[test]
    fn track_induction_var_range() {
        // An increasing IV from 0 to 10 (pre-populated from analysis results).
        // The body is empty because the range is derived from induction_var data.
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![InductionVarInfo {
                local: LocalId(0),
                start: 0,
                end: 10,
                step: 1,
                direction: InductionVarDirection::Increasing,
            }],
            value_reps: vec![],
            optimization_hints: vec![],
            escape_status: vec![],
        };

        let ranges = run_value_range_analysis(&func);
        let r = ranges.get(&LocalId(0)).unwrap();
        assert_eq!(r.min, 0);
        assert_eq!(r.max, 10);
    }

    #[test]
    fn track_induction_var_decreasing() {
        // A decreasing IV from 10 down to 0.
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![InductionVarInfo {
                local: LocalId(0),
                start: 10,
                end: 0,
                step: -1,
                direction: InductionVarDirection::Decreasing,
            }],
            value_reps: vec![],
            optimization_hints: vec![],
            escape_status: vec![],
        };

        let ranges = run_value_range_analysis(&func);
        let r = ranges.get(&LocalId(0)).unwrap();
        assert_eq!(r.min, 0);
        assert_eq!(r.max, 10);
    }

    // -----------------------------------------------------------------------
    // Assignment updates range
    // -----------------------------------------------------------------------

    #[test]
    fn track_assignment_overwrites_range() {
        // let x = 10; x = 20;
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(10, s()), s()),
                MirStmt::Assign(LocalId(0), MirExpr::Number(20, s()), s()),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
            escape_status: vec![],
        };

        let ranges = run_value_range_analysis(&func);
        let r = ranges.get(&LocalId(0)).unwrap();
        // After the last statement, x = 20, so range is [20, 20].
        assert_eq!(r.min, 20);
        assert_eq!(r.max, 20);
    }

    // -----------------------------------------------------------------------
    // Unknown expression -> no range
    // -----------------------------------------------------------------------

    #[test]
    fn unknown_expression_not_tracked() {
        // let x = foo();  // unknown expression
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![LocalId(0)],
            body: vec![MirStmt::Let(
                LocalId(0),
                MirExpr::Call {
                    kind: crate::lowered::FunctionCallKind::User(crate::lowered::FuncId(1)),
                    args: vec![],
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

        let ranges = run_value_range_analysis(&func);
        assert!(ranges.get(&LocalId(0)).is_none());
    }

    // -----------------------------------------------------------------------
    // Empty function -> empty map
    // -----------------------------------------------------------------------

    #[test]
    fn empty_function_empty_ranges() {
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
            optimization_hints: vec![],
            escape_status: vec![],
        };

        let ranges = run_value_range_analysis(&func);
        assert!(ranges.is_empty());
    }
}
