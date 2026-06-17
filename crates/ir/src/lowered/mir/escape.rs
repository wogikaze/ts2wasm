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
//! 5. **Call/OptionalCall/New argument** — the callee/constructor may
//!    store the reference.
//! 6. **ModuleExportsUpdate** — a local is bound into the module's
//!    exports table (top-level re-export).
//! 7. **Alias propagation** — if local `a` holds a reference to the
//!    same object as local `b` (via `let b = a` or `let a = b`), and
//!    either `a` or `b` escapes, then *both* are classified as
//!    `Escaped`.
//!
//! Locals that are only created, read, and written within their function
//! (and never appear in the positions above) are classified as
//! `NotEscaped`.

#[cfg(test)]
use super::types::MirFunction;
use super::types::{EscapeStatus, MirArraySlot, MirExpr, MirProgram, MirStmt};
use crate::lowered::LocalId;
#[cfg(test)]
use crate::lowered::{FuncId, types::ClassPrototypeRef, types::FunctionCallKind};
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
    propagate_aliases_in_stmts(&program.top_level_statements, &mut program.escape_status);

    // --- Functions ---
    for func in &mut program.functions {
        for stmt in &func.body {
            mark_stmt(stmt, &mut func.escape_status);
        }
        resolve_unknowns(&mut func.escape_status);
        propagate_aliases_in_stmts(&func.body, &mut func.escape_status);
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

        // Rule (6): module exports update — the local is bound to an export.
        MirStmt::ModuleExportsUpdate { local, .. } => {
            mark_escaped(*local, status);
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

        // Rule (5): Call/OptionalCall/New arguments escape.
        MirExpr::Call { args, .. } => {
            for arg in args {
                mark_locals_in_expr(arg, status);
            }
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

        MirExpr::MethodCall { object, .. } => {
            mark_expr(object, status);
        }

        MirExpr::PromiseGetValue { promise, .. } => {
            mark_expr(promise, status);
        }

        MirExpr::PropertyDelete { object, .. } | MirExpr::PropertyDeleteDynamic { object, .. } => {
            mark_expr(object, status);
        }

        MirExpr::ClassPrototype(_, _)
        | MirExpr::BuiltinErrorPrototype(_, _)
        | MirExpr::BuiltinConstructor(_, _) => {}

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
// Alias propagation
// ---------------------------------------------------------------------------

/// Propagate escape status transitively through aliasing assignments.
///
/// When `let dest = src` or equivalent aliasing patterns exist, `dest` and
/// `src` reference the same object. If either escapes, both must be
/// considered escaped. This function uses a fixed-point iteration over
/// a bidirectional edge graph to propagate escape status until convergence.
fn propagate_aliases_in_stmts(stmts: &[MirStmt], status: &mut [Option<EscapeStatus>]) {
    let mut edges: Vec<(LocalId, LocalId)> = Vec::new();
    collect_alias_edges_in_stmts(stmts, &mut edges);
    if edges.is_empty() {
        return;
    }

    // Fixed-point iteration: propagate bidirectionally
    loop {
        let mut changed = false;
        for &(dest, src) in &edges {
            let dest_idx = dest.0 as usize;
            let src_idx = src.0 as usize;
            if dest_idx >= status.len() || src_idx >= status.len() {
                continue;
            }
            let dest_escaped = status[dest_idx] == Some(EscapeStatus::Escaped);
            let src_escaped = status[src_idx] == Some(EscapeStatus::Escaped);

            if dest_escaped && !src_escaped {
                status[src_idx] = Some(EscapeStatus::Escaped);
                changed = true;
            }
            if src_escaped && !dest_escaped {
                status[dest_idx] = Some(EscapeStatus::Escaped);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

/// Collect aliasing edges from statements.
///
/// An edge `(dest, src)` means that `dest` gets its value from `src`
/// (e.g., `let dest = src` where `src` is a local), so they refer to
/// the same object.
fn collect_alias_edges_in_stmts(stmts: &[MirStmt], edges: &mut Vec<(LocalId, LocalId)>) {
    for stmt in stmts {
        match stmt {
            MirStmt::Let(dest, expr, _) => {
                collect_alias_edges_in_expr(*dest, expr, edges);
            }
            MirStmt::Block(inner, _) => {
                collect_alias_edges_in_stmts(inner, edges);
            }
            MirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_alias_edges_in_stmts(then_body, edges);
                collect_alias_edges_in_stmts(else_body, edges);
            }
            MirStmt::While { body, .. } => {
                collect_alias_edges_in_stmts(body, edges);
            }
            MirStmt::For { body, .. } => {
                collect_alias_edges_in_stmts(body, edges);
            }
            MirStmt::ForIn { body, .. } | MirStmt::ForOf { body, .. } => {
                collect_alias_edges_in_stmts(body, edges);
            }
            MirStmt::ForAwaitOfLower { body, .. } => {
                collect_alias_edges_in_stmts(body, edges);
            }
            MirStmt::DoWhile { body, .. } => {
                collect_alias_edges_in_stmts(body, edges);
            }
            MirStmt::Labeled { body, .. } => {
                collect_alias_edges_in_stmts(std::slice::from_ref(body.as_ref()), edges);
            }
            MirStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_alias_edges_in_stmts(body, edges);
                }
            }
            MirStmt::TryFinally {
                try_body,
                finally_body,
                ..
            } => {
                collect_alias_edges_in_stmts(try_body, edges);
                collect_alias_edges_in_stmts(finally_body, edges);
            }
            MirStmt::TryCatch {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                collect_alias_edges_in_stmts(try_body, edges);
                if let Some(body) = catch_body {
                    collect_alias_edges_in_stmts(body, edges);
                }
                if let Some(body) = finally_body {
                    collect_alias_edges_in_stmts(body, edges);
                }
            }
            // Other statement types do not introduce aliasing edges.
            MirStmt::Return(..)
            | MirStmt::Export { .. }
            | MirStmt::ModuleExportsUpdate { .. }
            | MirStmt::ModuleExportsAssign { .. }
            | MirStmt::Assign(..)
            | MirStmt::Expr(..)
            | MirStmt::Yield(..)
            | MirStmt::Throw(..)
            | MirStmt::Break { .. }
            | MirStmt::Continue { .. }
            | MirStmt::ClassDecl { .. } => {}
        }
    }
}

/// Collect aliasing edges from an expression assigned to a destination local.
fn collect_alias_edges_in_expr(dest: LocalId, expr: &MirExpr, edges: &mut Vec<(LocalId, LocalId)>) {
    match expr {
        // Direct aliases: `let dest = src` where src is a local.
        MirExpr::Local(src, _) => {
            edges.push((dest, *src));
        }
        // Recurse into sub-expressions to find indirect aliasing.
        MirExpr::Unary { expr: inner, .. }
        | MirExpr::EnvCellNew(inner, _)
        | MirExpr::Assign { expr: inner, .. }
        | MirExpr::GetLength(inner, _)
        | MirExpr::PromiseGetValue { promise: inner, .. } => {
            collect_alias_edges_in_expr(dest, inner, edges);
        }
        MirExpr::Binary { left, right, .. } => {
            collect_alias_edges_in_expr(dest, left, edges);
            collect_alias_edges_in_expr(dest, right, edges);
        }
        MirExpr::Call { args, .. } | MirExpr::RuntimeCall { args, .. } => {
            for arg in args {
                collect_alias_edges_in_expr(dest, arg, edges);
            }
        }
        MirExpr::OptionalCall { callee, call, .. } => {
            collect_alias_edges_in_expr(dest, callee, edges);
            collect_alias_edges_in_expr(dest, call, edges);
        }
        MirExpr::New { args, .. } => {
            for arg in args {
                collect_alias_edges_in_expr(dest, arg, edges);
            }
        }
        MirExpr::ArrayNew { elements, .. } => {
            for elem in elements {
                collect_alias_edges_in_expr(dest, elem, edges);
            }
        }
        MirExpr::ArrayNewSparse { slots, .. } => {
            for slot in slots {
                if let MirArraySlot::Present(expr) = slot {
                    collect_alias_edges_in_expr(dest, expr, edges);
                }
            }
        }
        MirExpr::ObjectNew { props, .. } => {
            for (_, val) in props {
                collect_alias_edges_in_expr(dest, val, edges);
            }
        }
        MirExpr::ErrorNew { message, cause, .. } => {
            collect_alias_edges_in_expr(dest, message, edges);
            if let Some(cause_expr) = cause {
                collect_alias_edges_in_expr(dest, cause_expr, edges);
            }
        }
        MirExpr::Block { stmts, result, .. } => {
            for s in stmts {
                collect_alias_edges_in_stmts(std::slice::from_ref(s), edges);
            }
            collect_alias_edges_in_expr(dest, result, edges);
        }
        // Literals and other non-alias-producing expressions.
        MirExpr::EnvCellGet(_, _)
        | MirExpr::EnvCellSet { .. }
        | MirExpr::Number(_, _)
        | MirExpr::DecimalNumber(_, _)
        | MirExpr::BigIntLiteral { .. }
        | MirExpr::String(_, _)
        | MirExpr::Bool(_, _)
        | MirExpr::Null(_)
        | MirExpr::Undefined(_)
        | MirExpr::This(_)
        | MirExpr::ClassPrototype(_, _)
        | MirExpr::BuiltinErrorPrototype(_, _)
        | MirExpr::BuiltinConstructor(_, _)
        | MirExpr::ModuleLoad { .. }
        | MirExpr::ArrowFn { .. }
        | MirExpr::PropertySet { .. }
        | MirExpr::PropertySetDynamic { .. }
        | MirExpr::PropertyGet { .. }
        | MirExpr::OptionalPropertyGet { .. }
        | MirExpr::PropertyGetDynamic { .. }
        | MirExpr::OptionalIndex { .. }
        | MirExpr::Index { .. }
        | MirExpr::ArrayGet { .. }
        | MirExpr::PropertyIn { .. }
        | MirExpr::PropertyInDynamic { .. }
        | MirExpr::PropertyDelete { .. }
        | MirExpr::PropertyDeleteDynamic { .. }
        | MirExpr::MethodCall { .. }
        | MirExpr::LogicalAssign { .. }
        | MirExpr::LogicalPropertyAssign { .. }
        | MirExpr::LogicalComputedPropertyAssign { .. }
        | MirExpr::LogicalComputedMemberAssign { .. }
        | MirExpr::LogicalMemberAssign { .. } => {}
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
        | MirExpr::BuiltinConstructor(_, _)
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
            value_reps: vec![],
            optimization_hints: vec![],
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
        // a (local 0) is aliased by b via `let b = a`; b is returned,
        // so both a and b escape through alias propagation.
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
            value_reps: vec![],
            optimization_hints: vec![],
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
            Some(EscapeStatus::Escaped),
            "local a should escape via alias propagation from b"
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
            value_reps: vec![],
            optimization_hints: vec![],
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
            value_reps: vec![],
            optimization_hints: vec![],
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

    // --- New tests for Phase C extensions ---

    #[test]
    fn escape_call_arg_escapes() {
        // let a = {}; foo(a);
        // Local 0 passed to Call -> Escaped
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
                    MirExpr::ObjectNew {
                        props: vec![],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                MirStmt::Expr(
                    MirExpr::Call {
                        kind: FunctionCallKind::User(FuncId(1)),
                        args: vec![MirExpr::Local(crate::lowered::LocalId(0), span)],
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
            value_reps: vec![],
            optimization_hints: vec![],
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
            Some(EscapeStatus::Escaped),
            "local passed to Call should be Escaped"
        );
    }

    #[test]
    fn escape_new_arg_escapes() {
        // let a = {}; new Foo(a);
        // Local 0 passed to New -> Escaped
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
                    MirExpr::ObjectNew {
                        props: vec![],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                MirStmt::Expr(
                    MirExpr::New {
                        constructor: FuncId(1),
                        prototype: ClassPrototypeRef {
                            constructor: FuncId(1),
                            parent_constructors: vec![],
                        },
                        args: vec![MirExpr::Local(crate::lowered::LocalId(0), span)],
                        base_local: crate::lowered::LocalId(0),
                        private_brand: None,
                        private_slot_count: 0,
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
            value_reps: vec![],
            optimization_hints: vec![],
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
            Some(EscapeStatus::Escaped),
            "local passed to New should be Escaped"
        );
    }

    #[test]
    fn escape_optional_call_arg_escapes() {
        // let a = {}; foo?.(a);
        // Local 0 passed to OptionalCall -> Escaped
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
                    MirExpr::ObjectNew {
                        props: vec![],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                MirStmt::Expr(
                    MirExpr::OptionalCall {
                        callee: Box::new(MirExpr::Local(crate::lowered::LocalId(1), span)),
                        call: Box::new(MirExpr::Call {
                            kind: FunctionCallKind::User(FuncId(1)),
                            args: vec![MirExpr::Local(crate::lowered::LocalId(0), span)],
                            span,
                        }),
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
            value_reps: vec![],
            optimization_hints: vec![],
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
            Some(EscapeStatus::Escaped),
            "local passed to OptionalCall should be Escaped"
        );
    }

    #[test]
    fn escape_module_exports_update_escapes() {
        // Top-level: ModuleExportsUpdate { local: 0, name: "foo" }
        // Local 0 is bound into module exports -> Escaped
        let span = Span { start: 0, end: 0 };
        let mut program = MirProgram {
            top_level_statements: vec![
                MirStmt::Let(crate::lowered::LocalId(0), MirExpr::Number(1, span), span),
                MirStmt::ModuleExportsUpdate {
                    name: "foo".to_string(),
                    local: crate::lowered::LocalId(0),
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
            "local in ModuleExportsUpdate should be Escaped"
        );
    }

    #[test]
    fn escape_alias_propagation_let_b_a() {
        // let a = {}; let b = a; return b;
        // a (local 0) is aliased by b (local 1); b is returned -> both escape
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
                MirStmt::Let(
                    crate::lowered::LocalId(1),
                    MirExpr::Local(crate::lowered::LocalId(0), span),
                    span,
                ),
                MirStmt::Return(MirExpr::Local(crate::lowered::LocalId(1), span), span),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
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
        // Both a and b escape because b is returned and a is aliased by b
        assert_eq!(
            program.functions[0].escape_status[0],
            Some(EscapeStatus::Escaped),
            "a should escape via alias propagation from b"
        );
        assert_eq!(
            program.functions[0].escape_status[1],
            Some(EscapeStatus::Escaped),
            "b is returned so it should escape"
        );
    }

    #[test]
    fn escape_alias_propagation_chain() {
        // let a = {}; let b = a; let c = b; return c;
        // a (0) -> b (1) -> c (2) -> returned
        // Through alias propagation, all three escape
        let span = Span { start: 0, end: 0 };
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![
                crate::lowered::LocalId(0),
                crate::lowered::LocalId(1),
                crate::lowered::LocalId(2),
            ],
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
                MirStmt::Let(
                    crate::lowered::LocalId(1),
                    MirExpr::Local(crate::lowered::LocalId(0), span),
                    span,
                ),
                MirStmt::Let(
                    crate::lowered::LocalId(2),
                    MirExpr::Local(crate::lowered::LocalId(1), span),
                    span,
                ),
                MirStmt::Return(MirExpr::Local(crate::lowered::LocalId(2), span), span),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
            escape_status: vec![None; 3],
        };

        let mut program = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![func],
            modules: vec![],
            escape_status: vec![],
        };

        analyze_escape(&mut program);
        // All three should escape through alias propagation
        for i in 0..3 {
            assert_eq!(
                program.functions[0].escape_status[i],
                Some(EscapeStatus::Escaped),
                "local {} should escape via alias chain",
                i
            );
        }
    }

    #[test]
    fn escape_alias_propagation_bidirectional() {
        // let b = {}; let a = b; return a;
        // b (0) -> a (1) via let a = b; a is returned -> both escape
        // This tests that dest->src propagation works (a escapes -> b escapes)
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
                MirStmt::Let(
                    crate::lowered::LocalId(1),
                    MirExpr::Local(crate::lowered::LocalId(0), span),
                    span,
                ),
                MirStmt::Return(MirExpr::Local(crate::lowered::LocalId(1), span), span),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
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
        // Both should escape: a is returned, and b is aliased by a
        assert_eq!(
            program.functions[0].escape_status[0],
            Some(EscapeStatus::Escaped),
            "b should escape via alias propagation from a"
        );
        assert_eq!(
            program.functions[0].escape_status[1],
            Some(EscapeStatus::Escaped),
            "a is returned so it should escape"
        );
    }

    #[test]
    fn escape_alias_no_escape_no_propagation() {
        // let a = {}; let b = a;
        // Neither a nor b escapes -> both NotEscaped
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
                MirStmt::Let(
                    crate::lowered::LocalId(1),
                    MirExpr::Local(crate::lowered::LocalId(0), span),
                    span,
                ),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
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
        // Neither escapes -> both NotEscaped
        for i in 0..2 {
            assert_eq!(
                program.functions[0].escape_status[i],
                Some(EscapeStatus::NotEscaped),
                "local {} should be NotEscaped when nothing escapes",
                i
            );
        }
    }

    #[test]
    fn escape_alias_propagation_through_blocks() {
        // let a = {};
        // {
        //   let b = a;
        //   return b;
        // }
        // a is aliased by b inside a block; b is returned -> both escape
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
                MirStmt::Block(
                    vec![
                        MirStmt::Let(
                            crate::lowered::LocalId(1),
                            MirExpr::Local(crate::lowered::LocalId(0), span),
                            span,
                        ),
                        MirStmt::Return(MirExpr::Local(crate::lowered::LocalId(1), span), span),
                    ],
                    span,
                ),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
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
        // Both should escape through alias propagation across block boundary
        assert_eq!(
            program.functions[0].escape_status[0],
            Some(EscapeStatus::Escaped),
            "a should escape via alias propagation from b in block"
        );
        assert_eq!(
            program.functions[0].escape_status[1],
            Some(EscapeStatus::Escaped),
            "b is returned so it should escape"
        );
    }

    #[test]
    fn escape_mixed_escaped_and_aliased() {
        // let a = {};     // local 0 - object
        // let b = {};     // local 1 - another object
        // let c = a;      // local 2 - alias of a
        // RuntimeCall(a); // a escapes via RuntimeCall
        // return c;       // c is returned -> escapes
        // b is not used after creation -> NotEscaped
        let span = Span { start: 0, end: 0 };
        let func = MirFunction {
            id: crate::lowered::FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![
                crate::lowered::LocalId(0),
                crate::lowered::LocalId(1),
                crate::lowered::LocalId(2),
            ],
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
                MirStmt::Let(
                    crate::lowered::LocalId(1),
                    MirExpr::ObjectNew {
                        props: vec![],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                MirStmt::Let(
                    crate::lowered::LocalId(2),
                    MirExpr::Local(crate::lowered::LocalId(0), span),
                    span,
                ),
                MirStmt::Expr(
                    MirExpr::RuntimeCall {
                        intrinsic: crate::lowered::RuntimeFn::ArrayPushGrow,
                        args: vec![MirExpr::Local(crate::lowered::LocalId(0), span)],
                        span,
                    },
                    span,
                ),
                MirStmt::Return(MirExpr::Local(crate::lowered::LocalId(2), span), span),
            ],
            recursion_depth: 0,
            is_async: false,
            is_generator: false,
            generator_state: None,
            induction_vars: vec![],
            value_reps: vec![],
            optimization_hints: vec![],
            escape_status: vec![None; 3],
        };

        let mut program = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![func],
            modules: vec![],
            escape_status: vec![],
        };

        analyze_escape(&mut program);
        // a (0) escapes via both RuntimeCall and alias from c
        assert_eq!(
            program.functions[0].escape_status[0],
            Some(EscapeStatus::Escaped),
            "a escapes via RuntimeCall and alias to c"
        );
        // b (1) never escapes
        assert_eq!(
            program.functions[0].escape_status[1],
            Some(EscapeStatus::NotEscaped),
            "b never escapes"
        );
        // c (2) escapes via being returned
        assert_eq!(
            program.functions[0].escape_status[2],
            Some(EscapeStatus::Escaped),
            "c escapes via return"
        );
    }
}
