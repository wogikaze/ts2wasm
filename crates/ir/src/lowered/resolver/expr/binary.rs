use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use crate::lowered::BuiltinErrorConstructor;
use ts2wasm_shared::{BinaryOp, DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_binary_expr(
        &mut self,
        left: &ResolvedExpr,
        op: &BinaryOp,
        right: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if *op == BinaryOp::InstanceOf {
            return self.lower_instanceof_expr(left, right);
        }
        if *op == BinaryOp::In {
            return self.lower_in_expr(left, right);
        }
        if let Some(result) = self.lower_bigint_binary_expr(left, op, right)? {
            return Ok(result);
        }
        Ok(LoweredExpr::Binary {
            left: Box::new(self.lower_expr(left)?),
            op: lower_binary_op(*op)?,
            right: Box::new(self.lower_expr(right)?),
            span: Span::generated("binary"),
        })
    }

    fn lower_instanceof_expr(
        &mut self,
        left: &ResolvedExpr,
        right: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        let prototype = match right {
            ResolvedExpr::Ident(name) => {
                if let Some(constructor) = BuiltinErrorConstructor::from_name(name) {
                    LoweredExpr::BuiltinErrorPrototype(constructor, Span::generated("builtin_error_proto"))
                } else {
                    self.class_prototype_ref(name)
                        .map(|p| LoweredExpr::ClassPrototype(p, Span::generated("class_proto")))?
                }
            }
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-207: instanceof right-hand side must be a supported class constructor"
                            .to_owned(),
                    span: None,
                    phase: None,
                });
            }
        };
        Ok(LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::InstanceOf,
            args: vec![self.lower_expr(left)?, prototype],
            span: Span::generated("runtime_call"),
        })
    }

    fn lower_in_expr(
        &mut self,
        left: &ResolvedExpr,
        right: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        match left {
            ResolvedExpr::Number(index) => Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayIndexPresent,
                args: vec![
                    self.lower_expr(right)?,
                    LoweredExpr::Number(*index, Span::generated("num")),
                ],
                span: Span::generated("runtime_call"),
            }),
            ResolvedExpr::String(key) => Ok(LoweredExpr::PropertyIn {
                obj: Box::new(self.lower_expr(right)?),
                key: key.clone(),
                span: Span::generated("prop_in"),
            }),
            _ => Ok(LoweredExpr::PropertyInDynamic {
                obj: Box::new(self.lower_expr(right)?),
                key: Box::new(self.lower_expr(left)?),
                span: Span::generated("prop_in_dyn"),
            }),
        }
    }

    fn lower_bigint_binary_expr(
        &mut self,
        left: &ResolvedExpr,
        op: &BinaryOp,
        right: &ResolvedExpr,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if matches!(op, BinaryOp::Divide | BinaryOp::Modulo)
            && self.resolved_expr_is_bigint_div_rem_operand(left)
            && self.resolved_expr_is_bigint_div_rem_operand(right)
        {
            let intrinsic = match op {
                BinaryOp::Divide => RuntimeFn::BigIntDiv,
                BinaryOp::Modulo => RuntimeFn::BigIntRem,
                _ => unreachable!("checked above"),
            };
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                span: Span::generated("runtime_call"),
            }));
        }
        if matches!(op, BinaryOp::Divide | BinaryOp::Modulo)
            && ((self.resolved_expr_is_control_flow_mixed_bigint(left)
                && self.resolved_expr_is_bigint_div_rem_operand(right))
                || (self.resolved_expr_is_bigint_div_rem_operand(left)
                    && self.resolved_expr_is_control_flow_mixed_bigint(right)))
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-370: mixed Number/BigInt arithmetic TypeError parity is not implemented in the control-flow BigInt div/rem slice"
                        .to_owned(),
                span: None,
                phase: None,
            });
        }
        if matches!(
            op,
            BinaryOp::Add
                | BinaryOp::Subtract
                | BinaryOp::Multiply
                | BinaryOp::Divide
                | BinaryOp::Modulo
        ) && self.resolved_expr_is_bigint(left)
            && self.resolved_expr_is_bigint(right)
        {
            let intrinsic = match op {
                BinaryOp::Add => RuntimeFn::BigIntAdd,
                BinaryOp::Subtract => RuntimeFn::BigIntSub,
                BinaryOp::Multiply => RuntimeFn::BigIntMul,
                BinaryOp::Divide => RuntimeFn::BigIntDiv,
                BinaryOp::Modulo => RuntimeFn::BigIntRem,
                _ => unreachable!("checked above"),
            };
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                span: Span::generated("runtime_call"),
            }));
        }
        if *op == BinaryOp::Power
            && self.resolved_expr_is_bigint(left)
            && self.resolved_expr_is_bigint(right)
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::BigIntPow,
                args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                span: Span::generated("runtime_call"),
            }));
        }
        if matches!(
            op,
            BinaryOp::Add
                | BinaryOp::Subtract
                | BinaryOp::Multiply
                | BinaryOp::Divide
                | BinaryOp::Modulo
                | BinaryOp::Power
        ) && (self.resolved_expr_is_bigint(left) || self.resolved_expr_is_bigint(right))
        {
            if *op == BinaryOp::Add {
                return Ok(Some(LoweredExpr::Binary {
                    left: Box::new(self.lower_expr(left)?),
                    op: LoweredBinaryOp::Add,
                    right: Box::new(self.lower_expr(right)?),
                    span: Span::generated("binary"),
                }));
            } else {
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntMixedArithmeticTypeError,
                    args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                    span: Span::generated("runtime_call"),
                }));
            }
        }
        if matches!(
            op,
            BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor
        ) && self.resolved_expr_is_bigint(left)
            && self.resolved_expr_is_bigint(right)
        {
            let intrinsic = match op {
                BinaryOp::BitwiseAnd => RuntimeFn::BigIntBitwiseAnd,
                BinaryOp::BitwiseOr => RuntimeFn::BigIntBitwiseOr,
                BinaryOp::BitwiseXor => RuntimeFn::BigIntBitwiseXor,
                _ => unreachable!("checked above"),
            };
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                span: Span::generated("runtime_call"),
            }));
        }
        if matches!(
            op,
            BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor
        ) && (self.resolved_expr_is_bigint(left) || self.resolved_expr_is_bigint(right))
            && !(self.resolved_expr_is_bigint(left) && self.resolved_expr_is_bigint(right))
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::BigIntMixedArithmeticTypeError,
                args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                span: Span::generated("runtime_call"),
            }));
        }
        if matches!(
            op,
            BinaryOp::LeftShift | BinaryOp::RightShift | BinaryOp::UnsignedRightShift
        ) && self.resolved_expr_is_bigint(left)
        {
            if *op == BinaryOp::UnsignedRightShift {
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntMixedArithmeticTypeError,
                    args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                    span: Span::generated("runtime_call"),
                }));
            } else if self.resolved_expr_is_bigint(right) {
                let intrinsic = match op {
                    BinaryOp::LeftShift => RuntimeFn::BigIntLeftShift,
                    BinaryOp::RightShift => RuntimeFn::BigIntRightShift,
                    _ => unreachable!("checked above"),
                };
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                    span: Span::generated("runtime_call"),
                }));
            } else {
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntMixedArithmeticTypeError,
                    args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                    span: Span::generated("runtime_call"),
                }));
            }
        }
        Ok(None)
    }
}
