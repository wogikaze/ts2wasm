use super::super::{
    bigint_runtime_fn_intrinsic, is_array_from_call_receiver,
    is_array_prototype_every_some_call_receiver, is_array_prototype_map_call_receiver,
    is_array_prototype_push_expr, is_error_class, is_identity_arrow_callback,
    is_set_prototype_property_expr, is_static_date_constructor_expr, is_string_split_result_expr,
    is_typed_array_class, numeric_ascending_sort_arrow_callback,
    private_storage_observable_access_diagnostic, string_constructor_arrow_callback,
    string_split_arrow_separator, unary_plus_arrow_callback, unsupported_array_map_diagnostic,
    unsupported_array_sort_diagnostic,
};
use super::builtin::{is_html_wrapper_string_method, lower_html_wrapper_string_method};
use super::receiver::extract_prototype_method_name;
use crate::builtin_resolved::{
    ResolvedArrayElement, ResolvedExpr, ResolvedObjectProp, ResolvedStmt,
};
use crate::lowered::classes::ObjectAccessorKey;
use crate::lowered::ctx::LoweringCtx;
use crate::lowered::facts::{
    GeneratorMethodIteratorBinding, GeneratorObjectResumePlan, HostExternalKind,
    IntlDateTimeFormatOptions, IntlNumberFormatOptions, ProxyTrapKind,
};
use crate::lowered::*;
use std::collections::HashMap;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::UnaryOp;

