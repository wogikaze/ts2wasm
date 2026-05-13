use ts2wasm_diagnostic::Diagnostic;
use ts2wasm_syntax::ObjectProp;

use crate::builtin_resolved::{ResolvedExpr, ResolvedObjectProp};

use super::super::resolve_expr;

/// Resolve an object literal expression.
pub fn resolve_object_literal(props: &[ObjectProp]) -> Result<ResolvedExpr, Diagnostic> {
    Ok(ResolvedExpr::Object(
        props
            .iter()
            .map(|prop| match prop {
                ObjectProp::KeyValue { key, value } => Ok(ResolvedObjectProp::KeyValue {
                    key: key.clone(),
                    value: resolve_expr(value)?,
                }),
                ObjectProp::Shorthand { key, value } => Ok(ResolvedObjectProp::Shorthand {
                    key: key.clone(),
                    value: resolve_expr(value)?,
                }),
                ObjectProp::ComputedKey { key, value } => Ok(ResolvedObjectProp::ComputedKey {
                    key: Box::new(resolve_expr(key)?),
                    value: resolve_expr(value)?,
                }),
                ObjectProp::MethodShorthand { key, value } => {
                    Ok(ResolvedObjectProp::MethodShorthand {
                        key: key.clone(),
                        value: resolve_expr(value)?,
                    })
                }
            })
            .collect::<Result<Vec<_>, _>>()?,
    ))
}
