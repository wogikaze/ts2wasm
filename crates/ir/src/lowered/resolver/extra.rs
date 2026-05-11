use std::collections::HashSet;

use super::{
    Resolver, StaticFunctionArrayLike, is_invalid_date_constructor_expr,
    is_set_prototype_property_expr, is_static_copy_safe_object_prop_value, lowered_binding_default,
    string_constructor_arrow_callback, unary_plus_arrow_callback,
};
use crate::binding_pattern::{ArrayBinding, BindingDefault, BindingPattern, ObjectBinding};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedStmt};
use crate::lowered::*;
use ts2wasm_shared::{BinaryOp, OBJECT_SPREAD_SENTINEL, SYMBOL_ITERATOR_OBJECT_KEY, UnaryOp};
use ts2wasm_shared::{DiagCode, Diagnostic, Span};

impl<'a> Resolver<'a> {
    pub(crate) fn lower_binding_pattern_declarations(
        &mut self,
        pattern: &BindingPattern,
        value: LoweredExpr,
        source: Option<&ResolvedExpr>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        match pattern {
            BindingPattern::Array(bindings) => {
                let mut statements = Vec::new();
                for binding in bindings {
                    statements.extend(self.lower_array_binding_declaration(binding, &value)?);
                }
                Ok(statements)
            }
            BindingPattern::Object(bindings) => {
                let mut statements = Vec::new();
                for binding in bindings {
                    statements.extend(
                        self.lower_object_binding_declaration(binding, bindings, &value, source)?,
                    );
                }
                Ok(statements)
            }
        }
    }

