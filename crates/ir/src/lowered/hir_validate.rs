use super::hir::{HirExpr, HirProgram, HirStmt};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};

/// Validate a `HirProgram` for internal consistency.
///
/// Checks:
/// - Function ids match their positional indices
/// - Local ids are within declared bounds
/// - No invalid references
pub fn validate_hir(program: &HirProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let body_local_count = program.locals.len();

    for local in &program.locals {
        validate_local_id(*local, body_local_count, "hir top-level local", &mut errors);
    }

    for (idx, func) in program.functions.iter().enumerate() {
        if func.id.0 != idx {
            push_invariant(
                &mut errors,
                format!(
                    "hir function id {} does not match its index {}",
                    func.id.0, idx
                ),
            );
        }
    }

    validate_hir_stmts(
        &program.body,
        body_local_count,
        program.functions.len(),
        true,
        &mut errors,
    );

    for func in &program.functions {
        let local_count = func.params.len() + func.locals.len();
        for param in &func.params {
            validate_local_id(*param, local_count, "hir function param", &mut errors);
        }
        for local in &func.locals {
            validate_local_id(*local, local_count, "hir function local", &mut errors);
        }
        validate_hir_stmts(
            &func.body,
            local_count,
            program.functions.len(),
            false,
            &mut errors,
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_hir_stmts(
    stmts: &[HirStmt],
    local_count: usize,
    func_count: usize,
    top_level: bool,
    errors: &mut Vec<Diagnostic>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Let { local, init } => {
                validate_local_id(*local, local_count, "hir let local", errors);
                validate_hir_expr(init, local_count, func_count, errors);
            }
            HirStmt::Assign { local, expr } => {
                validate_local_id(*local, local_count, "hir assign local", errors);
                validate_hir_expr(expr, local_count, func_count, errors);
            }
            HirStmt::Expr(expr) => {
                validate_hir_expr(expr, local_count, func_count, errors);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                validate_hir_expr(condition, local_count, func_count, errors);
                validate_hir_stmts(then_body, local_count, func_count, top_level, errors);
                validate_hir_stmts(else_body, local_count, func_count, top_level, errors);
            }
            HirStmt::While { condition, body } => {
                validate_hir_expr(condition, local_count, func_count, errors);
                validate_hir_stmts(body, local_count, func_count, top_level, errors);
            }
            HirStmt::Return(expr) => {
                if top_level {
                    errors.push(Diagnostic {
                        code: DiagCode::InvalidTopLevelReturn,
                        message: "hir top-level return is invalid".to_owned(),
                        span: None,
                        phase: None,
                    });
                }
                validate_hir_expr(expr, local_count, func_count, errors);
            }
            HirStmt::Throw(expr) => {
                validate_hir_expr(expr, local_count, func_count, errors);
            }
        }
    }
}

fn validate_hir_expr(
    expr: &HirExpr,
    local_count: usize,
    _func_count: usize,
    errors: &mut Vec<Diagnostic>,
) {
    match expr {
        HirExpr::Local(id) => {
            validate_local_id(*id, local_count, "hir local reference", errors);
        }
        HirExpr::Unary { expr: inner, .. } => {
            validate_hir_expr(inner, local_count, _func_count, errors);
        }
        HirExpr::Binary { left, right, .. } => {
            validate_hir_expr(left, local_count, _func_count, errors);
            validate_hir_expr(right, local_count, _func_count, errors);
        }
        HirExpr::GetProp { object, .. } => {
            validate_hir_expr(object, local_count, _func_count, errors);
        }
        HirExpr::GetIndex { object, index, .. } => {
            validate_hir_expr(object, local_count, _func_count, errors);
            validate_hir_expr(index, local_count, _func_count, errors);
        }
        HirExpr::SetProp { object, value, .. } => {
            validate_hir_expr(object, local_count, _func_count, errors);
            validate_hir_expr(value, local_count, _func_count, errors);
        }
        HirExpr::SetIndex {
            object,
            index,
            value,
            ..
        } => {
            validate_hir_expr(object, local_count, _func_count, errors);
            validate_hir_expr(index, local_count, _func_count, errors);
            validate_hir_expr(value, local_count, _func_count, errors);
        }
        HirExpr::HasProperty { object, key, .. } | HirExpr::DeleteProperty { object, key, .. } => {
            validate_hir_expr(object, local_count, _func_count, errors);
            validate_hir_expr(key, local_count, _func_count, errors);
        }
        HirExpr::ObjectLiteral { props } => {
            for (_, value) in props {
                validate_hir_expr(value, local_count, _func_count, errors);
            }
        }
        HirExpr::ArrayLiteral { elements } => {
            for elem in elements {
                validate_hir_expr(elem, local_count, _func_count, errors);
            }
        }
        HirExpr::Call { callee, args, .. }
        | HirExpr::MethodCall {
            receiver: callee,
            args,
            ..
        } => {
            validate_hir_expr(callee, local_count, _func_count, errors);
            for arg in args {
                validate_hir_expr(arg, local_count, _func_count, errors);
            }
        }
        HirExpr::New { constructor, args } => {
            validate_func_id(
                *constructor,
                _func_count,
                "hir constructor reference",
                errors,
            );
            for arg in args {
                validate_hir_expr(arg, local_count, _func_count, errors);
            }
        }
        HirExpr::If {
            condition,
            then_expr,
            else_expr,
        } => {
            validate_hir_expr(condition, local_count, _func_count, errors);
            validate_hir_expr(then_expr, local_count, _func_count, errors);
            validate_hir_expr(else_expr, local_count, _func_count, errors);
        }
        HirExpr::Number(_)
        | HirExpr::String(_)
        | HirExpr::Bool(_)
        | HirExpr::Null
        | HirExpr::Undefined => {}
    }
}

fn validate_local_id(
    id: crate::lowered::LocalId,
    local_count: usize,
    context: &str,
    errors: &mut Vec<Diagnostic>,
) {
    if id.0 >= local_count {
        push_invariant(
            errors,
            format!(
                "{} {} out of bounds (max {})",
                context,
                id.0,
                local_count.saturating_sub(1)
            ),
        );
    }
}

fn validate_func_id(
    id: crate::lowered::FuncId,
    func_count: usize,
    context: &str,
    errors: &mut Vec<Diagnostic>,
) {
    if id.0 >= func_count {
        push_invariant(
            errors,
            format!(
                "{} {} out of bounds (max {})",
                context,
                id.0,
                func_count.saturating_sub(1)
            ),
        );
    }
}

fn push_invariant(errors: &mut Vec<Diagnostic>, message: String) {
    errors.push(Diagnostic {
        code: DiagCode::InvariantViolation,
        message,
        span: None,
        phase: None,
    });
}
