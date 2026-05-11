use std::collections::HashSet;

use super::{
    ArrowClosure, Resolver, StaticFunctionArrayLike, binding_param_names,
    is_invalid_date_constructor_expr, is_set_prototype_property_expr,
    is_static_copy_safe_object_prop_value, lowered_binding_default,
    string_constructor_arrow_callback, unary_plus_arrow_callback,
};
use crate::binding_pattern::{ArrayBinding, BindingDefault, BindingPattern, ObjectBinding};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedParam, ResolvedStmt};
use crate::lowered::*;
use ts2wasm_shared::{BinaryOp, OBJECT_SPREAD_SENTINEL, SYMBOL_ITERATOR_OBJECT_KEY, UnaryOp};
use ts2wasm_shared::{DiagCode, Diagnostic, Span};

impl<'a> Resolver<'a> {
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

    pub(super) fn lower_arrow_fn(
        &mut self,
        params: &[String],
        body: &ResolvedExpr,
        body_stmts: &[ResolvedStmt],
    ) -> Result<LoweredExpr, Diagnostic> {
        self.lower_arrow_fn_with_self(params, body, body_stmts, None)
    }

    pub(super) fn lower_arrow_fn_with_self(
        &mut self,
        params: &[String],
        body: &ResolvedExpr,
        body_stmts: &[ResolvedStmt],
        self_name: Option<&str>,
    ) -> Result<LoweredExpr, Diagnostic> {
        let mut excluded = binding_param_names(params.iter().map(|param| (param.as_str(), None)))?;
        let active_self_name = self_name.filter(|name| {
            let is_shadowed_by_param = excluded.iter().any(|param| param == name);
            !is_shadowed_by_param && self.resolve_local(name).is_ok()
        });
        if let Some(name) = active_self_name {
            excluded.push(name.to_owned());
        }
        // Exclude names declared in body_stmts from capture analysis
        let mut excluded_set: HashSet<String> = excluded.iter().cloned().collect();
        collect_declared_names_in_stmts(body_stmts, &mut excluded_set);
        let mut capture_names = self.arrow_capture_names_with_excluded(body, &excluded);
        let mut stmt_captures = Vec::new();
        collect_stmt_captures(body_stmts, &excluded_set, &mut stmt_captures);
        for name in stmt_captures {
            if !capture_names.contains(&name) {
                capture_names.push(name);
            }
        }
        // If the arrow body references super.method() or super.property, the
        // super.method() lowering at resolver_expr.rs:1919 needs `this` as a
        // local to construct the first call argument.  Arrow functions do not
        // bind their own `this`, so we capture the enclosing `this` and make it
        // available in the arrow's scope so that super-method resolution works.
        if !capture_names.contains(&"this".to_owned())
            && !excluded_set.contains("this")
            && (expr_contains_super_ref(body) || block_contains_super_ref(body_stmts))
            && self.resolve_local("this").is_ok()
        {
            capture_names.push("this".to_owned());
        }
        let captures = capture_names
            .iter()
            .map(|name| self.resolve_local(name))
            .collect::<Result<Vec<_>, _>>()?;
        // Split explicit params into non-rest + rest (rest must be the final parameter
        // when captures are appended, so the WAT emitter and validator handle it correctly).
        let mut lowered_params: Vec<ResolvedParam> = Vec::new();
        let mut rest_param: Option<ResolvedParam> = None;
        for param in params {
            let rp = ResolvedParam {
                name: param.clone(),
                default: None,
                is_rest: param.starts_with("..."),
                span: None,
            };
            if rp.is_rest {
                rest_param = Some(rp);
            } else {
                lowered_params.push(rp);
            }
        }
        // Append captures (non-rest) before the rest param
        lowered_params.extend(capture_names.iter().map(|name| ResolvedParam {
            name: name.clone(),
            default: None,
            is_rest: false,
            span: None,
        }));
        // Rest param goes last
        if let Some(rp) = rest_param {
            lowered_params.push(rp);
        }

        let func_id = FuncId(self.functions.next_func_id);
        self.functions.next_func_id += 1;
        let mut lowered_body_stmts: Vec<ResolvedStmt> = body_stmts.to_vec();
        lowered_body_stmts.push(ResolvedStmt::Return((*body).clone()));
        let lowered = lower_function(
            func_id,
            &lowered_params,
            &lowered_body_stmts,
            false,
            self.symbols.function_ids,
            self.symbols.function_signatures,
            self.functions.function_captures,
            self.functions.function_mutable_captures,
            self.functions.class_method_captures,
            self.functions.class_method_mutable_captures,
            &self.captures.env_cell_names,
            &self.captures.heap_closure_names,
            self.classes.class_parents.clone(),
            self.classes.class_private_fields.clone(),
            self.classes.class_static_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.classes.current_class.as_deref(),
                in_constructor: false,
                next_func_id: self.functions.next_func_id,
                self_closure: active_self_name.map(|name| SelfClosureOptions {
                    name,
                    func_id,
                    capture_names: &capture_names,
                }),
                recursion_depth: 0,
            },
        )?;
        self.functions.next_func_id = lowered.next_func_id;
        self.functions.generated_functions.push(lowered.function);
        self.functions
            .generated_functions
            .extend(lowered.generated_functions);

