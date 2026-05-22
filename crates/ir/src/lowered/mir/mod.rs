// Native MIR types — independent data model with LoweredProgram compatibility bridge.
//
// These four core types (MirProgram, MirFunction, MirStmt, MirExpr) are native
// Rust types, not type aliases. They carry the same structure as the Lowered*
// equivalents so that From bridges can convert losslessly.
//
// MirExpr uses LoweredBinaryOp, LoweredUnaryOp, etc. directly for its field
// types rather than creating separate Mir* aliases. The original type names
// are already publicly available from `crate::lowered::*`.

pub mod escape;
pub mod induction_var;
mod lower;
mod raise;
pub mod scalar_replace;
pub mod types;
pub mod value_rep;

pub use induction_var::{InductionVarDirection, InductionVarInfo};
pub use types::{
    EscapeStatus, MirArraySlot, MirBinaryOp, MirBuiltinErrorConstructor, MirClassPrototypeRef,
    MirClosureRepresentation, MirExpr, MirFunction, MirFunctionCallKind, MirLogicalAssignOp,
    MirModuleInfo, MirProgram, MirStmt, MirUnaryOp, OptimizationHint, RepProof, ValueRep,
};

// ---------------------------------------------------------------------------
// MirExpr helper
// ---------------------------------------------------------------------------

impl MirExpr {
    /// Infer a conservative type for this MIR expression.
    /// Mirrors `LoweredExpr::inferred_type`.
    pub fn inferred_type(&self) -> super::InferredType {
        match self {
            Self::Number(_, _) | Self::DecimalNumber(_, _) => super::InferredType::Number,
            Self::BigIntLiteral { .. } => super::InferredType::Unknown,
            Self::String(_, _) => super::InferredType::String,
            Self::Bool(_, _) => super::InferredType::Boolean,
            Self::Unary { op, expr, .. } => match op {
                LoweredUnaryOp::Plus if expr.inferred_type() == super::InferredType::Number => {
                    super::InferredType::Number
                }
                LoweredUnaryOp::Negate if expr.inferred_type() == super::InferredType::Number => {
                    super::InferredType::Number
                }
                LoweredUnaryOp::Not => super::InferredType::Boolean,
                _ => super::InferredType::Unknown,
            },
            Self::Binary {
                left, op, right, ..
            } => match op {
                LoweredBinaryOp::Add => match (left.inferred_type(), right.inferred_type()) {
                    (super::InferredType::Number, super::InferredType::Number) => {
                        super::InferredType::Number
                    }
                    (super::InferredType::String, super::InferredType::String) => {
                        super::InferredType::String
                    }
                    _ => super::InferredType::Unknown,
                },
                LoweredBinaryOp::Subtract
                | LoweredBinaryOp::Multiply
                | LoweredBinaryOp::Power
                | LoweredBinaryOp::Divide
                | LoweredBinaryOp::Modulo
                | LoweredBinaryOp::BitwiseAnd
                | LoweredBinaryOp::BitwiseXor
                | LoweredBinaryOp::BitwiseOr => {
                    if left.inferred_type() == super::InferredType::Number
                        && right.inferred_type() == super::InferredType::Number
                    {
                        super::InferredType::Number
                    } else {
                        super::InferredType::Unknown
                    }
                }
                LoweredBinaryOp::Less
                | LoweredBinaryOp::LessEqual
                | LoweredBinaryOp::Greater
                | LoweredBinaryOp::GreaterEqual
                | LoweredBinaryOp::StrictEqual
                | LoweredBinaryOp::EqualEqual
                | LoweredBinaryOp::BangEqual
                | LoweredBinaryOp::StrictNotEqual => super::InferredType::Boolean,
                LoweredBinaryOp::And | LoweredBinaryOp::Or | LoweredBinaryOp::NullishCoalesce => {
                    super::InferredType::Unknown
                }
            },
            Self::Assign { expr, .. } => expr.inferred_type(),
            Self::LogicalAssign { .. }
            | Self::LogicalPropertyAssign { .. }
            | Self::LogicalMemberAssign { .. }
            | Self::LogicalComputedMemberAssign { .. }
            | Self::LogicalComputedPropertyAssign { .. } => super::InferredType::Unknown,
            _ => super::InferredType::Unknown,
        }
    }
}

