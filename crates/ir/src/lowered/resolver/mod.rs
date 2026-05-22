mod array;
mod call;
mod class;
mod expr;
mod function;
mod module;
mod object;
pub(crate) mod string;

use std::collections::{HashMap, HashSet};

use crate::binding_pattern::{BindingDefault, parse_binding_pattern};
use crate::builtin_resolved::{
    ClassMethod, ClassMethodKind, EvalHostPolicy, EvalSource, FunctionConstructorHostPolicy,
    ResolvedArrayElement, ResolvedExpr, ResolvedParam, ResolvedStmt,
};
use crate::lowered::ctx::LoweringCtx;
use crate::lowered::facts::{
    ArrowClosure, BoundConstructor, BoundFunction, FunctionMethodBinding, FunctionMethodKind,
    GeneratorMethodIteratorBinding, HostExternalKind,
};
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::{BinaryOp, SYMBOL_ITERATOR_OBJECT_KEY, TypeRef, UnaryOp};

/// New Resolver with ctx: LoweringCtx.
/// All mutable state is owned by ctx. Borrowed function maps are cloned into ctx on construction.
pub(super) struct Resolver {
    pub(crate) ctx: LoweringCtx,
}

pub(super) struct EvalClassDeclParts<'a> {
    pub(super) name: &'a str,
    pub(super) extends: &'a Option<String>,
    pub(super) constructor: &'a Option<(Vec<ResolvedParam>, Vec<ResolvedStmt>)>,
    pub(super) methods: &'a [ClassMethod],
    pub(super) private_fields: &'a [String],
    pub(super) static_private_fields: &'a [(String, ResolvedExpr, Span)],
    pub(super) static_blocks: &'a [(Span, Vec<ResolvedStmt>)],
}