impl super::super::Resolver {
    pub(crate) fn lower_method_call_expr(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if matches!(object, ResolvedExpr::Ident(name) if name == "$262") && method == "evalScript" {
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!("$262.evalScript expects 1 argument, got {}", args.len()),
                    span: Some(span),
                    phase: None,
                });
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::Dollar262Eval,
                args: vec![self.lower_expr(&args[0])?],
                span: Span::generated("runtime_call"),
            });
        }
        if let Some(result) = self.lower_mcall_early(object, method, args, span)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_arraybuffer(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_typed_array(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_intl_date_time_format(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_intl_duration_format(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_intl_list_format(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_intl_number_format(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_json_date_regexp(object, method, args, span)? {
            return Ok(result);
        }
        // Function.prototype.toString for user-defined functions
        if method == "toString"
            && args.is_empty()
            && let ResolvedExpr::Ident(name) = object
            && let Ok(func_id) = self.resolve_func(name.as_str())
        {
            let source = self
                .ctx
                .function_sources
                .get(&func_id)
                .map(|s| s.as_str())
                .unwrap_or("");
            let body = if source.is_empty() {
                format!("function {}() {{ [native code] }}", name)
            } else {
                source.to_owned()
            };
            return Ok(LoweredExpr::String(body, span));
        }
        if method == "toString"
            && args.is_empty()
            && let ResolvedExpr::Ident(name) = object
            && let Ok(local_id) = self.resolve_local(name)
            && self
                .ctx
                .facts
                .is_host_external(local_id, HostExternalKind::FunctionHandle)
        {
            let receiver = if self.ctx.facts.env_cell_locals.contains(&local_id) {
                LoweredExpr::EnvCellGet(local_id, Span::generated("env_cell_get"))
            } else {
                LoweredExpr::Local(local_id, Span::generated("local"))
            };
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::FunctionCallMethodHost,
                args: vec![
                    object_kernel::ordinary_get(receiver.clone(), method, span),
                    receiver,
                    LoweredExpr::ArrayNew {
                        elements: Vec::new(),
                        span: Span::generated("array"),
                    },
                ],
                span: Span::generated("runtime_call"),
            });
        }
        if let Some(result) = self.lower_mcall_date_string(object, method, args, span)? {
            return Ok(result);
        }
        if method == "next"
            && args.len() <= 1
            && self.resolved_expr_is_direct_generator_call(object)
        {
            if args.is_empty()
                && let Some(result) = self.lower_static_object_generator_return_next(object)
            {
                return Ok(result);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::GeneratorNext,
                args: vec![self.lower_expr(object)?],
                span: Span::generated("runtime_call"),
            });
        }
        if method == "next"
            && args.len() <= 1
            && crate::lowered::resolver::expr::facts::resolved_expr_is_generator_iterator(
                &self.ctx, object,
            )
        {
            if let Some(result) = self.lower_static_generator_next(object, args)? {
                return Ok(result);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::GeneratorNext,
                args: vec![self.lower_expr(object)?],
                span: Span::generated("runtime_call"),
            });
        }
        if method == "next"
            && args.is_empty()
            && crate::lowered::resolver::expr::facts::resolved_expr_is_array_iterator(
                &self.ctx, object,
            )
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayIteratorNext,
                args: vec![self.lower_expr(object)?],
                span: Span::generated("runtime_call"),
            });
        }
        if let Some(result) = self.lower_mcall_array_runtime(object, method, args, span)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_dispatch_early(object, method, args, span)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_nonident_receiver(object, method, args, span)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_function_call_apply_method(object, method, args, span)? {
            return Ok(result);
        }
        let ResolvedExpr::Ident(receiver_name) = object else {
            unreachable!()
        };
        self.lower_mcall_class_dispatch(receiver_name, object, method, args, span)
    }

    fn lower_static_generator_next(
        &mut self,
        object: &ResolvedExpr,
        args: &[ResolvedExpr],
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if let ResolvedExpr::Ident(name) = object
            && let Ok(local_id) = self.resolve_local(name)
            && let Some(binding) = self
                .ctx
                .facts
                .generator_method_iterator_bindings
                .get(&local_id)
                .cloned()
        {
            return Ok(Some(
                self.lower_generator_method_resume_with_state(&binding, args)?,
            ));
        }
        let (func_name, state_local, prelude, resume_args) = match object {
            ResolvedExpr::Ident(name) => {
                let local_id = self.resolve_local(name)?;
                let Some(binding) = self
                    .ctx
                    .facts
                    .generator_iterator_bindings
                    .get(&local_id)
                    .cloned()
                else {
                    return Ok(None);
                };
                let mut resume_args = binding.resume_args.clone();
                if let Some(arg) = args.first() {
                    resume_args.push(arg.clone());
                }
                if let Some(binding) = self
                    .ctx
                    .facts
                    .generator_iterator_bindings
                    .get_mut(&local_id)
                    && let Some(arg) = args.first()
                {
                    binding.resume_args.push(arg.clone());
                }
                (
                    binding.func_name,
                    binding.state_local,
                    Vec::new(),
                    resume_args,
                )
            }
            _ => {
                if !args.is_empty() {
                    return Ok(None);
                }
                let Some(func_name) =
                    crate::lowered::resolver::expr::facts::resolved_generator_function_call_name(
                        &self.ctx, object,
                    )
                else {
                    return Ok(None);
                };
                if !self
                    .ctx
                    .facts
                    .generator_function_steps
                    .contains_key(&func_name)
                {
                    return Ok(None);
                }
                let state_local = self.alloc_temp();
                (
                    func_name,
                    state_local,
                    vec![LoweredStmt::Let(
                        state_local,
                        LoweredExpr::Number(0, Span::generated("num")),
                        Span::generated("let_stmt"),
                    )],
                    Vec::new(),
                )
            }
        };
        if let Some(plan) = self
            .ctx
            .facts
            .generator_function_object_resume_plans
            .get(&func_name)
            .cloned()
        {
            return Ok(Some(self.lower_generator_object_resume_with_state(
                &plan,
                state_local,
                prelude,
                &resume_args,
            )?));
        }
        if !args.is_empty() {
            return Ok(None);
        }
        Ok(Some(self.lower_generator_resume_with_state(
            &func_name,
            state_local,
            prelude,
        )?))
    }

    fn resolved_expr_is_direct_generator_call(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::MethodCall { .. } => self.generator_method_func_id(expr).is_some(),
            ResolvedExpr::Call { callee, .. } => self.resolved_callee_is_generator(callee),
            _ => false,
        }
    }

    fn generator_method_func_id(&self, expr: &ResolvedExpr) -> Option<FuncId> {
        let ResolvedExpr::MethodCall { object, method, .. } = expr else {
            return None;
        };
        let ResolvedExpr::Ident(receiver_name) = object.as_ref() else {
            return None;
        };
        let Ok(receiver_local) = self.resolve_local(receiver_name) else {
            return None;
        };
        let method_id = self
            .ctx
            .classes
            .object_function_props
            .get(&receiver_local)
            .and_then(|props| props.get(&ObjectAccessorKey::Property(method.clone())))
            .copied()?;
        self.ctx
            .functions
            .generated_functions
            .iter()
            .any(|function| function.id == method_id && function.is_generator)
            .then_some(method_id)
    }

    fn lower_static_object_generator_return_next(
        &self,
        expr: &ResolvedExpr,
    ) -> Option<LoweredExpr> {
        let method_id = self.generator_method_func_id(expr)?;
        let ResolvedExpr::MethodCall { object, .. } = expr else {
            return None;
        };
        let ResolvedExpr::Ident(receiver_name) = object.as_ref() else {
            return None;
        };
        let receiver_local = self.resolve_local(receiver_name).ok()?;
        let function = self
            .ctx
            .functions
            .generated_functions
            .iter()
            .find(|function| function.id == method_id && function.is_generator)?;
        let receiver_param = function.params.first().copied();
        let value = match function.body.as_slice() {
            [] => LoweredExpr::Undefined(Span::generated("undefined")),
            [LoweredStmt::Yield(expr, _)] => {
                let value =
                    static_generator_bind_receiver(expr.clone(), receiver_param, receiver_local)?;
                return Some(Self::generator_next_result(value, false));
            }
            [LoweredStmt::Return(expr, _)] => static_generator_completion_value(expr)?,
            body => {
                if let Some(value) = static_generator_first_yield_value(body) {
                    let value =
                        static_generator_bind_receiver(value, receiver_param, receiver_local)?;
                    return Some(Self::generator_next_result(value, false));
                }
                static_generator_implicit_completion_value(body)?
            }
        };
        Some(Self::generator_next_result(value, true))
    }

    fn local_arrow_function_data_descriptor(&self, target: &str, key: &str) -> Option<LoweredExpr> {
        let value = match key {
            "name" => {
                let local = self.resolve_local(target).ok()?;
                LoweredExpr::String(
                    self.ctx
                        .facts
                        .function_metadata_name_locals
                        .get(&local)
                        .cloned()
                        .unwrap_or_else(|| target.to_owned()),
                    Span::generated("str"),
                )
            }
            "length" => {
                let local = self.resolve_local(target).ok()?;
                let closure = self.ctx.facts.arrow_locals.get(&local)?;
                let length = self
                    .ctx
                    .symbols
                    .function_signatures
                    .get(&closure.func_id)
                    .and_then(|signature| signature.metadata_length)?;
                LoweredExpr::Number(length as i32, Span::generated("num"))
            }
            _ => return None,
        };
        Some(LoweredExpr::ObjectNew {
            props: vec![
                ("value".to_owned(), value),
                (
                    "writable".to_owned(),
                    LoweredExpr::Bool(false, Span::generated("bool")),
                ),
                (
                    "enumerable".to_owned(),
                    LoweredExpr::Bool(false, Span::generated("bool")),
                ),
                (
                    "configurable".to_owned(),
                    LoweredExpr::Bool(true, Span::generated("bool")),
                ),
            ],
            non_enumerable: 0,
            span: Span::generated("function_descriptor"),
        })
    }

    fn static_object_accessor_descriptor(
        &self,
        target: &str,
        key: &str,
        span: Span,
    ) -> Option<LoweredExpr> {
        let local = self.resolve_local(target).ok()?;
        let accessor = self
            .ctx
            .classes
            .object_accessor_props
            .get(&local)?
            .get(&ObjectAccessorKey::Property(key.to_owned()))?;
        let mut props = Vec::new();
        if let Some(func_id) = accessor.get {
            props.push((
                "get".to_owned(),
                self.function_token_for_object_method(func_id, span)?,
            ));
        }
        if let Some(func_id) = accessor.set {
            props.push((
                "set".to_owned(),
                self.function_token_for_object_method(func_id, span)?,
            ));
        }
        props.extend([
            (
                "enumerable".to_owned(),
                LoweredExpr::Bool(true, Span::generated("bool")),
            ),
            (
                "configurable".to_owned(),
                LoweredExpr::Bool(true, Span::generated("bool")),
            ),
        ]);
        Some(LoweredExpr::ObjectNew {
            props,
            non_enumerable: 0,
            span: Span::generated("object_accessor_descriptor"),
        })
    }

    fn resolved_callee_is_generator(&self, callee: &ResolvedExpr) -> bool {
        let ResolvedExpr::Ident(name) = callee else {
            return false;
        };
        if self.ctx.facts.generator_function_names.contains(name) {
            return true;
        }
        let Ok(local) = self.resolve_local(name) else {
            return false;
        };
        let Some(closure) = self.ctx.facts.arrow_locals.get(&local) else {
            return false;
        };
        self.ctx
            .functions
            .generated_functions
            .iter()
            .any(|function| function.id == closure.func_id && function.is_generator)
    }

    fn lower_generator_resume_with_state(
        &mut self,
        func_name: &str,
        state_local: LocalId,
        prelude: Vec<LoweredStmt>,
    ) -> Result<LoweredExpr, Diagnostic> {
        let steps = self
            .ctx
            .facts
            .generator_function_steps
            .get(func_name)
            .cloned()
            .unwrap_or_default();
        let completion = self
            .ctx
            .facts
            .generator_function_completion_steps
            .get(func_name)
            .cloned()
            .unwrap_or_default();
        let result_local = self.alloc_temp();
        let snapshot_local = self.alloc_temp();
        let mut stmts = prelude;
        stmts.push(LoweredStmt::Let(
            result_local,
            Self::generator_next_result(LoweredExpr::Undefined(Span::generated("undefined")), true),
            Span::generated("let_stmt"),
        ));
        stmts.push(LoweredStmt::Let(
            snapshot_local,
            LoweredExpr::Local(state_local, Span::generated("local")),
            Span::generated("let_stmt"),
        ));
        for (index, step) in steps.iter().enumerate() {
            let mut then_body = Vec::new();
            for stmt in &step.statements {
                then_body.push(self.lower_stmt(stmt)?);
            }
            then_body.push(LoweredStmt::Assign(
                result_local,
                Self::generator_next_result(self.lower_expr(&step.value)?, false),
                Span::generated("assign"),
            ));
            then_body.push(LoweredStmt::Assign(
                state_local,
                LoweredExpr::Number((index + 1) as i32, Span::generated("num")),
                Span::generated("assign"),
            ));
            stmts.push(LoweredStmt::If {
                condition: Self::state_equals(snapshot_local, index),
                then_body,
                else_body: vec![],
                span: Span::generated("if_stmt"),
            });
        }
        let completed_state = steps.len() + 1;
        let mut completion_body = Vec::new();
        let mut completion_value = LoweredExpr::Undefined(Span::generated("undefined"));
        for stmt in &completion {
            if let ResolvedStmt::Return(expr) = stmt {
                completion_value = self.lower_expr(expr)?;
            } else {
                completion_body.push(self.lower_stmt(stmt)?);
            }
        }
        completion_body.push(LoweredStmt::Assign(
            result_local,
            Self::generator_next_result(completion_value, true),
            Span::generated("assign"),
        ));
        completion_body.push(LoweredStmt::Assign(
            state_local,
            LoweredExpr::Number(completed_state as i32, Span::generated("num")),
            Span::generated("assign"),
        ));
        stmts.push(LoweredStmt::If {
            condition: Self::state_equals(snapshot_local, steps.len()),
            then_body: completion_body,
            else_body: vec![],
            span: Span::generated("if_stmt"),
        });
        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(LoweredExpr::Local(result_local, Span::generated("local"))),
            span: Span::generated("block"),
        })
    }

    fn lower_generator_object_resume_with_state(
        &mut self,
        plan: &GeneratorObjectResumePlan,
        state_local: LocalId,
        prelude: Vec<LoweredStmt>,
        resume_args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        let result_local = self.alloc_temp();
        let snapshot_local = self.alloc_temp();
        let mut stmts = prelude;
        stmts.push(LoweredStmt::Let(
            result_local,
            Self::generator_next_result(LoweredExpr::Undefined(Span::generated("undefined")), true),
            Span::generated("let_stmt"),
        ));
        stmts.push(LoweredStmt::Let(
            snapshot_local,
            LoweredExpr::Local(state_local, Span::generated("local")),
            Span::generated("let_stmt"),
        ));

        for (index, value) in plan.yield_values.iter().enumerate() {
            let then_body = vec![
                LoweredStmt::Assign(
                    result_local,
                    Self::generator_next_result(self.lower_expr(value)?, false),
                    Span::generated("assign"),
                ),
                LoweredStmt::Assign(
                    state_local,
                    LoweredExpr::Number((index + 1) as i32, Span::generated("num")),
                    Span::generated("assign"),
                ),
            ];
            stmts.push(LoweredStmt::If {
                condition: Self::state_equals(snapshot_local, index),
                then_body,
                else_body: vec![],
                span: Span::generated("if_stmt"),
            });
        }

        let completed_state = plan.yield_values.len() + 1;
        let mut completion_body = Vec::new();
        if resume_args.len() >= plan.yield_values.len() {
            let resumed_props = replace_direct_computed_yield_keys(&plan.props, resume_args);
            completion_body.push(self.lower_stmt(&ResolvedStmt::Assign(
                plan.target.clone(),
                ResolvedExpr::Object(resumed_props),
            ))?);
        }
        completion_body.push(LoweredStmt::Assign(
            result_local,
            Self::generator_next_result(LoweredExpr::Undefined(Span::generated("undefined")), true),
            Span::generated("assign"),
        ));
        completion_body.push(LoweredStmt::Assign(
            state_local,
            LoweredExpr::Number(completed_state as i32, Span::generated("num")),
            Span::generated("assign"),
        ));
        stmts.push(LoweredStmt::If {
            condition: Self::state_equals(snapshot_local, plan.yield_values.len()),
            then_body: completion_body,
            else_body: vec![],
            span: Span::generated("if_stmt"),
        });

        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(LoweredExpr::Local(result_local, Span::generated("local"))),
            span: Span::generated("block"),
        })
    }

    fn lower_generator_method_resume_with_state(
        &mut self,
        binding: &GeneratorMethodIteratorBinding,
        _resume_args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        let function = self
            .ctx
            .functions
            .generated_functions
            .iter()
            .find(|function| function.id == binding.func_id && function.is_generator)
            .cloned()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "generator method iterator binding points at an unknown function"
                    .to_owned(),
                span: None,
                phase: None,
            })?;
        let result_local = self.alloc_temp();
        let snapshot_local = self.alloc_temp();
        let mut substitutions = HashMap::new();
        if let Some(receiver_param) = function.params.first().copied() {
            substitutions.insert(
                receiver_param,
                LoweredExpr::Local(binding.receiver_local, Span::generated("local")),
            );
        }
        for (param, arg) in function.params.iter().skip(1).copied().zip(&binding.args) {
            substitutions.insert(param, self.lower_expr(arg)?);
        }

        let mut stmts = vec![
            LoweredStmt::Let(
                result_local,
                Self::generator_next_result(
                    LoweredExpr::Undefined(Span::generated("undefined")),
                    true,
                ),
                Span::generated("let_stmt"),
            ),
            LoweredStmt::Let(
                snapshot_local,
                LoweredExpr::Local(binding.state_local, Span::generated("local")),
                Span::generated("let_stmt"),
            ),
        ];
        if let Some(value) = static_generator_first_yield_value(&function.body)
            && let Some(value) = static_generator_bind_locals(value, &substitutions)
        {
            stmts.push(LoweredStmt::If {
                condition: Self::state_equals(snapshot_local, 0),
                then_body: vec![
                    LoweredStmt::Assign(
                        result_local,
                        Self::generator_next_result(value, false),
                        Span::generated("assign"),
                    ),
                    LoweredStmt::Assign(
                        binding.state_local,
                        LoweredExpr::Number(1, Span::generated("num")),
                        Span::generated("assign"),
                    ),
                ],
                else_body: vec![],
                span: Span::generated("if_stmt"),
            });
        }
        let completion_value = static_generator_implicit_completion_value(&function.body)
            .and_then(|value| static_generator_bind_locals(value, &substitutions))
            .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undefined")));
        stmts.push(LoweredStmt::If {
            condition: Self::state_equals(snapshot_local, 1),
            then_body: vec![
                LoweredStmt::Assign(
                    result_local,
                    Self::generator_next_result(completion_value, true),
                    Span::generated("assign"),
                ),
                LoweredStmt::Assign(
                    binding.state_local,
                    LoweredExpr::Number(2, Span::generated("num")),
                    Span::generated("assign"),
                ),
            ],
            else_body: vec![],
            span: Span::generated("if_stmt"),
        });
        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(LoweredExpr::Local(result_local, Span::generated("local"))),
            span: Span::generated("block"),
        })
    }

    fn generator_next_result(value: LoweredExpr, done: bool) -> LoweredExpr {
        LoweredExpr::ObjectNew {
            props: vec![
                ("value".to_owned(), value),
                (
                    "done".to_owned(),
                    LoweredExpr::Bool(done, Span::generated("bool")),
                ),
            ],
            non_enumerable: 0,
            span: Span::generated("object"),
        }
    }

    fn state_equals(state_local: LocalId, state: usize) -> LoweredExpr {
        LoweredExpr::Binary {
            left: Box::new(LoweredExpr::Local(state_local, Span::generated("local"))),
            op: LoweredBinaryOp::StrictEqual,
            right: Box::new(LoweredExpr::Number(state as i32, Span::generated("num"))),
            span: Span::generated("binary"),
        }
    }

    fn lower_function_call_apply_method(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if method != "call" && method != "apply" {
            return Ok(None);
        }
        let ResolvedExpr::Ident(func_name) = object else {
            return Ok(None);
        };
        let Ok(func_id) = self.resolve_func(func_name) else {
            return Ok(None);
        };
        let receiver = match args.first() {
            Some(receiver) => self.lower_expr(receiver)?,
            None => LoweredExpr::Undefined(Span::generated("undef")),
        };
        let explicit_args = if method == "call" {
            args.iter().skip(1).cloned().collect::<Vec<_>>()
        } else {
            match args.get(1) {
                None | Some(ResolvedExpr::Undefined | ResolvedExpr::Null) => Vec::new(),
                Some(ResolvedExpr::Array(elements)) => elements
                    .iter()
                    .map(|element| match element {
                        ResolvedArrayElement::Present(expr) => expr.clone(),
                        ResolvedArrayElement::Hole => ResolvedExpr::Undefined,
                    })
                    .collect(),
                Some(ResolvedExpr::Ident(name)) => {
                    vec![ResolvedExpr::Spread(Box::new(ResolvedExpr::Ident(
                        name.clone(),
                    )))]
                }
                Some(_) => {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-458: Function.prototype.apply currently supports array literals, dense array locals, null, or undefined argArray".to_owned(),
                        span: Some(span),
                        phase: None,
                    });
                }
            }
        };
        let lowered_args = self.lower_function_call_args(func_id, receiver, &explicit_args)?;
        Ok(Some(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: lowered_args,
            span: Span::generated("call"),
        }))
    }

    /// Helper for lower_method_call_expr: early-return checks (array push.call,
    /// Array.from, prototype.map.call, Set.add.call, bigint runtime, private methods).
    fn lower_mcall_early(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if method == "call" && is_array_prototype_push_expr(object) {
            let Some((receiver, values)) = args.split_first() else {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: "Array.prototype.push.call expects a receiver argument".to_owned(),
                    span: Some(span),

                    phase: None,
                });
            };
            let mut lowered_args = vec![self.lower_expr(receiver)?];
            lowered_args.extend(
                values
                    .iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayPushMany,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        if is_array_from_call_receiver(object, method) {
            return Some(self.lower_array_from_call(args, span)).transpose();
        }
        if is_array_prototype_map_call_receiver(object, method) {
            return Some(self.lower_array_prototype_map_call(args, span)).transpose();
        }
        if is_array_prototype_every_some_call_receiver(object, method) {
            return Some(self.lower_array_prototype_every_some_call(args, object, span))
                .transpose();
        }
        if method == "call" && is_set_prototype_property_expr(object, "originalAdd") {
            return Some(self.lower_native_set_add_call(args, span)).transpose();
        }
        if method == "call"
            && let ResolvedExpr::Ident(name) = object
            && let Ok(local_id) = self.resolve_local(name)
            && self.ctx.facts.native_set_add_locals.contains(&local_id)
        {
            return Some(self.lower_native_set_add_call(args, span)).transpose();
        }
        if matches!(
            object,
            ResolvedExpr::Ident(name) if name == "__ts2wasm_bigint_runtime"
        ) && let Some(intrinsic) = bigint_runtime_fn_intrinsic(method)
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: self.lower_call_args(args)?,

                span: Span::generated("runtime_call"),
            }));
        }
        if let Some(formatted) = static_number_format_method_call(object, method, args) {
            return Ok(Some(LoweredExpr::String(
                formatted,
                Span::generated("number_format"),
            )));
        }
        if method.starts_with('#') {
            if let Some(method_id) = self.current_static_private_method_id(method) {
                if self.is_same_class_static_private_receiver(object) {
                    let lowered_args = args
                        .iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(Some(LoweredExpr::Call {
                        kind: FunctionCallKind::User(method_id),
                        args: lowered_args,

                        span: Span::generated("call"),
                    }));
                }
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-255: static private method `{method}` calls are currently supported only as `this.{method}(...)` inside static methods or `Class.{method}(...)` inside the declaring class"
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let method_id = self
                .current_private_method_id(method)
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-255: private method `{method}` is not declared in this class"
                    ),
                    span: Some(span),

                    phase: None,
                })?;
            let receiver = if matches!(object, ResolvedExpr::This { .. }) {
                LoweredExpr::Local(self.resolve_local("this")?, Span::generated("local"))
            } else {
                let class_name = self.ctx.classes.current_class.clone().ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-255: private method `{method}` call requires declaring class context"
                    ),
                    span: Some(span),

                    phase: None,
                })?;
                let brand = self.private_brand_for_class(&class_name, Some(span))?;
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::PrivateBrandCheck,
                    args: vec![
                        self.lower_expr(object)?,
                        LoweredExpr::Number(brand as i32, Span::generated("num")),
                    ],

                    span: Span::generated("runtime_call"),
                }
            };
            let mut lowered_args = vec![receiver];
            lowered_args.extend(
                args.iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            return Ok(Some(LoweredExpr::Call {
                kind: FunctionCallKind::User(method_id),
                args: lowered_args,

                span: Span::generated("call"),
            }));
        }
        Ok(None)
    }

    fn lower_mcall_intl_number_format(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if matches!(object, ResolvedExpr::Ident(name) if name == "Intl") && method == "NumberFormat"
        {
            return Ok(Some(self.lower_intl_number_format_constructor(args)?));
        }
        if self.is_intl_number_format_expr(object) && is_intl_number_format_method(method) {
            let options = self.intl_number_format_options_for_expr(object);
            return Ok(Some(self.lower_intl_number_format_method(
                method,
                args,
                options.as_ref(),
            )?));
        }
        Ok(None)
    }

    fn lower_mcall_intl_duration_format(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if matches!(object, ResolvedExpr::Ident(name) if name == "Intl")
            && method == "DurationFormat"
        {
            return Ok(Some(self.lower_intl_duration_format_constructor(args)?));
        }
        if self.is_intl_duration_format_expr(object) && is_intl_duration_format_method(method) {
            return Ok(Some(self.lower_intl_duration_format_method(method, args)?));
        }
        Ok(None)
    }

    fn lower_mcall_intl_list_format(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if matches!(object, ResolvedExpr::Ident(name) if name == "Intl") && method == "ListFormat" {
            return Ok(Some(self.lower_intl_list_format_constructor(args)?));
        }
        if self.is_intl_list_format_expr(object) && is_intl_list_format_method(method) {
            return Ok(Some(self.lower_intl_list_format_method(method, args)?));
        }
        Ok(None)
    }

    /// Try to lower test/exec on an Ident local that holds a regexp literal.
    /// The existing regexp_test_runtime / regexp_exec_runtime helpers only match
    /// String (inline literal) or New (constructor) patterns, so we must resolve
    /// the Ident to its local and check regexp_literal_locals here.
    fn try_lower_regexp_ident_local(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        let ResolvedExpr::Ident(name) = object else {
            return Ok(None);
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return Ok(None);
        };
        if !self.ctx.facts.regexp_literal_locals.contains(&local_id) {
            return Ok(None);
        }
        if method != "test" && method != "exec" {
            return Ok(None);
        }
        if args.len() > 1 {
            return Err(Diagnostic {
                code: DiagCode::ArityMismatch,
                message: format!(
                    "RegExp.prototype.{} expects at most 1 argument, got {}",
                    method,
                    args.len()
                ),
                span: Some(span),
                phase: None,
            });
        }
        let arg = args
            .first()
            .cloned()
            .unwrap_or(ResolvedExpr::String("undefined".to_owned()));
        let intrinsic = if method == "test" {
            RuntimeFn::RegExpTest
        } else {
            RuntimeFn::RegExpMatch
        };
        let lowered_arg = self.lower_expr(&arg)?;
        Ok(Some(LoweredExpr::RuntimeCall {
            intrinsic,
            args: vec![
                LoweredExpr::Local(local_id, Span::generated("local")),
                lowered_arg,
            ],
            span: Span::generated("runtime_call"),
        }))
    }

    fn lower_mcall_arraybuffer(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if matches!(object, ResolvedExpr::Ident(name) if name == "ArrayBuffer")
            && method == "isView"
        {
            let is_view = args
                .first()
                .and_then(|arg| self.infer_class_for_expr(arg))
                .is_some_and(|class_name| {
                    class_name == "DataView" || is_typed_array_class(&class_name)
                });
            return Ok(Some(LoweredExpr::Bool(is_view, Span::generated("bool"))));
        }
        if matches!(
            self.infer_class_for_expr(object).as_deref(),
            Some("ArrayBuffer" | "SharedArrayBuffer")
        ) && method == "transfer"
        {
            let new_len = args
                .first()
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Number(0, Span::generated("num")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayBufferNew,
                args: vec![new_len],
                span: Span::generated("runtime_call"),
            }));
        }
        Ok(None)
    }

    /// TypedArray static methods: TypedArray.from(source) → TypedArrayFromArray
    fn lower_mcall_typed_array(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if let ResolvedExpr::Ident(name) = object
            && is_typed_array_class(name)
            && method == "from"
        {
            let source = match args.first() {
                Some(arg) => self.lower_expr(arg)?,
                None => LoweredExpr::ArrayNew {
                    elements: Vec::new(),
                    span: Span::generated("typed_array_from"),
                },
            };
            if args.len() > 1 {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "TypedArray.from with mapFn/thisArg is not supported".to_owned(),
                    span: Some(Span::generated("typed_array_from")),
                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::TypedArrayFromArray,
                args: vec![source],
                span: Span::generated("typed_array_from"),
            }));
        }
        Ok(None)
    }

    fn lower_mcall_intl_date_time_format(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if matches!(object, ResolvedExpr::Ident(name) if name == "Intl")
            && method == "DateTimeFormat"
        {
            return Ok(Some(self.lower_intl_date_time_format_constructor(args)?));
        }
        if self.is_intl_date_time_format_expr(object) && is_intl_date_time_format_method(method) {
            let options = self.intl_date_time_format_options_for_expr(object);
            return Ok(Some(self.lower_intl_date_time_format_method(
                method,
                args,
                options.as_ref(),
            )?));
        }
        Ok(None)
    }

    /// Helper for lower_method_call_expr: JSON.stringify, Date.now, RegExp methods,
    /// Date getTime, Date getTimezoneOffset.
    fn lower_mcall_json_date_regexp(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if is_json_static_call(object, method) {
            validate_json_stringify_args(
                args,
                span,
                &self.ctx.symbols.function_ids,
                &self.ctx.symbols.function_signatures,
            )?;
            let mut lowered_args = Vec::with_capacity(3);
            let value = if let (ResolvedExpr::Object(props), Some(replacer_keys)) = (
                &args[0],
                json_stringify_replacer_keys(args, &self.ctx.symbols.function_ids),
            ) {
                let mut lowered_props = Vec::new();
                for allowed_key in replacer_keys {
                    if lowered_props
                        .iter()
                        .any(|(key, _): &(String, LoweredExpr)| key == &allowed_key)
                    {
                        continue;
                    }
                    if let Some(prop) = props
                        .iter()
                        .rev()
                        .find(|prop| prop.static_key() == Some(allowed_key.as_str()))
                    {
                        lowered_props.push((allowed_key.clone(), self.lower_expr(prop.value())?));
                    }
                }
                LoweredExpr::ObjectNew {
                    props: lowered_props,
                    non_enumerable: 0,
                    span: Span::generated("object_new"),
                }
            } else {
                self.lower_expr(&args[0])?
            };
            lowered_args.push(value);
            lowered_args.push(match args.get(1) {
                Some(ResolvedExpr::Array(_)) => LoweredExpr::Null(Span::generated("null")),
                Some(replacer) => {
                    if let Some(func_id) = json_stringify_function_replacer_id(
                        replacer,
                        &self.ctx.symbols.function_ids,
                    ) {
                        LoweredExpr::Number(func_id.0 as i32, Span::generated("num"))
                    } else {
                        self.lower_expr(replacer)?
                    }
                }
                None => LoweredExpr::Undefined(Span::generated("undef")),
            });
            lowered_args.push(match args.get(2) {
                Some(space)
                    if should_ignore_json_stringify_space(
                        space,
                        &self.ctx.symbols.function_ids,
                    ) =>
                {
                    LoweredExpr::Undefined(Span::generated("undef"))
                }
                Some(space) => {
                    if let Some(boxed_space) = json_stringify_boxed_space_value(space) {
                        self.lower_expr(boxed_space)?
                    } else {
                        self.lower_expr(space)?
                    }
                }
                None => LoweredExpr::Undefined(Span::generated("undef")),
            });
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::JsonStringify,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "JSON") && method == "parse" {
            if args.is_empty() || args.len() > 2 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!("JSON.parse expects 1 or 2 arguments, got {}", args.len()),
                    span: Some(span),

                    phase: None,
                });
            }
            let mut lowered_args = Vec::with_capacity(2);
            lowered_args.push(self.lower_expr(&args[0])?);
            lowered_args.push(match args.get(1) {
                Some(reviver) => {
                    if let Some(func_id) =
                        json_stringify_function_replacer_id(reviver, &self.ctx.symbols.function_ids)
                    {
                        LoweredExpr::Number(func_id.0 as i32, Span::generated("num"))
                    } else {
                        self.lower_expr(reviver)?
                    }
                }
                None => LoweredExpr::Undefined(Span::generated("undef")),
            });
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::JsonParse,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        if is_date_now_live_time_call(object, method) {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateNow,
                args: vec![],

                span: Span::generated("runtime_call"),
            }));
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "Date") && method == "parse" {
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!("Date.parse expects 1 argument, got {}", args.len()),
                    span: Some(span),
                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateParse,
                args: vec![self.lower_expr(&args[0])?],

                span: Span::generated("runtime_call"),
            }));
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "Date") && method == "UTC" {
            if args.is_empty() || args.len() > 7 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!("Date.UTC expects 1 to 7 arguments, got {}", args.len()),
                    span: Some(span),
                    phase: None,
                });
            }
            let mut lowered_args = Vec::with_capacity(7);
            for (idx, arg) in args.iter().enumerate() {
                if idx == 7 {
                    break;
                }
                let lowered_arg = self.lower_expr(arg)?;
                // Apply ToNumber coercion per spec: year → ToNumber(year), month → ToNumber(month), etc.
                // Wrap each arg with unary plus (+arg) which emits $primitive_to_number_for_equality.
                lowered_args.push(LoweredExpr::Unary {
                    op: LoweredUnaryOp::Plus,
                    expr: Box::new(lowered_arg),
                    span: Span::generated("date_utc_coerce"),
                });
            }
            while lowered_args.len() < 7 {
                let default = if lowered_args.len() == 2 { 1 } else { 0 };
                lowered_args.push(LoweredExpr::Number(
                    default,
                    Span::generated("date_utc_default"),
                ));
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateUTC,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        if self.is_unsupported_regexp_compile_receiver(object, method) {
            return Err(unsupported_regexp_compile_diagnostic(Some(span)));
        }
        if self.is_object_key_enumeration_leak(object, method, args) {
            return Err(private_storage_observable_access_diagnostic(Some(span)));
        }
        if method == "matchAll" {
            return Ok(Some(
                self.lower_string_match_all_literal(object, args, span)?,
            ));
        }
        if let Some(result) = self.try_lower_regexp_ident_local(object, method, args, span)? {
            return Ok(Some(result));
        }
        if let Some(regexp_args) = regexp_test_runtime(&self.ctx, object, method, args, span)? {
            let lowered_args = regexp_args
                .iter()
                .map(|e| self.lower_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::RegExpTest,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        if let Some(regexp_args) = regexp_exec_runtime(&self.ctx, object, method, args, span)? {
            let lowered_args = regexp_args
                .iter()
                .map(|e| self.lower_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::RegExpMatch,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        if let Some(regexp_args) =
            regexp_string_match_runtime(&self.ctx, object, method, args, span)?
        {
            let lowered_args = regexp_args
                .iter()
                .map(|e| self.lower_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: if method == "search" {
                    RuntimeFn::RegExpSearch
                } else {
                    RuntimeFn::RegExpMatch
                },
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        // RegExp.prototype.toString for literal-backed RegExp
        // Function.prototype.toString for named function identifiers
        if method == "toString" {
            match object {
                ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
                    // String.prototype.toString returns the string itself
                    return Ok(Some(self.lower_expr(object)?));
                }
                ResolvedExpr::New { class_name, .. } if class_name == "RegExp" => {
                    // Lower the new RegExp to string representation, toString returns it
                    return Ok(Some(self.lower_expr(object)?));
                }
                ResolvedExpr::Ident(name) => {
                    if let Ok(local) = self.resolve_local(name)
                        && let Some(closure) = self.ctx.facts.arrow_locals.get(&local)
                    {
                        let body = self
                            .ctx
                            .function_sources
                            .get(&closure.func_id)
                            .filter(|source| !source.is_empty())
                            .cloned()
                            .unwrap_or_else(|| {
                                let metadata_name = self
                                    .ctx
                                    .facts
                                    .function_metadata_name_locals
                                    .get(&local)
                                    .map(String::as_str)
                                    .unwrap_or(name);
                                format!("function {metadata_name}() {{ [native code] }}")
                            });
                        return Ok(Some(LoweredExpr::String(body, Span::generated("str"))));
                    }
                    if self.is_function_identifier(object) {
                        let body = if let Ok(func_id) = self.resolve_func(name) {
                            self.ctx
                                .function_sources
                                .get(&func_id)
                                .filter(|s| !s.is_empty())
                                .cloned()
                                .unwrap_or_else(|| {
                                    format!("function {}() {{ [native code] }}", name)
                                })
                        } else {
                            format!("function {}() {{ [native code] }}", name)
                        };
                        return Ok(Some(LoweredExpr::String(body, Span::generated("str"))));
                    }
                }
                ResolvedExpr::ArrowFn {
                    params,
                    source_text,
                    ..
                } => {
                    if source_text.is_empty() {
                        let params_str = params.join(", ");
                        return Ok(Some(LoweredExpr::String(
                            format!("({}) => {{ [native code] }}", params_str),
                            Span::generated("str"),
                        )));
                    }
                    return Ok(Some(LoweredExpr::String(
                        source_text.clone(),
                        Span::generated("str"),
                    )));
                }
                ResolvedExpr::FunctionExpr {
                    name, source_text, ..
                } => {
                    if source_text.is_empty() {
                        let name_part = if name.is_empty() {
                            String::new()
                        } else {
                            format!("{} ", name)
                        };
                        return Ok(Some(LoweredExpr::String(
                            format!("function {}() {{ [native code] }}", name_part),
                            Span::generated("str"),
                        )));
                    }
                    return Ok(Some(LoweredExpr::String(
                        source_text.clone(),
                        Span::generated("str"),
                    )));
                }
                _ => {}
            }
        }
        if matches!(method, "getTime" | "valueOf") && self.is_date_receiver(object) {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateGetTime,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setTime" && self.is_date_receiver(object) {
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 argument, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetTime,
                args: vec![self.lower_expr(object)?, self.lower_expr(&args[0])?],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setUTCFullYear" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 3 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 3 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let month = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            let day = args
                .get(2)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetUTCFullYear,
                args: vec![
                    self.lower_expr(object)?,
                    self.lower_expr(&args[0])?,
                    month,
                    day,
                ],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setUTCMonth" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 2 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 2 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let day = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetUTCMonth,
                args: vec![self.lower_expr(object)?, self.lower_expr(&args[0])?, day],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setUTCDate" && self.is_date_receiver(object) {
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 argument, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetUTCDate,
                args: vec![self.lower_expr(object)?, self.lower_expr(&args[0])?],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setUTCHours" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 4 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 4 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let minutes = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            let seconds = args
                .get(2)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            let ms = args
                .get(3)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetUTCHours,
                args: vec![
                    self.lower_expr(object)?,
                    self.lower_expr(&args[0])?,
                    minutes,
                    seconds,
                    ms,
                ],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setUTCMinutes" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 3 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 3 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let seconds = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            let ms = args
                .get(2)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetUTCMinutes,
                args: vec![
                    self.lower_expr(object)?,
                    self.lower_expr(&args[0])?,
                    seconds,
                    ms,
                ],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setUTCSeconds" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 2 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 2 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let ms = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetUTCSeconds,
                args: vec![self.lower_expr(object)?, self.lower_expr(&args[0])?, ms],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setUTCMilliseconds" && self.is_date_receiver(object) {
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 argument, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetUTCMilliseconds,
                args: vec![self.lower_expr(object)?, self.lower_expr(&args[0])?],

                span: Span::generated("runtime_call"),
            }));
        }
        // --- Local-time setters ---
        if method == "setFullYear" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 3 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 3 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let month = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            let day = args
                .get(2)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetFullYear,
                args: vec![
                    self.lower_expr(object)?,
                    self.lower_expr(&args[0])?,
                    month,
                    day,
                ],
                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setMonth" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 2 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 2 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let day = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetMonth,
                args: vec![self.lower_expr(object)?, self.lower_expr(&args[0])?, day],
                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setDate" && self.is_date_receiver(object) {
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 argument, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetDate,
                args: vec![self.lower_expr(object)?, self.lower_expr(&args[0])?],
                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setHours" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 4 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 4 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let minutes = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            let seconds = args
                .get(2)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            let ms = args
                .get(3)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetHours,
                args: vec![
                    self.lower_expr(object)?,
                    self.lower_expr(&args[0])?,
                    minutes,
                    seconds,
                    ms,
                ],
                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setMinutes" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 3 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 3 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let seconds = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            let ms = args
                .get(2)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetMinutes,
                args: vec![
                    self.lower_expr(object)?,
                    self.lower_expr(&args[0])?,
                    seconds,
                    ms,
                ],
                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setSeconds" && self.is_date_receiver(object) {
            if args.is_empty() || args.len() > 2 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 to 2 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let ms = args
                .get(1)
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetSeconds,
                args: vec![self.lower_expr(object)?, self.lower_expr(&args[0])?, ms],
                span: Span::generated("runtime_call"),
            }));
        }
        if method == "setMilliseconds" && self.is_date_receiver(object) {
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 argument, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetMilliseconds,
                args: vec![self.lower_expr(object)?, self.lower_expr(&args[0])?],
                span: Span::generated("runtime_call"),
            }));
        }
        if method == "getTimezoneOffset" && self.is_date_receiver(object) {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateGetTimezoneOffset,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }
        Ok(None)
    }

    /// Helper for lower_method_call_expr: Date local-time getters, Date getYear,
    /// Date UTC getters, Date toString/toISOString, and String builtins.
    fn lower_mcall_date_string(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if is_local_tz_date_method(method) && self.is_date_receiver(object) {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let field_index: i32 = match method {
                "getFullYear" => 0,
                "getMonth" => 1,
                "getDate" => 2,
                "getHours" => 3,
                "getMinutes" => 4,
                "getSeconds" => 5,
                "getMilliseconds" => 6,
                "getDay" => 7,
                _ => unreachable!(),
            };
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateGetLocalTimeField,
                args: vec![
                    self.lower_expr(object)?,
                    LoweredExpr::Number(field_index, Span::generated("num")),
                ],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "getYear" && is_static_date_constructor_expr(object) {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::Binary {
                left: Box::new(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::DateGetUtcFullYear,
                    args: vec![self.lower_expr(object)?],

                    span: Span::generated("runtime_call"),
                }),
                op: LoweredBinaryOp::Subtract,
                right: Box::new(LoweredExpr::Number(1900, Span::generated("num"))),
                span: Span::generated("binary"),
            }));
        }
        if method == "getYear"
            && crate::lowered::resolver::expr::facts::is_invalid_date_expr(&self.ctx, object)
        {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::Number(0, Span::generated("num"))));
        }
        if method == "getYear" && self.is_date_receiver(object) {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::Binary {
                left: Box::new(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::DateGetLocalTimeField,
                    args: vec![
                        self.lower_expr(object)?,
                        LoweredExpr::Number(0, Span::generated("num")),
                    ],

                    span: Span::generated("runtime_call"),
                }),
                op: LoweredBinaryOp::Subtract,
                right: Box::new(LoweredExpr::Number(1900, Span::generated("num"))),
                span: Span::generated("binary"),
            }));
        }
        if method == "getYear" && self.is_date_receiver(object) {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),
                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::Binary {
                left: Box::new(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::DateGetUtcFullYear,
                    args: vec![self.lower_expr(object)?],
                    span: Span::generated("runtime_call"),
                }),
                op: LoweredBinaryOp::Subtract,
                right: Box::new(LoweredExpr::Number(1900, Span::generated("num"))),
                span: Span::generated("binary"),
            }));
        }
        if method == "setYear" && self.is_date_receiver(object) {
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 1 argument, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            // B.2.4.2: if 0 ≤ ToInteger(year) ≤ 99, add 1900
            let year_arg = match &args[0] {
                ResolvedExpr::Number(n) if *n >= 0 && *n <= 99 => {
                    LoweredExpr::Number(*n + 1900, Span::generated("num"))
                }
                ResolvedExpr::DecimalNumber(s) => {
                    if let Ok(n) = s.parse::<i32>() {
                        if n >= 0 && n <= 99 {
                            LoweredExpr::Number(n + 1900, Span::generated("num"))
                        } else {
                            self.lower_expr(&args[0])?
                        }
                    } else {
                        self.lower_expr(&args[0])?
                    }
                }
                _ => self.lower_expr(&args[0])?,
            };
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateSetFullYear,
                args: vec![
                    self.lower_expr(object)?,
                    year_arg,
                    LoweredExpr::Number(0, Span::generated("num")),
                    LoweredExpr::Number(1, Span::generated("num")),
                ],

                span: Span::generated("runtime_call"),
            }));
        }
        if self.is_date_receiver(object) && matches!(method, "toGMTString" | "toUTCString") {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateToString,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "toString" && self.is_date_receiver(object) {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateToString,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }
        if self.is_date_receiver(object)
            && matches!(
                method,
                "getUTCMilliseconds"
                    | "getUTCSeconds"
                    | "getUTCMinutes"
                    | "getUTCHours"
                    | "getUTCDay"
                    | "getUTCDate"
                    | "getUTCMonth"
                    | "getUTCFullYear"
            )
        {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let intrinsic: RuntimeFn = match method {
                "getUTCMilliseconds" => RuntimeFn::DateGetUtcMilliseconds,
                "getUTCSeconds" => RuntimeFn::DateGetUtcSeconds,
                "getUTCMinutes" => RuntimeFn::DateGetUtcMinutes,
                "getUTCHours" => RuntimeFn::DateGetUtcHours,
                "getUTCDay" => RuntimeFn::DateGetUtcDay,
                "getUTCDate" => RuntimeFn::DateGetUtcDate,
                "getUTCMonth" => RuntimeFn::DateGetUtcMonth,
                "getUTCFullYear" => RuntimeFn::DateGetUtcFullYear,
                _ => unreachable!(),
            };
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }
        if self.is_date_receiver(object) && method == "toDateString" {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateToDateString,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }
        if self.is_date_receiver(object) && method == "toTimeString" {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateToTimeString,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }
        if self.is_date_receiver(object) && matches!(method, "toISOString" | "toJSON") {
            if !args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "Date.prototype.{method} expects 0 arguments, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateToISOString,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "normalize" {
            return Ok(Some(
                self.lower_static_ascii_string_normalize(object, args, span)?,
            ));
        }
        if matches!(object, ResolvedExpr::String(_)) {
            if is_html_wrapper_string_method(method) {
                let lowered_object = self.lower_expr(object)?;
                let mut lowered_args = Vec::new();
                for arg in args {
                    lowered_args.push(self.lower_expr(arg)?);
                }
                return Ok(Some(lower_html_wrapper_string_method(
                    method,
                    lowered_object,
                    lowered_args,
                    span,
                )?));
            }
            if let Some(diagnostic) = unsupported_annex_b_string_method(method, span) {
                return Err(diagnostic);
            }
            if let Some(intrinsic) = resolve_method_to_runtime_fn(object, method) {
                let mut lowered_args = vec![self.lower_expr(object)?];
                lowered_args.extend(
                    args.iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?,
                );
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: lowered_args,

                    span: Span::generated("runtime_call"),
                }));
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("String.prototype.{method} is not supported in this milestone"),
                span: Some(span),

                phase: None,
            });
        }
        Ok(None)
    }

    fn lower_static_ascii_string_normalize(
        &mut self,
        object: &ResolvedExpr,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if args.len() > 1 {
            return Err(Diagnostic {
                code: DiagCode::ArityMismatch,
                message: format!(
                    "String.prototype.normalize expects at most 1 argument, got {}",
                    args.len()
                ),
                span: Some(span),

                phase: None,
            });
        }
        if let Some(form) = args.first()
            && !matches!(form, ResolvedExpr::Undefined)
        {
            let Some(form_value) =
                crate::lowered::resolver::string::resolved_expr_static_string_value(
                    &self.ctx, form,
                )
            else {
                // Dynamic form — delegate to runtime
                return self.emit_normalize_runtime_call(object, args, span);
            };
            if !matches!(form_value.as_str(), "NFC" | "NFD" | "NFKC" | "NFKD") {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-460: String.prototype.normalize form `{form_value}` is not supported in this milestone"
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
        }
        let Some(value) =
            crate::lowered::resolver::string::resolved_expr_static_string_value(&self.ctx, object)
        else {
            // Dynamic receiver — delegate to runtime
            return self.emit_normalize_runtime_call(object, args, span);
        };
        if !value.is_ascii() {
            // Non-ASCII receiver — delegate to runtime
            return self.emit_normalize_runtime_call(object, args, span);
        }
        self.lower_expr(object)
    }

    /// Emit a RuntimeCall to `StringNormalize` for dynamic or non-ASCII cases.
    fn emit_normalize_runtime_call(
        &mut self,
        object: &ResolvedExpr,
        args: &[ResolvedExpr],
        _span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let mut lowered_args = vec![self.lower_expr(object)?];
        if let Some(form) = args.first() {
            lowered_args.push(self.lower_expr(form)?);
        } else {
            lowered_args.push(LoweredExpr::Undefined(Span::generated("undefined")));
        }
        Ok(LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::StringNormalize,
            args: lowered_args,

            span: Span::generated("runtime_call"),
        })
    }

    /// Helper for lower_method_call_expr: array method dispatch (indexOf, includes,
    /// concat, identity-arrow optimizations) and runtime_fn routing (push, Math, etc.).
    fn lower_mcall_array_runtime(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if let Some(result) = self.lower_static_proxy_object_call(object, method, args, span)? {
            return Ok(Some(result));
        }
        if let Some(result) = self.lower_object_prototype_dispatch(object, method, args, span)? {
            return Ok(Some(result));
        }
        // ProxyDispatch: compile-time proxy trap dispatch for Reflect.* and Object.*
        if matches!(object, ResolvedExpr::Ident(name) if name == "Reflect" || name == "Object")
            && !args.is_empty()
            && let Some(proxy) = crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(
                &self.ctx, &args[0],
            )
            && let Some(trap) = Self::reflect_or_object_method_to_proxy_trap(method)
        {
            let rest_args = args[1..].to_vec();
            return Ok(Some(
                self.lower_proxy_trap_call(proxy, trap, rest_args, span)?,
            ));
        }
        if (method == "indexOf" || method == "includes")
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
            && !args.is_empty()
        {
            let mut lowered_args = vec![self.lower_expr(object)?, self.lower_expr(&args[0])?];
            // Pass fromIndex if provided, otherwise default to 0
            if args.len() > 1 {
                lowered_args.push(self.lower_expr(&args[1])?);
            } else {
                lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: if method == "indexOf" {
                    RuntimeFn::ArrayIndexOf
                } else {
                    RuntimeFn::ArrayIncludes
                },
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "concat"
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
        {
            let mut lowered_args = vec![self.lower_expr(object)?];
            lowered_args.extend(
                args.iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayConcat,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        if method == "push"
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
            && let [ResolvedExpr::Spread(spread_expr)] = args
        {
            return Ok(Some(self.lower_array_push_single_spread_arg(
                object,
                spread_expr.as_ref(),
            )?));
        }
        if method == "at"
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
        {
            let index = if let Some(arg) = args.first() {
                self.lower_expr(arg)?
            } else {
                LoweredExpr::Undefined(Span::generated("undef"))
            };
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayAt,
                args: vec![self.lower_expr(object)?, index],
                span: Span::generated("runtime_call"),
            }));
        }
        if method == "lastIndexOf"
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
        {
            let search = if let Some(arg) = args.first() {
                self.lower_expr(arg)?
            } else {
                LoweredExpr::Undefined(Span::generated("undef"))
            };
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayLastIndexOf,
                args: vec![self.lower_expr(object)?, search],
                span: Span::generated("runtime_call"),
            }));
        }
        if crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
            && matches!(method, "copyWithin" | "fill" | "slice" | "subarray")
        {
            let receiver = self.lower_expr(object)?;
            let intrinsic = if method == "subarray" {
                RuntimeFn::ArraySlice
            } else {
                collection_method_runtime_fn_arg(method).expect("array method runtime")
            };
            let mut lowered_args = vec![receiver.clone()];
            match method {
                "copyWithin" => {
                    for arg in args.iter().take(3) {
                        lowered_args.push(self.lower_expr(arg)?);
                    }
                    while lowered_args.len() < 4 {
                        lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                    }
                }
                "fill" => {
                    // fill(value, start?, end?) — pad missing start/end with Undefined
                    for arg in args.iter().take(3) {
                        lowered_args.push(self.lower_expr(arg)?);
                    }
                    while lowered_args.len() < 4 {
                        lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                    }
                }
                "slice" | "subarray" => {
                    for arg in args.iter().take(2) {
                        lowered_args.push(self.lower_expr(arg)?);
                    }
                    if lowered_args.len() == 2 {
                        lowered_args.push(LoweredExpr::GetLength(
                            Box::new(receiver),
                            Span::generated("get_length"),
                        ));
                    }
                }
                _ => {
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);
                }
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,
                span: Span::generated("runtime_call"),
            }));
        }
        if (method == "find"
            || method == "findIndex"
            || method == "findLast"
            || method == "findLastIndex"
            || method == "filter"
            || method == "every"
            || method == "some")
            && is_identity_arrow_callback(args)
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: match method {
                    "find" => RuntimeFn::ArrayFind,
                    "findIndex" => RuntimeFn::ArrayFindIndex,
                    "findLast" => RuntimeFn::ArrayFindLast,
                    "findLastIndex" => RuntimeFn::ArrayFindLastIndex,
                    "filter" => RuntimeFn::ArrayFilter,
                    "every" => RuntimeFn::ArrayEvery,
                    "some" => RuntimeFn::ArraySome,
                    _ => unreachable!(),
                },
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }
        if let Some(formatted) = static_number_format_method(&self.ctx, object, method, args) {
            return Ok(Some(LoweredExpr::String(
                formatted,
                Span::generated("number_format"),
            )));
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "Object")
            && method == "defineProperty"
            && args.len() == 3
        {
            let lowered_target = self.lower_expr(&args[0])?;
            let lowered_key = self.lower_expr(&args[1])?;
            let lowered_desc = self.lower_expr(&args[2])?;
            if let (ResolvedExpr::Ident(target_name), Some(static_key), Some(accessor)) = (
                &args[0],
                super::super::string::resolved_expr_static_accessor_key(&self.ctx, &args[1]),
                self.accessor_prop_from_descriptor_expr(&lowered_desc),
            ) && let Ok(target_local) = self.resolve_local(target_name)
            {
                self.ctx
                    .classes
                    .object_accessor_props
                    .entry(target_local)
                    .or_default()
                    .entry(static_key)
                    .and_modify(|existing| {
                        if accessor.get.is_some() {
                            existing.get = accessor.get;
                        }
                        if accessor.set.is_some() {
                            existing.set = accessor.set;
                        }
                    })
                    .or_insert(accessor);
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ObjectDefineProperty,
                args: vec![lowered_target, lowered_key, lowered_desc],
                span: Span::generated("runtime_call"),
            }));
        }
        // Skip ObjectToString catch-all for Error/Array/BigInt locals — let class dispatch handle it
        let is_class_dispatch_receiver = matches!(method, "toString" | "valueOf")
            && match object {
                ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local| {
                    let class_name = self.ctx.classes.local_classes.get(&local);
                    class_name.is_some_and(|c| c == "BigInt")
                        || self.ctx.facts.bigint_locals.contains(&local)
                }),
                _ => crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(
                    &self.ctx, object,
                ),
            };
        // Also skip ObjectToString catch-all for Error.toString — let class dispatch route to ErrorToString
        let is_error_to_string = method == "toString"
            && match object {
                ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local| {
                    self.ctx
                        .classes
                        .local_classes
                        .get(&local)
                        .is_some_and(|c| is_error_class(c))
                }),
                ResolvedExpr::New { class_name, .. } => is_error_class(class_name),
                _ => false,
            };
        if is_class_dispatch_receiver || is_error_to_string {
            // fall through to class dispatch
        } else if method == "toString"
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
        {
            // fall through to class dispatch — Array.toString → ArrayJoin
        } else if let Some(intrinsic) = resolve_method_to_runtime_fn(object, method) {
            if intrinsic == RuntimeFn::JsonParse {
                if args.is_empty() || args.len() > 2 {
                    return Err(Diagnostic {
                        code: DiagCode::ArityMismatch,
                        message: format!("JSON.parse expects 1 to 2 arguments, got {}", args.len()),
                        span: Some(span),
                        phase: None,
                    });
                }
                let mut lowered_args = vec![self.lower_expr(&args[0])?];
                lowered_args.push(match args.get(1) {
                    None | Some(ResolvedExpr::Undefined) => {
                        LoweredExpr::Undefined(Span::generated("undef"))
                    }
                    Some(ResolvedExpr::Null) => LoweredExpr::Null(Span::generated("null")),
                    Some(ResolvedExpr::Ident(name))
                        if self
                            .ctx
                            .symbols
                            .function_ids
                            .get(name)
                            .and_then(|id| self.ctx.symbols.function_signatures.get(id))
                            .is_some_and(|signature| {
                                !signature.has_rest && !signature.needs_arguments
                            }) =>
                    {
                        let func_id = self.ctx.symbols.function_ids[name];
                        LoweredExpr::Number(func_id.0 as i32, Span::generated("num"))
                    }
                    Some(ResolvedExpr::Ident(name))
                        if self.ctx.symbols.function_ids.contains_key(name) =>
                    {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-432: JSON.parse reviver callbacks with rest parameters or `arguments` are not supported yet".to_owned(),
                            span: Some(span),
                            phase: None,
                        });
                    }
                    Some(_) => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-432: JSON.parse reviver currently supports named function declarations, null, or undefined".to_owned(),
                            span: Some(span),
                            phase: None,
                        });
                    }
                });
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: lowered_args,
                    span: Span::generated("runtime_call"),
                }));
            }
            if (intrinsic == RuntimeFn::ArrayPush || intrinsic == RuntimeFn::ArrayPushGrow)
                && let [ResolvedExpr::Spread(spread_expr)] = args
            {
                return Ok(Some(self.lower_array_push_single_spread_arg(
                    object,
                    spread_expr.as_ref(),
                )?));
            }
            if (intrinsic == RuntimeFn::ArrayPush || intrinsic == RuntimeFn::ArrayPushGrow)
                && args.len() != 1
            {
                if !matches!(object, ResolvedExpr::Ident(_)) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-271: multi-argument Array.prototype.push is currently supported only for identifier array receivers".to_owned(),
                        span: Some(span),

                        phase: None,
                    });
                }
                let mut lowered_args = vec![self.lower_expr(object)?];
                lowered_args.extend(
                    args.iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?,
                );
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ArrayPushMany,
                    args: lowered_args,

                    span: Span::generated("runtime_call"),
                }));
            }
            if (intrinsic == RuntimeFn::MathMax || intrinsic == RuntimeFn::MathMin)
                && args.len() > 2
            {
                let mut lowered_args = Vec::new();
                if !matches!(
                    object,
                    ResolvedExpr::Ident(name)
                        if name == "Math"
                            || name == "JSON"
                            || name == "Object"
                            || name == "String"
                ) {
                    lowered_args.push(self.lower_expr(object)?);
                }
                for arg in args {
                    lowered_args.push(self.lower_expr(arg)?);
                }
                let mut result = lowered_args[0].clone();
                for arg in &lowered_args[1..] {
                    result = LoweredExpr::RuntimeCall {
                        intrinsic,
                        args: vec![result, arg.clone()],

                        span: Span::generated("runtime_call"),
                    };
                }
                return Ok(Some(result));
            }
            // Handle zero-argument case for Math.max/min
            if (intrinsic == RuntimeFn::MathMax || intrinsic == RuntimeFn::MathMin)
                && args.is_empty()
            {
                use ts2wasm_runtime_abi::ValueTag;
                let infinity_value = if intrinsic == RuntimeFn::MathMax {
                    ValueTag::NUMBER_PAYLOAD_MIN
                } else {
                    ValueTag::NUMBER_PAYLOAD_MAX
                };
                return Ok(Some(LoweredExpr::Number(
                    infinity_value,
                    Span::generated("num"),
                )));
            }
            let mut lowered_args = Vec::new();
            let is_static_call = matches!(
                object,
                ResolvedExpr::Ident(name)
                    if name == "Math"
                        || name == "JSON"
                        || name == "Object"
                        || name == "String"
                        || name == "Number"
                        || name == "Boolean"
                        || name == "Symbol"
                        || name == "Array"
                        || name == "Promise"
                        || name == "Atomics"
                        || name == "Reflect"
                        || name == "$262"
            );
            if !is_static_call {
                lowered_args.push(self.lower_expr(object)?);
            }
            lowered_args.extend(
                args.iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            if is_number_format_runtime_fn(intrinsic) && args.is_empty() {
                lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
            }
            if intrinsic == RuntimeFn::ParseInt && args.len() == 1 {
                lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
            }
            if intrinsic == RuntimeFn::ObjectGetOwnPropertyDescriptor {
                while lowered_args.len() < 2 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
            }
            // Object.assign(target, source) — pad source to undefined
            if intrinsic == RuntimeFn::ObjectAssign && lowered_args.len() < 2 {
                while lowered_args.len() < 2 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
            }
            // Object.create(proto, properties) — pad properties to undefined
            if intrinsic == RuntimeFn::ObjectCreate && lowered_args.len() < 2 {
                while lowered_args.len() < 2 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
            }
            // Object.is(a, b) — pad to 2 args
            if intrinsic == RuntimeFn::ObjectIs && lowered_args.len() < 2 {
                while lowered_args.len() < 2 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
            }
            // Object.fromEntries(entries) — pad to 1 arg
            if intrinsic == RuntimeFn::ObjectFromEntries && lowered_args.is_empty() {
                lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
            }
            // Reflect.get(target, key, receiver) — pad receiver to undefined
            if intrinsic == RuntimeFn::ReflectGet && lowered_args.len() < 3 {
                while lowered_args.len() < 3 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
            }
            // Reflect.set(target, key, value, receiver) — pad receiver to undefined
            if intrinsic == RuntimeFn::ReflectSet && lowered_args.len() < 4 {
                while lowered_args.len() < 4 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
            }
            // Reflect.deleteProperty(target, key) — ensure 2 args
            if intrinsic == RuntimeFn::ReflectDeleteProperty && lowered_args.len() < 2 {
                while lowered_args.len() < 2 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        Ok(None)
    }

    /// Map a Reflect.* or Object.* method name to a ProxyTrapKind for compile-time proxy dispatch.
    fn reflect_or_object_method_to_proxy_trap(method: &str) -> Option<ProxyTrapKind> {
        match method {
            "get" => Some(ProxyTrapKind::ProxyGet),
            "set" => Some(ProxyTrapKind::ProxySet),
            "has" => Some(ProxyTrapKind::ProxyHas),
            "deleteProperty" => Some(ProxyTrapKind::ProxyDeleteProperty),
            "construct" => Some(ProxyTrapKind::ProxyConstruct),
            "apply" => Some(ProxyTrapKind::ProxyApply),
            "getPrototypeOf" => Some(ProxyTrapKind::ProxyGetPrototypeOf),
            "setPrototypeOf" => Some(ProxyTrapKind::ProxySetPrototypeOf),
            "isExtensible" => Some(ProxyTrapKind::ProxyIsExtensible),
            "preventExtensions" => Some(ProxyTrapKind::ProxyPreventExtensions),
            "getOwnPropertyDescriptor" => Some(ProxyTrapKind::ProxyGetOwnPropertyDescriptor),
            "defineProperty" => Some(ProxyTrapKind::ProxyDefineProperty),
            "ownKeys" | "keys" | "values" | "getOwnPropertyNames" => {
                Some(ProxyTrapKind::ProxyOwnKeys)
            }
            _ => None,
        }
    }

    fn lower_static_proxy_object_call(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if matches!(object, ResolvedExpr::Ident(name) if name == "Proxy") && method == "revocable" {
            let [target, _handler] = args else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-438: Proxy.revocable requires target and handler arguments"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            };
            return Ok(Some(LoweredExpr::ObjectNew {
                props: vec![
                    ("proxy".to_owned(), self.lower_expr(target)?),
                    (
                        "revoke".to_owned(),
                        LoweredExpr::Undefined(Span::generated("undef")),
                    ),
                ],
                non_enumerable: 0,
                span: Span::generated("object_new"),
            }));
        }

        if !matches!(object, ResolvedExpr::Ident(name) if name == "Object") {
            return Ok(None);
        }
        if method == "getOwnPropertyDescriptor"
            && let [ResolvedExpr::Ident(target), ResolvedExpr::String(key)] = args
            && target == "Number"
            && let Some(desc) =
                crate::lowered::program_builtins::builtin_function_data_descriptor(key, span)
        {
            return Ok(Some(desc));
        }
        if method == "getOwnPropertyDescriptor"
            && let [ResolvedExpr::Ident(target), ResolvedExpr::String(key)] = args
            && matches!(key.as_str(), "name" | "length")
            && let Some(desc) = self.local_arrow_function_data_descriptor(target, key)
        {
            return Ok(Some(desc));
        }
        if method == "getOwnPropertyDescriptor"
            && let [ResolvedExpr::Ident(target), ResolvedExpr::String(key)] = args
            && let Some(desc) = self.static_object_accessor_descriptor(target, key, span)
        {
            return Ok(Some(desc));
        }
        let Some(proxy) = args.first().and_then(|arg| {
            crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(&self.ctx, arg)
        }) else {
            return Ok(None);
        };

        let trap = match method {
            "keys" => "ownKeys",
            "getOwnPropertyDescriptor" => "getOwnPropertyDescriptor",
            "defineProperty" => "defineProperty",
            "getPrototypeOf" => "getPrototypeOf",
            "setPrototypeOf" => "setPrototypeOf",
            _ => return Ok(None),
        };
        let trap_args = match method {
            "keys" | "getPrototypeOf" => Vec::new(),
            "getOwnPropertyDescriptor" => {
                let Some(key) = args.get(1) else {
                    return Err(Diagnostic {
                        code: DiagCode::ArityMismatch,
                        message:
                            "Object.getOwnPropertyDescriptor proxy trap requires a property key"
                                .to_owned(),
                        span: Some(span),
                        phase: None,
                    });
                };
                vec![key.clone()]
            }
            "defineProperty" => {
                let (Some(key), Some(desc)) = (args.get(1), args.get(2)) else {
                    return Err(Diagnostic {
                        code: DiagCode::ArityMismatch,
                        message: "Object.defineProperty proxy trap requires a property key and descriptor"
                            .to_owned(),
                        span: Some(span),
                        phase: None,
                    });
                };
                vec![key.clone(), desc.clone()]
            }
            "setPrototypeOf" => {
                let Some(proto) = args.get(1) else {
                    return Err(Diagnostic {
                        code: DiagCode::ArityMismatch,
                        message: "Object.setPrototypeOf proxy trap requires a prototype argument"
                            .to_owned(),
                        span: Some(span),
                        phase: None,
                    });
                };
                vec![proto.clone()]
            }
            _ => unreachable!("filtered above"),
        };

        Ok(Some(self.lower_proxy_trap_call(
            proxy,
            crate::lowered::facts::ProxyTrapKind::Named(trap),
            trap_args,
            span,
        )?))
    }

    pub(crate) fn lower_array_push_single_spread_arg(
        &mut self,
        object: &ResolvedExpr,
        spread_expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        let span = Span::generated("array_push_spread");
        let receiver = self.alloc_temp();
        Ok(LoweredExpr::Block {
            stmts: vec![
                LoweredStmt::Let(receiver, self.lower_expr(object)?, span),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ArrayPushOrSpread,
                        args: vec![
                            LoweredExpr::Local(receiver, Span::generated("local")),
                            self.lower_expr(spread_expr)?,
                        ],
                        span,
                    },
                    span,
                ),
            ],
            result: Box::new(LoweredExpr::GetLength(
                Box::new(LoweredExpr::Local(receiver, Span::generated("local"))),
                Span::generated("get_length"),
            )),
            span,
        })
    }

    fn lower_static_group_by_dispatch(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if matches!(object, ResolvedExpr::Ident(name) if name == "Object") && method == "groupBy" {
            return Ok(Some(self.lower_object_group_by_callback(args, span)?));
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "Map") && method == "groupBy" {
            return Ok(Some(self.lower_map_group_by_callback(args, span)?));
        }
        Ok(None)
    }

    /// Helper for lower_method_call_expr: dispatch early-returns —
    /// object function props, array map special cases (holes, string constructor,
    /// unary-plus, literal, split-result), sort, prototype.map call, user-callback
    /// array methods, and this.method dispatch.
    fn lower_mcall_dispatch_early(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if let Some(lowered) = self.lower_static_group_by_dispatch(object, method, args, span)? {
            return Ok(Some(lowered));
        }
        if let Some(lowered) = self.lower_object_prototype_dispatch(object, method, args, span)? {
            return Ok(Some(lowered));
        }

        if let Some(formatted) = static_number_format_method(&self.ctx, object, method, args) {
            return Ok(Some(LoweredExpr::String(
                formatted,
                Span::generated("number_format"),
            )));
        }

        if matches!(object, ResolvedExpr::Ident(name) if name == "globalThis")
            && let Some(method_id) = self
                .ctx
                .classes
                .global_object_function_props
                .get(&ObjectAccessorKey::Property(method.to_owned()))
                .copied()
        {
            let lowered_args = self.lower_function_call_args(
                method_id,
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::GlobalThis,
                    args: Vec::new(),
                    span: Span::generated("globalThis"),
                },
                args,
            )?;
            return Ok(Some(LoweredExpr::Call {
                kind: FunctionCallKind::User(method_id),
                args: lowered_args,
                span: Span::generated("call"),
            }));
        }

        if let ResolvedExpr::Ident(receiver_name) = object
            && let Ok(obj_local) = self.resolve_local(receiver_name)
            && let Some(method_id) = self
                .ctx
                .classes
                .object_function_props
                .get(&obj_local)
                .and_then(|props| props.get(&ObjectAccessorKey::Property(method.to_owned())))
                .copied()
        {
            let lowered_args = self.lower_function_call_args(
                method_id,
                LoweredExpr::Local(obj_local, Span::generated("local")),
                args,
            )?;
            return Ok(Some(LoweredExpr::Call {
                kind: FunctionCallKind::User(method_id),
                args: lowered_args,

                span: Span::generated("call"),
            }));
        }

        // Sparse arrays with known holes — route through hole-aware
        // lower_array_map_elements before optimized paths.
        if method == "map"
            && let Some(elements) =
                crate::lowered::resolver::expr::facts::resolved_expr_static_array_slots(
                    &self.ctx, object,
                )
            && elements
                .iter()
                .any(|element| matches!(element, ResolvedArrayElement::Hole))
        {
            return Ok(Some(
                self.lower_array_map_elements(object, &elements, args, span)?,
            ));
        }

        if method == "map"
            && string_constructor_arrow_callback(args)
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayMapValueToString,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }

        if method == "map"
            && unary_plus_arrow_callback(args)
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayMapUnaryPlus,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            }));
        }

        if method == "map" && matches!(object, ResolvedExpr::Array(_)) {
            return Ok(Some(self.lower_array_literal_map(object, args, span)?));
        }

        if method == "map"
            && is_string_split_result_expr(object)
            && is_identity_arrow_callback(args)
        {
            return Ok(Some(self.lower_expr(object)?));
        }

        if method == "map"
            && is_string_split_result_expr(object)
            && let Some(separator) = string_split_arrow_separator(args)
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayMapStringSplit,
                args: vec![self.lower_expr(object)?, self.lower_expr(separator)?],

                span: Span::generated("runtime_call"),
            }));
        }

        if method == "sort"
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
        {
            if args.is_empty() {
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ArraySortLexicographic,
                    args: vec![self.lower_expr(object)?],

                    span: Span::generated("runtime_call"),
                }));
            }
            if numeric_ascending_sort_arrow_callback(args) {
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ArraySortNumeric,
                    args: vec![self.lower_expr(object)?],

                    span: Span::generated("runtime_call"),
                }));
            }
            return Err(unsupported_array_sort_diagnostic(Some(span)));
        }

        if is_array_prototype_map_call_receiver(object, method) {
            return Err(unsupported_array_map_diagnostic(Some(span)));
        }

        // User-callback array methods expanded at IR level with While loops.
        if (method == "forEach"
            || method == "filter"
            || method == "find"
            || method == "findIndex"
            || method == "findLast"
            || method == "findLastIndex"
            || method == "some"
            || method == "every"
            || method == "reduce"
            || method == "reduceRight"
            || method == "map"
            || method == "flatMap")
            && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, object)
            && !args.is_empty()
            && match &args[0] {
                ResolvedExpr::ArrowFn { .. }
                | ResolvedExpr::FunctionExpr {
                    is_generator: false,
                    ..
                } => true,
                ResolvedExpr::Ident(name) => {
                    self.ctx.symbols.function_ids.contains_key(name.as_str())
                }
                _ => false,
            }
        {
            let lowered_receiver = self.lower_expr(object)?;
            return Ok(Some(self.lower_array_callback_method(
                method,
                lowered_receiver,
                object,
                args,
                span,
            )?));
        }

        if matches!(object, ResolvedExpr::This { .. }) {
            let class_name = self
                .ctx
                .classes
                .current_class
                .as_ref()
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "this.method(...) requires class context".to_owned(),
                    span: Some(span),

                    phase: None,
                })?;
            let method_id = self
                .resolve_class_method(class_name, method)
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("method `{}.{}` not found", class_name, method),
                    span: Some(span),

                    phase: None,
                })?;

            let mut lowered_args = vec![LoweredExpr::Local(
                self.resolve_local("this")?,
                Span::generated("local"),
            )];
            lowered_args.extend(
                args.iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            self.append_class_method_captures(method_id, &mut lowered_args)?;
            return Ok(Some(LoweredExpr::Call {
                kind: FunctionCallKind::User(method_id),
                args: lowered_args,

                span: Span::generated("call"),
            }));
        }

        Ok(None)
    }

    /// Helper for lower_method_call_expr: handle non-Ident receivers —
    /// PropertyAccess this.field, prototype.call unwrap, runtime_fn_arg,
    /// new C().method(), and issue-211 error.
    fn lower_mcall_nonident_receiver(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        // Ident receivers are handled by lower_mcall_class_dispatch
        if matches!(object, ResolvedExpr::Ident(_)) {
            return Ok(None);
        }

        // Array.prototype.forEach/find/findIndex/findLast/findLastIndex with
        // ArrowFn/FunctionExpr callback — route through IR-level While loop
        // expansion even for non-ident receivers
        if matches!(
            method,
            "forEach" | "find" | "findIndex" | "findLast" | "findLastIndex"
        ) && !args.is_empty()
            && match &args[0] {
                ResolvedExpr::ArrowFn { .. }
                | ResolvedExpr::FunctionExpr {
                    is_generator: false,
                    ..
                } => true,
                ResolvedExpr::Ident(name) => {
                    self.ctx.symbols.function_ids.contains_key(name.as_str())
                }
                _ => false,
            }
        {
            let lowered_receiver = self.lower_expr(object)?;
            return Ok(Some(self.lower_array_callback_method(
                method,
                lowered_receiver,
                object,
                args,
                span,
            )?));
        }

        // this.field.method(...) — PropertyAccess with This receiver
        if let ResolvedExpr::PropertyAccess {
            object: prop_obj,
            key,
            ..
        } = object
            && matches!(prop_obj.as_ref(), ResolvedExpr::This { .. })
        {
            if let Some(intrinsic) = collection_method_runtime_fn_arg(method) {
                let receiver_expr = self.lower_expr(object)?;
                let mut lowered_args = vec![receiver_expr];
                if !is_identity_array_method(method)
                    && !matches!(method, "find" | "findIndex" | "findLast" | "findLastIndex")
                {
                    let max_args = if method == "indexOf" || method == "includes" {
                        1
                    } else {
                        args.len()
                    };
                    for arg in args.iter().take(max_args) {
                        lowered_args.push(self.lower_expr(arg)?);
                    }
                    // $array_index_of / $array_includes expect 3 params:
                    // $arr, $search, $from_idx. Pad missing fromIndex with 0.
                    if (method == "indexOf" || method == "includes") && lowered_args.len() < 3 {
                        lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
                    }
                    // $array_fill expects 4 params: $arr, $val, $start, $end
                    if method == "fill" && lowered_args.len() < 4 {
                        lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                    }
                    // Promise prototype methods: pad missing callbacks with undefined
                    let promise_expected = match method {
                        "then" => Some(3),              // receiver + onFulfilled + onRejected
                        "catch" | "finally" => Some(2), // receiver + callback
                        _ => None,
                    };
                    if let Some(expected) = promise_expected {
                        while lowered_args.len() < expected {
                            lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                        }
                    }
                }
                return Ok(Some(LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: lowered_args,

                    span: Span::generated("runtime_call"),
                }));
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-211: method `{}` on `this.{}` requires an identifier receiver",
                    method, key
                ),
                span: Some(span),

                phase: None,
            });
        }

        // Non-identifier receiver
        // Handle ClassName.prototype.method.call(thisArg, ...args) pattern
        if method == "call"
            && let Some((class_name, proto_method)) = extract_prototype_method_name(object)
        {
            if let Some((receiver, call_args)) = args.split_first() {
                // Array callback methods (every, some, find, filter, etc.)
                // with ArrowFn — route through IR-level While loop expansion
                if class_name == "Array"
                    && !call_args.is_empty()
                    && (matches!(call_args[0], ResolvedExpr::ArrowFn { .. })
                        || matches!(
                            call_args[0],
                            ResolvedExpr::FunctionExpr {
                                is_generator: false,
                                ..
                            }
                        ))
                    && (proto_method == "every"
                        || proto_method == "some"
                        || proto_method == "find"
                        || proto_method == "findIndex"
                        || proto_method == "findLast"
                        || proto_method == "findLastIndex"
                        || proto_method == "filter"
                        || proto_method == "forEach"
                        || proto_method == "map"
                        || proto_method == "reduce"
                        || proto_method == "reduceRight"
                        || proto_method == "flatMap")
                    && crate::lowered::resolver::expr::facts::is_known_array_expr(
                        &self.ctx, receiver,
                    )
                {
                    let lowered_receiver = self.lower_expr(receiver)?;
                    return Ok(Some(self.lower_array_callback_method(
                        proto_method,
                        lowered_receiver,
                        receiver,
                        call_args,
                        span,
                    )?));
                }
                // String HTML wrapper methods
                if class_name == "String" && is_html_wrapper_string_method(proto_method) {
                    let lowered_receiver = self.lower_expr(receiver)?;
                    let lowered_call_args = call_args
                        .iter()
                        .map(|a| self.lower_expr(a))
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(Some(lower_html_wrapper_string_method(
                        proto_method,
                        lowered_receiver,
                        lowered_call_args,
                        span,
                    )?));
                }
                if class_name == "Object" && is_object_prototype_method(proto_method) {
                    let lowered_receiver = self.lower_expr(receiver)?;
                    return Ok(Some(self.lower_object_prototype_method(
                        lowered_receiver,
                        proto_method,
                        call_args,
                        span,
                    )?));
                }
                // Non-callback runtime functions
                if let Some(intrinsic) = collection_method_runtime_fn(class_name, proto_method) {
                    let lowered_receiver = self.lower_expr(receiver)?;
                    let lowered_args = self.lower_collection_method_args(
                        lowered_receiver,
                        class_name,
                        proto_method,
                        call_args,
                    )?;
                    return Ok(Some(LoweredExpr::RuntimeCall {
                        intrinsic,
                        args: lowered_args,

                        span: Span::generated("runtime_call"),
                    }));
                }
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-211: {class_name}.prototype.{proto_method}.call is not supported"
                ),
                span: Some(span),

                phase: None,
            });
        }

        // Fall through to runtime_fn_arg dispatch
        if let Some(intrinsic) = number_format_method_runtime_fn(method) {
            let mut lowered_args = vec![self.lower_expr(object)?];
            lowered_args.extend(
                args.iter()
                    .take(1)
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            if lowered_args.len() == 1 {
                lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }

        // BigInt non-ident receiver (e.g. (a + b).toString()) — skip ArrayJoin catch-all
        if matches!(method, "toString" | "toLocaleString" | "valueOf")
            && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, object)
        {
            let receiver = self.lower_expr(object)?;
            if method == "valueOf" {
                return Ok(Some(receiver));
            }
            let mut bi_args = vec![receiver];
            bi_args.extend(
                args.iter()
                    .take(1)
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            if bi_args.len() == 1 {
                bi_args.push(LoweredExpr::Undefined(Span::generated("radix")));
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::BigIntToString,
                args: bi_args,
                span: Span::generated("runtime_call"),
            }));
        }

        if let Some(intrinsic) = collection_method_runtime_fn_arg(method) {
            let receiver_expr = self.lower_expr(object)?;
            let mut lowered_args = vec![receiver_expr];
            if !is_identity_array_method(method)
                && !matches!(method, "find" | "findIndex" | "findLast" | "findLastIndex")
            {
                let max_args = if method == "indexOf" || method == "includes" {
                    2
                } else {
                    args.len()
                };
                for arg in args.iter().take(max_args) {
                    lowered_args.push(self.lower_expr(arg)?);
                }
                // $array_index_of / $array_includes expect 3 params:
                // $arr, $search, $from_idx. Pad missing fromIndex with 0.
                if (method == "indexOf" || method == "includes") && lowered_args.len() < 3 {
                    lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
                }
                // $array_fill expects 4 params: $arr, $val, $start, $end
                if method == "fill" && lowered_args.len() < 4 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
                // Promise prototype methods: pad missing callbacks with undefined
                let promise_expected = match method {
                    "then" => Some(3),              // receiver + onFulfilled + onRejected
                    "catch" | "finally" => Some(2), // receiver + callback
                    _ => None,
                };
                if let Some(expected) = promise_expected {
                    while lowered_args.len() < expected {
                        lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                    }
                }
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }

        if crate::lowered::resolver::expr::facts::resolved_expr_returns_host_external_object(
            &self.ctx, object,
        ) {
            let receiver_temp = self.alloc_temp();
            let receiver = LoweredExpr::Local(receiver_temp, Span::generated("local"));
            let args_array = ResolvedExpr::Array(
                args.iter()
                    .cloned()
                    .map(ResolvedArrayElement::Present)
                    .collect(),
            );
            return Ok(Some(LoweredExpr::Block {
                stmts: vec![LoweredStmt::Let(
                    receiver_temp,
                    self.lower_expr(object)?,
                    Span::generated("let_stmt"),
                )],
                result: Box::new(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::FunctionCallMethodHost,
                    args: vec![
                        object_kernel::ordinary_get(receiver.clone(), method, span),
                        receiver,
                        self.lower_expr(&args_array)?,
                    ],
                    span: Span::generated("runtime_call"),
                }),
                span: Span::generated("block"),
            }));
        }

        // new C().method() — route through runtime_fn for Map/Set/Array collection methods
        if let ResolvedExpr::New { class_name, .. } = object
            && let Some(intrinsic) = collection_method_runtime_fn(class_name, method)
        {
            let lowered_receiver = self.lower_expr(object)?;
            let mut lowered_args = vec![lowered_receiver];
            lowered_args.extend(
                args.iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            // ArrayJoin expects a separator argument. Inject "," for toString/toLocaleString
            // when no separator was explicitly passed.
            if (method == "toString" || method == "toLocaleString") && lowered_args.len() == 1 {
                lowered_args.push(LoweredExpr::String(",".to_owned(), Span::generated("str")));
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }

        // new C().method() — lower through class method dispatch
        if let ResolvedExpr::New { class_name, .. } = object
            && let Some(method_id) = self.resolve_class_method(class_name, method)
        {
            let lowered_receiver = self.lower_expr(object)?;
            let mut lowered_args = vec![lowered_receiver];
            lowered_args.extend(
                args.iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            self.append_class_method_captures(method_id, &mut lowered_args)?;
            return Ok(Some(LoweredExpr::Call {
                kind: FunctionCallKind::User(method_id),
                args: lowered_args,

                span: Span::generated("call"),
            }));
        }

        // Function.prototype.call/apply with non-Ident receiver:
        // bypass ordinary_get("call"/"apply") and inline as HeapClosureCall
        if method == "call" || method == "apply" {
            let fn_temp = self.alloc_temp();
            let fn_val = LoweredExpr::Local(fn_temp, Span::generated("local"));
            let this_arg = match args.first() {
                Some(receiver) => self.lower_expr(receiver)?,
                None => LoweredExpr::Undefined(Span::generated("undef")),
            };
            let mut call_args = vec![fn_val.clone(), this_arg];
            if method == "call" {
                for arg in args.iter().skip(1) {
                    call_args.push(self.lower_expr(arg)?);
                }
            } else {
                // apply: expand argArray elements
                match args.get(1) {
                    None | Some(ResolvedExpr::Undefined | ResolvedExpr::Null) => {}
                    Some(ResolvedExpr::Array(elements)) => {
                        for element in elements {
                            match element {
                                ResolvedArrayElement::Present(expr) => {
                                    call_args.push(self.lower_expr(expr)?);
                                }
                                ResolvedArrayElement::Hole => {
                                    call_args
                                        .push(LoweredExpr::Undefined(Span::generated("undef")));
                                }
                            }
                        }
                    }
                    Some(_) => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-458: Function.prototype.apply currently supports array literals, null, or undefined argArray for non-Ident receivers".to_owned(),
                            span: Some(span),
                            phase: None,
                        });
                    }
                }
            }
            return Ok(Some(LoweredExpr::Block {
                stmts: vec![LoweredStmt::Let(
                    fn_temp,
                    self.lower_expr(object)?,
                    Span::generated("let_stmt"),
                )],
                result: Box::new(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::HeapClosureCall,
                    args: call_args,
                    span: Span::generated("runtime_call"),
                }),
                span: Span::generated("block"),
            }));
        }

        let receiver_temp = self.alloc_temp();
        let receiver = LoweredExpr::Local(receiver_temp, Span::generated("local"));
        let callee = object_kernel::ordinary_get(receiver.clone(), method, span);
        let mut call_args = vec![callee, receiver];
        call_args.extend(
            args.iter()
                .map(|arg| self.lower_expr(arg))
                .collect::<Result<Vec<_>, _>>()?,
        );
        Ok(Some(LoweredExpr::Block {
            stmts: vec![LoweredStmt::Let(
                receiver_temp,
                self.lower_expr(object)?,
                Span::generated("let_stmt"),
            )],
            result: Box::new(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::HeapClosureCall,
                args: call_args,
                span: Span::generated("runtime_call"),
            }),
            span: Span::generated("block"),
        }))
    }

    /// Helper for lower_method_call_expr: Ident receiver class method dispatch —
    /// Map/Set forEach, local class runtime_fn, super.method, static methods,
    /// and final class method resolution.
    fn lower_mcall_class_dispatch(
        &mut self,
        receiver_name: &str,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        // Map.forEach or Set.forEach with ArrowFn/FunctionExpr/Ident — expand at IR level
        if method == "forEach"
            && !args.is_empty()
            && match &args[0] {
                ResolvedExpr::ArrowFn { .. }
                | ResolvedExpr::FunctionExpr {
                    is_generator: false,
                    ..
                } => true,
                ResolvedExpr::Ident(name) => {
                    self.ctx.symbols.function_ids.contains_key(name.as_str())
                }
                _ => false,
            }
            && let Ok(obj_local) = self.resolve_local(receiver_name)
            && let Some(class_name) = self.ctx.classes.local_classes.get(&obj_local)
            && (class_name == "Map" || class_name == "Set")
        {
            let is_map = *class_name == "Map";
            let lowered_receiver = self.lower_expr(object)?;
            if is_map {
                return self.lower_map_for_each_method(lowered_receiver, object, args, span);
            } else {
                return self.lower_set_for_each_method(lowered_receiver, object, args, span);
            }
        }

        if let Ok(obj_local) = self.resolve_local(receiver_name)
            && self
                .ctx
                .classes
                .local_classes
                .get(&obj_local)
                .is_some_and(|class_name| is_intl_number_format_class(class_name.as_str()))
            && is_intl_number_format_method(method)
        {
            let options = self
                .ctx
                .facts
                .intl_number_format_locals
                .get(&obj_local)
                .cloned();
            return self.lower_intl_number_format_method(method, args, options.as_ref());
        }
        if let Ok(obj_local) = self.resolve_local(receiver_name)
            && self
                .ctx
                .classes
                .local_classes
                .get(&obj_local)
                .is_some_and(|class_name| is_intl_date_time_format_class(class_name.as_str()))
            && is_intl_date_time_format_method(method)
        {
            let options = self
                .ctx
                .facts
                .intl_date_time_format_locals
                .get(&obj_local)
                .cloned();
            return self.lower_intl_date_time_format_method(method, args, options.as_ref());
        }

        // Local class runtime_fn
        if let Ok(obj_local) = self.resolve_local(receiver_name)
            && let Some(class_name) = self.ctx.classes.local_classes.get(&obj_local)
            && let Some(intrinsic) = collection_method_runtime_fn(class_name, method)
        {
            let class_name = class_name.clone();
            let class_name = class_name.as_str();
            let is_array_like_class = class_name == "Array" || is_typed_array_class(class_name);
            if class_name == "Array"
                && method == "push"
                && let [ResolvedExpr::Spread(spread_expr)] = args
            {
                return self.lower_array_push_single_spread_arg(object, spread_expr.as_ref());
            }
            if class_name == "RegExp" && args.len() > 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "RegExp.prototype.{method} expects at most 1 argument, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let mut lowered_args = vec![LoweredExpr::Local(obj_local, Span::generated("local"))];
            // Array.prototype.flat defaults depth to 1 when omitted
            if is_array_like_class && method == "flat" && args.is_empty() {
                lowered_args.push(LoweredExpr::Number(1, Span::generated("num")));
            } else if is_array_like_class && method == "join" && args.is_empty() {
                lowered_args.push(LoweredExpr::String(",".to_owned(), Span::generated("str")));
            } else if is_array_like_class && method == "copyWithin" {
                // copyWithin(target, start, end) — pad missing args with undefined
                for arg in args.iter().take(3) {
                    lowered_args.push(self.lower_expr(arg)?);
                }
                while lowered_args.len() < 4 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
            } else if is_array_like_class && method == "fill" {
                // fill(value, start?, end?) — pad missing args with undefined
                for arg in args.iter().take(3) {
                    lowered_args.push(self.lower_expr(arg)?);
                }
                while lowered_args.len() < 4 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
            } else if is_typed_array_class(class_name) && method == "set" {
                for arg in args.iter().take(2) {
                    lowered_args.push(self.lower_expr(arg)?);
                }
                if lowered_args.len() == 2 {
                    lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
                }
            } else if is_array_like_class && (method == "toString" || method == "toLocaleString") {
                // toString/toLocaleString calls join(",") internally
                lowered_args.push(LoweredExpr::String(",".to_owned(), Span::generated("str")));
            } else {
                let receiver = lowered_args.remove(0);
                lowered_args =
                    self.lower_collection_method_args(receiver, class_name, method, args)?;
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            });
        }

        // super.method
        if receiver_name == "super" {
            if self.ctx.classes.current_class.is_none() {
                let this_local = self.resolve_local("this").map_err(|_| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "super.method(...) requires class context or object method receiver"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                })?;
                let this_expr = LoweredExpr::Local(this_local, Span::generated("local"));
                let callee = object_kernel::ordinary_get(
                    object_kernel::ordinary_get_prototype_of(
                        this_expr.clone(),
                        Span::generated("object_home_proto"),
                    ),
                    method,
                    span,
                );
                let mut lowered_args = vec![callee, this_expr];
                lowered_args.extend(
                    args.iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?,
                );
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::HeapClosureCall,
                    args: lowered_args,
                    span: Span::generated("runtime_call"),
                });
            }
            let class_name = self
                .ctx
                .classes
                .current_class
                .as_ref()
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "super.method(...) requires class context".to_owned(),
                    span: Some(span),

                    phase: None,
                })?;
            let parent_name = self
                .ctx
                .classes
                .class_parents
                .get(class_name)
                .and_then(|p| p.clone())
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "super.method(...) used in class without extends".to_owned(),
                    span: Some(span),

                    phase: None,
                })?;
            let mut lowered_args = Vec::new();
            let method_id = if let Ok(this_local) = self.resolve_local("this") {
                lowered_args.push(LoweredExpr::Local(this_local, Span::generated("local")));
                self.resolve_class_method(&parent_name, method)
            } else {
                self.resolve_static_class_method(&parent_name, method)
            }
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("super method `{}.{}` not found", parent_name, method),
                span: Some(span),

                phase: None,
            })?;
            lowered_args.extend(
                args.iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            self.append_class_method_captures(method_id, &mut lowered_args)?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(method_id),
                args: lowered_args,

                span: Span::generated("call"),
            });
        }

        // Static method dispatch
        if let Some(method_id) = self
            .ctx
            .classes
            .class_static_method_ids
            .get(&(receiver_name.to_owned(), method.to_owned()))
            .copied()
        {
            let receiver = self.lower_expr(object)?;
            let mut lowered_args = self.lower_function_call_args(method_id, receiver, args)?;
            self.append_class_method_captures(method_id, &mut lowered_args)?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(method_id),
                args: lowered_args,

                span: Span::generated("call"),
            });
        }

        // Final class dispatch: resolve local, determine class, find method
        let obj_local = self.resolve_local(receiver_name)?;

        // Ambient interface-typed receivers without a concrete class
        let array_like_methods = [
            "filter",
            "map",
            "forEach",
            "find",
            "findIndex",
            "some",
            "every",
            "reduce",
            "flatMap",
            "join",
            "at",
        ];
        let number_methods = ["toFixed", "toExponential", "toPrecision"];
        let promise_methods = ["then", "catch", "finally"];
        let regexp_methods = ["test", "exec", "compile"];
        let class_name_str = match self.ctx.classes.local_classes.get(&obj_local) {
            Some(c) => c.clone(),
            None if array_like_methods.contains(&method) => "Array".to_owned(),
            None if number_methods.contains(&method) => "Number".to_owned(),
            None if regexp_methods.contains(&method) => "RegExp".to_owned(),
            None if promise_methods.contains(&method) => {
                let intrinsic = match method {
                    "then" => RuntimeFn::PromiseThen,
                    "catch" => RuntimeFn::PromiseCatch,
                    "finally" => RuntimeFn::PromiseFinally,
                    _ => unreachable!(),
                };
                let mut lowered_args =
                    vec![LoweredExpr::Local(obj_local, Span::generated("local"))];
                lowered_args.extend(
                    args.iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?,
                );
                let expected_count: usize = match method {
                    "then" => 2,
                    "catch" | "finally" => 1,
                    _ => unreachable!(),
                };
                while lowered_args.len() < expected_count + 1 {
                    lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                }
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: lowered_args,
                    span: Span::generated("runtime_call"),
                });
            }
            None => {
                if receiver_name == "Constructor" && method == "supportedLocalesOf" {
                    return Ok(LoweredExpr::ArrayNew {
                        elements: Vec::new(),
                        span: Span::generated("array_new"),
                    });
                }
                if receiver_name == "durationFormat" && is_intl_duration_format_method(method) {
                    return self.lower_intl_duration_format_method(method, args);
                }
                if self
                    .ctx
                    .facts
                    .is_host_external(obj_local, HostExternalKind::Object)
                    || (method == "toString"
                        && self
                            .ctx
                            .facts
                            .is_host_external(obj_local, HostExternalKind::FunctionHandle))
                {
                    let args_array = ResolvedExpr::Array(
                        args.iter()
                            .cloned()
                            .map(ResolvedArrayElement::Present)
                            .collect(),
                    );
                    let receiver = if self.ctx.facts.env_cell_locals.contains(&obj_local) {
                        LoweredExpr::EnvCellGet(obj_local, Span::generated("env_cell_get"))
                    } else {
                        LoweredExpr::Local(obj_local, Span::generated("local"))
                    };
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::FunctionCallMethodHost,
                        args: vec![
                            object_kernel::ordinary_get(receiver.clone(), method, span),
                            receiver,
                            self.lower_expr(&args_array)?,
                        ],
                        span: Span::generated("runtime_call"),
                    });
                }
                // BigInt.prototype.toString/valueOf handling
                if self.ctx.facts.bigint_locals.contains(&obj_local) {
                    match method {
                        "toString" | "toLocaleString" => {
                            let mut bi_args =
                                vec![LoweredExpr::Local(obj_local, Span::generated("local"))];
                            bi_args.extend(
                                args.iter()
                                    .take(1)
                                    .map(|e| self.lower_expr(e))
                                    .collect::<Result<Vec<_>, _>>()?,
                            );
                            if bi_args.len() == 1 {
                                bi_args.push(LoweredExpr::Undefined(Span::generated("radix")));
                            }
                            return Ok(LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::BigIntToString,
                                args: bi_args,
                                span: Span::generated("runtime_call"),
                            });
                        }
                        "valueOf" => {
                            return Ok(LoweredExpr::Local(obj_local, Span::generated("local")));
                        }
                        _ => {}
                    }
                }
                // Object.prototype methods: route to RuntimeFn for untyped receivers
                let obj_methods = [
                    "hasOwnProperty",
                    "propertyIsEnumerable",
                    "isPrototypeOf",
                    "toString",
                    "toLocaleString",
                    "valueOf",
                ];
                if obj_methods.contains(&method) {
                    let intrinsic = resolve_method_to_runtime_fn(
                        &ResolvedExpr::Ident(receiver_name.to_string()),
                        method,
                    )
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!("method `{}` not found for untyped receiver", method),
                        span: Some(span),
                        phase: None,
                    })?;
                    let mut lowered_args =
                        vec![LoweredExpr::Local(obj_local, Span::generated("local"))];
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic,
                        args: lowered_args,
                        span: Span::generated("runtime_call"),
                    });
                }
                // Generic fallback: route known method names through
                // resolve_method_to_runtime_fn for methods that have defined
                // runtime functions (e.g., String methods like "substr").
                // This allows untyped/ambient receivers to call instance methods
                // that the runtime already supports.
                // Skip methods that are ambiguous between String and Array
                // (at, slice, indexOf, lastIndexOf, includes, concat) — these
                // would be misrouted to String variants for Array receivers.
                let is_ambiguous = matches!(
                    method,
                    "at" | "slice" | "indexOf" | "lastIndexOf" | "includes" | "concat"
                );
                if !is_ambiguous
                    && let Some(intrinsic) = resolve_method_to_runtime_fn(
                        &ResolvedExpr::Ident(receiver_name.to_string()),
                        method,
                    )
                {
                    let mut lowered_args =
                        vec![LoweredExpr::Local(obj_local, Span::generated("local"))];
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic,
                        args: lowered_args,
                        span: Span::generated("runtime_call"),
                    });
                }

                // RegExp.prototype.compile — emit known-unsupported diagnostic
                if method == "compile" {
                    return Err(unsupported_regexp_compile_diagnostic(Some(span)));
                }

                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-211: unknown receiver class for method `{}` (receiver `{}` is an untyped or ambient variable; issue-5261: the method may be a static member or not exist on the instance type)",
                        method, receiver_name
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
        };
        let class_name = class_name_str.as_str();

        // RegExp.prototype.compile — emit known-unsupported diagnostic
        if class_name == "RegExp" && method == "compile" {
            return Err(unsupported_regexp_compile_diagnostic(Some(span)));
        }

        // BigInt.prototype.toString / valueOf
        if class_name == "BigInt" {
            match method {
                "toString" | "toLocaleString" => {
                    let mut bi_args = vec![LoweredExpr::Local(obj_local, Span::generated("local"))];
                    bi_args.extend(
                        args.iter()
                            .take(1)
                            .map(|e| self.lower_expr(e))
                            .collect::<Result<Vec<_>, _>>()?,
                    );
                    if bi_args.len() == 1 {
                        bi_args.push(LoweredExpr::Undefined(Span::generated("radix")));
                    }
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::BigIntToString,
                        args: bi_args,
                        span: Span::generated("runtime_call"),
                    });
                }
                "valueOf" => {
                    return Ok(LoweredExpr::Local(obj_local, Span::generated("local")));
                }
                _ => {}
            }
        }

        if class_name == "Array"
            && (method == "forEach"
                || method == "filter"
                || method == "find"
                || method == "findIndex"
                || method == "findLast"
                || method == "findLastIndex"
                || method == "some"
                || method == "every"
                || method == "reduce"
                || method == "reduceRight"
                || method == "map"
                || method == "flatMap")
            && !args.is_empty()
            && matches!(
                &args[0],
                ResolvedExpr::ArrowFn { .. }
                    | ResolvedExpr::FunctionExpr {
                        is_generator: false,
                        ..
                    }
            )
        {
            return self.lower_array_callback_method(
                method,
                LoweredExpr::Local(obj_local, Span::generated("local")),
                object,
                args,
                span,
            );
        }

        if let Some(intrinsic) = collection_method_runtime_fn(class_name, method) {
            if class_name == "Array"
                && method == "push"
                && let [ResolvedExpr::Spread(spread_expr)] = args
            {
                return self.lower_array_push_single_spread_arg(object, spread_expr.as_ref());
            }
            if class_name == "RegExp" && args.len() > 1 {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: format!(
                        "RegExp.prototype.{method} expects at most 1 argument, got {}",
                        args.len()
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            let receiver = LoweredExpr::Local(obj_local, Span::generated("local"));
            let lowered_args =
                self.lower_collection_method_args(receiver, class_name, method, args)?;
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,
                span: Span::generated("runtime_call"),
            });
        }

        if class_name == "Array"
            && method == "push"
            && let [ResolvedExpr::Spread(spread_expr)] = args
        {
            return self.lower_array_push_single_spread_arg(object, spread_expr.as_ref());
        }

        if class_name == "Number"
            && let Some(intrinsic) = number_format_method_runtime_fn(method)
        {
            let mut lowered_args = vec![LoweredExpr::Local(obj_local, Span::generated("local"))];
            lowered_args.extend(
                args.iter()
                    .take(1)
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            if lowered_args.len() == 1 {
                lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,
                span: Span::generated("runtime_call"),
            });
        }

        let method_id = self
            .resolve_class_method(class_name, method)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("method `{}.{}` not found", class_name, method),
                span: Some(span),

                phase: None,
            })?;

        let receiver = LoweredExpr::Local(obj_local, Span::generated("local"));
        let mut lowered_args = self.lower_function_call_args(method_id, receiver, args)?;
        self.append_class_method_captures(method_id, &mut lowered_args)?;

        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(method_id),
            args: lowered_args,

            span: Span::generated("call"),
        })
    }

    fn lower_collection_method_args(
        &mut self,
        receiver: LoweredExpr,
        class_name: &str,
        method: &str,
        args: &[ResolvedExpr],
    ) -> Result<Vec<LoweredExpr>, Diagnostic> {
        let mut lowered_args = vec![receiver];
        lowered_args.extend(
            args.iter()
                .map(|e| self.lower_expr(e))
                .collect::<Result<Vec<_>, _>>()?,
        );
        if class_name == "DataView" {
            match method {
                "getInt16" | "getUint16" | "getInt32" | "getUint32" | "getFloat32"
                | "getFloat64" | "getFloat16"
                    if args.len() == 1 =>
                {
                    lowered_args.push(LoweredExpr::Bool(false, Span::generated("bool")));
                }
                "setInt8" | "setUint8" if args.len() == 1 => {
                    lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
                }
                "setInt16" | "setUint16" | "setInt32" | "setUint32" | "setFloat32"
                | "setFloat64" | "setFloat16"
                    if args.len() == 1 =>
                {
                    lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
                    lowered_args.push(LoweredExpr::Bool(false, Span::generated("bool")));
                }
                "setInt16" | "setUint16" | "setInt32" | "setUint32" | "setFloat32"
                | "setFloat64" | "setFloat16"
                    if args.len() == 2 =>
                {
                    lowered_args.push(LoweredExpr::Bool(false, Span::generated("bool")));
                }
                _ => {}
            }
        } else if class_name == "Promise" {
            let expected_count: usize = match method {
                "then" => 2,
                "catch" | "finally" => 1,
                _ => args.len(),
            };
            while lowered_args.len() < expected_count + 1 {
                lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
            }
        } else if args.is_empty()
            && ((class_name == "ArrayBuffer" && method == "transfer")
                || (class_name == "Number" && is_number_format_method(method)))
        {
            lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
        } else if (class_name == "Array" || is_typed_array_class(class_name))
            && (method == "indexOf" || method == "includes")
        {
            // $array_index_of / $array_includes expect 3 params: $arr, $search, $from_idx.
            // Pad missing args: searchElement defaults to undefined, fromIndex defaults to 0.
            if lowered_args.len() < 2 {
                lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
            }
            if lowered_args.len() < 3 {
                lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
            }
        }
        // ArrayJoin expects a separator argument. Inject "," for toString/toLocaleString
        // on Array-like classes when no separator was explicitly passed (prototype.call path).
        if lowered_args.len() == 1
            && (method == "toString" || method == "toLocaleString")
            && (class_name == "Array" || is_typed_array_class(class_name))
        {
            lowered_args.push(LoweredExpr::String(",".to_owned(), Span::generated("str")));
        }
        Ok(lowered_args)
    }

    pub(crate) fn lower_intl_number_format_constructor(
        &mut self,
        args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        let options = self
            .intl_number_format_options_from_args(args)
            .unwrap_or_else(default_intl_number_format_options);
        Ok(LoweredExpr::ObjectNew {
            props: vec![
                ("locale".to_owned(), string_lit(options.locale)),
                ("style".to_owned(), string_lit(options.style)),
                ("currency".to_owned(), string_lit(options.currency)),
                ("notation".to_owned(), string_lit(options.notation)),
                (
                    "compactDisplay".to_owned(),
                    string_lit(options.compact_display),
                ),
                ("signDisplay".to_owned(), string_lit(options.sign_display)),
            ],
            non_enumerable: 0,
            span: Span::generated("intl_number_format"),
        })
    }

    pub(crate) fn lower_intl_duration_format_constructor(
        &mut self,
        _args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        Ok(intl_duration_format_options_object())
    }

    pub(crate) fn lower_intl_list_format_constructor(
        &mut self,
        _args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        Ok(intl_list_format_options_object())
    }

    pub(crate) fn intl_number_format_options_for_expr(
        &self,
        expr: &ResolvedExpr,
    ) -> Option<IntlNumberFormatOptions> {
        match expr {
            ResolvedExpr::New {
                class_name, args, ..
            } if is_intl_number_format_class(class_name.as_str()) => {
                self.intl_number_format_options_from_args(args)
            }
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                ..
            } if method == "NumberFormat"
                && matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "Intl") =>
            {
                self.intl_number_format_options_from_args(args)
            }
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().and_then(|local| {
                self.ctx
                    .facts
                    .intl_number_format_locals
                    .get(&local)
                    .cloned()
            }),
            _ => None,
        }
    }

    pub(crate) fn intl_number_format_options_from_args(
        &self,
        args: &[ResolvedExpr],
    ) -> Option<IntlNumberFormatOptions> {
        if args.len() > 2 {
            return None;
        }
        let defaults = default_intl_number_format_options();
        let locale = args
            .first()
            .and_then(static_string_expr)
            .unwrap_or(defaults.locale.as_str())
            .to_owned();
        let options = args.get(1);
        Some(IntlNumberFormatOptions {
            locale,
            style: static_object_string_option(options, "style")
                .unwrap_or(defaults.style.as_str())
                .to_owned(),
            currency: static_object_string_option(options, "currency")
                .unwrap_or(defaults.currency.as_str())
                .to_owned(),
            notation: static_object_string_option(options, "notation")
                .unwrap_or(defaults.notation.as_str())
                .to_owned(),
            compact_display: static_object_string_option(options, "compactDisplay")
                .unwrap_or(defaults.compact_display.as_str())
                .to_owned(),
            sign_display: static_object_string_option(options, "signDisplay")
                .unwrap_or(defaults.sign_display.as_str())
                .to_owned(),
        })
    }

    fn is_intl_number_format_expr(&self, expr: &ResolvedExpr) -> bool {
        matches!(
            self.infer_class_for_expr(expr).as_deref(),
            Some("Intl.NumberFormat" | "NumberFormat")
        )
    }

    fn is_intl_date_time_format_expr(&self, expr: &ResolvedExpr) -> bool {
        matches!(
            self.infer_class_for_expr(expr).as_deref(),
            Some("Intl.DateTimeFormat" | "DateTimeFormat")
        )
    }

    fn is_intl_duration_format_expr(&self, expr: &ResolvedExpr) -> bool {
        matches!(
            self.infer_class_for_expr(expr).as_deref(),
            Some("Intl.DurationFormat" | "DurationFormat")
        )
    }

    fn is_intl_list_format_expr(&self, expr: &ResolvedExpr) -> bool {
        matches!(
            self.infer_class_for_expr(expr).as_deref(),
            Some("Intl.ListFormat" | "ListFormat")
        )
    }

    fn lower_intl_number_format_method(
        &mut self,
        method: &str,
        args: &[ResolvedExpr],
        options: Option<&IntlNumberFormatOptions>,
    ) -> Result<LoweredExpr, Diagnostic> {
        let defaults = default_intl_number_format_options();
        let options = options.unwrap_or(&defaults);
        match method {
            "format" => {
                if let Some(static_val) = args
                    .first()
                    .and_then(|arg| static_number_format_arg(arg, options))
                {
                    return Ok(string_lit(static_val));
                }
                // Dynamic arg: emit RuntimeCall to host shim
                let lowered_arg = self.lower_expr(args.first().ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "Intl.NumberFormat.format requires an argument".to_owned(),
                    span: None,
                    phase: None,
                })?)?;
                let options_json = serialize_intl_options(options);
                Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::IntlNumberFormatFormat,
                    args: vec![lowered_arg, string_lit(options_json)],
                    span: Span::generated("runtime_call"),
                })
            }
            "formatToParts" => {
                if let Some(static_val) = args
                    .first()
                    .and_then(|arg| static_number_format_arg(arg, options))
                {
                    return Ok(LoweredExpr::ArrayNew {
                        elements: vec![LoweredExpr::ObjectNew {
                            props: vec![
                                (
                                    "type".to_owned(),
                                    string_lit(number_format_part_type(options)),
                                ),
                                ("value".to_owned(), string_lit(static_val)),
                            ],
                            non_enumerable: 0,
                            span: Span::generated("object_new"),
                        }],
                        span: Span::generated("array_new"),
                    });
                }
                // Dynamic arg: emit RuntimeCall to host shim
                let lowered_arg = self.lower_expr(args.first().ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "Intl.NumberFormat.formatToParts requires an argument".to_owned(),
                    span: None,
                    phase: None,
                })?)?;
                let options_json = serialize_intl_options(options);
                // At runtime formatToParts returns a JSON string that the caller must parse
                Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::IntlNumberFormatFormat,
                    args: vec![lowered_arg, string_lit(options_json)],
                    span: Span::generated("runtime_call"),
                })
            }
            "resolvedOptions" => Ok(LoweredExpr::ObjectNew {
                props: vec![
                    ("locale".to_owned(), string_lit(options.locale.clone())),
                    ("numberingSystem".to_owned(), string_lit("latn")),
                    ("style".to_owned(), string_lit(options.style.clone())),
                    ("currency".to_owned(), string_lit(options.currency.clone())),
                    ("notation".to_owned(), string_lit(options.notation.clone())),
                    (
                        "compactDisplay".to_owned(),
                        string_lit(options.compact_display.clone()),
                    ),
                    (
                        "signDisplay".to_owned(),
                        string_lit(options.sign_display.clone()),
                    ),
                ],
                non_enumerable: 0,
                span: Span::generated("object_new"),
            }),
            _ => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("Intl.NumberFormat.prototype.{method} is not supported"),
                span: None,
                phase: None,
            }),
        }
    }

    pub(crate) fn lower_intl_date_time_format_constructor(
        &mut self,
        args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        let options = self
            .intl_date_time_format_options_from_args(args)
            .unwrap_or_else(default_intl_date_time_format_options);
        Ok(LoweredExpr::ObjectNew {
            props: vec![
                (
                    "__class".to_owned(),
                    string_lit("Intl.DateTimeFormat".to_owned()),
                ),
                ("locale".to_owned(), string_lit(options.locale)),
                ("timeZone".to_owned(), string_lit(options.time_zone)),
                (
                    "localeMatcher".to_owned(),
                    string_lit(options.locale_matcher),
                ),
            ],
            non_enumerable: 0,
            span: Span::generated("intl_date_time_format"),
        })
    }

    pub(crate) fn intl_date_time_format_options_for_expr(
        &self,
        expr: &ResolvedExpr,
    ) -> Option<IntlDateTimeFormatOptions> {
        match expr {
            ResolvedExpr::New {
                class_name, args, ..
            } if matches!(
                class_name.as_str(),
                "Intl.DateTimeFormat" | "DateTimeFormat"
            ) =>
            {
                self.intl_date_time_format_options_from_args(args)
            }
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                ..
            } if method == "DateTimeFormat"
                && matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "Intl") =>
            {
                self.intl_date_time_format_options_from_args(args)
            }
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().and_then(|local| {
                self.ctx
                    .facts
                    .intl_date_time_format_locals
                    .get(&local)
                    .cloned()
            }),
            _ => None,
        }
    }

    pub(crate) fn intl_date_time_format_options_from_args(
        &self,
        args: &[ResolvedExpr],
    ) -> Option<IntlDateTimeFormatOptions> {
        if args.len() > 2 {
            return None;
        }
        let defaults = default_intl_date_time_format_options();
        let locale = args
            .first()
            .and_then(static_string_expr)
            .unwrap_or(defaults.locale.as_str())
            .to_owned();
        let options = args.get(1);
        Some(IntlDateTimeFormatOptions {
            locale,
            time_zone: static_object_string_option(options, "timeZone")
                .unwrap_or(defaults.time_zone.as_str())
                .to_owned(),
            locale_matcher: static_object_string_option(options, "localeMatcher")
                .unwrap_or(defaults.locale_matcher.as_str())
                .to_owned(),
        })
    }

    fn lower_intl_date_time_format_method(
        &mut self,
        method: &str,
        args: &[ResolvedExpr],
        options: Option<&IntlDateTimeFormatOptions>,
    ) -> Result<LoweredExpr, Diagnostic> {
        let defaults = default_intl_date_time_format_options();
        let options = options.unwrap_or(&defaults);
        match method {
            "format" => {
                if static_epoch_ms_date(args.first()).is_some() {
                    return Ok(string_lit(format_intl_datetime_arg(args.first(), options)));
                }
                // Dynamic arg: emit RuntimeCall to host shim
                let lowered_arg = self.lower_expr(args.first().ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "Intl.DateTimeFormat.format requires an argument".to_owned(),
                    span: None,
                    phase: None,
                })?)?;
                let options_json = serialize_intl_date_time_options(options);
                Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::IntlDateTimeFormatFormat,
                    args: vec![lowered_arg, string_lit(options_json)],
                    span: Span::generated("runtime_call"),
                })
            }
            "formatToParts" => {
                let parts = format_intl_datetime_parts(args.first(), options);
                Ok(LoweredExpr::ArrayNew {
                    elements: parts
                        .into_iter()
                        .map(|(part_type, value)| LoweredExpr::ObjectNew {
                            props: vec![
                                ("type".to_owned(), string_lit(part_type)),
                                ("value".to_owned(), string_lit(value)),
                            ],
                            non_enumerable: 0,
                            span: Span::generated("object_new"),
                        })
                        .collect(),
                    span: Span::generated("array_new"),
                })
            }
            "resolvedOptions" => Ok(LoweredExpr::ObjectNew {
                props: vec![
                    ("locale".to_owned(), string_lit(options.locale.clone())),
                    ("calendar".to_owned(), string_lit("gregory")),
                    ("numberingSystem".to_owned(), string_lit("latn")),
                    ("timeZone".to_owned(), string_lit(options.time_zone.clone())),
                    (
                        "localeMatcher".to_owned(),
                        string_lit(options.locale_matcher.clone()),
                    ),
                ],
                non_enumerable: 0,
                span: Span::generated("object_new"),
            }),
            _ => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("Intl.DateTimeFormat.prototype.{method} is not supported"),
                span: None,
                phase: None,
            }),
        }
    }

    fn lower_intl_duration_format_method(
        &mut self,
        method: &str,
        _args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        match method {
            "format" => Ok(string_lit("")),
            "formatToParts" => Ok(LoweredExpr::ArrayNew {
                elements: Vec::new(),
                span: Span::generated("array_new"),
            }),
            "resolvedOptions" => Ok(intl_duration_format_options_object()),
            _ => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("Intl.DurationFormat.prototype.{method} is not supported"),
                span: None,
                phase: None,
            }),
        }
    }

    fn lower_intl_list_format_method(
        &mut self,
        method: &str,
        _args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        match method {
            "format" => Ok(string_lit("")),
            "formatToParts" => Ok(LoweredExpr::ArrayNew {
                elements: vec![intl_list_format_part_object()],
                span: Span::generated("array_new"),
            }),
            "resolvedOptions" => Ok(intl_list_format_options_object()),
            _ => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("Intl.ListFormat.prototype.{method} is not supported"),
                span: None,
                phase: None,
            }),
        }
    }

    fn lower_object_prototype_dispatch(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        if !is_object_prototype_method(method) {
            return Ok(None);
        }
        let is_object_receiver =
            matches!(self.infer_class_for_expr(object).as_deref(), Some("Object"))
                || matches!(object, ResolvedExpr::Object(_));
        if !is_object_receiver {
            return Ok(None);
        }
        let lowered_receiver = self.lower_expr(object)?;
        Ok(Some(self.lower_object_prototype_method(
            lowered_receiver,
            method,
            args,
            span,
        )?))
    }

    fn lower_object_prototype_method(
        &mut self,
        receiver: LoweredExpr,
        method: &str,
        args: &[ResolvedExpr],
        _span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        match method {
            "hasOwnProperty" => {
                let key = args
                    .first()
                    .map(|arg| self.lower_expr(arg))
                    .transpose()?
                    .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
                Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ObjectHasOwnProperty,
                    args: vec![receiver, key],
                    span: Span::generated("runtime_call"),
                })
            }
            "propertyIsEnumerable" => {
                let key = args
                    .first()
                    .map(|arg| self.lower_expr(arg))
                    .transpose()?
                    .unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("undef")));
                let desc = self.alloc_temp();
                let result = self.alloc_temp();
                Ok(LoweredExpr::Block {
                    stmts: vec![
                        LoweredStmt::Let(
                            desc,
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::ObjectGetOwnPropertyDescriptor,
                                args: vec![receiver, key],
                                span: Span::generated("runtime_call"),
                            },
                            Span::generated("let_stmt"),
                        ),
                        LoweredStmt::Let(
                            result,
                            LoweredExpr::Bool(false, Span::generated("bool")),
                            Span::generated("let_stmt"),
                        ),
                        LoweredStmt::If {
                            condition: LoweredExpr::Binary {
                                left: Box::new(LoweredExpr::Local(desc, Span::generated("local"))),
                                op: LoweredBinaryOp::StrictNotEqual,
                                right: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                                span: Span::generated("binary"),
                            },
                            then_body: vec![LoweredStmt::Assign(
                                result,
                                LoweredExpr::PropertyGet {
                                    obj: Box::new(LoweredExpr::Local(
                                        desc,
                                        Span::generated("local"),
                                    )),
                                    key: "enumerable".to_owned(),
                                    span: Span::generated("property_get"),
                                },
                                Span::generated("assign"),
                            )],
                            else_body: Vec::new(),
                            span: Span::generated("if_stmt"),
                        },
                    ],
                    result: Box::new(LoweredExpr::Local(result, Span::generated("local"))),
                    span: Span::generated("object_property_is_enumerable"),
                })
            }
            "isPrototypeOf" => {
                let Some(candidate) = args.first() else {
                    return Ok(LoweredExpr::Bool(false, Span::generated("bool")));
                };
                let current = self.alloc_temp();
                let found = self.alloc_temp();
                Ok(LoweredExpr::Block {
                    stmts: vec![
                        LoweredStmt::Let(
                            current,
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::ObjectGetPrototypeOf,
                                args: vec![self.lower_expr(candidate)?],
                                span: Span::generated("runtime_call"),
                            },
                            Span::generated("let_stmt"),
                        ),
                        LoweredStmt::Let(
                            found,
                            LoweredExpr::Bool(false, Span::generated("bool")),
                            Span::generated("let_stmt"),
                        ),
                        LoweredStmt::While {
                            condition: LoweredExpr::Binary {
                                left: Box::new(LoweredExpr::Binary {
                                    left: Box::new(LoweredExpr::Local(
                                        current,
                                        Span::generated("local"),
                                    )),
                                    op: LoweredBinaryOp::StrictNotEqual,
                                    right: Box::new(LoweredExpr::Null(Span::generated("null"))),
                                    span: Span::generated("binary"),
                                }),
                                op: LoweredBinaryOp::And,
                                right: Box::new(LoweredExpr::Binary {
                                    left: Box::new(LoweredExpr::Local(
                                        current,
                                        Span::generated("local"),
                                    )),
                                    op: LoweredBinaryOp::StrictNotEqual,
                                    right: Box::new(LoweredExpr::Undefined(Span::generated(
                                        "undef",
                                    ))),
                                    span: Span::generated("binary"),
                                }),
                                span: Span::generated("binary"),
                            },
                            body: vec![LoweredStmt::If {
                                condition: LoweredExpr::Binary {
                                    left: Box::new(LoweredExpr::Local(
                                        current,
                                        Span::generated("local"),
                                    )),
                                    op: LoweredBinaryOp::StrictEqual,
                                    right: Box::new(receiver),
                                    span: Span::generated("binary"),
                                },
                                then_body: vec![
                                    LoweredStmt::Assign(
                                        found,
                                        LoweredExpr::Bool(true, Span::generated("bool")),
                                        Span::generated("assign"),
                                    ),
                                    LoweredStmt::Assign(
                                        current,
                                        LoweredExpr::Null(Span::generated("null")),
                                        Span::generated("assign"),
                                    ),
                                ],
                                else_body: vec![LoweredStmt::Assign(
                                    current,
                                    LoweredExpr::RuntimeCall {
                                        intrinsic: RuntimeFn::ObjectGetPrototypeOf,
                                        args: vec![LoweredExpr::Local(
                                            current,
                                            Span::generated("local"),
                                        )],
                                        span: Span::generated("runtime_call"),
                                    },
                                    Span::generated("assign"),
                                )],
                                span: Span::generated("if_stmt"),
                            }],
                            span: Span::generated("while"),
                        },
                    ],
                    result: Box::new(LoweredExpr::Local(found, Span::generated("local"))),
                    span: Span::generated("object_is_prototype_of"),
                })
            }
            "toString" | "toLocaleString" => Ok(LoweredExpr::String(
                "[object Object]".to_owned(),
                Span::generated("str"),
            )),
            "valueOf" => Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ValueOf,
                args: vec![receiver],
                span: Span::generated("runtime_call"),
            }),
            _ => Ok(LoweredExpr::Undefined(Span::generated("undef"))),
        }
    }
}

