use super::program_builtins::looks_like_regexp_literal;
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_syntax::BinaryOp;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::Resolver {
    pub(super) fn static_string_spread_value(&self, spread_expr: &ResolvedExpr) -> Option<String> {
        self.resolved_expr_static_string_value(spread_expr)
    }

    pub(super) fn lower_ascii_string_spread_chars(
        value: &str,
    ) -> Result<Vec<LoweredExpr>, Diagnostic> {
        if !value.is_ascii() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-274: string spread is currently limited to ASCII literal-derived strings"
                        .to_owned(),
                span: None,

                phase: None,
            });
        }
        Ok(value
            .chars()
            .map(|ch| LoweredExpr::String(ch.to_string(), Span::generated("str")))
            .collect())
    }

    pub(super) fn update_regexp_literal_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if matches!(expr, ResolvedExpr::String(raw) if looks_like_regexp_literal(raw)) {
            self.ctx.facts.regexp_literal_locals.insert(local_id);
        } else {
            self.ctx.facts.regexp_literal_locals.remove(&local_id);
        }
    }

    pub(super) fn update_string_literal_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if let Some(value) = self.resolved_expr_static_string_value(expr) {
            self.ctx.facts.string_literal_locals.insert(local_id, value);
        } else {
            self.ctx.facts.string_literal_locals.remove(&local_id);
        }
    }

    pub(super) fn resolved_expr_static_string_value(&self, expr: &ResolvedExpr) -> Option<String> {
        match expr {
            ResolvedExpr::String(value) => Some(value.clone()),
            ResolvedExpr::Ident(name) => {
                let local_id = self.resolve_local(name).ok()?;
                if self.ctx.facts.env_cell_locals.contains(&local_id) {
                    return None;
                }
                self.ctx.facts.string_literal_locals.get(&local_id).cloned()
            }
            ResolvedExpr::Binary { left, op, right } if *op == BinaryOp::Add => {
                let mut value = self.resolved_expr_static_string_value(left)?;
                value.push_str(&self.resolved_expr_static_string_value(right)?);
                Some(value)
            }
            _ => None,
        }
    }
}