impl Resolver {
    fn generator_method_iterator_binding_for_expr(
        &mut self,
        expr: &ResolvedExpr,
    ) -> Option<GeneratorMethodIteratorBinding> {
        let ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } = expr
        else {
            return None;
        };
        let ResolvedExpr::Ident(receiver_name) = object.as_ref() else {
            return None;
        };
        let receiver_local = self.resolve_local(receiver_name).ok()?;
        let func_id = self
            .ctx
            .classes
            .object_function_props
            .get(&receiver_local)
            .and_then(|props| {
                props.get(&crate::lowered::classes::ObjectAccessorKey::Property(
                    method.clone(),
                ))
            })
            .copied()?;
        self.ctx
            .functions
            .generated_functions
            .iter()
            .any(|function| function.id == func_id && function.is_generator)
            .then(|| {
                let state_local = self.alloc_temp();
                GeneratorMethodIteratorBinding {
                    func_id,
                    receiver_local,
                    args: args.to_vec(),
                    state_local,
                }
            })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        function_ids: &HashMap<String, FuncId>,
        function_signatures: &HashMap<FuncId, FunctionSignature>,
        function_sources: HashMap<FuncId, String>,
        function_captures: &HashMap<FuncId, Vec<String>>,
        function_mutable_captures: &HashMap<FuncId, Vec<String>>,
        class_method_captures: &HashMap<FuncId, Vec<String>>,
        class_method_mutable_captures: &HashMap<FuncId, Vec<String>>,
        env_cell_names: &HashSet<String>,
        heap_closure_names: &HashSet<String>,
        class_parents: HashMap<String, Option<String>>,
        class_private_fields: ClassPrivateFieldSlots,
        class_static_private_fields: ClassStaticPrivateFields,
        generator_function_names: HashSet<String>,
        next_func_id: usize,
        current_module_url: &str,
        is_strict_context: bool,
        type_aliases: HashMap<String, ts2wasm_syntax::TypeRef>,
        interface_definitions: HashMap<String, Vec<(String, ts2wasm_syntax::TypeRef)>>,
    ) -> Self {
        let (class_constructor_ids, class_method_ids, class_static_method_ids) =
            class_maps(function_ids);
        Self {
            ctx: LoweringCtx::with_resolver_state(
                function_ids,
                function_signatures,
                function_sources,
                function_captures,
                function_mutable_captures,
                class_method_captures,
                class_method_mutable_captures,
                env_cell_names,
                heap_closure_names,
                generator_function_names,
                class_constructor_ids,
                class_method_ids,
                class_static_method_ids,
                class_parents,
                class_private_fields,
                class_static_private_fields,
                next_func_id,
                current_module_url,
                is_strict_context,
                type_aliases,
                interface_definitions,
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn with_params(
        function_ids: &HashMap<String, FuncId>,
        function_signatures: &HashMap<FuncId, FunctionSignature>,
        function_sources: HashMap<FuncId, String>,
        function_captures: &HashMap<FuncId, Vec<String>>,
        function_mutable_captures: &HashMap<FuncId, Vec<String>>,
        class_method_captures: &HashMap<FuncId, Vec<String>>,
        class_method_mutable_captures: &HashMap<FuncId, Vec<String>>,
        env_cell_names: &HashSet<String>,
        heap_closure_names: &HashSet<String>,
        params: &[String],
        synthetic_arguments_param_index: Option<usize>,
        class_parents: HashMap<String, Option<String>>,
        class_private_fields: ClassPrivateFieldSlots,
        class_static_private_fields: ClassStaticPrivateFields,
        current_class: Option<&str>,
        in_constructor: bool,
        new_target_class: Option<&str>,
        next_func_id: usize,
        current_module_url: &str,
        is_strict_context: bool,
        type_aliases: HashMap<String, ts2wasm_syntax::TypeRef>,
        interface_definitions: HashMap<String, Vec<(String, ts2wasm_syntax::TypeRef)>>,
    ) -> Result<(Self, Vec<LocalId>), Diagnostic> {
        let (class_constructor_ids, class_method_ids, class_static_method_ids) =
            class_maps(function_ids);
        let mut resolver = Self {
            ctx: LoweringCtx::with_resolver_state(
                function_ids,
                function_signatures,
                function_sources,
                function_captures,
                function_mutable_captures,
                class_method_captures,
                class_method_mutable_captures,
                env_cell_names,
                heap_closure_names,
                HashSet::new(),
                class_constructor_ids,
                class_method_ids,
                class_static_method_ids,
                class_parents,
                class_private_fields,
                class_static_private_fields,
                next_func_id,
                current_module_url,
                is_strict_context,
                type_aliases,
                interface_definitions,
            ),
        };
        resolver
            .ctx
            .set_class_context(current_class, in_constructor, new_target_class);

        let mut param_ids = Vec::new();
        let mut seen_params = HashMap::new();

        for (index, param) in params.iter().enumerate() {
            let clean_name = param.strip_prefix("...").unwrap_or(param.as_str());
            let is_synthetic_arguments_param =
                synthetic_arguments_param_index == Some(index) && clean_name == "arguments";
            if resolver.ctx.is_strict_context()
                && matches!(clean_name, "eval" | "arguments")
                && !is_synthetic_arguments_param
            {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-450: {:?} strict mode forbids binding parameter `{clean_name}`",
                        crate::lowered::ctx::StrictModeCheck::StrictEval
                    ),
                    span: None,
                    phase: None,
                });
            }
            if seen_params.contains_key(clean_name) {
                // Non-strict mode allows duplicate parameter names (per ES spec).
                // In non-strict mode, the second declaration shadows the first.
                if resolver.ctx.is_strict_context() {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateParameter,
                        message: format!("duplicate parameter name: `{clean_name}`"),
                        span: None,
                        phase: None,
                    });
                }
                // Skip seen_params re-insert so the second declare_parameter creates a
                // distinct local that shadows the first during name resolution.
                seen_params.remove(clean_name);
            }
            seen_params.insert(clean_name.to_owned(), ());
            let local_id = resolver.ctx.declare_parameter(clean_name);
            param_ids.push(local_id);
        }

        Ok((resolver, param_ids))
    }

    pub(crate) fn lower_block(
        &mut self,
        statements: &[ResolvedStmt],
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let mut lowered = Vec::with_capacity(statements.len());
        for statement in statements {
            lowered.push(self.lower_stmt(statement)?);
        }
        Ok(lowered)
    }

    fn lower_nested_block(
        &mut self,
        statements: &[ResolvedStmt],
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        self.ctx.symbols.scopes.push(HashMap::new());
        let lowered = self.lower_block(statements);
        self.ctx.symbols.scopes.pop();
        lowered
    }

    /// Lower `yield* iterable` (with optional target local for the return value).
    ///
    /// Produces lowered IR equivalent to:
    /// ```text
    /// let iter_fn = iterable["@@iterator"];
    /// let iter = iter_fn();
    /// loop {
    ///   let next_fn = iter["next"];
    ///   let result = next_fn();
    ///   let done = result["done"];
    ///   if (!done) { yield result["value"]; }
    ///   if (done) break;
    /// }
    /// // if target_local is Some(id): let id = result["value"];
    /// ```
    fn lower_yield_star_stmt(
        &mut self,
        iterable_expr: &ResolvedExpr,
        target_local: Option<LocalId>,
    ) -> Result<LoweredStmt, Diagnostic> {
        let span = Span::generated("yield_star");
        let sentinel_key = SYMBOL_ITERATOR_OBJECT_KEY.to_owned();
        let iterable = self.lower_expr(iterable_expr)?;
        let iter_fn = self.alloc_temp();
        let iterator = self.alloc_temp();
        let next_fn = self.alloc_temp();
        let result = self.alloc_temp();
        let done_val = self.alloc_temp();

        // let iter_fn = iterable["@@iterator"]
        // let iterator = iter_fn()
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
                    intrinsic: RuntimeFn::HeapClosureCall,
                    args: vec![LoweredExpr::Local(iter_fn, Span::generated("local"))],
                    span,
                },
                span,
            ),
        ];

        // Loop body
        let mut body = Vec::new();

        // let next_fn = iterator["next"]
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

        // let result = next_fn()
        body.push(LoweredStmt::Let(
            result,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::HeapClosureCall,
                args: vec![LoweredExpr::Local(next_fn, Span::generated("local"))],
                span,
            },
            span,
        ));

        // let done_val = result["done"]
        body.push(LoweredStmt::Let(
            done_val,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(result, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "done".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        ));

        // if (!done_val) { yield result["value"]; }
        let if_body = vec![LoweredStmt::Yield(
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(result, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "value".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        )];
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

        // do { ... } while (!done_val)
        stmts.push(LoweredStmt::DoWhile {
            body,
            condition: LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Local(done_val, Span::generated("local"))),
                span,
            },
            span,
        });

        // If a target local was provided, assign the inner iterator's return value.
        if let Some(target) = target_local {
            stmts.push(LoweredStmt::Let(
                target,
                LoweredExpr::PropertyGetDynamic {
                    obj: Box::new(LoweredExpr::Local(result, Span::generated("local"))),
                    key: Box::new(LoweredExpr::String(
                        "value".to_owned(),
                        Span::generated("str"),
                    )),
                    span,
                },
                span,
            ));
        }

        Ok(LoweredStmt::Block(stmts, span))
    }

    fn bound_function_for_expr(
        &self,
        expr: &ResolvedExpr,
    ) -> Result<Option<BoundFunction>, Diagnostic> {
        let ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } = expr
        else {
            return Ok(None);
        };
        if method == "call"
            && let Some("bind") = function_prototype_method_name(object)
            && let Some(ResolvedExpr::Ident(func_name)) = args.first()
        {
            let Ok(func_id) = self.resolve_func(func_name) else {
                return Ok(None);
            };
            return Ok(Some(BoundFunction {
                func_id,
                receiver: args.get(1).cloned().unwrap_or(ResolvedExpr::Undefined),
                bound_args: args.iter().skip(2).cloned().collect(),
            }));
        }
        if method != "bind" {
            return Ok(None);
        }
        let ResolvedExpr::Ident(func_name) = object.as_ref() else {
            return Ok(None);
        };
        let Ok(func_id) = self.resolve_func(func_name) else {
            return Ok(None);
        };
        Ok(Some(BoundFunction {
            func_id,
            receiver: args.first().cloned().unwrap_or(ResolvedExpr::Undefined),
            bound_args: args.iter().skip(1).cloned().collect(),
        }))
    }

    fn function_method_binding_for_expr(
        &self,
        expr: &ResolvedExpr,
    ) -> Result<Option<FunctionMethodBinding>, Diagnostic> {
        let ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } = expr
        else {
            return Ok(None);
        };
        if method != "bind" {
            return Ok(None);
        }
        let Some(kind) = function_prototype_method_name(object).and_then(|name| match name {
            "call" => Some(FunctionMethodKind::Call),
            "apply" => Some(FunctionMethodKind::Apply),
            _ => None,
        }) else {
            return Ok(None);
        };
        let Some(ResolvedExpr::Ident(func_name)) = args.first() else {
            return Ok(None);
        };
        let Ok(func_id) = self.resolve_func(func_name) else {
            return Ok(None);
        };
        Ok(Some(FunctionMethodBinding { func_id, kind }))
    }

    fn bound_constructor_for_expr(&self, expr: &ResolvedExpr) -> Option<BoundConstructor> {
        let ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } = expr
        else {
            return None;
        };
        if method != "bind" {
            return None;
        }
        let ResolvedExpr::Ident(class_name) = object.as_ref() else {
            return None;
        };
        if !self
            .ctx
            .classes
            .class_constructor_ids
            .contains_key(class_name)
        {
            return None;
        }
        Some(BoundConstructor {
            class_name: class_name.clone(),
            bound_args: args.iter().skip(1).cloned().collect(),
        })
    }

    fn lower_direct_iife_stmt(
        &mut self,
        expr: &ResolvedExpr,
    ) -> Result<Option<LoweredStmt>, Diagnostic> {
        let ResolvedExpr::Call { callee, args, span } = expr else {
            return Ok(None);
        };
        let ResolvedExpr::FunctionExpr { params, body, .. } = callee.as_ref() else {
            return Ok(None);
        };
        if !args.is_empty() || !params.is_empty() {
            return Ok(None);
        }
        if !direct_iife_body_has_static_eval_block_function_binding(body) {
            return Ok(None);
        }
        if direct_iife_body_has_unsupported_return(body) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-302: direct eval IIFE lowering does not support function returns"
                    .to_owned(),
                span: Some(*span),

                phase: None,
            });
        }
        if block_contains_this(body) || block_contains_arguments(body) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-302: direct eval IIFE lowering does not support `this` or `arguments`"
                        .to_owned(),
                span: Some(*span),

                phase: None,
            });
        }

        self.ctx.symbols.scopes.push(HashMap::new());
        let lowered = self.lower_block(body);
        self.ctx.symbols.scopes.pop();
        lowered.map(|statements| Some(LoweredStmt::Block(statements, Span::generated("block"))))
    }

    pub(crate) fn lower_stmt(&mut self, stmt: &ResolvedStmt) -> Result<LoweredStmt, Diagnostic> {
        match stmt {
            ResolvedStmt::AmbientValue(name) => {
                self.declare_local(name)?;
                Ok(LoweredStmt::Expr(
                    LoweredExpr::Undefined(Span::generated("undef")),
                    Span::generated("expr_stmt"),
                ))
            }
            ResolvedStmt::Expr(ResolvedExpr::Yield { expr, delegate }) => {
                if *delegate {
                    let inner = expr.as_ref().ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "yield* requires an expression".to_owned(),
                        span: Some(Span::generated("yield_star")),
                        phase: None,
                    })?;
                    return self.lower_yield_star_stmt(inner, None);
                }
                Ok(LoweredStmt::Yield(
                    expr.as_ref()
                        .map(|expr| self.lower_expr(expr))
                        .transpose()?
                        .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("yield"))),
                    Span::generated("yield_stmt"),
                ))
            }
            ResolvedStmt::DestructureLet { pattern, expr } => {
                let value_local = self.alloc_temp();
                let mut statements = if let ResolvedExpr::Yield {
                    expr: Some(inner),
                    delegate: true,
                } = expr
                {
                    // `let [a, b] = yield* iterable;` — yield* loop first,
                    // then destructure the return value.
                    match self.lower_yield_star_stmt(inner, Some(value_local))? {
                        LoweredStmt::Block(s, _) => s,
                        other => vec![other],
                    }
                } else {
                    vec![LoweredStmt::Let(
                        value_local,
                        self.lower_expr(expr)?,
                        Span::generated("let_stmt"),
                    )]
                };
                statements.extend(self.lower_binding_pattern_declarations(
                    pattern,
                    LoweredExpr::Local(value_local, Span::generated("local")),
                    Some(expr),
                )?);
                Ok(LoweredStmt::Block(statements, Span::generated("block")))
            }
            ResolvedStmt::DestructureAssign { pattern, expr } => {
                let value_local = self.alloc_temp();
                let mut statements = vec![LoweredStmt::Let(
                    value_local,
                    self.lower_expr(expr)?,
                    Span::generated("let_stmt"),
                )];
                statements.extend(self.lower_binding_pattern_assignments(
                    pattern,
                    LoweredExpr::Local(value_local, Span::generated("local")),
                    Some(expr),
                )?);
                Ok(LoweredStmt::Block(statements, Span::generated("block")))
            }
            ResolvedStmt::Let(name, expr) => {
                let local_id = self.declare_local(name)?;
                // Handle `let x = yield* iterable;` — delegate to yield* lowering
                // with a target local to receive the inner iterator's return value.
                if let ResolvedExpr::Yield {
                    expr: Some(inner),
                    delegate: true,
                } = expr
                {
                    return self.lower_yield_star_stmt(inner, Some(local_id));
                }
                // Infer class before lowering so closures inside the initializer
                // can resolve the class of this local (e.g. `new Howl(...)` with a
                // callback that calls `instance.once(...)`).
                let expr_class = self.infer_class_for_expr(expr);
                if let Some(class_name) = &expr_class {
                    self.ctx
                        .classes
                        .local_classes
                        .insert(local_id, class_name.clone());
                }
                let mut function_props = self.function_props_for_object_expr(expr);
                let bound_function = self.bound_function_for_expr(expr)?;
                let function_method = self.function_method_binding_for_expr(expr)?;
                let bound_constructor = self.bound_constructor_for_expr(expr);
                let generator_method_binding =
                    self.generator_method_iterator_binding_for_expr(expr);
                let generator_state_local =
                    crate::lowered::resolver::expr::facts::resolved_generator_function_call_name(
                        &self.ctx, expr,
                    )
                    .filter(|func_name| {
                        self.ctx
                            .facts
                            .generator_function_steps
                            .contains_key(func_name)
                            || self
                                .ctx
                                .facts
                                .generator_function_object_resume_plans
                                .contains_key(func_name)
                    })
                    .map(|_| self.alloc_temp());
                let lowered = if bound_function.is_some()
                    || function_method.is_some()
                    || bound_constructor.is_some()
                {
                    LoweredExpr::Undefined(Span::generated("undef"))
                } else if let ResolvedExpr::ArrowFn {
                    params,
                    body,
                    body_stmts,
                    ..
                } = expr
                {
                    self.lower_arrow_fn_with_self(params, body, body_stmts, Some(name))?
                } else if generator_method_binding.is_some() {
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::GeneratorYield,
                        args: vec![LoweredExpr::ArrayNew {
                            elements: Vec::new(),
                            span: Span::generated("array"),
                        }],
                        span: Span::generated("runtime_call"),
                    }
                } else {
                    self.lower_expr(expr)?
                };
                let arrow_closure = if let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = &lowered
                {
                    Some(ArrowClosure {
                        func_id: *func_id,
                        captures: captures.clone(),
                    })
                } else {
                    None
                };
                let lowered = if self.ctx.facts.env_cell_names.contains(name) {
                    self.ctx.facts.env_cell_locals.insert(local_id);
                    self.ctx.facts.initialized_env_cell_locals.insert(local_id);
                    LoweredExpr::EnvCellNew(Box::new(lowered), Span::generated("env_cell_new"))
                } else {
                    lowered
                };
                let accessor_props = self.accessor_props_for_lowered_object_expr(&lowered);
                if let Some(lowered_props) = self.function_props_for_lowered_object_expr(&lowered) {
                    function_props
                        .get_or_insert_with(HashMap::new)
                        .extend(lowered_props);
                }
                if let Some(closure) = arrow_closure {
                    self.ctx.facts.arrow_locals.insert(local_id, closure);
                    if let Some(metadata_name) =
                        static_function_metadata_name_for_expr(&self.ctx, expr)
                    {
                        self.ctx
                            .facts
                            .function_metadata_name_locals
                            .insert(local_id, metadata_name);
                    } else {
                        self.ctx
                            .facts
                            .function_metadata_name_locals
                            .remove(&local_id);
                    }
                    if static_function_constructable_for_expr(expr) {
                        self.ctx
                            .facts
                            .constructable_function_locals
                            .insert(local_id);
                    } else {
                        self.ctx
                            .facts
                            .constructable_function_locals
                            .remove(&local_id);
                    }
                } else {
                    self.ctx.facts.arrow_locals.remove(&local_id);
                    self.ctx
                        .facts
                        .function_metadata_name_locals
                        .remove(&local_id);
                    self.ctx
                        .facts
                        .constructable_function_locals
                        .remove(&local_id);
                }
                if let Some(bound_function) = bound_function {
                    self.ctx
                        .facts
                        .bound_function_locals
                        .insert(local_id, bound_function);
                } else {
                    self.ctx.facts.bound_function_locals.remove(&local_id);
                }
                if let Some(function_method) = function_method {
                    self.ctx
                        .facts
                        .function_method_locals
                        .insert(local_id, function_method);
                } else {
                    self.ctx.facts.function_method_locals.remove(&local_id);
                }
                if let Some(bound_constructor) = bound_constructor {
                    self.ctx
                        .facts
                        .bound_constructor_locals
                        .insert(local_id, bound_constructor);
                } else {
                    self.ctx.facts.bound_constructor_locals.remove(&local_id);
                }
                self.update_heap_closure_local(local_id, expr, &lowered);
                if self.ctx.facts.heap_closure_names.contains(name) {
                    self.ctx.facts.heap_closure_locals.insert(local_id);
                }
                crate::lowered::resolver::expr::facts::update_nullish_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_host_function_handle_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_host_external_object_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_bigint_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_control_flow_bigint_assignment(
                    &mut self.ctx,
                    local_id,
                );
                crate::lowered::resolver::expr::facts::update_array_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_symbol_iterator_object_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_array_iterator_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_generator_iterator_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                    generator_state_local,
                );
                if let Some(binding) = generator_method_binding {
                    self.ctx.facts.generator_iterator_locals.insert(local_id);
                    self.ctx
                        .facts
                        .generator_method_iterator_bindings
                        .insert(local_id, binding);
                } else {
                    self.ctx
                        .facts
                        .generator_method_iterator_bindings
                        .remove(&local_id);
                }
                crate::lowered::resolver::expr::facts::update_proxy_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_static_object_literal_local_on_let(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_static_function_array_like_local_on_let(&mut self.ctx, local_id, expr);
                crate::lowered::resolver::string::update_string_literal_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::string::update_number_literal_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::string::update_symbol_value_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_native_set_add_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_invalid_date_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                if let Some(options) = self.intl_number_format_options_for_expr(expr) {
                    self.ctx
                        .facts
                        .intl_number_format_locals
                        .insert(local_id, options);
                } else {
                    self.ctx.facts.intl_number_format_locals.remove(&local_id);
                }
                if let Some(options) = self.intl_date_time_format_options_for_expr(expr) {
                    self.ctx
                        .facts
                        .intl_date_time_format_locals
                        .insert(local_id, options);
                } else {
                    self.ctx
                        .facts
                        .intl_date_time_format_locals
                        .remove(&local_id);
                }
                if let Some(props) = function_props {
                    self.ctx
                        .classes
                        .object_function_props
                        .insert(local_id, props);
                } else {
                    self.ctx.classes.object_function_props.remove(&local_id);
                }
                if let Some(props) = accessor_props {
                    self.ctx
                        .classes
                        .object_accessor_props
                        .insert(local_id, props);
                } else {
                    self.ctx.classes.object_accessor_props.remove(&local_id);
                }
                crate::lowered::resolver::string::update_regexp_literal_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                let local_stmt = LoweredStmt::Let(local_id, lowered, Span::generated("let_stmt"));
                let state_local = generator_state_local.or_else(|| {
                    self.ctx
                        .facts
                        .generator_method_iterator_bindings
                        .get(&local_id)
                        .map(|binding| binding.state_local)
                });
                if let Some(state_local) = state_local {
                    Ok(LoweredStmt::Block(
                        vec![
                            local_stmt,
                            LoweredStmt::Let(
                                state_local,
                                LoweredExpr::Number(0, Span::generated("num")),
                                Span::generated("let_stmt"),
                            ),
                        ],
                        Span::generated("block"),
                    ))
                } else {
                    Ok(local_stmt)
                }
            }
            ResolvedStmt::Assign(name, expr) => {
                let local_id = self.resolve_local(name)?;
                // Handle `x = yield* iterable;` — delegate to yield* lowering,
                // then assign the inner iterator's return value to x.
                if let ResolvedExpr::Yield {
                    expr: Some(inner),
                    delegate: true,
                } = expr
                {
                    let result_local = self.alloc_temp();
                    let mut stmts = match self.lower_yield_star_stmt(inner, Some(result_local))? {
                        LoweredStmt::Block(s, _) => s,
                        other => vec![other],
                    };
                    stmts.push(LoweredStmt::Assign(
                        local_id,
                        LoweredExpr::Local(result_local, Span::generated("local")),
                        Span::generated("assign_stmt"),
                    ));
                    return Ok(LoweredStmt::Block(stmts, Span::generated("block")));
                }
                crate::lowered::resolver::expr::facts::invalidate_static_object_literal_local(
                    &mut self.ctx,
                    local_id,
                );
                // Infer class before lowering so closures inside the RHS
                // can resolve the class of this local.
                let expr_class = self.infer_class_for_expr(expr);
                if let Some(class_name) = &expr_class {
                    self.ctx
                        .classes
                        .local_classes
                        .insert(local_id, class_name.clone());
                } else {
                    self.ctx.classes.local_classes.remove(&local_id);
                }
                let mut function_props = self.function_props_for_object_expr(expr);
                let bound_function = self.bound_function_for_expr(expr)?;
                let function_method = self.function_method_binding_for_expr(expr)?;
                let bound_constructor = self.bound_constructor_for_expr(expr);
                let generator_state_local =
                    crate::lowered::resolver::expr::facts::resolved_generator_function_call_name(
                        &self.ctx, expr,
                    )
                    .filter(|func_name| {
                        self.ctx
                            .facts
                            .generator_function_steps
                            .contains_key(func_name)
                            || self
                                .ctx
                                .facts
                                .generator_function_object_resume_plans
                                .contains_key(func_name)
                    })
                    .map(|_| self.alloc_temp());
                let lowered = if bound_function.is_some()
                    || function_method.is_some()
                    || bound_constructor.is_some()
                {
                    LoweredExpr::Undefined(Span::generated("undef"))
                } else {
                    self.lower_expr(expr)?
                };
                let accessor_props = self.accessor_props_for_lowered_object_expr(&lowered);
                if let Some(lowered_props) = self.function_props_for_lowered_object_expr(&lowered) {
                    function_props
                        .get_or_insert_with(HashMap::new)
                        .extend(lowered_props);
                }
                if let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = &lowered
                {
                    self.ctx.facts.arrow_locals.insert(
                        local_id,
                        ArrowClosure {
                            func_id: *func_id,
                            captures: captures.clone(),
                        },
                    );
                    if let Some(metadata_name) =
                        static_function_metadata_name_for_expr(&self.ctx, expr)
                    {
                        self.ctx
                            .facts
                            .function_metadata_name_locals
                            .insert(local_id, metadata_name);
                    } else {
                        self.ctx
                            .facts
                            .function_metadata_name_locals
                            .remove(&local_id);
                    }
                    if static_function_constructable_for_expr(expr) {
                        self.ctx
                            .facts
                            .constructable_function_locals
                            .insert(local_id);
                    } else {
                        self.ctx
                            .facts
                            .constructable_function_locals
                            .remove(&local_id);
                    }
                } else {
                    self.ctx.facts.arrow_locals.remove(&local_id);
                    self.ctx
                        .facts
                        .function_metadata_name_locals
                        .remove(&local_id);
                    self.ctx
                        .facts
                        .constructable_function_locals
                        .remove(&local_id);
                }
                if let Some(bound_function) = bound_function {
                    self.ctx
                        .facts
                        .bound_function_locals
                        .insert(local_id, bound_function);
                } else {
                    self.ctx.facts.bound_function_locals.remove(&local_id);
                }
                if let Some(function_method) = function_method {
                    self.ctx
                        .facts
                        .function_method_locals
                        .insert(local_id, function_method);
                } else {
                    self.ctx.facts.function_method_locals.remove(&local_id);
                }
                if let Some(bound_constructor) = bound_constructor {
                    self.ctx
                        .facts
                        .bound_constructor_locals
                        .insert(local_id, bound_constructor);
                } else {
                    self.ctx.facts.bound_constructor_locals.remove(&local_id);
                }
                self.update_heap_closure_local(local_id, expr, &lowered);
                crate::lowered::resolver::expr::facts::update_nullish_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_host_function_handle_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_host_external_object_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_bigint_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_control_flow_bigint_assignment(
                    &mut self.ctx,
                    local_id,
                );
                crate::lowered::resolver::expr::facts::update_array_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_symbol_iterator_object_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_array_iterator_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_generator_iterator_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                    generator_state_local,
                );
                crate::lowered::resolver::expr::facts::update_proxy_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::invalidate_static_function_array_like_local(
                    &mut self.ctx,
                    local_id,
                );
                crate::lowered::resolver::string::update_string_literal_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::string::update_number_literal_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::string::update_symbol_value_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_native_set_add_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                crate::lowered::resolver::expr::facts::update_invalid_date_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                if let Some(props) = function_props {
                    self.ctx
                        .classes
                        .object_function_props
                        .insert(local_id, props);
                } else {
                    self.ctx.classes.object_function_props.remove(&local_id);
                }
                if let Some(props) = accessor_props {
                    self.ctx
                        .classes
                        .object_accessor_props
                        .insert(local_id, props);
                } else {
                    self.ctx.classes.object_accessor_props.remove(&local_id);
                }
                crate::lowered::resolver::string::update_regexp_literal_local(
                    &mut self.ctx,
                    local_id,
                    expr,
                );
                let assign_stmt = if self.ctx.facts.env_cell_locals.contains(&local_id) {
                    LoweredStmt::Expr(
                        LoweredExpr::EnvCellSet {
                            cell: local_id,
                            expr: Box::new(lowered),

                            span: Span::generated("env_cell_set"),
                        },
                        Span::generated("expr_stmt"),
                    )
                } else {
                    LoweredStmt::Assign(local_id, lowered, Span::generated("assign"))
                };
                if let Some(state_local) = generator_state_local {
                    Ok(LoweredStmt::Block(
                        vec![
                            assign_stmt,
                            LoweredStmt::Let(
                                state_local,
                                LoweredExpr::Number(0, Span::generated("num")),
                                Span::generated("let_stmt"),
                            ),
                        ],
                        Span::generated("block"),
                    ))
                } else {
                    Ok(assign_stmt)
                }
            }
            ResolvedStmt::Expr(expr) => {
                if let Some(lowered) = self.lower_direct_iife_stmt(expr)? {
                    return Ok(lowered);
                }
                crate::lowered::resolver::expr::facts::update_static_array_slot_assignment(
                    &mut self.ctx,
                    expr,
                );
                if let ResolvedExpr::MethodCall {
                    object,
                    method,
                    args,
                    ..
                } = expr
                    && method == "push"
                    && args.len() == 1
                    && let ResolvedExpr::Ident(name) = object.as_ref()
                    && let Ok(local_id) = self.resolve_local(name)
                    && self.ctx.facts.array_locals.contains(&local_id)
                {
                    if let ResolvedExpr::Spread(spread_expr) = &args[0] {
                        return Ok(LoweredStmt::Expr(
                            self.lower_array_push_single_spread_arg(object, spread_expr.as_ref())?,
                            Span::generated("expr_stmt"),
                        ));
                    }
                    return Ok(LoweredStmt::Assign(
                        local_id,
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::ArrayPushGrow,
                            args: vec![
                                LoweredExpr::Local(local_id, Span::generated("local")),
                                self.lower_expr(&args[0])?,
                            ],

                            span: Span::generated("runtime_call"),
                        },
                        Span::generated("assign_stmt"),
                    ));
                }
                Ok(LoweredStmt::Expr(
                    self.lower_expr(expr)?,
                    Span::generated("expr_stmt"),
                ))
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let condition = self.lower_expr(condition)?;
                let incoming_bigint_locals = self.ctx.facts.bigint_locals.clone();
                let incoming_div_rem_locals =
                    self.ctx.facts.control_flow_bigint_div_rem_locals.clone();
                let incoming_mixed_locals = self.ctx.facts.control_flow_mixed_bigint_locals.clone();

                let then_body = self.lower_nested_block(then_body)?;
                let then_add_sub_bigint_locals = self.ctx.facts.bigint_locals.clone();
                let then_div_rem_bigint_locals =
                    crate::lowered::resolver::expr::facts::bigint_div_rem_candidate_locals(
                        &self.ctx,
                    );
                let then_mixed_locals = self.ctx.facts.control_flow_mixed_bigint_locals.clone();

                self.ctx.facts.bigint_locals = incoming_bigint_locals.clone();
                self.ctx.facts.control_flow_bigint_div_rem_locals = incoming_div_rem_locals.clone();
                self.ctx.facts.control_flow_mixed_bigint_locals = incoming_mixed_locals.clone();

                let else_body = self.lower_nested_block(else_body)?;
                let else_add_sub_bigint_locals = self.ctx.facts.bigint_locals.clone();
                let else_div_rem_bigint_locals =
                    crate::lowered::resolver::expr::facts::bigint_div_rem_candidate_locals(
                        &self.ctx,
                    );
                let else_mixed_locals = self.ctx.facts.control_flow_mixed_bigint_locals.clone();

                self.ctx.facts.bigint_locals = then_add_sub_bigint_locals
                    .intersection(&else_add_sub_bigint_locals)
                    .copied()
                    .collect();
                let definite_div_rem_locals = then_div_rem_bigint_locals
                    .intersection(&else_div_rem_bigint_locals)
                    .copied()
                    .collect::<HashSet<_>>();
                let branch_mixed_locals = then_div_rem_bigint_locals
                    .symmetric_difference(&else_div_rem_bigint_locals)
                    .copied()
                    .chain(then_mixed_locals.union(&else_mixed_locals).copied())
                    .filter(|local| !definite_div_rem_locals.contains(local))
                    .collect::<HashSet<_>>();

                self.ctx.facts.control_flow_bigint_div_rem_locals = definite_div_rem_locals
                    .difference(&self.ctx.facts.bigint_locals)
                    .copied()
                    .collect();
                self.ctx.facts.control_flow_mixed_bigint_locals = branch_mixed_locals;
                Ok(LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,

                    span: Span::generated("if_stmt"),
                })
            }
            ResolvedStmt::While { condition, body } => Ok(LoweredStmt::While {
                condition: self.lower_expr(condition)?,
                body: self.lower_nested_block(body)?,
                span: Span::generated("while"),
            }),
            ResolvedStmt::Return(expr) => {
                if let ResolvedExpr::Ident(name) = expr
                    && let Some(closure) = self
                        .resolve_local(name)
                        .ok()
                        .and_then(|local| self.ctx.facts.arrow_locals.get(&local))
                {
                    return Ok(LoweredStmt::Return(
                        closure.to_expr(ClosureRepresentation::HeapObject),
                        Span::generated("return_stmt"),
                    ));
                }
                if let ResolvedExpr::ArrowFn {
                    params,
                    body,
                    body_stmts,
                    ..
                } = expr
                {
                    let lowered = self.lower_arrow_fn(params, body, body_stmts)?;
                    if let LoweredExpr::ArrowFn {
                        func_id, captures, ..
                    } = &lowered
                        && !captures.is_empty()
                    {
                        return Ok(LoweredStmt::Return(
                            LoweredExpr::ArrowFn {
                                func_id: *func_id,
                                captures: captures.clone(),
                                representation: ClosureRepresentation::HeapObject,
                                span: Span::generated("arrow_fn"),
                            },
                            Span::generated("return_stmt"),
                        ));
                    }
                    return Ok(LoweredStmt::Return(lowered, Span::generated("return_stmt")));
                }
                // Handle `return yield* iterable;` — delegate yield* then return the result.
                if let ResolvedExpr::Yield {
                    expr: Some(inner),
                    delegate: true,
                } = expr
                {
                    let result_local = self.alloc_temp();
                    let stmts = match self.lower_yield_star_stmt(inner, Some(result_local))? {
                        LoweredStmt::Block(s, _) => s,
                        other => vec![other],
                    };
                    let mut all_stmts = stmts;
                    all_stmts.push(LoweredStmt::Return(
                        LoweredExpr::Local(result_local, Span::generated("local")),
                        Span::generated("return_stmt"),
                    ));
                    return Ok(LoweredStmt::Block(all_stmts, Span::generated("block")));
                }
                Ok(LoweredStmt::Return(
                    self.lower_expr(expr)?,
                    Span::generated("return_stmt"),
                ))
            }
            ResolvedStmt::Function {
                name,
                params,
                body,
                is_async,
                ..
            } => {
                let local_id = self.declare_local(name)?;
                if self.ctx.facts.env_cell_names.contains(name) {
                    self.ctx.facts.env_cell_locals.insert(local_id);
                    self.ctx.facts.initialized_env_cell_locals.insert(local_id);
                }
                let closure = self.lower_nested_function(name, params, body, *is_async)?;
                if let LoweredExpr::ArrowFn {
                    func_id,
                    captures,
                    representation,

                    span: _,
                } = &closure
                {
                    if matches!(representation, ClosureRepresentation::HeapObject) {
                        self.ctx.facts.heap_closure_locals.insert(local_id);
                    } else {
                        self.ctx.facts.arrow_locals.insert(
                            local_id,
                            ArrowClosure {
                                func_id: *func_id,
                                captures: captures.clone(),
                            },
                        );
                    }
                }
                self.ctx.facts.nullish_locals.remove(&local_id);
                if self.ctx.facts.env_cell_locals.contains(&local_id) {
                    Ok(LoweredStmt::Block(
                        vec![
                            LoweredStmt::Let(
                                local_id,
                                LoweredExpr::EnvCellNew(
                                    Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                                    Span::generated("env_cell_new"),
                                ),
                                Span::generated("let_stmt"),
                            ),
                            LoweredStmt::Expr(
                                LoweredExpr::EnvCellSet {
                                    cell: local_id,
                                    expr: Box::new(closure),
                                    span: Span::generated("env_cell_set"),
                                },
                                Span::generated("expr_stmt"),
                            ),
                        ],
                        Span::generated("block"),
                    ))
                } else {
                    Ok(LoweredStmt::Let(
                        local_id,
                        closure,
                        Span::generated("let_stmt"),
                    ))
                }
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_param,
                catch_block,
                finally_block,
            } => {
                if catch_block.is_none() && finally_block.is_some() {
                    // Pure try-finally (no catch): emit as TryFinally for
                    // proper completion record semantics via CompletionRecord types.
                    Ok(LoweredStmt::TryFinally {
                        try_body: self.lower_nested_block(try_block)?,
                        finally_body: self.lower_nested_block(finally_block.as_ref().unwrap())?,
                        span: Span::generated("try_finally"),
                    })
                } else {
                    let try_body = self.lower_nested_block(try_block)?;
                    let (catch_var, catch_body) = if let Some(block) = catch_block {
                        self.ctx.symbols.scopes.push(HashMap::new());
                        let lowered = (|| {
                            let catch_var = if let Some(param) = catch_param.as_deref() {
                                let local_id = self.declare_local(param)?;
                                if self.ctx.facts.env_cell_names.contains(param) {
                                    self.ctx.facts.env_cell_locals.insert(local_id);
                                    self.ctx.facts.initialized_env_cell_locals.insert(local_id);
                                }
                                if block_may_catch_host_external_object(&self.ctx, try_block) {
                                    self.ctx.facts.mark_host_external(
                                        local_id,
                                        HostExternalKind::Object,
                                        true,
                                    );
                                }
                                Some(local_id)
                            } else {
                                None
                            };
                            let mut catch_body = self.lower_block(block)?;
                            if let Some(local_id) = catch_var
                                && self.ctx.facts.env_cell_locals.contains(&local_id)
                            {
                                catch_body.insert(
                                    0,
                                    LoweredStmt::Assign(
                                        local_id,
                                        LoweredExpr::EnvCellNew(
                                            Box::new(LoweredExpr::Local(
                                                local_id,
                                                Span::generated("catch_binding"),
                                            )),
                                            Span::generated("env_cell_new"),
                                        ),
                                        Span::generated("assign"),
                                    ),
                                );
                            }
                            Ok((catch_var, Some(catch_body)))
                        })();
                        self.ctx.symbols.scopes.pop();
                        lowered?
                    } else {
                        (None, None)
                    };
                    Ok(LoweredStmt::TryCatch {
                        try_body,
                        catch_var,
                        catch_body,
                        finally_body: finally_block
                            .as_ref()
                            .map(|b| self.lower_nested_block(b))
                            .transpose()?,
                        span: Span::generated("try_catch"),
                    })
                }
            }
            ResolvedStmt::Throw(expr) => Ok(LoweredStmt::Throw(
                self.lower_expr(expr)?,
                Span::generated("throw_stmt"),
            )),
            ResolvedStmt::Switch { expr, cases } => {
                let resolved_cases = cases
                    .iter()
                    .map(|(cond, body)| {
                        Ok((
                            cond.as_ref().map(|e| self.lower_expr(e)).transpose()?,
                            self.lower_nested_block(body)?,
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(LoweredStmt::Switch {
                    expr: self.lower_expr(expr)?,
                    cases: resolved_cases,
                    span: Span::generated("switch"),
                })
            }
            ResolvedStmt::DoWhile { body, condition } => Ok(LoweredStmt::DoWhile {
                body: self.lower_nested_block(body)?,
                condition: self.lower_expr(condition)?,
                span: Span::generated("do_while"),
            }),
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                self.ctx.symbols.scopes.push(HashMap::new());
                let resolved_init = init
                    .as_ref()
                    .map(|s| self.lower_stmt(s))
                    .transpose()?
                    .map(Box::new);
                let resolved = LoweredStmt::For {
                    init: resolved_init,
                    condition: condition.as_ref().map(|c| self.lower_expr(c)).transpose()?,
                    update: update.as_ref().map(|u| self.lower_expr(u)).transpose()?,
                    body: self.lower_nested_block(body)?,
                    span: Span::generated("for"),
                };
                self.ctx.symbols.scopes.pop();
                Ok(resolved)
            }
            ResolvedStmt::ForIn { var, iter, body } => {
                let var_id = self.declare_local(var)?;
                Ok(LoweredStmt::ForIn {
                    var: var_id,
                    iter: self.lower_expr(iter)?,
                    iter_local: self.alloc_temp(),
                    index_local: self.alloc_temp(),
                    len_local: self.alloc_temp(),
                    body: self.lower_nested_block(body)?,
                    span: Span::generated("for_in"),
                })
            }
            ResolvedStmt::ForOf { var, iter, body } => {
                let var_id = self.declare_local(var)?;
                // Route custom iterables through iterator protocol (Symbol.iterator)
                if crate::lowered::resolver::expr::facts::resolved_expr_has_symbol_iterator_property(
                    &self.ctx, iter,
                ) {
                    return self.lower_for_of_via_iterator(var_id, iter, body);
                }
                let lowered_iter = if let ResolvedExpr::Ident(name) = iter
                    && let Ok(local_id) = self.resolve_local(name)
                {
                    let class_name = self.ctx.classes.local_classes.get(&local_id);
                    if class_name.is_some_and(|c| c == "Set") {
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::SetValuesArray,
                            args: vec![LoweredExpr::Local(local_id, Span::generated("local"))],

                            span: Span::generated("runtime_call"),
                        }
                    } else if class_name.is_some_and(|c| c == "Map") {
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::MapEntryPairsArray,
                            args: vec![LoweredExpr::Local(local_id, Span::generated("local"))],
                            span: Span::generated("runtime_call"),
                        }
                    } else {
                        self.lower_expr(iter)?
                    }
                } else {
                    self.lower_expr(iter)?
                };
                Ok(LoweredStmt::ForOf {
                    var: var_id,
                    iter: lowered_iter,
                    iter_local: self.alloc_temp(),
                    index_local: self.alloc_temp(),
                    len_local: self.alloc_temp(),
                    body: self.lower_nested_block(body)?,
                    span: Span::generated("for_of"),
                })
            }
            ResolvedStmt::ForAwaitOf { var, iter, body } => {
                let var_id = self.declare_local(var)?;
                Ok(LoweredStmt::ForAwaitOfLower {
                    var: var_id,
                    iter: self.lower_expr(iter)?,
                    async_iter_local: self.alloc_temp(),
                    next_result_local: self.alloc_temp(),
                    done_local: self.alloc_temp(),
                    value_local: self.alloc_temp(),
                    body: self.lower_nested_block(body)?,
                    span: Span::generated("for_await_of"),
                })
            }
            ResolvedStmt::Labeled { label, body } => Ok(LoweredStmt::Labeled {
                label: label.clone(),
                body: Box::new(self.lower_stmt(body)?),
                span: Span::generated("labeled"),
            }),
            ResolvedStmt::Break { label } => Ok(LoweredStmt::Break {
                label: label.clone(),
                span: Span::generated("break"),
            }),
            ResolvedStmt::Continue { label } => Ok(LoweredStmt::Continue {
                label: label.clone(),
                span: Span::generated("continue"),
            }),
            ResolvedStmt::Export { name, expr } => Ok(LoweredStmt::Export {
                name: name.clone(),
                expr: self.lower_expr(expr)?,
                span: Span::generated("export"),
            }),
            ResolvedStmt::ModuleExportsAssign { expr } => Ok(LoweredStmt::ModuleExportsAssign {
                expr: self.lower_expr(expr)?,
                span: Span::generated("module_exports_assign"),
            }),
            ResolvedStmt::ClassDecl { .. } => Ok(LoweredStmt::Expr(
                LoweredExpr::Undefined(Span::generated("undef")),
                Span::generated("expr_stmt"),
            )),
            ResolvedStmt::Block { statements, .. } => Ok(LoweredStmt::Block(
                self.lower_nested_block(statements)?,
                Span::generated("block"),
            )),
        }
    }

    pub(super) fn lower_eval_class_decl(
        &mut self,
        class_decl: EvalClassDeclParts<'_>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let EvalClassDeclParts {
            name,
            extends,
            constructor,
            methods,
            private_fields,
            static_private_fields,
            static_blocks,
        } = class_decl;

        let ctor_id = FuncId(self.ctx.functions.next_func_id);
        self.ctx.functions.next_func_id += 1;
        let mut eval_function_ids = self.ctx.symbols.function_ids.clone();
        eval_function_ids.insert(eval_class_constructor_key(name), ctor_id);
        self.ctx
            .classes
            .class_constructor_ids
            .insert(name.to_owned(), ctor_id);
        self.ctx
            .classes
            .class_parents
            .insert(name.to_owned(), extends.clone());
        self.ctx.classes.class_private_fields.insert(
            name.to_owned(),
            private_fields
                .iter()
                .enumerate()
                .map(|(slot, field)| (field.clone(), slot))
                .collect(),
        );
        self.ctx.classes.class_static_private_fields.insert(
            name.to_owned(),
            static_private_fields
                .iter()
                .map(|(field, _, _)| {
                    (
                        field.clone(),
                        crate::builtin_resolver::static_private_field_local_name(name, field),
                    )
                })
                .collect(),
        );

        let mut instance_methods = Vec::new();
        let mut static_methods = Vec::new();
        let mut method_ids = Vec::new();
        for method in methods {
            let method_id = FuncId(self.ctx.functions.next_func_id);
            self.ctx.functions.next_func_id += 1;
            eval_function_ids.insert(eval_class_method_key(name, &method.name), method_id);
            if let Some(stripped) = method.name.strip_prefix("static::") {
                self.ctx
                    .classes
                    .class_static_method_ids
                    .insert((name.to_owned(), stripped.to_owned()), method_id);
                static_methods.push((stripped.to_owned(), method_id));
            } else {
                self.ctx
                    .classes
                    .class_method_ids
                    .insert((name.to_owned(), method.name.clone()), method_id);
                if method.kind == ClassMethodKind::Method {
                    instance_methods.push((method.name.clone(), method_id));
                }
            }
            method_ids.push((method, method_id));
        }

        let (ctor_params, ctor_body) = constructor
            .as_ref()
            .cloned()
            .unwrap_or_else(|| (Vec::new(), Vec::new()));
        let mut ctor_params_for_lowering = ctor_params;
        if constructor.is_none() && extends.is_some() {
            ctor_params_for_lowering.push(ResolvedParam {
                name: "...args".to_owned(),
                default: None,
                is_rest: true,
                span: None,
            });
        }

        self.ctx.symbols.function_signatures.insert(
            ctor_id,
            FunctionSignature {
                explicit_params: ctor_params_for_lowering.len(),
                needs_receiver: true,
                has_rest: ctor_params_for_lowering.iter().any(|param| param.is_rest),
                is_strict: true,
                ..FunctionSignature::default()
            },
        );
        let function_signatures = self.ctx.symbols.function_signatures.clone();
        let function_captures = self.ctx.functions.function_captures.clone();
        let function_mutable_captures = self.ctx.functions.function_mutable_captures.clone();
        let lowered = lower_function(
            ctor_id,
            &ctor_params_for_lowering,
            &ctor_body,
            false,
            false,
            &eval_function_ids,
            &function_signatures,
            &function_captures,
            &function_mutable_captures,
            &self.ctx.functions.class_method_captures,
            &self.ctx.functions.class_method_mutable_captures,
            &collect_dynamic_direct_eval_env_cell_names(
                &ctor_params_for_lowering,
                &ctor_body,
                true,
                true,
            ),
            &self.ctx.facts.heap_closure_names,
            self.ctx.classes.class_parents.clone(),
            self.ctx.classes.class_private_fields.clone(),
            self.ctx.classes.class_static_private_fields.clone(),
            LowerFunctionOptions {
                current_class: Some(name),
                in_constructor: true,
                in_static_method: false,
                next_func_id: self.ctx.functions.next_func_id,
                self_closure: None,
                capture_facts: FunctionCaptureFacts::default(),
                recursion_depth: 0,
                new_target_class: Some(name),
                module_url: self.ctx.current_module_url.as_str(),
                strict_context: true,
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

        for (method, method_id) in method_ids {
            let mut method_params_for_lowering = method.params.clone();
            method_params_for_lowering.extend(method.captures.iter().map(|capture| {
                ResolvedParam {
                    name: capture.clone(),
                    default: None,
                    is_rest: false,
                    span: None,
                }
            }));
            self.ctx.symbols.function_signatures.insert(
                method_id,
                FunctionSignature {
                    explicit_params: method.params.len(),
                    needs_receiver: block_contains_this(&method.body)
                        || block_contains_dynamic_direct_eval(&method.body)
                        || (!method.name.starts_with("static::")
                            && block_contains_super(&method.body)),
                    needs_arguments: (block_contains_arguments(&method.body)
                        || block_contains_dynamic_direct_eval(&method.body))
                        && !method.params.iter().any(|param| param.name == "arguments"),
                    has_rest: method.params.iter().any(|param| param.is_rest),
                    is_strict: true,
                    metadata_length: Some(method.params.len()),
                    ..FunctionSignature::default()
                },
            );
            if !method.captures.is_empty() {
                self.ctx
                    .functions
                    .class_method_captures
                    .insert(method_id, method.captures.clone());
            }
            let function_signatures = self.ctx.symbols.function_signatures.clone();
            let function_captures = self.ctx.functions.function_captures.clone();
            let function_mutable_captures = self.ctx.functions.function_mutable_captures.clone();
            let lowered = lower_function(
                method_id,
                &method_params_for_lowering,
                &method.body,
                false,
                false,
                &eval_function_ids,
                &function_signatures,
                &function_captures,
                &function_mutable_captures,
                &self.ctx.functions.class_method_captures,
                &self.ctx.functions.class_method_mutable_captures,
                &collect_dynamic_direct_eval_env_cell_names(
                    &method_params_for_lowering,
                    &method.body,
                    true,
                    true,
                ),
                &self.ctx.facts.heap_closure_names,
                self.ctx.classes.class_parents.clone(),
                self.ctx.classes.class_private_fields.clone(),
                self.ctx.classes.class_static_private_fields.clone(),
                LowerFunctionOptions {
                    current_class: Some(name),
                    in_constructor: false,
                    in_static_method: method.name.starts_with("static::"),
                    next_func_id: self.ctx.functions.next_func_id,
                    self_closure: None,
                    capture_facts: FunctionCaptureFacts::default(),
                    recursion_depth: 0,
                    new_target_class: None,
                    module_url: self.ctx.current_module_url.as_str(),
                    strict_context: true,
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
        }

        let mut stmts = vec![LoweredStmt::ClassDecl {
            name: name.to_owned(),
            extends: extends.clone(),
            constructor: Some(ctor_id),
            methods: instance_methods,
            static_methods,
            private_fields: private_fields.to_vec(),
            span: Span::generated("eval_class_decl"),
        }];
        for (field, initializer, _) in static_private_fields {
            stmts.push(self.lower_class_static_private_field(name, field, initializer)?);
        }
        for (_, block) in static_blocks {
            stmts.extend(self.lower_class_static_block(name, block)?);
        }
        Ok(stmts)
    }

    pub(super) fn lower_class_static_private_field(
        &mut self,
        class_name: &str,
        field: &str,
        initializer: &ResolvedExpr,
    ) -> Result<LoweredStmt, Diagnostic> {
        let local_name =
            crate::builtin_resolver::static_private_field_local_name(class_name, field);
        self.with_current_class(class_name, |resolver| {
            resolver.lower_stmt(&ResolvedStmt::Let(local_name, initializer.clone()))
        })
    }

    pub(super) fn lower_class_static_block(
        &mut self,
        class_name: &str,
        block: &[ResolvedStmt],
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        self.with_current_class(class_name, |resolver| {
            let previous = resolver
                .ctx
                .classes
                .static_block_this_class
                .replace(class_name.to_owned());
            let lowered = resolver.lower_nested_block(block);
            resolver.ctx.classes.static_block_this_class = previous;
            lowered
        })
    }

    fn with_current_class<T>(
        &mut self,
        class_name: &str,
        f: impl FnOnce(&mut Self) -> Result<T, Diagnostic>,
    ) -> Result<T, Diagnostic> {
        let previous = self
            .ctx
            .classes
            .current_class
            .replace(class_name.to_owned());
        let result = f(self);
        self.ctx.classes.current_class = previous;
        result
    }

    /// Resolve a type alias name to its underlying TypeRef, delegating to
    /// `LoweringCtx::resolve_type_alias`. Returns `None` if the name is not
    /// a known type alias.
    pub(crate) fn resolve_type_alias(&self, name: &str) -> Option<&TypeRef> {
        self.ctx.resolve_type_alias(name)
    }

    /// Look up the property signatures of an interface by name, delegating to
    /// `LoweringCtx::lookup_interface_properties`. Returns `None` if the name
    /// is not a known interface definition.
    pub(crate) fn lookup_interface_properties(&self, name: &str) -> Option<&[(String, TypeRef)]> {
        self.ctx.lookup_interface_properties(name)
    }
}

fn eval_class_constructor_key(class_name: &str) -> String {
    format!("class::{class_name}::constructor")
}

fn eval_class_method_key(class_name: &str, method_name: &str) -> String {
    format!("class::{class_name}::{method_name}")
}

pub(crate) fn class_maps(
    function_ids: &HashMap<String, FuncId>,
) -> (ClassConstructorMap, ClassMethodMap, ClassMethodMap) {
    let mut ctor_ids = HashMap::new();
    let mut method_ids = HashMap::new();
    let mut static_method_ids = HashMap::new();

    for (name, id) in function_ids {
        if let Some(rest) = name.strip_prefix("class::") {
            let mut parts = rest.splitn(2, "::");
            let class = parts.next().unwrap_or_default();
            let member = parts.next().unwrap_or_default();
            if member == "constructor" {
                ctor_ids.insert(class.to_owned(), *id);
            } else if let Some(static_name) = member.strip_prefix("static::") {
                static_method_ids.insert((class.to_owned(), static_name.to_owned()), *id);
            } else if !class.is_empty() && !member.is_empty() {
                method_ids.insert((class.to_owned(), member.to_owned()), *id);
            }
        }
    }

    (ctor_ids, method_ids, static_method_ids)
}

pub(crate) fn lowered_binding_default(default: &BindingDefault) -> Option<LoweredExpr> {
    match default {
        BindingDefault::Number(value) => Some(LoweredExpr::Number(*value, Span::generated("num"))),
        BindingDefault::String(value) => {
            Some(LoweredExpr::String(value.clone(), Span::generated("str")))
        }
        BindingDefault::Bool(value) => Some(LoweredExpr::Bool(*value, Span::generated("bool"))),
        BindingDefault::Null => Some(LoweredExpr::Null(Span::generated("null"))),
        BindingDefault::Undefined => Some(LoweredExpr::Undefined(Span::generated("undef"))),
        BindingDefault::Array(elements) => {
            let elements = elements
                .iter()
                .map(|element| {
                    if let Some(element) = element.as_ref() {
                        lowered_binding_default(element)
                    } else {
                        Some(LoweredExpr::Undefined(Span::generated("undef")))
                    }
                })
                .collect::<Option<Vec<_>>>()?;
            Some(LoweredExpr::ArrayNew {
                elements,
                span: Span::generated("array_new"),
            })
        }
        BindingDefault::Object(props) => {
            let props = props
                .iter()
                .map(|(key, value)| {
                    lowered_binding_default(value).map(|value| (key.clone(), value))
                })
                .collect::<Option<Vec<_>>>()?;
            Some(LoweredExpr::ObjectNew {
                props,
                non_enumerable: 0,
                span: Span::generated("object_new"),
            })
        }
        BindingDefault::Ident(_)
        | BindingDefault::FunctionExpr { .. }
        | BindingDefault::ArrowFn
        | BindingDefault::ClassExpr { .. }
        | BindingDefault::Call(_)
        | BindingDefault::PreIncrement(_)
        | BindingDefault::FunctionIife { .. } => None,
    }
}

fn binding_param_names<'a>(
    params: impl Iterator<Item = (&'a str, Option<Span>)>,
) -> Result<Vec<String>, Diagnostic> {
    let mut names = Vec::new();
    for (param, span) in params {
        if let Some(inner) = param.strip_prefix("...") {
            // Rest param: extract inner names from binding pattern
            if let Some(pattern) = parse_binding_pattern(inner, span)? {
                names.extend(pattern.names().into_iter().map(ToOwned::to_owned));
            } else {
                // Push the clean name (without `...`) so capture exclusion matches
                names.push(inner.to_owned());
            }
        } else if let Some(pattern) = parse_binding_pattern(param, span)? {
            names.extend(pattern.names().into_iter().map(ToOwned::to_owned));
        } else {
            names.push(param.to_owned());
        }
    }
    Ok(names)
}

fn binding_param_default_ref_names<'a>(
    params: impl Iterator<Item = (&'a str, Option<Span>)>,
) -> Result<Vec<String>, Diagnostic> {
    let mut names = Vec::new();
    for (param, span) in params {
        let binding = param.strip_prefix("...").unwrap_or(param);
        if let Some(pattern) = parse_binding_pattern(binding, span)? {
            names.extend(
                pattern
                    .default_ref_names()
                    .into_iter()
                    .map(ToOwned::to_owned),
            );
        }
    }
    Ok(names)
}

const PRIVATE_FIELD_STORAGE_PREFIX: &str = "__ts2wasm_private::";

pub(crate) fn is_private_field_storage_key(key: &str) -> bool {
    key.starts_with(PRIVATE_FIELD_STORAGE_PREFIX)
}

pub(crate) fn private_storage_observable_access_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-255: private field backing storage is not accessible through ordinary property access in this private field runtime slice".to_owned(),
        span,


        phase: None,}
}

