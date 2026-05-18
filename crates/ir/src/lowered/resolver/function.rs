use std::collections::{HashMap, HashSet};

use super::{binding_param_default_ref_names, binding_param_names};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedParam, ResolvedStmt};
use crate::lowered::classes::{ObjectAccessorKey, ObjectAccessorProp};
use crate::lowered::facts::ArrowClosure;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::FunctionExprOrigin;

#[derive(Clone, Copy, Default)]
struct NestedFunctionOptions {
    force_receiver: bool,
    is_generator: bool,
    is_async: bool,
    suppress_captures: bool,
}

impl super::Resolver {
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
            if !capture_names.contains(&name) && self.resolve_local(&name).is_ok() {
                capture_names.push(name);
            }
        }
        let mut lowered_body_stmts: Vec<ResolvedStmt> = body_stmts.to_vec();
        lowered_body_stmts.push(ResolvedStmt::Return((*body).clone()));
        let arrow_body_has_dynamic_direct_eval =
            block_contains_dynamic_direct_eval(&lowered_body_stmts);

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
        if arrow_body_has_dynamic_direct_eval {
            let mut dynamic_eval_captures = self
                .ctx
                .facts
                .env_cell_names
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            dynamic_eval_captures.extend(["this".to_owned(), "arguments".to_owned()]);
            dynamic_eval_captures.sort();
            dynamic_eval_captures.dedup();
            for name in dynamic_eval_captures {
                if !capture_names.iter().any(|capture| capture == &name)
                    && (!excluded_set.contains(&name)
                        || active_self_name.is_some_and(|self_name| self_name == name))
                    && self.resolve_local(&name).is_ok()
                {
                    capture_names.push(name);
                }
            }
        }
        let captures = capture_names
            .iter()
            .map(|name| self.resolve_local(name))
            .collect::<Result<Vec<_>, _>>()?;
        let capture_facts = self.capture_facts_for_names(&capture_names);
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

        let func_id = FuncId(self.ctx.functions.next_func_id);
        self.ctx.functions.next_func_id += 1;
        if !capture_names.is_empty() {
            self.ctx
                .functions
                .function_captures
                .insert(func_id, capture_names.clone());
        }
        let dynamic_direct_eval_env_cell_names = collect_dynamic_direct_eval_env_cell_names(
            &lowered_params,
            &lowered_body_stmts,
            false,
            false,
        );
        let nested_env_cell_names = self
            .ctx
            .facts
            .env_cell_names
            .union(&dynamic_direct_eval_env_cell_names)
            .cloned()
            .collect::<HashSet<_>>();
        let lowered = lower_function(
            func_id,
            &lowered_params,
            &lowered_body_stmts,
            false,
            false,
            &self.ctx.symbols.function_ids,
            &self.ctx.symbols.function_signatures,
            &self.ctx.functions.function_captures,
            &self.ctx.functions.function_mutable_captures,
            &self.ctx.functions.class_method_captures,
            &self.ctx.functions.class_method_mutable_captures,
            &nested_env_cell_names,
            &self.ctx.facts.heap_closure_names,
            self.ctx.classes.class_parents.clone(),
            self.ctx.classes.class_private_fields.clone(),
            self.ctx.classes.class_static_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.ctx.classes.current_class.as_deref(),
                in_constructor: false,
                in_static_method: false,
                next_func_id: self.ctx.functions.next_func_id,
                self_closure: active_self_name.map(|name| SelfClosureOptions {
                    name,
                    func_id,
                    capture_names: &capture_names,
                    object_function_props: None,
                }),
                capture_facts,
                recursion_depth: 0,
                // NewTargetArrow: arrow functions inherit the enclosing new.target.
                new_target_class: self.ctx.classes.new_target_class.as_deref(),
                module_url: self.ctx.current_module_url.as_str(),
                strict_context: self.ctx.is_strict_context(),
            },
        )?;
        self.ctx.functions.next_func_id = lowered.next_func_id;
        self.ctx
            .functions
            .generated_functions
            .push(lowered.function);
        self.ctx
            .functions
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
        is_async: bool,
    ) -> Result<LoweredExpr, Diagnostic> {
        if params.iter().any(|param| param.is_rest) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` closure rest parameters are not supported in this slice"
                ),
                span: None,

                phase: None,
            });
        }
        if block_contains_this(body) {
            // If the function has an explicit `this` parameter (TypeScript syntax),
            // the `this` references are valid receiver accesses, not closure captures.
            if params.iter().any(|p| p.name == "this") {
                // Explicit `this` parameter: this is a receiver function, not a closure issue.
            } else {
                // No explicit `this` parameter — unresolved `this` lowers to `undefined`
                // via lower_this_expr (both strict and non-strict). This matches the spec:
                // in strict mode, this is undefined; in non-strict mode the runtime would
                // substitute globalThis, but we use undefined for simplicity.
            }
        }
        // `arguments` in nested functions is handled by needs_arguments in the function
        // signature, which injects `arguments` as a parameter via lower_function.

        let capture_names = self.nested_function_capture_names(name, params, body)?;
        let mutable_captures = capture_names
            .iter()
            .filter(|capture| {
                let names = std::slice::from_ref(*capture);
                block_assigns_any_name(body, names)
                    || params
                        .iter()
                        .filter_map(|param| param.default.as_ref())
                        .any(|default| expr_assigns_any_name(default, names))
            })
            .cloned()
            .collect::<Vec<_>>();
        if mutable_captures
            .iter()
            .any(|capture| !self.ctx.facts.env_cell_names.contains(capture))
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
        let capture_facts = self.capture_facts_for_names(&capture_names);
        let mut lowered_params = params.to_vec();
        lowered_params.extend(capture_names.iter().map(|capture| ResolvedParam {
            name: capture.clone(),
            default: None,
            is_rest: false,
            span: None,
        }));

        let func_id = FuncId(self.ctx.functions.next_func_id);
        self.ctx.functions.next_func_id += 1;
        let self_closure = (!name.is_empty())
            .then_some(SelfClosureOptions {
                name,
                func_id,
                capture_names: &capture_names,
                object_function_props: None,
            })
            .filter(|_| !self.ctx.facts.env_cell_names.contains(name));

        self.ctx.symbols.function_signatures.insert(
            func_id,
            FunctionSignature {
                explicit_params: params.len(),
                needs_receiver: block_contains_super_ref(body),
                is_strict: crate::lowered::program::function_body_is_strict(
                    self.ctx.is_strict_context(),
                    body,
                ),
                needs_arguments: (block_contains_arguments(body)
                    || block_contains_dynamic_direct_eval(body))
                    && !params.iter().any(|param| param.name == "arguments"),
                ..FunctionSignature::default()
            },
        );
        if !capture_names.is_empty() {
            self.ctx
                .functions
                .function_captures
                .insert(func_id, capture_names.clone());
        }
        if !mutable_captures.is_empty() {
            self.ctx
                .functions
                .function_mutable_captures
                .insert(func_id, mutable_captures.clone());
        }
        let function_signatures = self.ctx.symbols.function_signatures.clone();
        let function_captures = self.ctx.functions.function_captures.clone();
        let function_mutable_captures = self.ctx.functions.function_mutable_captures.clone();

        let lowered = lower_function(
            func_id,
            &lowered_params,
            body,
            false,
            is_async,
            &self.ctx.symbols.function_ids,
            &function_signatures,
            &function_captures,
            &function_mutable_captures,
            &self.ctx.functions.class_method_captures,
            &self.ctx.functions.class_method_mutable_captures,
            &self.ctx.facts.env_cell_names,
            &self.ctx.facts.heap_closure_names,
            self.ctx.classes.class_parents.clone(),
            self.ctx.classes.class_private_fields.clone(),
            self.ctx.classes.class_static_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.ctx.classes.current_class.as_deref(),
                in_constructor: false,
                in_static_method: false,
                next_func_id: self.ctx.functions.next_func_id,
                self_closure,
                capture_facts,
                recursion_depth: 0,
                new_target_class: None,
                module_url: self.ctx.current_module_url.as_str(),
                strict_context: self.ctx.is_strict_context(),
            },
        )?;
        self.ctx.functions.next_func_id = lowered.next_func_id;
        self.ctx
            .functions
            .generated_functions
            .push(lowered.function);
        self.ctx
            .functions
            .generated_functions
            .extend(lowered.generated_functions);

        Ok(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: if self.ctx.facts.heap_closure_names.contains(name) {
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
        is_generator: bool,
        origin: FunctionExprOrigin,
    ) -> Result<LoweredExpr, Diagnostic> {
        self.lower_nested_function_with_receiver(
            name,
            params,
            body,
            NestedFunctionOptions {
                is_generator,
                suppress_captures: origin == FunctionExprOrigin::FunctionConstructor,
                ..NestedFunctionOptions::default()
            },
        )
    }

    pub(crate) fn constructable_function_prototype_ref(
        &self,
        name: &str,
    ) -> Option<ClassPrototypeRef> {
        let local = self.resolve_local(name).ok()?;
        if !self
            .ctx
            .facts
            .constructable_function_locals
            .contains(&local)
        {
            return None;
        }
        let closure = self.ctx.facts.arrow_locals.get(&local)?;
        Some(ClassPrototypeRef {
            constructor: closure.func_id,
            parent_constructors: Vec::new(),
        })
    }

    pub(super) fn lower_object_method_function_expr(
        &mut self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
        is_generator: bool,
    ) -> Result<LoweredExpr, Diagnostic> {
        self.lower_nested_function_with_receiver(
            name,
            params,
            body,
            NestedFunctionOptions {
                force_receiver: true,
                is_generator,
                ..NestedFunctionOptions::default()
            },
        )
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
        if !self.ctx.facts.env_cell_names.contains(name) {
            excluded.insert(name.to_owned());
        }
        collect_declared_names_in_stmts(body, &mut excluded);

        let mut captures = Vec::new();
        let excluded_names = excluded.iter().cloned().collect::<Vec<_>>();
        for name in binding_param_default_ref_names(
            params.iter().map(|param| (param.name.as_str(), param.span)),
        )? {
            push_capture(&name, &excluded_names, &mut captures);
        }
        for default in params.iter().filter_map(|param| param.default.as_ref()) {
            collect_expr_captures(default, &excluded, &mut captures);
            collect_nested_function_captures_in_expr(default, &excluded, &mut captures)?;
        }
        collect_stmt_captures(body, &excluded, &mut captures);
        collect_nested_function_captures_in_stmts(body, &excluded, &mut captures)?;
        Ok(captures
            .into_iter()
            .filter(|capture| self.resolve_local(capture).is_ok())
            .collect())
    }

    fn capture_facts_for_names(&self, capture_names: &[String]) -> FunctionCaptureFacts {
        let mut facts = FunctionCaptureFacts::default();
        for name in capture_names {
            let Ok(local_id) = self.resolve_local(name) else {
                continue;
            };
            if let Some(class_name) = self.ctx.classes.local_classes.get(&local_id) {
                facts.local_classes.insert(name.clone(), class_name.clone());
            }
            if let Some(options) = self.ctx.facts.intl_number_format_locals.get(&local_id) {
                facts
                    .intl_number_format_options
                    .insert(name.clone(), options.clone());
            }
            if let Some(props) = self.ctx.classes.object_function_props.get(&local_id)
                && let Some(props) = property_keyed_object_function_props(props)
            {
                facts.object_function_props.insert(name.clone(), props);
            }
            if let Some(props) = self.ctx.classes.object_accessor_props.get(&local_id)
                && let Some(props) = property_keyed_object_accessor_props(props)
            {
                facts.object_accessor_props.insert(name.clone(), props);
            }
        }
        facts
    }

    fn lower_nested_function_with_receiver(
        &mut self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
        options: NestedFunctionOptions,
    ) -> Result<LoweredExpr, Diagnostic> {
        if params.iter().any(|param| param.is_rest) && !options.suppress_captures {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` closure rest parameters are not supported in this slice"
                ),
                span: None,

                phase: None,
            });
        }
        if block_contains_this(body) || block_contains_arguments(body) {
            if block_contains_arguments(body) && !block_contains_this(body) {
                // `arguments` is handled by needs_arguments in the function signature;
                // the function already inserts the signature unconditionally below.
            } else if options.force_receiver {
                // Object literal method shorthand has an implicit receiver.
            } else if block_contains_this(body) && params.iter().any(|p| p.name == "this") {
                // Explicit `this` parameter: this is a receiver function, not a closure issue.
            } else if block_contains_this(body)
                && crate::lowered::program::function_body_is_strict(
                    self.ctx.is_strict_context(),
                    body,
                )
            {
                // Strict functions do not substitute globalThis for direct calls;
                // unresolved `this` lowers to undefined in this resolver scope.
            } else if block_contains_this(body) {
                // No explicit `this` parameter — unresolved `this` lowers to `undefined`
                // via lower_this_expr (both strict and non-strict).
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

        let capture_names = if options.suppress_captures {
            Vec::new()
        } else {
            self.nested_function_capture_names(name, params, body)?
        };
        let mutable_captures = capture_names
            .iter()
            .filter(|capture| {
                let names = std::slice::from_ref(*capture);
                block_assigns_any_name(body, names)
                    || params
                        .iter()
                        .filter_map(|param| param.default.as_ref())
                        .any(|default| expr_assigns_any_name(default, names))
            })
            .cloned()
            .collect::<Vec<_>>();
        if mutable_captures
            .iter()
            .any(|capture| !self.ctx.facts.env_cell_names.contains(capture))
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
        let capture_facts = self.capture_facts_for_names(&capture_names);
        let mut lowered_params = params.to_vec();
        lowered_params.extend(capture_names.iter().map(|capture| ResolvedParam {
            name: capture.clone(),
            default: None,
            is_rest: false,
            span: None,
        }));

        let func_id = FuncId(self.ctx.functions.next_func_id);
        self.ctx.functions.next_func_id += 1;
        let self_closure = (!options.suppress_captures && !name.is_empty())
            .then_some(SelfClosureOptions {
                name,
                func_id,
                capture_names: &capture_names,
                object_function_props: None,
            })
            .filter(|_| !self.ctx.facts.env_cell_names.contains(name));

        let needs_receiver = options.force_receiver || block_contains_super_ref(body);
        let dynamic_direct_eval_env_cell_names =
            collect_dynamic_direct_eval_env_cell_names(&lowered_params, body, true, needs_receiver);
        let nested_env_cell_names = self
            .ctx
            .facts
            .env_cell_names
            .union(&dynamic_direct_eval_env_cell_names)
            .cloned()
            .collect::<HashSet<_>>();
        self.ctx.symbols.function_signatures.insert(
            func_id,
            FunctionSignature {
                explicit_params: params.len(),
                needs_receiver,
                is_strict: crate::lowered::program::function_body_is_strict(
                    self.ctx.is_strict_context(),
                    body,
                ),
                needs_arguments: (block_contains_arguments(body)
                    || block_contains_dynamic_direct_eval(body))
                    && !params.iter().any(|param| param.name == "arguments"),
                has_rest: params.iter().any(|param| param.is_rest),
                metadata_length: Some(function_length_metadata(params)),
                ..FunctionSignature::default()
            },
        );
        if let Some(value) = crate::lowered::program::body_returns_static_string(body) {
            self.ctx
                .functions
                .static_string_returns
                .insert(func_id, value);
        }
        if !capture_names.is_empty() {
            self.ctx
                .functions
                .function_captures
                .insert(func_id, capture_names.clone());
        }
        if !mutable_captures.is_empty() {
            self.ctx
                .functions
                .function_mutable_captures
                .insert(func_id, mutable_captures);
        }
        let function_signatures = self.ctx.symbols.function_signatures.clone();
        let function_captures = self.ctx.functions.function_captures.clone();
        let function_mutable_captures = self.ctx.functions.function_mutable_captures.clone();

        let lowered = lower_function(
            func_id,
            &lowered_params,
            body,
            options.is_generator,
            options.is_async,
            &self.ctx.symbols.function_ids,
            &function_signatures,
            &function_captures,
            &function_mutable_captures,
            &self.ctx.functions.class_method_captures,
            &self.ctx.functions.class_method_mutable_captures,
            &nested_env_cell_names,
            &self.ctx.facts.heap_closure_names,
            self.ctx.classes.class_parents.clone(),
            self.ctx.classes.class_private_fields.clone(),
            self.ctx.classes.class_static_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.ctx.classes.current_class.as_deref(),
                in_constructor: false,
                in_static_method: false,
                next_func_id: self.ctx.functions.next_func_id,
                self_closure,
                capture_facts,
                recursion_depth: 0,
                new_target_class: None,
                module_url: self.ctx.current_module_url.as_str(),
                strict_context: self.ctx.is_strict_context(),
            },
        )?;
        self.ctx.functions.next_func_id = lowered.next_func_id;
        self.ctx
            .functions
            .generated_functions
            .push(lowered.function);
        self.ctx
            .functions
            .generated_functions
            .extend(lowered.generated_functions);

        Ok(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: if !capture_names.is_empty()
                || self.ctx.facts.heap_closure_names.contains(name)
            {
                ClosureRepresentation::HeapObject
            } else {
                ClosureRepresentation::DirectLocalToken
            },

            span: Span::generated("arrow_fn"),
        })
    }

    pub(crate) fn declare_local(&mut self, name: &str) -> Result<LocalId, Diagnostic> {
        if self.ctx.is_strict_context() && matches!(name, "eval" | "arguments") {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-450: {:?} strict mode forbids binding local `{name}`",
                    crate::lowered::ctx::StrictModeCheck::StrictEval
                ),
                span: None,
                phase: None,
            });
        }
        let scope = self
            .ctx
            .symbols
            .scopes
            .last_mut()
            .expect("scope must exist");
        if let Some(&existing) = scope.get(name) {
            return Ok(existing);
        }
        let local_id = LocalId(self.ctx.symbols.next_local_id);
        self.ctx.symbols.next_local_id += 1;
        self.ctx.symbols.locals.push(local_id);
        scope.insert(name.to_owned(), local_id);
        Ok(local_id)
    }

    pub(crate) fn declare_self_closure(
        &mut self,
        name: &str,
        func_id: FuncId,
        capture_names: &[String],
        object_function_props: Option<&HashMap<ObjectAccessorKey, FuncId>>,
    ) -> Result<(), Diagnostic> {
        let local_id = self.declare_local(name)?;
        let captures = capture_names
            .iter()
            .map(|capture| self.resolve_local(capture))
            .collect::<Result<Vec<_>, _>>()?;
        self.ctx
            .facts
            .arrow_locals
            .insert(local_id, ArrowClosure { func_id, captures });
        if let Some(props) = object_function_props {
            self.ctx
                .classes
                .object_function_props
                .insert(local_id, props.clone());
        }
        Ok(())
    }

    pub(super) fn alloc_temp(&mut self) -> LocalId {
        let id = LocalId(self.ctx.symbols.next_local_id);
        self.ctx.symbols.next_local_id += 1;
        self.ctx.symbols.locals.push(id);
        id
    }

    pub(crate) fn resolve_local(&self, name: &str) -> Result<LocalId, Diagnostic> {
        self.ctx
            .symbols
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
        self.ctx
            .symbols
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

    pub(super) fn update_heap_closure_local(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
        lowered: &LoweredExpr,
    ) {
        if crate::lowered::resolver::expr::facts::expr_is_known_heap_closure(&self.ctx, expr)
            || matches!(
                lowered,
                LoweredExpr::ArrowFn {
                    representation: ClosureRepresentation::HeapObject,
                    ..
                }
            )
        {
            self.ctx.facts.heap_closure_locals.insert(local_id);
        } else {
            self.ctx.facts.heap_closure_locals.remove(&local_id);
        }
    }
}

