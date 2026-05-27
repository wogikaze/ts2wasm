use crate::builtin_resolved::{ResolvedExpr, ResolvedObjectProp, ResolvedStmt};
use crate::lowered::BuiltinErrorConstructor;
use crate::lowered::object_kernel;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::BinaryOp;

/// Helper: convert a ResolvedExpr numeric literal to f64, if possible.
fn resolved_expr_to_f64(expr: &ResolvedExpr) -> Option<f64> {
    match expr {
        ResolvedExpr::Number(n) => Some(*n as f64),
        ResolvedExpr::DecimalNumber(s) => s.parse::<f64>().ok(),
        _ => None,
    }
}

/// Returns true if the operator is a relational comparison (<, >, <=, >=)
/// that requires ToNumber coercion per ECMAScript Abstract Relational Comparison.
fn is_relational_op(op: &BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual
    )
}

/// Wrap an expression in unary `+` (ToNumber) to ensure proper numeric
/// coercion before comparison. The WAT runtime functions do not perform
/// string-to-number or boolean-to-number conversion internally.
fn wrap_to_number(expr: LoweredExpr, span: Span) -> LoweredExpr {
    LoweredExpr::Unary {
        op: LoweredUnaryOp::Plus,
        expr: Box::new(expr),
        span,
    }
}

/// Format an f64 as a JavaScript number string (e.g. for DecimalNumber).
/// Handles NaN, Infinity, -Infinity with correct JS casing.
fn f64_to_js_string(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value == f64::INFINITY {
        return "Infinity".to_string();
    }
    if value == f64::NEG_INFINITY {
        return "-Infinity".to_string();
    }
    value.to_string()
}

/// Constant-fold `===` / `!==` when both operands are same-type literals.
///
/// Eliminates the runtime StrictEqual/StrictNotEqual call for patterns like
/// `x === null`, `typeof x === "string"`, `1 === 2`, etc.
fn try_constant_fold_strict_compare(
    left: &ResolvedExpr,
    op: BinaryOp,
    right: &ResolvedExpr,
) -> Option<LoweredExpr> {
    if !matches!(op, BinaryOp::StrictEqual | BinaryOp::StrictNotEqual) {
        return None;
    }
    let equal = match (left, right) {
        (ResolvedExpr::Number(l), ResolvedExpr::Number(r)) => l == r,
        (ResolvedExpr::DecimalNumber(l), ResolvedExpr::DecimalNumber(r)) => {
            // f64::NAN == f64::NAN is false, matching JS NaN !== NaN semantics.
            l.parse::<f64>().ok()? == r.parse::<f64>().ok()?
        }
        (ResolvedExpr::String(l), ResolvedExpr::String(r)) => l == r,
        (ResolvedExpr::Bool(l), ResolvedExpr::Bool(r)) => l == r,
        (ResolvedExpr::Null, ResolvedExpr::Null) => true,
        (ResolvedExpr::Undefined, ResolvedExpr::Undefined) => true,
        (ResolvedExpr::Null, ResolvedExpr::Undefined)
        | (ResolvedExpr::Undefined, ResolvedExpr::Null) => false,
        _ => return None,
    };
    let value = if matches!(op, BinaryOp::StrictNotEqual) {
        !equal
    } else {
        equal
    };
    Some(LoweredExpr::Bool(value, Span::generated("const_fold")))
}

/// Constant-fold `>` / `<` / `>=` / `<=` when both operands are numeric literals.
///
/// Avoids the runtime ToNumber coercion and comparison call for
/// compile-time-known numeric values.
fn try_constant_fold_numeric_compare(
    left: &ResolvedExpr,
    op: &BinaryOp,
    right: &ResolvedExpr,
) -> Option<LoweredExpr> {
    if !matches!(
        op,
        BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual
    ) {
        return None;
    }
    let l = resolved_expr_to_f64(left)?;
    let r = resolved_expr_to_f64(right)?;
    let result = match op {
        BinaryOp::Less => l < r,
        BinaryOp::LessEqual => l <= r,
        BinaryOp::Greater => l > r,
        BinaryOp::GreaterEqual => l >= r,
        _ => unreachable!(),
    };
    Some(LoweredExpr::Bool(result, Span::generated("const_fold")))
}