pub(crate) fn is_static_copy_safe_object_prop_value(expr: &ResolvedExpr) -> bool {
    matches!(
        expr,
        ResolvedExpr::Number(_)
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined
    )
}

pub(crate) fn is_set_prototype_property(
    object: &ResolvedExpr,
    key: &str,
    expected_key: &str,
) -> bool {
    key == expected_key && matches_set_prototype_object(object)
}

pub(crate) fn is_set_prototype_property_expr(expr: &ResolvedExpr, expected_key: &str) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    is_set_prototype_property(object, key, expected_key)
}

pub(crate) fn is_array_prototype_push_property(object: &ResolvedExpr, key: &str) -> bool {
    key == "push" && matches_array_prototype_object(object)
}

pub(crate) fn is_array_prototype_push_expr(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    is_array_prototype_push_property(object, key)
}

pub(crate) fn matches_array_prototype_object(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Array"
        )
}

pub(crate) fn matches_set_prototype_object(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Set"
        )
}

pub(crate) fn matches_map_prototype_object(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Map"
        )
}

pub(crate) fn is_map_prototype_property(
    object: &ResolvedExpr,
    key: &str,
    expected_key: &str,
) -> bool {
    key == expected_key && matches_map_prototype_object(object)
}

pub(crate) fn unsupported_array_map_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-270: Array.prototype.map requires callback dispatch and new array allocation semantics that are not supported in this runtime slice".to_owned(),
        span,


        phase: None,}
}

