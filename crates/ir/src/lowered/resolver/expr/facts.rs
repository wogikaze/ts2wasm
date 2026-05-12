use super::super::program_builtins::{is_typed_array_class, looks_like_regexp_literal};
use super::super::{
    is_invalid_date_constructor_expr, is_set_prototype_property_expr,
    is_static_copy_safe_object_prop_value, string_constructor_arrow_callback,
    unary_plus_arrow_callback,
};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr};
use crate::lowered::ctx::LoweringCtx;
use crate::lowered::facts::StaticFunctionArrayLike;
use crate::lowered::*;
use std::collections::HashSet;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_syntax::{BinaryOp, OBJECT_SPREAD_SENTINEL, SYMBOL_ITERATOR_OBJECT_KEY, UnaryOp};

pub(crate) fn update_bigint_local(ctx: &mut LoweringCtx, local_id: LocalId, expr: &ResolvedExpr) {
    if resolved_expr_is_bigint(ctx, expr) {
        ctx.facts.bigint_locals.insert(local_id);
    } else {
        ctx.facts.bigint_locals.remove(&local_id);
    }
}

pub(crate) fn update_control_flow_bigint_assignment(ctx: &mut LoweringCtx, local_id: LocalId) {
    ctx.facts
        .control_flow_bigint_div_rem_locals
        .remove(&local_id);
    ctx.facts.control_flow_mixed_bigint_locals.remove(&local_id);
}

pub(crate) fn update_nullish_local(ctx: &mut LoweringCtx, local_id: LocalId, expr: &ResolvedExpr) {
    if resolved_expr_is_nullish(ctx, expr) {
        ctx.facts.nullish_locals.insert(local_id);
    } else {
        ctx.facts.nullish_locals.remove(&local_id);
    }
}

pub(crate) fn resolved_expr_is_nullish(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Null | ResolvedExpr::Undefined => true,
        ResolvedExpr::Ident(name) => ctx
            .resolve_local(name)
            .ok()
            .is_some_and(|local_id| ctx.facts.nullish_locals.contains(&local_id)),
        _ => false,
    }
}

pub(crate) fn update_array_local(ctx: &mut LoweringCtx, local_id: LocalId, expr: &ResolvedExpr) {
    if let Some(slots) = resolved_expr_static_array_slots(ctx, expr) {
        ctx.facts.array_locals.insert(local_id);
        ctx.facts.static_array_slots.insert(local_id, slots);
    } else if resolved_expr_produces_dense_array(ctx, expr) {
        ctx.facts.array_locals.insert(local_id);
        ctx.facts.static_array_slots.remove(&local_id);
    } else {
        ctx.facts.array_locals.remove(&local_id);
        ctx.facts.static_array_slots.remove(&local_id);
    }
}

pub(crate) fn update_symbol_iterator_object_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    if resolved_expr_has_symbol_iterator_property(ctx, expr) {
        ctx.facts.symbol_iterator_object_locals.insert(local_id);
    } else {
        ctx.facts.symbol_iterator_object_locals.remove(&local_id);
    }
}

pub(crate) fn resolved_expr_has_symbol_iterator_property(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> bool {
    match expr {
        ResolvedExpr::Object(props) => props
            .iter()
            .any(|(key, _)| key == SYMBOL_ITERATOR_OBJECT_KEY),
        ResolvedExpr::Ident(name) => ctx
            .resolve_local(name)
            .ok()
            .is_some_and(|local_id| ctx.facts.symbol_iterator_object_locals.contains(&local_id)),
        _ => false,
    }
}

pub(crate) fn is_generator_call_spread_operand(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::Call { callee, args, .. } = expr else {
        return false;
    };
    if !args.is_empty() {
        return false;
    }
    let ResolvedExpr::Ident(name) = callee.as_ref() else {
        return false;
    };
    ctx.facts.generator_function_names.contains(name)
}

pub(crate) fn unsupported_generator_spread_diagnostic() -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedRuntimeSubset,
        message:
            "issue-353: generator result spread requires iterator protocol runtime lowering in this milestone"
                .to_owned(),
        span: None,
        phase: None,
    }
}