        Ok(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: ClosureRepresentation::DirectLocalToken,

            span: Span::generated("arrow_fn"),
        })
    }

    pub(super) fn lower_nested_function(
        &mut self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
    ) -> Result<LoweredExpr, Diagnostic> {
        if params
            .iter()
            .any(|param| param.default.is_some() || param.is_rest)
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` closure parameters with defaults or rest are not supported in this slice"
                ),
                span: None,

                phase: None,
            });
        }
        if block_contains_this(body) || block_contains_arguments(body) {
            // If the function has an explicit `this` parameter (TypeScript syntax),
            // the `this` references are valid receiver accesses, not closure captures.
            if block_contains_this(body) && params.iter().any(|p| p.name == "this") {
                // Explicit `this` parameter: this is a receiver function, not a closure issue.
            } else if block_contains_this(body) {
                // No explicit `this` parameter — this usage will have implicit `any` type.
                // Report a more specific TS2683-compatible diagnostic.
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-5179: 'this' implicitly has type 'any' because it does not have a type annotation in nested function `{name}`"
                    ),
                    span: None,

                    phase: None,
                });
            } else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-062e: nested function `{name}` closures with `this` or `arguments` are not supported in this slice"
                    ),
                    span: None,

                    phase: None,
                });
            }
        }

        let capture_names = self.nested_function_capture_names(name, params, body)?;
        let mutable_captures = capture_names
            .iter()
            .filter(|capture| block_assigns_any_name(body, std::slice::from_ref(capture)))
            .cloned()
            .collect::<Vec<_>>();
        if mutable_captures
            .iter()
            .any(|capture| !self.captures.env_cell_names.contains(capture))
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` mutates a captured outer local; mutable closure environments require heap environment support"
                ),
                span: None,

                phase: None,
            });
        }
        let captures = capture_names
            .iter()
            .map(|capture| self.resolve_local(capture))
            .collect::<Result<Vec<_>, _>>()?;
        let mut lowered_params = params.to_vec();
        lowered_params.extend(capture_names.iter().map(|capture| ResolvedParam {
            name: capture.clone(),
            default: None,
            is_rest: false,
            span: None,
        }));

        let func_id = FuncId(self.functions.next_func_id);
        self.functions.next_func_id += 1;
        let self_closure = (!name.is_empty())
            .then_some(SelfClosureOptions {
                name,
                func_id,
                capture_names: &capture_names,
            })
            .filter(|_| !self.captures.env_cell_names.contains(name));

        let lowered = lower_function(
            func_id,
            &lowered_params,
            body,
            false,
            self.symbols.function_ids,
            self.symbols.function_signatures,
            self.functions.function_captures,
            self.functions.function_mutable_captures,
            self.functions.class_method_captures,
            self.functions.class_method_mutable_captures,
            &self.captures.env_cell_names,
            &self.captures.heap_closure_names,
            self.classes.class_parents.clone(),
            self.classes.class_private_fields.clone(),
            self.classes.class_static_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.classes.current_class.as_deref(),
                in_constructor: false,
                next_func_id: self.functions.next_func_id,
                self_closure,
                recursion_depth: 0,
            },
        )?;
        self.functions.next_func_id = lowered.next_func_id;
        self.functions.generated_functions.push(lowered.function);
        self.functions
            .generated_functions
            .extend(lowered.generated_functions);

        Ok(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: if self.captures.heap_closure_names.contains(name) {
                ClosureRepresentation::HeapObject
            } else {
                ClosureRepresentation::DirectLocalToken
            },

            span: Span::generated("arrow_fn"),
        })
    }

    pub(super) fn lower_named_function_expr(
        &mut self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
    ) -> Result<LoweredExpr, Diagnostic> {
        self.lower_nested_function(name, params, body)
    }

    pub(super) fn arrow_capture_names_with_excluded(
        &self,
        body: &ResolvedExpr,
        excluded: &[String],
    ) -> Vec<String> {
        let mut captures = Vec::new();
        collect_arrow_captures(body, excluded, &mut captures);
        captures
            .into_iter()
            .filter(|name| self.resolve_local(name).is_ok())
            .collect()
    }

    pub(super) fn nested_function_capture_names(
        &self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
    ) -> Result<Vec<String>, Diagnostic> {
        let mut excluded =
            binding_param_names(params.iter().map(|param| (param.name.as_str(), param.span)))?
                .into_iter()
                .collect::<HashSet<_>>();
        if !self.captures.env_cell_names.contains(name) {
            excluded.insert(name.to_owned());
        }
        collect_declared_names_in_stmts(body, &mut excluded);

        let mut captures = Vec::new();
        collect_stmt_captures(body, &excluded, &mut captures);
        Ok(captures
            .into_iter()
            .filter(|capture| self.resolve_local(capture).is_ok())
            .collect())
    }

    pub(crate) fn declare_local(&mut self, name: &str) -> Result<LocalId, Diagnostic> {
        let scope = self.locals.scopes.last_mut().expect("scope must exist");
        if let Some(&existing) = scope.get(name) {
            return Ok(existing);
        }
        let local_id = LocalId(self.locals.next_local_id);
        self.locals.next_local_id += 1;
        self.locals.locals.push(local_id);
        scope.insert(name.to_owned(), local_id);
        Ok(local_id)
    }

    pub(crate) fn declare_self_closure(
        &mut self,
        name: &str,
        func_id: FuncId,
        capture_names: &[String],
    ) -> Result<(), Diagnostic> {
        let local_id = self.declare_local(name)?;
        let captures = capture_names
            .iter()
            .map(|capture| self.resolve_local(capture))
            .collect::<Result<Vec<_>, _>>()?;
        self.facts
            .arrow_locals
            .insert(local_id, ArrowClosure { func_id, captures });
        Ok(())
    }

    pub(super) fn alloc_temp(&mut self) -> LocalId {
        let id = LocalId(self.locals.next_local_id);
        self.locals.next_local_id += 1;
        self.locals.locals.push(id);
        id
    }

    pub(crate) fn resolve_local(&self, name: &str) -> Result<LocalId, Diagnostic> {
        self.locals
            .scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedName,
                message: format!("unresolved name: `{name}`"),
                span: None,

                phase: None,
            })
    }

    pub(crate) fn resolve_func(&self, name: &str) -> Result<FuncId, Diagnostic> {
        self.symbols
            .function_ids
            .get(name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedFunction,
                message: format!("unresolved function: `{name}`"),
                span: Some(Span::generated("resolve_func")),

                phase: None,
            })
    }

    pub(super) fn module_id_for_specifier(&mut self, specifier: &str) -> usize {
        if let Some(id) = self.modules.module_ids.get(specifier) {
            return *id;
        }

        let id = self.modules.modules.len() + 1;
        self.modules.module_ids.insert(specifier.to_owned(), id);
        self.modules.modules.push(ModuleInfo {
            id,
            specifier: specifier.to_owned(),
            statements: Vec::new(),
            locals_count: 0,
        });
        id
    }

    pub(super) fn update_regexp_literal_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if matches!(expr, ResolvedExpr::String(raw) if looks_like_regexp_literal(raw)) {
            self.facts.regexp_literal_locals.insert(local_id);
        } else {
            self.facts.regexp_literal_locals.remove(&local_id);
        }
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

    pub(super) fn update_heap_closure_local(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
        lowered: &LoweredExpr,
    ) {
        if self.expr_is_known_heap_closure(expr)
            || matches!(
                lowered,
                LoweredExpr::ArrowFn {
                    representation: ClosureRepresentation::HeapObject,
                    ..
                }
            )
        {
            self.captures.heap_closure_locals.insert(local_id);
        } else {
            self.captures.heap_closure_locals.remove(&local_id);
        }
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

    pub(super) fn update_string_literal_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if let Some(value) = self.resolved_expr_static_string_value(expr) {
            self.facts.string_literal_locals.insert(local_id, value);
        } else {
            self.facts.string_literal_locals.remove(&local_id);
        }
    }

    pub(super) fn resolved_expr_static_string_value(&self, expr: &ResolvedExpr) -> Option<String> {
        match expr {
            ResolvedExpr::String(value) => Some(value.clone()),
            ResolvedExpr::Ident(name) => {
                let local_id = self.resolve_local(name).ok()?;
                if self.captures.env_cell_locals.contains(&local_id) {
                    return None;
                }
                self.facts.string_literal_locals.get(&local_id).cloned()
            }
            ResolvedExpr::Binary { left, op, right } if *op == BinaryOp::Add => {
                let mut value = self.resolved_expr_static_string_value(left)?;
                value.push_str(&self.resolved_expr_static_string_value(right)?);
                Some(value)
            }
            _ => None,
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

/// Returns true when `expr` contains a `super.method()` call or a `super.property`
/// access.  These expressions require `this` to be available as a local in whatever
/// scope they are lowered into (see the `receiver_name == "super"` branch in
/// resolver_expr.rs:1891).
fn expr_contains_super_ref(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::MethodCall { object, args, .. } => {
            (matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "super"))
                || expr_contains_super_ref(object)
                || args.iter().any(expr_contains_super_ref)
        }
        ResolvedExpr::PropertyAccess { object, .. } => {
            (matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "super"))
                || expr_contains_super_ref(object)
        }
        ResolvedExpr::Call { callee, args, .. } => {
            expr_contains_super_ref(callee) || args.iter().any(expr_contains_super_ref)
        }
        ResolvedExpr::Await { expr } => expr_contains_super_ref(expr),
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => {
            expr_contains_super_ref(expr)
        }
        ResolvedExpr::Binary { left, right, .. } => {
            expr_contains_super_ref(left) || expr_contains_super_ref(right)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_super_ref(condition)
                || expr_contains_super_ref(then_expr)
                || expr_contains_super_ref(else_expr)
        }
        ResolvedExpr::Assign { name: _, expr } | ResolvedExpr::LogicalAssign { expr, .. } => {
            expr_contains_super_ref(expr)
        }
        ResolvedExpr::LogicalPropertyAssign {
            object: _,
            key: _,
            expr,
            op: _,
        } => expr_contains_super_ref(expr),
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_contains_super_ref(object) || expr_contains_super_ref(expr)
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_contains_super_ref(key) || expr_contains_super_ref(expr)
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            expr_contains_super_ref(object)
                || expr_contains_super_ref(key)
                || expr_contains_super_ref(expr)
        }
        ResolvedExpr::Array(elements) => elements.iter().any(|element| match element {
            ResolvedArrayElement::Present(expr) => expr_contains_super_ref(expr),
            ResolvedArrayElement::Hole => false,
        }),
        ResolvedExpr::Object(props) => props
            .iter()
            .any(|(_, value)| expr_contains_super_ref(value)),
        ResolvedExpr::ComputedIndex { object, index } => {
            expr_contains_super_ref(object) || expr_contains_super_ref(index)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(expr_contains_super_ref)
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => expr_contains_super_ref(object),
        ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            expr_contains_super_ref(object) || expr_contains_super_ref(index)
        }
        ResolvedExpr::OptionalCall { callee, args, .. } => {
            expr_contains_super_ref(callee) || args.iter().any(expr_contains_super_ref)
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            expr_contains_super_ref(object) || expr_contains_super_ref(value)
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            expr_contains_super_ref(object)
                || expr_contains_super_ref(key)
                || expr_contains_super_ref(value)
        }
        ResolvedExpr::ArrowFn { body, .. } => expr_contains_super_ref(body),
        ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::Ident(_) => false,
    }
}

