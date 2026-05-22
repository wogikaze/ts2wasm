//! Escape analysis for MIR — Phase A: classification rules.
//!
//! Determines, for each local variable, whether the value it holds
//! (an object, array, or closure) can be referenced from outside of the
//! current function scope. A local that does not escape is a candidate
//! for scalar replacement (stack allocation / field decomposition).
//!
//! ## Escape rules (Phase A)
//!
//! A local is classified as `Escaped` if it appears in any of these
//! positions:
//!
//! 1. **RuntimeCall argument** — the callee may store the reference.
//! 2. **Stored as an object property** — `PropertySet` / `PropertySetDynamic`
//!    value position.
//! 3. **Returned** — the reference leaves the function.
//! 4. **Exported** — bound to an exported symbol at the top level.
//!
//! Locals that are only created, read, and written within their function
//! (and never appear in the positions above) are classified as
//! `NotEscaped`.

#[cfg(test)]
use super::types::MirFunction;
use super::types::{EscapeStatus, MirArraySlot, MirExpr, MirProgram, MirStmt};
use crate::lowered::LocalId;
#[cfg(test)]
use ts2wasm_source::Span;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Run escape analysis on the entire MIR program.
///
/// After calling this function, every entry in `escape_status` (both
/// per-function and top-level) will be `Some(Escaped)` or
/// `Some(NotEscaped)` — never `None`.
pub fn analyze_escape(program: &mut MirProgram) {
    // --- Top level ---
    for stmt in &program.top_level_statements {
        mark_stmt(stmt, &mut program.escape_status);
    }
    resolve_unknowns(&mut program.escape_status);

    // --- Functions ---
    for func in &mut program.functions {
        for stmt in &func.body {
            mark_stmt(stmt, &mut func.escape_status);
        }
        resolve_unknowns(&mut func.escape_status);
    }
}

// ---------------------------------------------------------------------------
// Statement walker
// ---------------------------------------------------------------------------

fn mark_stmt(stmt: &MirStmt, status: &mut [Option<EscapeStatus>]) {
    match stmt {
        // --- Escape contexts (top-level) ---

        // Rule (c): returned values escape.
        MirStmt::Return(expr, _) => {
            mark_locals_in_expr(expr, status);
        }

        // Rule (d): exported values escape.
        MirStmt::Export { expr, .. } => {
            mark_locals_in_expr(expr, status);
        }

        // --- Containers — recurse ---
        MirStmt::Block(stmts, _) => {
            for s in stmts {
                mark_stmt(s, status);
            }
        }

        MirStmt::Let(_, expr, _) => {
            mark_expr(expr, status);
        }

        MirStmt::Assign(_, expr, _) => {
            mark_expr(expr, status);
        }

        MirStmt::Expr(expr, _) => {
            mark_expr(expr, status);
        }

        MirStmt::Yield(expr, _) => {
            mark_expr(expr, status);
        }

        MirStmt::Throw(expr, _) => {
            mark_expr(expr, status);
        }

        MirStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            mark_expr(condition, status);
            for s in then_body {
                mark_stmt(s, status);
            }
            for s in else_body {
                mark_stmt(s, status);
            }
        }

        MirStmt::While {
            condition, body, ..
        } => {
            mark_expr(condition, status);
            for s in body {
                mark_stmt(s, status);
            }
        }

        MirStmt::TryFinally {
            try_body,
            finally_body,
            ..
        } => {
            for s in try_body {
                mark_stmt(s, status);
            }
            for s in finally_body {
                mark_stmt(s, status);
            }
        }

        MirStmt::TryCatch {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            for s in try_body {
                mark_stmt(s, status);
            }
            if let Some(body) = catch_body {
                for s in body {
                    mark_stmt(s, status);
                }
            }
            if let Some(body) = finally_body {
                for s in body {
                    mark_stmt(s, status);
                }
            }
        }

        MirStmt::Switch { expr, cases, .. } => {
            mark_expr(expr, status);
            for (cond, body) in cases {
                if let Some(cond_expr) = cond {
                    mark_expr(cond_expr, status);
                }
                for s in body {
                    mark_stmt(s, status);
                }
            }
        }

        MirStmt::DoWhile {
            body, condition, ..
        } => {
            for s in body {
                mark_stmt(s, status);
            }
            mark_expr(condition, status);
        }

        MirStmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            if let Some(init_stmt) = init {
                mark_stmt(init_stmt, status);
            }
            if let Some(cond_expr) = condition {
                mark_expr(cond_expr, status);
            }
            if let Some(update_expr) = update {
                mark_expr(update_expr, status);
            }
            for s in body {
                mark_stmt(s, status);
            }
        }

        MirStmt::ForIn { iter, body, .. } | MirStmt::ForOf { iter, body, .. } => {
            mark_expr(iter, status);
            for s in body {
                mark_stmt(s, status);
            }
        }

        MirStmt::ForAwaitOfLower { iter, body, .. } => {
            mark_expr(iter, status);
            for s in body {
                mark_stmt(s, status);
            }
        }

        MirStmt::Labeled { body, .. } => {
            mark_stmt(body, status);
        }

        MirStmt::Break { .. } | MirStmt::Continue { .. } => {}

        MirStmt::ModuleExportsUpdate { .. } => {}

        MirStmt::ModuleExportsAssign { expr, .. } => {
            mark_expr(expr, status);
        }

        MirStmt::ClassDecl { .. } => {}
    }
}

