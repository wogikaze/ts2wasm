use super::{
    DirectEvalBlockFunctionEnv, block_contains_arguments, block_contains_this,
    direct_iife_body_has_static_eval_block_function_binding,
};
use crate::builtin_resolved::{
    EvalKind, EvalSource, ResolvedArrayElement, ResolvedExpr, ResolvedParam, ResolvedStmt,
};
use std::collections::{HashMap, HashSet};
use ts2wasm_resolve::direct_eval_source::{eval_function_names, eval_var_and_function_names};

#[path = "program_direct_eval/env.rs"]
mod env;

pub(crate) use env::*;
pub(crate) fn collect_direct_eval_block_function_env(
    program: &[ResolvedStmt],
) -> DirectEvalBlockFunctionEnv {
    let mut env = DirectEvalBlockFunctionEnv::default();
    collect_direct_eval_block_function_env_from_stmts(program, &mut env);
    env
}

pub(crate) fn collect_dynamic_direct_eval_env_cell_names(
    params: &[ResolvedParam],
    body: &[ResolvedStmt],
    include_arguments: bool,
    include_this: bool,
) -> HashSet<String> {
    if !block_contains_dynamic_direct_eval(body) {
        return HashSet::new();
    }

    let mut names = HashSet::new();
    for param in params {
        let name = param.name.strip_prefix("...").unwrap_or(&param.name);
        if is_direct_eval_env_binding_name(name) {
            names.insert(name.to_owned());
        }
    }
    if include_arguments && !params.iter().any(|param| param.name == "arguments") {
        names.insert("arguments".to_owned());
    }
    if include_this && !params.iter().any(|param| param.name == "this") {
        names.insert("this".to_owned());
    }
    collect_block_declared_names(body, &mut names);
    names
}

pub(crate) fn collect_dynamic_direct_eval_created_binding_names(
    body: &[ResolvedStmt],
) -> HashSet<String> {
    let known_sources = collect_unassigned_string_bindings(body);
    let mut names = HashSet::new();
    collect_dynamic_direct_eval_created_binding_names_from_stmts(body, &known_sources, &mut names);
    names
}

pub(crate) fn collect_dynamic_direct_eval_created_function_names(
    body: &[ResolvedStmt],
) -> HashSet<String> {
    let known_sources = collect_unassigned_string_bindings(body);
    let mut names = HashSet::new();
    collect_dynamic_direct_eval_created_function_names_from_stmts(body, &known_sources, &mut names);
    names
}