pub(crate) fn unsupported_symbol_iterator_spread_diagnostic() -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message:
            "issue-353: custom iterable spread via Symbol.iterator requires iterator protocol runtime support in this milestone"
                .to_owned(),
        span: None,
        phase: None,
    }
}

pub(crate) fn update_static_object_literal_local_on_let(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    if let Some(props) = static_copy_safe_object_literal_props(ctx, expr) {
        ctx.facts
            .static_object_literal_locals
            .insert(local_id, props);
        update_static_object_literal_alias_sources(ctx, local_id, expr);
    } else {
        ctx.facts.static_object_literal_locals.remove(&local_id);
        ctx.facts
            .static_object_literal_alias_sources
            .remove(&local_id);
    }
}

pub(crate) fn update_static_function_array_like_local_on_let(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    let ResolvedExpr::FunctionExpr { params, .. } = expr else {
        ctx.facts
            .static_function_array_like_locals
            .remove(&local_id);
        return;
    };
    if params
        .iter()
        .any(|param| param.default.is_some() || param.is_rest)
    {
        ctx.facts
            .static_function_array_like_locals
            .remove(&local_id);
        return;
    }
    ctx.facts.static_function_array_like_locals.insert(
        local_id,
        StaticFunctionArrayLike {
            elements: vec![None; params.len()],
        },
    );
}

pub(crate) fn invalidate_static_function_array_like_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
) {
    ctx.facts
        .static_function_array_like_locals
        .remove(&local_id);
}

pub(crate) fn update_static_function_array_like_index(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    index: &ResolvedExpr,
    value: &ResolvedExpr,
) {
    let Some(static_receiver) = ctx
        .facts
        .static_function_array_like_locals
        .get_mut(&local_id)
    else {
        return;
    };
    let ResolvedExpr::Number(index) = index else {
        invalidate_static_function_array_like_local(ctx, local_id);
        return;
    };
    let Ok(index) = usize::try_from(*index) else {
        invalidate_static_function_array_like_local(ctx, local_id);
        return;
    };
    if index < static_receiver.elements.len() {
        static_receiver.elements[index] = Some(ResolvedArrayElement::Present(value.clone()));
    }
}