/// Constant-fold `-` / `*` / `/` / `%` / `**` when both operands are numeric literals.
///
/// Avoids the runtime arithmetic call (even the backend Fast paths) for
/// compile-time-known numeric values.
fn try_constant_fold_numeric_arithmetic(
    left: &ResolvedExpr,
    op: &BinaryOp,
    right: &ResolvedExpr,
) -> Option<LoweredExpr> {
    if !matches!(
        op,
        BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Modulo
            | BinaryOp::Power
    ) {
        return None;
    }
    let l = resolved_expr_to_f64(left)?;
    let r = resolved_expr_to_f64(right)?;
    let result: f64 = match op {
        BinaryOp::Subtract => l - r,
        BinaryOp::Multiply => l * r,
        BinaryOp::Divide => l / r,
        BinaryOp::Modulo => l % r,
        BinaryOp::Power => l.powf(r),
        _ => unreachable!(),
    };
    // Produce Number(i32) when the result is an integer in i32 range,
    // otherwise DecimalNumber with correct JS number string.
    if result.fract() == 0.0
        && result.is_finite()
        && result >= i32::MIN as f64
        && result <= i32::MAX as f64
    {
        Some(LoweredExpr::Number(
            result as i32,
            Span::generated("const_fold"),
        ))
    } else {
        Some(LoweredExpr::DecimalNumber(
            f64_to_js_string(result),
            Span::generated("const_fold"),
        ))
    }
}

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
        if let Some(result) = self.lower_bigint_toprimitive_type_error(left, op, right) {
            return Ok(result);
        }
        if let Some(result) = self.lower_bigint_binary_expr(left, op, right)? {
            return Ok(result);
        }
        // --- String concatenation optimizations ---

        // Both operands are string literals: fold at compile time.
        if *op == BinaryOp::Add
            && matches!(left, ResolvedExpr::String(_))
            && matches!(right, ResolvedExpr::String(_))
        {
            if let (ResolvedExpr::String(l), ResolvedExpr::String(r)) = (left, right) {
                let mut result = l.clone();
                result.push_str(r);
                return Ok(LoweredExpr::String(result, Span::generated("const_fold")));
            }
        }

        // Per ECMAScript spec (13.15.3, 7.2.21):
        // The `+` operator: If either operand is a String, the result is string concatenation.
        // For literal string operands, emit $concat directly (which handles ToString conversion
        // of non-string operands internally via $value_to_string_into), matching the spec
        // evaluation order where concatenation takes priority over numeric addition.
        if *op == BinaryOp::Add
            && (matches!(left, ResolvedExpr::String(_)) || matches!(right, ResolvedExpr::String(_)))
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::Concat,
                args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                span: Span::generated("runtime_call"),
            });
        }

        // --- Constant folding for literal operands ---

        if let Some(result) = try_constant_fold_strict_compare(left, *op, right) {
            return Ok(result);
        }
        if let Some(result) = try_constant_fold_numeric_compare(left, op, right) {
            return Ok(result);
        }
        if let Some(result) = try_constant_fold_numeric_arithmetic(left, op, right) {
            return Ok(result);
        }

        // Generic binary expression fallthrough.
        let lowered_left = self.lower_expr(left)?;
        let lowered_right = self.lower_expr(right)?;
        // Relational comparisons (<, >, <=, >=) require ToNumber coercion
        // per ECMAScript Abstract Relational Comparison algorithm.
        // The WAT runtime functions ($less, $greater, etc.) do not perform
        // string-to-number or boolean-to-number conversion, so we wrap
        // non-BigInt operands with unary `+` (ToNumber) here in the IR.
        let (coerced_left, coerced_right) = if is_relational_op(op)
            && !crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, left)
            && !crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, right)
        {
            let span = Span::generated("unary_plus_to_number");
            (
                wrap_to_number(lowered_left, span),
                wrap_to_number(lowered_right, span),
            )
        } else {
            (lowered_left, lowered_right)
        };
        Ok(LoweredExpr::Binary {
            left: Box::new(coerced_left),
            op: lower_binary_op(*op)?,
            right: Box::new(coerced_right),
            span: Span::generated("binary"),
        })
    }

    fn lower_instanceof_expr(
        &mut self,
        left: &ResolvedExpr,
        right: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        // Try static path: RHS is a known class constructor.
        if let ResolvedExpr::Ident(name) = right {
            if let Some(constructor) = BuiltinErrorConstructor::from_name(name) {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::InstanceOf,
                    args: vec![
                        self.lower_expr(left)?,
                        LoweredExpr::BuiltinErrorPrototype(
                            constructor,
                            Span::generated("builtin_error_proto"),
                        ),
                    ],
                    span: Span::generated("runtime_call"),
                });
            }
            if let Ok(prototype) = self.class_prototype_ref(name) {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::InstanceOf,
                    args: vec![
                        self.lower_expr(left)?,
                        LoweredExpr::ClassPrototype(prototype, Span::generated("class_proto")),
                    ],
                    span: Span::generated("runtime_call"),
                });
            }
            if let Some(prototype) = self.constructable_function_prototype_ref(name) {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::InstanceOf,
                    args: vec![
                        self.lower_expr(left)?,
                        LoweredExpr::ClassPrototype(prototype, Span::generated("function_proto")),
                    ],
                    span: Span::generated("runtime_call"),
                });
            }
        }
        // Dynamic path: RHS is not statically known.
        // Evaluate RHS at runtime and use SymbolHasInstance which checks
        // constructor[Symbol.hasInstance] or falls back to OrdinaryHasInstance.
        Ok(LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::SymbolHasInstance,
            args: vec![self.lower_expr(right)?, self.lower_expr(left)?],
            span: Span::generated("runtime_call"),
        })
    }

    fn lower_in_expr(
        &mut self,
        left: &ResolvedExpr,
        right: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let Some(proxy) =
            crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(&self.ctx, right)
        {
            return self.lower_proxy_trap_call(
                proxy,
                crate::lowered::facts::ProxyTrapKind::ProxyHas,
                vec![left.clone()],
                Span::generated("proxy_has"),
            );
        }
        match left {
            ResolvedExpr::Number(index) => Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayIndexPresent,
                args: vec![
                    self.lower_expr(right)?,
                    LoweredExpr::Number(*index, Span::generated("num")),
                ],
                span: Span::generated("runtime_call"),
            }),
            ResolvedExpr::String(key) => Ok(object_kernel::ordinary_has_property(
                self.lower_expr(right)?,
                key,
                Span::generated("prop_in"),
            )),
            _ => Ok(object_kernel::ordinary_has_property_dynamic(
                self.lower_expr(right)?,
                self.lower_expr(left)?,
                Span::generated("prop_in_dyn"),
            )),
        }
    }

    fn lower_bigint_binary_expr(
        &mut self,
        left: &ResolvedExpr,
        op: &BinaryOp,
        right: &ResolvedExpr,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if matches!(op, BinaryOp::Divide | BinaryOp::Modulo)
            && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint_div_rem_operand(
                &self.ctx, left,
            )
            && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint_div_rem_operand(
                &self.ctx, right,
            )
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
            && ((crate::lowered::resolver::expr::facts::resolved_expr_is_control_flow_mixed_bigint(&self.ctx, left)
                && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint_div_rem_operand(&self.ctx, right))
                || (crate::lowered::resolver::expr::facts::resolved_expr_is_bigint_div_rem_operand(&self.ctx, left)
                    && crate::lowered::resolver::expr::facts::resolved_expr_is_control_flow_mixed_bigint(&self.ctx, right)))
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-370: mixed Number/BigInt arithmetic TypeError parity is not implemented in the control-flow BigInt div/rem slice"
                        .to_owned(),
                span: Some(Span::generated("issue-370")),
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
        ) && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, left)
            && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, right)
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
            && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, left)
            && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, right)
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
        ) && (crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, left)
            || crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, right))
        {
            if *op == BinaryOp::Add {
                if matches!(left, ResolvedExpr::String(_))
                    || matches!(right, ResolvedExpr::String(_))
                {
                    return Ok(None);
                }
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntMixedArithmeticTypeError,
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
        if matches!(
            op,
            BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor
        ) && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, left)
            && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, right)
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
        ) && (crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, left)
            || crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, right))
            && !(crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, left)
                && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, right))
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
        ) && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, left)
        {
            if *op == BinaryOp::UnsignedRightShift {
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntMixedArithmeticTypeError,
                    args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                    span: Span::generated("runtime_call"),
                }));
            } else if crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(
                &self.ctx, right,
            ) {
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

    fn lower_bigint_toprimitive_type_error(
        &self,
        left: &ResolvedExpr,
        op: &BinaryOp,
        right: &ResolvedExpr,
    ) -> Option<LoweredExpr> {
        if !matches!(
            op,
            BinaryOp::EqualEqual
                | BinaryOp::BangEqual
                | BinaryOp::Less
                | BinaryOp::LessEqual
                | BinaryOp::Greater
                | BinaryOp::GreaterEqual
        ) {
            return None;
        }
        let left_bigint =
            crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, left);
        let right_bigint =
            crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, right);
        if (left_bigint && resolved_expr_toprimitive_type_error(&self.ctx, right))
            || (right_bigint && resolved_expr_toprimitive_type_error(&self.ctx, left))
        {
            let span = Span::generated("bigint_toprimitive_type_error");
            return Some(LoweredExpr::Block {
                stmts: vec![LoweredStmt::Throw(
                    LoweredExpr::ErrorNew {
                        constructor: BuiltinErrorConstructor::TypeError,
                        message: Box::new(LoweredExpr::String(
                            "Cannot convert object to primitive value".to_owned(),
                            span,
                        )),
                        cause: None,
                        errors: None,
                        span,
                    },
                    span,
                )],
                result: Box::new(LoweredExpr::Undefined(span)),
                span,
            });
        }
        None
    }
}

