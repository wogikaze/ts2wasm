use super::super::{
    is_private_field_storage_key, private_storage_observable_access_diagnostic,
};
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_syntax::UnaryOp;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_unary_expr(
        &mut self,
        op: &UnaryOp,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if *op == UnaryOp::Negate {
            if let ResolvedExpr::Ident(name) = expr
                && name == "Infinity"
            {
                use ts2wasm_runtime_abi::ValueTag;
                return Ok(LoweredExpr::Number(
                    ValueTag::NUMBER_PAYLOAD_MIN,
                    Span::generated("num"),
                ));
            }
            if self.resolved_expr_is_bigint(expr) {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntUnaryMinus,
                    args: vec![self.lower_expr(expr)?],
                    span: Span::generated("runtime_call"),
                });
            }
        }
        if *op == UnaryOp::BitwiseNot && self.resolved_expr_is_bigint(expr) {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::BigIntBitwiseNot,
                args: vec![self.lower_expr(expr)?],
                span: Span::generated("runtime_call"),
            });
        }
        if *op == UnaryOp::Delete {
            return self.lower_delete_expr(expr);
        }
        Ok(LoweredExpr::Unary {
            op: lower_unary_op(*op)?,
            expr: Box::new(self.lower_expr(expr)?),
            span: Span::generated("unary"),
        })
    }

    fn lower_delete_expr(
        &mut self,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            ResolvedExpr::PropertyAccess {
                object,
                key,
                span,
            } => {
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
                Ok(LoweredExpr::PropertyDelete {
                    object: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                    span: Span::generated("prop_delete"),
                })
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
                Ok(LoweredExpr::PropertyDeleteDynamic {
                    object: Box::new(self.lower_expr(object)?),
                    key: Box::new(self.lower_expr(index)?),
                    span: Span::generated("prop_delete_dyn"),
                })
            }
            _ => Ok(LoweredExpr::Unary {
                op: lower_unary_op(UnaryOp::Delete)?,
                expr: Box::new(self.lower_expr(expr)?),
                span: Span::generated("unary"),
            }),
        }
    }
}
