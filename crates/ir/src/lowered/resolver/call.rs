use super::{
    is_array_from_call_receiver, is_array_prototype_map_call_receiver,
    is_array_prototype_push_expr, is_identity_arrow_callback, is_invalid_date_constructor_expr,
    is_set_prototype_property_expr, is_static_date_constructor_expr, is_string_split_result_expr,
    numeric_ascending_sort_arrow_callback, private_storage_observable_access_diagnostic,
    string_constructor_arrow_callback, string_split_arrow_separator, unary_plus_arrow_callback,
    unsupported_array_map_diagnostic, unsupported_array_sort_diagnostic,
};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr};
use crate::lowered::*;
use ts2wasm_shared::{DiagCode, Diagnostic, Span};

impl<'a> super::Resolver<'a> {
    pub(super) fn lower_call_expr(
        &mut self,
        callee: &ResolvedExpr,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let ResolvedExpr::FunctionExpr { name, params, body } = callee {
            return self.lower_function_expr_call(name, params, body, args, span);
        }

        if let ResolvedExpr::ArrowFn {
            params,
            body,
            body_stmts,
        } = callee
        {
            return self.lower_arrow_fn_iife(params, body, body_stmts, args, span);
        }

        let func_name = match callee {
            ResolvedExpr::Ident(name) => name,
            expr @ (ResolvedExpr::Call { .. } | ResolvedExpr::New { .. }) => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "nested call expression is not supported; {} has no call signatures",
                        match expr {
                            ResolvedExpr::Call { .. } => "the return value of the outer call",
                            _ => "the constructed instance",
                        }
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "only identifier calls are supported in expression context".to_owned(),
                    span: Some(span),

                    phase: None,
                });
            }
        };

        if let Some(intrinsic) = super::bigint_runtime_fn_intrinsic(func_name) {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic,
                args: self.lower_call_args(args)?,

                span: Span::generated("runtime_call"),
            });
        }

        if let Ok(local_id) = self.resolve_local(func_name)
            && let Some(closure) = self.facts.arrow_locals.get(&local_id).cloned()
        {
            let mut lowered_args = self.lower_call_args(args)?;
            lowered_args.extend(
                closure
                    .captures
                    .iter()
                    .copied()
                    .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
            );
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(closure.func_id),
                args: lowered_args,

                span: Span::generated("call"),
            });
        }

        if let Ok(local_id) = self.resolve_local(func_name)
            && self.captures.heap_closure_locals.contains(&local_id)
        {
            let receiver = if self.captures.env_cell_locals.contains(&local_id) {
                LoweredExpr::EnvCellGet(local_id, Span::generated("env_cell_get"))
            } else {
                LoweredExpr::Local(local_id, Span::generated("local"))
            };
            let mut lowered_args = vec![receiver];
            lowered_args.extend(self.lower_call_args(args)?);
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::HeapClosureCall,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            });
        }

        if func_name == "super" {
            if !self.classes.in_constructor {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "super(...) is only supported in constructors".to_owned(),
                    span: None,

                    phase: None,
                });
            }
            let class_name = self
                .classes
                .current_class
                .as_ref()
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "super(...) requires class context".to_owned(),
                    span: None,

                    phase: None,
                })?;
            let parent_name = self
                .classes
                .class_parents
                .get(class_name)
                .and_then(|p| p.clone())
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "super(...) used in class without extends".to_owned(),
                    span: None,

                    phase: None,
                })?;
            let parent_ctor = self
                .classes
                .class_constructor_ids
                .get(&parent_name)
                .copied()
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("super class constructor for `{}` not found", parent_name),
                    span: None,

                    phase: None,
                })?;

            let mut lowered_args = vec![LoweredExpr::Local(
                self.resolve_local("this")?,
                Span::generated("local"),
            )];
            lowered_args.extend(
                args.iter()
                    .map(|arg| self.lower_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?,
            );

            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(parent_ctor),
                args: lowered_args,

                span: Span::generated("call"),
            });
        }

        if func_name == "String"
            && let [arg] = args
            && self.resolved_expr_is_bigint(arg)
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::BigIntToString,
                args: vec![self.lower_expr(arg)?],

                span: Span::generated("runtime_call"),
            });
        }

        if func_name == "Boolean"
            && let [ResolvedExpr::BigIntLiteral { .. }] = args
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::BigIntToBoolean,
                args: vec![self.lower_expr(&args[0])?],

                span: Span::generated("runtime_call"),
            });
        }

        // Symbol("desc") constructor: produces "Symbol(desc)" string.
        if func_name == "Symbol" {
            let arg = match args.first() {
                Some(first) => self.lower_expr(first)?,
                None => LoweredExpr::Undefined(Span::generated("undef")),
            };
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::SymbolNew,
                args: vec![arg],
                span: Span::generated("runtime_call"),
            });
        }

        // Global setTimeout(): DOM timer host APIs are outside
        // the WASM subset. Return Undefined to advance past the
        // UnresolvedFunction blocker.
        if func_name == "setTimeout" {
            return Ok(LoweredExpr::Undefined(Span::generated("undef")));
        }

        // diagnostic, check if the callee is a local whose value is
        // null/undefined — this covers TypeScript callable interface
        // typed locals such as `var i: I<string>; i("")`. These are
        // not extracted methods but simply uninitialized variables,
        // and deserve a more precise diagnostic.
        if let Ok(local_id) = self.resolve_local(func_name)
            && self.facts.nullish_locals.contains(&local_id)
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-5195: callable interface-typed local `{func_name}` is not callable — the variable is never assigned"
                ),
                span: Some(span),

                phase: None,
            });
        }

        let func_id = match self.resolve_func(func_name) {
            Ok(func_id) => func_id,
            Err(_) if self.resolve_local(func_name).is_ok() => {
                // Check if this local is a function parameter (e.g., typed
                // through a conditional type alias) for a more specific diagnostic.
                if let Ok(local_id) = self.resolve_local(func_name)
                    && self.locals.param_locals.contains(&local_id)
                {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-5196: callable parameter `{func_name}(...)` typed through a conditional type is not supported in this milestone"
                        ),
                        span: Some(span),

                        phase: None,
                    });
                }
                // Function-valued local: emit a HeapClosureCall to dispatch
                // at runtime based on the value's tag. The HeapClosureCall
                // runtime handles both DirectLocalToken and HeapObject
                // closure representations.
                let closure_local = self.resolve_local(func_name).unwrap();
                let mut lowered_args =
                    vec![LoweredExpr::Local(closure_local, Span::generated("local"))];
                lowered_args.extend(self.lower_call_args(args)?);
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::HeapClosureCall,
                    args: lowered_args,

                    span: Span::generated("runtime_call"),
                });
            }
            Err(_)
                if self
                    .classes
                    .class_constructor_ids
                    .contains_key(func_name.as_str()) =>
            {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-5197: class `{func_name}` cannot be called without `new` — constructors are not callable"
                    ),
                    span: Some(span),

                    phase: None,
                });
            }
            Err(_) => {
                return Err(Diagnostic {
                    code: DiagCode::UnresolvedFunction,
                    message: format!("unresolved function: `{func_name}`"),
                    span: Some(span),

                    phase: None,
                });
            }
        };
        if self
            .symbols
            .function_signatures
            .get(&func_id)
            .is_some_and(|signature| signature.needs_receiver)
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062d: direct call `{func_name}(...)` cannot bind a supported receiver for `this`; call through a supported receiver object"
                ),
                span: Some(span),

                phase: None,
            });
        }
        let lowered_args = self.lower_function_call_args(
            func_id,
            LoweredExpr::Undefined(Span::generated("undef")),
            args,
        )?;

        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: lowered_args,

            span: Span::generated("call"),
        })
    }

    pub(super) fn lower_method_call_expr(
        &mut self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
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
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::ArrayPushMany,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            });
        }
        if is_array_from_call_receiver(object, method) {
            return self.lower_array_from_call(args, span);
        }
        if is_array_prototype_map_call_receiver(object, method) {
            return self.lower_array_prototype_map_call(args, span);
        }
        if method == "call" && is_set_prototype_property_expr(object, "originalAdd") {
            return self.lower_native_set_add_call(args, span);
        }
        if method == "call"
            && let ResolvedExpr::Ident(name) = object
            && let Ok(local_id) = self.resolve_local(name)
            && self.facts.native_set_add_locals.contains(&local_id)
        {
            return self.lower_native_set_add_call(args, span);
        }
        if matches!(
            object,
            ResolvedExpr::Ident(name) if name == "__ts2wasm_bigint_runtime"
        ) && let Some(intrinsic) = super::bigint_runtime_fn_intrinsic(method)
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic,
                args: self.lower_call_args(args)?,

                span: Span::generated("runtime_call"),
            });
        }
        if method.starts_with('#') {
            if let Some(method_id) = self.current_static_private_method_id(method) {
                let same_class_static_receiver = match object {
                    ResolvedExpr::This { .. } => self.resolve_local("this").is_err(),
                    ResolvedExpr::Ident(name) => {
                        self.classes.current_class.as_deref() == Some(name.as_str())
                    }
                    _ => false,
                };
                if same_class_static_receiver {
                    let lowered_args = args
                        .iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(method_id),
                        args: lowered_args,

                        span: Span::generated("call"),
                    });
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
                let class_name = self.classes.current_class.clone().ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-255: private method `{method}` call requires declaring class context"
                    ),
                    span: Some(span),

                    phase: None,
                })?;
                let brand = self.private_brand_for_class(&class_name, Some(span))?;
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::PrivateBrandCheck,
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
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(method_id),
                args: lowered_args,

                span: Span::generated("call"),
            });
        }
        if is_json_static_call(object, method) {
            validate_json_stringify_args(
                args,
                span,
                self.symbols.function_ids,
                self.symbols.function_signatures,
            )?;
            let mut lowered_args = Vec::with_capacity(3);
            let value = if let (ResolvedExpr::Object(props), Some(replacer_keys)) = (
                &args[0],
                json_stringify_replacer_keys(args, self.symbols.function_ids),
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
                    if let Some(func_id) =
                        json_stringify_function_replacer_id(replacer, self.symbols.function_ids)
                    {
                        LoweredExpr::Number(func_id.0 as i32, Span::generated("num"))
                    } else {
                        self.lower_expr(replacer)?
                    }
                }
                None => LoweredExpr::Undefined(Span::generated("undef")),
            });
            lowered_args.push(match args.get(2) {
                Some(space)
                    if should_ignore_json_stringify_space(space, self.symbols.function_ids) =>
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
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::JsonStringify,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            })
        } else if is_date_now_live_time_call(object, method) {
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::DateNow,
                args: vec![],

                span: Span::generated("runtime_call"),
            })
        } else if self.is_unsupported_regexp_compile_receiver(object, method) {
            Err(unsupported_regexp_compile_diagnostic(Some(span)))
        } else if self.is_object_key_enumeration_leak(object, method, args) {
            Err(private_storage_observable_access_diagnostic(Some(span)))
        } else if method == "matchAll" {
            self.lower_string_match_all_literal(object, args, span)
        } else if let Some(regexp_args) = regexp_test_runtime(object, method, args, span)? {
            let lowered_args = regexp_args
                .iter()
                .map(|e| self.lower_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::RegExpTest,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            })
        } else if let Some(regexp_args) = regexp_exec_runtime(object, method, args, span)? {
            let lowered_args = regexp_args
                .iter()
                .map(|e| self.lower_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::RegExpMatch,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            })
        } else if let Some(regexp_args) = regexp_string_match_runtime(object, method, args, span)? {
            let lowered_args = regexp_args
                .iter()
                .map(|e| self.lower_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: if method == "search" {
                    RuntimeIntrinsic::RegExpSearch
                } else {
                    RuntimeIntrinsic::RegExpMatch
                },
                args: lowered_args,

                span: Span::generated("runtime_call"),
            })
        } else if matches!(method, "getTime" | "valueOf") && self.is_date_receiver(object) {
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
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::DateGetTime,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            })
        } else if method == "getTimezoneOffset" && self.is_date_receiver(object) {
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
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::DateGetTimezoneOffset,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            })
        } else if is_local_tz_date_method(method) && self.is_date_receiver(object) {
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
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::DateGetLocalTimeField,
                args: vec![
                    self.lower_expr(object)?,
                    LoweredExpr::Number(field_index, Span::generated("num")),
                ],

                span: Span::generated("runtime_call"),
            })
        } else if method == "getYear" && is_static_date_constructor_expr(object) {
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
                return Ok(LoweredExpr::Number(year - 1900, Span::generated("num")));
            }
            Err(unsupported_annex_b_date_method_diagnostic(
                method,
                Some(span),
            ))
        } else if method == "getYear" && self.is_invalid_date_expr(object) {
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
            Ok(LoweredExpr::Number(0, Span::generated("num")))
        } else if method == "getYear" && self.is_date_receiver(object) {
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
            Ok(LoweredExpr::Binary {
                left: Box::new(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::DateGetLocalTimeField,
                    args: vec![
                        self.lower_expr(object)?,
                        LoweredExpr::Number(0, Span::generated("num")),
                    ],

                    span: Span::generated("runtime_call"),
                }),
                op: LoweredBinaryOp::Subtract,
                right: Box::new(LoweredExpr::Number(1900, Span::generated("num"))),
                span: Span::generated("binary"),
            })
        } else if is_annex_b_date_method(method) && self.is_date_receiver(object) {
            Err(unsupported_annex_b_date_method_diagnostic(
                method,
                Some(span),
            ))
        } else if method == "toString" && self.is_date_receiver(object) {
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
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::DateToString,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            })
        } else if self.is_date_receiver(object)
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
            let intrinsic: RuntimeIntrinsic = match method {
                "getUTCMilliseconds" => RuntimeIntrinsic::DateGetUtcMilliseconds,
                "getUTCSeconds" => RuntimeIntrinsic::DateGetUtcSeconds,
                "getUTCMinutes" => RuntimeIntrinsic::DateGetUtcMinutes,
                "getUTCHours" => RuntimeIntrinsic::DateGetUtcHours,
                "getUTCDay" => RuntimeIntrinsic::DateGetUtcDay,
                "getUTCDate" => RuntimeIntrinsic::DateGetUtcDate,
                "getUTCMonth" => RuntimeIntrinsic::DateGetUtcMonth,
                "getUTCFullYear" => RuntimeIntrinsic::DateGetUtcFullYear,
                _ => unreachable!(),
            };
            Ok(LoweredExpr::RuntimeCall {
                intrinsic,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            })
        } else if self.is_date_receiver(object) && matches!(method, "toISOString" | "toJSON") {
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
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::DateToISOString,
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            })
        } else if matches!(object, ResolvedExpr::String(_)) {
            if is_html_wrapper_string_method(method) {
                let lowered_object = self.lower_expr(object)?;
                let mut lowered_args = Vec::new();
                for arg in args {
                    lowered_args.push(self.lower_expr(arg)?);
                }
                lower_html_wrapper_string_method(method, lowered_object, lowered_args, span)
            } else if let Some(diagnostic) = unsupported_annex_b_string_method(method, span) {
                Err(diagnostic)
            } else if let Some(intrinsic) = resolve_method_to_runtime_fn(object, method) {
                let mut lowered_args = vec![self.lower_expr(object)?];
                lowered_args.extend(
                    args.iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?,
                );
                Ok(LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: lowered_args,

                    span: Span::generated("runtime_call"),
                })
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "String.prototype.{method} is not supported in this milestone"
                    ),
                    span: Some(span),

                    phase: None,
                })
            }
        } else if (method == "indexOf" || method == "includes")
            && self.is_known_array_expr(object)
            && !args.is_empty()
        {
            let mut lowered_args = vec![self.lower_expr(object)?, self.lower_expr(&args[0])?];
            // Pass fromIndex if provided, otherwise default to 0
            if args.len() > 1 {
                lowered_args.push(self.lower_expr(&args[1])?);
            } else {
                lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
            }
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: if method == "indexOf" {
                    RuntimeIntrinsic::ArrayIndexOf
                } else {
                    RuntimeIntrinsic::ArrayIncludes
                },
                args: lowered_args,

                span: Span::generated("runtime_call"),
            })
        } else if method == "concat" && self.is_known_array_expr(object) {
            let mut lowered_args = vec![self.lower_expr(object)?];
            lowered_args.extend(
                args.iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::ArrayConcat,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            })
        } else if (method == "find"
            || method == "findIndex"
            || method == "findLast"
            || method == "findLastIndex"
            || method == "filter"
            || method == "every"
            || method == "some")
            && is_identity_arrow_callback(args)
            && self.is_known_array_expr(object)
        {
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: match method {
                    "find" => RuntimeIntrinsic::ArrayFind,
                    "findIndex" => RuntimeIntrinsic::ArrayFindIndex,
                    "findLast" => RuntimeIntrinsic::ArrayFindLast,
                    "findLastIndex" => RuntimeIntrinsic::ArrayFindLastIndex,
                    "filter" => RuntimeIntrinsic::ArrayFilter,
                    "every" => RuntimeIntrinsic::ArrayEvery,
                    "some" => RuntimeIntrinsic::ArraySome,
                    _ => unreachable!(),
                },
                args: vec![self.lower_expr(object)?],

                span: Span::generated("runtime_call"),
            })
        } else if let Some(intrinsic) = resolve_method_to_runtime_fn(object, method) {
            if (intrinsic == RuntimeIntrinsic::ArrayPush
                || intrinsic == RuntimeIntrinsic::ArrayPushGrow)
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
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::ArrayPushMany,
                    args: lowered_args,

                    span: Span::generated("runtime_call"),
                });
            }
            if (intrinsic == RuntimeIntrinsic::MathMax || intrinsic == RuntimeIntrinsic::MathMin)
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
                return Ok(result);
            }
            // Handle zero-argument case for Math.max/min
            // Math.max() with no arguments returns -Infinity (approximated as NUMBER_PAYLOAD_MIN)
            // Math.min() with no arguments returns +Infinity (approximated as NUMBER_PAYLOAD_MAX)
            // Note: Proper Infinity support requires broader number-model support (issue-281)
            if (intrinsic == RuntimeIntrinsic::MathMax || intrinsic == RuntimeIntrinsic::MathMin)
                && args.is_empty()
            {
                use ts2wasm_runtime_abi::ValueTag;
                let infinity_value = if intrinsic == RuntimeIntrinsic::MathMax {
                    // -Infinity approximated as minimum representable number
                    ValueTag::NUMBER_PAYLOAD_MIN
                } else {
                    // +Infinity approximated as maximum representable number
                    ValueTag::NUMBER_PAYLOAD_MAX
                };
                return Ok(LoweredExpr::Number(infinity_value, Span::generated("num")));
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
            Ok(LoweredExpr::RuntimeCall {
                intrinsic,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            })
        } else {
            if let ResolvedExpr::Ident(receiver_name) = object
                && let Ok(obj_local) = self.resolve_local(receiver_name)
                && let Some(method_id) = self
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
                return Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(method_id),
                    args: lowered_args,

                    span: Span::generated("call"),
                });
            }

            // Sparse arrays with known holes must route through the hole-aware
            // lower_array_map_elements before optimized runtime paths or literal
            // expansion, because those paths assume dense arrays.
            if method == "map"
                && let Some(elements) = self.resolved_expr_static_array_slots(object)
                && elements
                    .iter()
                    .any(|element| matches!(element, ResolvedArrayElement::Hole))
            {
                return self.lower_array_map_elements(object, &elements, args, span);
            }

            if method == "map"
                && string_constructor_arrow_callback(args)
                && self.is_known_array_expr(object)
            {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::ArrayMapValueToString,
                    args: vec![self.lower_expr(object)?],

                    span: Span::generated("runtime_call"),
                });
            }

            if method == "map"
                && unary_plus_arrow_callback(args)
                && self.is_known_array_expr(object)
            {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::ArrayMapUnaryPlus,
                    args: vec![self.lower_expr(object)?],

                    span: Span::generated("runtime_call"),
                });
            }

            if method == "map" && matches!(object, ResolvedExpr::Array(_)) {
                return self.lower_array_literal_map(object, args, span);
            }

            if method == "map"
                && is_string_split_result_expr(object)
                && is_identity_arrow_callback(args)
            {
                return self.lower_expr(object);
            }

            if method == "map"
                && is_string_split_result_expr(object)
                && let Some(separator) = string_split_arrow_separator(args)
            {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::ArrayMapStringSplit,
                    args: vec![self.lower_expr(object)?, self.lower_expr(separator)?],

                    span: Span::generated("runtime_call"),
                });
            }

            if method == "sort" && self.is_known_array_expr(object) {
                if numeric_ascending_sort_arrow_callback(args) {
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::ArraySortNumeric,
                        args: vec![self.lower_expr(object)?],

                        span: Span::generated("runtime_call"),
                    });
                }
                return Err(unsupported_array_sort_diagnostic(Some(span)));
            }

            if is_array_prototype_map_call_receiver(object, method) {
                return Err(unsupported_array_map_diagnostic(Some(span)));
            }

            // User-callback array methods (forEach, filter, find, some, every, reduce, map)
            // are expanded at IR level with While loops.
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
                && self.is_known_array_expr(object)
                && !args.is_empty()
                && matches!(&args[0], ResolvedExpr::ArrowFn { .. })
            {
                let lowered_receiver = self.lower_expr(object)?;
                return self.lower_array_callback_method(
                    method,
                    lowered_receiver,
                    object,
                    args,
                    span,
                );
            }

            if matches!(object, ResolvedExpr::This { .. }) {
                let class_name = self
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
                return Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(method_id),
                    args: lowered_args,

                    span: Span::generated("call"),
                });
            }

            let receiver_name = match object {
                ResolvedExpr::Ident(name) => name,
                ResolvedExpr::PropertyAccess {
                    object: prop_obj,
                    key,
                    ..
                } if matches!(prop_obj.as_ref(), ResolvedExpr::This { .. }) => {
                    // this.field.method(...) — try to use a runtime function
                    if let Some(intrinsic) = collection_method_runtime_fn_arg(method) {
                        let receiver_expr = self.lower_expr(object)?;
                        let mut lowered_args = vec![receiver_expr];
                        // Identity methods (every/some/find/filter) don't accept
                        // user callbacks — just pass the receiver for build_smoke
                        if !is_identity_array_method(method) {
                            // indexOf/includes only accept searchElement, not fromIndex
                            let max_args = if method == "indexOf" || method == "includes" {
                                1
                            } else {
                                args.len()
                            };
                            for arg in args.iter().take(max_args) {
                                lowered_args.push(self.lower_expr(arg)?);
                            }
                        }
                        return Ok(LoweredExpr::RuntimeCall {
                            intrinsic,
                            args: lowered_args,

                            span: Span::generated("runtime_call"),
                        });
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
                _ => {
                    // Non-identifier receiver (e.g. `[1].indexOf(2)`, `"hi".charAt(0)`)
                    // Handle ClassName.prototype.method.call(thisArg, ...args) pattern
                    if method == "call"
                        && let Some((class_name, proto_method)) =
                            extract_prototype_method_name(object)
                    {
                        if let Some((receiver, call_args)) = args.split_first() {
                            // For Array callback methods (every, some, find, filter, etc.)
                            // with an ArrowFn as first call arg, route through
                            // lower_array_callback_method
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
                                && self.is_known_array_expr(receiver)
                            {
                                let lowered_receiver = self.lower_expr(receiver)?;
                                return self.lower_array_callback_method(
                                    proto_method,
                                    lowered_receiver,
                                    receiver,
                                    call_args,
                                    span,
                                );
                            }
                            // For String HTML wrapper methods, route through IR-level Concat lowering
                            if class_name == "String" && is_html_wrapper_string_method(proto_method)
                            {
                                let lowered_receiver = self.lower_expr(receiver)?;
                                let lowered_call_args = call_args
                                    .iter()
                                    .map(|a| self.lower_expr(a))
                                    .collect::<Result<Vec<_>, _>>()?;
                                return lower_html_wrapper_string_method(
                                    proto_method,
                                    lowered_receiver,
                                    lowered_call_args,
                                    span,
                                );
                            }
                            // For non-callback runtime functions, unwrap call
                            if let Some(intrinsic) =
                                collection_method_runtime_fn(class_name, proto_method)
                            {
                                let mut lowered_args = vec![self.lower_expr(receiver)?];
                                for arg in call_args {
                                    lowered_args.push(self.lower_expr(arg)?);
                                }
                                return Ok(LoweredExpr::RuntimeCall {
                                    intrinsic,
                                    args: lowered_args,

                                    span: Span::generated("runtime_call"),
                                });
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
                    // Fall through to issue-211 error below
                    if let Some(intrinsic) = collection_method_runtime_fn_arg(method) {
                        let receiver_expr = self.lower_expr(object)?;
                        let mut lowered_args = vec![receiver_expr];
                        // Identity methods (every/some/find/filter) just pass receiver
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
                        return Ok(LoweredExpr::RuntimeCall {
                            intrinsic,
                            args: lowered_args,

                            span: Span::generated("runtime_call"),
                        });
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
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,

                            span: Span::generated("call"),
                        });
                    }
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-211: method `{}` requires an identifier receiver",
                            method
                        ),
                        span: Some(span),

                        phase: None,
                    });
                }
            };

            // Map.forEach or Set.forEach with ArrowFn — expand at IR level
            if method == "forEach"
                && !args.is_empty()
                && matches!(&args[0], ResolvedExpr::ArrowFn { .. })
                && let Ok(obj_local) = self.resolve_local(receiver_name)
                && let Some(class_name) = self.classes.local_classes.get(&obj_local)
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
                && let Some(class_name) = self.classes.local_classes.get(&obj_local)
                && let Some(intrinsic) = collection_method_runtime_fn(class_name, method)
            {
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
                let mut lowered_args =
                    vec![LoweredExpr::Local(obj_local, Span::generated("local"))];
                // Array.prototype.flat defaults depth to 1 when omitted
                if class_name == "Array" && method == "flat" && args.is_empty() {
                    lowered_args.push(LoweredExpr::Number(1, Span::generated("num")));
                } else if class_name == "Array" && method == "join" && args.is_empty() {
                    lowered_args.push(LoweredExpr::String(",".to_owned(), Span::generated("str")));
                } else if class_name == "Array" && method == "copyWithin" {
                    // copyWithin(target, start, end) — pad missing args with undefined
                    for arg in args.iter().take(3) {
                        lowered_args.push(self.lower_expr(arg)?);
                    }
                    while lowered_args.len() < 4 {
                        lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                    }
                } else if class_name == "Array"
                    && (method == "toString" || method == "toLocaleString")
                {
                    // toString/toLocaleString calls join(",") internally
                    lowered_args.push(LoweredExpr::String(",".to_owned(), Span::generated("str")));
                } else {
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);
                }
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: lowered_args,

                    span: Span::generated("runtime_call"),
                });
            }

            if receiver_name == "super" {
                let class_name = self
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

            if let Some(method_id) = self
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

            let obj_local = self.resolve_local(receiver_name)?;

            // For ambient interface-typed receivers without a concrete class
            // in local_classes, fall back to Array for known array-like methods.
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
            let class_name_str = match self.classes.local_classes.get(&obj_local) {
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

    pub(super) fn lower_new_expr(
        &mut self,
        class_name: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if class_name == "RegExp" {
            return Ok(LoweredExpr::String(
                regexp_constructor_literal(args)?,
                Span::generated("str"),
            ));
        }
        if class_name == "Proxy" {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-106: Proxy constructor — Proxy is not implemented yet; use plain objects instead"
                        .to_owned(),
                span: Some(span),

                phase: None,
            });
        }
        if class_name == "Reflect" {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-106: Reflect API is not implemented yet".to_owned(),
                span: Some(span),

                phase: None,
            });
        }
        if class_name == "Date" {
            if args.is_empty() {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::DateNewLive,
                    args: vec![],

                    span: Span::generated("runtime_call"),
                });
            }
            let is_invalid_date = class_name == "Date"
                && (matches!(args, [ResolvedExpr::Object(_)])
                    || matches!(args, [ResolvedExpr::Ident(name)] if name == "NaN"));
            if is_invalid_date {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::DateNew,
                    args: vec![LoweredExpr::Number(0, Span::generated("num"))],

                    span: Span::generated("runtime_call"),
                });
            }
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: if args.len() > 1 {
                        "issue-5243: multi-argument new Date(year, month, ...) is not supported in this slice".to_string()
                    } else {
                        "issue-5243: Date constructor requires an epoch-millisecond number, not a string or expression".to_string()
                    },
                    span: None,

                    phase: None,
                });
            }
            let epoch_ms = &args[0];
            if is_date_now_expr(epoch_ms) {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::DateNew,
                    args: vec![self.lower_expr(epoch_ms)?],

                    span: Span::generated("runtime_call"),
                });
            }
            if !is_date_constructor_epoch_arg(epoch_ms) {
                let msg = if matches!(epoch_ms, ResolvedExpr::String(_)) {
                    "issue-5243: string-based Date parsing like new Date(\"2024-01-01\") is not supported in this slice"
                } else {
                    "issue-5243: Date constructor requires an epoch-millisecond number argument"
                };
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: msg.to_owned(),
                    span: None,

                    phase: None,
                });
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::DateNew,
                args: vec![self.lower_expr(epoch_ms)?],

                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "Array" {
            if args.is_empty() {
                return Ok(LoweredExpr::ArrayNewSparse {
                    slots: Vec::new(),

                    span: Span::generated("array_new_sparse"),
                });
            }
            let [length] = args else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-405: new Array(length) currently supports exactly one small non-negative integer length".to_owned(),
                    span: None,

                    phase: None,
                });
            };
            let ResolvedExpr::Number(length) = length else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-405: new Array(length) currently requires a small non-negative integer length literal"
                            .to_owned(),
                    span: None,

                    phase: None,
                });
            };
            if *length < 0 || *length > 32 {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-405: new Array(length) currently supports lengths from 0 through 32"
                            .to_owned(),
                    span: None,

                    phase: None,
                });
            }
            return Ok(LoweredExpr::ArrayNewSparse {
                slots: vec![LoweredArraySlot::Hole; *length as usize],
                span: Span::generated("array_new_sparse"),
            });
        }
        if class_name == "Map"
            || class_name == "Set"
            || class_name == "WeakMap"
            || class_name == "WeakSet"
        {
            if args.is_empty() || class_name == "WeakMap" || class_name == "WeakSet" {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: match class_name {
                        "Map" => RuntimeIntrinsic::MapNew,
                        "Set" => RuntimeIntrinsic::SetNew,
                        "WeakMap" => RuntimeIntrinsic::WeakMapNew,
                        "WeakSet" => RuntimeIntrinsic::WeakSetNew,
                        _ => unreachable!(),
                    },
                    args: Vec::new(),

                    span: Span::generated("runtime_call"),
                });
            }
            if class_name == "Set" && args.len() == 1 && self.is_known_array_expr(&args[0]) {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::SetFromArray,
                    args: vec![self.lower_expr(&args[0])?],

                    span: Span::generated("runtime_call"),
                });
            }
            if class_name == "Set" {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-276: new Set(iterable) currently supports only known dense array inputs"
                            .to_owned(),
                    span: None,

                    phase: None,
                });
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("issue-049: new {class_name}(iterable) is not supported yet"),
                span: None,

                phase: None,
            });
        }
        if class_name == "Promise" {
            if args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-5422: new Promise() without executor is not supported"
                        .to_owned(),
                    span: None,

                    phase: None,
                });
            }
            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(self.lower_expr(arg)?);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::PromiseConstructor,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            });
        }
        if matches!(
            class_name,
            "Int8Array"
                | "Uint8Array"
                | "Uint8ClampedArray"
                | "Int16Array"
                | "Uint16Array"
                | "Int32Array"
                | "Uint32Array"
                | "Float32Array"
                | "Float64Array"
                | "BigInt64Array"
        ) {
            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(self.lower_expr(arg)?);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::TypedArrayFromArray,
                args: lowered_args,
                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "ArrayBuffer" {
            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(self.lower_expr(arg)?);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::ArrayBufferNew,
                args: lowered_args,
                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "DataView" {
            if args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-206: DataView constructor requires a buffer argument"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            }
            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(self.lower_expr(arg)?);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::DataViewNew,
                args: lowered_args,
                span: Span::generated("runtime_call"),
            });
        }
        if let Some(constructor) = BuiltinErrorConstructor::from_name(class_name) {
            let message = match args.first() {
                Some(message) => LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::ErrorMessage,
                    args: vec![self.lower_expr(message)?],

                    span: Span::generated("runtime_call"),
                },
                None => LoweredExpr::String(String::new(), Span::generated("str")),
            };
            return Ok(LoweredExpr::ErrorNew {
                constructor,
                message: Box::new(message),
                span: Span::generated("error_new"),
            });
        }

        let prototype = match self.class_prototype_ref(class_name) {
            Ok(proto) => proto,
            Err(_diag) => {
                return Ok(LoweredExpr::Null(Span::generated("null")));
            }
        };

        let lowered_args = args
            .iter()
            .map(|arg| self.lower_expr(arg))
            .collect::<Result<Vec<_>, _>>()?;
        let private_slot_count = self.private_slot_count(class_name);
        let private_brand = if self.class_has_instance_private_brand(class_name) {
            Some(self.private_brand_for_class(class_name, None)?)
        } else {
            None
        };

        Ok(LoweredExpr::New {
            constructor: prototype.constructor,
            prototype,
            args: lowered_args,
            base_local: self.alloc_temp(),
            private_brand,
            private_slot_count,
            span: Span::generated("new"),
        })
    }
}