    pub(super) fn lower_array_binding_declaration(
        &mut self,
        binding: &ArrayBinding,
        value: &LoweredExpr,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let element_value = if binding.is_rest {
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::ArraySlice,
                args: vec![
                    value.clone(),
                    LoweredExpr::Number(binding.index as i32, Span::generated("num")),
                    LoweredExpr::GetLength(Box::new(value.clone()), Span::generated("get_length")),
                ],

                span: Span::generated("runtime_call"),
            }
        } else {
            LoweredExpr::Index {
                object: Box::new(value.clone()),
                index: Box::new(LoweredExpr::Number(
                    binding.index as i32,
                    Span::generated("num"),
                )),
                span: Span::generated("index"),
            }
        };
        let Some(name) = binding.target.identifier() else {
            if let Some(pattern) = binding.target.pattern() {
                return self.lower_binding_pattern_declarations(pattern, element_value, None);
            }
            unreachable!("binding target must be identifier or pattern");
        };
        let local_id = self.declare_local(name)?;
        if binding.is_rest {
            return Ok(vec![LoweredStmt::Let(
                local_id,
                element_value,
                Span::generated("let_stmt"),
            )]);
        }
        self.lower_binding_declaration_with_default(
            local_id,
            element_value,
            binding.default.as_ref(),
        )
    }

    pub(super) fn lower_object_binding_declaration(
        &mut self,
        binding: &ObjectBinding,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        source: Option<&ResolvedExpr>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let property_value = if binding.computed {
            // Computed key: resolve the identifier inside [foo] and use dynamic lookup
            // Extract identifier from serialized AST: "[Ident { name: \"key\", ...}]"
            let key_raw = binding.key.trim_start_matches('[').trim_end_matches(']');
            let key_name = if let Some(start) = key_raw.find("name: \"") {
                let after_start = &key_raw[start + 7..];
                if let Some(end) = after_start.find('\"') {
                    &after_start[..end]
                } else {
                    key_raw
                }
            } else {
                key_raw
            };
            let key_local = self.resolve_local(key_name)?;
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(value.clone()),
                key: Box::new(LoweredExpr::Local(key_local, Span::generated("local"))),
                span: Span::generated("prop_get_dynamic"),
            }
        } else {
            LoweredExpr::PropertyGet {
                obj: Box::new(value.clone()),
                key: binding.key.clone(),
                span: Span::generated("prop_get"),
            }
        };
        let Some(name) = binding.target.identifier() else {
            if let Some(pattern) = binding.target.pattern() {
                return self.lower_binding_pattern_declarations(pattern, property_value, None);
            }
            unreachable!("binding target must be identifier or pattern");
        };
        let local_id = self.declare_local(name)?;
        if binding.is_rest {
            return self.lower_object_rest_binding_declaration(
                local_id,
                siblings,
                value,
                source,
                binding.span,
            );
        }
        self.lower_binding_declaration_with_default(
            local_id,
            property_value,
            binding.default.as_ref(),
        )
    }

    pub(super) fn lower_object_rest_binding_declaration(
        &mut self,
        local_id: LocalId,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        source: Option<&ResolvedExpr>,
        span: Option<Span>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let Some(ResolvedExpr::Object(props)) = source else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-251: object rest binding currently requires a static object literal source in this runtime slice".to_owned(),
                span,

                phase: None,});
        };
        let excluded_keys = siblings
            .iter()
            .filter(|binding| !binding.is_rest)
            .map(|binding| binding.key.as_str())
            .collect::<HashSet<_>>();
        let rest_props = props
            .iter()
            .filter(|(key, _)| !excluded_keys.contains(key.as_str()))
            .map(|(key, _)| {
                (
                    key.clone(),
                    LoweredExpr::PropertyGet {
                        obj: Box::new(value.clone()),
                        key: key.clone(),
                        span: Span::generated("prop_get"),
                    },
                )
            })
            .collect();
        Ok(vec![LoweredStmt::Let(
            local_id,
            LoweredExpr::ObjectNew {
                props: rest_props,
                non_enumerable: 0,
                span: Span::generated("object_new"),
            },
            Span::generated("let_stmt"),
        )])
    }

    pub(super) fn lower_binding_declaration_with_default(
        &mut self,
        local_id: LocalId,
        value: LoweredExpr,
        default: Option<&BindingDefault>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let Some(default) = default else {
            return Ok(vec![LoweredStmt::Let(
                local_id,
                value,
                Span::generated("let_stmt"),
            )]);
        };
        let temp_id = self.alloc_temp();
        Ok(vec![
            LoweredStmt::Let(temp_id, value, Span::generated("let_stmt")),
            LoweredStmt::Let(
                local_id,
                LoweredExpr::Local(temp_id, Span::generated("local")),
                Span::generated("let"),
            ),
            LoweredStmt::If {
                condition: LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(temp_id, Span::generated("local"))),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),

                    span: Span::generated("binary"),
                },
                then_body: vec![LoweredStmt::Assign(
                    local_id,
                    lowered_binding_default(default),
                    Span::generated("assign"),
                )],
                else_body: vec![],
                span: Span::generated("If"),
            },
        ])
    }

    pub(super) fn update_bigint_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if self.resolved_expr_is_bigint(expr) {
            self.facts.bigint_locals.insert(local_id);
        } else {
            self.facts.bigint_locals.remove(&local_id);
        }
    }

    pub(super) fn update_control_flow_bigint_assignment(&mut self, local_id: LocalId) {
        self.facts
            .control_flow_bigint_div_rem_locals
            .remove(&local_id);
        self.facts
            .control_flow_mixed_bigint_locals
            .remove(&local_id);
    }

    pub(super) fn update_nullish_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if self.resolved_expr_is_nullish(expr) {
            self.facts.nullish_locals.insert(local_id);
        } else {
            self.facts.nullish_locals.remove(&local_id);
        }
    }

    pub(super) fn resolved_expr_is_nullish(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Null | ResolvedExpr::Undefined => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.facts.nullish_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(super) fn update_array_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if let Some(slots) = self.resolved_expr_static_array_slots(expr) {
            self.facts.array_locals.insert(local_id);
            self.facts.static_array_slots.insert(local_id, slots);
        } else if self.resolved_expr_produces_dense_array(expr) {
            self.facts.array_locals.insert(local_id);
            self.facts.static_array_slots.remove(&local_id);
        } else {
            self.facts.array_locals.remove(&local_id);
            self.facts.static_array_slots.remove(&local_id);
        }
    }

    pub(super) fn update_symbol_iterator_object_local(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        if self.resolved_expr_has_symbol_iterator_property(expr) {
            self.facts.symbol_iterator_object_locals.insert(local_id);
        } else {
            self.facts.symbol_iterator_object_locals.remove(&local_id);
        }
    }

    pub(super) fn resolved_expr_has_symbol_iterator_property(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Object(props) => props
                .iter()
                .any(|(key, _)| key == SYMBOL_ITERATOR_OBJECT_KEY),
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local_id| {
                self.facts.symbol_iterator_object_locals.contains(&local_id)
            }),
            _ => false,
        }
    }

    pub(super) fn is_generator_call_spread_operand(&self, expr: &ResolvedExpr) -> bool {
        let ResolvedExpr::Call { callee, args, .. } = expr else {
            return false;
        };
        if !args.is_empty() {
            return false;
        }
        let ResolvedExpr::Ident(name) = callee.as_ref() else {
            return false;
        };
        self.facts.generator_function_names.contains(name)
    }

    pub(super) fn unsupported_generator_spread_diagnostic() -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedRuntimeSubset,
            message:
                "issue-353: generator result spread requires iterator protocol runtime lowering in this milestone"
                    .to_owned(),
            span: None,


            phase: None,}
    }

    pub(super) fn unsupported_symbol_iterator_spread_diagnostic() -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-353: custom iterable spread via Symbol.iterator requires iterator protocol runtime support in this milestone"
                    .to_owned(),
            span: None,


            phase: None,}
    }

    pub(super) fn lower_spread_via_iterator(
        &mut self,
        spread_expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        // Implements the ECMAScript iterator protocol via IR-level While loop.
        // Gets obj[Symbol.iterator] via PropertyGetDynamic with sentinel key,
        // calls it via HeapClosureCall, then loops calling .next() and collecting values.
        let sentinel_key = SYMBOL_ITERATOR_OBJECT_KEY.to_owned();
        let span = Span::generated("spread_via_iterator");
        let iterable = self.lower_expr(spread_expr)?;
        let iter_fn = self.alloc_temp();
        let iterator = self.alloc_temp();
        let result_arr = self.alloc_temp();
        let done_val = self.alloc_temp();
        let mut stmts = vec![LoweredStmt::Let(
            iter_fn,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(iterable),
                key: Box::new(LoweredExpr::String(sentinel_key, Span::generated("str"))),
                span,
            },
            span,
        )];
        stmts.push(LoweredStmt::Let(
            iterator,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::HeapClosureCall,
                args: vec![LoweredExpr::Local(iter_fn, Span::generated("local"))],
                span,
            },
            span,
        ));
        stmts.push(LoweredStmt::Let(
            result_arr,
            LoweredExpr::ArrayNew {
                elements: vec![],
                span,
            },
            span,
        ));
        stmts.push(LoweredStmt::Let(
            done_val,
            LoweredExpr::Bool(false, Span::generated("bool")),
            span,
        ));
        let next_fn = self.alloc_temp();
        let r = self.alloc_temp();
        let value = self.alloc_temp();
        let mut body = Vec::new();
        body.push(LoweredStmt::Let(
            next_fn,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(iterator, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "next".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        ));
        body.push(LoweredStmt::Let(
            r,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::HeapClosureCall,
                args: vec![LoweredExpr::Local(next_fn, Span::generated("local"))],
                span,
            },
            span,
        ));
        body.push(LoweredStmt::Let(
            done_val,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(r, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "done".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        ));
        let mut push_body = vec![LoweredStmt::Let(
            value,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(r, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "value".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        )];
        push_body.push(LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::ArrayPush,
                args: vec![
                    LoweredExpr::Local(result_arr, Span::generated("local")),
                    LoweredExpr::Local(value, Span::generated("local")),
                ],
                span,
            },
            span,
        ));
        body.push(LoweredStmt::If {
            condition: LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Local(done_val, Span::generated("local"))),
                span,
            },
            then_body: push_body,
            else_body: vec![],
            span,
        });
        stmts.push(LoweredStmt::DoWhile {
            body,
            condition: LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Local(done_val, Span::generated("local"))),
                span,
            },
            span,
        });
        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(LoweredExpr::Local(result_arr, Span::generated("local"))),
            span,
        })
    }

    /// Lower for-of to IR-level iterator protocol (PropertyGetDynamic + HeapClosureCall + DoWhile).
    /// Follows the same pattern as lower_spread_via_iterator.
    /// For-of body is placed inside an `If(!done)` guard within the loop.
    pub(super) fn lower_for_of_via_iterator(
        &mut self,
        var_id: LocalId,
        iter_expr: &ResolvedExpr,
        body_stmts: &[ResolvedStmt],
    ) -> Result<LoweredStmt, Diagnostic> {
        let sentinel_key = SYMBOL_ITERATOR_OBJECT_KEY.to_owned();
        let span = Span::generated("for_of_via_iterator");
        let iterable = self.lower_expr(iter_expr)?;
        let iter_fn = self.alloc_temp();
        let iterator = self.alloc_temp();
        let done_val = self.alloc_temp();
        // Setup: get [Symbol.iterator] function, call it to get iterator, init done = false
        let mut stmts = vec![
            LoweredStmt::Let(
                iter_fn,
                LoweredExpr::PropertyGetDynamic {
                    obj: Box::new(iterable),
                    key: Box::new(LoweredExpr::String(sentinel_key, Span::generated("str"))),
                    span,
                },
                span,
            ),
            LoweredStmt::Let(
                iterator,
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::HeapClosureCall,
                    args: vec![LoweredExpr::Local(iter_fn, Span::generated("local"))],
                    span,
                },
                span,
            ),
            LoweredStmt::Let(
                done_val,
                LoweredExpr::Bool(false, Span::generated("bool")),
                span,
            ),
        ];
        // Loop body: call .next(), check .done, extract .value
        let next_fn = self.alloc_temp();
        let r = self.alloc_temp();
        let mut body = Vec::new();
        body.push(LoweredStmt::Let(
            next_fn,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(iterator, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "next".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        ));
        body.push(LoweredStmt::Let(
            r,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::HeapClosureCall,
                args: vec![LoweredExpr::Local(next_fn, Span::generated("local"))],
                span,
            },
            span,
        ));
        body.push(LoweredStmt::Let(
            done_val,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(r, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "done".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        ));
        // If !done: assign var_id = r.value, then execute for-of body
        let mut if_body = vec![LoweredStmt::Assign(
            var_id,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(r, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "value".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        )];
        if_body.extend(self.lower_nested_block(body_stmts)?);
        body.push(LoweredStmt::If {
            condition: LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Local(done_val, Span::generated("local"))),
                span,
            },
            then_body: if_body,
            else_body: vec![],
            span,
        });
        stmts.push(LoweredStmt::DoWhile {
            body,
            condition: LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Local(done_val, Span::generated("local"))),
                span,
            },
            span,
        });
        Ok(LoweredStmt::Block(stmts, span))
    }

    pub(super) fn update_static_object_literal_local_on_let(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        if let Some(props) = self.static_copy_safe_object_literal_props(expr) {
            self.facts
                .static_object_literal_locals
                .insert(local_id, props);
            self.update_static_object_literal_alias_sources(local_id, expr);
        } else {
            self.facts.static_object_literal_locals.remove(&local_id);
            self.facts
                .static_object_literal_alias_sources
                .remove(&local_id);
        }
    }

    pub(super) fn update_static_function_array_like_local_on_let(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        let ResolvedExpr::FunctionExpr { params, .. } = expr else {
            self.facts
                .static_function_array_like_locals
                .remove(&local_id);
            return;
        };
        if params
            .iter()
            .any(|param| param.default.is_some() || param.is_rest)
        {
            self.facts
                .static_function_array_like_locals
                .remove(&local_id);
            return;
        }
        self.facts.static_function_array_like_locals.insert(
            local_id,
            StaticFunctionArrayLike {
                elements: vec![None; params.len()],
            },
        );
    }

    pub(super) fn invalidate_static_function_array_like_local(&mut self, local_id: LocalId) {
        self.facts
            .static_function_array_like_locals
            .remove(&local_id);
    }

    pub(super) fn update_static_function_array_like_index(
        &mut self,
        local_id: LocalId,
        index: &ResolvedExpr,
        value: &ResolvedExpr,
    ) {
        let Some(static_receiver) = self
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
            static_receiver.elements[index] = Some(value.clone());
        }
    }

    pub(super) fn static_function_array_like_elements(
        &self,
        name: &str,
    ) -> Option<Vec<ResolvedExpr>> {
        let local_id = self.resolve_local(name).ok()?;
        let static_receiver = self
            .facts
            .static_function_array_like_locals
            .get(&local_id)?;
        static_receiver
            .elements
            .iter()
            .cloned()
            .collect::<Option<Vec<_>>>()
    }

    pub(super) fn invalidate_static_object_literal_local(&mut self, local_id: LocalId) {
        self.facts.static_object_literal_locals.remove(&local_id);
        self.facts
            .static_object_literal_alias_sources
            .remove(&local_id);
        let dependent_aliases = self
            .facts
            .static_object_literal_alias_sources
            .iter()
            .filter_map(|(alias, sources)| sources.contains(&local_id).then_some(*alias))
            .collect::<Vec<_>>();
        for alias in dependent_aliases {
            self.facts.static_object_literal_locals.remove(&alias);
            self.facts
                .static_object_literal_alias_sources
                .remove(&alias);
        }
    }

    pub(super) fn static_copy_safe_object_literal_props(
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
                if self.captures.env_cell_locals.contains(&local_id) {
                    return None;
                }
                self.facts
                    .static_object_literal_locals
                    .get(&local_id)
                    .cloned()
            }
            _ => None,
        }
    }

    pub(super) fn update_static_object_literal_alias_sources(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        self.facts
            .static_object_literal_alias_sources
            .remove(&local_id);
        if let ResolvedExpr::Ident(name) = expr
            && let Ok(source_id) = self.resolve_local(name)
        {
            let mut sources = self
                .facts
                .static_object_literal_alias_sources
                .get(&source_id)
                .cloned()
                .unwrap_or_default();
            sources.insert(source_id);
            self.facts
                .static_object_literal_alias_sources
                .insert(local_id, sources);
        }
    }

    pub(super) fn resolved_expr_produces_dense_array(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Array(_) => true,
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local_id| {
                self.facts.array_locals.contains(&local_id)
                    && !self.captures.env_cell_locals.contains(&local_id)
            }),
            // Logical OR/AND where either side produces a dense array
            // (e.g., `x || []`, `x && []`)
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
                    && (string_constructor_arrow_callback(args) || unary_plus_arrow_callback(args))
            }
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                ..
            } if method == "matchAll" => {
                self.resolved_expr_static_string_value(object).is_some()
                    && matches!(args.as_slice(), [ResolvedExpr::String(raw)] if looks_like_regexp_literal(raw))
            }
            ResolvedExpr::Call { callee, .. } => match callee.as_ref() {
                ResolvedExpr::Ident(name) => self
                    .resolve_func(name)
                    .ok()
                    .and_then(|func_id| self.symbols.function_signatures.get(&func_id))
                    .is_some_and(|signature| signature.returns_dense_array),
                _ => false,
            },
            _ => false,
        }
    }

    pub(super) fn update_native_set_add_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if is_set_prototype_property_expr(expr, "add") {
            self.facts.native_set_add_locals.insert(local_id);
        } else {
            self.facts.native_set_add_locals.remove(&local_id);
        }
    }

    pub(super) fn update_invalid_date_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if is_invalid_date_constructor_expr(expr) {
            self.facts.invalid_date_locals.insert(local_id);
        } else {
            self.facts.invalid_date_locals.remove(&local_id);
        }
    }

    pub(super) fn is_invalid_date_expr(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::New { .. } => is_invalid_date_constructor_expr(expr),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.facts.invalid_date_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(super) fn is_known_array_expr(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Array(_) => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.facts.array_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(super) fn resolved_expr_static_array_slots(
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
                .and_then(|local_id| self.facts.static_array_slots.get(&local_id).cloned()),
            _ => None,
        }
    }

    pub(super) fn update_static_array_slot_assignment(&mut self, expr: &ResolvedExpr) {
        let ResolvedExpr::PropertyAssignDynamic { object, key, value } = expr else {
            return;
        };
        let ResolvedExpr::Ident(name) = object.as_ref() else {
            return;
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return;
        };
        if !self.facts.static_array_slots.contains_key(&local_id) {
            return;
        }
        let ResolvedExpr::Number(index) = key.as_ref() else {
            self.facts.static_array_slots.remove(&local_id);
            return;
        };
        let Some(slots) = self.facts.static_array_slots.get_mut(&local_id) else {
            return;
        };
        if *index < 0 || *index as usize >= slots.len() {
            self.facts.static_array_slots.remove(&local_id);
            return;
        }
        slots[*index as usize] = ResolvedArrayElement::Present(value.as_ref().clone());
    }

    pub(super) fn expr_is_known_heap_closure(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Call { callee, .. } => match callee.as_ref() {
                ResolvedExpr::Ident(name) => self
                    .resolve_func(name)
                    .ok()
                    .and_then(|func_id| self.symbols.function_signatures.get(&func_id))
                    .is_some_and(|signature| signature.returns_heap_closure),
                _ => false,
            },
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.captures.heap_closure_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(super) fn resolved_expr_is_bigint(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::BigIntLiteral { .. } => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.facts.bigint_locals.contains(&local_id)),
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
            ResolvedExpr::Call { callee, .. } => {
                matches!(
                    callee.as_ref(),
                    ResolvedExpr::Ident(name)
                        if super::bigint_runtime_fn_intrinsic(name).is_some()
                )
            }
            ResolvedExpr::MethodCall { object, method, .. } => {
                matches!(
                    object.as_ref(),
                    ResolvedExpr::Ident(name) if name == "__ts2wasm_bigint_runtime"
                ) && super::bigint_runtime_fn_intrinsic(method).is_some()
            }
            _ => false,
        }
    }

    pub(super) fn resolved_expr_is_bigint_div_rem_operand(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local_id| {
                self.facts.bigint_locals.contains(&local_id)
                    || self
                        .facts
                        .control_flow_bigint_div_rem_locals
                        .contains(&local_id)
            }),
            ResolvedExpr::Unary { op, expr } => {
                *op == UnaryOp::Negate && self.resolved_expr_is_bigint_div_rem_operand(expr)
            }
            _ => self.resolved_expr_is_bigint(expr),
        }
    }

    pub(super) fn resolved_expr_is_control_flow_mixed_bigint(&self, expr: &ResolvedExpr) -> bool {
        let ResolvedExpr::Ident(name) = expr else {
            return false;
        };
        self.resolve_local(name).ok().is_some_and(|local_id| {
            self.facts
                .control_flow_mixed_bigint_locals
                .contains(&local_id)
        })
    }

    pub(super) fn bigint_div_rem_candidate_locals(&self) -> HashSet<LocalId> {
        self.facts
            .bigint_locals
            .union(&self.facts.control_flow_bigint_div_rem_locals)
            .copied()
            .collect()
    }
}
