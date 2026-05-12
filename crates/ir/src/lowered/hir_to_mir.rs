// HIR to MIR lowering — translates HirProgram into LoweredProgram.
//
// This is a straightforward structural translation that maps each HirStmt
// and HirExpr variant to its closest LoweredStmt / LoweredExpr equivalent.
// Complex runtime-level lowering (e.g., full method call dispatch) is
// represented as RuntimeCall nodes that downstream passes expand further.

use crate::{HirExpr, HirFunction, HirProgram, HirRelationalOp, HirStmt};
use super::*;

/// Lower a HirProgram to a LoweredProgram (MIR).
pub fn lower_hir_to_mir(program: &HirProgram) -> LoweredProgram {
    let lowerer = HirToMirLowerer::new(program);
    lowerer.lower_program()
}

struct HirToMirLowerer {
    body: Vec<HirStmt>,
    locals_map: Vec<LocalId>,
    functions: Vec<HirFunction>,
}

impl HirToMirLowerer {
    fn new(program: &HirProgram) -> Self {
        let locals_map: Vec<LocalId> = program.locals.iter().map(|id| LocalId(id.0)).collect();
        Self {
            body: program.body.clone(),
            locals_map,
            functions: program.functions.clone(),
        }
    }

    fn lower_program(&self) -> LoweredProgram {
        LoweredProgram {
            top_level_statements: self.lower_stmts(&self.body),
            top_level_locals: self.locals_map.clone(),
            functions: self.lower_functions(),
            modules: vec![],
        }
    }

    fn lower_functions(&self) -> Vec<LoweredFunction> {
        self.functions
            .iter()
            .map(|hir_fn| {
                let lowered_locals: Vec<LocalId> =
                    hir_fn.locals.iter().map(|id| LocalId(id.0)).collect();
                LoweredFunction {
                    id: FuncId(hir_fn.id.0),
                    params: hir_fn.params.iter().map(|id| LocalId(id.0)).collect(),
                    uses_receiver: false,
                    min_required_params: hir_fn.params.len(),
                    rest_param_index: None,
                    locals: lowered_locals,
                    body: self.lower_stmts(&hir_fn.body),
                    recursion_depth: 0,
                    is_async: false,
                }
            })
            .collect()
    }
}

impl HirToMirLowerer {
    fn lower_stmts(&self, stmts: &[HirStmt]) -> Vec<LoweredStmt> {
        stmts.iter().map(|s| self.lower_stmt(s)).collect()
    }

    fn lower_stmt(&self, stmt: &HirStmt) -> LoweredStmt {
        let span = ts2wasm_source::Span::default();
        match stmt {
            HirStmt::Let { local, init } => {
                LoweredStmt::Let(LocalId(local.0), self.lower_expr(init), span)
            }
            HirStmt::StoreLocal { local, value } => {
                LoweredStmt::Assign(LocalId(local.0), self.lower_expr(value), span)
            }
            HirStmt::Expr(expr) => LoweredStmt::Expr(self.lower_expr(expr), span),
            HirStmt::BranchIfTruthy {
                condition,
                then_body,
                else_body,
            } => LoweredStmt::If {
                condition: self.lower_expr(condition),
                then_body: self.lower_stmts(then_body),
                else_body: self.lower_stmts(else_body),
                span,
            },
            HirStmt::LoopWhile { condition, body } => LoweredStmt::While {
                condition: self.lower_expr(condition),
                body: self.lower_stmts(body),
                span,
            },
            HirStmt::Return(expr) => LoweredStmt::Return(self.lower_expr(expr), span),
        }
    }

