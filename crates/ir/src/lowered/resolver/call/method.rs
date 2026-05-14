use super::super::{
    bigint_runtime_fn_intrinsic, is_array_from_call_receiver, is_array_prototype_map_call_receiver,
    is_array_prototype_push_expr, is_identity_arrow_callback, is_set_prototype_property_expr,
    is_static_date_constructor_expr, is_string_split_result_expr, is_typed_array_class,
    numeric_ascending_sort_arrow_callback, private_storage_observable_access_diagnostic,
    string_constructor_arrow_callback, string_split_arrow_separator, unary_plus_arrow_callback,
    unsupported_array_map_diagnostic, unsupported_array_sort_diagnostic,
};
use super::builtin::{is_html_wrapper_string_method, lower_html_wrapper_string_method};
use super::receiver::extract_prototype_method_name;
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr};
use crate::lowered::ctx::LoweringCtx;
use crate::lowered::facts::{IntlDateTimeFormatOptions, IntlNumberFormatOptions};
use crate::lowered::*;
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
        if let Some(result) = self.lower_mcall_early(object, method, args, span)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_arraybuffer(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_intl_number_format(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_intl_date_time_format(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_json_date_regexp(object, method, args, span)? {
            return Ok(result);
        }
        // Function.prototype.toString for user-defined functions
        if method == "toString"
            && args.is_empty()
            && let ResolvedExpr::Ident(name) = object
            && self.resolve_func(name.as_str()).is_ok()
        {
            return Ok(LoweredExpr::String(
                format!("function {}() {{ [native code] }}", name),
                span,
            ));
        }
        if let Some(result) = self.lower_mcall_date_string(object, method, args, span)? {
            return Ok(result);
        }
        if method == "next"
            && args.is_empty()
            && crate::lowered::resolver::expr::facts::resolved_expr_is_generator_iterator(
                &self.ctx, object,
            )
        {
            if let Some(result) = self.lower_static_generator_next(object)? {
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
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        let (func_name, state_local, prelude) = match object {
            ResolvedExpr::Ident(name) => {
                let local_id = self.resolve_local(name)?;
                let Some(binding) = self.ctx.facts.generator_iterator_bindings.get(&local_id)
                else {
                    return Ok(None);
                };
                (binding.func_name.clone(), binding.state_local, Vec::new())
            }
            _ => {
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
                )
            }
        };
        Ok(Some(self.lower_generator_resume_with_state(
            &func_name,
            state_local,
            prelude,
        )?))
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
        for stmt in &completion {
            completion_body.push(self.lower_stmt(stmt)?);
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
                let same_class_static_receiver = match object {
                    ResolvedExpr::This { .. } => self.resolve_local("this").is_err(),
                    ResolvedExpr::Ident(name) => {
                        self.ctx.classes.current_class.as_deref() == Some(name.as_str())
                    }
                    _ => false,
                };
                if same_class_static_receiver {
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
                lowered_args.push(self.lower_expr(arg)?);
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
        if let Some(regexp_args) = regexp_test_runtime(object, method, args, span)? {
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
        if let Some(regexp_args) = regexp_exec_runtime(object, method, args, span)? {
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
        if let Some(regexp_args) = regexp_string_match_runtime(object, method, args, span)? {
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
                ResolvedExpr::Ident(name) if self.is_function_identifier(object) => {
                    return Ok(Some(LoweredExpr::String(
                        format!("function {}() {{ [native code] }}", name),
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
        if is_annex_b_date_method(method) && self.is_date_receiver(object) {
            return Err(unsupported_annex_b_date_method_diagnostic(
                method,
                Some(span),
            ));
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
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-460: String.prototype.normalize currently requires a static normalization form".to_owned(),
                    span: Some(span),

                    phase: None,
                });
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
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-460: String.prototype.normalize currently requires a static receiver"
                        .to_owned(),
                span: Some(span),

                phase: None,
            });
        };
        if !value.is_ascii() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-460: String.prototype.normalize currently supports ASCII strings only"
                        .to_owned(),
                span: Some(span),

                phase: None,
            });
        }
        self.lower_expr(object)
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
        if let Some(intrinsic) = resolve_method_to_runtime_fn(object, method) {
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
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
        }
        Ok(None)
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

        if let ResolvedExpr::Ident(receiver_name) = object
            && let Ok(obj_local) = self.resolve_local(receiver_name)
            && let Some(method_id) = self
                .ctx
                .classes
                .object_function_props
                .get(&obj_local)
                .and_then(|props| props.get(method))
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
            && matches!(&args[0], ResolvedExpr::ArrowFn { .. })
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
                if !is_identity_array_method(method) {
                    let max_args = if method == "indexOf" || method == "includes" {
                        1
                    } else {
                        args.len()
                    };
                    for arg in args.iter().take(max_args) {
                        lowered_args.push(self.lower_expr(arg)?);
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
                    && matches!(call_args[0], ResolvedExpr::ArrowFn { .. })
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

        if let Some(intrinsic) = collection_method_runtime_fn_arg(method) {
            let receiver_expr = self.lower_expr(object)?;
            let mut lowered_args = vec![receiver_expr];
            if !is_identity_array_method(method) {
                let max_args = if method == "indexOf" || method == "includes" {
                    2
                } else {
                    args.len()
                };
                for arg in args.iter().take(max_args) {
                    lowered_args.push(self.lower_expr(arg)?);
                }
            }
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,

                span: Span::generated("runtime_call"),
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

        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-211: method `{}` requires an identifier receiver",
                method
            ),
            span: Some(span),

            phase: None,
        })
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
        // Map.forEach or Set.forEach with ArrowFn — expand at IR level
        if method == "forEach"
            && !args.is_empty()
            && matches!(&args[0], ResolvedExpr::ArrowFn { .. })
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
                .is_some_and(is_intl_date_time_format_class)
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
            let lowered_args = args
                .iter()
                .map(|e| self.lower_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            let mut lowered_args = lowered_args;
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
        ];
        let number_methods = ["toFixed", "toExponential", "toPrecision"];
        let promise_methods = ["then", "catch", "finally"];
        let class_name_str = match self.ctx.classes.local_classes.get(&obj_local) {
            Some(c) => c.clone(),
            None if array_like_methods.contains(&method) => "Array".to_owned(),
            None if number_methods.contains(&method) => "Number".to_owned(),
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

        let mut lowered_args = vec![LoweredExpr::Local(obj_local, Span::generated("local"))];
        lowered_args.extend(
            args.iter()
                .map(|e| self.lower_expr(e))
                .collect::<Result<Vec<_>, _>>()?,
        );
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
                | "getFloat64"
                    if args.len() == 1 =>
                {
                    lowered_args.push(LoweredExpr::Bool(false, Span::generated("bool")));
                }
                "setInt16" | "setUint16" | "setInt32" | "setUint32" | "setFloat32"
                | "setFloat64"
                    if args.len() == 2 =>
                {
                    lowered_args.push(LoweredExpr::Bool(false, Span::generated("bool")));
                }
                _ => {}
            }
        } else if class_name == "Number" && is_number_format_method(method) && args.is_empty() {
            lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
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

    pub(crate) fn intl_number_format_options_for_expr(
        &self,
        expr: &ResolvedExpr,
    ) -> Option<IntlNumberFormatOptions> {
        match expr {
            ResolvedExpr::New {
                class_name, args, ..
            } if matches!(class_name.as_str(), "Intl.NumberFormat" | "NumberFormat") => {
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

    fn lower_intl_number_format_method(
        &mut self,
        method: &str,
        args: &[ResolvedExpr],
        options: Option<&IntlNumberFormatOptions>,
    ) -> Result<LoweredExpr, Diagnostic> {
        let defaults = default_intl_number_format_options();
        let options = options.unwrap_or(&defaults);
        match method {
            "format" => Ok(string_lit(
                args.first()
                    .and_then(|arg| static_number_format_arg(arg, options))
                    .unwrap_or_else(|| "NaN".to_owned()),
            )),
            "formatToParts" => Ok(LoweredExpr::ArrayNew {
                elements: vec![LoweredExpr::ObjectNew {
                    props: vec![
                        (
                            "type".to_owned(),
                            string_lit(number_format_part_type(options)),
                        ),
                        (
                            "value".to_owned(),
                            string_lit(
                                args.first()
                                    .and_then(|arg| static_number_format_arg(arg, options))
                                    .unwrap_or_else(|| "NaN".to_owned()),
                            ),
                        ),
                    ],
                    non_enumerable: 0,
                    span: Span::generated("object_new"),
                }],
                span: Span::generated("array_new"),
            }),
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
            "format" => Ok(string_lit(format_intl_datetime_arg(args.first(), options))),
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
    matches!(class_name, "Intl.NumberFormat" | "NumberFormat")
}

fn is_intl_date_time_format_method(method: &str) -> bool {
    matches!(method, "format" | "formatToParts" | "resolvedOptions")
}

fn is_intl_date_time_format_class(class_name: &String) -> bool {
    matches!(
        class_name.as_str(),
        "Intl.DateTimeFormat" | "DateTimeFormat"
    )
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