fn function_length_metadata(params: &[ResolvedParam]) -> usize {
    params
        .iter()
        .take_while(|param| param.default.is_none() && !param.is_rest)
        .count()
}

fn property_keyed_object_function_props(
    props: &HashMap<ObjectAccessorKey, FuncId>,
) -> Option<HashMap<ObjectAccessorKey, FuncId>> {
    let props = props
        .iter()
        .filter_map(|(key, func_id)| match key {
            ObjectAccessorKey::Property(property) => {
                Some((ObjectAccessorKey::Property(property.clone()), *func_id))
            }
            ObjectAccessorKey::SymbolLocal(_) => None,
        })
        .collect::<HashMap<_, _>>();
    (!props.is_empty()).then_some(props)
}

fn property_keyed_object_accessor_props(
    props: &HashMap<ObjectAccessorKey, ObjectAccessorProp>,
) -> Option<HashMap<ObjectAccessorKey, ObjectAccessorProp>> {
    let props = props
        .iter()
        .filter_map(|(key, accessor)| match key {
            ObjectAccessorKey::Property(property) => {
                Some((ObjectAccessorKey::Property(property.clone()), *accessor))
            }
            ObjectAccessorKey::SymbolLocal(_) => None,
        })
        .collect::<HashMap<_, _>>();
    (!props.is_empty()).then_some(props)
}