/// Returns true when any statement in `stmts` contains an expression with a super
/// reference (super.method() or super.property).
fn block_contains_super_ref(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_contains_super_ref)
}

fn stmt_contains_super_ref(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Let(_, expr) | ResolvedStmt::Assign(_, expr) | ResolvedStmt::Expr(expr) => {
            expr_contains_super_ref(expr)
        }
        ResolvedStmt::Return(expr) => expr_contains_super_ref(expr),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            expr_contains_super_ref(condition)
                || block_contains_super_ref(then_body)
                || block_contains_super_ref(else_body)
        }
        ResolvedStmt::While {
            condition, body, ..
        } => expr_contains_super_ref(condition) || block_contains_super_ref(body),
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            init.as_ref().is_some_and(|s| stmt_contains_super_ref(s))
                || condition.as_ref().is_some_and(expr_contains_super_ref)
                || update.as_ref().is_some_and(expr_contains_super_ref)
                || block_contains_super_ref(body)
        }
        ResolvedStmt::ForIn {
            var: _, iter, body, ..
        }
        | ResolvedStmt::ForOf {
            var: _, iter, body, ..
        } => expr_contains_super_ref(iter) || block_contains_super_ref(body),
        ResolvedStmt::Block { statements } => block_contains_super_ref(statements),
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_contains_super_ref(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|b| block_contains_super_ref(b))
                || finally_block
                    .as_ref()
                    .is_some_and(|b| block_contains_super_ref(b))
        }
        ResolvedStmt::Throw(expr) => expr_contains_super_ref(expr),
        ResolvedStmt::Switch { expr, cases } => {
            expr_contains_super_ref(expr)
                || cases.iter().any(|(_, body)| block_contains_super_ref(body))
        }
        ResolvedStmt::DoWhile { body, condition } => {
            block_contains_super_ref(body) || expr_contains_super_ref(condition)
        }
        ResolvedStmt::Labeled { body, .. } => stmt_contains_super_ref(body),
        ResolvedStmt::Export { expr, .. } => expr_contains_super_ref(expr),
        ResolvedStmt::ModuleExportsAssign { expr } => expr_contains_super_ref(expr),
        ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::DestructureLet { .. }
        | ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::Break { label: _ }
        | ResolvedStmt::Continue { label: _ } => false,
    }
}