use crate::lowered::{LoweredBinaryOp, LoweredUnaryOp};

// ---------------------------------------------------------------------------
// Induction variable analysis pass
// ---------------------------------------------------------------------------

/// Run induction variable analysis on all functions in a `MirProgram`.
///
/// This is an idempotent pass that populates `induction_vars` on every
/// `MirFunction` that contains detectable for-loop induction variables.
///
/// Calling it multiple times is safe — each call replaces the previous
/// results rather than appending.
pub fn run_induction_var_analysis(program: &mut MirProgram) {
    for func in &mut program.functions {
        func.induction_vars = induction_var::analyze_function(func);
    }
}

// ---------------------------------------------------------------------------
// Escape analysis pass
// ---------------------------------------------------------------------------

/// Run escape analysis on the entire `MirProgram`.
///
/// Populates `escape_status` on every `MirFunction` and on the top-level
/// program. Must be called **before** `run_scalar_replacement` since that
/// pass depends on escape analysis results.
///
/// Calling this multiple times is safe — each call recomputes the analysis
/// from scratch.
pub fn run_escape_analysis(program: &mut MirProgram) {
    escape::analyze_escape(program);
}

// ---------------------------------------------------------------------------
// Scalar replacement pass
// ---------------------------------------------------------------------------

/// Run scalar replacement on the entire `MirProgram`.
///
/// Replaces non-escaping objects (determined by escape analysis) with
/// individual locals for each property. Must be called **after**
/// `run_escape_analysis` so that `escape_status` is populated.
pub fn run_scalar_replacement(program: &mut MirProgram) {
    scalar_replace::scalar_replace(program);
}

// ---------------------------------------------------------------------------
// Value representation inference pass
// ---------------------------------------------------------------------------

/// Run value representation inference on all functions in a `MirProgram`.
///
/// Populates `value_reps` on every `MirFunction` by examining the RHS
/// expressions of `Let` and `Assign` statements. Locals initialized with
/// literal expressions (numbers, booleans, strings, null) get a concrete
/// `ValueRep`; all others remain `None` (JsVal fallback).
///
/// This pass is independent of escape analysis and scalar replacement.
pub fn run_value_rep_analysis(program: &mut MirProgram) {
    for func in &mut program.functions {
        let local_count = func.locals.len();
        if func.value_reps.len() < local_count {
            func.value_reps.resize(local_count, None);
        }
        infer_value_reps_in_stmts(&func.body, &mut func.value_reps);
    }
}

