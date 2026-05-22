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
    MirModuleInfo, MirProgram, MirStmt, MirUnaryOp, RepProof, ValueRep,
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
