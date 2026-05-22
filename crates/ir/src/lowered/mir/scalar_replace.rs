//! Scalar replacement for MIR — Phase B of escape analysis.
//!
//! Replaces objects that do not escape their function with their individual
//! property values. This eliminates heap allocation for short-lived objects
//! and enables further optimization down the pipeline.
//!
//! ## Transformation
//!
//! Given:
//! ```ignore
//! let obj = { x: 1, y: 2 };
//! let a = obj.x;
//! let b = obj.y;
//! ```
//!
//! After scalar replacement:
//! ```ignore
//! let obj_0 = 1;   // property "x"
//! let obj_1 = 2;   // property "y"
//! let a = obj_0;
//! let b = obj_1;
//! ```
//!
//! ## Limitations
//!
//! - Objects with computed/dynamic property access are skipped.
//! - Only objects created directly via `ObjectNew` are candidates.
//! - The object must be classified `NotEscaped` by Phase A escape analysis.

use std::collections::{HashMap, HashSet};

use crate::lowered::LocalId;

use super::types::{EscapeStatus, MirExpr, MirProgram, MirStmt};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Run scalar replacement on all functions in a MIR program.
///
/// For each function body (and top-level), finds `Let` statements that create
/// an `ObjectNew` whose local is marked `NotEscaped`, and replaces the object
/// with individual locals for each property.
///
/// Must be called **after** `analyze_escape` (Phase A) so that escape status
/// is available for all locals.
pub fn scalar_replace(program: &mut MirProgram) {
    // Process each function body
    for func in &mut program.functions {
        if func.body.is_empty() {
            continue;
        }
        let escape_snapshot = func.escape_status.clone();
        scalar_replace_body(
            &mut func.body,
            &escape_snapshot,
            &mut func.locals,
            &mut func.escape_status,
        );
    }

    // Process top-level statements
    if !program.top_level_statements.is_empty() {
        let escape_snapshot = program.escape_status.clone();
        scalar_replace_body(
            &mut program.top_level_statements,
            &escape_snapshot,
            &mut program.top_level_locals,
            &mut program.escape_status,
        );
    }
}

// ---------------------------------------------------------------------------
// Per-body transformation
// ---------------------------------------------------------------------------

/// Run scalar replacement on a single statement list (function body or top-level).
fn scalar_replace_body(
    stmts: &mut Vec<MirStmt>,
    escape_status: &[Option<EscapeStatus>],
    locals: &mut Vec<LocalId>,
    escape: &mut Vec<Option<EscapeStatus>>,
) {
    // Phase 1: collect candidate object locals and their property keys.
    let candidates = collect_candidates(stmts, escape_status);
    if candidates.is_empty() {
        return;
    }

    let replaced_set: HashSet<LocalId> = candidates.keys().copied().collect();

    // Phase 2: filter out objects with dynamic/computed property access.
    let clean: HashMap<LocalId, HashMap<String, LocalId>> =
        filter_dynamic(stmts, candidates, &replaced_set);
    if clean.is_empty() {
        return;
    }

    let clean_set: HashSet<LocalId> = clean.keys().copied().collect();

    // Phase 3: allocate new scalar locals.
    let total_new: usize = clean.values().map(|keys| keys.len()).sum();
    if total_new == 0 {
        return;
    }
    let base = locals.len();
    for i in 0..total_new {
        let new_id = LocalId(base + i);
        locals.push(new_id);
    }
    escape.resize(locals.len(), Some(EscapeStatus::NotEscaped));

    // Fill in the actual LocalIds in the mapping.
    let mapping = fill_mapping(clean, base);

    // Phase 4: rewrite the body.
    let mut new_stmts: Vec<MirStmt> = Vec::with_capacity(stmts.len() + total_new);
    rewrite_stmts(stmts, &clean_set, &mapping, &mut new_stmts);
    *stmts = new_stmts;
}

// ---------------------------------------------------------------------------
// Candidate collection
// ---------------------------------------------------------------------------

/// Scan statements for `Let(local, ObjectNew { props, .. })` where the local
/// is `NotEscaped`. Returns a map from object local to its property keys
/// (with placeholder LocalIds to be filled in later).
fn collect_candidates(
    stmts: &[MirStmt],
    escape_status: &[Option<EscapeStatus>],
) -> HashMap<LocalId, Vec<String>> {
    let mut candidates: HashMap<LocalId, Vec<String>> = HashMap::new();
    for stmt in stmts {
        collect_candidates_in_stmt(stmt, escape_status, &mut candidates);
    }
    candidates
}

fn collect_candidates_in_stmt(
    stmt: &MirStmt,
    escape_status: &[Option<EscapeStatus>],
    candidates: &mut HashMap<LocalId, Vec<String>>,
) {
    match stmt {
        MirStmt::Let(local, MirExpr::ObjectNew { props, .. }, _) => {
            let idx = local.0 as usize;
            if idx < escape_status.len() && escape_status[idx] == Some(EscapeStatus::NotEscaped) {
                let keys: Vec<String> = props.iter().map(|(k, _)| k.clone()).collect();
                candidates.insert(*local, keys);
            }
        }
        // Recurse into nested blocks and control flow to find candidates at any depth.
        MirStmt::Block(children, _) => {
            for child in children {
                collect_candidates_in_stmt(child, escape_status, candidates);
            }
        }
        MirStmt::If {
            then_body,
            else_body,
            ..
        } => {
            for s in then_body {
                collect_candidates_in_stmt(s, escape_status, candidates);
            }
            for s in else_body {
                collect_candidates_in_stmt(s, escape_status, candidates);
            }
        }
        MirStmt::While { body, .. } => {
            for s in body {
                collect_candidates_in_stmt(s, escape_status, candidates);
            }
        }
        MirStmt::For { init, body, .. } => {
            if let Some(init_stmt) = init {
                collect_candidates_in_stmt(init_stmt, escape_status, candidates);
            }
            for s in body {
                collect_candidates_in_stmt(s, escape_status, candidates);
            }
        }
        MirStmt::DoWhile { body, .. } => {
            for s in body {
                collect_candidates_in_stmt(s, escape_status, candidates);
            }
        }
        MirStmt::ForIn { body, .. }
        | MirStmt::ForOf { body, .. }
        | MirStmt::ForAwaitOfLower { body, .. } => {
            for s in body {
                collect_candidates_in_stmt(s, escape_status, candidates);
            }
        }
        MirStmt::TryFinally {
            try_body,
            finally_body,
            ..
        } => {
            for s in try_body {
                collect_candidates_in_stmt(s, escape_status, candidates);
            }
            for s in finally_body {
                collect_candidates_in_stmt(s, escape_status, candidates);
            }
        }
        MirStmt::TryCatch {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            for s in try_body {
                collect_candidates_in_stmt(s, escape_status, candidates);
            }
            if let Some(body) = catch_body {
                for s in body {
                    collect_candidates_in_stmt(s, escape_status, candidates);
                }
            }
            if let Some(body) = finally_body {
                for s in body {
                    collect_candidates_in_stmt(s, escape_status, candidates);
                }
            }
        }
        MirStmt::Switch { cases, .. } => {
            for (_, body) in cases {
                for s in body {
                    collect_candidates_in_stmt(s, escape_status, candidates);
                }
            }
        }
        MirStmt::Labeled { body, .. } => {
            collect_candidates_in_stmt(body, escape_status, candidates);
        }
        MirStmt::Let(..)
        | MirStmt::Assign(..)
        | MirStmt::Expr(..)
        | MirStmt::Return(..)
        | MirStmt::Throw(..)
        | MirStmt::Yield(..)
        | MirStmt::Break { .. }
        | MirStmt::Continue { .. }
        | MirStmt::Export { .. }
        | MirStmt::ModuleExportsUpdate { .. }
        | MirStmt::ModuleExportsAssign { .. }
        | MirStmt::ClassDecl { .. } => {}
    }
}

