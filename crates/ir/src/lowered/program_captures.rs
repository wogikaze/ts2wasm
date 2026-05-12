use super::*;
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedStmt};
use std::collections::HashSet;

pub(crate) fn collect_arrow_captures(
    expr: &ResolvedExpr,
    params: &[String],
    captures: &mut Vec<String>,
) {
    match expr {
        ResolvedExpr::This { .. } => push_capture("this", params, captures),
        ResolvedExpr::NewTarget { .. } => {}
        ResolvedExpr::Ident(name) => push_capture(name, params, captures),
        ResolvedExpr::Await { expr } => {
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => {
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::Binary { left, right, .. } => {
            collect_arrow_captures(left, params, captures);
            collect_arrow_captures(right, params, captures);
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_arrow_captures(condition, params, captures);
            collect_arrow_captures(then_expr, params, captures);
            collect_arrow_captures(else_expr, params, captures);
        }
        ResolvedExpr::Call { callee, args, .. } => {
            collect_arrow_captures(callee, params, captures);
            for arg in args {
                collect_arrow_captures(arg, params, captures);
            }
        }
        ResolvedExpr::Assign { name, expr } => {
            push_capture(name, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalAssign { name, expr, .. } => {
            push_capture(name, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalPropertyAssign { object, expr, .. } => {
            push_capture(object, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalComputedPropertyAssign {
            object, key, expr, ..
        } => {
            push_capture(object, params, captures);
            collect_arrow_captures(key, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(key, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                if let ResolvedArrayElement::Present(expr) = element {
                    collect_arrow_captures(expr, params, captures);
                }
            }
        }
        ResolvedExpr::Object(props) => {
            for (_, value) in props {
                collect_arrow_captures(value, params, captures);
            }
        }
        ResolvedExpr::ComputedIndex { object, index } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(index, params, captures);
        }
        ResolvedExpr::BuiltinCall { args, .. } => {
            for arg in args {
                collect_arrow_captures(arg, params, captures);
            }
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
            collect_arrow_captures(object, params, captures);
        }
        ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(index, params, captures);
        }
        ResolvedExpr::OptionalCall { callee, args, .. } => {
            collect_arrow_captures(callee, params, captures);
            for arg in args {
                collect_arrow_captures(arg, params, captures);
            }
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            collect_arrow_captures(object, params, captures);
            for arg in args {
                collect_arrow_captures(arg, params, captures);
            }
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(value, params, captures);
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(key, params, captures);
            collect_arrow_captures(value, params, captures);
        }
        ResolvedExpr::New { args, .. } => {
            for arg in args {
                collect_arrow_captures(arg, params, captures);
            }
        }
        ResolvedExpr::ArrowFn { .. }
        | ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined => {}
    }
}

pub(crate) fn collect_declared_names_in_stmts(stmts: &[ResolvedStmt], names: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Let(name, _) => {
                names.insert(name.clone());
            }
            ResolvedStmt::DestructureLet { pattern, .. } => {
                for name in pattern.names() {
                    names.insert(name.to_owned());
                }
            }
            ResolvedStmt::Function { name, body, .. } => {
                names.insert(name.clone());
                collect_declared_names_in_stmts(body, names);
            }
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_declared_names_in_stmts(then_body, names);
                collect_declared_names_in_stmts(else_body, names);
            }
            ResolvedStmt::While { body, .. }
            | ResolvedStmt::DoWhile { body, .. }
            | ResolvedStmt::ForIn { body, .. }
            | ResolvedStmt::ForOf { body, .. } => collect_declared_names_in_stmts(body, names),
            ResolvedStmt::For { init, body, .. } => {
                if let Some(init) = init {
                    collect_declared_names_in_stmt(init, names);
                }
                collect_declared_names_in_stmts(body, names);
            }
            ResolvedStmt::TryCatch {
                catch_param,
                try_block,
                catch_block,
                finally_block,
            } => {
                if let Some(param) = catch_param {
                    names.insert(param.clone());
                }
                collect_declared_names_in_stmts(try_block, names);
                if let Some(block) = catch_block {
                    collect_declared_names_in_stmts(block, names);
                }
                if let Some(block) = finally_block {
                    collect_declared_names_in_stmts(block, names);
                }
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_declared_names_in_stmts(body, names);
                }
            }
            ResolvedStmt::Labeled { body, .. } => collect_declared_names_in_stmt(body, names),
            ResolvedStmt::Block { statements, .. } => {
                collect_declared_names_in_stmts(statements, names);
            }
            ResolvedStmt::ClassDecl { name, .. } => {
                names.insert(name.clone());
            }
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Assign(_, _)
            | ResolvedStmt::Expr(_)
            | ResolvedStmt::Return(_)
            | ResolvedStmt::Throw(_)
            | ResolvedStmt::Export { .. }
            | ResolvedStmt::ModuleExportsAssign { .. }
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. } => {}
        }
    }
}

pub(crate) fn collect_declared_names_in_stmt(stmt: &ResolvedStmt, names: &mut HashSet<String>) {
    collect_declared_names_in_stmts(std::slice::from_ref(stmt), names);
}

pub(crate) fn collect_stmt_captures(
    stmts: &[ResolvedStmt],
    excluded: &HashSet<String>,
    captures: &mut Vec<String>,
) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Let(_, expr)
            | ResolvedStmt::DestructureLet { expr, .. }
            | ResolvedStmt::Expr(expr)
            | ResolvedStmt::Return(expr)
            | ResolvedStmt::Throw(expr) => collect_expr_captures(expr, excluded, captures),
            ResolvedStmt::Assign(name, expr) => {
                push_capture(
                    name,
                    &excluded.iter().cloned().collect::<Vec<_>>(),
                    captures,
                );
                collect_expr_captures(expr, excluded, captures);
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_expr_captures(condition, excluded, captures);
                collect_stmt_captures(then_body, excluded, captures);
                collect_stmt_captures(else_body, excluded, captures);
            }
            ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
                collect_expr_captures(condition, excluded, captures);
                collect_stmt_captures(body, excluded, captures);
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_stmt_captures(try_block, excluded, captures);
                if let Some(block) = catch_block {
                    collect_stmt_captures(block, excluded, captures);
                }
                if let Some(block) = finally_block {
                    collect_stmt_captures(block, excluded, captures);
                }
            }
            ResolvedStmt::Switch { expr, cases } => {
                collect_expr_captures(expr, excluded, captures);
                for (case_expr, body) in cases {
                    if let Some(case_expr) = case_expr {
                        collect_expr_captures(case_expr, excluded, captures);
                    }
                    collect_stmt_captures(body, excluded, captures);
                }
            }
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init) = init {
                    collect_stmt_captures(std::slice::from_ref(init.as_ref()), excluded, captures);
                }
                if let Some(condition) = condition {
                    collect_expr_captures(condition, excluded, captures);
                }
                if let Some(update) = update {
                    collect_expr_captures(update, excluded, captures);
                }
                collect_stmt_captures(body, excluded, captures);
            }
            ResolvedStmt::ForIn { iter, body, .. } | ResolvedStmt::ForOf { iter, body, .. } => {
                collect_expr_captures(iter, excluded, captures);
                collect_stmt_captures(body, excluded, captures);
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_stmt_captures(std::slice::from_ref(body.as_ref()), excluded, captures);
            }
            ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
                collect_expr_captures(expr, excluded, captures);
            }
            ResolvedStmt::Block { statements, .. } => {
                collect_stmt_captures(statements, excluded, captures);
            }
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Function { .. }
            | ResolvedStmt::ClassDecl { .. }
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. } => {}
        }
    }
}

