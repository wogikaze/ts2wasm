use super::super::{
    bigint_runtime_fn_intrinsic, block_contains_arguments, block_contains_this,
    function_body_is_strict,
};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedParam, ResolvedStmt};
use crate::lowered::classes::{ObjectAccessorKey, ObjectAccessorProp};
use crate::lowered::facts::FunctionMethodKind;
use crate::lowered::*;
use std::collections::HashMap;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
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

        if let ResolvedExpr::ComputedIndex { object, index } = callee
            && let ResolvedExpr::Ident(receiver_name) = object.as_ref()
            && let Ok(obj_local) = self.resolve_local(receiver_name)
            && let Some(key) =
                super::super::string::resolved_expr_static_property_key_value(&self.ctx, index)
            && let Some(method_id) = self
                .ctx
                .classes
                .object_function_props
                .get(&obj_local)
                .and_then(|props| props.get(&key))
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

        if let ResolvedExpr::MethodCall {
            object,
            method,
            args: bind_args,
            ..
        } = callee
            && method == "bind"
        {
            return self.lower_function_bind_direct_call(object, bind_args, args, span);
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

        if func_name == "String" {
            let lowered = match args.first() {
                None => LoweredExpr::String(String::new(), Span::generated("str")),
                Some(ResolvedExpr::String(value)) => {
                    LoweredExpr::String(value.clone(), Span::generated("str"))
                }
                Some(value) => LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BooleanToString,
                    args: vec![self.lower_expr(value)?],
                    span: Span::generated("runtime_call"),
                },
            };
            return Ok(lowered);
        }

        if let Some(intrinsic) = bigint_runtime_fn_intrinsic(func_name) {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic,
                args: self.lower_call_args(args)?,

                span: Span::generated("runtime_call"),
            });
        }

        if let Ok(local_id) = self.resolve_local(func_name)
            && let Some(binding) = self
                .ctx
                .facts
                .function_method_locals
                .get(&local_id)
                .cloned()
        {
            let receiver = match args.first() {
                Some(receiver) => self.lower_expr(receiver)?,
                None => LoweredExpr::Undefined(Span::generated("undef")),
            };
            let explicit_args = match binding.kind {
                FunctionMethodKind::Call => args.iter().skip(1).cloned().collect::<Vec<_>>(),
                FunctionMethodKind::Apply => function_apply_explicit_args(args, span)?,
            };
            let lowered_args =
                self.lower_function_call_args(binding.func_id, receiver, &explicit_args)?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(binding.func_id),
                args: lowered_args,
                span: Span::generated("call"),
            });
        }

        if let Ok(local_id) = self.resolve_local(func_name)
            && let Some(bound) = self.ctx.facts.bound_function_locals.get(&local_id).cloned()
        {
            let combined_args = bound
                .bound_args
                .iter()
                .chain(args.iter())
                .cloned()
                .collect::<Vec<_>>();
            let receiver = self.lower_expr(&bound.receiver)?;
            let lowered_args =
                self.lower_function_call_args(bound.func_id, receiver, &combined_args)?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(bound.func_id),
                args: lowered_args,
                span: Span::generated("call"),
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

            // SuperCallThis: derived constructors pass the active this local into
            // the parent constructor before forwarding explicit super(...) args.
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
            && crate::lowered::resolver::expr::facts::resolved_expr_is_bigint(&self.ctx, arg)
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

    fn lower_generator_call(&mut self, func_name: &str) -> Result<LoweredExpr, Diagnostic> {
        if self
            .ctx
            .facts
            .generator_function_steps
            .contains_key(func_name)
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::GeneratorYield,
                args: vec![LoweredExpr::ArrayNew {
                    elements: vec![],
                    span: Span::generated("array"),
                }],
                span: Span::generated("runtime_call"),
            });
        }
        let yields = self
            .ctx
            .facts
            .generator_function_yields
            .get(func_name)
            .cloned()
            .unwrap_or_default();
        let elements = yields
            .iter()
            .map(|expr| self.lower_expr(expr))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::GeneratorYield,
            args: vec![LoweredExpr::ArrayNew {
                elements,
                span: Span::generated("array"),
            }],
            span: Span::generated("runtime_call"),
        })
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
        if args.is_empty() && self.ctx.facts.generator_function_names.contains(func_name) {
            return self.lower_generator_call(func_name);
        }
        let signature = self
            .ctx
            .symbols
            .function_signatures
            .get(&func_id)
            .copied()
            .unwrap_or_default();
        if signature.needs_receiver && !signature.is_strict {
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

    fn lower_function_bind_direct_call(
        &mut self,
        object: &ResolvedExpr,
        bind_args: &[ResolvedExpr],
        call_args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let ResolvedExpr::Ident(func_name) = object else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-458: Function.prototype.bind direct calls require an identifier function"
                        .to_owned(),
                span: Some(span),
                phase: None,
            });
        };
        let func_id = self.resolve_func(func_name)?;
        let receiver = match bind_args.first() {
            Some(receiver) => self.lower_expr(receiver)?,
            None => LoweredExpr::Undefined(Span::generated("undef")),
        };
        let combined_args = bind_args
            .iter()
            .skip(1)
            .chain(call_args.iter())
            .cloned()
            .collect::<Vec<_>>();
        let lowered_args = self.lower_function_call_args(func_id, receiver, &combined_args)?;
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
        if let Some(result) =
            Self::direct_return_this_iife_result(self.ctx.is_strict_context(), params, body, args)
        {
            return Ok(result);
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

    fn direct_return_this_iife_result(
        parent_is_strict: bool,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
        args: &[ResolvedExpr],
    ) -> Option<LoweredExpr> {
        if !params.is_empty() || !args.is_empty() {
            return None;
        }

        let mut first_non_directive = 0;
        while let Some(ResolvedStmt::Expr(ResolvedExpr::String(_))) = body.get(first_non_directive)
        {
            first_non_directive += 1;
        }
        let returns_this = matches!(
            &body[first_non_directive..],
            [ResolvedStmt::Return(ResolvedExpr::This { .. })]
        );
        if !returns_this {
            return None;
        }

        if function_body_is_strict(parent_is_strict, body) {
            let _check = crate::lowered::ctx::StrictModeCheck::StrictThis;
            return Some(LoweredExpr::Undefined(Span::generated("undef")));
        }

        Some(LoweredExpr::ObjectNew {
            props: Vec::new(),
            non_enumerable: 0,
            span: Span::generated("object_new"),
        })
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
            .filter_map(|prop| {
                let key = prop.static_key()?;
                if let ResolvedExpr::Ident(name) = prop.value() {
                    self.resolve_func(name)
                        .ok()
                        .map(|func_id| (key.to_owned(), func_id))
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

    pub(crate) fn function_props_for_lowered_object_expr(
        &self,
        expr: &LoweredExpr,
    ) -> Option<HashMap<String, FuncId>> {
        let mut function_props = HashMap::new();
        self.collect_lowered_object_function_props(expr, &mut function_props)?;
        if function_props.is_empty() {
            None
        } else {
            Some(function_props)
        }
    }

    pub(crate) fn accessor_props_for_lowered_object_expr(
        &self,
        expr: &LoweredExpr,
    ) -> Option<HashMap<ObjectAccessorKey, ObjectAccessorProp>> {
        let mut accessor_props = HashMap::new();
        self.collect_lowered_object_accessor_props(expr, &mut accessor_props)?;
        if accessor_props.is_empty() {
            None
        } else {
            Some(accessor_props)
        }
    }

    fn collect_lowered_object_accessor_props(
        &self,
        expr: &LoweredExpr,
        accessor_props: &mut HashMap<ObjectAccessorKey, ObjectAccessorProp>,
    ) -> Option<()> {
        match expr {
            LoweredExpr::ObjectNew { .. } => Some(()),
            LoweredExpr::Block { stmts, result, .. } => {
                let LoweredExpr::Local(object_local, _) = result.as_ref() else {
                    return None;
                };
                let mut saw_object_init = false;
                for stmt in stmts {
                    match stmt {
                        LoweredStmt::Let(local, value, _) if local == object_local => {
                            self.collect_lowered_object_accessor_props(value, accessor_props)?;
                            saw_object_init = true;
                        }
                        LoweredStmt::Expr(expr, _) if saw_object_init => {
                            self.apply_lowered_object_property_write_to_accessor_props(
                                *object_local,
                                expr,
                                accessor_props,
                            )?;
                        }
                        _ => {}
                    }
                }
                saw_object_init.then_some(())
            }
            _ => None,
        }
    }

    fn collect_lowered_object_function_props(
        &self,
        expr: &LoweredExpr,
        function_props: &mut HashMap<String, FuncId>,
    ) -> Option<()> {
        match expr {
            LoweredExpr::ObjectNew { props, .. } => {
                self.apply_lowered_object_props_to_function_props(props, function_props);
                Some(())
            }
            LoweredExpr::Block { stmts, result, .. } => {
                let LoweredExpr::Local(object_local, _) = result.as_ref() else {
                    return None;
                };
                let mut saw_object_init = false;
                for stmt in stmts {
                    match stmt {
                        LoweredStmt::Let(local, value, _) if local == object_local => {
                            self.collect_lowered_object_function_props(value, function_props)?;
                            saw_object_init = true;
                        }
                        LoweredStmt::Expr(expr, _) if saw_object_init => {
                            self.apply_lowered_object_property_write_to_function_props(
                                *object_local,
                                expr,
                                function_props,
                            )?;
                        }
                        _ => {}
                    }
                }
                saw_object_init.then_some(())
            }
            _ => None,
        }
    }

    fn apply_lowered_object_props_to_function_props(
        &self,
        props: &[(String, LoweredExpr)],
        function_props: &mut HashMap<String, FuncId>,
    ) {
        for (key, value) in props {
            self.apply_function_prop_value(key.clone(), value, function_props);
        }
    }

    fn apply_lowered_object_property_write_to_function_props(
        &self,
        object_local: LocalId,
        expr: &LoweredExpr,
        function_props: &mut HashMap<String, FuncId>,
    ) -> Option<()> {
        match expr {
            LoweredExpr::PropertySet {
                object, key, value, ..
            } if matches!(object.as_ref(), LoweredExpr::Local(local, _) if *local == object_local) =>
            {
                self.apply_function_prop_value(key.clone(), value, function_props);
                Some(())
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
                ..
            } if matches!(object.as_ref(), LoweredExpr::Local(local, _) if *local == object_local) =>
            {
                let key = self.lowered_static_property_key(index)?;
                self.apply_function_prop_value(key, value, function_props);
                Some(())
            }
            _ => Some(()),
        }
    }

    fn apply_lowered_object_property_write_to_accessor_props(
        &self,
        object_local: LocalId,
        expr: &LoweredExpr,
        accessor_props: &mut HashMap<ObjectAccessorKey, ObjectAccessorProp>,
    ) -> Option<()> {
        match expr {
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ObjectDefineProperty,
                args,
                ..
            } if args.len() == 3
                && matches!(&args[0], LoweredExpr::Local(local, _) if *local == object_local) =>
            {
                let key = self.lowered_static_accessor_key(&args[1])?;
                self.apply_accessor_descriptor_value(key, &args[2], accessor_props);
                Some(())
            }
            LoweredExpr::PropertySet {
                object, key, value, ..
            } if matches!(object.as_ref(), LoweredExpr::Local(local, _) if *local == object_local) =>
            {
                let accessor_key = ObjectAccessorKey::Property(key.clone());
                if let Some(prop) = accessor_props.get(&accessor_key)
                    && prop.set.is_some()
                {
                    return Some(());
                }
                if !matches!(value.as_ref(), LoweredExpr::ArrowFn { .. }) {
                    accessor_props.remove(&accessor_key);
                }
                Some(())
            }
            _ => Some(()),
        }
    }

    pub(crate) fn accessor_prop_from_descriptor_expr(
        &self,
        desc: &LoweredExpr,
    ) -> Option<ObjectAccessorProp> {
        let LoweredExpr::ObjectNew { props, .. } = desc else {
            return None;
        };
        let mut accessor = ObjectAccessorProp::default();
        for (key, value) in props {
            match (key.as_str(), value) {
                ("get", LoweredExpr::ArrowFn { func_id, .. }) => accessor.get = Some(*func_id),
                ("set", LoweredExpr::ArrowFn { func_id, .. }) => accessor.set = Some(*func_id),
                _ => {}
            }
        }
        (accessor.get.is_some() || accessor.set.is_some()).then_some(accessor)
    }

    fn apply_accessor_descriptor_value(
        &self,
        key: ObjectAccessorKey,
        desc: &LoweredExpr,
        accessor_props: &mut HashMap<ObjectAccessorKey, ObjectAccessorProp>,
    ) {
        if let Some(accessor) = self.accessor_prop_from_descriptor_expr(desc) {
            accessor_props
                .entry(key)
                .and_modify(|existing| {
                    if accessor.get.is_some() {
                        existing.get = accessor.get;
                    }
                    if accessor.set.is_some() {
                        existing.set = accessor.set;
                    }
                })
                .or_insert(accessor);
            return;
        }
        if matches!(desc, LoweredExpr::ObjectNew { .. }) {
            accessor_props.remove(&key);
        }
    }

    fn apply_function_prop_value(
        &self,
        key: String,
        value: &LoweredExpr,
        function_props: &mut HashMap<String, FuncId>,
    ) {
        if let LoweredExpr::ArrowFn { func_id, .. } = value {
            function_props.insert(key, *func_id);
        } else {
            function_props.remove(&key);
        }
    }

    fn lowered_static_property_key(&self, expr: &LoweredExpr) -> Option<String> {
        match expr {
            LoweredExpr::String(value, _) => Some(value.clone()),
            LoweredExpr::Number(value, _) => Some(value.to_string()),
            LoweredExpr::Local(local, _) => self.ctx.facts.string_value(*local).cloned(),
            _ => None,
        }
    }

    fn lowered_static_accessor_key(&self, expr: &LoweredExpr) -> Option<ObjectAccessorKey> {
        match expr {
            LoweredExpr::String(value, _) => Some(ObjectAccessorKey::Property(value.clone())),
            LoweredExpr::Number(value, _) => Some(ObjectAccessorKey::Property(value.to_string())),
            LoweredExpr::Local(local, _) => self
                .ctx
                .facts
                .string_value(*local)
                .cloned()
                .map(ObjectAccessorKey::Property)
                .or_else(|| {
                    self.ctx
                        .facts
                        .symbol_value_locals
                        .contains(local)
                        .then_some(ObjectAccessorKey::SymbolLocal(*local))
                }),
            _ => None,
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

fn function_apply_explicit_args(
    args: &[ResolvedExpr],
    span: Span,
) -> Result<Vec<ResolvedExpr>, Diagnostic> {
    match args.get(1) {
        None | Some(ResolvedExpr::Undefined | ResolvedExpr::Null) => Ok(Vec::new()),
        Some(ResolvedExpr::Array(elements)) => Ok(elements
            .iter()
            .map(|element| match element {
                ResolvedArrayElement::Present(expr) => expr.clone(),
                ResolvedArrayElement::Hole => ResolvedExpr::Undefined,
            })
            .collect()),
        Some(ResolvedExpr::Ident(name)) => Ok(vec![ResolvedExpr::Spread(Box::new(
            ResolvedExpr::Ident(name.clone()),
        ))]),
        Some(_) => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-458: Function.prototype.apply bound values currently support array literals, dense array locals, null, or undefined argArray".to_owned(),
            span: Some(span),
            phase: None,
        }),
    }
}
