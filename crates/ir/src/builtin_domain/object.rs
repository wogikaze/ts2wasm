use ts2wasm_diagnostic::Diagnostic;
use ts2wasm_syntax::Expr;

use crate::builtin_resolved::ResolvedExpr;

use super::super::resolve_expr;

/// Resolve an object literal expression.
pub fn resolve_object_literal(props: &[(String, Expr)]) -> Result<ResolvedExpr, Diagnostic> {
    Ok(ResolvedExpr::Object(
        props
            .iter()
            .map(|(k, v)| Ok((k.clone(), resolve_expr(v)?)))
            .collect::<Result<Vec<_>, _>>()?,
    ))
}