pub(crate) fn unsupported_array_sort_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-299: Array.prototype.sort is currently supported only for dense numeric arrays with comparator `(a, b) => a - b`".to_owned(),
        span,


        phase: None,}
}

pub(crate) fn is_static_date_constructor_expr(expr: &ResolvedExpr) -> bool {
    matches!(expr, ResolvedExpr::New { class_name, .. } if class_name == "Date")
}

pub(crate) fn is_invalid_date_constructor_expr(expr: &ResolvedExpr) -> bool {
    matches!(
        expr,
        ResolvedExpr::New {
            class_name,
            args,
            ..
        } if class_name == "Date"
            && (matches!(args.as_slice(), [ResolvedExpr::Object(_)])
                || matches!(args.as_slice(), [ResolvedExpr::Ident(name)] if name == "NaN"))
    )
}

pub(crate) fn is_array_prototype_map_call_receiver(object: &ResolvedExpr, method: &str) -> bool {
    method == "call" && matches_array_prototype_map_property(object)
}

pub(crate) fn is_array_prototype_every_some_call_receiver(
    object: &ResolvedExpr,
    method: &str,
) -> bool {
    method == "call" && matches_array_prototype_every_some_property(object)
}

pub(crate) fn is_array_from_call_receiver(object: &ResolvedExpr, method: &str) -> bool {
    method == "from" && matches!(object, ResolvedExpr::Ident(name) if name == "Array")
}

