use crate::builtin_resolved::ResolvedArrayElement;
use super::*;

impl<'a> Resolver<'a> {
    pub(super) fn lower_call_args(&mut self, args: &[ResolvedExpr]) -> Result<Vec<LoweredExpr>, Diagnostic> {
        let mut lowered_args = Vec::new();
        for arg in args {
            match arg {
                ResolvedExpr::Spread(spread_expr) => {
                    if let ResolvedExpr::Array(elements) = spread_expr.as_ref() {
                        for elem in elements {
                            match elem {
                                ResolvedArrayElement::Present(expr) => {
                                    lowered_args.push(self.lower_expr(expr)?);
                                }
                                ResolvedArrayElement::Hole => {
                                    lowered_args.push(LoweredExpr::Undefined);
                                }
                            }
                        }
                    } else if let Some(value) = self.static_string_spread_value(spread_expr.as_ref()) {
                        lowered_args.extend(Self::lower_ascii_string_spread_chars(&value)?);
                    } else if self.is_generator_call_spread_operand(spread_expr.as_ref()) {
                        return Err(Self::unsupported_generator_spread_diagnostic());
                    } else if self.resolved_expr_has_symbol_iterator_property(spread_expr.as_ref()) {
                        return Err(Self::unsupported_symbol_iterator_spread_diagnostic());
                    } else if self.is_known_map_local_spread_operand(spread_expr.as_ref()) {
                        return Err(Self::unsupported_map_spread_diagnostic());
                    } else {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message:
                                "issue-274: spread arguments are only supported for literal arrays and ASCII literal-derived strings in this milestone"
                                    .to_owned(),
                            span: None,
                        });
                    }
                }
                _ => lowered_args.push(self.lower_expr(arg)?),
            }
        }
        Ok(lowered_args)
    }

    pub(super) fn static_string_spread_value(&self, spread_expr: &ResolvedExpr) -> Option<String> {
        self.resolved_expr_static_string_value(spread_expr)
    }

    pub(super) fn lower_ascii_string_spread_chars(value: &str) -> Result<Vec<LoweredExpr>, Diagnostic> {
        if !value.is_ascii() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-274: string spread is currently limited to ASCII literal-derived strings"
                        .to_owned(),
                span: None,
            });
        }
        Ok(value
            .chars()
            .map(|ch| LoweredExpr::String(ch.to_string()))
            .collect())
    }

    pub(super) fn lower_array_literal(
        &mut self,
        elements: &[ResolvedArrayElement],
    ) -> Result<LoweredExpr, Diagnostic> {
        if elements.iter().any(|element| matches!(element, ResolvedArrayElement::Hole)) {
            let mut slots = Vec::with_capacity(elements.len());
            for element in elements {
                match element {
                    ResolvedArrayElement::Present(expr) => {
                        slots.push(LoweredArraySlot::Present(self.lower_expr(expr)?));
                    }
                    ResolvedArrayElement::Hole => slots.push(LoweredArraySlot::Hole),
                }
            }
            return Ok(LoweredExpr::ArrayNewSparse { slots });
        }
        if !elements.iter().any(|element| {
            matches!(
                element,
                ResolvedArrayElement::Present(ResolvedExpr::Spread(spread_expr))
                    if self.is_known_set_local_spread_operand(spread_expr.as_ref())
                        || self.is_known_dense_array_local_spread_operand(spread_expr.as_ref())
            )
        }) {
            let lowered = self.lower_array_literal_elements(elements)?;
            return Ok(LoweredExpr::ArrayNew { elements: lowered });
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

                    if let Some(value) = self.static_string_spread_value(spread_expr.as_ref()) {
                        pending_dense.extend(Self::lower_ascii_string_spread_chars(&value)?);
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

                    if self.is_generator_call_spread_operand(spread_expr.as_ref()) {
                        return Err(Self::unsupported_generator_spread_diagnostic());
                    }

                    if self.resolved_expr_has_symbol_iterator_property(spread_expr.as_ref()) {
                        return Err(Self::unsupported_symbol_iterator_spread_diagnostic());
                    }

                    if self.is_known_map_local_spread_operand(spread_expr.as_ref()) {
                        return Err(Self::unsupported_map_spread_diagnostic());
                    }

                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "issue-274: array literal spread is only supported for literal arrays, known dense array locals, and known Set locals in this milestone"
                                .to_owned(),
                        span: None,
                    });
                }
                ResolvedArrayElement::Present(expr) => pending_dense.push(self.lower_expr(expr)?),
                ResolvedArrayElement::Hole => pending_dense.push(LoweredExpr::Undefined),
            }
        }

        Self::flush_array_segment(&mut segments, &mut pending_dense);

        let mut iter = segments.into_iter();
        let Some(mut combined) = iter.next() else {
            return Ok(LoweredExpr::ArrayNew { elements: vec![] });
        };
        for segment in iter {
            combined = LoweredExpr::RuntimeCall {
                runtime_fn: "ArrayConcat".to_owned(),
                args: vec![combined, segment],
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
                let Some(elements) = self.static_function_array_like_elements(name) else {
                    if is_identity_arrow_callback(map_args) {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "ArrayMapArrayLikeIdentity".to_owned(),
                            args: vec![self.lower_expr(receiver)?],
                        });
                    }
                    if is_number_double_arrow_callback(map_args) {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "ArrayMapArrayLikeDouble".to_owned(),
                            args: vec![self.lower_expr(receiver)?],
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
                runtime_fn: "ArrayMapArrayLikeIdentity".to_owned(),
                args: vec![self.lower_expr(receiver)?],
            }),
            _ if is_number_double_arrow_callback(map_args) => Ok(LoweredExpr::RuntimeCall {
                runtime_fn: "ArrayMapArrayLikeDouble".to_owned(),
                args: vec![self.lower_expr(receiver)?],
            }),
            _ => Err(unsupported_array_map_diagnostic(Some(span))),
        }
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
        let is_sparse = elements.iter().any(|element| matches!(element, ResolvedArrayElement::Hole));
        let mut mapped = Vec::with_capacity(elements.len());
        for (index, element) in elements.iter().enumerate() {
            match element {
                ResolvedArrayElement::Present(expr) => {
                    let element = self.lower_expr(expr)?;
                    mapped.push(LoweredArraySlot::Present(self.lower_array_map_callback_call(
                        callback,
                        args.get(1),
                        element,
                        index,
                        array_expr,
                        span,
                    )?));
                }
                ResolvedArrayElement::Hole => mapped.push(LoweredArraySlot::Hole),
            }
        }
        if is_sparse {
            Ok(LoweredExpr::ArrayNewSparse { slots: mapped })
        } else {
            Ok(LoweredExpr::ArrayNew {
                elements: mapped
                    .into_iter()
                    .map(|slot| match slot {
                        LoweredArraySlot::Present(expr) => expr,
                        LoweredArraySlot::Hole => LoweredExpr::Undefined,
                    })
                    .collect(),
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
            ResolvedExpr::ArrowFn { params, body } => {
                if params.len() > 3 {
                    return Err(unsupported_array_map_diagnostic(Some(span)));
                }
                let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = self.lower_arrow_fn(params, body)?
                else {
                    return Err(unsupported_array_map_diagnostic(Some(span)));
                };
                let mut explicit_args = vec![element, LoweredExpr::Number(index as i32)];
                explicit_args.push(self.lower_expr(array_expr)?);
                let mut call_args = explicit_args
                    .into_iter()
                    .take(params.len())
                    .collect::<Vec<_>>();
                call_args.extend(captures.iter().copied().map(LoweredExpr::Local));
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(func_id),
                    args: call_args,
                })
            }
            ResolvedExpr::FunctionExpr { name, params, body } => {
                self.lower_array_map_function_expr_callback_call(
                    name, params, body, this_arg, element, index, array_expr, span,
                )
            }
            ResolvedExpr::Ident(name) => {
                let func_id = self.resolve_func(name)?;
                let receiver = match this_arg {
                    Some(expr) => self.lower_expr(expr)?,
                    None => LoweredExpr::Undefined,
                };
                let signature = self
                    .function_signatures
                    .get(&func_id)
                    .copied()
                    .unwrap_or_default();
                let mut explicit_args = vec![element, LoweredExpr::Number(index as i32)];
                explicit_args.push(self.lower_expr(array_expr)?);
                let argument_props = explicit_args
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(index, arg)| (index.to_string(), arg))
                    .chain(std::iter::once((
                        "length".to_owned(),
                        LoweredExpr::Number(explicit_args.len() as i32),
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
                        call_args.push(LoweredExpr::Undefined);
                    }
                }
                if signature.needs_arguments {
                    call_args.push(LoweredExpr::ObjectNew {
                        props: argument_props,
                    });
                }
                self.append_function_captures(func_id, &mut call_args)?;
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(func_id),
                    args: call_args,
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
            None => LoweredExpr::Undefined,
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
            .any(|capture| !self.env_cell_names.contains(capture))
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` mutates a captured outer local; mutable closure environments require heap environment support"
                ),
                span: None,
            });
        }
        let captures = capture_names
            .iter()
            .map(|capture| self.resolve_local(capture))
            .collect::<Result<Vec<_>, _>>()?;
        let mut lowered_params = params.to_vec();
        lowered_params.extend(
            capture_names
                .iter()
                .map(|capture| ResolvedParam {
                    name: capture.clone(),
                    default: None,
                    is_rest: false,
                    span: None,
                }),
        );

        let func_id = FuncId(self.next_func_id);
        self.next_func_id += 1;
        let self_closure = (!name.is_empty())
            .then_some(SelfClosureOptions {
                name,
                func_id,
                capture_names: &capture_names,
            })
            .filter(|_| !self.env_cell_names.contains(name));
        let mut function_signatures = self.function_signatures.clone();
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
            self.function_ids,
            &function_signatures,
            self.function_captures,
            self.function_mutable_captures,
            self.class_method_captures,
            self.class_method_mutable_captures,
            &self.env_cell_names,
            &self.heap_closure_names,
            self.class_parents.clone(),
            self.class_private_fields.clone(),
            self.class_static_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.current_class.as_deref(),
                in_constructor: false,
                next_func_id: self.next_func_id,
                self_closure,
            },
        )?;
        self.next_func_id = lowered.next_func_id;
        self.generated_functions.push(lowered.function);
        self.generated_functions.extend(lowered.generated_functions);

        let explicit_args = [
            element,
            LoweredExpr::Number(index as i32),
            self.lower_expr(array_expr)?,
        ];
        let mut call_args = vec![receiver];
        call_args.extend(explicit_args.into_iter().take(params.len()));
        for _ in call_args.len()..=params.len() {
            call_args.push(LoweredExpr::Undefined);
        }
        call_args.extend(captures.into_iter().map(LoweredExpr::Local));
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: call_args,
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
                    } else if let Some(value) = self.static_string_spread_value(spread_expr.as_ref()) {
                        lowered.extend(Self::lower_ascii_string_spread_chars(&value)?);
                    } else if self.is_generator_call_spread_operand(spread_expr.as_ref()) {
                        return Err(Self::unsupported_generator_spread_diagnostic());
                    } else if self.resolved_expr_has_symbol_iterator_property(spread_expr.as_ref()) {
                        return Err(Self::unsupported_symbol_iterator_spread_diagnostic());
                    } else if self.is_known_map_local_spread_operand(spread_expr.as_ref()) {
                        return Err(Self::unsupported_map_spread_diagnostic());
                    } else {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message:
                                "issue-274: array literal spread is only supported for literal arrays and ASCII literal-derived strings in this milestone"
                                    .to_owned(),
                            span: None,
                        });
                    }
                }
                ResolvedArrayElement::Present(expr) => lowered.push(self.lower_expr(expr)?),
                ResolvedArrayElement::Hole => lowered.push(LoweredExpr::Undefined),
            }
        }
        Ok(lowered)
    }

    pub(super) fn flush_array_segment(segments: &mut Vec<LoweredExpr>, pending_dense: &mut Vec<LoweredExpr>) {
        if pending_dense.is_empty() {
            return;
        }
        segments.push(LoweredExpr::ArrayNew {
            elements: std::mem::take(pending_dense),
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
        if self.array_locals.contains(&local_id) {
            return Ok(Some(LoweredExpr::RuntimeCall {
                runtime_fn: "ArrayConcat".to_owned(),
                args: vec![
                    LoweredExpr::ArrayNew { elements: vec![] },
                    LoweredExpr::Local(local_id),
                ],
            }));
        }
        Ok(None)
    }

    pub(super) fn is_known_dense_array_local_spread_operand(&self, spread_expr: &ResolvedExpr) -> bool {
        let ResolvedExpr::Ident(name) = spread_expr else {
            return false;
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return false;
        };
        self.array_locals.contains(&local_id)
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
            .local_classes
            .get(&local_id)
            .is_some_and(|class_name| class_name == "Set")
        {
            return Ok(Some(LoweredExpr::RuntimeCall {
                runtime_fn: "SetValuesArray".to_owned(),
                args: vec![LoweredExpr::Local(local_id)],
            }));
        }
        Ok(None)
    }

    pub(super) fn is_known_set_local_spread_operand(&self, spread_expr: &ResolvedExpr) -> bool {
        let ResolvedExpr::Ident(name) = spread_expr else {
            return false;
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return false;
        };
        self.local_classes
            .get(&local_id)
            .is_some_and(|class_name| class_name == "Set")
    }

    pub(super) fn is_known_map_local_spread_operand(&self, spread_expr: &ResolvedExpr) -> bool {
        let ResolvedExpr::Ident(name) = spread_expr else {
            return false;
        };
        let Ok(local_id) = self.resolve_local(name) else {
            return false;
        };
        self.local_classes
            .get(&local_id)
            .is_some_and(|class_name| class_name == "Map")
    }

    pub(super) fn lower_object_literal_props(
        &mut self,
        props: &[(String, ResolvedExpr)],
    ) -> Result<Vec<(String, LoweredExpr)>, Diagnostic> {
        let mut lowered = Vec::new();
        for (key, value) in props {
            if key == OBJECT_SPREAD_SENTINEL {
                let spread_props = self.static_object_literal_spread_props(value).ok_or_else(|| {
                    Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "issue-274: object literal spread is only supported for object literals and known static object-literal locals in this milestone"
                                .to_owned(),
                        span: None,
                    }
                })?;
                lowered.extend(self.lower_object_literal_props(&spread_props)?);
                continue;
            }
            if self.is_function_identifier(value) {
                continue;
            }
            lowered.push((key.clone(), self.lower_expr(value)?));
        }
        Ok(lowered)
    }

    pub(super) fn lower_object_literal_expr(
        &mut self,
        props: &[(String, ResolvedExpr)],
    ) -> Result<LoweredExpr, Diagnostic> {
        let mut result: Option<LoweredExpr> = None;
        let mut pending = Vec::new();

        for (key, value) in props {
            if key == OBJECT_SPREAD_SENTINEL {
                if let Some(spread_props) = self.static_object_literal_spread_props(value) {
                    pending.extend(self.lower_object_literal_props(&spread_props)?);
                    continue;
                }

                let target = result
                    .take()
                    .unwrap_or_else(|| LoweredExpr::ObjectNew { props: Vec::new() });
                let target = if pending.is_empty() {
                    target
                } else {
                    LoweredExpr::RuntimeCall {
                        runtime_fn: "ObjectSpread".to_owned(),
                        args: vec![
                            target,
                            LoweredExpr::ObjectNew {
                                props: std::mem::take(&mut pending),
                            },
                        ],
                    }
                };
                result = Some(LoweredExpr::RuntimeCall {
                    runtime_fn: "ObjectSpread".to_owned(),
                    args: vec![target, self.lower_expr(value)?],
                });
                continue;
            }

            if self.is_function_identifier(value) {
                continue;
            }
            pending.push((key.clone(), self.lower_expr(value)?));
        }

        let target = result.unwrap_or_else(|| LoweredExpr::ObjectNew { props: Vec::new() });
        if pending.is_empty() {
            Ok(target)
        } else if matches!(target, LoweredExpr::ObjectNew { ref props } if props.is_empty()) {
            Ok(LoweredExpr::ObjectNew { props: pending })
        } else {
            Ok(LoweredExpr::RuntimeCall {
                runtime_fn: "ObjectSpread".to_owned(),
                args: vec![target, LoweredExpr::ObjectNew { props: pending }],
            })
        }
    }

    pub(super) fn static_object_literal_spread_props(
        &self,
        value: &ResolvedExpr,
    ) -> Option<Vec<(String, ResolvedExpr)>> {
        match value {
            ResolvedExpr::Object(spread_props) => Some(spread_props.clone()),
            ResolvedExpr::Ident(name) => {
                let local_id = self.resolve_local(name).ok()?;
                if self.env_cell_locals.contains(&local_id) {
                    return None;
                }
                self.static_object_literal_locals.get(&local_id).cloned()
            }
            _ => None,
        }
    }

    pub(super) fn lower_set_prototype_add_assignment_value(
        &mut self,
        value: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let ResolvedExpr::Ident(name) = value
            && let Ok(func_id) = self.resolve_func(name)
        {
            return Ok(LoweredExpr::Number(func_id.0 as i32));
        }
        self.lower_expr(value)
    }

    pub(super) fn lower_native_set_add_call(
        &mut self,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if args.len() != 2 {
            return Err(Diagnostic {
                code: DiagCode::ArityMismatch,
                message: format!(
                    "Set.prototype.add.call expects receiver and value arguments, got {}",
                    args.len()
                ),
                span: Some(span),
            });
        }
        Ok(LoweredExpr::RuntimeCall {
            runtime_fn: "SetAdd".to_owned(),
            args: vec![self.lower_expr(&args[0])?, self.lower_expr(&args[1])?],
        })
    }

    pub(super) fn lower_binding_pattern_declarations(
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
                    statements.extend(self.lower_object_binding_declaration(
                        binding, bindings, &value, source,
                    )?);
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
                runtime_fn: "ArraySlice".to_owned(),
                args: vec![
                    value.clone(),
                    LoweredExpr::Number(binding.index as i32),
                    LoweredExpr::GetLength(Box::new(value.clone())),
                ],
            }
        } else {
            LoweredExpr::Index {
                object: Box::new(value.clone()),
                index: Box::new(LoweredExpr::Number(binding.index as i32)),
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
            return Ok(vec![LoweredStmt::Let(local_id, element_value)]);
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
        let property_value = LoweredExpr::PropertyGet {
            obj: Box::new(value.clone()),
            key: binding.key.clone(),
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
            });
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
                    },
                )
            })
            .collect();
        Ok(vec![LoweredStmt::Let(
            local_id,
            LoweredExpr::ObjectNew { props: rest_props },
        )])
    }

    pub(super) fn lower_binding_declaration_with_default(
        &mut self,
        local_id: LocalId,
        value: LoweredExpr,
        default: Option<&BindingDefault>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let Some(default) = default else {
            return Ok(vec![LoweredStmt::Let(local_id, value)]);
        };
        let temp_id = self.alloc_temp();
        Ok(vec![
            LoweredStmt::Let(temp_id, value),
            LoweredStmt::Let(local_id, LoweredExpr::Local(temp_id)),
            LoweredStmt::If {
                condition: LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(temp_id)),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined),
                },
                then_body: vec![LoweredStmt::Assign(
                    local_id,
                    lowered_binding_default(default),
                )],
                else_body: vec![],
            },
        ])
    }

    pub(super) fn lower_optional_call(
        &mut self,
        callee: &ResolvedExpr,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let func_name = match callee {
            ResolvedExpr::Ident(name) => name,
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-253: optional calls are currently supported only for identifier callees"
                            .to_owned(),
                    span: Some(span),
                });
            }
        };

        if let Ok(local_id) = self.resolve_local(func_name) {
            if self.nullish_locals.contains(&local_id) {
                return Ok(LoweredExpr::Undefined);
            }

            if let Some(closure) = self.arrow_locals.get(&local_id).cloned() {
                let mut lowered_args = self.lower_call_args(args)?;
                lowered_args.extend(closure.captures.iter().copied().map(LoweredExpr::Local));
                return Ok(LoweredExpr::OptionalCall {
                    callee: Box::new(LoweredExpr::Local(local_id)),
                    call: Box::new(LoweredExpr::Call {
                        kind: FunctionCallKind::User(closure.func_id),
                        args: lowered_args,
                    }),
                });
            }

            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-253: optional call `{func_name}?.(...)` is supported only for known functions or nullish locals"
                ),
                span: Some(span),
            });
        }

        let func_id = self.resolve_func(func_name)?;
        if self
            .function_signatures
            .get(&func_id)
            .is_some_and(|signature| signature.needs_receiver)
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062d: optional direct call `{func_name}?.(...)` cannot bind a supported receiver for `this`; call through a supported receiver object"
                ),
                span: Some(span),
            });
        }
        let lowered_args = self.lower_function_call_args(func_id, LoweredExpr::Undefined, args)?;
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: lowered_args,
        })
    }

    pub(super) fn lower_function_call_args(
        &mut self,
        func_id: FuncId,
        receiver: LoweredExpr,
        args: &[ResolvedExpr],
    ) -> Result<Vec<LoweredExpr>, Diagnostic> {
        let signature = self
            .function_signatures
            .get(&func_id)
            .copied()
            .unwrap_or_default();
        let explicit_args = if !signature.has_rest && !signature.needs_arguments {
            if let Some(local_id) = self.single_dense_array_local_spread_arg(args) {
                (0..signature.explicit_params)
                    .map(|index| LoweredExpr::ArrayGet {
                        arr: Box::new(LoweredExpr::Local(local_id)),
                        index: Box::new(LoweredExpr::Number(index as i32)),
                    })
                    .collect()
            } else if let Some(local_id) = self.single_set_local_spread_arg(args) {
                (0..signature.explicit_params)
                    .map(|index| LoweredExpr::ArrayGet {
                        arr: Box::new(LoweredExpr::RuntimeCall {
                            runtime_fn: "SetValuesArray".to_owned(),
                            args: vec![LoweredExpr::Local(local_id)],
                        }),
                        index: Box::new(LoweredExpr::Number(index as i32)),
                    })
                    .collect()
            } else {
                self.lower_call_args(args)?
            }
        } else {
            self.lower_call_args(args)?
        };
        let mut lowered_args = Vec::new();

        if signature.needs_receiver {
            lowered_args.push(receiver);
        }

        if signature.has_rest {
            lowered_args.extend(explicit_args.iter().cloned());
        } else {
            lowered_args.extend(explicit_args.iter().take(signature.explicit_params).cloned());
            for _ in explicit_args.len()..signature.explicit_params {
                lowered_args.push(LoweredExpr::Undefined);
            }
        }

        if signature.needs_arguments {
            let argument_count = explicit_args.len();
            let mut props = explicit_args
                .into_iter()
                .enumerate()
                .map(|(index, arg)| (index.to_string(), arg))
                .collect::<Vec<_>>();
            props.push((
                "length".to_owned(),
                LoweredExpr::Number(argument_count as i32),
            ));
            lowered_args.push(LoweredExpr::ObjectNew { props });
        }

        self.append_function_captures(func_id, &mut lowered_args)?;

        Ok(lowered_args)
    }

    pub(super) fn single_dense_array_local_spread_arg(&self, args: &[ResolvedExpr]) -> Option<LocalId> {
        let [ResolvedExpr::Spread(spread_expr)] = args else {
            return None;
        };
        let ResolvedExpr::Ident(name) = spread_expr.as_ref() else {
            return None;
        };
        let local_id = self.resolve_local(name).ok()?;
        if self.array_locals.contains(&local_id) && !self.env_cell_locals.contains(&local_id) {
            Some(local_id)
        } else {
            None
        }
    }

    pub(super) fn single_set_local_spread_arg(&self, args: &[ResolvedExpr]) -> Option<LocalId> {
        let [ResolvedExpr::Spread(spread_expr)] = args else {
            return None;
        };
        let ResolvedExpr::Ident(name) = spread_expr.as_ref() else {
            return None;
        };
        let local_id = self.resolve_local(name).ok()?;
        if self.env_cell_locals.contains(&local_id) {
            return None;
        }
        self.local_classes
            .get(&local_id)
            .is_some_and(|class_name| class_name == "Set")
            .then_some(local_id)
    }

    pub(super) fn append_class_method_captures(
        &self,
        method_id: FuncId,
        lowered_args: &mut Vec<LoweredExpr>,
    ) -> Result<(), Diagnostic> {
        let Some(captures) = self.class_method_captures.get(&method_id) else {
            return Ok(());
        };
        let mutable_captures = self
            .class_method_mutable_captures
            .get(&method_id)
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        for capture in captures {
            let local = self.resolve_local(capture).map_err(|_| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-289: class method capture `{capture}` is not available at this call site; escaped class lexical environments require heap environment support"
                ),
                span: None,
            })?;
            if mutable_captures.contains(capture) && !self.env_cell_locals.contains(&local) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-301: mutable class method capture `{capture}` is not available as an environment cell at this call site"
                    ),
                    span: None,
                });
            }
            lowered_args.push(LoweredExpr::Local(local));
        }

        Ok(())
    }

    pub(super) fn append_function_captures(
        &self,
        func_id: FuncId,
        lowered_args: &mut Vec<LoweredExpr>,
    ) -> Result<(), Diagnostic> {
        let Some(captures) = self.function_captures.get(&func_id) else {
            return Ok(());
        };
        let mutable_captures = self
            .function_mutable_captures
            .get(&func_id)
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        for capture in captures {
            let local = self.resolve_local(capture).map_err(|_| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-404: callback capture `{capture}` is not available at this call site; escaped callback lexical environments require heap environment support"
                ),
                span: None,
            })?;
            if mutable_captures.contains(capture) && !self.env_cell_locals.contains(&local) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-404: mutable callback capture `{capture}` is not available as an environment cell at this call site"
                    ),
                    span: None,
                });
            }
            lowered_args.push(LoweredExpr::Local(local));
        }

        Ok(())
    }

    pub(super) fn lower_function_expr_call(
        &mut self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if Self::is_direct_return_this_iife(params, body, args) {
            return Ok(LoweredExpr::ObjectNew { props: Vec::new() });
        }
        if params.iter().any(|param| param.is_rest) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-274: direct function-expression spread calls do not support rest parameters in this slice".to_owned(),
                span: Some(span),
            });
        }
        if block_contains_this(body) || block_contains_arguments(body) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-274: direct function-expression spread calls with `this` or `arguments` require broader call-expression runtime support".to_owned(),
                span: Some(span),
            });
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
            });
        };

        let explicit_args = self.lower_call_args(args)?;
        let mut lowered_args = explicit_args
            .into_iter()
            .take(params.len())
            .collect::<Vec<_>>();
        for _ in lowered_args.len()..params.len() {
            lowered_args.push(LoweredExpr::Undefined);
        }
        lowered_args.extend(captures.into_iter().map(LoweredExpr::Local));
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: lowered_args,
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

    pub(super) fn function_props_for_object_expr(
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
                    self.resolve_func(name).ok().map(|func_id| (key.clone(), func_id))
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

    pub(super) fn is_function_identifier(&self, expr: &ResolvedExpr) -> bool {
        matches!(expr, ResolvedExpr::Ident(name) if self.resolve_func(name).is_ok())
    }

    pub(super) fn lower_function_metadata_property(
        &self,
        name: &str,
        key: &str,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let func_id = self.resolve_func(name)?;
        match key {
            "name" => Ok(LoweredExpr::String(name.to_owned())),
            "length" => {
                let signature = self
                    .function_signatures
                    .get(&func_id)
                    .copied()
                    .unwrap_or_default();
                if let Some(length) = signature.metadata_length {
                    Ok(LoweredExpr::Number(length as i32))
                } else {
                    Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-062f: function `{name}` length metadata is only supported for fixed-arity function declarations"
                        ),
                        span: Some(span),
                    })
                }
            }
            "prototype" => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062f: function `{name}` prototype metadata is not supported in this slice"
                ),
                span: Some(span),
            }),
            _ => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062f: function `{name}` metadata property `{key}` is not supported"
                ),
                span: Some(span),
            }),
        }
    }

    pub(super) fn lower_arrow_fn(
        &mut self,
        params: &[String],
        body: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        self.lower_arrow_fn_with_self(params, body, None)
    }

    pub(super) fn lower_arrow_fn_with_self(
        &mut self,
        params: &[String],
        body: &ResolvedExpr,
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
        let capture_names = self.arrow_capture_names_with_excluded(body, &excluded);
        let captures = capture_names
            .iter()
            .map(|name| self.resolve_local(name))
            .collect::<Result<Vec<_>, _>>()?;
        let mut lowered_params = params
            .iter()
            .map(|name| ResolvedParam {
                name: name.clone(),
                default: None,
                is_rest: false,
                span: None,
            })
            .collect::<Vec<_>>();
        lowered_params.extend(capture_names.iter().map(|name| ResolvedParam {
            name: name.clone(),
            default: None,
            is_rest: false,
            span: None,
        }));

        let func_id = FuncId(self.next_func_id);
        self.next_func_id += 1;
        let body_stmts = vec![ResolvedStmt::Return((*body).clone())];
        let lowered = lower_function(
            func_id,
            &lowered_params,
            &body_stmts,
            self.function_ids,
            self.function_signatures,
            self.function_captures,
            self.function_mutable_captures,
            self.class_method_captures,
            self.class_method_mutable_captures,
            &self.env_cell_names,
            &self.heap_closure_names,
            self.class_parents.clone(),
            self.class_private_fields.clone(),
            self.class_static_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.current_class.as_deref(),
                in_constructor: false,
                next_func_id: self.next_func_id,
                self_closure: active_self_name.map(|name| SelfClosureOptions {
                    name,
                    func_id,
                    capture_names: &capture_names,
                }),
            },
        )?;
        self.next_func_id = lowered.next_func_id;
        self.generated_functions.push(lowered.function);
        self.generated_functions.extend(lowered.generated_functions);

        Ok(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: ClosureRepresentation::DirectLocalToken,
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
            });
        }
        if block_contains_this(body) || block_contains_arguments(body) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` closures with `this` or `arguments` are not supported in this slice"
                ),
                span: None,
            });
        }

        let capture_names = self.nested_function_capture_names(name, params, body)?;
        let mutable_captures = capture_names
            .iter()
            .filter(|capture| block_assigns_any_name(body, std::slice::from_ref(capture)))
            .cloned()
            .collect::<Vec<_>>();
        if mutable_captures
            .iter()
            .any(|capture| !self.env_cell_names.contains(capture))
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` mutates a captured outer local; mutable closure environments require heap environment support"
                ),
                span: None,
            });
        }
        let captures = capture_names
            .iter()
            .map(|capture| self.resolve_local(capture))
            .collect::<Result<Vec<_>, _>>()?;
        let mut lowered_params = params.to_vec();
        lowered_params.extend(
            capture_names
                .iter()
                .map(|capture| ResolvedParam {
                    name: capture.clone(),
                    default: None,
                    is_rest: false,
                    span: None,
                }),
        );

        let func_id = FuncId(self.next_func_id);
        self.next_func_id += 1;
        let self_closure = (!name.is_empty()).then_some(SelfClosureOptions {
            name,
            func_id,
            capture_names: &capture_names,
        }).filter(|_| !self.env_cell_names.contains(name));

        let lowered = lower_function(
            func_id,
            &lowered_params,
            body,
            self.function_ids,
            self.function_signatures,
            self.function_captures,
            self.function_mutable_captures,
            self.class_method_captures,
            self.class_method_mutable_captures,
            &self.env_cell_names,
            &self.heap_closure_names,
            self.class_parents.clone(),
            self.class_private_fields.clone(),
            self.class_static_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.current_class.as_deref(),
                in_constructor: false,
                next_func_id: self.next_func_id,
                self_closure,
            },
        )?;
        self.next_func_id = lowered.next_func_id;
        self.generated_functions.push(lowered.function);
        self.generated_functions.extend(lowered.generated_functions);

        Ok(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: if self.heap_closure_names.contains(name) {
                ClosureRepresentation::HeapObject
            } else {
                ClosureRepresentation::DirectLocalToken
            },
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
        let mut excluded = binding_param_names(
            params
                .iter()
                .map(|param| (param.name.as_str(), param.span)),
        )?
        .into_iter()
        .collect::<HashSet<_>>();
        if !self.env_cell_names.contains(name) {
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

    pub(super) fn declare_local(&mut self, name: &str) -> Result<LocalId, Diagnostic> {
        let scope = self.scopes.last_mut().expect("scope must exist");
        if scope.contains_key(name) {
            return Err(Diagnostic {
                code: DiagCode::DuplicateLocal,
                message: format!("duplicate local binding: `{name}`"),
                span: None,
            });
        }
        let local_id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(local_id);
        scope.insert(name.to_owned(), local_id);
        Ok(local_id)
    }

    pub(super) fn declare_self_closure(
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
        self.arrow_locals
            .insert(local_id, ArrowClosure { func_id, captures });
        Ok(())
    }

    pub(super) fn alloc_temp(&mut self) -> LocalId {
        let id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(id);
        id
    }

    pub(super) fn resolve_local(&self, name: &str) -> Result<LocalId, Diagnostic> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedName,
                message: format!("unresolved name: `{name}`"),
                span: None,
            })
    }

    pub(super) fn resolve_func(&self, name: &str) -> Result<FuncId, Diagnostic> {
        self.function_ids
            .get(name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedFunction,
                message: format!("unresolved function: `{name}`"),
                span: None,
            })
    }

    pub(super) fn module_id_for_specifier(&mut self, specifier: &str) -> usize {
        if let Some(id) = self.module_ids.get(specifier) {
            return *id;
        }

        let id = self.modules.len() + 1;
        self.module_ids.insert(specifier.to_owned(), id);
        self.modules.push(ModuleInfo {
            id,
            specifier: specifier.to_owned(),
            statements: Vec::new(),
            locals_count: 0,
        });
        id
    }

    pub(super) fn resolve_class_method(&self, class_name: &str, method: &str) -> Option<FuncId> {
        let mut current = Some(class_name.to_owned());
        while let Some(class) = current {
            if let Some(id) = self
                .class_method_ids
                .get(&(class.clone(), method.to_owned()))
                .copied()
            {
                return Some(id);
            }
            current = self.class_parents.get(&class).and_then(|p| p.clone());
        }
        None
    }

    pub(super) fn current_private_method_id(&self, method: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        self.class_method_ids
            .get(&(class_name.clone(), method.to_owned()))
            .copied()
    }

    pub(super) fn current_static_private_method_id(&self, method: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        self.class_static_method_ids
            .get(&(class_name.clone(), method.to_owned()))
            .copied()
    }

    pub(super) fn current_static_private_field_local_name(&self, key: &str) -> Option<String> {
        let class_name = self.current_class.as_ref()?;
        let field_name = key.strip_prefix('#')?;
        self.class_static_private_fields
            .get(class_name)
            .and_then(|fields| fields.get(field_name))
            .cloned()
    }

    pub(super) fn current_static_private_getter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        let getter_name = key.strip_prefix('#')?;
        self.class_static_method_ids
            .get(&(class_name.clone(), format!("#get::{getter_name}")))
            .copied()
    }

    pub(super) fn current_static_private_setter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        let setter_name = key.strip_prefix('#')?;
        self.class_static_method_ids
            .get(&(class_name.clone(), format!("#set::{setter_name}")))
            .copied()
    }

    pub(super) fn is_same_class_static_private_receiver(&self, object: &ResolvedExpr) -> bool {
        match object {
            ResolvedExpr::This { .. } => self.resolve_local("this").is_err(),
            ResolvedExpr::Ident(name) => self.current_class.as_deref() == Some(name.as_str()),
            _ => false,
        }
    }

    pub(super) fn current_private_getter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        self.private_getter_id_for_class(class_name, key)
    }

    pub(super) fn private_getter_id_for_class(&self, class_name: &str, key: &str) -> Option<FuncId> {
        let getter_name = key.strip_prefix('#')?;
        self.class_method_ids
            .get(&(class_name.to_owned(), format!("#get::{getter_name}")))
            .copied()
    }

    pub(super) fn current_private_setter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        self.private_setter_id_for_class(class_name, key)
    }

    pub(super) fn private_setter_id_for_class(&self, class_name: &str, key: &str) -> Option<FuncId> {
        let setter_name = key.strip_prefix('#')?;
        self.class_method_ids
            .get(&(class_name.to_owned(), format!("#set::{setter_name}")))
            .copied()
    }

    pub(super) fn private_field_brand_and_slot(
        &self,
        _object: &ResolvedExpr,
        key: &str,
        span: Span,
    ) -> Result<(u32, usize), Diagnostic> {
        let Some(field_name) = key.strip_prefix('#') else {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!("private field slot lookup requires private key, got `{key}`"),
                span: Some(span),
            });
        };
        let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-255: private field `#{field_name}` access requires declaring class context"
            ),
            span: Some(span),
        })?;
        let Some(slot) = self
            .class_private_fields
            .get(class_name)
            .and_then(|fields| fields.get(field_name))
            .copied()
        else
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-255: private field `#{field_name}` is not declared in class `{class_name}`"
                ),
                span: Some(span),
            });
        };
        let brand = self.private_brand_for_class(class_name, Some(span))?;
        Ok((brand, slot))
    }

    pub(super) fn private_brand_for_class(
        &self,
        class_name: &str,
        span: Option<Span>,
    ) -> Result<u32, Diagnostic> {
        let constructor = self
            .class_constructor_ids
            .get(class_name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "private brand lookup requires constructor for class `{class_name}`"
                ),
                span,
            })?;
        u32::try_from(constructor.0.saturating_add(1)).map_err(|_| Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!("private brand for class `{class_name}` exceeds u32"),
            span,
        })
    }

    pub(super) fn private_slot_count(&self, class_name: &str) -> usize {
        self.class_private_fields
            .get(class_name)
            .map_or(0, HashMap::len)
    }

    pub(super) fn class_has_instance_private_brand(&self, class_name: &str) -> bool {
        self.private_slot_count(class_name) > 0
            || self
                .class_method_ids
                .keys()
                .any(|(owner, method)| owner == class_name && method.starts_with('#'))
    }

    pub(super) fn is_object_key_enumeration_leak(
        &self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
    ) -> bool {
        matches!(object, ResolvedExpr::Ident(name) if name == "Object")
            && matches!(method, "keys" | "values" | "entries")
            && args
                .first()
                .is_some_and(|arg| self.expr_has_private_progress_storage(arg))
    }

    pub(super) fn expr_has_private_progress_storage(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::This { .. } => self
                .current_class
                .as_ref()
                .is_some_and(|class_name| self.class_has_private_progress_storage(class_name)),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local| self.local_has_private_progress_storage(local)),
            ResolvedExpr::New { class_name, .. } => self.class_has_private_progress_storage(class_name),
            _ => false,
        }
    }

    pub(super) fn local_has_private_progress_storage(&self, local: LocalId) -> bool {
        self.local_classes
            .get(&local)
            .is_some_and(|class_name| self.class_has_private_progress_storage(class_name))
    }

    pub(super) fn class_has_private_progress_storage(&self, class_name: &str) -> bool {
        self.class_private_fields
            .get(class_name)
            .is_some_and(|fields| !fields.is_empty())
    }

    pub(super) fn is_date_receiver(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::New { class_name, .. } => class_name == "Date",
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .and_then(|local_id| self.local_classes.get(&local_id))
                .is_some_and(|class_name| class_name == "Date"),
            _ => false,
        }
    }

    pub(super) fn is_unsupported_regexp_compile_receiver(&self, expr: &ResolvedExpr, method: &str) -> bool {
        if method != "compile" {
            return false;
        }
        match expr {
            ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => true,
            ResolvedExpr::New { class_name, .. } => class_name == "RegExp",
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local_id| {
                self.regexp_literal_locals.contains(&local_id)
                    || self
                        .local_classes
                        .get(&local_id)
                        .is_some_and(|class_name| class_name == "RegExp")
            }),
            _ => false,
        }
    }

    pub(super) fn update_regexp_literal_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if matches!(expr, ResolvedExpr::String(raw) if looks_like_regexp_literal(raw)) {
            self.regexp_literal_locals.insert(local_id);
        } else {
            self.regexp_literal_locals.remove(&local_id);
        }
    }

    pub(super) fn update_bigint_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if self.resolved_expr_is_bigint(expr) {
            self.bigint_locals.insert(local_id);
        } else {
            self.bigint_locals.remove(&local_id);
        }
    }

    pub(super) fn update_control_flow_bigint_assignment(&mut self, local_id: LocalId) {
        self.control_flow_bigint_div_rem_locals.remove(&local_id);
        self.control_flow_mixed_bigint_locals.remove(&local_id);
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
            self.heap_closure_locals.insert(local_id);
        } else {
            self.heap_closure_locals.remove(&local_id);
        }
    }

    pub(super) fn update_nullish_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if self.resolved_expr_is_nullish(expr) {
            self.nullish_locals.insert(local_id);
        } else {
            self.nullish_locals.remove(&local_id);
        }
    }

    pub(super) fn resolved_expr_is_nullish(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Null | ResolvedExpr::Undefined => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.nullish_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(super) fn update_array_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if let Some(slots) = self.resolved_expr_static_array_slots(expr) {
            self.array_locals.insert(local_id);
            self.static_array_slots.insert(local_id, slots);
        } else if self.resolved_expr_produces_dense_array(expr) {
            self.array_locals.insert(local_id);
            self.static_array_slots.remove(&local_id);
        } else {
            self.array_locals.remove(&local_id);
            self.static_array_slots.remove(&local_id);
        }
    }

    pub(super) fn update_symbol_iterator_object_local(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        if self.resolved_expr_has_symbol_iterator_property(expr) {
            self.symbol_iterator_object_locals.insert(local_id);
        } else {
            self.symbol_iterator_object_locals.remove(&local_id);
        }
    }

    pub(super) fn resolved_expr_has_symbol_iterator_property(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Object(props) => props
                .iter()
                .any(|(key, _)| key == SYMBOL_ITERATOR_OBJECT_KEY),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.symbol_iterator_object_locals.contains(&local_id)),
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
        self.generator_function_names.contains(name)
    }

    pub(super) fn unsupported_generator_spread_diagnostic() -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedRuntimeSubset,
            message:
                "issue-353: generator result spread requires iterator protocol runtime lowering in this milestone"
                    .to_owned(),
            span: None,
        }
    }

    pub(super) fn unsupported_symbol_iterator_spread_diagnostic() -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-353: custom iterable spread via Symbol.iterator requires iterator protocol runtime support in this milestone"
                    .to_owned(),
            span: None,
        }
    }

    pub(super) fn unsupported_map_spread_diagnostic() -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedRuntimeSubset,
            message:
                "issue-353/407: Map spread requires key-preserving Map iterator entry storage before iterator protocol lowering"
                    .to_owned(),
            span: None,
        }
    }

    pub(super) fn update_static_object_literal_local_on_let(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        if let Some(props) = self.static_copy_safe_object_literal_props(expr) {
            self.static_object_literal_locals.insert(local_id, props);
            self.update_static_object_literal_alias_sources(local_id, expr);
        } else {
            self.static_object_literal_locals.remove(&local_id);
            self.static_object_literal_alias_sources.remove(&local_id);
        }
    }

    pub(super) fn update_static_function_array_like_local_on_let(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
    ) {
        let ResolvedExpr::FunctionExpr { params, .. } = expr else {
            self.static_function_array_like_locals.remove(&local_id);
            return;
        };
        if params
            .iter()
            .any(|param| param.default.is_some() || param.is_rest)
        {
            self.static_function_array_like_locals.remove(&local_id);
            return;
        }
        self.static_function_array_like_locals.insert(
            local_id,
            StaticFunctionArrayLike {
                elements: vec![None; params.len()],
            },
        );
    }

    pub(super) fn invalidate_static_function_array_like_local(&mut self, local_id: LocalId) {
        self.static_function_array_like_locals.remove(&local_id);
    }

    pub(super) fn update_static_function_array_like_index(
        &mut self,
        local_id: LocalId,
        index: &ResolvedExpr,
        value: &ResolvedExpr,
    ) {
        let Some(static_receiver) = self.static_function_array_like_locals.get_mut(&local_id)
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
        let static_receiver = self.static_function_array_like_locals.get(&local_id)?;
        static_receiver
            .elements
            .iter()
            .cloned()
            .collect::<Option<Vec<_>>>()
    }

    pub(super) fn invalidate_static_object_literal_local(&mut self, local_id: LocalId) {
        self.static_object_literal_locals.remove(&local_id);
        self.static_object_literal_alias_sources.remove(&local_id);
        let dependent_aliases = self
            .static_object_literal_alias_sources
            .iter()
            .filter_map(|(alias, sources)| sources.contains(&local_id).then_some(*alias))
            .collect::<Vec<_>>();
        for alias in dependent_aliases {
            self.static_object_literal_locals.remove(&alias);
            self.static_object_literal_alias_sources.remove(&alias);
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
                if self.env_cell_locals.contains(&local_id) {
                    return None;
                }
                self.static_object_literal_locals.get(&local_id).cloned()
            }
            _ => None,
        }
    }

    pub(super) fn update_static_object_literal_alias_sources(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        self.static_object_literal_alias_sources.remove(&local_id);
        if let ResolvedExpr::Ident(name) = expr
            && let Ok(source_id) = self.resolve_local(name)
        {
            let mut sources = self
                .static_object_literal_alias_sources
                .get(&source_id)
                .cloned()
                .unwrap_or_default();
            sources.insert(source_id);
            self.static_object_literal_alias_sources
                .insert(local_id, sources);
        }
    }

    pub(super) fn update_string_literal_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if let Some(value) = self.resolved_expr_static_string_value(expr) {
            self.string_literal_locals.insert(local_id, value);
        } else {
            self.string_literal_locals.remove(&local_id);
        }
    }

    pub(super) fn resolved_expr_static_string_value(&self, expr: &ResolvedExpr) -> Option<String> {
        match expr {
            ResolvedExpr::String(value) => Some(value.clone()),
            ResolvedExpr::Ident(name) => {
                let local_id = self.resolve_local(name).ok()?;
                if self.env_cell_locals.contains(&local_id) {
                    return None;
                }
                self.string_literal_locals.get(&local_id).cloned()
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
                self.array_locals.contains(&local_id) && !self.env_cell_locals.contains(&local_id)
            }),
            ResolvedExpr::MethodCall { object, method, args, .. } if method == "map" => {
                self.is_known_array_expr(object)
                    && (string_constructor_arrow_callback(args) || unary_plus_arrow_callback(args))
            }
            ResolvedExpr::Call { callee, .. } => match callee.as_ref() {
                ResolvedExpr::Ident(name) => self
                    .resolve_func(name)
                    .ok()
                    .and_then(|func_id| self.function_signatures.get(&func_id))
                    .is_some_and(|signature| signature.returns_dense_array),
                _ => false,
            },
            _ => false,
        }
    }

    pub(super) fn update_native_set_add_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if is_set_prototype_property_expr(expr, "add") {
            self.native_set_add_locals.insert(local_id);
        } else {
            self.native_set_add_locals.remove(&local_id);
        }
    }

    pub(super) fn is_known_array_expr(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Array(_) => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.array_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(super) fn resolved_expr_static_array_slots(
        &self,
        expr: &ResolvedExpr,
    ) -> Option<Vec<ResolvedArrayElement>> {
        match expr {
            ResolvedExpr::Array(elements) => Some(elements.clone()),
            ResolvedExpr::New { class_name, args, .. } if class_name == "Array" => {
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
                .and_then(|local_id| self.static_array_slots.get(&local_id).cloned()),
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
        if !self.static_array_slots.contains_key(&local_id) {
            return;
        }
        let ResolvedExpr::Number(index) = key.as_ref() else {
            self.static_array_slots.remove(&local_id);
            return;
        };
        let Some(slots) = self.static_array_slots.get_mut(&local_id) else {
            return;
        };
        if *index < 0 || *index as usize >= slots.len() {
            self.static_array_slots.remove(&local_id);
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
                    .and_then(|func_id| self.function_signatures.get(&func_id))
                    .is_some_and(|signature| signature.returns_heap_closure),
                _ => false,
            },
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.heap_closure_locals.contains(&local_id)),
            _ => false,
        }
    }

    pub(super) fn resolved_expr_is_bigint(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::BigIntLiteral { .. } => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.bigint_locals.contains(&local_id)),
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
                )
                    && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
            }
            ResolvedExpr::Call { callee, .. } => {
                matches!(
                    callee.as_ref(),
                    ResolvedExpr::Ident(name)
                        if crate::builtin_resolver::bigint_runtime_fn_name(name).is_some()
                )
            }
            ResolvedExpr::MethodCall { object, method, .. } => {
                matches!(
                    object.as_ref(),
                    ResolvedExpr::Ident(name) if name == "__ts2wasm_bigint_runtime"
                ) && crate::builtin_resolver::bigint_runtime_fn_name(method).is_some()
            }
            _ => false,
        }
    }

    pub(super) fn resolved_expr_is_bigint_div_rem_operand(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local_id| {
                self.bigint_locals.contains(&local_id)
                    || self
                        .control_flow_bigint_div_rem_locals
                        .contains(&local_id)
            }),
            ResolvedExpr::Unary { op, expr } => {
                *op == UnaryOp::Negate && self.resolved_expr_is_bigint_div_rem_operand(expr)
            }
            _ => self.resolved_expr_is_bigint(expr),
        }
    }

    pub(super) fn resolved_expr_is_control_flow_mixed_bigint(
        &self,
        expr: &ResolvedExpr,
    ) -> bool {
        let ResolvedExpr::Ident(name) = expr else {
            return false;
        };
        self.resolve_local(name).ok().is_some_and(|local_id| {
            self.control_flow_mixed_bigint_locals.contains(&local_id)
        })
    }

    pub(super) fn bigint_div_rem_candidate_locals(&self) -> HashSet<LocalId> {
        self.bigint_locals
            .union(&self.control_flow_bigint_div_rem_locals)
            .copied()
            .collect()
    }

    pub(super) fn class_prototype_ref(&self, class_name: &str) -> Result<ClassPrototypeRef, Diagnostic> {
        let constructor = self
            .class_constructor_ids
            .get(class_name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-207: instanceof right-hand side must be a supported class constructor `{}`",
                    class_name
                ),
                span: None,
            })?;

        let mut parent_constructors = Vec::new();
        let mut current = self.class_parents.get(class_name).and_then(|p| p.clone());
        while let Some(parent) = current {
            let parent_constructor = self
                .class_constructor_ids
                .get(&parent)
                .copied()
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-207: superclass constructor `{}` is not available for instanceof",
                        parent
                    ),
                    span: None,
                })?;
            parent_constructors.push(parent_constructor);
            current = self.class_parents.get(&parent).and_then(|p| p.clone());
        }

        Ok(ClassPrototypeRef {
            constructor,
            parent_constructors,
        })
    }

    pub(super) fn infer_class_for_expr(&self, expr: &ResolvedExpr) -> Option<String> {
        match expr {
            ResolvedExpr::New { class_name, .. } => Some(class_name.clone()),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .and_then(|local_id| self.local_classes.get(&local_id).cloned()),
            _ => None,
        }
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