/// Extract `(class_name, method_name)` from `ClassName.prototype.methodName` patterns.
/// Used for unwrapping `Array.prototype.every.call(obj, fn)` into `ArrayEvery`.
fn extract_prototype_method_name(expr: &ResolvedExpr) -> Option<(&str, &str)> {
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
    let ResolvedExpr::Ident(class_name) = class_expr.as_ref() else {
        return None;
    };
    Some((class_name, method_name))
}

/// Returns true if `method` is an HTML wrapper (Annex B String.prototype method
/// that can be lowered to Concat calls at the IR level).
fn is_html_wrapper_string_method(method: &str) -> bool {
    matches!(
        method,
        "anchor"
            | "big"
            | "blink"
            | "bold"
            | "fixed"
            | "fontcolor"
            | "fontsize"
            | "italics"
            | "link"
            | "small"
            | "strike"
            | "sub"
            | "sup"
    )
}

/// Lower an HTML wrapper String.prototype method to nested Concat runtime calls.
fn lower_html_wrapper_string_method(
    method: &str,
    object: LoweredExpr,
    args: Vec<LoweredExpr>,
    span: Span,
) -> Result<LoweredExpr, Diagnostic> {
    let (open_prefix, open_suffix, close_tag) = match method {
        "anchor" => ("<a name=\"", "\"", "</a>"),
        "big" => ("<big>", "", "</big>"),
        "blink" => ("<blink>", "", "</blink>"),
        "bold" => ("<b>", "", "</b>"),
        "fixed" => ("<tt>", "", "</tt>"),
        "fontcolor" => ("<font color=\"", "\"", "</font>"),
        "fontsize" => ("<font size=\"", "\"", "</font>"),
        "italics" => ("<i>", "", "</i>"),
        "link" => ("<a href=\"", "\"", "</a>"),
        "small" => ("<small>", "", "</small>"),
        "strike" => ("<strike>", "", "</strike>"),
        "sub" => ("<sub>", "", "</sub>"),
        "sup" => ("<sup>", "", "</sup>"),
        _ => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("String.prototype.{method} is not supported in this milestone"),
                span: Some(span),

                phase: None,
            });
        }
    };

    let mut result = LoweredExpr::RuntimeCall {
        intrinsic: RuntimeIntrinsic::Concat,
        args: vec![
            object,
            LoweredExpr::String(close_tag.to_owned(), Span::generated("str")),
        ],

        span: Span::generated("runtime_call"),
    };

    let has_arg = !open_suffix.is_empty();
    if has_arg {
        let needs_escaping = matches!(method, "anchor" | "fontcolor" | "fontsize" | "link");
        let mut arg = args.into_iter().next().unwrap_or(LoweredExpr::String(
            "undefined".to_owned(),
            Span::generated("str"),
        ));
        // Spec requires escaping " as &quot; in attribute values (B.2.3.10, B.2.3.6, etc.)
        if needs_escaping {
            arg = LoweredExpr::RuntimeCall {
                intrinsic: RuntimeIntrinsic::StringReplaceAll,
                args: vec![
                    arg,
                    LoweredExpr::String("\"".to_owned(), Span::generated("str")),
                    LoweredExpr::String("&quot;".to_owned(), Span::generated("str")),
                ],

                span: Span::generated("runtime_call"),
            };
        }
        result = LoweredExpr::RuntimeCall {
            intrinsic: RuntimeIntrinsic::Concat,
            args: vec![
                arg,
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeIntrinsic::Concat,
                    args: vec![
                        LoweredExpr::String(open_suffix.to_owned(), Span::generated("str")),
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeIntrinsic::Concat,
                            args: vec![
                                LoweredExpr::String(">".to_owned(), Span::generated("str")),
                                result,
                            ],

                            span: Span::generated("runtime_call"),
                        },
                    ],
                    span: Span::generated("RuntimeCall"),
                },
            ],
            span: Span::generated("RuntimeCall"),
        };
    }

    Ok(LoweredExpr::RuntimeCall {
        intrinsic: RuntimeIntrinsic::Concat,
        args: vec![
            LoweredExpr::String(open_prefix.to_owned(), Span::generated("str")),
            result,
        ],
        span: Span::generated("runtime_call"),
    })
}
