use super::program_builtins::looks_like_regexp_literal;
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::classes::ObjectAccessorKey;
use crate::lowered::ctx::LoweringCtx;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::{BinaryOp, UnaryOp};

pub(super) fn lower_ascii_string_spread_chars(value: &str) -> Result<Vec<LoweredExpr>, Diagnostic> {
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

pub(super) fn update_regexp_literal_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    if matches!(expr, ResolvedExpr::String(raw) if looks_like_regexp_literal(raw)) {
        ctx.facts.regexp_literal_locals.insert(local_id);
    } else {
        ctx.facts.regexp_literal_locals.remove(&local_id);
    }
}

pub(super) fn update_string_literal_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    if let Some(value) = resolved_expr_static_string_value(ctx, expr) {
        ctx.facts.string_literal_locals.insert(local_id, value);
    } else {
        ctx.facts.string_literal_locals.remove(&local_id);
    }
}

pub(super) fn update_number_literal_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    if let Some(value) = resolved_expr_static_number_literal_value(ctx, expr) {
        ctx.facts.number_literal_locals.insert(local_id, value);
    } else {
        ctx.facts.number_literal_locals.remove(&local_id);
    }
}

pub(super) fn update_symbol_value_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    if resolved_expr_is_symbol_value(ctx, expr) {
        ctx.facts.symbol_value_locals.insert(local_id);
        if let Some(description) = resolved_expr_static_symbol_description(ctx, expr) {
            ctx.facts
                .symbol_description_locals
                .insert(local_id, description);
        } else {
            ctx.facts.symbol_description_locals.remove(&local_id);
        }
    } else {
        ctx.facts.symbol_value_locals.remove(&local_id);
        ctx.facts.symbol_description_locals.remove(&local_id);
    }
}

pub(super) fn symbol_local_name(ctx: &LoweringCtx, local_id: LocalId) -> Option<String> {
    let description = ctx.facts.symbol_description_locals.get(&local_id)?;
    Some(
        description
            .as_ref()
            .map(|value| format!("[{value}]"))
            .unwrap_or_default(),
    )
}

pub(super) fn resolved_expr_static_string_value(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<String> {
    match expr {
        ResolvedExpr::String(value) => Some(value.clone()),
        ResolvedExpr::Ident(name) => {
            let local_id = ctx.resolve_local(name).ok()?;
            if ctx.facts.env_cell_locals.contains(&local_id) {
                return None;
            }
            ctx.facts.string_literal_locals.get(&local_id).cloned()
        }
        ResolvedExpr::Binary { left, op, right } if *op == BinaryOp::Add => {
            let mut value = resolved_expr_static_string_value(ctx, left)?;
            value.push_str(&resolved_expr_static_string_value(ctx, right)?);
            Some(value)
        }
        ResolvedExpr::Call { callee, args, .. } => {
            let ResolvedExpr::Ident(name) = callee.as_ref() else {
                return None;
            };
            let func_id = ctx.resolve_func(name).ok()?;
            let signature = ctx.symbols.function_signatures.get(&func_id)?;
            if signature.returns_first_param_identity && args.len() == 1 {
                resolved_expr_static_string_value(ctx, &args[0])
            } else {
                None
            }
        }
        _ => None,
    }
}

pub(super) fn resolved_expr_static_property_key_value(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<String> {
    resolved_expr_static_string_value(ctx, expr)
        .or_else(|| resolved_expr_static_number_literal_value(ctx, expr))
}

pub(super) fn resolved_expr_static_accessor_key(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<ObjectAccessorKey> {
    resolved_expr_static_property_key_value(ctx, expr)
        .map(ObjectAccessorKey::Property)
        .or_else(|| {
            resolved_expr_static_symbol_local(ctx, expr).map(ObjectAccessorKey::SymbolLocal)
        })
}

fn resolved_expr_static_symbol_local(ctx: &LoweringCtx, expr: &ResolvedExpr) -> Option<LocalId> {
    match expr {
        ResolvedExpr::Ident(name) => {
            let local_id = ctx.resolve_local(name).ok()?;
            if ctx.facts.env_cell_locals.contains(&local_id)
                || !ctx.facts.symbol_value_locals.contains(&local_id)
            {
                return None;
            }
            Some(local_id)
        }
        ResolvedExpr::Call { callee, args, .. } => {
            let ResolvedExpr::Ident(name) = callee.as_ref() else {
                return None;
            };
            let func_id = ctx.resolve_func(name).ok()?;
            let signature = ctx.symbols.function_signatures.get(&func_id)?;
            if signature.returns_first_param_identity && args.len() == 1 {
                resolved_expr_static_symbol_local(ctx, &args[0])
            } else {
                None
            }
        }
        _ => None,
    }
}

fn resolved_expr_is_symbol_value(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Ident(name) => ctx
            .resolve_local(name)
            .ok()
            .is_some_and(|local_id| ctx.facts.symbol_value_locals.contains(&local_id)),
        ResolvedExpr::Call { callee, .. } => {
            matches!(callee.as_ref(), ResolvedExpr::Ident(name) if name == "Symbol")
        }
        ResolvedExpr::MethodCall { object, method, .. } => {
            method == "for"
                && matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "Symbol")
        }
        ResolvedExpr::PropertyAccess { object, key, .. } => {
            key.starts_with("__symbol_")
                && matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "Symbol")
        }
        _ => false,
    }
}