fn is_object_prototype_method(method: &str) -> bool {
    matches!(
        method,
        "hasOwnProperty"
            | "propertyIsEnumerable"
            | "isPrototypeOf"
            | "toString"
            | "valueOf"
            | "toLocaleString"
    )
}

fn is_intl_number_format_method(method: &str) -> bool {
    matches!(method, "format" | "formatToParts" | "resolvedOptions")
}

fn is_intl_date_time_format_method(method: &str) -> bool {
    matches!(
        method,
        "format" | "formatRange" | "formatToParts" | "formatRangeToParts" | "resolvedOptions"
    )
}

fn is_intl_duration_format_method(method: &str) -> bool {
    matches!(method, "format" | "formatToParts" | "resolvedOptions")
}

fn is_intl_list_format_method(method: &str) -> bool {
    matches!(method, "format" | "formatToParts" | "resolvedOptions")
}

fn is_number_format_method(method: &str) -> bool {
    matches!(method, "toFixed" | "toExponential" | "toPrecision")
}

fn is_number_format_runtime_fn(intrinsic: RuntimeFn) -> bool {
    matches!(
        intrinsic,
        RuntimeFn::NumberToFixed | RuntimeFn::NumberToExponential | RuntimeFn::NumberToPrecision
    )
}

fn is_intl_number_format_class(class_name: &str) -> bool {
    matches!(
        class_name,
        "Intl.NumberFormat" | "NumberFormat" | "Constructor"
    )
}