pub(crate) fn matches_array_prototype_map_property(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "map" && matches_array_prototype_property(object)
}

pub(crate) fn matches_array_prototype_every_some_property(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    (key == "every" || key == "some") && matches_array_prototype_property(object)
}

pub(crate) fn matches_array_prototype_property(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Array"
        )
}

pub(crate) fn is_string_split_result_expr(expr: &ResolvedExpr) -> bool {
    matches!(
        expr,
        ResolvedExpr::MethodCall { method, .. } if method == "split"
    )
}

pub(crate) fn is_identity_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [param] = params.as_slice() else {
        return false;
    };
    matches!(body.as_ref(), ResolvedExpr::Ident(name) if name == param)
}

pub(crate) fn is_number_double_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [param] = params.as_slice() else {
        return false;
    };
    let ResolvedExpr::Binary {
        left,
        op: BinaryOp::Multiply,
        right,
    } = body.as_ref()
    else {
        return false;
    };
    matches!(
        (left.as_ref(), right.as_ref()),
        (ResolvedExpr::Ident(name), ResolvedExpr::Number(2))
            if name == param
    ) || matches!(
        (left.as_ref(), right.as_ref()),
        (ResolvedExpr::Number(2), ResolvedExpr::Ident(name))
            if name == param
    )
}