pub(crate) fn static_function_array_like_elements(
    ctx: &LoweringCtx,
    name: &str,
) -> Option<Vec<ResolvedExpr>> {
    let local_id = ctx.resolve_local(name).ok()?;
    let static_receiver = ctx.facts.static_function_array_like_locals.get(&local_id)?;
    static_receiver
        .elements
        .iter()
        .map(|elem| match elem {
            Some(ResolvedArrayElement::Present(expr)) => Some(expr.clone()),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
}

pub(crate) fn invalidate_static_object_literal_local(ctx: &mut LoweringCtx, local_id: LocalId) {
    ctx.facts.static_object_literal_locals.remove(&local_id);
    ctx.facts
        .static_object_literal_alias_sources
        .remove(&local_id);
    let dependent_aliases = ctx
        .facts
        .static_object_literal_alias_sources
        .iter()
        .filter_map(|(alias, sources)| sources.contains(&local_id).then_some(*alias))
        .collect::<Vec<_>>();
    for alias in dependent_aliases {
        ctx.facts.static_object_literal_locals.remove(&alias);
        ctx.facts.static_object_literal_alias_sources.remove(&alias);
    }
}

pub(crate) fn static_copy_safe_object_literal_props(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<Vec<(String, ResolvedExpr)>> {
    match expr {
        ResolvedExpr::Object(props) => {
            let mut flattened = Vec::new();
            for (key, value) in props {
                if key == OBJECT_SPREAD_SENTINEL {
                    flattened.extend(static_copy_safe_object_literal_props(ctx, value)?);
                    continue;
                }
                if !is_static_copy_safe_object_prop_value(value) {
                    return None;
                }
                flattened.push((key.clone(), value.clone()));
            }
            Some(flattened)
        }
        ResolvedExpr::Ident(name) => {
            let local_id = ctx.resolve_local(name).ok()?;
            if ctx.facts.env_cell_locals.contains(&local_id) {
                return None;
            }
            ctx.facts
                .static_object_literal_locals
                .get(&local_id)
                .cloned()
        }
        _ => None,
    }
}

pub(crate) fn update_static_object_literal_alias_sources(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    ctx.facts
        .static_object_literal_alias_sources
        .remove(&local_id);
    if let ResolvedExpr::Ident(name) = expr
        && let Ok(source_id) = ctx.resolve_local(name)
    {
        let mut sources = ctx
            .facts
            .static_object_literal_alias_sources
            .get(&source_id)
            .cloned()
            .unwrap_or_default();
        sources.insert(source_id);
        ctx.facts
            .static_object_literal_alias_sources
            .insert(local_id, sources);
    }
}

pub(crate) fn resolved_expr_produces_dense_array(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Array(_) => true,
        ResolvedExpr::Ident(name) => ctx.resolve_local(name).ok().is_some_and(|local_id| {
            ctx.facts.array_locals.contains(&local_id)
                && !ctx.facts.env_cell_locals.contains(&local_id)
        }),
        ResolvedExpr::Binary {
            left,
            op: BinaryOp::Or | BinaryOp::And,
            right,
        } => {
            resolved_expr_produces_dense_array(ctx, left)
                || resolved_expr_produces_dense_array(ctx, right)
        }
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } if method == "map" => {
            is_known_array_expr(ctx, object)
                && (string_constructor_arrow_callback(args) || unary_plus_arrow_callback(args))
        }
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } if method == "matchAll" => {
            crate::lowered::resolver::string::resolved_expr_static_string_value(ctx, object)
                .is_some()
                && matches!(
                    args.as_slice(),
                    [ResolvedExpr::String(raw)] if looks_like_regexp_literal(raw)
                )
        }
        ResolvedExpr::Call { callee, .. } => match callee.as_ref() {
            ResolvedExpr::Ident(name) => ctx
                .resolve_func(name)
                .ok()
                .and_then(|func_id| ctx.symbols.function_signatures.get(&func_id))
                .is_some_and(|signature| signature.returns_dense_array),
            _ => false,
        },
        _ => false,
    }
}

pub(crate) fn update_native_set_add_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    if is_set_prototype_property_expr(expr, "add") {
        ctx.facts.native_set_add_locals.insert(local_id);
    } else {
        ctx.facts.native_set_add_locals.remove(&local_id);
    }
}

pub(crate) fn update_invalid_date_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    if is_invalid_date_constructor_expr(expr) {
        ctx.facts.invalid_date_locals.insert(local_id);
    } else {
        ctx.facts.invalid_date_locals.remove(&local_id);
    }
}

pub(crate) fn is_invalid_date_expr(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::New { .. } => is_invalid_date_constructor_expr(expr),
        ResolvedExpr::Ident(name) => ctx
            .resolve_local(name)
            .ok()
            .is_some_and(|local_id| ctx.facts.invalid_date_locals.contains(&local_id)),
        _ => false,
    }
}

pub(crate) fn is_known_array_expr(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Array(_) => true,
        ResolvedExpr::Ident(name) => ctx.resolve_local(name).ok().is_some_and(|local_id| {
            ctx.facts.array_locals.contains(&local_id)
                || ctx
                    .classes
                    .local_classes
                    .get(&local_id)
                    .is_some_and(|class_name| is_typed_array_class(class_name))
        }),
        _ => false,
    }
}

