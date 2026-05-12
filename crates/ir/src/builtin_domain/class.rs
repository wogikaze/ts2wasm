use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_syntax::Stmt;

use crate::builtin_resolved::{ClassMethodKind, ResolvedExpr};

use super::super::resolve_stmt;

/// Determine the method kind (getter, setter, or regular method) from the method name.
pub fn class_method_kind(method_name: &str) -> ClassMethodKind {
    let name = method_name.strip_prefix("static::").unwrap_or(method_name);
    if name.starts_with("get ") {
        ClassMethodKind::Getter
    } else if name.starts_with("set ") {
        ClassMethodKind::Setter
    } else {
        ClassMethodKind::Method
    }
}

/// Resolve a class expression.
pub fn resolve_class_expr(name: &str, body: &[Stmt]) -> Result<ResolvedExpr, Diagnostic> {
    Ok(ResolvedExpr::ClassExpr {
        name: name.to_owned(),
        body: body
            .iter()
            .map(resolve_stmt)
            .collect::<Result<Vec<_>, _>>()?,
    })
}