fn resolved_expr_toprimitive_type_error(
    ctx: &crate::lowered::ctx::LoweringCtx,
    expr: &ResolvedExpr,
) -> bool {
    let Some(props) =
        crate::lowered::resolver::expr::facts::object_toprimitive_literal_props_for_expr(ctx, expr)
    else {
        return false;
    };
    object_toprimitive_type_error_props(&props)
}

fn object_toprimitive_type_error_props(props: &[ResolvedObjectProp]) -> bool {
    if let Some(prop) = props
        .iter()
        .find(|prop| prop.static_key() == Some("valueOf"))
    {
        match object_toprimitive_return_kind(prop.value()) {
            Some(ObjectToPrimitiveReturnKind::Primitive) => return false,
            Some(ObjectToPrimitiveReturnKind::Object) => {}
            None => return false,
        }
    }
    let Some(prop) = props
        .iter()
        .find(|prop| prop.static_key() == Some("toString"))
    else {
        return false;
    };
    matches!(
        object_toprimitive_return_kind(prop.value()),
        Some(ObjectToPrimitiveReturnKind::Object)
    )
}

enum ObjectToPrimitiveReturnKind {
    Primitive,
    Object,
}

fn object_toprimitive_return_kind(expr: &ResolvedExpr) -> Option<ObjectToPrimitiveReturnKind> {
    match expr {
        ResolvedExpr::ArrowFn { params, body, .. } if params.is_empty() => {
            object_toprimitive_expr_kind(body)
        }
        ResolvedExpr::FunctionExpr { params, body, .. } if params.is_empty() => {
            let [ResolvedStmt::Return(expr)] = body.as_slice() else {
                return None;
            };
            object_toprimitive_expr_kind(expr)
        }
        _ => None,
    }
}

fn object_toprimitive_expr_kind(expr: &ResolvedExpr) -> Option<ObjectToPrimitiveReturnKind> {
    match expr {
        ResolvedExpr::Object(_) => Some(ObjectToPrimitiveReturnKind::Object),
        ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::String(_) => Some(ObjectToPrimitiveReturnKind::Primitive),
        ResolvedExpr::Unary { op, expr } if *op == ts2wasm_syntax::UnaryOp::Negate => matches!(
            expr.as_ref(),
            ResolvedExpr::Number(_)
                | ResolvedExpr::DecimalNumber(_)
                | ResolvedExpr::BigIntLiteral { .. }
        )
        .then_some(ObjectToPrimitiveReturnKind::Primitive),
        _ => None,
    }
}
