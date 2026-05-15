use super::super::program_builtins::{is_typed_array_class, looks_like_regexp_literal};
use super::super::{
    is_invalid_date_constructor_expr, is_set_prototype_property_expr,
    is_static_copy_safe_object_prop_value, string_constructor_arrow_callback,
    unary_plus_arrow_callback,
};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedObjectProp};
use crate::lowered::ctx::LoweringCtx;
use crate::lowered::facts::StaticFunctionArrayLike;
use crate::lowered::facts::{GeneratorIteratorBinding, ProxyBinding};
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

pub(crate) fn update_array_iterator_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
) {
    if resolved_expr_is_array_iterator(ctx, expr) {
        ctx.facts.array_iterator_locals.insert(local_id);
    } else {
        ctx.facts.array_iterator_locals.remove(&local_id);
    }
}

pub(crate) fn update_generator_iterator_local(
    ctx: &mut LoweringCtx,
    local_id: LocalId,
    expr: &ResolvedExpr,
    state_local: Option<LocalId>,
) {
    if let Some(func_name) = resolved_generator_function_call_name(ctx, expr) {
        ctx.facts.generator_iterator_locals.insert(local_id);
        if let Some(state_local) = state_local {
            ctx.facts.generator_iterator_bindings.insert(
                local_id,
                GeneratorIteratorBinding {
                    func_name,
                    state_local,
                    resume_args: Vec::new(),
                },
            );
        } else {
            ctx.facts.generator_iterator_bindings.remove(&local_id);
        }
    } else if let ResolvedExpr::Ident(name) = expr
        && let Ok(source_local) = ctx.resolve_local(name)
        && let Some(binding) = ctx
            .facts
            .generator_iterator_bindings
            .get(&source_local)
            .cloned()
    {
        ctx.facts.generator_iterator_locals.insert(local_id);
        ctx.facts
            .generator_iterator_bindings
            .insert(local_id, binding);
    } else if resolved_expr_is_generator_iterator(ctx, expr) {
        ctx.facts.generator_iterator_locals.insert(local_id);
        ctx.facts.generator_iterator_bindings.remove(&local_id);
    } else {
        ctx.facts.generator_iterator_locals.remove(&local_id);
        ctx.facts.generator_iterator_bindings.remove(&local_id);
    }
}

pub(crate) fn update_proxy_local(ctx: &mut LoweringCtx, local_id: LocalId, expr: &ResolvedExpr) {
    if let ResolvedExpr::New {
        class_name, args, ..
    } = expr
        && class_name == "Proxy"
        && let [target, handler] = args.as_slice()
    {
        ctx.facts.proxy_locals.insert(
            local_id,
            ProxyBinding {
                target: target.clone(),
                handler: handler.clone(),
            },
        );
    } else {
        ctx.facts.proxy_locals.remove(&local_id);
    }
}

pub(crate) fn resolved_expr_proxy_binding(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<ProxyBinding> {
    let ResolvedExpr::Ident(name) = expr else {
        return None;
    };
    ctx.resolve_local(name)
        .ok()
        .and_then(|local_id| ctx.facts.proxy_locals.get(&local_id).cloned())
}

pub(crate) fn resolved_expr_is_array_iterator(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Ident(name) => ctx
            .resolve_local(name)
            .ok()
            .is_some_and(|local_id| ctx.facts.array_iterator_locals.contains(&local_id)),
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } => {
            args.is_empty()
                && matches!(method.as_str(), "values" | "keys" | "entries")
                && is_known_array_expr(ctx, object)
        }
        _ => false,
    }
}

pub(crate) fn resolved_expr_is_generator_iterator(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Ident(name) => ctx
            .resolve_local(name)
            .ok()
            .is_some_and(|local_id| ctx.facts.generator_iterator_locals.contains(&local_id)),
        ResolvedExpr::Call { callee, .. } => {
            resolved_generator_function_call_name(ctx, expr).is_some()
                || resolved_callee_is_local_generator_function(ctx, callee)
                || matches!(
                    callee.as_ref(),
                    ResolvedExpr::FunctionExpr {
                        is_generator: true,
                        ..
                    }
                )
        }
        ResolvedExpr::MethodCall { object, method, .. } => {
            resolved_object_method_is_generator(ctx, object, method)
        }
        _ => false,
    }
}

fn resolved_callee_is_local_generator_function(ctx: &LoweringCtx, callee: &ResolvedExpr) -> bool {
    let ResolvedExpr::Ident(name) = callee else {
        return false;
    };
    let Ok(local_id) = ctx.resolve_local(name) else {
        return false;
    };
    let Some(closure) = ctx.facts.arrow_locals.get(&local_id) else {
        return false;
    };
    ctx.functions
        .generated_functions
        .iter()
        .any(|function| function.id == closure.func_id && function.is_generator)
}

fn resolved_object_method_is_generator(
    ctx: &LoweringCtx,
    object: &ResolvedExpr,
    method: &str,
) -> bool {
    let ResolvedExpr::Ident(receiver_name) = object else {
        return false;
    };
    let Ok(receiver_local) = ctx.resolve_local(receiver_name) else {
        return false;
    };
    let Some(method_id) = ctx
        .classes
        .object_function_props
        .get(&receiver_local)
        .and_then(|props| {
            props.get(&crate::lowered::classes::ObjectAccessorKey::Property(
                method.to_owned(),
            ))
        })
    else {
        return false;
    };
    ctx.functions
        .generated_functions
        .iter()
        .any(|function| function.id == *method_id && function.is_generator)
}

pub(crate) fn resolved_generator_function_call_name(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<String> {
    match expr {
        ResolvedExpr::Call { callee, args, .. } if args.is_empty() => match callee.as_ref() {
            ResolvedExpr::Ident(name) if ctx.facts.generator_function_names.contains(name) => {
                Some(name.clone())
            }
            _ => None,
        },
        _ => None,
    }
}

pub(crate) fn resolved_expr_has_symbol_iterator_property(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> bool {
    match expr {
        ResolvedExpr::Object(props) => props
            .iter()
            .any(|prop| prop.static_key() == Some(SYMBOL_ITERATOR_OBJECT_KEY)),
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
) -> Option<Vec<ResolvedObjectProp>> {
    match expr {
        ResolvedExpr::Object(props) => {
            let mut flattened = Vec::new();
            for prop in props {
                let key = prop.static_key()?;
                let value = prop.value();
                if key == OBJECT_SPREAD_SENTINEL {
                    flattened.extend(static_copy_safe_object_literal_props(ctx, value)?);
                    continue;
                }
                if !is_static_copy_safe_object_prop_value(value) {
                    return None;
                }
                flattened.push(prop.clone());
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
                    .is_some_and(|class_name| {
                        class_name == "Array" || is_typed_array_class(class_name)
                    })
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
