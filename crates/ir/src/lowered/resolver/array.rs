use super::{
    is_identity_arrow_callback, is_number_double_arrow_callback, unsupported_array_map_diagnostic,
};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedParam, ResolvedStmt};
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::Resolver {
    pub(super) fn lower_array_literal(
        &mut self,
        elements: &[ResolvedArrayElement],
    ) -> Result<LoweredExpr, Diagnostic> {
        if elements
            .iter()
            .any(|element| matches!(element, ResolvedArrayElement::Hole))
        {
            let mut slots = Vec::with_capacity(elements.len());
            for element in elements {
                match element {
                    ResolvedArrayElement::Present(expr) => {
                        slots.push(LoweredArraySlot::Present(self.lower_expr(expr)?));
                    }
                    ResolvedArrayElement::Hole => slots.push(LoweredArraySlot::Hole),
                }
            }
            return Ok(LoweredExpr::ArrayNewSparse {
                slots,
                span: Span::generated("array_new_sparse"),
            });
        }
        if !elements.iter().any(|element| {
            matches!(
                element,
                ResolvedArrayElement::Present(ResolvedExpr::Spread(_))
            )
        }) {
            let lowered = self.lower_array_literal_elements(elements)?;
            return Ok(LoweredExpr::ArrayNew {
                elements: lowered,
                span: Span::generated("array_new"),
            });
        }

        let mut segments = Vec::new();
        let mut pending_dense = Vec::new();

        for element in elements {
            match element {
                ResolvedArrayElement::Present(ResolvedExpr::Spread(spread_expr)) => {
                    if let ResolvedExpr::Array(spread_elements) = spread_expr.as_ref() {
                        pending_dense.extend(self.lower_array_literal_elements(spread_elements)?);
                        continue;
                    }

                    if let Some(value) =
                        crate::lowered::resolver::string::static_string_spread_value(
                            &self.ctx,
                            spread_expr.as_ref(),
                        )
                    {
                        pending_dense.extend(
                            crate::lowered::resolver::string::lower_ascii_string_spread_chars(
                                &value,
                            )?,
                        );
                        continue;
                    }

                    if let Some(array_segment) =
                        self.lower_dense_array_local_spread_operand(spread_expr.as_ref())?
                    {
                        Self::flush_array_segment(&mut segments, &mut pending_dense);
                        segments.push(array_segment);
                        continue;
                    }

                    if let Some(set_array) = self.lower_set_spread_operand(spread_expr.as_ref())? {
                        Self::flush_array_segment(&mut segments, &mut pending_dense);
                        segments.push(set_array);
                        continue;
                    }

                    if let Some(map_array) = self.lower_map_spread_operand(spread_expr.as_ref())? {
                        Self::flush_array_segment(&mut segments, &mut pending_dense);
                        segments.push(map_array);
                        continue;
                    }

                    if crate::lowered::resolver::expr::facts::is_generator_call_spread_operand(
                        &self.ctx,
                        spread_expr.as_ref(),
                    ) {
                        return Err(crate::lowered::resolver::expr::facts::unsupported_generator_spread_diagnostic());
                    }

                    if crate::lowered::resolver::expr::facts::resolved_expr_has_symbol_iterator_property(&self.ctx, spread_expr.as_ref()) {
                        Self::flush_array_segment(&mut segments, &mut pending_dense);
                        segments.push(self.lower_spread_via_iterator(spread_expr.as_ref())?);
                        continue;
                    }

                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "issue-274: array literal spread is only supported for literal arrays, known dense array locals, and known Set locals in this milestone"
                                .to_owned(),
                        span: None,

                        phase: None,});
                }
                ResolvedArrayElement::Present(expr) => pending_dense.push(self.lower_expr(expr)?),
                ResolvedArrayElement::Hole => {
                    pending_dense.push(LoweredExpr::Undefined(Span::generated("undef")))
                }
            }
        }

        Self::flush_array_segment(&mut segments, &mut pending_dense);

        let mut iter = segments.into_iter();
        let Some(mut combined) = iter.next() else {
            return Ok(LoweredExpr::ArrayNew {
                elements: vec![],
                span: Span::generated("array_new"),
            });
        };
        for segment in iter {
            combined = LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayConcat,
                args: vec![combined, segment],

                span: Span::generated("runtime_call"),
            };
        }
        Ok(combined)
    }

    pub(super) fn lower_array_literal_map(
        &mut self,
        array_expr: &ResolvedExpr,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let ResolvedExpr::Array(elements) = array_expr else {
            return Err(unsupported_array_map_diagnostic(Some(span)));
        };
        self.lower_array_map_elements(array_expr, elements, args, span)
    }

    pub(super) fn lower_array_prototype_map_call(
        &mut self,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let Some((receiver, map_args)) = args.split_first() else {
            return Err(Diagnostic {
                code: DiagCode::ArityMismatch,
                message: "Array.prototype.map.call expects a receiver argument".to_owned(),
                span: Some(span),

                phase: None,
            });
        };
        match receiver {
            ResolvedExpr::Array(elements) => {
                self.lower_array_map_elements(receiver, elements, map_args, span)
            }
            ResolvedExpr::Object(props) => {
                let Some(elements) = dense_array_like_object_elements(props) else {
                    return Err(unsupported_array_map_diagnostic(Some(span)));
                };
                let elements = elements
                    .into_iter()
                    .map(ResolvedArrayElement::Present)
                    .collect::<Vec<_>>();
                self.lower_array_map_elements(receiver, &elements, map_args, span)
            }
            ResolvedExpr::Ident(name) => {
                let Some(elements) =
                    crate::lowered::resolver::expr::facts::static_function_array_like_elements(
                        &self.ctx, name,
                    )
                else {
                    if is_identity_arrow_callback(map_args) {
                        return Ok(LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::ArrayMapArrayLikeIdentity,
                            args: vec![self.lower_expr(receiver)?],

                            span: Span::generated("runtime_call"),
                        });
                    }
                    if is_number_double_arrow_callback(map_args) {
                        return Ok(LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::ArrayMapArrayLikeDouble,
                            args: vec![self.lower_expr(receiver)?],

                            span: Span::generated("runtime_call"),
                        });
                    }
                    return Err(unsupported_array_map_diagnostic(Some(span)));
                };
                let elements = elements
                    .into_iter()
                    .map(ResolvedArrayElement::Present)
                    .collect::<Vec<_>>();
                self.lower_array_map_elements(receiver, &elements, map_args, span)
            }
            _ if is_identity_arrow_callback(map_args) => Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayMapArrayLikeIdentity,
                args: vec![self.lower_expr(receiver)?],

                span: Span::generated("runtime_call"),
            }),
            _ if is_number_double_arrow_callback(map_args) => Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayMapArrayLikeDouble,
                args: vec![self.lower_expr(receiver)?],

                span: Span::generated("runtime_call"),
            }),
            _ => Err(unsupported_array_map_diagnostic(Some(span))),
        }
    }

    pub(super) fn lower_array_from_call(
        &mut self,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let [source] = args else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-313: Array.from currently supports exactly one source argument"
                    .to_owned(),
                span: Some(span),

                phase: None,
            });
        };

        if crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, source) {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayValues,
                args: vec![self.lower_expr(source)?],

                span: Span::generated("runtime_call"),
            });
        }

        Ok(LoweredExpr::ArrayNew {
            elements: Vec::new(),

            span: Span::generated("array_new"),
        })
    }

    pub(super) fn lower_array_map_elements(
        &mut self,
        array_expr: &ResolvedExpr,
        elements: &[ResolvedArrayElement],
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let ([callback] | [callback, _]) = args else {
            return Err(unsupported_array_map_diagnostic(Some(span)));
        };
        let is_sparse = elements
            .iter()
            .any(|element| matches!(element, ResolvedArrayElement::Hole));
        let mut mapped = Vec::with_capacity(elements.len());
        for (index, element) in elements.iter().enumerate() {
            match element {
                ResolvedArrayElement::Present(expr) => {
                    let element = self.lower_expr(expr)?;
                    mapped.push(LoweredArraySlot::Present(
                        self.lower_array_map_callback_call(
                            callback,
                            args.get(1),
                            element,
                            index,
                            array_expr,
                            span,
                        )?,
                    ));
                }
                ResolvedArrayElement::Hole => mapped.push(LoweredArraySlot::Hole),
            }
        }
        if is_sparse {
            Ok(LoweredExpr::ArrayNewSparse {
                slots: mapped,
                span: Span::generated("array_new_sparse"),
            })
        } else {
            Ok(LoweredExpr::ArrayNew {
                elements: mapped
                    .into_iter()
                    .map(|slot| match slot {
                        LoweredArraySlot::Present(expr) => expr,
                        LoweredArraySlot::Hole => LoweredExpr::Undefined(Span::generated("undef")),
                    })
                    .collect(),

                span: Span::generated("array_new"),
            })
        }
    }

    fn lower_array_map_callback_call(
        &mut self,
        callback: &ResolvedExpr,
        this_arg: Option<&ResolvedExpr>,
        element: LoweredExpr,
        index: usize,
        array_expr: &ResolvedExpr,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        match callback {
            ResolvedExpr::ArrowFn {
                params,
                body,
                body_stmts,
                ..
            } => {
                if params.len() > 3 {
                    return Err(unsupported_array_map_diagnostic(Some(span)));
                }
                let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = self.lower_arrow_fn(params, body, body_stmts)?
                else {
                    return Err(unsupported_array_map_diagnostic(Some(span)));
                };
                let mut explicit_args = vec![
                    element,
                    LoweredExpr::Number(index as i32, Span::generated("num")),
                ];
                explicit_args.push(self.lower_expr(array_expr)?);
                let mut call_args = explicit_args
                    .into_iter()
                    .take(params.len())
                    .collect::<Vec<_>>();
                call_args.extend(
                    captures
                        .iter()
                        .copied()
                        .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
                );
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(func_id),
                    args: call_args,

                    span: Span::generated("call"),
                })
            }
            ResolvedExpr::FunctionExpr { name, params, body } => self
                .lower_array_map_function_expr_callback_call(
                    name, params, body, this_arg, element, index, array_expr, span,
                ),
            ResolvedExpr::Ident(name) => {
                let func_id = self.resolve_func(name)?;
                let receiver = match this_arg {
                    Some(expr) => self.lower_expr(expr)?,
                    None => LoweredExpr::Undefined(Span::generated("undef")),
                };
                let signature = self
                    .ctx
                    .symbols
                    .function_signatures
                    .get(&func_id)
                    .copied()
                    .unwrap_or_default();
                let mut explicit_args = vec![
                    element,
                    LoweredExpr::Number(index as i32, Span::generated("num")),
                ];
                explicit_args.push(self.lower_expr(array_expr)?);
                let argument_props = explicit_args
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(index, arg)| (index.to_string(), arg))
                    .chain(std::iter::once((
                        "length".to_owned(),
                        LoweredExpr::Number(explicit_args.len() as i32, Span::generated("num")),
                    )))
                    .collect::<Vec<_>>();
                let mut call_args = Vec::new();
                if signature.needs_receiver {
                    call_args.push(receiver);
                }
                if signature.has_rest {
                    call_args.extend(explicit_args);
                } else {
                    let explicit_len = explicit_args.len();
                    call_args.extend(explicit_args.into_iter().take(signature.explicit_params));
                    for _ in explicit_len..signature.explicit_params {
                        call_args.push(LoweredExpr::Undefined(Span::generated("undef")));
                    }
                }
                if signature.needs_arguments {
                    call_args.push(LoweredExpr::ObjectNew {
                        props: argument_props,
                        non_enumerable: 0,

                        span: Span::generated("object_new"),
                    });
                }
                self.append_function_captures(func_id, &mut call_args)?;
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(func_id),
                    args: call_args,

                    span: Span::generated("call"),
                })
            }
            _ => Err(unsupported_array_map_diagnostic(Some(span))),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_array_map_function_expr_callback_call(
        &mut self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
        this_arg: Option<&ResolvedExpr>,
        element: LoweredExpr,
        index: usize,
        array_expr: &ResolvedExpr,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if params
            .iter()
            .any(|param| param.default.is_some() || param.is_rest)
        {
            return Err(unsupported_array_map_diagnostic(Some(span)));
        }
        if block_contains_arguments(body) {
            return Err(unsupported_array_map_diagnostic(Some(span)));
        }

        let receiver = match this_arg {
            Some(expr) => self.lower_expr(expr)?,
            None => LoweredExpr::Undefined(Span::generated("undef")),
        };
        let mut params_with_this = vec![ResolvedParam {
            name: "this".to_owned(),
            default: None,
            is_rest: false,
            span: None,
        }];
        params_with_this.extend(params.iter().cloned());
        let capture_names = self.nested_function_capture_names(name, &params_with_this, body)?;
        let mutable_captures = capture_names
            .iter()
            .filter(|capture| block_assigns_any_name(body, std::slice::from_ref(capture)))
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
            })
            .filter(|_| !self.ctx.facts.env_cell_names.contains(name));
        let mut function_signatures = self.ctx.symbols.function_signatures.clone();
        function_signatures.insert(
            func_id,
            FunctionSignature {
                explicit_params: params.len(),
                needs_receiver: true,
                ..FunctionSignature::default()
            },
        );
        let lowered = lower_function(
            func_id,
            &lowered_params,
            body,
            false,
            &self.ctx.symbols.function_ids,
            &function_signatures,
            &self.ctx.functions.function_captures,
            &self.ctx.functions.function_mutable_captures,
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
                next_func_id: self.ctx.functions.next_func_id,
                self_closure,
                recursion_depth: 0,
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

        let explicit_args = [
            element,
            LoweredExpr::Number(index as i32, Span::generated("num")),
            self.lower_expr(array_expr)?,
        ];
        let mut call_args = vec![receiver];
        call_args.extend(explicit_args.into_iter().take(params.len()));
        for _ in call_args.len()..=params.len() {
            call_args.push(LoweredExpr::Undefined(Span::generated("undef")));
        }
        call_args.extend(
            captures
                .into_iter()
                .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
        );
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: call_args,

            span: Span::generated("call"),
        })
    }

    pub(super) fn lower_array_literal_elements(
        &mut self,
        elements: &[ResolvedArrayElement],
    ) -> Result<Vec<LoweredExpr>, Diagnostic> {
        let mut lowered = Vec::new();
        for element in elements {
            match element {
                ResolvedArrayElement::Present(ResolvedExpr::Spread(spread_expr)) => {
                    if let ResolvedExpr::Array(spread_elements) = spread_expr.as_ref() {
                        lowered.extend(self.lower_array_literal_elements(spread_elements)?);
                    } else if let Some(value) =
                        crate::lowered::resolver::string::static_string_spread_value(&self.ctx, spread_expr.as_ref())
                    {
                        lowered.extend(crate::lowered::resolver::string::lower_ascii_string_spread_chars(&value)?);
                    } else if crate::lowered::resolver::expr::facts::is_generator_call_spread_operand(&self.ctx, spread_expr.as_ref()) {
                        return Err(crate::lowered::resolver::expr::facts::unsupported_generator_spread_diagnostic());
                    } else if crate::lowered::resolver::expr::facts::resolved_expr_has_symbol_iterator_property(&self.ctx, spread_expr.as_ref())
                    {
                        return Err(crate::lowered::resolver::expr::facts::unsupported_symbol_iterator_spread_diagnostic());
                    } else {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message:
                                "issue-274: array literal spread is only supported for literal arrays and ASCII literal-derived strings in this milestone"
                                    .to_owned(),
                            span: None,

                            phase: None,});
                    }
                }
                ResolvedArrayElement::Present(expr) => lowered.push(self.lower_expr(expr)?),
                ResolvedArrayElement::Hole => {
                    lowered.push(LoweredExpr::Undefined(Span::generated("undef")))
                }
            }
        }
        Ok(lowered)
    }

    pub(super) fn flush_array_segment(
        segments: &mut Vec<LoweredExpr>,
        pending_dense: &mut Vec<LoweredExpr>,
    ) {
        if pending_dense.is_empty() {
            return;
        }
        segments.push(LoweredExpr::ArrayNew {
            elements: std::mem::take(pending_dense),

            span: Span::generated("array_new"),
        });
    }

    pub(super) fn lower_dense_array_local_spread_operand(
        &mut self,
        spread_expr: &ResolvedExpr,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        let ResolvedExpr::Ident(name) = spread_expr else {
            return Ok(None);
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return Ok(None);
        };
        if self.ctx.facts.array_locals.contains(&local_id) {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayConcat,
                args: vec![
                    LoweredExpr::ArrayNew {
                        elements: vec![],
                        span: Span::generated("array_new"),
                    },
                    LoweredExpr::Local(local_id, Span::generated("local")),
                ],
                span: Span::generated("RuntimeCall"),
            }));
        }
        Ok(None)
    }

    #[allow(dead_code)]
    pub(super) fn is_known_dense_array_local_spread_operand(
        &self,
        spread_expr: &ResolvedExpr,
    ) -> bool {
        let ResolvedExpr::Ident(name) = spread_expr else {
            return false;
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return false;
        };
        self.ctx.facts.array_locals.contains(&local_id)
    }

    pub(super) fn lower_set_spread_operand(
        &mut self,
        spread_expr: &ResolvedExpr,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        let ResolvedExpr::Ident(name) = spread_expr else {
            return Ok(None);
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return Ok(None);
        };
        if self
            .ctx
            .classes
            .local_classes
            .get(&local_id)
            .is_some_and(|class_name| class_name == "Set")
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetValuesArray,
                args: vec![LoweredExpr::Local(local_id, Span::generated("local"))],

                span: Span::generated("runtime_call"),
            }));
        }
        Ok(None)
    }

    pub(super) fn lower_map_spread_operand(
        &mut self,
        spread_expr: &ResolvedExpr,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        let ResolvedExpr::Ident(name) = spread_expr else {
            return Ok(None);
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return Ok(None);
        };
        if self
            .ctx
            .classes
            .local_classes
            .get(&local_id)
            .is_some_and(|class_name| class_name == "Map")
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapValuesArray,
                args: vec![LoweredExpr::Local(local_id, Span::generated("local"))],

                span: Span::generated("runtime_call"),
            }));
        }
        Ok(None)
    }

    #[allow(dead_code)]
    pub(super) fn is_known_set_local_spread_operand(&self, spread_expr: &ResolvedExpr) -> bool {
        let ResolvedExpr::Ident(name) = spread_expr else {
            return false;
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return false;
        };
        self.ctx
            .classes
            .local_classes
            .get(&local_id)
            .is_some_and(|class_name| class_name == "Set")
    }

    #[allow(dead_code)]
    pub(super) fn is_known_map_local_spread_operand(&self, spread_expr: &ResolvedExpr) -> bool {
        let ResolvedExpr::Ident(name) = spread_expr else {
            return false;
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return false;
        };
        self.ctx
            .classes
            .local_classes
            .get(&local_id)
            .is_some_and(|class_name| class_name == "Map")
    }

    pub(super) fn lower_array_callback_method(
        &mut self,
        method: &str,
        receiver: LoweredExpr,
        resolved_receiver: &ResolvedExpr,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let callback = &args[0];

        let (func_id, captures, param_count) = match callback {
            ResolvedExpr::ArrowFn {
                params,
                body,
                body_stmts,
                ..
            } => {
                if params.len() > 4 {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "issue-270: array method callbacks with more than 4 parameters are not supported"
                                .to_owned(),
                        span: Some(span),

                        phase: None,});
                }
                let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = self.lower_arrow_fn(params, body, body_stmts)?
                else {
                    return Err(unsupported_array_map_diagnostic(Some(span)));
                };
                (func_id, captures, params.len())
            }
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-270: non-arrow-function callbacks are not yet supported for array methods in this slice"
                            .to_owned(),
                    span: Some(span),

                    phase: None,});
            }
        };

        // Determine the init expression for reduce (if applicable)
        let init_expr = if method == "reduce" || method == "reduceRight" {
            let Some(init_arg) = args.get(1) else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-270: Array.prototype.reduce without initialValue is not yet supported"
                            .to_owned(),
                    span: None,

                    phase: None,});
            };
            Some(self.lower_expr(init_arg)?)
        } else {
            None
        };

        // For now, only handle Ident receivers (variable arrays)
        let receiver_local = match &receiver {
            LoweredExpr::Local(id, _) => *id,
            _ => {
                let temp = self.alloc_temp();
                // receiver is a literal array: store in temp
                return self.lower_literal_array_callback_method(
                    method,
                    temp,
                    receiver,
                    resolved_receiver,
                    args,
                    func_id,
                    captures,
                    param_count,
                    init_expr,
                    span,
                );
            }
        };

        self.lower_variable_array_callback_method(
            method,
            receiver_local,
            receiver,
            func_id,
            captures,
            param_count,
            init_expr,
        )
    }

    /// Lower a callback method on a variable array (Ident receiver).
    /// receiver must be a LoweredExpr::Local.
    /// init_expr is the initial value for reduce (if applicable).
    #[allow(clippy::too_many_arguments)]
    fn lower_variable_array_callback_method(
        &mut self,
        method: &str,
        receiver_local: LocalId,
        receiver: LoweredExpr,
        func_id: FuncId,
        captures: Vec<LocalId>,
        param_count: usize,
        init_expr: Option<LoweredExpr>,
    ) -> Result<LoweredExpr, Diagnostic> {
        let i = self.alloc_temp();
        let len_local = self.alloc_temp();

        let mut stmts = Vec::new();

        // Let(len_local, GetLength(receiver))
        stmts.push(LoweredStmt::Let(
            len_local,
            LoweredExpr::GetLength(Box::new(receiver.clone()), Span::generated("get_length")),
            Span::generated("Let"),
        ));

        let (init_stmts, while_body, result_expr) = match method {
            "forEach" => self.lower_array_foreach_callback(
                receiver_local,
                i,
                func_id,
                &captures,
                param_count,
            )?,
            "filter" => self.lower_array_filter_callback(
                receiver_local,
                i,
                func_id,
                &captures,
                param_count,
            )?,
            "find" | "findIndex" => self.lower_array_find_callback(
                receiver_local,
                i,
                func_id,
                &captures,
                param_count,
                method,
            )?,
            "findLast" | "findLastIndex" => self.lower_array_find_last_callback(
                receiver_local,
                i,
                func_id,
                &captures,
                param_count,
                method,
            )?,
            "some" => self.lower_array_some_every_callback(
                receiver_local,
                i,
                func_id,
                &captures,
                param_count,
                true,
            )?,
            "every" => self.lower_array_some_every_callback(
                receiver_local,
                i,
                func_id,
                &captures,
                param_count,
                false,
            )?,
            "reduce" | "reduceRight" => self.lower_array_reduce_callback(
                receiver_local,
                i,
                func_id,
                &captures,
                param_count,
                method,
                init_expr,
            )?,
            "flatMap" => self.lower_array_flatmap_callback(
                receiver_local,
                i,
                func_id,
                &captures,
                param_count,
            )?,
            "map" => {
                self.lower_array_map_callback(receiver_local, i, func_id, &captures, param_count)?
            }
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-270: array method `{}` is not supported for user callbacks",
                        method
                    ),
                    span: None,

                    phase: None,
                });
            }
        };
        stmts.extend(init_stmts);

        // Add initial Let(i, ...) based on iteration direction
        if method == "findLast" || method == "findLastIndex" || method == "reduceRight" {
            stmts.push(LoweredStmt::Let(
                i,
                LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(len_local, Span::generated("local"))),
                    op: LoweredBinaryOp::Subtract,
                    right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                    span: Span::generated("binary"),
                },
                Span::generated("Let"),
            ));
        } else {
            stmts.push(LoweredStmt::Let(
                i,
                LoweredExpr::Number(0, Span::generated("num")),
                Span::generated("Let"),
            ));
        }

        // Determine the While condition based on method
        let condition = match method {
            "find" => LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                    op: LoweredBinaryOp::Less,
                    right: Box::new(LoweredExpr::Local(len_local, Span::generated("local"))),
                    span: Span::generated("binary"),
                }),
                op: LoweredBinaryOp::And,
                right: Box::new(LoweredExpr::Binary {
                    left: Box::new(result_expr.clone()),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                    span: Span::generated("binary"),
                }),

                span: Span::generated("binary"),
            },
            "findIndex" => LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                    op: LoweredBinaryOp::Less,
                    right: Box::new(LoweredExpr::Local(len_local, Span::generated("local"))),
                    span: Span::generated("binary"),
                }),
                op: LoweredBinaryOp::And,
                right: Box::new(LoweredExpr::Binary {
                    left: Box::new(result_expr.clone()),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Number(-1, Span::generated("num"))),
                    span: Span::generated("binary"),
                }),

                span: Span::generated("binary"),
            },
            "findLast" => LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                    op: LoweredBinaryOp::GreaterEqual,
                    right: Box::new(LoweredExpr::Number(0, Span::generated("num"))),
                    span: Span::generated("binary"),
                }),
                op: LoweredBinaryOp::And,
                right: Box::new(LoweredExpr::Binary {
                    left: Box::new(result_expr.clone()),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                    span: Span::generated("binary"),
                }),

                span: Span::generated("binary"),
            },
            "findLastIndex" => LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                    op: LoweredBinaryOp::GreaterEqual,
                    right: Box::new(LoweredExpr::Number(0, Span::generated("num"))),
                    span: Span::generated("binary"),
                }),
                op: LoweredBinaryOp::And,
                right: Box::new(LoweredExpr::Binary {
                    left: Box::new(result_expr.clone()),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Number(-1, Span::generated("num"))),

                    span: Span::generated("binary"),
                }),

                span: Span::generated("binary"),
            },
            "some" => LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                    op: LoweredBinaryOp::Less,
                    right: Box::new(LoweredExpr::Local(len_local, Span::generated("local"))),

                    span: Span::generated("binary"),
                }),
                op: LoweredBinaryOp::And,
                right: Box::new(LoweredExpr::Unary {
                    op: LoweredUnaryOp::Not,
                    expr: Box::new(result_expr.clone()),
                    span: Span::generated("unary"),
                }),
                span: Span::generated("binary"),
            },
            "every" => LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                    op: LoweredBinaryOp::Less,
                    right: Box::new(LoweredExpr::Local(len_local, Span::generated("local"))),

                    span: Span::generated("binary"),
                }),
                op: LoweredBinaryOp::And,
                right: Box::new(result_expr.clone()),
                span: Span::generated("binary"),
            },
            "reduceRight" => LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::GreaterEqual,
                right: Box::new(LoweredExpr::Number(0, Span::generated("num"))),
                span: Span::generated("binary"),
            },
            _ => LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Less,
                right: Box::new(LoweredExpr::Local(len_local, Span::generated("local"))),
                span: Span::generated("binary"),
            },
        };

        stmts.push(LoweredStmt::While {
            condition,
            body: while_body,
            span: Span::generated("while"),
        });

        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(result_expr),

            span: Span::generated("block"),
        })
    }

    /// Lower a callback method on a literal array receiver.
    #[allow(clippy::too_many_arguments)]
    fn lower_literal_array_callback_method(
        &mut self,
        method: &str,
        arr_temp: LocalId,
        receiver: LoweredExpr,
        _resolved_receiver: &ResolvedExpr,
        _args: &[ResolvedExpr],
        func_id: FuncId,
        captures: Vec<LocalId>,
        param_count: usize,
        init_expr: Option<LoweredExpr>,
        _span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        // Store receiver in a temp, then delegate to variable array handling
        let arr_ref = LoweredExpr::Local(arr_temp, Span::generated("local"));
        let mut stmts = vec![LoweredStmt::Let(
            arr_temp,
            receiver,
            Span::generated("let_stmt"),
        )];
        let inner = self.lower_variable_array_callback_method(
            method,
            arr_temp,
            arr_ref,
            func_id,
            captures,
            param_count,
            init_expr,
        )?;

        // Prepend the Let(arr_temp, receiver) before the inner Block's stmts
        // But the inner already returns a LoweredExpr::Block.
        // We should combine them.
        match inner {
            LoweredExpr::Block {
                stmts: inner_stmts,
                result,
                ..
            } => {
                stmts.extend(inner_stmts);
                Ok(LoweredExpr::Block {
                    stmts,
                    result,

                    span: Span::generated("block"),
                })
            }
            _ => Ok(inner),
        }
    }

    fn lower_array_foreach_callback(
        &mut self,
        receiver_local: LocalId,
        i: LocalId,
        func_id: FuncId,
        captures: &[LocalId],
        param_count: usize,
    ) -> Result<(Vec<LoweredStmt>, Vec<LoweredStmt>, LoweredExpr), Diagnostic> {
        let mut while_body = Vec::new();
        let arr_ref =
            || -> LoweredExpr { LoweredExpr::Local(receiver_local, Span::generated("local")) };

        let elem = self.alloc_temp();
        while_body.push(LoweredStmt::Let(
            elem,
            LoweredExpr::ArrayGet {
                arr: Box::new(arr_ref()),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),

                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        let call_args = {
            let explicit_args = vec![
                LoweredExpr::Local(elem, Span::generated("local")),
                LoweredExpr::Local(i, Span::generated("local")),
                arr_ref(),
            ];
            let mut call_args: Vec<LoweredExpr> =
                explicit_args.into_iter().take(param_count).collect();
            call_args.extend(
                captures
                    .iter()
                    .copied()
                    .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
            );
            LoweredExpr::Call {
                kind: FunctionCallKind::User(func_id),
                args: call_args,

                span: Span::generated("call"),
            }
        };

        while_body.push(LoweredStmt::Expr(call_args, Span::generated("expr_stmt")));
        while_body.push(LoweredStmt::Assign(
            i,
            LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Add,
                right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                span: Span::generated("binary"),
            },
            Span::generated("Assign"),
        ));

        let result_expr = LoweredExpr::Undefined(Span::generated("undef"));
        Ok((Vec::new(), while_body, result_expr))
    }

    fn lower_array_filter_callback(
        &mut self,
        receiver_local: LocalId,
        i: LocalId,
        func_id: FuncId,
        captures: &[LocalId],
        param_count: usize,
    ) -> Result<(Vec<LoweredStmt>, Vec<LoweredStmt>, LoweredExpr), Diagnostic> {
        let mut init_stmts = Vec::new();
        let mut while_body = Vec::new();
        let arr_ref =
            || -> LoweredExpr { LoweredExpr::Local(receiver_local, Span::generated("local")) };

        let result = self.alloc_temp();
        init_stmts.push(LoweredStmt::Let(
            result,
            LoweredExpr::ArrayNew {
                elements: vec![],

                span: Span::generated("array_new"),
            },
            Span::generated("let_stmt"),
        ));

        let elem = self.alloc_temp();
        while_body.push(LoweredStmt::Let(
            elem,
            LoweredExpr::ArrayGet {
                arr: Box::new(arr_ref()),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),

                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        let pred = self.alloc_temp();
        let call_args = {
            let explicit_args = vec![
                LoweredExpr::Local(elem, Span::generated("local")),
                LoweredExpr::Local(i, Span::generated("local")),
                arr_ref(),
            ];
            let mut call_args: Vec<LoweredExpr> =
                explicit_args.into_iter().take(param_count).collect();
            call_args.extend(
                captures
                    .iter()
                    .copied()
                    .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
            );
            LoweredExpr::Call {
                kind: FunctionCallKind::User(func_id),
                args: call_args,

                span: Span::generated("call"),
            }
        };
        while_body.push(LoweredStmt::Let(
            pred,
            call_args,
            Span::generated("let_stmt"),
        ));
        while_body.push(LoweredStmt::If {
            condition: LoweredExpr::Local(pred, Span::generated("local")),
            then_body: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ArrayPushGrow,
                    args: vec![
                        LoweredExpr::Local(result, Span::generated("local")),
                        LoweredExpr::Local(elem, Span::generated("local")),
                    ],

                    span: Span::generated("runtime_call"),
                },
                Span::generated("Expr"),
            )],
            else_body: vec![],
            span: Span::generated("If"),
        });
        while_body.push(LoweredStmt::Assign(
            i,
            LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Add,
                right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                span: Span::generated("binary"),
            },
            Span::generated("Assign"),
        ));

        let result_expr = LoweredExpr::Local(result, Span::generated("local"));
        Ok((init_stmts, while_body, result_expr))
    }

    fn lower_array_find_callback(
        &mut self,
        receiver_local: LocalId,
        i: LocalId,
        func_id: FuncId,
        captures: &[LocalId],
        param_count: usize,
        method: &str,
    ) -> Result<(Vec<LoweredStmt>, Vec<LoweredStmt>, LoweredExpr), Diagnostic> {
        let mut init_stmts = Vec::new();
        let mut while_body = Vec::new();
        let arr_ref =
            || -> LoweredExpr { LoweredExpr::Local(receiver_local, Span::generated("local")) };

        let found = self.alloc_temp();
        if method == "findIndex" {
            init_stmts.push(LoweredStmt::Let(
                found,
                LoweredExpr::Number(-1, Span::generated("num")),
                Span::generated("Let"),
            ));
        } else {
            init_stmts.push(LoweredStmt::Let(
                found,
                LoweredExpr::Undefined(Span::generated("undef")),
                Span::generated("Let"),
            ));
        }

        let elem = self.alloc_temp();
        while_body.push(LoweredStmt::Let(
            elem,
            LoweredExpr::ArrayGet {
                arr: Box::new(arr_ref()),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),

                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        let pred = self.alloc_temp();
        let call_args = {
            let explicit_args = vec![
                LoweredExpr::Local(elem, Span::generated("local")),
                LoweredExpr::Local(i, Span::generated("local")),
                arr_ref(),
            ];
            let mut call_args: Vec<LoweredExpr> =
                explicit_args.into_iter().take(param_count).collect();
            call_args.extend(
                captures
                    .iter()
                    .copied()
                    .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
            );
            LoweredExpr::Call {
                kind: FunctionCallKind::User(func_id),
                args: call_args,

                span: Span::generated("call"),
            }
        };
        while_body.push(LoweredStmt::Let(
            pred,
            call_args,
            Span::generated("let_stmt"),
        ));
        if method == "findIndex" {
            while_body.push(LoweredStmt::If {
                condition: LoweredExpr::Local(pred, Span::generated("local")),
                then_body: vec![
                    LoweredStmt::Assign(
                        found,
                        LoweredExpr::Local(i, Span::generated("local")),
                        Span::generated("Assign"),
                    ),
                    LoweredStmt::Break {
                        label: None,
                        span: Span::generated("brk"),
                    },
                ],
                else_body: vec![],

                span: Span::generated("if_stmt"),
            });
        } else {
            while_body.push(LoweredStmt::If {
                condition: LoweredExpr::Local(pred, Span::generated("local")),
                then_body: vec![LoweredStmt::Assign(
                    found,
                    LoweredExpr::Local(elem, Span::generated("local")),
                    Span::generated("Assign"),
                )],
                else_body: vec![],

                span: Span::generated("if_stmt"),
            });
        }
        while_body.push(LoweredStmt::Assign(
            i,
            LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Add,
                right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                span: Span::generated("binary"),
            },
            Span::generated("Assign"),
        ));

        let result_expr = LoweredExpr::Local(found, Span::generated("local"));
        Ok((init_stmts, while_body, result_expr))
    }

    fn lower_array_find_last_callback(
        &mut self,
        receiver_local: LocalId,
        i: LocalId,
        func_id: FuncId,
        captures: &[LocalId],
        param_count: usize,
        method: &str,
    ) -> Result<(Vec<LoweredStmt>, Vec<LoweredStmt>, LoweredExpr), Diagnostic> {
        let mut init_stmts = Vec::new();
        let mut while_body = Vec::new();
        let arr_ref =
            || -> LoweredExpr { LoweredExpr::Local(receiver_local, Span::generated("local")) };

        let found = self.alloc_temp();
        if method == "findLastIndex" {
            init_stmts.push(LoweredStmt::Let(
                found,
                LoweredExpr::Number(-1, Span::generated("num")),
                Span::generated("Let"),
            ));
        } else {
            init_stmts.push(LoweredStmt::Let(
                found,
                LoweredExpr::Undefined(Span::generated("undef")),
                Span::generated("Let"),
            ));
        }

        let elem = self.alloc_temp();
        while_body.push(LoweredStmt::Let(
            elem,
            LoweredExpr::ArrayGet {
                arr: Box::new(arr_ref()),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),

                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        let pred = self.alloc_temp();
        let call_args = {
            let explicit_args = vec![
                LoweredExpr::Local(elem, Span::generated("local")),
                LoweredExpr::Local(i, Span::generated("local")),
                arr_ref(),
            ];
            let mut call_args: Vec<LoweredExpr> =
                explicit_args.into_iter().take(param_count).collect();
            call_args.extend(
                captures
                    .iter()
                    .copied()
                    .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
            );
            LoweredExpr::Call {
                kind: FunctionCallKind::User(func_id),
                args: call_args,

                span: Span::generated("call"),
            }
        };
        while_body.push(LoweredStmt::Let(
            pred,
            call_args,
            Span::generated("let_stmt"),
        ));
        if method == "findLastIndex" {
            while_body.push(LoweredStmt::If {
                condition: LoweredExpr::Local(pred, Span::generated("local")),
                then_body: vec![
                    LoweredStmt::Assign(
                        found,
                        LoweredExpr::Local(i, Span::generated("local")),
                        Span::generated("Assign"),
                    ),
                    LoweredStmt::Break {
                        label: None,
                        span: Span::generated("break"),
                    },
                ],
                else_body: vec![],
                span: Span::generated("If"),
            });
        } else {
            while_body.push(LoweredStmt::If {
                condition: LoweredExpr::Local(pred, Span::generated("local")),
                then_body: vec![
                    LoweredStmt::Assign(
                        found,
                        LoweredExpr::Local(elem, Span::generated("local")),
                        Span::generated("Assign"),
                    ),
                    LoweredStmt::Break {
                        label: None,
                        span: Span::generated("break"),
                    },
                ],
                else_body: vec![],
                span: Span::generated("If"),
            });
        }
        while_body.push(LoweredStmt::Assign(
            i,
            LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Subtract,
                right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                span: Span::generated("binary"),
            },
            Span::generated("Assign"),
        ));

        let result_expr = LoweredExpr::Local(found, Span::generated("local"));
        Ok((init_stmts, while_body, result_expr))
    }

    fn lower_array_some_every_callback(
        &mut self,
        receiver_local: LocalId,
        i: LocalId,
        func_id: FuncId,
        captures: &[LocalId],
        param_count: usize,
        is_some: bool,
    ) -> Result<(Vec<LoweredStmt>, Vec<LoweredStmt>, LoweredExpr), Diagnostic> {
        let mut init_stmts = Vec::new();
        let mut while_body = Vec::new();
        let arr_ref =
            || -> LoweredExpr { LoweredExpr::Local(receiver_local, Span::generated("local")) };

        let acc = self.alloc_temp();
        init_stmts.push(LoweredStmt::Let(
            acc,
            LoweredExpr::Bool(!is_some, Span::generated("bool")),
            Span::generated("Let"),
        ));

        let elem = self.alloc_temp();
        while_body.push(LoweredStmt::Let(
            elem,
            LoweredExpr::ArrayGet {
                arr: Box::new(arr_ref()),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),

                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        let pred = self.alloc_temp();
        let call_args = {
            let explicit_args = vec![
                LoweredExpr::Local(elem, Span::generated("local")),
                LoweredExpr::Local(i, Span::generated("local")),
                arr_ref(),
            ];
            let mut call_args: Vec<LoweredExpr> =
                explicit_args.into_iter().take(param_count).collect();
            call_args.extend(
                captures
                    .iter()
                    .copied()
                    .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
            );
            LoweredExpr::Call {
                kind: FunctionCallKind::User(func_id),
                args: call_args,

                span: Span::generated("call"),
            }
        };
        while_body.push(LoweredStmt::Let(
            pred,
            call_args,
            Span::generated("let_stmt"),
        ));
        if is_some {
            while_body.push(LoweredStmt::If {
                condition: LoweredExpr::Local(pred, Span::generated("local")),
                then_body: vec![LoweredStmt::Assign(
                    acc,
                    LoweredExpr::Bool(true, Span::generated("bool")),
                    Span::generated("Assign"),
                )],
                else_body: vec![],

                span: Span::generated("if_stmt"),
            });
        } else {
            while_body.push(LoweredStmt::If {
                condition: LoweredExpr::Unary {
                    op: LoweredUnaryOp::Not,
                    expr: Box::new(LoweredExpr::Local(pred, Span::generated("local"))),

                    span: Span::generated("unary"),
                },
                then_body: vec![LoweredStmt::Assign(
                    acc,
                    LoweredExpr::Bool(false, Span::generated("bool")),
                    Span::generated("Assign"),
                )],
                else_body: vec![],
                span: Span::generated("if_stmt"),
            });
        }
        while_body.push(LoweredStmt::Assign(
            i,
            LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Add,
                right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                span: Span::generated("binary"),
            },
            Span::generated("Assign"),
        ));

        let result_expr = LoweredExpr::Local(acc, Span::generated("local"));
        Ok((init_stmts, while_body, result_expr))
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_array_reduce_callback(
        &mut self,
        receiver_local: LocalId,
        i: LocalId,
        func_id: FuncId,
        captures: &[LocalId],
        param_count: usize,
        method: &str,
        init_expr: Option<LoweredExpr>,
    ) -> Result<(Vec<LoweredStmt>, Vec<LoweredStmt>, LoweredExpr), Diagnostic> {
        let mut init_stmts = Vec::new();
        let mut while_body = Vec::new();
        let arr_ref =
            || -> LoweredExpr { LoweredExpr::Local(receiver_local, Span::generated("local")) };

        let Some(init_expr) = init_expr else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-270: Array.prototype.reduce without initialValue is not yet supported"
                        .to_owned(),
                span: None,

                phase: None,
            });
        };
        let acc = self.alloc_temp();
        init_stmts.push(LoweredStmt::Let(
            acc,
            init_expr,
            Span::generated("let_stmt"),
        ));

        let elem = self.alloc_temp();
        while_body.push(LoweredStmt::Let(
            elem,
            LoweredExpr::ArrayGet {
                arr: Box::new(arr_ref()),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),

                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        // Reduce callback args: (acc, elem, i, arr)
        let reduce_explicit = vec![
            LoweredExpr::Local(acc, Span::generated("local")),
            LoweredExpr::Local(elem, Span::generated("local")),
            LoweredExpr::Local(i, Span::generated("local")),
            arr_ref(),
        ];
        let mut reduce_call_args: Vec<LoweredExpr> =
            reduce_explicit.into_iter().take(param_count).collect();
        reduce_call_args.extend(
            captures
                .iter()
                .copied()
                .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
        );
        while_body.push(LoweredStmt::Assign(
            acc,
            LoweredExpr::Call {
                kind: FunctionCallKind::User(func_id),
                args: reduce_call_args,

                span: Span::generated("call"),
            },
            Span::generated("Assign"),
        ));
        if method == "reduceRight" {
            while_body.push(LoweredStmt::Assign(
                i,
                LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                    op: LoweredBinaryOp::Subtract,
                    right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                    span: Span::generated("binary"),
                },
                Span::generated("Assign"),
            ));
        } else {
            while_body.push(LoweredStmt::Assign(
                i,
                LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                    op: LoweredBinaryOp::Add,
                    right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                    span: Span::generated("binary"),
                },
                Span::generated("Assign"),
            ));
        }

        let result_expr = LoweredExpr::Local(acc, Span::generated("local"));
        Ok((init_stmts, while_body, result_expr))
    }

    fn lower_array_flatmap_callback(
        &mut self,
        receiver_local: LocalId,
        i: LocalId,
        func_id: FuncId,
        captures: &[LocalId],
        param_count: usize,
    ) -> Result<(Vec<LoweredStmt>, Vec<LoweredStmt>, LoweredExpr), Diagnostic> {
        let mut init_stmts = Vec::new();
        let mut while_body = Vec::new();
        let arr_ref =
            || -> LoweredExpr { LoweredExpr::Local(receiver_local, Span::generated("local")) };

        let result = self.alloc_temp();
        init_stmts.push(LoweredStmt::Let(
            result,
            LoweredExpr::ArrayNew {
                elements: vec![],
                span: Span::generated("array_new"),
            },
            Span::generated("Let"),
        ));
        let elem = self.alloc_temp();
        while_body.push(LoweredStmt::Let(
            elem,
            LoweredExpr::ArrayGet {
                arr: Box::new(arr_ref()),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),

                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));
        let mapped = self.alloc_temp();
        let call_args = {
            let explicit_args = vec![
                LoweredExpr::Local(elem, Span::generated("local")),
                LoweredExpr::Local(i, Span::generated("local")),
                arr_ref(),
            ];
            let mut call_args: Vec<LoweredExpr> =
                explicit_args.into_iter().take(param_count).collect();
            call_args.extend(
                captures
                    .iter()
                    .copied()
                    .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
            );
            LoweredExpr::Call {
                kind: FunctionCallKind::User(func_id),
                args: call_args,

                span: Span::generated("call"),
            }
        };
        while_body.push(LoweredStmt::Let(
            mapped,
            call_args,
            Span::generated("let_stmt"),
        ));
        // Push or spread the result (handles array vs non-array)
        while_body.push(LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayPushOrSpread,
                args: vec![
                    LoweredExpr::Local(result, Span::generated("local")),
                    LoweredExpr::Local(mapped, Span::generated("local")),
                ],

                span: Span::generated("runtime_call"),
            },
            Span::generated("Expr"),
        ));
        while_body.push(LoweredStmt::Assign(
            i,
            LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Add,
                right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                span: Span::generated("binary"),
            },
            Span::generated("Assign"),
        ));
        let result_expr = LoweredExpr::Local(result, Span::generated("local"));
        Ok((init_stmts, while_body, result_expr))
    }

    fn lower_array_map_callback(
        &mut self,
        receiver_local: LocalId,
        i: LocalId,
        func_id: FuncId,
        captures: &[LocalId],
        param_count: usize,
    ) -> Result<(Vec<LoweredStmt>, Vec<LoweredStmt>, LoweredExpr), Diagnostic> {
        let mut init_stmts = Vec::new();
        let mut while_body = Vec::new();
        let arr_ref =
            || -> LoweredExpr { LoweredExpr::Local(receiver_local, Span::generated("local")) };

        let result = self.alloc_temp();
        init_stmts.push(LoweredStmt::Let(
            result,
            LoweredExpr::ArrayNew {
                elements: vec![],
                span: Span::generated("array_new"),
            },
            Span::generated("Let"),
        ));

        let elem = self.alloc_temp();
        while_body.push(LoweredStmt::Let(
            elem,
            LoweredExpr::ArrayGet {
                arr: Box::new(arr_ref()),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        let mapped = self.alloc_temp();
        let call_args = {
            let explicit_args = vec![
                LoweredExpr::Local(elem, Span::generated("local")),
                LoweredExpr::Local(i, Span::generated("local")),
                arr_ref(),
            ];
            let mut call_args: Vec<LoweredExpr> =
                explicit_args.into_iter().take(param_count).collect();
            call_args.extend(
                captures
                    .iter()
                    .copied()
                    .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
            );
            LoweredExpr::Call {
                kind: FunctionCallKind::User(func_id),
                args: call_args,
                span: Span::generated("call"),
            }
        };
        while_body.push(LoweredStmt::Let(
            mapped,
            call_args,
            Span::generated("let_stmt"),
        ));
        while_body.push(LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayPushGrow,
                args: vec![
                    LoweredExpr::Local(result, Span::generated("local")),
                    LoweredExpr::Local(mapped, Span::generated("local")),
                ],
                span: Span::generated("runtime_call"),
            },
            Span::generated("Expr"),
        ));
        while_body.push(LoweredStmt::Assign(
            i,
            LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Add,
                right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),

                span: Span::generated("binary"),
            },
            Span::generated("Assign"),
        ));

        let result_expr = LoweredExpr::Local(result, Span::generated("local"));
        Ok((init_stmts, while_body, result_expr))
    }
}

fn dense_array_like_object_elements(props: &[(String, ResolvedExpr)]) -> Option<Vec<ResolvedExpr>> {
    let len = props.iter().find_map(|(key, value)| {
        if key == "length" {
            if let ResolvedExpr::Number(len) = value {
                usize::try_from(*len).ok()
            } else {
                None
            }
        } else {
            None
        }
    })?;
    let mut elements = Vec::with_capacity(len);
    for index in 0..len {
        let key = index.to_string();
        let value = props
            .iter()
            .find_map(|(prop_key, prop_value)| (prop_key == &key).then(|| prop_value.clone()))?;
        elements.push(value);
    }
    Some(elements)
}
