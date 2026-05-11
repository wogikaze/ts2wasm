use super::mir::{MirExpr, MirFunction, MirProgram, MirStmt};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};

/// Validate a `MirProgram` for internal consistency.
///
/// Checks:
/// - Function ids match their positional indices
/// - Local ids are within declared bounds
/// - No invalid references
pub fn validate_mir(program: &MirProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();

    for (idx, func) in program.functions.iter().enumerate() {
        if func.id.0 != idx {
            errors.push(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "mir function id {} does not match its index {}",
                    func.id.0, idx
                ),
                span: None,
                phase: None,
            });
        }
    }

    validate_mir_stmts(
        &program.top_level_statements,
        program.top_level_locals.len(),
        &program.functions,
        &mut errors,
    );

    for func in &program.functions {
        let local_count = func.params.len() + func.locals.len();
        validate_mir_stmts(&func.body, local_count, &program.functions, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_mir_stmts(
    stmts: &[MirStmt],
    local_count: usize,
    functions: &[MirFunction],
    errors: &mut Vec<Diagnostic>,
) {
    for stmt in stmts {
        match stmt {
            MirStmt::Let { init, .. }
            | MirStmt::Assign { init, .. } => {
                validate_mir_expr(init, local_count, functions.len(), errors);
            }
            MirStmt::Expr(expr) => {
                validate_mir_expr(expr, local_count, functions.len(), errors);
            }
            MirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                validate_mir_expr(condition, local_count, functions.len(), errors);
                validate_mir_stmts(then_body, local_count, functions, errors);
                validate_mir_stmts(else_body, local_count, functions, errors);
            }
            MirStmt::While { condition, body } => {
                validate_mir_expr(condition, local_count, functions.len(), errors);
                validate_mir_stmts(body, local_count, functions, errors);
            }
            MirStmt::Return(expr) | MirStmt::Throw(expr) => {
                validate_mir_expr(expr, local_count, functions.len(), errors);
            }
            MirStmt::TryCatch {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                validate_mir_stmts(try_body, local_count, functions, errors);
                if let Some(body) = catch_body {
                    validate_mir_stmts(body, local_count, functions, errors);
                }
                if let Some(body) = finally_body {
                    validate_mir_stmts(body, local_count, functions, errors);
                }
            }
            MirStmt::Switch { expr, cases } => {
                validate_mir_expr(expr, local_count, functions.len(), errors);
                for (_, body) in cases {
                    validate_mir_stmts(body, local_count, functions, errors);
                }
            }
            MirStmt::Labeled { body, .. } => {
                validate_mir_stmts(std::slice::from_ref(body.as_ref()), local_count, functions, errors);
            }
            MirStmt::Break { .. } | MirStmt::Continue { .. } => {}
            MirStmt::ClassDecl { .. } => {}
            MirStmt::Export { expr, .. } | MirStmt::ModuleExportsAssign { expr } => {
                validate_mir_expr(expr, local_count, functions.len(), errors);
            }
        }
    }
}

fn validate_mir_expr(
    expr: &MirExpr,
    local_count: usize,
    _func_count: usize,
    errors: &mut Vec<Diagnostic>,
) {
    match expr {
        MirExpr::Local(id) => {
            if id.0 >= local_count {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "mir local id {} out of bounds (max {})",
                        id.0,
                        local_count.saturating_sub(1)
                    ),
                    span: None,
                    phase: None,
                });
            }
        }
        MirExpr::CallRuntime { args, .. }
        | MirExpr::CallFunction { args, .. } => {
            for arg in args {
                validate_mir_expr(arg, local_count, _func_count, errors);
            }
        }
        MirExpr::CallClosure { closure, args } => {
            validate_mir_expr(closure, local_count, _func_count, errors);
            for arg in args {
                validate_mir_expr(arg, local_count, _func_count, errors);
            }
        }
        MirExpr::NewObject { props } => {
            for (_, value) in props {
                validate_mir_expr(value, local_count, _func_count, errors);
            }
        }
        MirExpr::NewArray { elements } => {
            for elem in elements {
                validate_mir_expr(elem, local_count, _func_count, errors);
            }
        }
        MirExpr::Block { stmts, result } => {
            validate_mir_stmts(
                stmts,
                local_count,
                &[],
                errors,
            );
            validate_mir_expr(result, local_count, _func_count, errors);
        }
        MirExpr::I32Const(_)
        | MirExpr::StringConst(_)
        | MirExpr::LoadModule { .. } => {}
    }
}