pub(crate) fn collect_expr_captures(
    expr: &ResolvedExpr,
    excluded: &HashSet<String>,
    captures: &mut Vec<String>,
) {
    let mut params = excluded.iter().cloned().collect::<Vec<_>>();
    params.sort();
    collect_arrow_captures(expr, &params, captures);
}

pub(crate) fn block_assigns_any_name(stmts: &[ResolvedStmt], names: &[String]) -> bool {
    stmts.iter().any(|stmt| stmt_assigns_any_name(stmt, names))
}

pub(crate) fn stmt_assigns_any_name(stmt: &ResolvedStmt, names: &[String]) -> bool {
    match stmt {
        ResolvedStmt::Assign(name, expr) => {
            names.iter().any(|capture| capture == name) || expr_assigns_any_name(expr, names)
        }
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr) => expr_assigns_any_name(expr, names),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_assigns_any_name(condition, names)
                || block_assigns_any_name(then_body, names)
                || block_assigns_any_name(else_body, names)
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
            expr_assigns_any_name(condition, names) || block_assigns_any_name(body, names)
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_assigns_any_name(try_block, names)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block_assigns_any_name(block, names))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block_assigns_any_name(block, names))
        }
        ResolvedStmt::Switch { expr, cases } => {
            expr_assigns_any_name(expr, names)
                || cases.iter().any(|(case_expr, body)| {
                    case_expr
                        .as_ref()
                        .is_some_and(|expr| expr_assigns_any_name(expr, names))
                        || block_assigns_any_name(body, names)
                })
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            init.as_ref()
                .is_some_and(|stmt| stmt_assigns_any_name(stmt, names))
                || condition
                    .as_ref()
                    .is_some_and(|expr| expr_assigns_any_name(expr, names))
                || update
                    .as_ref()
                    .is_some_and(|expr| expr_assigns_any_name(expr, names))
                || block_assigns_any_name(body, names)
        }
        ResolvedStmt::ForIn { iter, body, .. } | ResolvedStmt::ForOf { iter, body, .. } => {
            expr_assigns_any_name(iter, names) || block_assigns_any_name(body, names)
        }
        ResolvedStmt::Labeled { body, .. } => stmt_assigns_any_name(body, names),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_assigns_any_name(expr, names)
        }
        ResolvedStmt::Block { statements, .. } => block_assigns_any_name(statements, names),
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