// ---------------------------------------------------------------------------
// Dynamic access filter
// ---------------------------------------------------------------------------

/// Remove candidates that have computed/dynamic property access anywhere in
/// the body. Returns a map from object local to (key -> placeholder LocalId).
fn filter_dynamic(
    stmts: &[MirStmt],
    candidates: HashMap<LocalId, Vec<String>>,
    _replaced_set: &HashSet<LocalId>,
) -> HashMap<LocalId, HashMap<String, LocalId>> {
    let mut result: HashMap<LocalId, HashMap<String, LocalId>> = HashMap::new();

    'candidate: for (obj_local, keys) in candidates {
        // Check for dynamic access to this candidate anywhere in the body.
        if has_dynamic_access_in_stmts(stmts, obj_local) {
            continue 'candidate;
        }

        // Allocate placeholder LocalIds (will be filled after allocation).
        let mut key_map: HashMap<String, LocalId> = HashMap::new();
        for key in keys {
            key_map.insert(key, LocalId(usize::MAX)); // placeholder
        }
        result.insert(obj_local, key_map);
    }

    result
}

/// Check if a specific local has dynamic/computed property access in any
/// statement of the list.
fn has_dynamic_access_in_stmts(stmts: &[MirStmt], obj_local: LocalId) -> bool {
    stmts.iter().any(|s| stmt_has_dynamic_access(s, obj_local))
}

fn stmt_has_dynamic_access(stmt: &MirStmt, obj_local: LocalId) -> bool {
    match stmt {
        MirStmt::Block(children, _) => children
            .iter()
            .any(|s| stmt_has_dynamic_access(s, obj_local)),
        MirStmt::Let(_, expr, _) => expr_has_dynamic_access(expr, obj_local),
        MirStmt::Assign(_, expr, _) => expr_has_dynamic_access(expr, obj_local),
        MirStmt::Expr(expr, _) => expr_has_dynamic_access(expr, obj_local),
        MirStmt::Return(expr, _) => expr_has_dynamic_access(expr, obj_local),
        MirStmt::Throw(expr, _) => expr_has_dynamic_access(expr, obj_local),
        MirStmt::Yield(expr, _) => expr_has_dynamic_access(expr, obj_local),
        MirStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            expr_has_dynamic_access(condition, obj_local)
                || has_dynamic_access_in_stmts(then_body, obj_local)
                || has_dynamic_access_in_stmts(else_body, obj_local)
        }
        MirStmt::While {
            condition, body, ..
        } => {
            expr_has_dynamic_access(condition, obj_local)
                || has_dynamic_access_in_stmts(body, obj_local)
        }
        MirStmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            init.as_ref()
                .is_some_and(|i| stmt_has_dynamic_access(i, obj_local))
                || condition
                    .as_ref()
                    .is_some_and(|c| expr_has_dynamic_access(c, obj_local))
                || update
                    .as_ref()
                    .is_some_and(|u| expr_has_dynamic_access(u, obj_local))
                || has_dynamic_access_in_stmts(body, obj_local)
        }
        MirStmt::DoWhile {
            body, condition, ..
        } => {
            has_dynamic_access_in_stmts(body, obj_local)
                || expr_has_dynamic_access(condition, obj_local)
        }
        MirStmt::ForIn { iter, body, .. } | MirStmt::ForOf { iter, body, .. } => {
            expr_has_dynamic_access(iter, obj_local) || has_dynamic_access_in_stmts(body, obj_local)
        }
        MirStmt::ForAwaitOfLower { iter, body, .. } => {
            expr_has_dynamic_access(iter, obj_local) || has_dynamic_access_in_stmts(body, obj_local)
        }
        MirStmt::TryFinally {
            try_body,
            finally_body,
            ..
        } => {
            has_dynamic_access_in_stmts(try_body, obj_local)
                || has_dynamic_access_in_stmts(finally_body, obj_local)
        }
        MirStmt::TryCatch {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            has_dynamic_access_in_stmts(try_body, obj_local)
                || catch_body
                    .as_ref()
                    .is_some_and(|b| has_dynamic_access_in_stmts(b, obj_local))
                || finally_body
                    .as_ref()
                    .is_some_and(|b| has_dynamic_access_in_stmts(b, obj_local))
        }
        MirStmt::Switch { expr, cases, .. } => {
            expr_has_dynamic_access(expr, obj_local)
                || cases.iter().any(|(cond, body)| {
                    cond.as_ref()
                        .is_some_and(|c| expr_has_dynamic_access(c, obj_local))
                        || has_dynamic_access_in_stmts(body, obj_local)
                })
        }
        MirStmt::Labeled { body, .. } => stmt_has_dynamic_access(body, obj_local),
        MirStmt::Export { expr, .. } => expr_has_dynamic_access(expr, obj_local),
        MirStmt::ModuleExportsAssign { expr, .. } => expr_has_dynamic_access(expr, obj_local),
        MirStmt::Break { .. }
        | MirStmt::Continue { .. }
        | MirStmt::ModuleExportsUpdate { .. }
        | MirStmt::ClassDecl { .. } => false,
    }
}

