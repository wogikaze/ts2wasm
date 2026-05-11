use super::super::{
    is_invalid_date_constructor_expr, is_set_prototype_property_expr,
    is_static_copy_safe_object_prop_value, string_constructor_arrow_callback,
    unary_plus_arrow_callback,
};
use super::super::program_builtins::looks_like_regexp_literal;
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr};
use crate::lowered::facts::StaticFunctionArrayLike;
use crate::lowered::*;
use std::collections::HashSet;
use ts2wasm_shared::{BinaryOp, OBJECT_SPREAD_SENTINEL, SYMBOL_ITERATOR_OBJECT_KEY, UnaryOp};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(crate) fn update_bigint_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if self.resolved_expr_is_bigint(expr) {
            self.ctx.facts.bigint_locals.insert(local_id);
        } else {
            self.ctx.facts.bigint_locals.remove(&local_id);
        }
    }

    pub(crate) fn update_control_flow_bigint_assignment(&mut self, local_id: LocalId) {
        self.ctx
            .facts
            .control_flow_bigint_div_rem_locals
            .remove(&local_id);
        self.ctx
            .facts
            .control_flow_mixed_bigint_locals
            .remove(&local_id);
    }

    pub(crate) fn update_nullish_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if self.resolved_expr_is_nullish(expr) {
            self.ctx.facts.nullish_locals.insert(local_id);
        } else {
            self.ctx.facts.nullish_locals.remove(&local_id);
        }
    }

    pub(crate) fn resolved_expr_is_nullish(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Null | ResolvedExpr::Undefined => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.ctx.facts.nullish_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(crate) fn update_array_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if let Some(slots) = self.resolved_expr_static_array_slots(expr) {
            self.ctx.facts.array_locals.insert(local_id);
            self.ctx.facts.static_array_slots.insert(local_id, slots);
        } else if self.resolved_expr_produces_dense_array(expr) {
            self.ctx.facts.array_locals.insert(local_id);
            self.ctx.facts.static_array_slots.remove(&local_id);
        } else {
            self.ctx.facts.array_locals.remove(&local_id);
            self.ctx.facts.static_array_slots.remove(&local_id);
        }
    }

    pub(crate) fn update_symbol_iterator_object_local(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        if self.resolved_expr_has_symbol_iterator_property(expr) {
            self.ctx
                .facts
                .symbol_iterator_object_locals
                .insert(local_id);
        } else {
            self.ctx
                .facts
                .symbol_iterator_object_locals
                .remove(&local_id);
        }
    }

    pub(crate) fn resolved_expr_has_symbol_iterator_property(
        &self,
        expr: &ResolvedExpr,
    ) -> bool {
        match expr {
            ResolvedExpr::Object(props) => props
                .iter()
                .any(|(key, _)| key == SYMBOL_ITERATOR_OBJECT_KEY),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| {
                    self.ctx
                        .facts
                        .symbol_iterator_object_locals
                        .contains(&local_id)
                }),
            _ => false,
        }
    }

    pub(crate) fn is_generator_call_spread_operand(&self, expr: &ResolvedExpr) -> bool {
        let ResolvedExpr::Call { callee, args, .. } = expr else {
            return false;
        };
        if !args.is_empty() {
            return false;
        }
        let ResolvedExpr::Ident(name) = callee.as_ref() else {
            return false;
        };
        self.ctx.facts.generator_function_names.contains(name)
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
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        if let Some(props) = self.static_copy_safe_object_literal_props(expr) {
            self.ctx
                .facts
                .static_object_literal_locals
                .insert(local_id, props);
            self.update_static_object_literal_alias_sources(local_id, expr);
        } else {
            self.ctx
                .facts
                .static_object_literal_locals
                .remove(&local_id);
            self.ctx
                .facts
                .static_object_literal_alias_sources
                .remove(&local_id);
        }
    }

    pub(crate) fn update_static_function_array_like_local_on_let(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        let ResolvedExpr::FunctionExpr { params, .. } = expr else {
            self.ctx
                .facts
                .static_function_array_like_locals
                .remove(&local_id);
            return;
        };
        if params
            .iter()
            .any(|param| param.default.is_some() || param.is_rest)
        {
            self.ctx
                .facts
                .static_function_array_like_locals
                .remove(&local_id);
            return;
        }
        self.ctx.facts.static_function_array_like_locals.insert(
            local_id,
            StaticFunctionArrayLike {
                elements: vec![None; params.len()],
            },
        );
    }

    pub(crate) fn invalidate_static_function_array_like_local(&mut self, local_id: LocalId) {
        self.ctx
            .facts
            .static_function_array_like_locals
            .remove(&local_id);
    }

    pub(crate) fn update_static_function_array_like_index(
        &mut self,
        local_id: LocalId,
        index: &ResolvedExpr,
        value: &ResolvedExpr,
    ) {
        let Some(static_receiver) = self
            .ctx
            .facts
            .static_function_array_like_locals
            .get_mut(&local_id)
        else {
            return;
        };
        let ResolvedExpr::Number(index) = index else {
            self.invalidate_static_function_array_like_local(local_id);
            return;
        };
        let Ok(index) = usize::try_from(*index) else {
            self.invalidate_static_function_array_like_local(local_id);
            return;
        };
        if index < static_receiver.elements.len() {
            static_receiver.elements[index] = Some(ResolvedArrayElement::Present(value.clone()));
        }
    }

    pub(crate) fn static_function_array_like_elements(
        &self,
        name: &str,
    ) -> Option<Vec<ResolvedExpr>> {
        let local_id = self.resolve_local(name).ok()?;
        let static_receiver = self
            .ctx
            .facts
            .static_function_array_like_locals
            .get(&local_id)?;
        static_receiver
            .elements
            .iter()
            .map(|elem| match elem {
                Some(ResolvedArrayElement::Present(expr)) => Some(expr.clone()),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()
    }

    pub(crate) fn invalidate_static_object_literal_local(&mut self, local_id: LocalId) {
        self.ctx
            .facts
            .static_object_literal_locals
            .remove(&local_id);
        self.ctx
            .facts
            .static_object_literal_alias_sources
            .remove(&local_id);
        let dependent_aliases = self
            .ctx
            .facts
            .static_object_literal_alias_sources
            .iter()
            .filter_map(|(alias, sources)| sources.contains(&local_id).then_some(*alias))
            .collect::<Vec<_>>();
        for alias in dependent_aliases {
            self.ctx.facts.static_object_literal_locals.remove(&alias);
            self.ctx
                .facts
                .static_object_literal_alias_sources
                .remove(&alias);
        }
    }

    pub(crate) fn static_copy_safe_object_literal_props(
        &self,
        expr: &ResolvedExpr,
    ) -> Option<Vec<(String, ResolvedExpr)>> {
        match expr {
            ResolvedExpr::Object(props) => {
                let mut flattened = Vec::new();
                for (key, value) in props {
                    if key == OBJECT_SPREAD_SENTINEL {
                        flattened.extend(self.static_copy_safe_object_literal_props(value)?);
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
                let local_id = self.resolve_local(name).ok()?;
                if self.ctx.facts.env_cell_locals.contains(&local_id) {
                    return None;
                }
                self.ctx
                    .facts
                    .static_object_literal_locals
                    .get(&local_id)
                    .cloned()
            }
            _ => None,
        }
    }

    pub(crate) fn update_static_object_literal_alias_sources(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        self.ctx
            .facts
            .static_object_literal_alias_sources
            .remove(&local_id);
        if let ResolvedExpr::Ident(name) = expr
            && let Ok(source_id) = self.resolve_local(name)
        {
            let mut sources = self
                .ctx
                .facts
                .static_object_literal_alias_sources
                .get(&source_id)
                .cloned()
                .unwrap_or_default();
            sources.insert(source_id);
            self.ctx
                .facts
                .static_object_literal_alias_sources
                .insert(local_id, sources);
        }
    }

    pub(crate) fn resolved_expr_produces_dense_array(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Array(_) => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| {
                    self.ctx.facts.array_locals.contains(&local_id)
                        && !self.ctx.facts.env_cell_locals.contains(&local_id)
                }),
            ResolvedExpr::Binary {
                left,
                op: BinaryOp::Or | BinaryOp::And,
                right,
            } => {
                self.resolved_expr_produces_dense_array(left)
                    || self.resolved_expr_produces_dense_array(right)
            }
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                ..
            } if method == "map" => {
                self.is_known_array_expr(object)
                    && (string_constructor_arrow_callback(args)
                        || unary_plus_arrow_callback(args))
            }
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                ..
            } if method == "matchAll" => {
                self.resolved_expr_static_string_value(object).is_some()
                    && matches!(
                        args.as_slice(),
                        [ResolvedExpr::String(raw)] if looks_like_regexp_literal(raw)
                    )
            }
            ResolvedExpr::Call { callee, .. } => match callee.as_ref() {
                ResolvedExpr::Ident(name) => self
                    .resolve_func(name)
                    .ok()
                    .and_then(|func_id| {
                        self.ctx.symbols.function_signatures.get(&func_id)
                    })
                    .is_some_and(|signature| signature.returns_dense_array),
                _ => false,
            },
            _ => false,
        }
    }

    pub(crate) fn update_native_set_add_local(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        if is_set_prototype_property_expr(expr, "add") {
            self.ctx.facts.native_set_add_locals.insert(local_id);
        } else {
            self.ctx.facts.native_set_add_locals.remove(&local_id);
        }
    }

    pub(crate) fn update_invalid_date_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if is_invalid_date_constructor_expr(expr) {
            self.ctx.facts.invalid_date_locals.insert(local_id);
        } else {
            self.ctx.facts.invalid_date_locals.remove(&local_id);
        }
    }

    pub(crate) fn is_invalid_date_expr(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::New { .. } => is_invalid_date_constructor_expr(expr),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.ctx.facts.invalid_date_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(crate) fn is_known_array_expr(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Array(_) => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.ctx.facts.array_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(crate) fn resolved_expr_static_array_slots(
        &self,
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
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .and_then(|local_id| {
                    self.ctx
                        .facts
                        .static_array_slots
                        .get(&local_id)
                        .cloned()
                }),
            _ => None,
        }
    }

    pub(crate) fn update_static_array_slot_assignment(&mut self, expr: &ResolvedExpr) {
        let ResolvedExpr::PropertyAssignDynamic { object, key, value } = expr else {
            return;
        };
        let ResolvedExpr::Ident(name) = object.as_ref() else {
            return;
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return;
        };
        if !self.ctx.facts.static_array_slots.contains_key(&local_id) {
            return;
        }
        let ResolvedExpr::Number(index) = key.as_ref() else {
            self.ctx.facts.static_array_slots.remove(&local_id);
            return;
        };
        let Some(slots) = self.ctx.facts.static_array_slots.get_mut(&local_id) else {
            return;
        };
        if *index < 0 || *index as usize >= slots.len() {
            self.ctx.facts.static_array_slots.remove(&local_id);
            return;
        }
        slots[*index as usize] = ResolvedArrayElement::Present(value.as_ref().clone());
    }

    pub(crate) fn expr_is_known_heap_closure(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Call { callee, .. } => match callee.as_ref() {
                ResolvedExpr::Ident(name) => self
                    .resolve_func(name)
                    .ok()
                    .and_then(|func_id| {
                        self.ctx.symbols.function_signatures.get(&func_id)
                    })
                    .is_some_and(|signature| signature.returns_heap_closure),
                _ => false,
            },
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| {
                    self.ctx.facts.heap_closure_locals.contains(&local_id)
                }),
            _ => false,
        }
    }

    pub(crate) fn resolved_expr_is_bigint(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::BigIntLiteral { .. } => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.ctx.facts.bigint_locals.contains(&local_id)),
            ResolvedExpr::Unary { op, expr } => {
                *op == UnaryOp::Negate && self.resolved_expr_is_bigint(expr)
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
                ) && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
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
        &self,
        expr: &ResolvedExpr,
    ) -> bool {
        match expr {
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| {
                    self.ctx.facts.bigint_locals.contains(&local_id)
                        || self
                            .ctx
                            .facts
                            .control_flow_bigint_div_rem_locals
                            .contains(&local_id)
                }),
            ResolvedExpr::Unary { op, expr } => {
                *op == UnaryOp::Negate
                    && self.resolved_expr_is_bigint_div_rem_operand(expr)
            }
            _ => self.resolved_expr_is_bigint(expr),
        }
    }

    pub(crate) fn resolved_expr_is_control_flow_mixed_bigint(
        &self,
        expr: &ResolvedExpr,
    ) -> bool {
        let ResolvedExpr::Ident(name) = expr else {
            return false;
        };
        self.resolve_local(name).ok().is_some_and(|local_id| {
            self.ctx
                .facts
                .control_flow_mixed_bigint_locals
                .contains(&local_id)
        })
    }

    pub(crate) fn bigint_div_rem_candidate_locals(&self) -> HashSet<LocalId> {
        self.ctx
            .facts
            .bigint_locals
            .union(&self.ctx.facts.control_flow_bigint_div_rem_locals)
            .copied()
            .collect()
    }
}