pub(crate) fn string_split_arrow_separator(args: &[ResolvedExpr]) -> Option<&ResolvedExpr> {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return None;
    };
    let [param] = params.as_slice() else {
        return None;
    };
    let ResolvedExpr::MethodCall {
        object,
        method,
        args,
        ..
    } = body.as_ref()
    else {
        return None;
    };
    if method != "split" {
        return None;
    }
    let ResolvedExpr::Ident(name) = object.as_ref() else {
        return None;
    };
    if name != param {
        return None;
    }
    let [separator] = args.as_slice() else {
        return None;
    };
    Some(separator)
}

pub(crate) fn string_constructor_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [param] = params.as_slice() else {
        return false;
    };
    let ResolvedExpr::Call { callee, args, .. } = body.as_ref() else {
        return false;
    };
    if !matches!(callee.as_ref(), ResolvedExpr::Ident(name) if name == "String") {
        return false;
    }
    let [ResolvedExpr::Ident(arg_name)] = args.as_slice() else {
        return false;
    };
    arg_name == param
}

pub(crate) fn unary_plus_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [param] = params.as_slice() else {
        return false;
    };
    let ResolvedExpr::Unary { op, expr } = body.as_ref() else {
        return false;
    };
    *op == UnaryOp::Plus && matches!(expr.as_ref(), ResolvedExpr::Ident(name) if name == param)
}