/// Check if an expression (or any sub-expression) contains a dynamic/computed
/// property access on the given local.
fn expr_has_dynamic_access(expr: &MirExpr, obj_local: LocalId) -> bool {
    match expr {
        // Dynamic access targeting our candidate -> disqualify.
        MirExpr::PropertyGetDynamic { obj, .. } => {
            local_matches(obj, obj_local) || expr_has_dynamic_access(obj, obj_local)
        }
        MirExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => {
            local_matches(object, obj_local)
                || expr_has_dynamic_access(object, obj_local)
                || expr_has_dynamic_access(index, obj_local)
                || expr_has_dynamic_access(value, obj_local)
        }
        MirExpr::PropertyInDynamic { obj, key, .. } => {
            local_matches(obj, obj_local)
                || expr_has_dynamic_access(obj, obj_local)
                || expr_has_dynamic_access(key, obj_local)
        }
        MirExpr::PropertyDeleteDynamic { object, key, .. } => {
            local_matches(object, obj_local)
                || expr_has_dynamic_access(object, obj_local)
                || expr_has_dynamic_access(key, obj_local)
        }
        MirExpr::OptionalIndex { object, index, .. } => {
            local_matches(object, obj_local)
                || expr_has_dynamic_access(object, obj_local)
                || expr_has_dynamic_access(index, obj_local)
        }
        MirExpr::Index { object, index, .. } => {
            local_matches(object, obj_local)
                || expr_has_dynamic_access(object, obj_local)
                || expr_has_dynamic_access(index, obj_local)
        }

        // Recursive cases — walk children.
        MirExpr::Unary { expr: e, .. } => expr_has_dynamic_access(e, obj_local),
        MirExpr::Binary { left, right, .. } => {
            expr_has_dynamic_access(left, obj_local) || expr_has_dynamic_access(right, obj_local)
        }
        MirExpr::Call { args, .. } | MirExpr::RuntimeCall { args, .. } => {
            args.iter().any(|a| expr_has_dynamic_access(a, obj_local))
        }
        MirExpr::Assign { expr: e, .. } => expr_has_dynamic_access(e, obj_local),
        MirExpr::LogicalAssign { expr: e, .. } => expr_has_dynamic_access(e, obj_local),
        MirExpr::LogicalPropertyAssign { expr: e, .. } => expr_has_dynamic_access(e, obj_local),
        MirExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_has_dynamic_access(key, obj_local) || expr_has_dynamic_access(expr, obj_local)
        }
        MirExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            expr_has_dynamic_access(object, obj_local)
                || expr_has_dynamic_access(key, obj_local)
                || expr_has_dynamic_access(expr, obj_local)
        }
        MirExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_has_dynamic_access(object, obj_local) || expr_has_dynamic_access(expr, obj_local)
        }
        MirExpr::ArrayNew { elements, .. } => elements
            .iter()
            .any(|e| expr_has_dynamic_access(e, obj_local)),
        MirExpr::ArrayNewSparse { slots, .. } => slots.iter().any(|slot| match slot {
            super::types::MirArraySlot::Present(e) => expr_has_dynamic_access(e, obj_local),
            super::types::MirArraySlot::Hole => false,
        }),
        MirExpr::ArrayGet { arr, index, .. } => {
            expr_has_dynamic_access(arr, obj_local) || expr_has_dynamic_access(index, obj_local)
        }
        MirExpr::GetLength(e, _) => expr_has_dynamic_access(e, obj_local),
        MirExpr::ObjectNew { props, .. } => props
            .iter()
            .any(|(_, v)| expr_has_dynamic_access(v, obj_local)),
        MirExpr::ErrorNew { message, cause, .. } => {
            expr_has_dynamic_access(message, obj_local)
                || cause
                    .as_ref()
                    .is_some_and(|c| expr_has_dynamic_access(c, obj_local))
        }
        MirExpr::PropertyGet { obj, .. } | MirExpr::OptionalPropertyGet { obj, .. } => {
            expr_has_dynamic_access(obj, obj_local)
        }
        MirExpr::PropertySet { object, value, .. } => {
            expr_has_dynamic_access(object, obj_local) || expr_has_dynamic_access(value, obj_local)
        }
        MirExpr::PropertyDelete { object, .. } => expr_has_dynamic_access(object, obj_local),
        MirExpr::MethodCall { object, .. } => expr_has_dynamic_access(object, obj_local),
        MirExpr::PromiseGetValue { promise, .. } => expr_has_dynamic_access(promise, obj_local),
        MirExpr::EnvCellNew(e, _) => expr_has_dynamic_access(e, obj_local),
        MirExpr::EnvCellSet { expr: e, .. } => expr_has_dynamic_access(e, obj_local),
        MirExpr::New { args, .. } => args.iter().any(|a| expr_has_dynamic_access(a, obj_local)),
        MirExpr::Block { stmts, result, .. } => {
            has_dynamic_access_in_stmts(stmts, obj_local)
                || expr_has_dynamic_access(result, obj_local)
        }
        MirExpr::OptionalCall { callee, call, .. } => {
            expr_has_dynamic_access(callee, obj_local) || expr_has_dynamic_access(call, obj_local)
        }
        MirExpr::PropertyIn { obj, .. } => expr_has_dynamic_access(obj, obj_local),

        // Leaves — no sub-expressions.
        MirExpr::Number(..)
        | MirExpr::DecimalNumber(..)
        | MirExpr::BigIntLiteral { .. }
        | MirExpr::String(..)
        | MirExpr::Bool(..)
        | MirExpr::Null(..)
        | MirExpr::Undefined(..)
        | MirExpr::Local(..)
        | MirExpr::EnvCellGet(..)
        | MirExpr::This(..)
        | MirExpr::ClassPrototype(..)
        | MirExpr::BuiltinErrorPrototype(..)
        | MirExpr::ModuleLoad { .. }
        | MirExpr::ArrowFn { .. } => false,
    }
}

/// Check if an expression is `MirExpr::Local(id, _)` matching the target.
fn local_matches(expr: &MirExpr, target: LocalId) -> bool {
    matches!(expr, MirExpr::Local(id, _) if *id == target)
}

// ---------------------------------------------------------------------------
// Local allocation
// ---------------------------------------------------------------------------

/// Fill in real LocalIds for the placeholder mapping. Returns a map from
/// object local to (key -> scalar local).
fn fill_mapping(
    candidates: HashMap<LocalId, HashMap<String, LocalId>>,
    base: usize,
) -> HashMap<LocalId, HashMap<String, LocalId>> {
    let mut next = base;
    let mut result: HashMap<LocalId, HashMap<String, LocalId>> =
        HashMap::with_capacity(candidates.len());

    for (obj_local, key_map) in candidates {
        let mut filled: HashMap<String, LocalId> = HashMap::with_capacity(key_map.len());
        // Preserve insertion order by iterating keys in a consistent way.
        // We rely on HashMap iteration, which is fine for tests.
        for key in key_map.into_keys() {
            filled.insert(key, LocalId(next));
            next += 1;
        }
        result.insert(obj_local, filled);
    }

    result
}

// ---------------------------------------------------------------------------
// Rewriting
// ---------------------------------------------------------------------------

/// Rewrite a statement list: replace ObjectNew lets with individual property
/// lets, replace PropertyGet with Local, and replace PropertySet with Assign.
fn rewrite_stmts(
    stmts: &[MirStmt],
    replaced_set: &HashSet<LocalId>,
    mapping: &HashMap<LocalId, HashMap<String, LocalId>>,
    out: &mut Vec<MirStmt>,
) {
    for stmt in stmts {
        match stmt {
            // Replace `Let(local, ObjectNew { props, .. })` with individual
            // `Let(scalar_local, prop_value)` for each property.
            MirStmt::Let(local, MirExpr::ObjectNew { props, .. }, _span) => {
                if replaced_set.contains(local) {
                    if let Some(key_map) = mapping.get(local) {
                        for (key, prop_expr) in props {
                            if let Some(&scalar_local) = key_map.get(key) {
                                let span = span_of(prop_expr);
                                // Clone the property expression and rewrite any
                                // PropertyGet inside it that references a replaced local.
                                let rewritten = rewrite_expr(prop_expr, replaced_set, mapping);
                                out.push(MirStmt::Let(scalar_local, rewritten, span));
                            }
                        }
                    }
                    // Drop the original ObjectNew let — it has been scalar-replaced.
                } else {
                    // Not a candidate — pass through with sub-expression rewriting.
                    let rewritten = rewrite_expr_in_stmt(stmt, replaced_set, mapping);
                    out.push(rewritten);
                }
            }

            // Transform `Expr(PropertySet { object, key, value })` for replaced objects
            // into `Assign(scalar_local, value)`.
            MirStmt::Expr(
                MirExpr::PropertySet {
                    object,
                    key,
                    value,
                    span,
                },
                _stmt_span,
            ) => {
                if let MirExpr::Local(obj_local, _) = object.as_ref() {
                    if replaced_set.contains(obj_local) {
                        if let Some(key_map) = mapping.get(obj_local) {
                            if let Some(&scalar_local) = key_map.get(key.as_str()) {
                                let rewritten_value = rewrite_expr(value, replaced_set, mapping);
                                out.push(MirStmt::Assign(scalar_local, rewritten_value, *span));
                                continue;
                            }
                        }
                    }
                }
                // Fall through — rewrite sub-expressions normally.
                let rewritten = rewrite_expr_in_stmt(stmt, replaced_set, mapping);
                out.push(rewritten);
            }

            // All other statements — rewrite sub-expressions.
            _ => {
                let rewritten = rewrite_expr_in_stmt(stmt, replaced_set, mapping);
                out.push(rewritten);
            }
        }
    }
}

