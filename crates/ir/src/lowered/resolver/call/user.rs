use super::super::{
    bigint_runtime_fn_intrinsic, block_contains_arguments, block_contains_this,
    is_array_from_call_receiver, is_array_prototype_map_call_receiver,
    is_array_prototype_push_expr, is_identity_arrow_callback, is_set_prototype_property_expr,
    is_static_date_constructor_expr, is_string_split_result_expr,
    numeric_ascending_sort_arrow_callback, private_storage_observable_access_diagnostic,
    string_constructor_arrow_callback, string_split_arrow_separator, unary_plus_arrow_callback,
    unsupported_array_map_diagnostic, unsupported_array_sort_diagnostic,
};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedParam, ResolvedStmt};
use crate::lowered::*;
use std::collections::HashMap;
use ts2wasm_shared::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl<'a> super::super::Resolver {
    pub(crate) fn lower_call_expr(
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

        if let Some(intrinsic) = bigint_runtime_fn_intrinsic(func_name) {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic,
                args: self.lower_call_args(args)?,

                span: Span::generated("runtime_call"),
            });
        }

        if let Ok(local_id) = self.resolve_local(func_name)
            && let Some(closure) = self.ctx.facts.arrow_locals.get(&local_id).cloned()
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
            && self.ctx.facts.heap_closure_locals.contains(&local_id)
        {
            let receiver = if self.ctx.facts.env_cell_locals.contains(&local_id) {
                LoweredExpr::EnvCellGet(local_id, Span::generated("env_cell_get"))
            } else {
                LoweredExpr::Local(local_id, Span::generated("local"))
            };
            let mut lowered_args = vec![receiver];
            lowered_args.extend(self.lower_call_args(args)?);
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::HeapClosureCall,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            });
        }

        if func_name == "super" {
            if !self.ctx.classes.in_constructor {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "super(...) is only supported in constructors".to_owned(),
                    span: None,

                    phase: None,
                });
            }
            let class_name = self
                .ctx
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
                .ctx
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
                .ctx
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
                intrinsic: RuntimeFn::BigIntToString,
                args: vec![self.lower_expr(arg)?],

                span: Span::generated("runtime_call"),
            });
        }

        if func_name == "Boolean"
            && let [ResolvedExpr::BigIntLiteral { .. }] = args
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::BigIntToBoolean,
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
                intrinsic: RuntimeFn::SymbolNew,
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
            && self.ctx.facts.nullish_locals.contains(&local_id)
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
                    && self.ctx.symbols.param_locals.contains(&local_id)
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
                    intrinsic: RuntimeFn::HeapClosureCall,
                    args: lowered_args,

                    span: Span::generated("runtime_call"),
                });
            }
            Err(_)
                if self
                    .ctx
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
        self.lower_call_with_func_id(func_id, func_name, args, span)
    }

    /// Helper for lower_call_expr: emit the function call after resolution,
    /// checking for receiver binding requirements.
    fn lower_call_with_func_id(
        &mut self,
        func_id: FuncId,
        func_name: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if self
            .ctx
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
    pub(crate) fn lower_arrow_fn_iife(
        &mut self,
        params: &[String],
        body: &ResolvedExpr,
        body_stmts: &[ResolvedStmt],
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let lowered = self.lower_arrow_fn(params, body, body_stmts)?;
        let LoweredExpr::ArrowFn {
            func_id, captures, ..
        } = lowered
        else {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "arrow function lowering must produce an ArrowFn token".to_owned(),
                span: Some(span),

                phase: None,
            });
        };
        let explicit_args = self.lower_call_args(args)?;
        let mut lowered_args = explicit_args
            .into_iter()
            .take(params.len())
            .collect::<Vec<_>>();
        for _ in lowered_args.len()..params.len() {
            lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
        }
        lowered_args.extend(
            captures
                .into_iter()
                .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
        );
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: lowered_args,

            span: Span::generated("call"),
        })
    }

    pub(crate) fn lower_function_expr_call(
        &mut self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if Self::is_direct_return_this_iife(params, body, args) {
            return Ok(LoweredExpr::ObjectNew {
                props: Vec::new(),
                non_enumerable: 0,
                span: Span::generated("object_new"),
            });
        }
        if params.iter().any(|param| param.is_rest) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-274: direct function-expression spread calls do not support rest parameters in this slice".to_owned(),
                span: Some(span),

                phase: None,});
        }
        // Only reject this/arguments for spread calls, not all function-expr calls
        let has_spread_args = args.iter().any(|a| matches!(a, ResolvedExpr::Spread(_)));
        if has_spread_args && (block_contains_this(body) || block_contains_arguments(body)) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-274: direct function-expression spread calls with `this` or `arguments` require broader call-expression runtime support".to_owned(),
                span: Some(span),

                phase: None,});
        }

        let lowered = self.lower_named_function_expr(name, params, body)?;
        let LoweredExpr::ArrowFn {
            func_id, captures, ..
        } = lowered
        else {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "function expression lowering must produce a direct function token"
                    .to_owned(),
                span: Some(span),

                phase: None,
            });
        };

        let explicit_args = self.lower_call_args(args)?;
        let mut lowered_args = explicit_args
            .into_iter()
            .take(params.len())
            .collect::<Vec<_>>();
        for _ in lowered_args.len()..params.len() {
            lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
        }
        lowered_args.extend(
            captures
                .into_iter()
                .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
        );
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: lowered_args,

            span: Span::generated("call"),
        })
    }

    fn is_direct_return_this_iife(
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
        args: &[ResolvedExpr],
    ) -> bool {
        params.is_empty()
            && args.is_empty()
            && matches!(body, [ResolvedStmt::Return(ResolvedExpr::This { .. })])
    }

    pub(crate) fn function_props_for_object_expr(
        &self,
        expr: &ResolvedExpr,
    ) -> Option<HashMap<String, FuncId>> {
        let ResolvedExpr::Object(props) = expr else {
            return None;
        };
        let function_props = props
            .iter()
            .filter_map(|(key, value)| {
                if let ResolvedExpr::Ident(name) = value {
                    self.resolve_func(name)
                        .ok()
                        .map(|func_id| (key.clone(), func_id))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();
        if function_props.is_empty() {
            None
        } else {
            Some(function_props)
        }
    }

    pub(crate) fn is_function_identifier(&self, expr: &ResolvedExpr) -> bool {
        matches!(expr, ResolvedExpr::Ident(name) if self.resolve_func(name).is_ok())
    }

    pub(crate) fn lower_function_metadata_property(
        &self,
        name: &str,
        key: &str,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let func_id = self.resolve_func(name)?;
        match key {
            "name" => Ok(LoweredExpr::String(name.to_owned(), Span::generated("str"))),
            "length" => {
                let signature = self
                    .ctx
                    .symbols
                    .function_signatures
                    .get(&func_id)
                    .copied()
                    .unwrap_or_default();
                if let Some(length) = signature.metadata_length {
                    Ok(LoweredExpr::Number(length as i32, Span::generated("num")))
                } else {
                    Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-062f: function `{name}` length metadata is only supported for fixed-arity function declarations"
                        ),
                        span: Some(span),

                        phase: None,
                    })
                }
            }
            "prototype" => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062f: function `{name}` prototype metadata is not supported in this slice"
                ),
                span: Some(span),

                phase: None,
            }),
            _ => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062f: function `{name}` metadata property `{key}` is not supported"
                ),
                span: Some(span),

                phase: None,
            }),
        }
    }
}