/// Returns true when `expr` contains a `super.method()` call or a `super.property`
/// access.  These expressions require `this` to be available as a local in whatever
/// scope they are lowered into (see the `receiver_name == "super"` branch in
/// resolver_expr.rs:1891).
pub(super) fn expr_contains_super_ref(expr: &ResolvedExpr) -> bool {
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
        ResolvedExpr::Yield { expr, .. } => expr.as_deref().is_some_and(expr_contains_super_ref),
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
        ResolvedExpr::Object(props) => props.iter().any(|prop| {
            prop.computed_key().is_some_and(expr_contains_super_ref)
                || expr_contains_super_ref(prop.value())
        }),
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
        ResolvedExpr::Sequence(exprs) => exprs.iter().any(expr_contains_super_ref),
        ResolvedExpr::EvalCompletion(steps) => steps
            .iter()
            .filter_map(|step| step.expr())
            .any(expr_contains_super_ref),
        ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::Eval { .. } => false,
        ResolvedExpr::ImportMeta { .. } | ResolvedExpr::Ident(_) => false,
    }
}

/// Returns true when any statement in `stmts` contains an expression with a super
/// reference (super.method() or super.property).
pub(super) fn block_contains_super_ref(stmts: &[ResolvedStmt]) -> bool {
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
        }
        | ResolvedStmt::ForAwaitOf {
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