/// Rewrite expressions within a statement, handling recursion into nested
/// statement lists (blocks, if/else, loops, etc.).
fn rewrite_expr_in_stmt(
    stmt: &MirStmt,
    replaced_set: &HashSet<LocalId>,
    mapping: &HashMap<LocalId, HashMap<String, LocalId>>,
) -> MirStmt {
    match stmt {
        MirStmt::Block(children, span) => {
            let mut new_children = Vec::with_capacity(children.len());
            rewrite_stmts(children, replaced_set, mapping, &mut new_children);
            MirStmt::Block(new_children, *span)
        }
        MirStmt::Let(local, expr, span) => {
            MirStmt::Let(*local, rewrite_expr(expr, replaced_set, mapping), *span)
        }
        MirStmt::Assign(local, expr, span) => {
            MirStmt::Assign(*local, rewrite_expr(expr, replaced_set, mapping), *span)
        }
        MirStmt::Expr(expr, span) => {
            MirStmt::Expr(rewrite_expr(expr, replaced_set, mapping), *span)
        }
        MirStmt::Return(expr, span) => {
            MirStmt::Return(rewrite_expr(expr, replaced_set, mapping), *span)
        }
        MirStmt::Throw(expr, span) => {
            MirStmt::Throw(rewrite_expr(expr, replaced_set, mapping), *span)
        }
        MirStmt::Yield(expr, span) => {
            MirStmt::Yield(rewrite_expr(expr, replaced_set, mapping), *span)
        }
        MirStmt::If {
            condition,
            then_body,
            else_body,
            span,
        } => {
            let mut new_then = Vec::with_capacity(then_body.len());
            rewrite_stmts(then_body, replaced_set, mapping, &mut new_then);
            let mut new_else = Vec::with_capacity(else_body.len());
            rewrite_stmts(else_body, replaced_set, mapping, &mut new_else);
            MirStmt::If {
                condition: rewrite_expr(condition, replaced_set, mapping),
                then_body: new_then,
                else_body: new_else,
                span: *span,
            }
        }
        MirStmt::While {
            condition,
            body,
            span,
        } => {
            let mut new_body = Vec::with_capacity(body.len());
            rewrite_stmts(body, replaced_set, mapping, &mut new_body);
            MirStmt::While {
                condition: rewrite_expr(condition, replaced_set, mapping),
                body: new_body,
                span: *span,
            }
        }
        MirStmt::For {
            init,
            condition,
            update,
            body,
            span,
        } => {
            let new_init = init.as_ref().map(|i| {
                let mut buf = Vec::new();
                rewrite_stmts(&[(**i).clone()], replaced_set, mapping, &mut buf);
                Box::new(buf.into_iter().next().unwrap())
            });
            let mut new_body = Vec::with_capacity(body.len());
            rewrite_stmts(body, replaced_set, mapping, &mut new_body);
            MirStmt::For {
                init: new_init,
                condition: condition
                    .as_ref()
                    .map(|c| rewrite_expr(c, replaced_set, mapping)),
                update: update
                    .as_ref()
                    .map(|u| rewrite_expr(u, replaced_set, mapping)),
                body: new_body,
                span: *span,
            }
        }
        MirStmt::DoWhile {
            body,
            condition,
            span,
        } => {
            let mut new_body = Vec::with_capacity(body.len());
            rewrite_stmts(body, replaced_set, mapping, &mut new_body);
            MirStmt::DoWhile {
                body: new_body,
                condition: rewrite_expr(condition, replaced_set, mapping),
                span: *span,
            }
        }
        MirStmt::ForIn {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            span,
        } => {
            let mut new_body = Vec::with_capacity(body.len());
            rewrite_stmts(body, replaced_set, mapping, &mut new_body);
            MirStmt::ForIn {
                var: *var,
                iter: rewrite_expr(iter, replaced_set, mapping),
                iter_local: *iter_local,
                index_local: *index_local,
                len_local: *len_local,
                body: new_body,
                span: *span,
            }
        }
        MirStmt::ForOf {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            span,
        } => {
            let mut new_body = Vec::with_capacity(body.len());
            rewrite_stmts(body, replaced_set, mapping, &mut new_body);
            MirStmt::ForOf {
                var: *var,
                iter: rewrite_expr(iter, replaced_set, mapping),
                iter_local: *iter_local,
                index_local: *index_local,
                len_local: *len_local,
                body: new_body,
                span: *span,
            }
        }
        MirStmt::ForAwaitOfLower {
            var,
            iter,
            async_iter_local,
            next_result_local,
            done_local,
            value_local,
            body,
            span,
        } => {
            let mut new_body = Vec::with_capacity(body.len());
            rewrite_stmts(body, replaced_set, mapping, &mut new_body);
            MirStmt::ForAwaitOfLower {
                var: *var,
                iter: rewrite_expr(iter, replaced_set, mapping),
                async_iter_local: *async_iter_local,
                next_result_local: *next_result_local,
                done_local: *done_local,
                value_local: *value_local,
                body: new_body,
                span: *span,
            }
        }
        MirStmt::TryFinally {
            try_body,
            finally_body,
            span,
        } => {
            let mut new_try = Vec::with_capacity(try_body.len());
            rewrite_stmts(try_body, replaced_set, mapping, &mut new_try);
            let mut new_finally = Vec::with_capacity(finally_body.len());
            rewrite_stmts(finally_body, replaced_set, mapping, &mut new_finally);
            MirStmt::TryFinally {
                try_body: new_try,
                finally_body: new_finally,
                span: *span,
            }
        }
        MirStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
            span,
        } => {
            let mut new_try = Vec::with_capacity(try_body.len());
            rewrite_stmts(try_body, replaced_set, mapping, &mut new_try);
            let new_catch = catch_body.as_ref().map(|b| {
                let mut buf = Vec::with_capacity(b.len());
                rewrite_stmts(b, replaced_set, mapping, &mut buf);
                buf
            });
            let new_finally = finally_body.as_ref().map(|b| {
                let mut buf = Vec::with_capacity(b.len());
                rewrite_stmts(b, replaced_set, mapping, &mut buf);
                buf
            });
            MirStmt::TryCatch {
                try_body: new_try,
                catch_var: *catch_var,
                catch_body: new_catch,
                finally_body: new_finally,
                span: *span,
            }
        }
        MirStmt::Switch { expr, cases, span } => {
            let new_cases: Vec<(Option<MirExpr>, Vec<MirStmt>)> = cases
                .iter()
                .map(|(cond, body)| {
                    let new_cond = cond
                        .as_ref()
                        .map(|c| rewrite_expr(c, replaced_set, mapping));
                    let mut new_body = Vec::with_capacity(body.len());
                    rewrite_stmts(body, replaced_set, mapping, &mut new_body);
                    (new_cond, new_body)
                })
                .collect();
            MirStmt::Switch {
                expr: rewrite_expr(expr, replaced_set, mapping),
                cases: new_cases,
                span: *span,
            }
        }
        MirStmt::Labeled { label, body, span } => {
            let mut buf = Vec::new();
            rewrite_stmts(&[(**body).clone()], replaced_set, mapping, &mut buf);
            let new_body = Box::new(buf.into_iter().next().unwrap());
            MirStmt::Labeled {
                label: label.clone(),
                body: new_body,
                span: *span,
            }
        }
        MirStmt::Export { name, expr, span } => MirStmt::Export {
            name: name.clone(),
            expr: rewrite_expr(expr, replaced_set, mapping),
            span: *span,
        },
        MirStmt::ModuleExportsAssign { expr, span } => MirStmt::ModuleExportsAssign {
            expr: rewrite_expr(expr, replaced_set, mapping),
            span: *span,
        },
        MirStmt::Break { .. }
        | MirStmt::Continue { .. }
        | MirStmt::ModuleExportsUpdate { .. }
        | MirStmt::ClassDecl { .. } => stmt.clone(),
    }
}

