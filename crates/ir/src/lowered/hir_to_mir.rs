use crate::lowered::hir::{HirBinaryOp, HirExpr, HirFunction, HirProgram, HirStmt};
use crate::lowered::mir::{
    MirExpr, MirFunction, MirProgram, MirStmt,
};
use crate::lowered::{FuncId, LocalId, ModuleInfo};
use ts2wasm_runtime_catalog::RuntimeFn;

/// Lower a HIR program to MIR.
///
/// This is the key transformation that bridges JavaScript-semantic operations
/// (HIR) to runtime ABI operations (MIR). Each HirExpr variant is mapped to
/// a MirExpr that uses RuntimeFn calls or WASM primitives.
pub fn lower_hir_to_mir(program: &HirProgram) -> MirProgram {
    let functions: Vec<MirFunction> = program
        .functions
        .iter()
        .map(|f| lower_hir_function(f, &program.functions))
        .collect();

    MirProgram {
        top_level_statements: lower_hir_stmts(&program.body, &program.functions, &[]),
        top_level_locals: program.locals.clone(),
        functions,
        modules: vec![],
    }
}

fn lower_hir_function(func: &HirFunction, all_functions: &[HirFunction]) -> MirFunction {
    MirFunction {
        id: func.id,
        params: func.params.clone(),
        uses_receiver: false,
        min_required_params: func.params.len(),
        rest_param_index: None,
        locals: func.locals.clone(),
        body: lower_hir_stmts(&func.body, all_functions, &func.params),
        recursion_depth: 0,
        is_async: false,
    }
}

fn lower_hir_stmts(
    stmts: &[HirStmt],
    all_functions: &[HirFunction],
    params: &[LocalId],
) -> Vec<MirStmt> {
    let local_count = params.len() + stmts.iter().filter_map(|s| match s {
        HirStmt::Let { local, .. } => Some(*local),
        _ => None,
    }).count();
    let mut lowered = Vec::new();

    for stmt in stmts {
        match stmt {
            HirStmt::Let { local, init } => {
                lowered.push(MirStmt::Let {
                    local: *local,
                    init: lower_hir_expr(init, local_count, all_functions.len()),
                });
            }
            HirStmt::Assign { local, expr } => {
                lowered.push(MirStmt::Assign {
                    local: *local,
                    init: lower_hir_expr(expr, local_count, all_functions.len()),
                });
            }
            HirStmt::Expr(expr) => {
                lowered.push(MirStmt::Expr(lower_hir_expr(
                    expr,
                    local_count,
                    all_functions.len(),
                )));
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                lowered.push(MirStmt::If {
                    condition: lower_hir_expr(condition, local_count, all_functions.len()),
                    then_body: lower_hir_stmts(then_body, all_functions, params),
                    else_body: lower_hir_stmts(else_body, all_functions, params),
                });
            }
            HirStmt::While { condition, body } => {
                lowered.push(MirStmt::While {
                    condition: lower_hir_expr(condition, local_count, all_functions.len()),
                    body: lower_hir_stmts(body, all_functions, params),
                });
            }
            HirStmt::Return(expr) => {
                lowered.push(MirStmt::Return(lower_hir_expr(
                    expr,
                    local_count,
                    all_functions.len(),
                )));
            }
            HirStmt::Throw(expr) => {
                lowered.push(MirStmt::Throw(lower_hir_expr(
                    expr,
                    local_count,
                    all_functions.len(),
                )));
            }
        }
    }

    lowered
}

