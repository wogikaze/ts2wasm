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
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

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
        if let Some(result) = self.lower_mcall_json_date_regexp(object, method, args, span)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_mcall_date_string(object, method, args, span)? {
            return Ok(result);
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
        let ResolvedExpr::Ident(receiver_name) = object else {
            unreachable!()
        };
        self.lower_mcall_class_dispatch(receiver_name, object, method, args, span)
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

                    if let Some((key, value)) =
                        props.iter().rev().find(|(key, _)| key == &allowed_key)
                    {
                        lowered_props.push((key.clone(), self.lower_expr(value)?));
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
        if is_date_now_live_time_call(object, method) {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateNow,
                args: vec![],

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
            if let ResolvedExpr::New {
                args: date_args, ..
            } = object
                && let Some(ResolvedExpr::Number(year)) = date_args.first()
            {
                return Ok(Some(LoweredExpr::Number(
                    year - 1900,
                    Span::generated("num"),
                )));
            }
            return Err(unsupported_annex_b_date_method_diagnostic(
                method,
                Some(span),
            ));
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

    /// Helper for lower_method_call_expr: array method dispatch (indexOf, includes,
    /// concat, identity-arrow optimizations) and runtime_fn routing (push, Math, etc.).
    fn lower_mcall_array_runtime(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
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
        if let Some(intrinsic) = resolve_method_to_runtime_fn(object, method) {
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
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            }));
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
                // Non-callback runtime functions
                if let Some(intrinsic) = collection_method_runtime_fn(class_name, proto_method) {
                    let mut lowered_args = vec![self.lower_expr(receiver)?];
                    for arg in call_args {
                        lowered_args.push(self.lower_expr(arg)?);
                    }
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

        // Local class runtime_fn
        if let Ok(obj_local) = self.resolve_local(receiver_name)
            && let Some(class_name) = self.ctx.classes.local_classes.get(&obj_local)
            && let Some(intrinsic) = collection_method_runtime_fn(class_name, method)
        {
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
            } else if is_array_like_class && (method == "toString" || method == "toLocaleString") {
                // toString/toLocaleString calls join(",") internally
                lowered_args.push(LoweredExpr::String(",".to_owned(), Span::generated("str")));
            } else {
                lowered_args.extend(
                    args.iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?,
                );
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
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-104: Promise.prototype.{}(...) — Promise runtime is not implemented yet. Register Promise as a tracked class to route through the Promise runtime path.",
                        method
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            None => {
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
}