pub(crate) fn numeric_ascending_sort_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [left_param, right_param] = params.as_slice() else {
        return false;
    };
    let ResolvedExpr::Binary {
        left,
        op: BinaryOp::Subtract,
        right,
    } = body.as_ref()
    else {
        return false;
    };
    matches!(left.as_ref(), ResolvedExpr::Ident(name) if name == left_param)
        && matches!(right.as_ref(), ResolvedExpr::Ident(name) if name == right_param)
}

pub(crate) fn function_prototype_method_name(expr: &ResolvedExpr) -> Option<&str> {
    let ResolvedExpr::PropertyAccess {
        object,
        key: method_name,
        ..
    } = expr
    else {
        return None;
    };
    let ResolvedExpr::PropertyAccess {
        object: class_expr,
        key: proto_key,
        ..
    } = object.as_ref()
    else {
        return None;
    };
    if proto_key != "prototype" {
        return None;
    }
    if !matches!(class_expr.as_ref(), ResolvedExpr::Ident(name) if name == "Function") {
        return None;
    }
    Some(method_name)
}

pub(crate) fn static_function_metadata_name_for_expr(
    ctx: &LoweringCtx,
    expr: &ResolvedExpr,
) -> Option<String> {
    match expr {
        ResolvedExpr::FunctionExpr {
            constructor_metadata: Some(metadata),
            ..
        } => Some(metadata.name.clone()),
        ResolvedExpr::FunctionExpr { origin, .. }
            if *origin == ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor =>
        {
            Some("anonymous".to_owned())
        }
        ResolvedExpr::PropertyAccess { key, .. } => Some(key.clone()),
        ResolvedExpr::ComputedIndex { index, .. } => {
            let key = string::resolved_expr_static_accessor_key(ctx, index)?;
            static_accessor_key_metadata_name(ctx, key)
        }
        _ => None,
    }
}