fn collect_dynamic_direct_eval_created_function_names_from_stmts(
    stmts: &[ResolvedStmt],
    known_sources: &HashMap<String, String>,
    names: &mut HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Let(_, expr)
            | ResolvedStmt::Assign(_, expr)
            | ResolvedStmt::Expr(expr)
            | ResolvedStmt::Return(expr)
            | ResolvedStmt::Throw(expr) => {
                collect_dynamic_direct_eval_created_function_names_from_expr(
                    expr,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::ModuleExportsAssign { expr } => {
                collect_dynamic_direct_eval_created_function_names_from_expr(
                    expr,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_dynamic_direct_eval_created_function_names_from_expr(
                    condition,
                    known_sources,
                    names,
                );
                collect_dynamic_direct_eval_created_function_names_from_stmts(
                    then_body,
                    known_sources,
                    names,
                );
                collect_dynamic_direct_eval_created_function_names_from_stmts(
                    else_body,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { condition, body } => {
                collect_dynamic_direct_eval_created_function_names_from_expr(
                    condition,
                    known_sources,
                    names,
                );
                collect_dynamic_direct_eval_created_function_names_from_stmts(
                    body,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init) = init {
                    collect_dynamic_direct_eval_created_function_names_from_stmts(
                        std::slice::from_ref(init.as_ref()),
                        known_sources,
                        names,
                    );
                }
                if let Some(condition) = condition {
                    collect_dynamic_direct_eval_created_function_names_from_expr(
                        condition,
                        known_sources,
                        names,
                    );
                }
                if let Some(update) = update {
                    collect_dynamic_direct_eval_created_function_names_from_expr(
                        update,
                        known_sources,
                        names,
                    );
                }
                collect_dynamic_direct_eval_created_function_names_from_stmts(
                    body,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::ForIn { iter, body, .. }
            | ResolvedStmt::ForOf { iter, body, .. }
            | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
                collect_dynamic_direct_eval_created_function_names_from_expr(
                    iter,
                    known_sources,
                    names,
                );
                collect_dynamic_direct_eval_created_function_names_from_stmts(
                    body,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::Switch { expr, cases } => {
                collect_dynamic_direct_eval_created_function_names_from_expr(
                    expr,
                    known_sources,
                    names,
                );
                for (_, body) in cases {
                    collect_dynamic_direct_eval_created_function_names_from_stmts(
                        body,
                        known_sources,
                        names,
                    );
                }
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_dynamic_direct_eval_created_function_names_from_stmts(
                    try_block,
                    known_sources,
                    names,
                );
                if let Some(catch_block) = catch_block {
                    collect_dynamic_direct_eval_created_function_names_from_stmts(
                        catch_block,
                        known_sources,
                        names,
                    );
                }
                if let Some(finally_block) = finally_block {
                    collect_dynamic_direct_eval_created_function_names_from_stmts(
                        finally_block,
                        known_sources,
                        names,
                    );
                }
            }
            ResolvedStmt::Block { statements } => {
                collect_dynamic_direct_eval_created_function_names_from_stmts(
                    statements,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_dynamic_direct_eval_created_function_names_from_stmts(
                    std::slice::from_ref(body.as_ref()),
                    known_sources,
                    names,
                );
            }
            _ => {}
        }
    }
}

fn collect_dynamic_direct_eval_created_function_names_from_expr(
    expr: &ResolvedExpr,
    known_sources: &HashMap<String, String>,
    names: &mut HashSet<String>,
) {
    if let Some(source) = dynamic_direct_eval_known_source(expr, known_sources) {
        collect_eval_function_names(source, names);
    }
    visit_dynamic_direct_eval_expr_children(
        expr,
        known_sources,
        names,
        collect_dynamic_direct_eval_created_function_names_from_stmts,
        collect_dynamic_direct_eval_created_function_names_from_expr,
    );
}

fn visit_dynamic_direct_eval_expr_children(
    expr: &ResolvedExpr,
    known_sources: &HashMap<String, String>,
    names: &mut HashSet<String>,
    visit_stmts: fn(&[ResolvedStmt], &HashMap<String, String>, &mut HashSet<String>),
    visit_expr: fn(&ResolvedExpr, &HashMap<String, String>, &mut HashSet<String>),
) {
    match expr {
        ResolvedExpr::Eval { plan } => {
            if let EvalSource::Runtime(source) = &plan.source {
                visit_expr(source, known_sources, names);
            }
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            visit_expr(callee, known_sources, names);
            visit_exprs(args, known_sources, names, visit_expr);
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            visit_expr(object, known_sources, names);
            visit_exprs(args, known_sources, names, visit_expr);
        }
        ResolvedExpr::Await { expr }
        | ResolvedExpr::Yield {
            expr: Some(expr), ..
        }
        | ResolvedExpr::Unary { expr, .. }
        | ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::Spread(expr)
        | ResolvedExpr::PropertyAccess { object: expr, .. }
        | ResolvedExpr::OptionalPropertyAccess { object: expr, .. }
        | ResolvedExpr::BuiltinProperty { object: expr, .. } => {
            visit_expr(expr, known_sources, names);
        }
        ResolvedExpr::Binary { left, right, .. }
        | ResolvedExpr::ComputedIndex {
            object: left,
            index: right,
        }
        | ResolvedExpr::OptionalComputedIndex {
            object: left,
            index: right,
            ..
        } => {
            visit_exprs(
                [left.as_ref(), right.as_ref()],
                known_sources,
                names,
                visit_expr,
            );
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            visit_exprs(
                [condition.as_ref(), then_expr.as_ref(), else_expr.as_ref()],
                known_sources,
                names,
                visit_expr,
            );
        }
        ResolvedExpr::New { args, .. } | ResolvedExpr::BuiltinCall { args, .. } => {
            visit_exprs(args, known_sources, names, visit_expr);
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                if let ResolvedArrayElement::Present(expr) = element {
                    visit_expr(expr, known_sources, names);
                }
            }
        }
        ResolvedExpr::Object(props) => {
            for prop in props {
                if let Some(key) = prop.computed_key() {
                    visit_expr(key, known_sources, names);
                }
                visit_expr(prop.value(), known_sources, names);
            }
        }
        ResolvedExpr::PropertyAssign { object, value, .. }
        | ResolvedExpr::LogicalMemberAssign {
            object,
            expr: value,
            ..
        } => {
            visit_expr(object, known_sources, names);
            visit_expr(value, known_sources, names);
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value }
        | ResolvedExpr::LogicalComputedMemberAssign {
            object,
            key,
            expr: value,
            ..
        } => {
            visit_exprs(
                [object.as_ref(), key.as_ref(), value.as_ref()],
                known_sources,
                names,
                visit_expr,
            );
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            visit_expr(key, known_sources, names);
            visit_expr(expr, known_sources, names);
        }
        ResolvedExpr::Sequence(exprs) => {
            visit_exprs(exprs, known_sources, names, visit_expr);
        }
        ResolvedExpr::ArrowFn {
            body, body_stmts, ..
        } => {
            visit_stmts(body_stmts, known_sources, names);
            visit_expr(body, known_sources, names);
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            visit_exprs(&plan.args, known_sources, names, visit_expr);
        }
        _ => {}
    }
}

fn visit_exprs<'a>(
    exprs: impl IntoIterator<Item = &'a ResolvedExpr>,
    known_sources: &HashMap<String, String>,
    names: &mut HashSet<String>,
    visit_expr: fn(&ResolvedExpr, &HashMap<String, String>, &mut HashSet<String>),
) {
    for expr in exprs {
        visit_expr(expr, known_sources, names);
    }
}

fn collect_dynamic_direct_eval_created_binding_names_from_stmts(
    stmts: &[ResolvedStmt],
    known_sources: &HashMap<String, String>,
    names: &mut HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Let(_, expr)
            | ResolvedStmt::Assign(_, expr)
            | ResolvedStmt::Expr(expr)
            | ResolvedStmt::Return(expr)
            | ResolvedStmt::Throw(expr) => {
                collect_dynamic_direct_eval_created_binding_names_from_expr(
                    expr,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::ModuleExportsAssign { expr } => {
                collect_dynamic_direct_eval_created_binding_names_from_expr(
                    expr,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_dynamic_direct_eval_created_binding_names_from_expr(
                    condition,
                    known_sources,
                    names,
                );
                collect_dynamic_direct_eval_created_binding_names_from_stmts(
                    then_body,
                    known_sources,
                    names,
                );
                collect_dynamic_direct_eval_created_binding_names_from_stmts(
                    else_body,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { condition, body } => {
                collect_dynamic_direct_eval_created_binding_names_from_expr(
                    condition,
                    known_sources,
                    names,
                );
                collect_dynamic_direct_eval_created_binding_names_from_stmts(
                    body,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init) = init {
                    collect_dynamic_direct_eval_created_binding_names_from_stmts(
                        std::slice::from_ref(init.as_ref()),
                        known_sources,
                        names,
                    );
                }
                if let Some(condition) = condition {
                    collect_dynamic_direct_eval_created_binding_names_from_expr(
                        condition,
                        known_sources,
                        names,
                    );
                }
                if let Some(update) = update {
                    collect_dynamic_direct_eval_created_binding_names_from_expr(
                        update,
                        known_sources,
                        names,
                    );
                }
                collect_dynamic_direct_eval_created_binding_names_from_stmts(
                    body,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::ForIn { iter, body, .. }
            | ResolvedStmt::ForOf { iter, body, .. }
            | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
                collect_dynamic_direct_eval_created_binding_names_from_expr(
                    iter,
                    known_sources,
                    names,
                );
                collect_dynamic_direct_eval_created_binding_names_from_stmts(
                    body,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::Switch { expr, cases } => {
                collect_dynamic_direct_eval_created_binding_names_from_expr(
                    expr,
                    known_sources,
                    names,
                );
                for (_, body) in cases {
                    collect_dynamic_direct_eval_created_binding_names_from_stmts(
                        body,
                        known_sources,
                        names,
                    );
                }
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_dynamic_direct_eval_created_binding_names_from_stmts(
                    try_block,
                    known_sources,
                    names,
                );
                if let Some(catch_block) = catch_block {
                    collect_dynamic_direct_eval_created_binding_names_from_stmts(
                        catch_block,
                        known_sources,
                        names,
                    );
                }
                if let Some(finally_block) = finally_block {
                    collect_dynamic_direct_eval_created_binding_names_from_stmts(
                        finally_block,
                        known_sources,
                        names,
                    );
                }
            }
            ResolvedStmt::Block { statements } => {
                collect_dynamic_direct_eval_created_binding_names_from_stmts(
                    statements,
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_dynamic_direct_eval_created_binding_names_from_stmts(
                    std::slice::from_ref(body.as_ref()),
                    known_sources,
                    names,
                );
            }
            ResolvedStmt::Function { .. }
            | ResolvedStmt::ClassDecl { .. }
            | ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::DestructureLet { .. }
            | ResolvedStmt::DestructureAssign { .. }
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. }
            | ResolvedStmt::Export { .. } => {}
        }
    }
}

fn collect_dynamic_direct_eval_created_binding_names_from_expr(
    expr: &ResolvedExpr,
    known_sources: &HashMap<String, String>,
    names: &mut HashSet<String>,
) {
    if let Some(source) = dynamic_direct_eval_known_source(expr, known_sources) {
        collect_eval_var_function_names(source, names);
    }
    visit_dynamic_direct_eval_expr_children(
        expr,
        known_sources,
        names,
        collect_dynamic_direct_eval_created_binding_names_from_stmts,
        collect_dynamic_direct_eval_created_binding_names_from_expr,
    );
}