fn lower_hir_expr(expr: &HirExpr, local_count: usize, _func_count: usize) -> MirExpr {
    match expr {
        HirExpr::Number(n) => MirExpr::I32Const(*n),
        HirExpr::String(s) => MirExpr::StringConst(s.clone()),
        HirExpr::Bool(b) => MirExpr::I32Const(if *b { 1 } else { 0 }),
        HirExpr::Null => MirExpr::I32Const(0),
        HirExpr::Undefined => MirExpr::I32Const(1),
        HirExpr::Local(id) => MirExpr::Local(*id),

        HirExpr::Unary { op, expr: inner } => {
            let runtime_fn = match op {
                crate::lowered::LoweredUnaryOp::Not => RuntimeFn::Not,
                crate::lowered::LoweredUnaryOp::Negate => RuntimeFn::Negate,
                crate::lowered::LoweredUnaryOp::TypeOf => RuntimeFn::TypeOf,
                crate::lowered::LoweredUnaryOp::Plus => RuntimeFn::EqualEqual,
                crate::lowered::LoweredUnaryOp::Delete | crate::lowered::LoweredUnaryOp::Void => {
                    return lower_hir_expr(inner, local_count, _func_count);
                }
            };
            MirExpr::CallRuntime {
                intrinsic: runtime_fn,
                args: vec![lower_hir_expr(inner, local_count, _func_count)],
            }
        }

        HirExpr::Binary { left, right, op } => {
            let runtime_fn = match op {
                HirBinaryOp::Add => RuntimeFn::Add,
                HirBinaryOp::Subtract => RuntimeFn::Sub,
                HirBinaryOp::Multiply => RuntimeFn::Mul,
                HirBinaryOp::Divide => RuntimeFn::Div,
                HirBinaryOp::Modulo => RuntimeFn::Mod,
                HirBinaryOp::Power => RuntimeFn::MathPow,
                HirBinaryOp::StrictEqual => RuntimeFn::StrictEqual,
                HirBinaryOp::EqualEqual => RuntimeFn::EqualEqual,
                HirBinaryOp::StrictNotEqual => RuntimeFn::StrictNotEqual,
                HirBinaryOp::BangEqual => RuntimeFn::BangEqual,
                HirBinaryOp::Less => RuntimeFn::Less,
                HirBinaryOp::LessEqual => RuntimeFn::LessEqual,
                HirBinaryOp::Greater => RuntimeFn::Greater,
                HirBinaryOp::GreaterEqual => RuntimeFn::GreaterEqual,
                HirBinaryOp::And | HirBinaryOp::Or => RuntimeFn::TruthyBool,
                HirBinaryOp::NullishCoalesce => RuntimeFn::EqualEqual,
                HirBinaryOp::BitwiseAnd => RuntimeFn::BitwiseAnd,
                HirBinaryOp::BitwiseXor => RuntimeFn::BitwiseXor,
                HirBinaryOp::BitwiseOr => RuntimeFn::BitwiseOr,
                HirBinaryOp::LeftShift | HirBinaryOp::RightShift | HirBinaryOp::UnsignedRightShift => {
                    RuntimeFn::BitwiseToI32
                }
                HirBinaryOp::In => RuntimeFn::PropertyHas,
                HirBinaryOp::InstanceOf => RuntimeFn::InstanceOf,
                HirBinaryOp::Exponentiate => RuntimeFn::MathPow,
            };
            MirExpr::CallRuntime {
                intrinsic: runtime_fn,
                args: vec![
                    lower_hir_expr(left, local_count, _func_count),
                    lower_hir_expr(right, local_count, _func_count),
                ],
            }
        }

        HirExpr::GetProp { object, key } => MirExpr::CallRuntime {
            intrinsic: RuntimeFn::PropertyGet,
            args: vec![
                lower_hir_expr(object, local_count, _func_count),
                MirExpr::StringConst(key.clone()),
            ],
        },

        HirExpr::GetIndex { object, index } => MirExpr::CallRuntime {
            intrinsic: RuntimeFn::Index,
            args: vec![
                lower_hir_expr(object, local_count, _func_count),
                lower_hir_expr(index, local_count, _func_count),
            ],
        },

        HirExpr::SetProp { object, key, value } => MirExpr::CallRuntime {
            intrinsic: RuntimeFn::PropertySet,
            args: vec![
                lower_hir_expr(object, local_count, _func_count),
                MirExpr::StringConst(key.clone()),
                lower_hir_expr(value, local_count, _func_count),
            ],
        },

        HirExpr::SetIndex {
            object,
            index,
            value,
        } => MirExpr::CallRuntime {
            intrinsic: RuntimeFn::PropertySet,
            args: vec![
                lower_hir_expr(object, local_count, _func_count),
                lower_hir_expr(index, local_count, _func_count),
                lower_hir_expr(value, local_count, _func_count),
            ],
        },

        HirExpr::HasProperty { object, key } => MirExpr::CallRuntime {
            intrinsic: RuntimeFn::PropertyHas,
            args: vec![
                lower_hir_expr(object, local_count, _func_count),
                lower_hir_expr(key, local_count, _func_count),
            ],
        },

        HirExpr::DeleteProperty { object, key } => MirExpr::CallRuntime {
            intrinsic: RuntimeFn::PropertyDelete,
            args: vec![
                lower_hir_expr(object, local_count, _func_count),
                lower_hir_expr(key, local_count, _func_count),
            ],
        },

        HirExpr::ObjectLiteral { props } => MirExpr::NewObject {
            props: props
                .iter()
                .map(|(k, v)| {
                    (k.clone(), lower_hir_expr(v, local_count, _func_count))
                })
                .collect(),
        },

        HirExpr::ArrayLiteral { elements } => MirExpr::NewArray {
            elements: elements
                .iter()
                .map(|e| lower_hir_expr(e, local_count, _func_count))
                .collect(),
        },

        HirExpr::Call { callee, args } => {
            let lowered_args: Vec<MirExpr> = args
                .iter()
                .map(|a| lower_hir_expr(a, local_count, _func_count))
                .collect();

            match callee.as_ref() {
                HirExpr::Local(lid) if lid.0 < _func_count => MirExpr::CallFunction {
                    func: FuncId(lid.0),
                    args: lowered_args,
                },
                _ => MirExpr::CallRuntime {
                    intrinsic: RuntimeFn::HeapClosureCall,
                    args: std::iter::once(lower_hir_expr(callee, local_count, _func_count))
                        .chain(lowered_args)
                        .collect(),
                },
            }
        }

        HirExpr::MethodCall {
            receiver,
            method,
            args,
        } => {
            let lowered_args: Vec<MirExpr> = vec![lower_hir_expr(receiver, local_count, _func_count)]
                .into_iter()
                .chain(args.iter().map(|a| lower_hir_expr(a, local_count, _func_count)))
                .collect();

            MirExpr::CallRuntime {
                intrinsic: RuntimeFn::PropertyGet,
                args: vec![
                    lower_hir_expr(receiver, local_count, _func_count),
                    MirExpr::StringConst(method.clone()),
                ],
            }
        }

        HirExpr::New { constructor: _id, args } => MirExpr::CallRuntime {
            intrinsic: RuntimeFn::AllocHeap,
            args: args
                .iter()
                .map(|a| lower_hir_expr(a, local_count, _func_count))
                .collect(),
        },

        HirExpr::If {
            condition,
            then_expr,
            else_expr,
        } => MirExpr::Block {
            stmts: vec![MirStmt::If {
                condition: lower_hir_expr(condition, local_count, _func_count),
                then_body: vec![MirStmt::Expr(lower_hir_expr(
                    then_expr,
                    local_count,
                    _func_count,
                ))],
                else_body: vec![MirStmt::Expr(lower_hir_expr(
                    else_expr,
                    local_count,
                    _func_count,
                ))],
            }],
            result: Box::new(MirExpr::I32Const(0)),
        },
    }
}