pub(crate) fn static_function_constructable_for_expr(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::FunctionExpr {
            constructor_metadata: Some(metadata),
            ..
        } => metadata.constructable,
        ResolvedExpr::FunctionExpr { origin, .. } => {
            *origin == ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor
        }
        _ => false,
    }
}

fn static_accessor_key_metadata_name(
    ctx: &LoweringCtx,
    key: crate::lowered::classes::ObjectAccessorKey,
) -> Option<String> {
    match key {
        crate::lowered::classes::ObjectAccessorKey::Property(name) => Some(name),
        crate::lowered::classes::ObjectAccessorKey::SymbolLocal(local) => {
            string::symbol_local_name(ctx, local)
        }
    }
}

/// Wrapper that converts bigint_runtime_fn_name string output to RuntimeFn.
fn bigint_runtime_fn_intrinsic(name: &str) -> Option<RuntimeFn> {
    match crate::builtin_resolver::bigint_runtime_fn_name(name) {
        Some("BigIntFromValue") => Some(RuntimeFn::BigIntFromValue),
        Some("BigIntAsIntN") => Some(RuntimeFn::BigIntAsIntN),
        Some("BigIntAsUintN") => Some(RuntimeFn::BigIntAsUintN),
        _ => None,
    }
}

fn block_may_catch_host_external_object(ctx: &LoweringCtx, stmts: &[ResolvedStmt]) -> bool {
    stmts
        .iter()
        .any(|stmt| stmt_may_throw_host_external_object(ctx, stmt))
}

fn stmt_may_throw_host_external_object(ctx: &LoweringCtx, stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::Assign(_, expr)
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr)
        | ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::DestructureAssign { expr, .. } => {
            expr_may_throw_host_external_object(ctx, expr)
        }
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_may_throw_host_external_object(ctx, expr)
        }
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_may_throw_host_external_object(ctx, condition)
                || block_may_catch_host_external_object(ctx, then_body)
                || block_may_catch_host_external_object(ctx, else_body)
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
            expr_may_throw_host_external_object(ctx, condition)
                || block_may_catch_host_external_object(ctx, body)
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_may_catch_host_external_object(ctx, try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block_may_catch_host_external_object(ctx, block))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block_may_catch_host_external_object(ctx, block))
        }
        ResolvedStmt::Switch { expr, cases } => {
            expr_may_throw_host_external_object(ctx, expr)
                || cases.iter().any(|(case_expr, body)| {
                    case_expr
                        .as_ref()
                        .is_some_and(|expr| expr_may_throw_host_external_object(ctx, expr))
                        || block_may_catch_host_external_object(ctx, body)
                })
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            init.as_ref()
                .is_some_and(|init| stmt_may_throw_host_external_object(ctx, init))
                || condition
                    .as_ref()
                    .is_some_and(|expr| expr_may_throw_host_external_object(ctx, expr))
                || update
                    .as_ref()
                    .is_some_and(|expr| expr_may_throw_host_external_object(ctx, expr))
                || block_may_catch_host_external_object(ctx, body)
        }
        ResolvedStmt::ForIn { iter, body, .. }
        | ResolvedStmt::ForOf { iter, body, .. }
        | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
            expr_may_throw_host_external_object(ctx, iter)
                || block_may_catch_host_external_object(ctx, body)
        }
        ResolvedStmt::Labeled { body, .. } => stmt_may_throw_host_external_object(ctx, body),
        ResolvedStmt::Block { statements } => block_may_catch_host_external_object(ctx, statements),
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

fn expr_may_throw_host_external_object(ctx: &LoweringCtx, expr: &ResolvedExpr) -> bool {
    if crate::lowered::resolver::expr::facts::resolved_expr_returns_host_external_object(ctx, expr)
    {
        return true;
    }

    match expr {
        ResolvedExpr::Eval { plan } => match plan.host_policy {
            EvalHostPolicy::DirectHost | EvalHostPolicy::IndirectHost => true,
            EvalHostPolicy::AotOnly => match &plan.source {
                EvalSource::NonStringStatic(expr) | EvalSource::Runtime(expr) => {
                    expr_may_throw_host_external_object(ctx, expr)
                }
                EvalSource::StaticLiteral(_) => false,
            },
        },
        ResolvedExpr::FunctionConstructor { plan } => {
            plan.host_policy == FunctionConstructorHostPolicy::HostCompile
                || plan
                    .args
                    .iter()
                    .any(|arg| expr_may_throw_host_external_object(ctx, arg))
        }
        ResolvedExpr::Await { expr }
        | ResolvedExpr::Spread(expr)
        | ResolvedExpr::Unary { expr, .. }
        | ResolvedExpr::Yield {
            expr: Some(expr), ..
        } => expr_may_throw_host_external_object(ctx, expr),
        ResolvedExpr::Binary { left, right, .. } => {
            expr_may_throw_host_external_object(ctx, left)
                || expr_may_throw_host_external_object(ctx, right)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_may_throw_host_external_object(ctx, condition)
                || expr_may_throw_host_external_object(ctx, then_expr)
                || expr_may_throw_host_external_object(ctx, else_expr)
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            expr_may_throw_host_external_object(ctx, callee)
                || args
                    .iter()
                    .any(|arg| expr_may_throw_host_external_object(ctx, arg))
        }
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } => {
            if matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "$262")
                && method == "evalScript"
            {
                return true;
            }
            expr_may_throw_host_external_object(ctx, object)
                || args
                    .iter()
                    .any(|arg| expr_may_throw_host_external_object(ctx, arg))
        }
        ResolvedExpr::Array(elements) => elements.iter().any(|element| {
            matches!(element, ResolvedArrayElement::Present(expr) if expr_may_throw_host_external_object(ctx, expr))
        }),
        ResolvedExpr::Object(props) => props.iter().any(|prop| {
            prop.computed_key()
                .is_some_and(|key| expr_may_throw_host_external_object(ctx, key))
                || expr_may_throw_host_external_object(ctx, prop.value())
        }),
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            expr_may_throw_host_external_object(ctx, object)
                || expr_may_throw_host_external_object(ctx, index)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => args
            .iter()
            .any(|arg| expr_may_throw_host_external_object(ctx, arg)),
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
            expr_may_throw_host_external_object(ctx, object)
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            expr_may_throw_host_external_object(ctx, object)
                || expr_may_throw_host_external_object(ctx, value)
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            expr_may_throw_host_external_object(ctx, object)
                || expr_may_throw_host_external_object(ctx, key)
                || expr_may_throw_host_external_object(ctx, value)
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
            expr_may_throw_host_external_object(ctx, expr)
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_may_throw_host_external_object(ctx, object)
                || expr_may_throw_host_external_object(ctx, expr)
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_may_throw_host_external_object(ctx, key)
                || expr_may_throw_host_external_object(ctx, expr)
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            expr_may_throw_host_external_object(ctx, object)
                || expr_may_throw_host_external_object(ctx, key)
                || expr_may_throw_host_external_object(ctx, expr)
        }
        ResolvedExpr::Sequence(exprs) => exprs
            .iter()
            .any(|expr| expr_may_throw_host_external_object(ctx, expr)),
        ResolvedExpr::EvalCompletion(steps) => steps
            .iter()
            .filter_map(|step| step.expr())
            .any(|expr| expr_may_throw_host_external_object(ctx, expr)),
        ResolvedExpr::ArrowFn {
            body, body_stmts, ..
        } => {
            block_may_catch_host_external_object(ctx, body_stmts)
                || expr_may_throw_host_external_object(ctx, body)
        }
        ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::Yield { expr: None, .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined => false,
    }
}