pub(crate) fn resolved_expr_static_array_slots(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<Vec<ResolvedArrayElement>> {
    match expr {
        ResolvedExpr::Array(elements) => Some(elements.clone()),
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "Array" => {
            let [ResolvedExpr::Number(length)] = args.as_slice() else {
                return None;
            };
            if *length < 0 || *length > 32 {
                return None;
            }
            Some(vec![ResolvedArrayElement::Hole; *length as usize])
        }
        ResolvedExpr::Ident(name) => ctx
            .resolve_local(name)
            .ok()
            .and_then(|local_id| ctx.facts.static_array_slots.get(&local_id).cloned()),
        _ => None,
    }
}

pub(crate) fn update_static_array_slot_assignment(ctx: &mut LoweringCtx, expr: &ResolvedExpr) {
    let ResolvedExpr::PropertyAssignDynamic { object, key, value } = expr else {
        return;
    };
    let ResolvedExpr::Ident(name) = object.as_ref() else {
        return;
    };
    let Ok(local_id) = ctx.resolve_local(name) else {
        return;
    };
    if !ctx.facts.static_array_slots.contains_key(&local_id) {
        return;
    }
    let ResolvedExpr::Number(index) = key.as_ref() else {
        ctx.facts.static_array_slots.remove(&local_id);
        return;
    };
    let Some(slots) = ctx.facts.static_array_slots.get_mut(&local_id) else {
        return;
    };
    if *index < 0 || *index as usize >= slots.len() {
        ctx.facts.static_array_slots.remove(&local_id);
        return;
    }
    slots[*index as usize] = ResolvedArrayElement::Present(value.as_ref().clone());
}

pub(crate) fn expr_is_known_heap_closure(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Call { callee, .. } => match callee.as_ref() {
            ResolvedExpr::Ident(name) => ctx
                .resolve_func(name)
                .ok()
                .and_then(|func_id| ctx.symbols.function_signatures.get(&func_id))
                .is_some_and(|signature| signature.returns_heap_closure),
            _ => false,
        },
        ResolvedExpr::Ident(name) => ctx
            .resolve_local(name)
            .ok()
            .is_some_and(|local_id| ctx.facts.heap_closure_locals.contains(&local_id)),
        _ => false,
    }
}

pub(crate) fn resolved_expr_is_bigint(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::BigIntLiteral { .. } => true,
        ResolvedExpr::Ident(name) => ctx
            .resolve_local(name)
            .ok()
            .is_some_and(|local_id| ctx.facts.bigint_locals.contains(&local_id)),
        ResolvedExpr::Unary { op, expr } => {
            *op == UnaryOp::Negate && resolved_expr_is_bigint(ctx, expr)
        }
        ResolvedExpr::Binary { left, op, right } => {
            matches!(
                op,
                BinaryOp::Add
                    | BinaryOp::Subtract
                    | BinaryOp::Multiply
                    | BinaryOp::Power
                    | BinaryOp::Divide
                    | BinaryOp::Modulo
            ) && resolved_expr_is_bigint(ctx, left)
                && resolved_expr_is_bigint(ctx, right)
        }
        ResolvedExpr::Call { callee, .. } => matches!(
            callee.as_ref(),
            ResolvedExpr::Ident(name) if super::super::bigint_runtime_fn_intrinsic(name).is_some()
        ),
        ResolvedExpr::MethodCall { object, method, .. } => {
            matches!(
                object.as_ref(),
                ResolvedExpr::Ident(name) if name == "__ts2wasm_bigint_runtime"
            ) && super::super::bigint_runtime_fn_intrinsic(method).is_some()
        }
        _ => false,
    }
}

pub(crate) fn resolved_expr_is_bigint_div_rem_operand(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> bool {
    match expr {
        ResolvedExpr::Ident(name) => ctx.resolve_local(name).ok().is_some_and(|local_id| {
            ctx.facts.bigint_locals.contains(&local_id)
                || ctx
                    .facts
                    .control_flow_bigint_div_rem_locals
                    .contains(&local_id)
        }),
        ResolvedExpr::Unary { op, expr } => {
            *op == UnaryOp::Negate && resolved_expr_is_bigint_div_rem_operand(ctx, expr)
        }
        _ => resolved_expr_is_bigint(ctx, expr),
    }
}

pub(crate) fn resolved_expr_is_control_flow_mixed_bigint(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> bool {
    let ResolvedExpr::Ident(name) = expr else {
        return false;
    };
    ctx.resolve_local(name).ok().is_some_and(|local_id| {
        ctx.facts
            .control_flow_mixed_bigint_locals
            .contains(&local_id)
    })
}

pub(crate) fn bigint_div_rem_candidate_locals(ctx: &LoweringCtx) -> HashSet<LocalId> {
    ctx.facts
        .bigint_locals
        .union(&ctx.facts.control_flow_bigint_div_rem_locals)
        .copied()
        .collect()
}