fn intl_duration_format_options_object() -> LoweredExpr {
    let mut props = vec![
        ("locale".to_owned(), string_lit("en")),
        ("numberingSystem".to_owned(), string_lit("latn")),
        ("style".to_owned(), string_lit("short")),
        (
            "fractionalDigits".to_owned(),
            LoweredExpr::Number(0, Span::generated("num")),
        ),
    ];
    for unit in [
        "years",
        "months",
        "weeks",
        "days",
        "hours",
        "minutes",
        "seconds",
        "milliseconds",
        "microseconds",
        "nanoseconds",
    ] {
        props.push((unit.to_owned(), string_lit("numeric")));
        props.push((format!("{unit}Display"), string_lit("auto")));
    }
    LoweredExpr::ObjectNew {
        props,
        non_enumerable: 0,
        span: Span::generated("object_new"),
    }
}

fn intl_list_format_options_object() -> LoweredExpr {
    LoweredExpr::ObjectNew {
        props: vec![
            ("locale".to_owned(), string_lit("en")),
            ("type".to_owned(), string_lit("unit")),
            ("style".to_owned(), string_lit("short")),
        ],
        non_enumerable: 0,
        span: Span::generated("object_new"),
    }
}

fn intl_list_format_part_object() -> LoweredExpr {
    LoweredExpr::ObjectNew {
        props: vec![
            ("type".to_owned(), string_lit("element")),
            ("value".to_owned(), string_lit("")),
        ],
        non_enumerable: 0,
        span: Span::generated("object_new"),
    }
}