    fn lower_expr(&self, expr: &HirExpr) -> LoweredExpr {
        let span = ts2wasm_source::Span::default();
        match expr {
            HirExpr::ConstUndefined => LoweredExpr::Undefined(span),
            HirExpr::ConstNull => LoweredExpr::Null(span),
            HirExpr::ConstBool(b) => LoweredExpr::Bool(*b, span),
            HirExpr::ConstNumber(n) => LoweredExpr::Number(*n, span),
            HirExpr::ConstBigInt(decimal) => {
                let (sign, limb_low, limb_high) = parse_bigint(decimal);
                LoweredExpr::BigIntLiteral {
                    decimal: decimal.clone(),
                    sign,
                    limb_low,
                    limb_high,
                    span,
                }
            }
            HirExpr::ConstString(s) => LoweredExpr::String(s.clone(), span),
            HirExpr::LoadLocal(id) => LoweredExpr::Local(LocalId(id.0), span),
            HirExpr::LoadBuiltin(name) => LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::PropertyGet,
                args: vec![
                    LoweredExpr::This(span),
                    LoweredExpr::String(name.clone(), span),
                ],
                span,
            },
            HirExpr::ToBoolean(inner) => LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::TruthyBool,
                args: vec![self.lower_expr(inner)],
                span,
            },
            HirExpr::JsUnaryNot(inner) => LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(self.lower_expr(inner)),
                span,
            },
            HirExpr::JsAdd { left, right } => LoweredExpr::Binary {
                left: Box::new(self.lower_expr(left)),
                op: LoweredBinaryOp::Add,
                right: Box::new(self.lower_expr(right)),
                span,
            },
            HirExpr::JsStrictEqual { left, right } => LoweredExpr::Binary {
                left: Box::new(self.lower_expr(left)),
                op: LoweredBinaryOp::StrictEqual,
                right: Box::new(self.lower_expr(right)),
                span,
            },
            HirExpr::JsAbstractEqual { left, right } => LoweredExpr::Binary {
                left: Box::new(self.lower_expr(left)),
                op: LoweredBinaryOp::EqualEqual,
                right: Box::new(self.lower_expr(right)),
                span,
            },
            HirExpr::JsRelational {
                op: hir_op,
                left,
                right,
            } => {
                let mir_op = match hir_op {
                    HirRelationalOp::Less => LoweredBinaryOp::Less,
                    HirRelationalOp::LessEqual => LoweredBinaryOp::LessEqual,
                    HirRelationalOp::Greater => LoweredBinaryOp::Greater,
                    HirRelationalOp::GreaterEqual => LoweredBinaryOp::GreaterEqual,
                };
                LoweredExpr::Binary {
                    left: Box::new(self.lower_expr(left)),
                    op: mir_op,
                    right: Box::new(self.lower_expr(right)),
                    span,
                }
            }
            HirExpr::GetProp { object, key } => LoweredExpr::PropertyGet {
                obj: Box::new(self.lower_expr(object)),
                key: key.clone(),
                span,
            },
            HirExpr::GetIndex { object, index } => LoweredExpr::PropertyGetDynamic {
                obj: Box::new(self.lower_expr(object)),
                key: Box::new(self.lower_expr(index)),
                span,
            },
            HirExpr::ArrayLength(inner) => {
                LoweredExpr::GetLength(Box::new(self.lower_expr(inner)), span)
            }
            HirExpr::CallBuiltin { builtin, args } => LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(*builtin),
                args: self.lower_exprs(args),
                span,
            },
            HirExpr::CallFunction { function, args } => LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(function.0)),
                args: self.lower_exprs(args),
                span,
            },
            HirExpr::CallMethod {
                receiver,
                method,
                args,
            } => {
                let mut lowered_args = vec![
                    self.lower_expr(receiver),
                    LoweredExpr::String(method.clone(), span),
                ];
                lowered_args.extend(self.lower_exprs(args));
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::PropertyGet,
                    args: lowered_args,
                    span,
                }
            }
        }
    }

    fn lower_exprs(&self, exprs: &[HirExpr]) -> Vec<LoweredExpr> {
        exprs.iter().map(|e| self.lower_expr(e)).collect()
    }
}

/// Parse a BigInt decimal string into (sign, limb_low, limb_high).
fn parse_bigint(decimal: &str) -> (i32, u32, u32) {
    let trimmed = decimal.trim_start_matches('-');
    let is_negative = decimal != trimmed;
    let u64_val: u64 = trimmed.parse().unwrap_or(0);
    let sign = if is_negative { -1 } else { 1 };
    (sign, u64_val as u32, (u64_val >> 32) as u32)
}