// ---------------------------------------------------------------------------
// Expression walker
// ---------------------------------------------------------------------------

fn mark_expr(expr: &MirExpr, status: &mut [Option<EscapeStatus>]) {
    match expr {
        // --- Escape contexts ---

        // Rule (a): RuntimeCall arguments escape.
        MirExpr::RuntimeCall { args, .. } => {
            for arg in args {
                mark_locals_in_expr(arg, status);
            }
        }

        // Rule (b): values stored as object properties escape.
        MirExpr::PropertySet { value, .. } | MirExpr::PropertySetDynamic { value, .. } => {
            mark_locals_in_expr(value, status);
        }

        // --- Non-escape expressions — recurse into children ---
        MirExpr::Number(_, _)
        | MirExpr::DecimalNumber(_, _)
        | MirExpr::BigIntLiteral { .. }
        | MirExpr::String(_, _)
        | MirExpr::Bool(_, _)
        | MirExpr::Null(_)
        | MirExpr::Undefined(_)
        | MirExpr::This(_) => {}

        // Local is a leaf — no sub-expressions to walk.
        MirExpr::Local(_, _) => {}

        MirExpr::EnvCellNew(expr, _) => {
            mark_expr(expr, status);
        }

        MirExpr::EnvCellGet(_, _) => {}

        MirExpr::EnvCellSet { expr, .. } => {
            mark_expr(expr, status);
        }

        MirExpr::Unary { expr, .. } => {
            mark_expr(expr, status);
        }

        MirExpr::Binary { left, right, .. } => {
            mark_expr(left, status);
            mark_expr(right, status);
        }

        MirExpr::PropertyIn { obj, .. } => {
            mark_expr(obj, status);
        }

        MirExpr::PropertyInDynamic { obj, key, .. } => {
            mark_expr(obj, status);
            mark_expr(key, status);
        }

        MirExpr::Call { args, .. } => {
            for arg in args {
                mark_expr(arg, status);
            }
        }

        MirExpr::Assign { expr, .. } => {
            mark_expr(expr, status);
        }

        MirExpr::LogicalAssign { expr, .. } => {
            mark_expr(expr, status);
        }

        MirExpr::LogicalPropertyAssign { expr, .. } => {
            mark_expr(expr, status);
        }

        MirExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            mark_expr(key, status);
            mark_expr(expr, status);
        }

        MirExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            mark_expr(object, status);
            mark_expr(key, status);
            mark_expr(expr, status);
        }

        MirExpr::LogicalMemberAssign { object, expr, .. } => {
            mark_expr(object, status);
            mark_expr(expr, status);
        }

        MirExpr::ArrayNew { elements, .. } => {
            for elem in elements {
                mark_expr(elem, status);
            }
        }

        MirExpr::ArrayNewSparse { slots, .. } => {
            for slot in slots {
                match slot {
                    MirArraySlot::Present(expr) => mark_expr(expr, status),
                    MirArraySlot::Hole => {}
                }
            }
        }

        MirExpr::ArrayGet { arr, index, .. } => {
            mark_expr(arr, status);
            mark_expr(index, status);
        }

        MirExpr::Index { object, index, .. } => {
            mark_expr(object, status);
            mark_expr(index, status);
        }

        MirExpr::GetLength(expr, _) => {
            mark_expr(expr, status);
        }

        MirExpr::ObjectNew { props, .. } => {
            for (_, val) in props {
                mark_expr(val, status);
            }
        }

        MirExpr::ErrorNew { message, cause, .. } => {
            mark_expr(message, status);
            if let Some(cause_expr) = cause {
                mark_expr(cause_expr, status);
            }
        }

        MirExpr::PropertyGet { obj, .. } | MirExpr::OptionalPropertyGet { obj, .. } => {
            mark_expr(obj, status);
        }

        MirExpr::PropertyGetDynamic { obj, key, .. } => {
            mark_expr(obj, status);
            mark_expr(key, status);
        }

        MirExpr::OptionalIndex { object, index, .. } => {
            mark_expr(object, status);
            mark_expr(index, status);
        }

        MirExpr::OptionalCall { callee, call, .. } => {
            mark_expr(callee, status);
            mark_expr(call, status);
        }

        MirExpr::MethodCall { object, .. } => {
            mark_expr(object, status);
        }

        MirExpr::PromiseGetValue { promise, .. } => {
            mark_expr(promise, status);
        }

        MirExpr::PropertyDelete { object, .. } | MirExpr::PropertyDeleteDynamic { object, .. } => {
            mark_expr(object, status);
        }

        MirExpr::New { args, .. } => {
            for arg in args {
                mark_expr(arg, status);
            }
        }

        MirExpr::ClassPrototype(_, _) | MirExpr::BuiltinErrorPrototype(_, _) => {}

        MirExpr::ModuleLoad { .. } => {}

        MirExpr::Block { stmts, result, .. } => {
            for s in stmts {
                mark_stmt(s, status);
            }
            mark_expr(result, status);
        }

        MirExpr::ArrowFn { .. } => {}
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Recursively collect all `LocalId` references from an expression tree
/// and mark them as `Escaped`.
fn mark_locals_in_expr(expr: &MirExpr, status: &mut [Option<EscapeStatus>]) {
    match expr {
        MirExpr::Local(id, _) => {
            mark_escaped(*id, status);
        }
        MirExpr::EnvCellGet(_, _) => {}
        MirExpr::Number(_, _)
        | MirExpr::DecimalNumber(_, _)
        | MirExpr::BigIntLiteral { .. }
        | MirExpr::String(_, _)
        | MirExpr::Bool(_, _)
        | MirExpr::Null(_)
        | MirExpr::Undefined(_)
        | MirExpr::This(_)
        | MirExpr::ClassPrototype(_, _)
        | MirExpr::BuiltinErrorPrototype(_, _)
        | MirExpr::ModuleLoad { .. }
        | MirExpr::ArrowFn { .. } => {}

        MirExpr::EnvCellNew(e, _)
        | MirExpr::Unary { expr: e, .. }
        | MirExpr::Assign { expr: e, .. }
        | MirExpr::GetLength(e, _)
        | MirExpr::PromiseGetValue { promise: e, .. } => {
            mark_locals_in_expr(e, status);
        }

        MirExpr::EnvCellSet { expr, .. } => {
            mark_locals_in_expr(expr, status);
        }

        MirExpr::Binary { left, right, .. } => {
            mark_locals_in_expr(left, status);
            mark_locals_in_expr(right, status);
        }

        MirExpr::PropertyIn { obj, .. }
        | MirExpr::PropertyGet { obj, .. }
        | MirExpr::OptionalPropertyGet { obj, .. }
        | MirExpr::MethodCall { object: obj, .. }
        | MirExpr::PropertyDelete { object: obj, .. } => {
            mark_locals_in_expr(obj, status);
        }

        MirExpr::PropertyInDynamic { obj, key, .. }
        | MirExpr::PropertyGetDynamic { obj, key, .. }
        | MirExpr::PropertyDeleteDynamic {
            object: obj, key, ..
        } => {
            mark_locals_in_expr(obj, status);
            mark_locals_in_expr(key, status);
        }

        MirExpr::PropertySet { object, value, .. }
        | MirExpr::PropertySetDynamic {
            object,
            index: _,
            value,
            ..
        } => {
            mark_locals_in_expr(object, status);
            mark_locals_in_expr(value, status);
        }

        MirExpr::Call { args, .. } | MirExpr::RuntimeCall { args, .. } => {
            for arg in args {
                mark_locals_in_expr(arg, status);
            }
        }

        MirExpr::LogicalAssign { expr, .. } => {
            mark_locals_in_expr(expr, status);
        }

        MirExpr::LogicalPropertyAssign { expr, .. } => {
            mark_locals_in_expr(expr, status);
        }

        MirExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            mark_locals_in_expr(key, status);
            mark_locals_in_expr(expr, status);
        }

        MirExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            mark_locals_in_expr(object, status);
            mark_locals_in_expr(key, status);
            mark_locals_in_expr(expr, status);
        }

        MirExpr::LogicalMemberAssign { object, expr, .. } => {
            mark_locals_in_expr(object, status);
            mark_locals_in_expr(expr, status);
        }

        MirExpr::ArrayNew { elements, .. } => {
            for elem in elements {
                mark_locals_in_expr(elem, status);
            }
        }

        MirExpr::ArrayNewSparse { slots, .. } => {
            for slot in slots {
                if let MirArraySlot::Present(expr) = slot {
                    mark_locals_in_expr(expr, status);
                }
            }
        }

        MirExpr::ArrayGet { arr, index, .. }
        | MirExpr::Index {
            object: arr, index, ..
        } => {
            mark_locals_in_expr(arr, status);
            mark_locals_in_expr(index, status);
        }

        MirExpr::ObjectNew { props, .. } => {
            for (_, val) in props {
                mark_locals_in_expr(val, status);
            }
        }

        MirExpr::ErrorNew { message, cause, .. } => {
            mark_locals_in_expr(message, status);
            if let Some(cause_expr) = cause {
                mark_locals_in_expr(cause_expr, status);
            }
        }

        MirExpr::OptionalIndex { object, index, .. } => {
            mark_locals_in_expr(object, status);
            mark_locals_in_expr(index, status);
        }

        MirExpr::OptionalCall { callee, call, .. } => {
            mark_locals_in_expr(callee, status);
            mark_locals_in_expr(call, status);
        }

        MirExpr::New { args, .. } => {
            for arg in args {
                mark_locals_in_expr(arg, status);
            }
        }

        MirExpr::Block { stmts, result, .. } => {
            for s in stmts {
                mark_stmt(s, status);
            }
            mark_locals_in_expr(result, status);
        }
    }
}