fn is_intl_date_time_format_class(class_name: &str) -> bool {
    matches!(class_name, "Intl.DateTimeFormat" | "DateTimeFormat")
}

fn static_string_expr(expr: &ResolvedExpr) -> Option<&str> {
    match expr {
        ResolvedExpr::String(value) => Some(value),
        _ => None,
    }
}

fn static_object_string_option<'a>(expr: Option<&'a ResolvedExpr>, key: &str) -> Option<&'a str> {
    let Some(ResolvedExpr::Object(props)) = expr else {
        return None;
    };
    props
        .iter()
        .rev()
        .find(|prop| prop.static_key() == Some(key))
        .and_then(|prop| static_string_expr(prop.value()))
}

fn static_number_format_arg(
    expr: &ResolvedExpr,
    options: &IntlNumberFormatOptions,
) -> Option<String> {
    match expr {
        ResolvedExpr::Number(value) => Some(format_intl_number_i32(*value, options)),
        ResolvedExpr::DecimalNumber(value) => {
            Some(apply_intl_number_affixes(value.clone(), options))
        }
        _ => None,
    }
}

fn default_intl_number_format_options() -> IntlNumberFormatOptions {
    IntlNumberFormatOptions {
        locale: "en-US".to_owned(),
        style: "decimal".to_owned(),
        currency: String::new(),
        notation: "standard".to_owned(),
        compact_display: "short".to_owned(),
        sign_display: "auto".to_owned(),
    }
}