pub(crate) fn expr_assigns_any_name(expr: &ResolvedExpr, names: &[String]) -> bool {
    match expr {
        ResolvedExpr::Assign { name, expr } | ResolvedExpr::LogicalAssign { name, expr, .. } => {
            names.iter().any(|capture| capture == name) || expr_assigns_any_name(expr, names)
        }
        ResolvedExpr::Await { expr } => expr_assigns_any_name(expr, names),
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => {
            expr_assigns_any_name(expr, names)
        }
        ResolvedExpr::Binary { left, right, .. } => {
            expr_assigns_any_name(left, names) || expr_assigns_any_name(right, names)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_assigns_any_name(condition, names)
                || expr_assigns_any_name(then_expr, names)
                || expr_assigns_any_name(else_expr, names)
        }
        ResolvedExpr::Call { callee, args, .. } => {
            expr_assigns_any_name(callee, names)
                || args.iter().any(|arg| expr_assigns_any_name(arg, names))
        }
        ResolvedExpr::LogicalPropertyAssign { expr, .. } => expr_assigns_any_name(expr, names),
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_assigns_any_name(key, names) || expr_assigns_any_name(expr, names)
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_assigns_any_name(object, names) || expr_assigns_any_name(expr, names)
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            expr_assigns_any_name(object, names)
                || expr_assigns_any_name(key, names)
                || expr_assigns_any_name(expr, names)
        }
        ResolvedExpr::Array(elements) => elements.iter().any(|element| match element {
            ResolvedArrayElement::Present(expr) => expr_assigns_any_name(expr, names),
            ResolvedArrayElement::Hole => false,
        }),
        ResolvedExpr::Object(props) => props
            .iter()
            .any(|(_, value)| expr_assigns_any_name(value, names)),
        ResolvedExpr::ComputedIndex { object, index } => {
            expr_assigns_any_name(object, names) || expr_assigns_any_name(index, names)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(|arg| expr_assigns_any_name(arg, names))
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
            expr_assigns_any_name(object, names)
        }
        ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            expr_assigns_any_name(object, names) || expr_assigns_any_name(index, names)
        }
        ResolvedExpr::OptionalCall { callee, args, .. } => {
            expr_assigns_any_name(callee, names)
                || args.iter().any(|arg| expr_assigns_any_name(arg, names))
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            expr_assigns_any_name(object, names)
                || args.iter().any(|arg| expr_assigns_any_name(arg, names))
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            expr_assigns_any_name(object, names) || expr_assigns_any_name(value, names)
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            expr_assigns_any_name(object, names)
                || expr_assigns_any_name(key, names)
                || expr_assigns_any_name(value, names)
        }
        ResolvedExpr::ArrowFn { .. }
        | ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined => false,
    }
}

pub(crate) fn push_capture(name: &str, params: &[String], captures: &mut Vec<String>) {
    if params.iter().any(|param| param == name) || captures.iter().any(|capture| capture == name) {
        return;
    }
    captures.push(name.to_owned());
}