fn mark_escaped(local: LocalId, status: &mut [Option<EscapeStatus>]) {
    let idx = local.0 as usize;
    if idx < status.len() {
        status[idx] = Some(EscapeStatus::Escaped);
    }
}

/// Convert all remaining `Unknown` entries to `NotEscaped`.
fn resolve_unknowns(status: &mut [Option<EscapeStatus>]) {
    for entry in status.iter_mut() {
        if *entry == Some(EscapeStatus::Unknown) || entry.is_none() {
            *entry = Some(EscapeStatus::NotEscaped);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_unknown_resolves_to_not_escaped() {
        let mut status: Vec<Option<EscapeStatus>> = vec![
            None,
            Some(EscapeStatus::Unknown),
            Some(EscapeStatus::Escaped),
        ];
        resolve_unknowns(&mut status);
        assert_eq!(status[0], Some(EscapeStatus::NotEscaped));
        assert_eq!(status[1], Some(EscapeStatus::NotEscaped));
        assert_eq!(status[2], Some(EscapeStatus::Escaped));
    }

    #[test]
    fn escape_func_not_escaped() {
        // let a = 42; return a;
        // Local 0 is only read/written locally — NotEscaped.
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![crate::lowered::LocalId(0)],
            body: vec![
                MirStmt::Let(
                    crate::lowered::LocalId(0),
                    MirExpr::Number(42, Span { start: 0, end: 0 }),
                    Span { start: 0, end: 0 },
                ),
                MirStmt::Return(
                    MirExpr::Local(crate::lowered::LocalId(0), Span { start: 0, end: 0 }),
                    Span { start: 0, end: 0 },
                ),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            escape_status: vec![None],
        };

        let mut program = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![func],
            modules: vec![],
            escape_status: vec![],
        };

        analyze_escape(&mut program);
        let func = &program.functions[0];
        // Local 0 is returned, so it escapes!
        assert_eq!(
            func.escape_status[0],
            Some(EscapeStatus::Escaped),
            "a returned local should be Escaped"
        );
    }

    #[test]
    fn escape_local_read_not_escaped() {
        // let a = 42; let b = a; return b;
        // a (local 0) is only read, never leaves the function -> NotEscaped
        // b (local 1) is returned -> Escaped
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![crate::lowered::LocalId(0), crate::lowered::LocalId(1)],
            body: vec![
                MirStmt::Let(
                    crate::lowered::LocalId(0),
                    MirExpr::Number(42, Span { start: 0, end: 0 }),
                    Span { start: 0, end: 0 },
                ),
                MirStmt::Let(
                    crate::lowered::LocalId(1),
                    MirExpr::Local(crate::lowered::LocalId(0), Span { start: 0, end: 0 }),
                    Span { start: 0, end: 0 },
                ),
                MirStmt::Return(
                    MirExpr::Local(crate::lowered::LocalId(1), Span { start: 0, end: 0 }),
                    Span { start: 0, end: 0 },
                ),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            escape_status: vec![None; 2],
        };

        let mut program = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![func],
            modules: vec![],
            escape_status: vec![],
        };

        analyze_escape(&mut program);
        let func = &program.functions[0];
        assert_eq!(
            func.escape_status[0],
            Some(EscapeStatus::NotEscaped),
            "local only read within function should be NotEscaped"
        );
        assert_eq!(
            func.escape_status[1],
            Some(EscapeStatus::Escaped),
            "returned local should be Escaped"
        );
    }

    #[test]
    fn escape_runtime_call_arg_escapes() {
        // let a = [1, 2]; ArrayPushGrow(a, 3);
        // Local 0 passed to RuntimeCall -> Escaped
        let span = Span { start: 0, end: 0 };
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![crate::lowered::LocalId(0)],
            body: vec![
                MirStmt::Let(
                    crate::lowered::LocalId(0),
                    MirExpr::ArrayNew {
                        elements: vec![MirExpr::Number(1, span), MirExpr::Number(2, span)],
                        span,
                    },
                    span,
                ),
                MirStmt::Expr(
                    MirExpr::RuntimeCall {
                        intrinsic: crate::lowered::RuntimeFn::ArrayPushGrow,
                        args: vec![
                            MirExpr::Local(crate::lowered::LocalId(0), span),
                            MirExpr::Number(3, span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            escape_status: vec![None],
        };

        let mut program = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![func],
            modules: vec![],
            escape_status: vec![],
        };

        analyze_escape(&mut program);
        assert_eq!(
            program.functions[0].escape_status[0],
            Some(EscapeStatus::Escaped),
            "local passed to RuntimeCall should be Escaped"
        );
    }

    #[test]
    fn escape_property_set_value_escapes() {
        // let a = {}; let b = 42; a.x = b;
        // b (local 1) is stored as a property value -> Escaped
        let span = Span { start: 0, end: 0 };
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![crate::lowered::LocalId(0), crate::lowered::LocalId(1)],
            body: vec![
                MirStmt::Let(
                    crate::lowered::LocalId(0),
                    MirExpr::ObjectNew {
                        props: vec![],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                MirStmt::Let(crate::lowered::LocalId(1), MirExpr::Number(42, span), span),
                MirStmt::Expr(
                    MirExpr::PropertySet {
                        object: Box::new(MirExpr::Local(crate::lowered::LocalId(0), span)),
                        key: "x".to_string(),
                        value: Box::new(MirExpr::Local(crate::lowered::LocalId(1), span)),
                        span,
                    },
                    span,
                ),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            escape_status: vec![None; 2],
        };

        let mut program = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![func],
            modules: vec![],
            escape_status: vec![],
        };

        analyze_escape(&mut program);
        assert_eq!(
            program.functions[0].escape_status[0],
            Some(EscapeStatus::NotEscaped),
            "object not stored elsewhere should be NotEscaped"
        );
        assert_eq!(
            program.functions[0].escape_status[1],
            Some(EscapeStatus::Escaped),
            "local stored as property value should be Escaped"
        );
    }

    #[test]
    fn escape_export_escapes() {
        // export { a };
        // Local 0 bound to an export -> Escaped
        let span = Span { start: 0, end: 0 };
        let mut program = MirProgram {
            top_level_statements: vec![
                MirStmt::Let(crate::lowered::LocalId(0), MirExpr::Number(1, span), span),
                MirStmt::Export {
                    name: "a".to_string(),
                    expr: MirExpr::Local(crate::lowered::LocalId(0), span),
                    span,
                },
            ],
            top_level_locals: vec![crate::lowered::LocalId(0)],
            functions: vec![],
            modules: vec![],
            escape_status: vec![None],
        };

        analyze_escape(&mut program);
        assert_eq!(
            program.escape_status[0],
            Some(EscapeStatus::Escaped),
            "exported local should be Escaped"
        );
    }

    #[test]
    fn escape_empty_program_all_not_escaped() {
        let mut program = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
            escape_status: vec![],
        };
        analyze_escape(&mut program);
        assert!(program.escape_status.is_empty());
        assert!(program.functions.is_empty());
    }
}