fn resolved_expr_static_symbol_description(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<Option<String>> {
    match expr {
        ResolvedExpr::Ident(name) => {
            let local_id = ctx.resolve_local(name).ok()?;
            ctx.facts.symbol_description_locals.get(&local_id).cloned()
        }
        ResolvedExpr::Call { callee, args, .. } => {
            let ResolvedExpr::Ident(name) = callee.as_ref() else {
                return None;
            };
            if name != "Symbol" {
                return None;
            }
            match args.as_slice() {
                [] => Some(None),
                [ResolvedExpr::String(description)] => Some(Some(description.clone())),
                _ => None,
            }
        }
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } if method == "for"
            && matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "Symbol") =>
        {
            match args.as_slice() {
                [ResolvedExpr::String(description)] => Some(Some(description.clone())),
                _ => None,
            }
        }
        _ => None,
    }
}

pub(super) fn resolved_expr_static_number_literal_value(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<String> {
    match expr {
        ResolvedExpr::Number(value) => Some(value.to_string()),
        ResolvedExpr::DecimalNumber(value) => Some(value.clone()),
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Negate => {
            resolved_expr_static_number_literal_value(ctx, expr).map(|value| {
                if value == "0" {
                    value
                } else {
                    format!("-{value}")
                }
            })
        }
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Plus => {
            resolved_expr_static_number_literal_value(ctx, expr)
        }
        ResolvedExpr::Binary { left, op, right } => {
            let left = resolved_expr_static_number_literal_value(ctx, left)
                .and_then(|value| value.parse::<i64>().ok())?;
            let right = resolved_expr_static_number_literal_value(ctx, right)
                .and_then(|value| value.parse::<i64>().ok())?;
            let value = match op {
                BinaryOp::Add => left.checked_add(right)?,
                BinaryOp::Subtract => left.checked_sub(right)?,
                BinaryOp::Multiply => left.checked_mul(right)?,
                BinaryOp::Divide if right != 0 && left % right == 0 => left / right,
                BinaryOp::Power if right >= 0 => left.checked_pow(right as u32)?,
                BinaryOp::BitwiseOr => (left as i32 | right as i32) as i64,
                _ => return None,
            };
            Some(value.to_string())
        }
        ResolvedExpr::Ident(name) => {
            let local_id = ctx.resolve_local(name).ok()?;
            if ctx.facts.env_cell_locals.contains(&local_id) {
                return None;
            }
            ctx.facts.number_literal_locals.get(&local_id).cloned()
        }
        ResolvedExpr::Call { callee, args, .. } => {
            let ResolvedExpr::Ident(name) = callee.as_ref() else {
                return None;
            };
            let func_id = ctx.resolve_func(name).ok()?;
            let signature = ctx.symbols.function_signatures.get(&func_id)?;
            if signature.returns_first_param_identity && args.len() == 1 {
                resolved_expr_static_number_literal_value(ctx, &args[0])
            } else {
                None
            }
        }
        _ => None,
    }
}

pub(super) fn static_string_spread_value(
    ctx: &LoweringCtx,
    spread_expr: &ResolvedExpr,
) -> Option<String> {
    resolved_expr_static_string_value(ctx, spread_expr)
}
