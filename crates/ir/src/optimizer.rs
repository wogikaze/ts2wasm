use ts2wasm_frontend::{DiagCode, Diagnostic};

use crate::semantic::{HirExpr, HirFunction, HirProgram, HirStmt, validate_hir};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    O0,
    O1,
    O2,
    O3,
}

impl OptimizationLevel {
    pub fn as_flag(self) -> &'static str {
        match self {
            Self::O0 => "-O0",
            Self::O1 => "-O1",
            Self::O2 => "-O2",
            Self::O3 => "-O3",
        }
    }
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::O0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationPass {
    LiteralNumericAddFold,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizedHirProgram {
    pub level: OptimizationLevel,
    pub applied_passes: Vec<OptimizationPass>,
    pub hir: HirProgram,
}

pub fn optimize_hir(
    program: &HirProgram,
    level: OptimizationLevel,
) -> Result<OptimizedHirProgram, Diagnostic> {
    let mut hir = program.clone();
    let mut applied_passes = Vec::new();

    if level != OptimizationLevel::O0 && fold_literal_numeric_add_program(&mut hir) {
        applied_passes.push(OptimizationPass::LiteralNumericAddFold);
    }

    validate_hir(&hir).map_err(|errs| {
        errs.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_hir failed with empty diagnostic list after optimization".to_owned(),
            span: None,
        })
    })?;

    Ok(OptimizedHirProgram {
        level,
        applied_passes,
        hir,
    })
}

fn fold_literal_numeric_add_program(program: &mut HirProgram) -> bool {
    let mut changed = fold_literal_numeric_add_stmts(&mut program.body);
    for function in &mut program.functions {
        changed |= fold_literal_numeric_add_function(function);
    }
    changed
}

fn fold_literal_numeric_add_function(function: &mut HirFunction) -> bool {
    fold_literal_numeric_add_stmts(&mut function.body)
}

fn fold_literal_numeric_add_stmts(stmts: &mut [HirStmt]) -> bool {
    let mut changed = false;
    for stmt in stmts {
        changed |= fold_literal_numeric_add_stmt(stmt);
    }
    changed
}

fn fold_literal_numeric_add_stmt(stmt: &mut HirStmt) -> bool {
    match stmt {
        HirStmt::Let { init, .. } => fold_literal_numeric_add_expr(init),
        HirStmt::StoreLocal { value, .. } => fold_literal_numeric_add_expr(value),
        HirStmt::Expr(expr) | HirStmt::Return(expr) => fold_literal_numeric_add_expr(expr),
        HirStmt::BranchIfTruthy {
            condition,
            then_body,
            else_body,
        } => {
            let mut changed = fold_literal_numeric_add_expr(condition);
            changed |= fold_literal_numeric_add_stmts(then_body);
            changed |= fold_literal_numeric_add_stmts(else_body);
            changed
        }
        HirStmt::LoopWhile { condition, body } => {
            let mut changed = fold_literal_numeric_add_expr(condition);
            changed |= fold_literal_numeric_add_stmts(body);
            changed
        }
    }
}

fn fold_literal_numeric_add_expr(expr: &mut HirExpr) -> bool {
    match expr {
        HirExpr::ToBoolean(inner) | HirExpr::JsUnaryNot(inner) | HirExpr::ArrayLength(inner) => {
            fold_literal_numeric_add_expr(inner)
        }
        HirExpr::JsAdd { left, right } => {
            let mut changed = fold_literal_numeric_add_expr(left);
            changed |= fold_literal_numeric_add_expr(right);
            if let (HirExpr::ConstNumber(left), HirExpr::ConstNumber(right)) =
                (left.as_ref(), right.as_ref())
                && let Some(value) = left.checked_add(*right)
            {
                *expr = HirExpr::ConstNumber(value);
                return true;
            }
            changed
        }
        HirExpr::JsStrictEqual { left, right }
        | HirExpr::JsAbstractEqual { left, right }
        | HirExpr::JsRelational { left, right, .. }
        | HirExpr::GetIndex {
            object: left,
            index: right,
        } => {
            let mut changed = fold_literal_numeric_add_expr(left);
            changed |= fold_literal_numeric_add_expr(right);
            changed
        }
        HirExpr::GetProp { object, .. } => fold_literal_numeric_add_expr(object),
        HirExpr::CallBuiltin { args, .. } | HirExpr::CallFunction { args, .. } => {
            fold_literal_numeric_add_args(args)
        }
        HirExpr::CallMethod { receiver, args, .. } => {
            let mut changed = fold_literal_numeric_add_expr(receiver);
            changed |= fold_literal_numeric_add_args(args);
            changed
        }
        HirExpr::ConstUndefined
        | HirExpr::ConstNull
        | HirExpr::ConstBool(_)
        | HirExpr::ConstNumber(_)
        | HirExpr::ConstString(_)
        | HirExpr::LoadLocal(_)
        | HirExpr::LoadBuiltin(_) => false,
    }
}

fn fold_literal_numeric_add_args(args: &mut [HirExpr]) -> bool {
    let mut changed = false;
    for arg in args {
        changed |= fold_literal_numeric_add_expr(arg);
    }
    changed
}
