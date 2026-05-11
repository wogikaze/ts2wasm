use ts2wasm_shared::{ArrayLiteralElement, Expr};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr};

use super::super::resolve_expr;

/// Resolve `Array()` called as a function (without `new`) — behaves like `new Array()`.
pub fn try_resolve_array_call(
    callee: &Expr,
    resolved_args: &[ResolvedExpr],
    span: Span,
) -> Option<ResolvedExpr> {
    let Expr::Ident { name, .. } = callee else {
        return None;
    };
    if name != "Array" {
        return None;
    }
    Some(ResolvedExpr::New {
        class_name: "Array".to_owned(),
        args: resolved_args.to_vec(),
        span,
    })
}

/// Resolve an array literal expression.
pub fn resolve_array_literal(elements: &[ArrayLiteralElement]) -> Result<ResolvedExpr, Diagnostic> {
    Ok(ResolvedExpr::Array(
        elements
            .iter()
            .map(|element| match element {
                ArrayLiteralElement::Present(expr) => {
                    Ok(ResolvedArrayElement::Present(resolve_expr(expr)?))
                }
                ArrayLiteralElement::Spread(expr) => {
                    Ok(ResolvedArrayElement::Present(resolve_expr(expr)?))
                }
                ArrayLiteralElement::Hole(_) => Ok(ResolvedArrayElement::Hole),
            })
            .collect::<Result<Vec<_>, _>>()?,
    ))
}
