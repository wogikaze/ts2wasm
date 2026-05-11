use super::hir::{HirExpr, HirFunction, HirProgram, HirStmt};
use ts2wasm_shared::{DiagCode, Diagnostic};

/// Validate a `HirProgram` for internal consistency.
///
/// Checks:
/// - Function ids match their positional indices
/// - Local ids are within declared bounds
/// - No invalid references
pub fn validate_hir(program: &HirProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let body_local_count = program.locals.len();

    for (idx, func) in program.functions.iter().enumerate() {
        if func.id.0 != idx {
            errors.push(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "hir function id {} does not match its index {}",
                    func.id.0, idx
                ),
                span: None,
                phase: None,
            });
        }
    }

    validate_hir_stmts(&program.body, body_local_count, &program.functions, &mut errors);

    for func in &program.functions {
        let local_count = func.params.len() + func.locals.len();
        validate_hir_stmts(&func.body, local_count, &program.functions, &mut errors);
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
    functions: &[HirFunction],
    errors: &mut Vec<Diagnostic>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Let { init, .. }
            | HirStmt::Assign { expr: init, .. } => {
                validate_hir_expr(init, local_count, functions.len(), errors);
            }
            HirStmt::Expr(expr) => {
                validate_hir_expr(expr, local_count, functions.len(), errors);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                validate_hir_expr(condition, local_count, functions.len(), errors);
                validate_hir_stmts(then_body, local_count, functions, errors);
                validate_hir_stmts(else_body, local_count, functions, errors);
            }
            HirStmt::While { condition, body } => {
                validate_hir_expr(condition, local_count, functions.len(), errors);
                validate_hir_stmts(body, local_count, functions, errors);
            }
            HirStmt::Return(expr) | HirStmt::Throw(expr) => {
                validate_hir_expr(expr, local_count, functions.len(), errors);
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
            if id.0 >= local_count {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "hir local id {} out of bounds (max {})",
                        id.0,
                        local_count.saturating_sub(1)
                    ),
                    span: None,
                    phase: None,
                });
            }
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
            object, index, value, ..
        } => {
            validate_hir_expr(object, local_count, _func_count, errors);
            validate_hir_expr(index, local_count, _func_count, errors);
            validate_hir_expr(value, local_count, _func_count, errors);
        }
        HirExpr::HasProperty { object, key, .. }
        | HirExpr::DeleteProperty { object, key, .. } => {
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
        HirExpr::Call { callee, args, .. } | HirExpr::MethodCall { receiver: callee, args, .. } => {
            validate_hir_expr(callee, local_count, _func_count, errors);
            for arg in args {
                validate_hir_expr(arg, local_count, _func_count, errors);
            }
        }
        HirExpr::New { args, .. } => {
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
