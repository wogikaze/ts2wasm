use super::mir::{MirExpr, MirProgram, MirStmt};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};

/// Validate a `MirProgram` for internal consistency.
///
/// Checks:
/// - Function ids match their positional indices
/// - Local ids are within declared bounds
/// - No invalid references
pub fn validate_mir(program: &MirProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();

    for local in &program.top_level_locals {
        validate_local_id(
            *local,
            program.top_level_locals.len(),
            "mir top-level local",
            &mut errors,
        );
    }

    for (idx, func) in program.functions.iter().enumerate() {
        if func.id.0 != idx {
            push_invariant(
                &mut errors,
                format!(
                    "mir function id {} does not match its index {}",
                    func.id.0, idx
                ),
            );
        }
    }

    validate_mir_stmts(
        &program.top_level_statements,
        program.top_level_locals.len(),
        program.functions.len(),
        program.modules.len(),
        true,
        &mut errors,
    );

    for func in &program.functions {
        let local_count = func.params.len() + func.locals.len();
        for param in &func.params {
            validate_local_id(*param, local_count, "mir function param", &mut errors);
        }
        for local in &func.locals {
            validate_local_id(*local, local_count, "mir function local", &mut errors);
        }
        validate_mir_stmts(
            &func.body,
            local_count,
            program.functions.len(),
            program.modules.len(),
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

fn validate_mir_stmts(
    stmts: &[MirStmt],
    local_count: usize,
    func_count: usize,
    module_count: usize,
    top_level: bool,
    errors: &mut Vec<Diagnostic>,
) {
    for stmt in stmts {
        match stmt {
            MirStmt::Let { local, init } => {
                validate_local_id(*local, local_count, "mir let local", errors);
                validate_mir_expr(init, local_count, func_count, module_count, errors);
            }
            MirStmt::Assign { local, init } => {
                validate_local_id(*local, local_count, "mir assign local", errors);
                validate_mir_expr(init, local_count, func_count, module_count, errors);
            }
            MirStmt::Expr(expr) => {
                validate_mir_expr(expr, local_count, func_count, module_count, errors);
            }
            MirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                validate_mir_expr(condition, local_count, func_count, module_count, errors);
                validate_mir_stmts(
                    then_body,
                    local_count,
                    func_count,
                    module_count,
                    top_level,
                    errors,
                );
                validate_mir_stmts(
                    else_body,
                    local_count,
                    func_count,
                    module_count,
                    top_level,
                    errors,
                );
            }
            MirStmt::While { condition, body } => {
                validate_mir_expr(condition, local_count, func_count, module_count, errors);
                validate_mir_stmts(
                    body,
                    local_count,
                    func_count,
                    module_count,
                    top_level,
                    errors,
                );
            }
            MirStmt::Return(expr) => {
                if top_level {
                    errors.push(Diagnostic {
                        code: DiagCode::InvalidTopLevelReturn,
                        message: "mir top-level return is invalid".to_owned(),
                        span: None,
                        phase: None,
                    });
                }
                validate_mir_expr(expr, local_count, func_count, module_count, errors);
            }
            MirStmt::Throw(expr) => {
                validate_mir_expr(expr, local_count, func_count, module_count, errors);
            }
            MirStmt::TryCatch {
                try_body,
                catch_var,
                catch_body,
                finally_body,
                ..
            } => {
                if let Some(catch_var) = catch_var {
                    validate_local_id(*catch_var, local_count, "mir catch local", errors);
                }
                validate_mir_stmts(
                    try_body,
                    local_count,
                    func_count,
                    module_count,
                    top_level,
                    errors,
                );
                if let Some(body) = catch_body {
                    validate_mir_stmts(
                        body,
                        local_count,
                        func_count,
                        module_count,
                        top_level,
                        errors,
                    );
                }
                if let Some(body) = finally_body {
                    validate_mir_stmts(
                        body,
                        local_count,
                        func_count,
                        module_count,
                        top_level,
                        errors,
                    );
                }
            }
            MirStmt::Switch { expr, cases } => {
                validate_mir_expr(expr, local_count, func_count, module_count, errors);
                for (case_expr, body) in cases {
                    if let Some(case_expr) = case_expr {
                        validate_mir_expr(case_expr, local_count, func_count, module_count, errors);
                    }
                    validate_mir_stmts(
                        body,
                        local_count,
                        func_count,
                        module_count,
                        top_level,
                        errors,
                    );
                }
            }
            MirStmt::Labeled { body, .. } => {
                validate_mir_stmts(
                    std::slice::from_ref(body.as_ref()),
                    local_count,
                    func_count,
                    module_count,
                    top_level,
                    errors,
                );
            }
            MirStmt::Break { .. } | MirStmt::Continue { .. } => {}
            MirStmt::ClassDecl {
                constructor,
                methods,
                static_methods,
                ..
            } => {
                if let Some(constructor) = constructor {
                    validate_func_id(*constructor, func_count, "mir class constructor", errors);
                }
                for (_, method) in methods {
                    validate_func_id(*method, func_count, "mir class method", errors);
                }
                for (_, method) in static_methods {
                    validate_func_id(*method, func_count, "mir static class method", errors);
                }
            }
            MirStmt::Export { expr, .. } | MirStmt::ModuleExportsAssign { expr } => {
                validate_mir_expr(expr, local_count, func_count, module_count, errors);
            }
        }
    }
}

fn validate_mir_expr(
    expr: &MirExpr,
    local_count: usize,
    _func_count: usize,
    module_count: usize,
    errors: &mut Vec<Diagnostic>,
) {
    match expr {
        MirExpr::Local(id) => {
            validate_local_id(*id, local_count, "mir local reference", errors);
        }
        MirExpr::CallRuntime { args, .. } => {
            for arg in args {
                validate_mir_expr(arg, local_count, _func_count, module_count, errors);
            }
        }
        MirExpr::CallFunction { func, args } => {
            validate_func_id(*func, _func_count, "mir function reference", errors);
            for arg in args {
                validate_mir_expr(arg, local_count, _func_count, module_count, errors);
            }
        }
        MirExpr::CallClosure { closure, args } => {
            validate_mir_expr(closure, local_count, _func_count, module_count, errors);
            for arg in args {
                validate_mir_expr(arg, local_count, _func_count, module_count, errors);
            }
        }
        MirExpr::NewObject { props } => {
            for (_, value) in props {
                validate_mir_expr(value, local_count, _func_count, module_count, errors);
            }
        }
        MirExpr::NewArray { elements } => {
            for elem in elements {
                validate_mir_expr(elem, local_count, _func_count, module_count, errors);
            }
        }
        MirExpr::LoadModule { module_id } => {
            if *module_id >= module_count {
                push_invariant(
                    errors,
                    format!(
                        "mir module id {} out of bounds (max {})",
                        module_id,
                        module_count.saturating_sub(1)
                    ),
                );
            }
        }
        MirExpr::Block { stmts, result } => {
            validate_mir_stmts(stmts, local_count, _func_count, module_count, false, errors);
            validate_mir_expr(result, local_count, _func_count, module_count, errors);
        }
        MirExpr::I32Const(_) | MirExpr::StringConst(_) => {}
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