/// Walk a list of statements and infer value reps for locals defined via
/// `Let` or `Assign`.
fn infer_value_reps_in_stmts(
    stmts: &[MirStmt],
    value_reps: &mut Vec<Option<(ValueRep, RepProof)>>,
) {
    for stmt in stmts {
        match stmt {
            MirStmt::Let(local, expr, _) | MirStmt::Assign(local, expr, _) => {
                let idx = local.0 as usize;
                if idx < value_reps.len() {
                    if let Some(rep) = value_rep::infer_expr_rep(expr) {
                        value_reps[idx] = Some(rep);
                    }
                }
            }
            // Recurse into nested statement containers.
            MirStmt::Block(children, _) => {
                infer_value_reps_in_stmts(children, value_reps);
            }
            MirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                infer_value_reps_in_stmts(then_body, value_reps);
                infer_value_reps_in_stmts(else_body, value_reps);
            }
            MirStmt::While { body, .. } => {
                infer_value_reps_in_stmts(body, value_reps);
            }
            MirStmt::For { init, body, .. } => {
                if let Some(init_stmt) = init {
                    infer_value_reps_in_stmts(&[init_stmt.as_ref().clone()], value_reps);
                }
                infer_value_reps_in_stmts(body, value_reps);
            }
            MirStmt::DoWhile { body, .. } => {
                infer_value_reps_in_stmts(body, value_reps);
            }
            MirStmt::ForIn { body, .. }
            | MirStmt::ForOf { body, .. }
            | MirStmt::ForAwaitOfLower { body, .. } => {
                infer_value_reps_in_stmts(body, value_reps);
            }
            MirStmt::TryFinally {
                try_body,
                finally_body,
                ..
            } => {
                infer_value_reps_in_stmts(try_body, value_reps);
                infer_value_reps_in_stmts(finally_body, value_reps);
            }
            MirStmt::TryCatch {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                infer_value_reps_in_stmts(try_body, value_reps);
                if let Some(body) = catch_body {
                    infer_value_reps_in_stmts(body, value_reps);
                }
                if let Some(body) = finally_body {
                    infer_value_reps_in_stmts(body, value_reps);
                }
            }
            MirStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    infer_value_reps_in_stmts(body, value_reps);
                }
            }
            MirStmt::Labeled { body, .. } => {
                infer_value_reps_in_stmts(&[body.as_ref().clone()], value_reps);
            }
            // Statements that do not introduce or modify locals.
            MirStmt::Expr(..)
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
}

// ---------------------------------------------------------------------------
// Orchestration: run all analysis passes in the correct order
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Value representation consumer pass
// ---------------------------------------------------------------------------

/// Consume value representation inference results and produce optimization hints.
pub fn run_value_rep_consumer(program: &mut MirProgram) {
    for func in &mut program.functions {
        let local_count = func.locals.len();
        if func.optimization_hints.len() < local_count {
            func.optimization_hints
                .resize(local_count, OptimizationHint::None);
        }
        for (i, hint) in func.optimization_hints.iter_mut().enumerate() {
            if i >= func.value_reps.len() {
                *hint = OptimizationHint::None;
                continue;
            }
            *hint = match func.value_reps[i] {
                Some((ValueRep::SmiI32, _)) => OptimizationHint::UnboxedSmi,
                Some((ValueRep::BoolI32, _)) => OptimizationHint::UnboxedBool,
                Some((ValueRep::StringRef, _)) => OptimizationHint::DirectStringRef,
                Some((ValueRep::ObjectRef, _)) => OptimizationHint::DirectObjectRef,
                Some((ValueRep::ArrayRef, _)) => OptimizationHint::DirectArrayRef,
                Some((ValueRep::RawI32, _)) => OptimizationHint::UnboxedRawI32,
                Some((ValueRep::JsVal, _)) | None => OptimizationHint::None,
            };
        }
    }
}

/// Run all MIR analysis passes in the correct order.
///
/// ## Pipeline order
///
/// 1. **Escape analysis** (`run_escape_analysis`) — marks which locals
///    escape their function scope. Required by scalar replacement.
///
/// 2. **Scalar replacement** (`run_scalar_replacement`) — replaces
///    non-escaping objects with per-property locals. Depends on escape
///    analysis results.
///
/// 3. **Induction variable analysis** (`run_induction_var_analysis`) —
///    detects for-loop induction variables. Independent of 1 and 2.
///
/// 4. **Value representation inference** (`run_value_rep_analysis`) —
///    infers per-local value representations. Independent of 1-3.
///
/// 5. **Value representation consumer** (`run_value_rep_consumer`) —
///    reads value_reps and produces optimization_hints per local.
///    Must run after step 4.
pub fn run_all_mir_analyses(program: &mut MirProgram) {
    run_escape_analysis(program);
    run_scalar_replacement(program);
    run_induction_var_analysis(program);
    run_value_rep_analysis(program);
    run_value_rep_consumer(program);
}