fn number_format_part_type(options: &IntlNumberFormatOptions) -> &'static str {
    match options.style.as_str() {
        "currency" => "currency",
        "percent" => "percent",
        _ => "integer",
    }
}

fn format_intl_number_i32(value: i32, options: &IntlNumberFormatOptions) -> String {
    let scaled = if options.style == "percent" {
        value.saturating_mul(100)
    } else {
        value
    };
    let formatted = if options.notation == "compact" {
        format_compact_i32(scaled, options)
    } else {
        format_i32_grouped(scaled)
    };
    apply_intl_number_affixes(formatted, options)
}

fn apply_intl_number_affixes(mut formatted: String, options: &IntlNumberFormatOptions) -> String {
    if options.sign_display == "never" && formatted.starts_with('-') {
        formatted.remove(0);
    } else if matches!(options.sign_display.as_str(), "always" | "exceptZero")
        && !formatted.starts_with('-')
        && formatted != "0"
    {
        formatted.insert(0, '+');
    }

    match options.style.as_str() {
        "currency" if options.currency == "USD" => format!("${formatted}.00"),
        "currency" if !options.currency.is_empty() => format!("{} {formatted}", options.currency),
        "percent" => format!("{formatted}%"),
        _ => formatted,
    }
}

fn static_number_format_method(
    ctx: &LoweringCtx,
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
) -> Option<String> {
    let value =
        crate::lowered::resolver::string::resolved_expr_static_number_literal_value(ctx, object)?;
    let precision = args.first().and_then(static_number_format_precision);
    match method {
        "toFixed" => Some(format_fixed_decimal(&value, precision.unwrap_or(0))),
        "toExponential" => Some(format_exponential_decimal(&value, precision)),
        "toPrecision" => match precision {
            Some(precision) => Some(format_precision_decimal(&value, precision.max(1))),
            None => Some(value),
        },
        _ => None,
    }
}

