use super::super::{is_private_field_storage_key, private_storage_observable_access_diagnostic};
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::object_kernel;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::UnaryOp;

impl super::super::Resolver {
    pub(super) fn lower_unary_expr(
        &mut self,
        op: &UnaryOp,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if *op == UnaryOp::Negate {
            if let ResolvedExpr::DecimalNumber(value) = expr {
                return Ok(LoweredExpr::DecimalNumber(
                    format!("-{value}"),
                    Span::generated("num"),
                ));
            }
            if let ResolvedExpr::Ident(name) = expr
                && name == "Infinity"
            {
                use ts2wasm_runtime_abi::ValueTag;
                return Ok(LoweredExpr::Number(
                    ValueTag::NEG_INFINITY_PAYLOAD << ValueTag::NUMBER_SHIFT | ValueTag::NUMBER,
                    Span::generated("num"),
                ));
            }
            if crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, expr) {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntUnaryMinus,
                    args: vec![self.lower_expr(expr)?],
                    span: Span::generated("runtime_call"),
                });
            }
        }
        if *op == UnaryOp::BitwiseNot
            && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, expr)
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::BigIntBitwiseNot,
                args: vec![self.lower_expr(expr)?],
                span: Span::generated("runtime_call"),
            });
        }
        if *op == UnaryOp::Delete {
            return self.lower_delete_expr(expr);
        }
        if *op == UnaryOp::TypeOf {
            if let ResolvedExpr::Ident(name) = expr
                && matches!(name.as_str(), "Atomics" | "Intl")
            {
                return Ok(LoweredExpr::String(
                    "object".to_owned(),
                    Span::generated("typeof_builtin_object"),
                ));
            }
            if let ResolvedExpr::Ident(name) = expr
                && self.resolve_local(name).is_err()
                && self.ctx.classes.class_constructor_ids.contains_key(name)
            {
                return Ok(LoweredExpr::String(
                    "function".to_owned(),
                    Span::generated("typeof_class_constructor"),
                ));
            }
            let lowered = match self.lower_expr(expr) {
                Ok(expr) => expr,
                Err(err) if err.code == DiagCode::UnresolvedName => {
                    return Ok(LoweredExpr::String(
                        "undefined".to_owned(),
                        Span::generated("typeof_undeclared"),
                    ));
                }
                Err(err) => return Err(err),
            };
            return Ok(LoweredExpr::Unary {
                op: lower_unary_op(*op)?,
                expr: Box::new(lowered),
                span: Span::generated("unary"),
            });
        }
        Ok(LoweredExpr::Unary {
            op: lower_unary_op(*op)?,
            expr: Box::new(self.lower_expr(expr)?),
            span: Span::generated("unary"),
        })
    }

    fn lower_delete_expr(&mut self, expr: &ResolvedExpr) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            ResolvedExpr::Ident(name)
                if self
                    .ctx
                    .strict_mode_check(crate::lowered::ctx::StrictModeCheck::StrictDelete) =>
            {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-450: {:?} strict mode forbids deleting identifier `{name}`",
                        crate::lowered::ctx::StrictModeCheck::StrictDelete
                    ),
                    span: None,
                    phase: None,
                })
            }
            ResolvedExpr::PropertyAccess { object, key, span } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(Some(*span)));
                }
                if key.starts_with('#') {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: private member `{key}` cannot be deleted in this private class runtime slice"
                        ),
                        span: Some(*span),
                        phase: None,
                    });
                }
                if let Some(proxy) =
                    crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(
                        &self.ctx, object,
                    )
                {
                    return self.lower_proxy_trap_call(
                        proxy,
                        crate::lowered::facts::ProxyTrapKind::ProxyDeleteProperty,
                        vec![ResolvedExpr::String(key.to_owned())],
                        *span,
                    );
                }
                Ok(object_kernel::ordinary_delete(
                    self.lower_expr(object)?,
                    key,
                    *span,
                ))
            }
            ResolvedExpr::ComputedIndex { object, index } => {
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                if let ResolvedExpr::String(key) = index.as_ref()
                    && is_private_field_storage_key(key)
                {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                if let Some(proxy) =
                    crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(
                        &self.ctx, object,
                    )
                {
                    return self.lower_proxy_trap_call(
                        proxy,
                        crate::lowered::facts::ProxyTrapKind::ProxyDeleteProperty,
                        vec![index.as_ref().clone()],
                        Span::generated("proxy_delete"),
                    );
                }
                Ok(object_kernel::ordinary_delete_dynamic(
                    self.lower_expr(object)?,
                    self.lower_expr(index)?,
                    Span::generated("prop_delete_dyn"),
                ))
            }
            _ => Ok(LoweredExpr::Unary {
                op: lower_unary_op(UnaryOp::Delete)?,
                expr: Box::new(self.lower_expr(expr)?),
                span: Span::generated("unary"),
            }),
        }
    }
}