// ---------------------------------------------------------------------------
// Tests: bridge conversions preserve structure
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::raise::mir_expr_to_lower;
    use super::types::*;
    use crate::lowered::RuntimeFn;
    use crate::lowered::{
        ClassPrototypeRef, FuncId, FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr,
        LoweredFunction, LoweredProgram, LoweredStmt, LoweredUnaryOp, ModuleLoadKind,
    };
    use ts2wasm_source::Span;

    fn make_span() -> Span {
        Span { start: 0, end: 0 }
    }

    fn sample_lowered_program() -> LoweredProgram {
        LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::Number(42, make_span()),
                    make_span(),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(crate::builtin::BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Bool(true, make_span())],
                        span: make_span(),
                    },
                    make_span(),
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0)],
                uses_receiver: false,
                min_required_params: 1,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![LocalId(0)],
                body: vec![LoweredStmt::Return(
                    LoweredExpr::Local(LocalId(0), make_span()),
                    make_span(),
                )],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        }
    }

    fn assert_mir_expr_roundtrip(expr: LoweredExpr) {
        let mir: MirExpr = super::lower::lower_expr_to_mir(&expr);
        let lowered_back: LoweredExpr = mir_expr_to_lower(&mir);
        assert_eq!(
            expr, lowered_back,
            "MirExpr roundtrip failed for {:?}",
            expr
        );
    }

    #[test]
    fn bridge_lowered_to_mir_roundtrip_full_program() {
        let lowered = sample_lowered_program();
        let mir: MirProgram = lowered.clone().into();
        let lowered_back: LoweredProgram = mir.into();
        assert_eq!(
            lowered, lowered_back,
            "Full program bridge roundtrip should preserve all data"
        );
    }

    #[test]
    fn bridge_lowered_to_mir_preserves_top_level_structure() {
        let lowered = sample_lowered_program();
        let mir: MirProgram = lowered.clone().into();
        assert_eq!(mir.top_level_locals, lowered.top_level_locals);
        assert_eq!(mir.functions.len(), lowered.functions.len());
        assert_eq!(mir.modules.len(), lowered.modules.len());
    }

    #[test]
    fn bridge_lowered_to_mir_preserves_functions() {
        let lowered = sample_lowered_program();
        let mir: MirProgram = lowered.clone().into();
        let mir_fn = &mir.functions[0];
        let lowered_fn = &lowered.functions[0];
        assert_eq!(mir_fn.id, lowered_fn.id);
        assert_eq!(mir_fn.params, lowered_fn.params);
        assert_eq!(mir_fn.uses_receiver, lowered_fn.uses_receiver);
        assert_eq!(mir_fn.min_required_params, lowered_fn.min_required_params);
        assert_eq!(mir_fn.rest_param_index, lowered_fn.rest_param_index);
        assert_eq!(mir_fn.locals, lowered_fn.locals);
        assert_eq!(mir_fn.body.len(), lowered_fn.body.len());
        assert_eq!(mir_fn.recursion_depth, lowered_fn.recursion_depth);
        assert_eq!(mir_fn.is_async, lowered_fn.is_async);
    }

    #[test]
    fn bridge_lowered_to_mir_preserves_stmts() {
        let lowered = sample_lowered_program();
        let mir: MirProgram = lowered.clone().into();
        assert_eq!(
            mir.top_level_statements.len(),
            lowered.top_level_statements.len()
        );

        // Let stmt
        let mir_stmt = &mir.top_level_statements[0];
        let lowered_stmt = &lowered.top_level_statements[0];
        match (mir_stmt, lowered_stmt) {
            (
                MirStmt::Let(mir_local, mir_expr, _),
                LoweredStmt::Let(lowered_local, lowered_expr, _),
            ) => {
                assert_eq!(mir_local, lowered_local);
                assert_eq!(mir_expr_to_lower(mir_expr), lowered_expr.clone());
            }
            _ => panic!("Expected Let stmt at index 0"),
        }
    }

    #[test]
    fn bridge_mir_expr_roundtrip_constants() {
        assert_mir_expr_roundtrip(LoweredExpr::Number(42, make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::String("hi".to_string(), make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::Bool(true, make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::Null(make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::Undefined(make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::Local(LocalId(0), make_span()));
    }

    #[test]
    fn bridge_mir_expr_roundtrip_complex() {
        assert_mir_expr_roundtrip(LoweredExpr::Binary {
            left: Box::new(LoweredExpr::Number(1, make_span())),
            op: LoweredBinaryOp::Add,
            right: Box::new(LoweredExpr::Number(2, make_span())),
            span: make_span(),
        });

        assert_mir_expr_roundtrip(LoweredExpr::Call {
            kind: FunctionCallKind::User(FuncId(0)),
            args: vec![LoweredExpr::Number(42, make_span())],
            span: make_span(),
        });

        assert_mir_expr_roundtrip(LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::ArrayPushGrow,
            args: vec![
                LoweredExpr::Local(LocalId(0), make_span()),
                LoweredExpr::Number(1, make_span()),
            ],
            span: make_span(),
        });
    }

    #[test]
    fn bridge_mir_expr_roundtrip_object_and_array() {
        assert_mir_expr_roundtrip(LoweredExpr::ObjectNew {
            props: vec![("x".to_string(), LoweredExpr::Number(1, make_span()))],
            non_enumerable: 0,
            span: make_span(),
        });

        assert_mir_expr_roundtrip(LoweredExpr::ArrayNew {
            elements: vec![
                LoweredExpr::Number(1, make_span()),
                LoweredExpr::Number(2, make_span()),
            ],
            span: make_span(),
        });
    }

    #[test]
    fn bridge_mir_expr_roundtrip_block() {
        assert_mir_expr_roundtrip(LoweredExpr::Block {
            stmts: vec![LoweredStmt::Let(
                LocalId(0),
                LoweredExpr::Number(1, make_span()),
                make_span(),
            )],
            result: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
            span: make_span(),
        });
    }

    #[test]
    fn bridge_mir_expr_roundtrip_all_variants() {
        let variants: Vec<LoweredExpr> = vec![
            LoweredExpr::BigIntLiteral {
                decimal: "42".to_string(),
                sign: 1,
                limb_low: 42,
                limb_high: 0,
                span: make_span(),
            },
            LoweredExpr::EnvCellNew(Box::new(LoweredExpr::Number(1, make_span())), make_span()),
            LoweredExpr::EnvCellGet(LocalId(0), make_span()),
            LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Bool(true, make_span())),
                span: make_span(),
            },
            LoweredExpr::PropertyGet {
                obj: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: "x".to_string(),
                span: make_span(),
            },
            LoweredExpr::ArrayGet {
                arr: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                index: Box::new(LoweredExpr::Number(0, make_span())),
                span: make_span(),
            },
            LoweredExpr::MethodCall {
                object: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                method: "toString".to_string(),
                span: make_span(),
            },
            LoweredExpr::This(make_span()),
            LoweredExpr::ModuleLoad {
                module_id: 1,
                kind: ModuleLoadKind::StaticRequire,
                span: make_span(),
            },
        ];
        for expr in variants {
            assert_mir_expr_roundtrip(expr);
        }
    }

    #[test]
    fn bridge_mir_expr_roundtrip_new() {
        assert_mir_expr_roundtrip(LoweredExpr::New {
            constructor: FuncId(0),
            prototype: ClassPrototypeRef {
                constructor: FuncId(0),
                parent_constructors: vec![],
            },
            args: vec![LoweredExpr::Number(1, make_span())],
            base_local: LocalId(0),
            private_brand: Some(1),
            private_slot_count: 1,
            span: make_span(),
        });
    }

    #[test]
    fn bridge_mir_is_independent_type() {
        // Compile-time check: MirProgram is NOT LoweredProgram
        fn takes_mir(_p: MirProgram) {}
        fn takes_lowered(_p: LoweredProgram) {}

        let mir = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
            escape_status: vec![],
        };
        let lowered = LoweredProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        takes_mir(mir);
        takes_lowered(lowered);
    }
}