/// Rewrite an expression tree, replacing `PropertyGet { obj: Local(id), key }`
/// with `Local(scalar_local)` when `id` is in `replaced_set`.
fn rewrite_expr(
    expr: &MirExpr,
    replaced_set: &HashSet<LocalId>,
    mapping: &HashMap<LocalId, HashMap<String, LocalId>>,
) -> MirExpr {
    match expr {
        // Replace PropertyGet with direct Local reference.
        MirExpr::PropertyGet { obj, key, span } => {
            if let MirExpr::Local(obj_local, _) = obj.as_ref() {
                if replaced_set.contains(obj_local) {
                    if let Some(key_map) = mapping.get(obj_local) {
                        if let Some(&scalar_local) = key_map.get(key.as_str()) {
                            return MirExpr::Local(scalar_local, *span);
                        }
                    }
                }
            }
            // If not a match, recurse into obj (which might itself be a PropertyGet
            // of another replaced local, e.g., chain access like obj.x.y).
            MirExpr::PropertyGet {
                obj: Box::new(rewrite_expr(obj, replaced_set, mapping)),
                key: key.clone(),
                span: *span,
            }
        }

        MirExpr::OptionalPropertyGet { obj, key, span } => MirExpr::OptionalPropertyGet {
            obj: Box::new(rewrite_expr(obj, replaced_set, mapping)),
            key: key.clone(),
            span: *span,
        },

        // PropertySet as a sub-expression (rare but handle it): rewrite children.
        MirExpr::PropertySet {
            object,
            key,
            value,
            span,
        } => MirExpr::PropertySet {
            object: Box::new(rewrite_expr(object, replaced_set, mapping)),
            key: key.clone(),
            value: Box::new(rewrite_expr(value, replaced_set, mapping)),
            span: *span,
        },

        // All other expressions — recurse into children.
        MirExpr::Number(..)
        | MirExpr::DecimalNumber(..)
        | MirExpr::BigIntLiteral { .. }
        | MirExpr::String(..)
        | MirExpr::Bool(..)
        | MirExpr::Null(..)
        | MirExpr::Undefined(..)
        | MirExpr::Local(..)
        | MirExpr::This(..)
        | MirExpr::EnvCellGet(..)
        | MirExpr::ClassPrototype(..)
        | MirExpr::BuiltinErrorPrototype(..)
        | MirExpr::ModuleLoad { .. }
        | MirExpr::ArrowFn { .. } => expr.clone(),

        MirExpr::Unary { op, expr: e, span } => MirExpr::Unary {
            op: *op,
            expr: Box::new(rewrite_expr(e, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::Binary {
            left,
            op,
            right,
            span,
        } => MirExpr::Binary {
            left: Box::new(rewrite_expr(left, replaced_set, mapping)),
            op: *op,
            right: Box::new(rewrite_expr(right, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::PropertyIn { obj, key, span } => MirExpr::PropertyIn {
            obj: Box::new(rewrite_expr(obj, replaced_set, mapping)),
            key: key.clone(),
            span: *span,
        },
        MirExpr::PropertyInDynamic { obj, key, span } => MirExpr::PropertyInDynamic {
            obj: Box::new(rewrite_expr(obj, replaced_set, mapping)),
            key: Box::new(rewrite_expr(key, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::Call { kind, args, span } => MirExpr::Call {
            kind: *kind,
            args: args
                .iter()
                .map(|a| rewrite_expr(a, replaced_set, mapping))
                .collect(),
            span: *span,
        },
        MirExpr::RuntimeCall {
            intrinsic,
            args,
            span,
        } => MirExpr::RuntimeCall {
            intrinsic: *intrinsic,
            args: args
                .iter()
                .map(|a| rewrite_expr(a, replaced_set, mapping))
                .collect(),
            span: *span,
        },
        MirExpr::Assign {
            local,
            expr: e,
            span,
        } => MirExpr::Assign {
            local: *local,
            expr: Box::new(rewrite_expr(e, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::LogicalAssign {
            local,
            op,
            expr: e,
            span,
        } => MirExpr::LogicalAssign {
            local: *local,
            op: *op,
            expr: Box::new(rewrite_expr(e, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::LogicalPropertyAssign {
            object,
            key,
            op,
            expr: e,
            span,
        } => MirExpr::LogicalPropertyAssign {
            object: *object,
            key: key.clone(),
            op: *op,
            expr: Box::new(rewrite_expr(e, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::LogicalComputedPropertyAssign {
            object,
            key,
            op,
            expr: e,
            span,
        } => MirExpr::LogicalComputedPropertyAssign {
            object: *object,
            key: Box::new(rewrite_expr(key, replaced_set, mapping)),
            op: *op,
            expr: Box::new(rewrite_expr(e, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::LogicalComputedMemberAssign {
            object,
            key,
            op,
            expr: e,
            span,
        } => MirExpr::LogicalComputedMemberAssign {
            object: Box::new(rewrite_expr(object, replaced_set, mapping)),
            key: Box::new(rewrite_expr(key, replaced_set, mapping)),
            op: *op,
            expr: Box::new(rewrite_expr(e, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::LogicalMemberAssign {
            object,
            key,
            op,
            expr: e,
            span,
        } => MirExpr::LogicalMemberAssign {
            object: Box::new(rewrite_expr(object, replaced_set, mapping)),
            key: key.clone(),
            op: *op,
            expr: Box::new(rewrite_expr(e, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::ArrayNew { elements, span } => MirExpr::ArrayNew {
            elements: elements
                .iter()
                .map(|e| rewrite_expr(e, replaced_set, mapping))
                .collect(),
            span: *span,
        },
        MirExpr::ArrayNewSparse { slots, span } => {
            let new_slots: Vec<super::types::MirArraySlot> = slots
                .iter()
                .map(|slot| match slot {
                    super::types::MirArraySlot::Present(e) => {
                        super::types::MirArraySlot::Present(rewrite_expr(e, replaced_set, mapping))
                    }
                    super::types::MirArraySlot::Hole => super::types::MirArraySlot::Hole,
                })
                .collect();
            MirExpr::ArrayNewSparse {
                slots: new_slots,
                span: *span,
            }
        }
        MirExpr::ArrayGet { arr, index, span } => MirExpr::ArrayGet {
            arr: Box::new(rewrite_expr(arr, replaced_set, mapping)),
            index: Box::new(rewrite_expr(index, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::Index {
            object,
            index,
            span,
        } => MirExpr::Index {
            object: Box::new(rewrite_expr(object, replaced_set, mapping)),
            index: Box::new(rewrite_expr(index, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::GetLength(e, span) => {
            MirExpr::GetLength(Box::new(rewrite_expr(e, replaced_set, mapping)), *span)
        }
        MirExpr::ObjectNew {
            props,
            non_enumerable,
            span,
        } => {
            let new_props: Vec<(String, MirExpr)> = props
                .iter()
                .map(|(k, v)| (k.clone(), rewrite_expr(v, replaced_set, mapping)))
                .collect();
            MirExpr::ObjectNew {
                props: new_props,
                non_enumerable: *non_enumerable,
                span: *span,
            }
        }
        MirExpr::ErrorNew {
            constructor,
            message,
            cause,
            span,
        } => MirExpr::ErrorNew {
            constructor: *constructor,
            message: Box::new(rewrite_expr(message, replaced_set, mapping)),
            cause: cause
                .as_ref()
                .map(|c| Box::new(rewrite_expr(c, replaced_set, mapping))),
            span: *span,
        },
        MirExpr::PropertyGetDynamic { obj, key, span } => MirExpr::PropertyGetDynamic {
            obj: Box::new(rewrite_expr(obj, replaced_set, mapping)),
            key: Box::new(rewrite_expr(key, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::PropertySetDynamic {
            object,
            index,
            value,
            span,
        } => MirExpr::PropertySetDynamic {
            object: Box::new(rewrite_expr(object, replaced_set, mapping)),
            index: Box::new(rewrite_expr(index, replaced_set, mapping)),
            value: Box::new(rewrite_expr(value, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::PropertyDelete { object, key, span } => MirExpr::PropertyDelete {
            object: Box::new(rewrite_expr(object, replaced_set, mapping)),
            key: key.clone(),
            span: *span,
        },
        MirExpr::PropertyDeleteDynamic { object, key, span } => MirExpr::PropertyDeleteDynamic {
            object: Box::new(rewrite_expr(object, replaced_set, mapping)),
            key: Box::new(rewrite_expr(key, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::MethodCall {
            object,
            method,
            span,
        } => MirExpr::MethodCall {
            object: Box::new(rewrite_expr(object, replaced_set, mapping)),
            method: method.clone(),
            span: *span,
        },
        MirExpr::PromiseGetValue { promise, span } => MirExpr::PromiseGetValue {
            promise: Box::new(rewrite_expr(promise, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::EnvCellNew(e, span) => {
            MirExpr::EnvCellNew(Box::new(rewrite_expr(e, replaced_set, mapping)), *span)
        }
        MirExpr::EnvCellSet {
            cell,
            expr: e,
            span,
        } => MirExpr::EnvCellSet {
            cell: *cell,
            expr: Box::new(rewrite_expr(e, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::New {
            constructor,
            prototype,
            args,
            base_local,
            private_brand,
            private_slot_count,
            span,
        } => MirExpr::New {
            constructor: *constructor,
            prototype: prototype.clone(),
            args: args
                .iter()
                .map(|a| rewrite_expr(a, replaced_set, mapping))
                .collect(),
            base_local: *base_local,
            private_brand: *private_brand,
            private_slot_count: *private_slot_count,
            span: *span,
        },
        MirExpr::Block {
            stmts,
            result,
            span,
        } => {
            let mut new_stmts = Vec::with_capacity(stmts.len());
            rewrite_stmts(stmts, replaced_set, mapping, &mut new_stmts);
            MirExpr::Block {
                stmts: new_stmts,
                result: Box::new(rewrite_expr(result, replaced_set, mapping)),
                span: *span,
            }
        }
        MirExpr::OptionalIndex {
            object,
            index,
            span,
        } => MirExpr::OptionalIndex {
            object: Box::new(rewrite_expr(object, replaced_set, mapping)),
            index: Box::new(rewrite_expr(index, replaced_set, mapping)),
            span: *span,
        },
        MirExpr::OptionalCall { callee, call, span } => MirExpr::OptionalCall {
            callee: Box::new(rewrite_expr(callee, replaced_set, mapping)),
            call: Box::new(rewrite_expr(call, replaced_set, mapping)),
            span: *span,
        },
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract the span from a MirExpr, defaulting to a zero-span if unavailable.
fn span_of(expr: &MirExpr) -> ts2wasm_source::Span {
    match expr {
        MirExpr::Number(_, s)
        | MirExpr::DecimalNumber(_, s)
        | MirExpr::String(_, s)
        | MirExpr::Bool(_, s)
        | MirExpr::Null(s)
        | MirExpr::Undefined(s)
        | MirExpr::Local(_, s)
        | MirExpr::This(s)
        | MirExpr::EnvCellGet(_, s)
        | MirExpr::ClassPrototype(_, s)
        | MirExpr::BuiltinErrorPrototype(_, s) => *s,
        MirExpr::BigIntLiteral { span: s, .. }
        | MirExpr::EnvCellNew(_, s)
        | MirExpr::Unary { span: s, .. }
        | MirExpr::Binary { span: s, .. }
        | MirExpr::PropertyIn { span: s, .. }
        | MirExpr::PropertyInDynamic { span: s, .. }
        | MirExpr::Call { span: s, .. }
        | MirExpr::Assign { span: s, .. }
        | MirExpr::LogicalAssign { span: s, .. }
        | MirExpr::LogicalPropertyAssign { span: s, .. }
        | MirExpr::LogicalComputedPropertyAssign { span: s, .. }
        | MirExpr::LogicalComputedMemberAssign { span: s, .. }
        | MirExpr::LogicalMemberAssign { span: s, .. }
        | MirExpr::ArrayNew { span: s, .. }
        | MirExpr::ArrayNewSparse { span: s, .. }
        | MirExpr::ArrayGet { span: s, .. }
        | MirExpr::Index { span: s, .. }
        | MirExpr::GetLength(_, s)
        | MirExpr::ObjectNew { span: s, .. }
        | MirExpr::ErrorNew { span: s, .. }
        | MirExpr::PropertyGet { span: s, .. }
        | MirExpr::OptionalPropertyGet { span: s, .. }
        | MirExpr::PropertyGetDynamic { span: s, .. }
        | MirExpr::OptionalIndex { span: s, .. }
        | MirExpr::OptionalCall { span: s, .. }
        | MirExpr::MethodCall { span: s, .. }
        | MirExpr::PromiseGetValue { span: s, .. }
        | MirExpr::RuntimeCall { span: s, .. }
        | MirExpr::PropertySet { span: s, .. }
        | MirExpr::PropertyDelete { span: s, .. }
        | MirExpr::PropertyDeleteDynamic { span: s, .. }
        | MirExpr::PropertySetDynamic { span: s, .. }
        | MirExpr::New { span: s, .. }
        | MirExpr::Block { span: s, .. }
        | MirExpr::ArrowFn { span: s, .. }
        | MirExpr::EnvCellSet { span: s, .. }
        | MirExpr::ModuleLoad { span: s, .. } => *s,
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

    fn span() -> Span {
        Span { start: 0, end: 0 }
    }

    /// Helper: create a small program with a single function body and run
    /// scalar replacement on it.
    fn run_scalar_replace(
        body: Vec<MirStmt>,
        escape_status: Vec<Option<EscapeStatus>>,
    ) -> Vec<MirStmt> {
        let mut locals: Vec<LocalId> = (0..escape_status.len() as u32)
            .map(|i| LocalId(i as usize))
            .collect();
        let mut escape = escape_status;

        let mut stmts = body;
        let esc_clone = escape.clone();
        scalar_replace_body(&mut stmts, &esc_clone, &mut locals, &mut escape);
        stmts
    }

    // -----------------------------------------------------------------------
    // Test: simple object with two properties, both read
    // -----------------------------------------------------------------------

    #[test]
    fn scalar_replace_simple_read_only() {
        // let obj = { x: 1, y: 2 };
        // let a = obj.x;
        // let b = obj.y;
        let body = vec![
            MirStmt::Let(
                LocalId(0),
                MirExpr::ObjectNew {
                    props: vec![
                        ("x".to_string(), MirExpr::Number(1, span())),
                        ("y".to_string(), MirExpr::Number(2, span())),
                    ],
                    non_enumerable: 0,
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(
                LocalId(1),
                MirExpr::PropertyGet {
                    obj: Box::new(MirExpr::Local(LocalId(0), span())),
                    key: "x".to_string(),
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(
                LocalId(2),
                MirExpr::PropertyGet {
                    obj: Box::new(MirExpr::Local(LocalId(0), span())),
                    key: "y".to_string(),
                    span: span(),
                },
                span(),
            ),
        ];

        let escape_status = vec![
            Some(EscapeStatus::NotEscaped), // local 0: obj
            Some(EscapeStatus::NotEscaped), // local 1: a
            Some(EscapeStatus::NotEscaped), // local 2: b
        ];

        let result = run_scalar_replace(body, escape_status);

        // After scalar replacement:
        // let _0 = 1;    // obj.x
        // let _1 = 2;    // obj.y
        // let a = _0;    // a = obj.x -> a = scalar_x
        // let b = _1;    // b = obj.y -> b = scalar_y
        assert_eq!(result.len(), 4, "should have 4 statements");

        // First stmt: let scalar_0 = 1 (property "x")
        assert!(
            matches!(&result[0], MirStmt::Let(local, MirExpr::Number(1, _), _) if local.0 >= 3),
            "first stmt should be let scalar = 1"
        );

        // Second stmt: let scalar_1 = 2 (property "y")
        assert!(
            matches!(&result[1], MirStmt::Let(local, MirExpr::Number(2, _), _) if local.0 >= 3),
            "second stmt should be let scalar = 2"
        );

        // Third stmt: let a = scalar_x  (PropertyGet replaced with Local)
        assert!(
            matches!(
                &result[2],
                MirStmt::Let(LocalId(1), MirExpr::Local(_, _), _)
            ),
            "third stmt should be let a = scalar_x"
        );

        // Fourth stmt: let b = scalar_y
        assert!(
            matches!(
                &result[3],
                MirStmt::Let(LocalId(2), MirExpr::Local(_, _), _)
            ),
            "fourth stmt should be let b = scalar_y"
        );

        // The scalar locals should be distinct.
        if let MirStmt::Let(_, MirExpr::Local(scalar_x, _), _) = &result[2] {
            if let MirStmt::Let(_, MirExpr::Local(scalar_y, _), _) = &result[3] {
                assert_ne!(
                    scalar_x, scalar_y,
                    "scalar locals for different keys should be distinct"
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Test: object with one property written then read
    // -----------------------------------------------------------------------

    #[test]
    fn scalar_replace_write_then_read() {
        // let obj = { x: 1 };
        // obj.x = 42;
        // let a = obj.x;
        let body = vec![
            MirStmt::Let(
                LocalId(0),
                MirExpr::ObjectNew {
                    props: vec![("x".to_string(), MirExpr::Number(1, span()))],
                    non_enumerable: 0,
                    span: span(),
                },
                span(),
            ),
            MirStmt::Expr(
                MirExpr::PropertySet {
                    object: Box::new(MirExpr::Local(LocalId(0), span())),
                    key: "x".to_string(),
                    value: Box::new(MirExpr::Number(42, span())),
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(
                LocalId(1),
                MirExpr::PropertyGet {
                    obj: Box::new(MirExpr::Local(LocalId(0), span())),
                    key: "x".to_string(),
                    span: span(),
                },
                span(),
            ),
        ];

        let escape_status = vec![
            Some(EscapeStatus::NotEscaped), // local 0: obj
            Some(EscapeStatus::NotEscaped), // local 1: a
        ];

        let result = run_scalar_replace(body, escape_status);

        // After scalar replacement:
        // let _0 = 1;     // property "x" initial value
        // _0 = 42;        // Assign to scalar local
        // let a = _0;     // read from scalar local
        assert_eq!(result.len(), 3, "should have 3 statements");

        // First stmt: let scalar = 1
        assert!(
            matches!(&result[0], MirStmt::Let(_, MirExpr::Number(1, _), _)),
            "first stmt should be let scalar = 1"
        );

        // Second stmt: assign scalar = 42 (PropertySet replaced with Assign)
        assert!(
            matches!(&result[1], MirStmt::Assign(_, MirExpr::Number(42, _), _)),
            "second stmt should be assign scalar = 42"
        );

        // Third stmt: let a = scalar (PropertyGet replaced with Local)
        assert!(
            matches!(
                &result[2],
                MirStmt::Let(LocalId(1), MirExpr::Local(_, _), _)
            ),
            "third stmt should be let a = scalar"
        );
    }

    // -----------------------------------------------------------------------
    // Test: object that escapes should NOT be transformed
    // -----------------------------------------------------------------------

    #[test]
    fn scalar_replace_escaped_object_not_transformed() {
        // let obj = { x: 1 };
        // return obj;   <- obj escapes!
        let body = vec![
            MirStmt::Let(
                LocalId(0),
                MirExpr::ObjectNew {
                    props: vec![("x".to_string(), MirExpr::Number(1, span()))],
                    non_enumerable: 0,
                    span: span(),
                },
                span(),
            ),
            MirStmt::Return(MirExpr::Local(LocalId(0), span()), span()),
        ];

        let escape_status = vec![Some(EscapeStatus::Escaped)];

        let result = run_scalar_replace(body, escape_status);

        // Body should be unchanged (2 statements, same structure).
        assert_eq!(result.len(), 2, "escaped object body should be unchanged");
        assert!(
            matches!(
                &result[0],
                MirStmt::Let(LocalId(0), MirExpr::ObjectNew { .. }, _)
            ),
            "first stmt should still be let obj = ObjectNew"
        );
        assert!(
            matches!(
                &result[1],
                MirStmt::Return(MirExpr::Local(LocalId(0), _), _)
            ),
            "second stmt should still be return obj"
        );
    }

    // -----------------------------------------------------------------------
    // Test: empty object should remain as-is
    // -----------------------------------------------------------------------

    #[test]
    fn scalar_replace_empty_object() {
        // let obj = {};
        let body = vec![MirStmt::Let(
            LocalId(0),
            MirExpr::ObjectNew {
                props: vec![],
                non_enumerable: 0,
                span: span(),
            },
            span(),
        )];

        let escape_status = vec![Some(EscapeStatus::NotEscaped)];

        let result = run_scalar_replace(body, escape_status);

        // Empty object has no properties to scalar-replace, so the original
        // Let should remain as-is.
        assert_eq!(result.len(), 1, "empty object body should be unchanged");
        assert!(
            matches!(&result[0], MirStmt::Let(LocalId(0), MirExpr::ObjectNew { props, .. }, _) if props.is_empty()),
            "empty ObjectNew should remain unchanged"
        );
    }

    // -----------------------------------------------------------------------
    // Test: object with dynamic property access is skipped
    // -----------------------------------------------------------------------

    #[test]
    fn scalar_replace_dynamic_access_skipped() {
        // let obj = { x: 1 };
        // let k = "x";
        // let a = obj[k];  // dynamic access -> should NOT transform
        let body = vec![
            MirStmt::Let(
                LocalId(0),
                MirExpr::ObjectNew {
                    props: vec![("x".to_string(), MirExpr::Number(1, span()))],
                    non_enumerable: 0,
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(LocalId(1), MirExpr::String("x".to_string(), span()), span()),
            MirStmt::Let(
                LocalId(2),
                MirExpr::PropertyGetDynamic {
                    obj: Box::new(MirExpr::Local(LocalId(0), span())),
                    key: Box::new(MirExpr::Local(LocalId(1), span())),
                    span: span(),
                },
                span(),
            ),
        ];

        let escape_status = vec![
            Some(EscapeStatus::NotEscaped),
            Some(EscapeStatus::NotEscaped),
            Some(EscapeStatus::NotEscaped),
        ];

        let result = run_scalar_replace(body, escape_status);

        // Should be unchanged because of dynamic access.
        assert_eq!(result.len(), 3, "dynamic access body should be unchanged");
        assert!(
            matches!(
                &result[0],
                MirStmt::Let(LocalId(0), MirExpr::ObjectNew { .. }, _)
            ),
            "object with dynamic access should remain as ObjectNew"
        );
    }

    // -----------------------------------------------------------------------
    // Test: multiple candidate objects
    // -----------------------------------------------------------------------

    #[test]
    fn scalar_replace_multiple_objects() {
        // let obj1 = { a: 10, b: 20 };
        // let obj2 = { c: 30 };
        // let x = obj1.a;
        // let y = obj2.c;
        let body = vec![
            MirStmt::Let(
                LocalId(0),
                MirExpr::ObjectNew {
                    props: vec![
                        ("a".to_string(), MirExpr::Number(10, span())),
                        ("b".to_string(), MirExpr::Number(20, span())),
                    ],
                    non_enumerable: 0,
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(
                LocalId(1),
                MirExpr::ObjectNew {
                    props: vec![("c".to_string(), MirExpr::Number(30, span()))],
                    non_enumerable: 0,
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(
                LocalId(2),
                MirExpr::PropertyGet {
                    obj: Box::new(MirExpr::Local(LocalId(0), span())),
                    key: "a".to_string(),
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(
                LocalId(3),
                MirExpr::PropertyGet {
                    obj: Box::new(MirExpr::Local(LocalId(1), span())),
                    key: "c".to_string(),
                    span: span(),
                },
                span(),
            ),
        ];

        let escape_status = vec![
            Some(EscapeStatus::NotEscaped),
            Some(EscapeStatus::NotEscaped),
            Some(EscapeStatus::NotEscaped),
            Some(EscapeStatus::NotEscaped),
        ];

        let result = run_scalar_replace(body, escape_status);

        // After replacement:
        // let _0 = 10;   // obj1.a
        // let _1 = 20;   // obj1.b
        // let _2 = 30;   // obj2.c
        // let x = _0;    // obj1.a
        // let y = _2;    // obj2.c
        assert_eq!(result.len(), 5, "should have 5 statements");

        // Check that object-new lets are gone.
        for stmt in &result {
            assert!(
                !matches!(stmt, MirStmt::Let(_, MirExpr::ObjectNew { .. }, _)),
                "no ObjectNew should remain after scalar replacement"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test: PropertyGet inside a nested expression (e.g., binary op)
    // -----------------------------------------------------------------------

    #[test]
    fn scalar_replace_property_get_in_binary() {
        // let obj = { x: 5, y: 3 };
        // let s = obj.x + obj.y;
        let body = vec![
            MirStmt::Let(
                LocalId(0),
                MirExpr::ObjectNew {
                    props: vec![
                        ("x".to_string(), MirExpr::Number(5, span())),
                        ("y".to_string(), MirExpr::Number(3, span())),
                    ],
                    non_enumerable: 0,
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(
                LocalId(1),
                MirExpr::Binary {
                    left: Box::new(MirExpr::PropertyGet {
                        obj: Box::new(MirExpr::Local(LocalId(0), span())),
                        key: "x".to_string(),
                        span: span(),
                    }),
                    op: crate::lowered::LoweredBinaryOp::Add,
                    right: Box::new(MirExpr::PropertyGet {
                        obj: Box::new(MirExpr::Local(LocalId(0), span())),
                        key: "y".to_string(),
                        span: span(),
                    }),
                    span: span(),
                },
                span(),
            ),
        ];

        let escape_status = vec![
            Some(EscapeStatus::NotEscaped),
            Some(EscapeStatus::NotEscaped),
        ];

        let result = run_scalar_replace(body, escape_status);

        // After replacement:
        // let _0 = 5;
        // let _1 = 3;
        // let s = _0 + _1;
        assert_eq!(result.len(), 3, "should have 3 statements");

        // Third stmt: let s = scalar_x + scalar_y
        assert!(
            matches!(
                &result[2],
                MirStmt::Let(LocalId(1), MirExpr::Binary { .. }, _)
            ),
            "third stmt should be let s = binary"
        );
        if let MirStmt::Let(_, MirExpr::Binary { left, right, .. }, _) = &result[2] {
            assert!(
                matches!(left.as_ref(), MirExpr::Local(_, _)),
                "left should be scalar local"
            );
            assert!(
                matches!(right.as_ref(), MirExpr::Local(_, _)),
                "right should be scalar local"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test: two objects, one escaped and one not
    // -----------------------------------------------------------------------

    #[test]
    fn scalar_replace_mixed_escape_status() {
        // let obj1 = { x: 1 };   // NotEscaped
        // let obj2 = { y: 2 };   // Escaped (returned)
        // let a = obj1.x;
        // return obj2;
        let body = vec![
            MirStmt::Let(
                LocalId(0),
                MirExpr::ObjectNew {
                    props: vec![("x".to_string(), MirExpr::Number(1, span()))],
                    non_enumerable: 0,
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(
                LocalId(1),
                MirExpr::ObjectNew {
                    props: vec![("y".to_string(), MirExpr::Number(2, span()))],
                    non_enumerable: 0,
                    span: span(),
                },
                span(),
            ),
            MirStmt::Let(
                LocalId(2),
                MirExpr::PropertyGet {
                    obj: Box::new(MirExpr::Local(LocalId(0), span())),
                    key: "x".to_string(),
                    span: span(),
                },
                span(),
            ),
            MirStmt::Return(MirExpr::Local(LocalId(1), span()), span()),
        ];

        let escape_status = vec![
            Some(EscapeStatus::NotEscaped),
            Some(EscapeStatus::Escaped), // obj2 escapes
            Some(EscapeStatus::NotEscaped),
        ];

        let result = run_scalar_replace(body, escape_status);

        // After replacement:
        // let _0 = 1;        // obj1.x
        // let obj2 = { y: 2 };  // unchanged (escaped)
        // let a = _0;        // a = obj1.x
        // return obj2;       // unchanged
        assert_eq!(result.len(), 4, "should have 4 statements");

        // obj2's ObjectNew should still be present (escaped).
        assert!(
            result
                .iter()
                .any(|s| matches!(s, MirStmt::Let(LocalId(1), MirExpr::ObjectNew { .. }, _))),
            "escaped obj2 ObjectNew should remain"
        );

        // obj1's ObjectNew should be replaced.
        assert!(
            !result
                .iter()
                .any(|s| matches!(s, MirStmt::Let(LocalId(0), MirExpr::ObjectNew { .. }, _))),
            "not-escaped obj1 ObjectNew should be removed"
        );
    }
}