fn static_number_format_precision(expr: &ResolvedExpr) -> Option<usize> {
    match expr {
        ResolvedExpr::Number(value) if *value >= 0 => Some(*value as usize),
        _ => None,
    }
}

fn decimal_parts(value: &str) -> (bool, String, String) {
    let (negative, unsigned) = match value.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, value),
    };
    let (int_part, frac_part) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    let int_part = if int_part.is_empty() { "0" } else { int_part };
    (negative, int_part.to_owned(), frac_part.to_owned())
}

fn round_decimal_digits(mut digits: Vec<u8>, keep: usize) -> Vec<u8> {
    while digits.len() <= keep {
        digits.push(b'0');
    }
    let round_up = digits.get(keep).is_some_and(|digit| *digit >= b'5');
    digits.truncate(keep);
    while digits.len() < keep {
        digits.push(b'0');
    }
    if round_up {
        let mut index = digits.len();
        loop {
            if index == 0 {
                digits.insert(0, b'1');
                break;
            }
            index -= 1;
            if digits[index] == b'9' {
                digits[index] = b'0';
            } else {
                digits[index] += 1;
                break;
            }
        }
    }
    digits
}

fn format_fixed_decimal(value: &str, frac_digits: usize) -> String {
    let (negative, int_part, frac_part) = decimal_parts(value);
    let mut int_len = int_part.len();
    let mut digits = int_part.into_bytes();
    digits.extend(frac_part.bytes());
    let keep = int_len + frac_digits;
    let mut rounded = round_decimal_digits(digits, keep);
    if rounded.len() > keep {
        int_len += 1;
    }
    while rounded.len() < int_len + frac_digits {
        rounded.push(b'0');
    }

    let mut output = String::new();
    if negative {
        output.push('-');
    }
    output.push_str(std::str::from_utf8(&rounded[..int_len]).unwrap_or("0"));
    if frac_digits > 0 {
        output.push('.');
        output
            .push_str(std::str::from_utf8(&rounded[int_len..int_len + frac_digits]).unwrap_or(""));
    }
    output
}

fn significant_digits_and_exponent(value: &str) -> (bool, Vec<u8>, i32) {
    let (negative, int_part, frac_part) = decimal_parts(value);
    let decimal_index = int_part.len();
    let mut digits = int_part.into_bytes();
    digits.extend(frac_part.bytes());
    if let Some(first_non_zero) = digits.iter().position(|digit| *digit != b'0') {
        let exponent = decimal_index as i32 - first_non_zero as i32 - 1;
        (negative, digits[first_non_zero..].to_vec(), exponent)
    } else {
        (negative, vec![b'0'], 0)
    }
}

fn round_significant_digits(digits: Vec<u8>, keep: usize, exponent: &mut i32) -> Vec<u8> {
    let rounded = round_decimal_digits(digits, keep);
    if rounded.len() > keep {
        *exponent += 1;
    }
    rounded
}

fn format_exponential_decimal(value: &str, frac_digits: Option<usize>) -> String {
    let (negative, digits, mut exponent) = significant_digits_and_exponent(value);
    let keep = frac_digits.map_or(digits.len().max(1), |digits| digits + 1);
    let mut rounded = round_significant_digits(digits, keep, &mut exponent);
    while rounded.len() < keep {
        rounded.push(b'0');
    }

    let mut output = String::new();
    if negative {
        output.push('-');
    }
    output.push(rounded[0] as char);
    if keep > 1 {
        output.push('.');
        output.push_str(std::str::from_utf8(&rounded[1..keep]).unwrap_or(""));
    }
    output.push('e');
    if exponent >= 0 {
        output.push('+');
    }
    output.push_str(&exponent.to_string());
    output
}

fn format_precision_decimal(value: &str, precision: usize) -> String {
    let (_, _, exponent) = significant_digits_and_exponent(value);
    if exponent + 1 > precision as i32 || exponent < -6 {
        return format_exponential_decimal(value, Some(precision.saturating_sub(1)));
    }
    let frac_digits = precision.saturating_sub((exponent + 1).max(0) as usize);
    format_fixed_decimal(value, frac_digits)
}

fn format_compact_i32(value: i32, options: &IntlNumberFormatOptions) -> String {
    let negative = value < 0;
    let abs = (value as i64).abs();
    let (divisor, short_suffix, long_suffix) = if abs >= 1_000_000_000 {
        (1_000_000_000_i64, "B", " billion")
    } else if abs >= 1_000_000 {
        (1_000_000_i64, "M", " million")
    } else if abs >= 1_000 {
        (1_000_i64, "K", " thousand")
    } else {
        return format_i32_grouped(value);
    };
    let whole = abs / divisor;
    let first_decimal = (abs % divisor) * 10 / divisor;
    let mut formatted = if first_decimal == 0 {
        whole.to_string()
    } else {
        format!("{whole}.{first_decimal}")
    };
    if options.compact_display == "long" {
        formatted.push_str(long_suffix);
    } else {
        formatted.push_str(short_suffix);
    }
    if negative {
        formatted.insert(0, '-');
    }
    formatted
}

fn format_i32_grouped(value: i32) -> String {
    let negative = value < 0;
    let digits = (value as i64).abs().to_string();
    let mut grouped = String::new();
    for (idx, ch) in digits.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(ch);
    }
    let mut formatted = grouped.chars().rev().collect::<String>();
    if negative {
        formatted.insert(0, '-');
    }
    formatted
}

fn default_intl_date_time_format_options() -> IntlDateTimeFormatOptions {
    IntlDateTimeFormatOptions {
        locale: "en-US".to_owned(),
        time_zone: "UTC".to_owned(),
        locale_matcher: "best fit".to_owned(),
    }
}

fn format_intl_datetime_arg(
    expr: Option<&ResolvedExpr>,
    options: &IntlDateTimeFormatOptions,
) -> String {
    let (year, month, day) = static_epoch_ms_date(expr)
        .map(epoch_ms_to_utc_ymd)
        .unwrap_or((1970, 1, 1));
    match options.locale.as_str() {
        "en-GB" => format!("{day:02}/{month:02}/{year:04}"),
        _ => format!("{month}/{day}/{year}"),
    }
}

fn format_intl_datetime_parts(
    expr: Option<&ResolvedExpr>,
    options: &IntlDateTimeFormatOptions,
) -> Vec<(String, String)> {
    let (year, month, day) = static_epoch_ms_date(expr)
        .map(epoch_ms_to_utc_ymd)
        .unwrap_or((1970, 1, 1));
    if options.locale == "en-GB" {
        return vec![
            ("day".to_owned(), format!("{day:02}")),
            ("literal".to_owned(), "/".to_owned()),
            ("month".to_owned(), format!("{month:02}")),
            ("literal".to_owned(), "/".to_owned()),
            ("year".to_owned(), format!("{year:04}")),
        ];
    }
    vec![
        ("month".to_owned(), month.to_string()),
        ("literal".to_owned(), "/".to_owned()),
        ("day".to_owned(), day.to_string()),
        ("literal".to_owned(), "/".to_owned()),
        ("year".to_owned(), year.to_string()),
    ]
}

fn static_epoch_ms_date(expr: Option<&ResolvedExpr>) -> Option<i64> {
    match expr? {
        ResolvedExpr::Number(value) => Some(i64::from(*value)),
        ResolvedExpr::DecimalNumber(value) => value.parse::<f64>().ok().map(|v| v as i64),
        _ => None,
    }
}

fn epoch_ms_to_utc_ymd(epoch_ms: i64) -> (i32, u32, u32) {
    let days = epoch_ms.div_euclid(86_400_000);
    civil_from_days(days)
}

fn civil_from_days(days_since_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year as i32, m as u32, d as u32)
}

/// Serialize IntlNumberFormatOptions to a JSON string for the host shim.
fn serialize_intl_options(options: &IntlNumberFormatOptions) -> String {
    format!(
        r#"{{"locale":"{}","style":"{}","currency":"{}","notation":"{}","compactDisplay":"{}","signDisplay":"{}"}}"#,
        options.locale,
        options.style,
        options.currency,
        options.notation,
        options.compact_display,
        options.sign_display,
    )
}

fn serialize_intl_date_time_options(options: &IntlDateTimeFormatOptions) -> String {
    format!(
        r#"{{"locale":"{}","timeZone":"{}","localeMatcher":"{}"}}"#,
        options.locale, options.time_zone, options.locale_matcher,
    )
}

fn string_lit(value: impl Into<String>) -> LoweredExpr {
    LoweredExpr::String(value.into(), Span::generated("str"))
}

fn static_number_format_method_call(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
) -> Option<String> {
    if !is_number_format_method(method) || args.len() > 1 {
        return None;
    }
    let value = static_i64_number_expr(object)?;
    let precision = args.first().and_then(static_usize_number_expr);
    if !args.is_empty() && precision.is_none() {
        return None;
    }
    match method {
        "toFixed" => Some(format_number_to_fixed_i64(value, precision.unwrap_or(0))),
        "toExponential" => format_number_to_exponential_i64(value, precision),
        "toPrecision" => format_number_to_precision_i64(value, precision),
        _ => None,
    }
}

fn static_i64_number_expr(expr: &ResolvedExpr) -> Option<i64> {
    match expr {
        ResolvedExpr::Number(value) => Some(i64::from(*value)),
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Negate => {
            static_i64_number_expr(expr).map(|value| -value)
        }
        _ => None,
    }
}

fn static_generator_completion_value(expr: &LoweredExpr) -> Option<LoweredExpr> {
    match expr {
        LoweredExpr::Number(..)
        | LoweredExpr::DecimalNumber(..)
        | LoweredExpr::BigIntLiteral { .. }
        | LoweredExpr::String(..)
        | LoweredExpr::Bool(..)
        | LoweredExpr::Null(..)
        | LoweredExpr::Undefined(..) => Some(expr.clone()),
        _ => None,
    }
}

fn static_generator_first_yield_value(body: &[LoweredStmt]) -> Option<LoweredExpr> {
    for stmt in body {
        match stmt {
            LoweredStmt::Yield(expr, _) => return Some(expr.clone()),
            LoweredStmt::Block(stmts, _) => {
                if let Some(expr) = static_generator_first_yield_value(stmts) {
                    return Some(expr);
                }
            }
            LoweredStmt::Let(_, expr, _)
            | LoweredStmt::Assign(_, expr, _)
            | LoweredStmt::Expr(expr, _)
                if static_generator_implicit_completion_expr_is_local_only(expr) => {}
            _ => return None,
        }
    }
    None
}

fn static_generator_bind_receiver(
    expr: LoweredExpr,
    receiver_param: Option<LocalId>,
    receiver_local: LocalId,
) -> Option<LoweredExpr> {
    let receiver_param = receiver_param?;
    let mut substitutions = HashMap::new();
    substitutions.insert(
        receiver_param,
        LoweredExpr::Local(receiver_local, Span::generated("local")),
    );
    static_generator_bind_locals(expr, &substitutions)
}

fn static_generator_bind_locals(
    expr: LoweredExpr,
    substitutions: &HashMap<LocalId, LoweredExpr>,
) -> Option<LoweredExpr> {
    match expr {
        LoweredExpr::Local(local, _) if substitutions.contains_key(&local) => {
            substitutions.get(&local).cloned()
        }
        LoweredExpr::Local(..) => None,
        LoweredExpr::Unary { op, expr, span } => Some(LoweredExpr::Unary {
            op,
            expr: Box::new(static_generator_bind_locals(*expr, substitutions)?),
            span,
        }),
        LoweredExpr::Binary {
            left,
            op,
            right,
            span,
        } => Some(LoweredExpr::Binary {
            left: Box::new(static_generator_bind_locals(*left, substitutions)?),
            op,
            right: Box::new(static_generator_bind_locals(*right, substitutions)?),
            span,
        }),
        LoweredExpr::RuntimeCall {
            intrinsic,
            args,
            span,
        } => Some(LoweredExpr::RuntimeCall {
            intrinsic,
            args: args
                .into_iter()
                .map(|arg| static_generator_bind_locals(arg, substitutions))
                .collect::<Option<Vec<_>>>()?,
            span,
        }),
        LoweredExpr::PropertyGet { obj, key, span } => Some(LoweredExpr::PropertyGet {
            obj: Box::new(static_generator_bind_locals(*obj, substitutions)?),
            key,
            span,
        }),
        LoweredExpr::PropertyGetDynamic { obj, key, span } => {
            Some(LoweredExpr::PropertyGetDynamic {
                obj: Box::new(static_generator_bind_locals(*obj, substitutions)?),
                key: Box::new(static_generator_bind_locals(*key, substitutions)?),
                span,
            })
        }
        LoweredExpr::Call { kind, args, span } => Some(LoweredExpr::Call {
            kind,
            args: args
                .into_iter()
                .map(|arg| static_generator_bind_locals(arg, substitutions))
                .collect::<Option<Vec<_>>>()?,
            span,
        }),
        LoweredExpr::ObjectNew {
            props,
            non_enumerable,
            span,
        } => Some(LoweredExpr::ObjectNew {
            props: props
                .into_iter()
                .map(|(key, value)| {
                    Some((key, static_generator_bind_locals(value, substitutions)?))
                })
                .collect::<Option<Vec<_>>>()?,
            non_enumerable,
            span,
        }),
        LoweredExpr::ArrayNew { elements, span } => Some(LoweredExpr::ArrayNew {
            elements: elements
                .into_iter()
                .map(|element| static_generator_bind_locals(element, substitutions))
                .collect::<Option<Vec<_>>>()?,
            span,
        }),
        LoweredExpr::Number(..)
        | LoweredExpr::DecimalNumber(..)
        | LoweredExpr::BigIntLiteral { .. }
        | LoweredExpr::String(..)
        | LoweredExpr::Bool(..)
        | LoweredExpr::Null(..)
        | LoweredExpr::Undefined(..)
        | LoweredExpr::ArrowFn { .. } => Some(expr),
        _ => None,
    }
}

fn replace_direct_computed_yield_keys(
    props: &[ResolvedObjectProp],
    resume_args: &[ResolvedExpr],
) -> Vec<ResolvedObjectProp> {
    let mut resume_index = 0;
    props
        .iter()
        .map(|prop| match prop {
            ResolvedObjectProp::ComputedKey { key, value }
                if matches!(
                    key.as_ref(),
                    ResolvedExpr::Yield {
                        delegate: false,
                        ..
                    }
                ) =>
            {
                let key = resume_args
                    .get(resume_index)
                    .cloned()
                    .unwrap_or(ResolvedExpr::Undefined);
                resume_index += 1;
                ResolvedObjectProp::ComputedKey {
                    key: Box::new(key),
                    value: value.clone(),
                }
            }
            _ => prop.clone(),
        })
        .collect()
}

fn static_generator_implicit_completion_value(body: &[LoweredStmt]) -> Option<LoweredExpr> {
    body.iter()
        .all(static_generator_implicit_completion_stmt_is_local_only)
        .then(|| LoweredExpr::Undefined(Span::generated("undefined")))
}

fn static_generator_implicit_completion_stmt_is_local_only(stmt: &LoweredStmt) -> bool {
    match stmt {
        LoweredStmt::Block(stmts, _) => stmts
            .iter()
            .all(static_generator_implicit_completion_stmt_is_local_only),
        LoweredStmt::Let(_, expr, _)
        | LoweredStmt::Assign(_, expr, _)
        | LoweredStmt::Expr(expr, _) => {
            static_generator_implicit_completion_expr_is_local_only(expr)
        }
        _ => false,
    }
}

fn static_generator_implicit_completion_expr_is_local_only(expr: &LoweredExpr) -> bool {
    match expr {
        LoweredExpr::Number(..)
        | LoweredExpr::DecimalNumber(..)
        | LoweredExpr::BigIntLiteral { .. }
        | LoweredExpr::String(..)
        | LoweredExpr::Bool(..)
        | LoweredExpr::Null(..)
        | LoweredExpr::Undefined(..)
        | LoweredExpr::Local(..)
        | LoweredExpr::ArrowFn { .. } => true,
        LoweredExpr::ObjectNew { props, .. } => props
            .iter()
            .all(|(_, value)| static_generator_implicit_completion_expr_is_local_only(value)),
        LoweredExpr::Block { stmts, result, .. } => {
            stmts
                .iter()
                .all(static_generator_implicit_completion_stmt_is_local_only)
                && static_generator_implicit_completion_expr_is_local_only(result)
        }
        LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::ObjectDefineProperty,
            args,
            ..
        } => args
            .iter()
            .all(static_generator_implicit_completion_expr_is_local_only),
        _ => false,
    }
}

fn static_usize_number_expr(expr: &ResolvedExpr) -> Option<usize> {
    let value = static_i64_number_expr(expr)?;
    usize::try_from(value).ok().filter(|value| *value <= 100)
}

fn format_number_to_fixed_i64(value: i64, digits: usize) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let mut result = format!("{sign}{}", value.unsigned_abs());
    if digits > 0 {
        result.push('.');
        result.push_str(&"0".repeat(digits));
    }
    result
}

fn format_number_to_exponential_i64(value: i64, fraction_digits: Option<usize>) -> Option<String> {
    if value == 0 {
        return Some(match fraction_digits {
            Some(digits) => format!("0.{}e+0", "0".repeat(digits)),
            None => "0e+0".to_owned(),
        });
    }

    let sign = if value < 0 { "-" } else { "" };
    let digits = value.unsigned_abs().to_string();
    let exponent = digits.len() - 1;
    let requested =
        fraction_digits.unwrap_or_else(|| digits.trim_end_matches('0').len().saturating_sub(1));
    if requested < digits.len().saturating_sub(1) {
        return None;
    }

    let first = &digits[..1];
    let rest = &digits[1..];
    let mut fraction = rest.to_owned();
    if requested > fraction.len() {
        fraction.push_str(&"0".repeat(requested - fraction.len()));
    }
    let mantissa = if requested == 0 {
        format!("{sign}{first}")
    } else {
        format!("{sign}{first}.{fraction}")
    };
    Some(format!("{mantissa}e+{exponent}"))
}

fn format_number_to_precision_i64(value: i64, precision: Option<usize>) -> Option<String> {
    let Some(precision) = precision else {
        return Some(value.to_string());
    };
    if precision == 0 {
        return None;
    }

    let sign = if value < 0 { "-" } else { "" };
    let digits = value.unsigned_abs().to_string();
    if precision < digits.len() {
        return format_number_to_exponential_i64(value, Some(precision.saturating_sub(1)));
    }
    let mut result = format!("{sign}{digits}");
    if precision > digits.len() {
        result.push('.');
        result.push_str(&"0".repeat(precision - digits.len()));
    }
    Some(result)
}
