use super::*;

impl<'a> Resolver<'a> {
    pub(super) fn lower_expr(&mut self, expr: &ResolvedExpr) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            ResolvedExpr::Number(value) => Ok(LoweredExpr::Number(*value)),
            ResolvedExpr::BigIntLiteral {
                decimal,
                sign,
                limb_low,
                limb_high,
            } => Ok(LoweredExpr::BigIntLiteral {
                decimal: decimal.clone(),
                sign: *sign,
                limb_low: *limb_low,
                limb_high: *limb_high,
            }),
            ResolvedExpr::String(value) => Ok(LoweredExpr::String(value.clone())),
            ResolvedExpr::Bool(value) => Ok(LoweredExpr::Bool(*value)),
            ResolvedExpr::Null => Ok(LoweredExpr::Null),
            ResolvedExpr::Undefined => Ok(LoweredExpr::Undefined),
            ResolvedExpr::This { span } => match self.resolve_local("this") {
                Ok(local) => Ok(LoweredExpr::Local(local)),
                Err(_) => Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-062d: `this` is only supported inside receiver-bound functions, class constructors, and instance methods in this milestone".to_owned(),
                    span: Some(*span),
                }),
            },
            ResolvedExpr::Ident(name) => {
                // Handle special global constants Infinity and NaN
                // Note: These are approximated as max/min representable numbers due to small-int number model
                // Proper Infinity/NaN support requires broader number-model support (issue-281)
                if name == "Infinity" {
                    use ts2wasm_runtime_abi::ValueTag;
                    return Ok(LoweredExpr::Number(ValueTag::NUMBER_PAYLOAD_MAX));
                }
                if name == "NaN" {
                    // NaN is approximated as 0 for now (not spec-compliant but pragmatic)
                    // Proper NaN support requires broader number-model support (issue-281)
                    return Ok(LoweredExpr::Number(0));
                }
                match self.resolve_local(name) {
                    Ok(local) if self.env_cell_locals.contains(&local) => Ok(LoweredExpr::EnvCellGet(local)),
                    Ok(local) => Ok(LoweredExpr::Local(local)),
                    Err(_) if name == "arguments" => Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-062d: `arguments` is only supported inside non-arrow functions in this milestone".to_owned(),
                        span: None,
                    }),
                    Err(err) => Err(err),
                }
            }
            ResolvedExpr::Spread(_) => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-274: spread expressions are only supported in call arguments over literal arrays in this milestone".to_owned(),
                span: None,
            }),
            ResolvedExpr::Unary { op, expr } => {
                // Handle -Infinity specially to ensure it returns the minimum representable number
                if *op == UnaryOp::Negate {
                    if let ResolvedExpr::Ident(name) = expr.as_ref() && name == "Infinity" {
                        use ts2wasm_runtime_abi::ValueTag;
                        return Ok(LoweredExpr::Number(ValueTag::NUMBER_PAYLOAD_MIN));
                    }
                    if self.resolved_expr_is_bigint(expr) {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "BigIntUnaryMinus".to_owned(),
                            args: vec![self.lower_expr(expr)?],
                        });
                    }
                }
                if *op == UnaryOp::BitwiseNot && self.resolved_expr_is_bigint(expr) {
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "BigIntBitwiseNot".to_owned(),
                        args: vec![self.lower_expr(expr)?],
                    });
                }
                if *op == UnaryOp::Delete {
                    // Lower delete to PropertyDelete or PropertyDeleteDynamic
                    match expr.as_ref() {
                        ResolvedExpr::PropertyAccess {
                            object,
                            key,
                            span,
                        } => {
                            if is_private_field_storage_key(key) {
                                return Err(private_storage_observable_access_diagnostic(Some(
                                    *span,
                                )));
                            }
                            if key.starts_with('#') {
                                return Err(Diagnostic {
                                    code: DiagCode::UnsupportedSyntax,
                                    message: format!(
                                        "issue-255: private member `{key}` cannot be deleted in this private class runtime slice"
                                    ),
                                    span: Some(*span),
                                });
                            }
                            Ok(LoweredExpr::PropertyDelete {
                                object: Box::new(self.lower_expr(object)?),
                                key: key.clone(),
                            })
                        }
                        ResolvedExpr::ComputedIndex { object, index } => {
                            if self.expr_has_private_progress_storage(object) {
                                return Err(private_storage_observable_access_diagnostic(None));
                            }
                            if let ResolvedExpr::String(key) = index.as_ref()
                                && is_private_field_storage_key(key)
                            {
                                return Err(private_storage_observable_access_diagnostic(None));
                            }
                            Ok(LoweredExpr::PropertyDeleteDynamic {
                                object: Box::new(self.lower_expr(object)?),
                                key: Box::new(self.lower_expr(index)?),
                            })
                        }
                        _ => Ok(LoweredExpr::Unary {
                            op: lower_unary_op(*op)?,
                            expr: Box::new(self.lower_expr(expr)?),
                        }),
                    }
                } else {
                    Ok(LoweredExpr::Unary {
                        op: lower_unary_op(*op)?,
                        expr: Box::new(self.lower_expr(expr)?),
                    })
                }
            }
            ResolvedExpr::Binary { left, op, right } => {
                if *op == BinaryOp::InstanceOf {
                    let prototype = match right.as_ref() {
                        ResolvedExpr::Ident(name) => {
                            if let Some(constructor) = BuiltinErrorConstructor::from_name(name) {
                                LoweredExpr::BuiltinErrorPrototype(constructor)
                            } else {
                                self.class_prototype_ref(name)
                                    .map(LoweredExpr::ClassPrototype)?
                            }
                        }
                        _ => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "issue-207: instanceof right-hand side must be a supported class constructor".to_owned(),
                                span: None,
                            });
                        }
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "$instanceof".to_string(),
                        args: vec![self.lower_expr(left)?, prototype],
                    })
                } else if *op == BinaryOp::In {
                    // Lower in to PropertyIn or PropertyInDynamic
                    // key in object -> check if key exists in object
                    match left.as_ref() {
                        ResolvedExpr::String(key) => Ok(LoweredExpr::PropertyIn {
                            obj: Box::new(self.lower_expr(right)?),
                            key: key.clone(),
                        }),
                        _ => Ok(LoweredExpr::PropertyInDynamic {
                            obj: Box::new(self.lower_expr(right)?),
                            key: Box::new(self.lower_expr(left)?),
                        }),
                    }
                } else if matches!(
                    op,
                    BinaryOp::Add
                        | BinaryOp::Subtract
                        | BinaryOp::Multiply
                        | BinaryOp::Divide
                        | BinaryOp::Modulo
                )
                    && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
                {
                    let runtime_fn = match op {
                        BinaryOp::Add => "BigIntAdd",
                        BinaryOp::Subtract => "BigIntSub",
                        BinaryOp::Multiply => "BigIntMul",
                        BinaryOp::Divide => "BigIntDiv",
                        BinaryOp::Modulo => "BigIntRem",
                        _ => unreachable!("checked above"),
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: runtime_fn.to_owned(),
                        args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                    })
                } else if *op == BinaryOp::Power
                    && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
                {
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "BigIntPow".to_owned(),
                        args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                    })
                } else if matches!(
                    op,
                    BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor
                ) && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
                {
                    let runtime_fn = match op {
                        BinaryOp::BitwiseAnd => "BigIntBitwiseAnd",
                        BinaryOp::BitwiseOr => "BigIntBitwiseOr",
                        BinaryOp::BitwiseXor => "BigIntBitwiseXor",
                        _ => unreachable!("checked above"),
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: runtime_fn.to_owned(),
                        args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                    })
                } else if *op == BinaryOp::Power
                    && (self.resolved_expr_is_bigint(left) || self.resolved_expr_is_bigint(right))
                {
                    Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-370: mixed Number/BigInt arithmetic TypeError parity is not implemented in the dynamic BigInt runtime slice".to_owned(),
                        span: None,
                    })
                } else {
                    Ok(LoweredExpr::Binary {
                        left: Box::new(self.lower_expr(left)?),
                        op: lower_binary_op(*op)?,
                        right: Box::new(self.lower_expr(right)?),
                    })
                }
            }
            ResolvedExpr::Assign { name, expr } => {
                let local = self.resolve_local(name)?;
                self.invalidate_static_object_literal_local(local);
                self.invalidate_static_function_array_like_local(local);
                let expr = Box::new(self.lower_expr(expr)?);
                if self.env_cell_locals.contains(&local) {
                    Ok(LoweredExpr::EnvCellSet { cell: local, expr })
                } else {
                    Ok(LoweredExpr::Assign { local, expr })
                }
            }
            ResolvedExpr::LogicalAssign { name, op, expr } => {
                let local = self.resolve_local(name)?;
                self.invalidate_static_object_literal_local(local);
                self.invalidate_static_function_array_like_local(local);
                Ok(LoweredExpr::LogicalAssign {
                    local,
                    op: lower_logical_assign_op(*op),
                    expr: Box::new(self.lower_expr(expr)?),
                })
            }
            ResolvedExpr::LogicalPropertyAssign {
                object,
                key,
                op,
                expr,
            } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                let object = self.resolve_local(object)?;
                self.invalidate_static_object_literal_local(object);
                Ok(LoweredExpr::LogicalPropertyAssign {
                    object,
                    key: key.clone(),
                    op: lower_logical_assign_op(*op),
                    expr: Box::new(self.lower_expr(expr)?),
                })
            }
            ResolvedExpr::LogicalComputedPropertyAssign {
                object,
                key,
                op,
                expr,
            } => {
                let object = self.resolve_local(object)?;
                self.invalidate_static_object_literal_local(object);
                if self.local_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::LogicalComputedPropertyAssign {
                    object,
                    key: Box::new(self.lower_expr(key)?),
                    op: lower_logical_assign_op(*op),
                    expr: Box::new(self.lower_expr(expr)?),
                })
            }
            ResolvedExpr::LogicalComputedMemberAssign {
                object,
                key,
                op,
                expr,
            } => Ok(LoweredExpr::LogicalComputedMemberAssign {
                object: {
                    if self.expr_has_private_progress_storage(object) {
                        return Err(private_storage_observable_access_diagnostic(None));
                    }
                    Box::new(self.lower_expr(object)?)
                },
                key: Box::new(self.lower_expr(key)?),
                op: lower_logical_assign_op(*op),
                expr: Box::new(self.lower_expr(expr)?),
            }),
            ResolvedExpr::LogicalMemberAssign {
                object,
                key,
                op,
                expr,
            } => Ok(LoweredExpr::LogicalMemberAssign {
                object: Box::new(self.lower_expr(object)?),
                key: key.clone(),
                op: lower_logical_assign_op(*op),
                expr: Box::new(self.lower_expr(expr)?),
            }),
            ResolvedExpr::Call { callee, args, span } => {
                if let ResolvedExpr::FunctionExpr { name, params, body } = callee.as_ref() {
                    return self.lower_function_expr_call(name, params, body, args, *span);
                }

                let func_name = match callee.as_ref() {
                    ResolvedExpr::Ident(name) => name,
                    _ => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "only identifier calls are supported in expression context"
                                .to_owned(),
                            span: Some(*span),
                        });
                    }
                };

                if let Some(runtime_fn) = crate::builtin_resolver::bigint_runtime_fn_name(func_name)
                {
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: runtime_fn.to_owned(),
                        args: self.lower_call_args(args)?,
                    });
                }

                if let Ok(local_id) = self.resolve_local(func_name)
                    && let Some(closure) = self.arrow_locals.get(&local_id).cloned()
                {
                    let mut lowered_args = self.lower_call_args(args)?;
                    lowered_args.extend(closure.captures.iter().copied().map(LoweredExpr::Local));
                    return Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(closure.func_id),
                        args: lowered_args,
                    });
                }

                if let Ok(local_id) = self.resolve_local(func_name)
                    && self.heap_closure_locals.contains(&local_id)
                {
                    let receiver = if self.env_cell_locals.contains(&local_id) {
                        LoweredExpr::EnvCellGet(local_id)
                    } else {
                        LoweredExpr::Local(local_id)
                    };
                    let mut lowered_args = vec![receiver];
                    lowered_args.extend(self.lower_call_args(args)?);
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "HeapClosureCall".to_owned(),
                        args: lowered_args,
                    });
                }

                if func_name == "super" {
                    if !self.in_constructor {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super(...) is only supported in constructors".to_owned(),
                            span: None,
                        });
                    }
                    let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "super(...) requires class context".to_owned(),
                        span: None,
                    })?;
                    let parent_name = self
                        .class_parents
                        .get(class_name)
                        .and_then(|p| p.clone())
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super(...) used in class without extends".to_owned(),
                            span: None,
                        })?;
                    let parent_ctor = self
                        .class_constructor_ids
                        .get(&parent_name)
                        .copied()
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "super class constructor for `{}` not found",
                                parent_name
                            ),
                            span: None,
                        })?;

                    let mut lowered_args = vec![LoweredExpr::Local(self.resolve_local("this")?)];
                    lowered_args.extend(
                        args.iter()
                            .map(|arg| self.lower_expr(arg))
                            .collect::<Result<Vec<_>, _>>()?,
                    );

                    return Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(parent_ctor),
                        args: lowered_args,
                    });
                }

                if func_name == "String" {
                    if let [arg] = args.as_slice()
                        && self.resolved_expr_is_bigint(arg)
                    {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "BigIntToString".to_owned(),
                            args: vec![self.lower_expr(arg)?],
                        });
                    }
                }

                if func_name == "Boolean" {
                    if let [ResolvedExpr::BigIntLiteral { .. }] = args.as_slice() {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "BigIntToBoolean".to_owned(),
                            args: vec![self.lower_expr(&args[0])?],
                        });
                    }
                }

                let func_id = match self.resolve_func(func_name) {
                    Ok(func_id) => func_id,
                    Err(_) if self.resolve_local(func_name).is_ok() => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-211: function-valued local calls such as extracted method `{func_name}(...)` are not supported; call receiver.method(...) directly"
                            ),
                            span: Some(*span),
                        });
                    }
                    Err(err) => return Err(err),
                };
                if self
                    .function_signatures
                    .get(&func_id)
                    .is_some_and(|signature| signature.needs_receiver)
                {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-062d: direct call `{func_name}(...)` cannot bind a supported receiver for `this`; call through a supported receiver object"
                        ),
                        span: Some(*span),
                    });
                }
                let lowered_args =
                    self.lower_function_call_args(func_id, LoweredExpr::Undefined, args)?;

                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(func_id),
                    args: lowered_args,
                })
            }
            ResolvedExpr::BuiltinCall { builtin, args } => {
                let lowered_args = args
                    .iter()
                    .map(|arg| self.lower_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(*builtin),
                    args: lowered_args,
                })
            }
            ResolvedExpr::BuiltinProperty {
                builtin,
                object,
                span,
            } => match builtin {
                BuiltinPropertyId::Length => match object.as_ref() {
                    ResolvedExpr::Ident(name) if self.resolve_func(name).is_ok() => {
                        self.lower_function_metadata_property(name, "length", *span)
                    }
                    _ => Ok(LoweredExpr::GetLength(Box::new(self.lower_expr(object)?))),
                },
            },
            ResolvedExpr::PropertyAccess { object, key, span } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(Some(*span)));
                }
                if is_array_prototype_push_property(object, key) {
                    return Ok(LoweredExpr::Number(0));
                }
                if key.starts_with('#') {
                    if let Some(local_name) = self.current_static_private_field_local_name(key) {
                        if self.is_same_class_static_private_receiver(object) {
                            let local = self.resolve_local(&local_name).map_err(|_| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-352: static private field `{key}` cannot be accessed before its declaration in class static initialization order"
                                ),
                                span: Some(*span),
                            })?;
                            return Ok(if self.env_cell_locals.contains(&local) {
                                LoweredExpr::EnvCellGet(local)
                            } else {
                                LoweredExpr::Local(local)
                            });
                        }
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: static private field `{key}` access is currently supported only as `this.{key}` inside static methods or `Class.{key}` inside the declaring class"
                            ),
                            span: Some(*span),
                        });
                    }
                    if let Some(getter_id) = self.current_static_private_getter_id(key) {
                        if self.is_same_class_static_private_receiver(object) {
                            return Ok(LoweredExpr::Call {
                                kind: FunctionCallKind::User(getter_id),
                                args: Vec::new(),
                            });
                        }
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: static private getter `{key}` access is currently supported only as `this.{key}` inside static methods or `Class.{key}` inside the declaring class"
                            ),
                            span: Some(*span),
                        });
                    }
                    if self.current_private_method_id(key).is_some() {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private method `{key}` extraction is not supported in this private method runtime slice; call it directly as `this.{key}(...)`"
                            ),
                            span: Some(*span),
                        });
                    }
                    if let Some(getter_id) = self.current_private_getter_id(key) {
                        if !matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-255: private getter `{key}` access is currently supported only as `this.{key}` inside the declaring class"
                                ),
                                span: Some(*span),
                            });
                        }
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(getter_id),
                            args: vec![LoweredExpr::Local(self.resolve_local("this")?)],
                        });
                    }
                    if let Some(class_name) = self.infer_class_for_expr(object)
                        && self.private_getter_id_for_class(&class_name, key).is_some()
                    {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private getter `{key}` external access is not supported in this private accessor runtime slice"
                            ),
                            span: Some(*span),
                        });
                    }
                    let slot = self.private_field_slot(object, key, *span)?;
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "PrivateFieldGet".to_owned(),
                        args: vec![self.lower_expr(object)?, LoweredExpr::Number(slot as i32)],
                    });
                }
                if let ResolvedExpr::Ident(name) = object.as_ref()
                    && self.resolve_func(name).is_ok()
                {
                    return self.lower_function_metadata_property(name, key, *span);
                }
                if key == "size"
                    && let ResolvedExpr::Ident(receiver_name) = object.as_ref()
                    && let Ok(obj_local) = self.resolve_local(receiver_name)
                    && self
                        .local_classes
                        .get(&obj_local)
                        .is_some_and(|class_name| class_name == "Set")
                {
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "SetSize".to_owned(),
                        args: vec![LoweredExpr::Local(obj_local)],
                    });
                }
                if is_set_prototype_property(object, key, "add") {
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "SetPrototypeAddGet".to_owned(),
                        args: Vec::new(),
                    });
                }
                Ok(LoweredExpr::PropertyGet {
                    obj: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                })
            },
            ResolvedExpr::OptionalPropertyAccess { object, key, .. } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::OptionalPropertyGet {
                    obj: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                })
            }
            ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::OptionalIndex {
                    object: Box::new(self.lower_expr(object)?),
                    index: Box::new(self.lower_expr(index)?),
                })
            }
            ResolvedExpr::OptionalCall { callee, args, span } => {
                self.lower_optional_call(callee, args, *span)
            }
            ResolvedExpr::ComputedIndex { object, index } => {
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                // Lower the object first to determine its type
                let lowered_object = self.lower_expr(object)?;
                let lowered_index = self.lower_expr(index)?;

                // Keep obvious array literals on the compact array helper. Unknown
                // receivers must use the generic index helper so object[stringKey]
                // and array[numberIndex] both preserve JavaScript semantics.
                if matches!(object.as_ref(), ResolvedExpr::String(_)) {
                    Ok(LoweredExpr::Index {
                        object: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                    })
                } else if matches!(object.as_ref(), ResolvedExpr::Array(_))
                    || matches!(lowered_object, LoweredExpr::ArrayNew { .. })
                {
                    Ok(LoweredExpr::ArrayGet {
                        arr: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                    })
                } else {
                    Ok(LoweredExpr::Index {
                        object: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                    })
                }
            }
            ResolvedExpr::Array(elements) => {
                self.lower_array_literal(elements)
            }
            ResolvedExpr::Object(props) => {
                self.lower_object_literal_expr(props)
            }
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                span,
            } => {
                if method == "call" && is_array_prototype_push_expr(object) {
                    let Some((receiver, values)) = args.split_first() else {
                        return Err(Diagnostic {
                            code: DiagCode::ArityMismatch,
                            message: "Array.prototype.push.call expects a receiver argument"
                                .to_owned(),
                            span: Some(*span),
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
                        runtime_fn: "ArrayPushMany".to_owned(),
                        args: lowered_args,
                    });
                }
                if is_array_prototype_map_call_receiver(object, method) {
                    return self.lower_array_prototype_map_call(args, *span);
                }
                if method == "call" && is_set_prototype_property_expr(object, "originalAdd") {
                    return self.lower_native_set_add_call(args, *span);
                }
                if method == "call"
                    && let ResolvedExpr::Ident(name) = object.as_ref()
                    && let Ok(local_id) = self.resolve_local(name)
                    && self.native_set_add_locals.contains(&local_id)
                {
                    return self.lower_native_set_add_call(args, *span);
                }
                if matches!(
                    object.as_ref(),
                    ResolvedExpr::Ident(name) if name == "__ts2wasm_bigint_runtime"
                ) && let Some(runtime_fn) = crate::builtin_resolver::bigint_runtime_fn_name(method)
                {
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: runtime_fn.to_owned(),
                        args: self.lower_call_args(args)?,
                    });
                }
                if method.starts_with('#') {
                    if let Some(method_id) = self.current_static_private_method_id(method) {
                        let same_class_static_receiver = match object.as_ref() {
                            ResolvedExpr::This { .. } => self.resolve_local("this").is_err(),
                            ResolvedExpr::Ident(name) => {
                                self.current_class.as_deref() == Some(name.as_str())
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
                            });
                        }
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: static private method `{method}` calls are currently supported only as `this.{method}(...)` inside static methods or `Class.{method}(...)` inside the declaring class"
                            ),
                            span: Some(*span),
                        });
                    }
                    if !matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private method `{method}` calls are currently supported only as `this.{method}(...)` inside the declaring class"
                            ),
                            span: Some(*span),
                        });
                    }
                    let method_id = self.current_private_method_id(method).ok_or_else(|| {
                        Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private method `{method}` is not declared in this class"
                            ),
                            span: Some(*span),
                        }
                    })?;
                    let mut lowered_args = vec![LoweredExpr::Local(self.resolve_local("this")?)];
                    lowered_args.extend(
                        args.iter()
                            .map(|e| self.lower_expr(e))
                            .collect::<Result<Vec<_>, _>>()?,
                    );
                    return Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(method_id),
                        args: lowered_args,
                    });
                }
                if is_json_static_call(object, method) {
                    validate_json_stringify_args(
                        args,
                        *span,
                        self.function_ids,
                        self.function_signatures,
                    )?;
                    let mut lowered_args = Vec::with_capacity(3);
                    let value = if let (
                        ResolvedExpr::Object(props),
                        Some(replacer_keys),
                    ) = (&args[0], json_stringify_replacer_keys(args, self.function_ids))
                    {
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
                        }
                    } else {
                        self.lower_expr(&args[0])?
                    };
                    lowered_args.push(value);
                    lowered_args.push(match args.get(1) {
                        Some(ResolvedExpr::Array(_)) => LoweredExpr::Null,
                        Some(replacer) => {
                            if let Some(func_id) =
                                json_stringify_function_replacer_id(replacer, self.function_ids)
                            {
                                LoweredExpr::Number(func_id.0 as i32)
                            } else {
                                self.lower_expr(replacer)?
                            }
                        }
                        None => LoweredExpr::Undefined,
                    });
                    lowered_args.push(match args.get(2) {
                        Some(space)
                            if should_ignore_json_stringify_space(space, self.function_ids) =>
                        {
                            LoweredExpr::Undefined
                        }
                        Some(space) => {
                            if let Some(boxed_space) = json_stringify_boxed_space_value(space) {
                                self.lower_expr(boxed_space)?
                            } else {
                                self.lower_expr(space)?
                            }
                        }
                        None => LoweredExpr::Undefined,
                    });
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "JsonStringify".to_owned(),
                        args: lowered_args,
                    })
                } else if is_date_now_live_time_call(object, method) {
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "DateNow".to_owned(),
                        args: vec![],
                    })
                } else if self.is_unsupported_regexp_compile_receiver(object, method) {
                    Err(unsupported_regexp_compile_diagnostic(Some(*span)))
                } else if self.is_object_key_enumeration_leak(object, method, args) {
                    Err(private_storage_observable_access_diagnostic(Some(*span)))
                } else if let Some(regexp_args) = regexp_test_runtime(object, method, args, *span)?
                {
                    let lowered_args = regexp_args
                        .iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "RegExpTest".to_owned(),
                        args: lowered_args,
                    })
                } else if let Some(regexp_args) = regexp_exec_runtime(object, method, args, *span)?
                {
                    let lowered_args = regexp_args
                        .iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "RegExpMatch".to_owned(),
                        args: lowered_args,
                    })
                } else if let Some(regexp_args) =
                    regexp_string_match_runtime(object, method, args, *span)?
                {
                    let lowered_args = regexp_args
                        .iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "RegExpMatch".to_owned(),
                        args: lowered_args,
                    })
                } else if matches!(method.as_str(), "getTime" | "valueOf")
                    && self.is_date_receiver(object)
                {
                    if !args.is_empty() {
                        return Err(Diagnostic {
                            code: DiagCode::ArityMismatch,
                            message: format!(
                                "Date.prototype.{method} expects 0 arguments, got {}",
                                args.len()
                            ),
                            span: Some(*span),
                        });
                    }
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "DateGetTime".to_owned(),
                        args: vec![self.lower_expr(object)?],
                    })
                } else if is_annex_b_date_method(method) && self.is_date_receiver(object) {
                    Err(unsupported_annex_b_date_method_diagnostic(
                        method,
                        Some(*span),
                    ))
                } else if method == "toString" && self.is_date_receiver(object) {
                    Err(unsupported_date_timezone_diagnostic(method, Some(*span)))
                } else if matches!(object.as_ref(), ResolvedExpr::String(_)) {
                    if let Some(diagnostic) = unsupported_annex_b_string_method(method, *span) {
                        Err(diagnostic)
                    } else if let Some(runtime_fn) = resolve_method_to_runtime_fn(object, method) {
                        let mut lowered_args = vec![self.lower_expr(object)?];
                        lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<
                            Result<Vec<_>, _>,
                        >(
                        )?);
                        Ok(LoweredExpr::RuntimeCall {
                            runtime_fn,
                            args: lowered_args,
                        })
                    } else {
                        Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "String.prototype.{method} is not supported in this milestone"
                            ),
                            span: Some(*span),
                        })
                    }
                } else if let Some(runtime_fn) = resolve_method_to_runtime_fn(object, method) {
                    if runtime_fn == "ArrayPush" && args.len() != 1 {
                        if !matches!(object.as_ref(), ResolvedExpr::Ident(_)) {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "issue-271: multi-argument Array.prototype.push is currently supported only for identifier array receivers".to_owned(),
                                span: Some(*span),
                            });
                        }
                        let mut lowered_args = vec![self.lower_expr(object)?];
                        lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<
                            Result<Vec<_>, _>,
                        >(
                        )?);
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "ArrayPushMany".to_owned(),
                            args: lowered_args,
                        });
                    }
                    if (runtime_fn == "MathMax" || runtime_fn == "MathMin") && args.len() > 2 {
                        let mut lowered_args = Vec::new();
                        if !matches!(
                            object.as_ref(),
                            ResolvedExpr::Ident(name) if name == "Math" || name == "JSON" || name == "Object" || name == "String"
                        ) {
                            lowered_args.push(self.lower_expr(object)?);
                        }
                        for arg in args {
                            lowered_args.push(self.lower_expr(arg)?);
                        }
                        let mut result = lowered_args[0].clone();
                        for arg in &lowered_args[1..] {
                            result = LoweredExpr::RuntimeCall {
                                runtime_fn: runtime_fn.clone(),
                                args: vec![result, arg.clone()],
                            };
                        }
                        return Ok(result);
                    }
                    // Handle zero-argument case for Math.max/min
                    // Math.max() with no arguments returns -Infinity (approximated as NUMBER_PAYLOAD_MIN)
                    // Math.min() with no arguments returns +Infinity (approximated as NUMBER_PAYLOAD_MAX)
                    // Note: Proper Infinity support requires broader number-model support (issue-281)
                    if (runtime_fn == "MathMax" || runtime_fn == "MathMin") && args.is_empty() {
                        use ts2wasm_runtime_abi::ValueTag;
                        let infinity_value = if runtime_fn == "MathMax" {
                            // -Infinity approximated as minimum representable number
                            ValueTag::NUMBER_PAYLOAD_MIN
                        } else {
                            // +Infinity approximated as maximum representable number
                            ValueTag::NUMBER_PAYLOAD_MAX
                        };
                        return Ok(LoweredExpr::Number(infinity_value));
                    }
                    let mut lowered_args = Vec::new();
                    let is_static_call = matches!(
                        object.as_ref(),
                        ResolvedExpr::Ident(name) if name == "Math" || name == "JSON" || name == "Object" || name == "String"
                    );
                    if !is_static_call {
                        lowered_args.push(self.lower_expr(object)?);
                    }
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn,
                        args: lowered_args,
                    })
                } else {
                    if let ResolvedExpr::Ident(receiver_name) = object.as_ref()
                        && let Ok(obj_local) = self.resolve_local(receiver_name)
                        && let Some(method_id) = self
                            .object_function_props
                            .get(&obj_local)
                            .and_then(|props| props.get(method))
                            .copied()
                    {
                        let lowered_args = self.lower_function_call_args(
                            method_id,
                            LoweredExpr::Local(obj_local),
                            args,
                        )?;
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    if method == "map"
                        && string_constructor_arrow_callback(args)
                        && self.is_known_array_expr(object)
                    {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "ArrayMapValueToString".to_owned(),
                            args: vec![self.lower_expr(object)?],
                        });
                    }

                    if method == "map"
                        && unary_plus_arrow_callback(args)
                        && self.is_known_array_expr(object)
                    {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "ArrayMapUnaryPlus".to_owned(),
                            args: vec![self.lower_expr(object)?],
                        });
                    }

                    if method == "map"
                        && let ResolvedExpr::Array(_) = object.as_ref()
                    {
                        return self.lower_array_literal_map(object, args, *span);
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
                            runtime_fn: "ArrayMapStringSplit".to_owned(),
                            args: vec![self.lower_expr(object)?, self.lower_expr(separator)?],
                        });
                    }

                    if method == "sort" && self.is_known_array_expr(object) {
                        if numeric_ascending_sort_arrow_callback(args) {
                            return Ok(LoweredExpr::RuntimeCall {
                                runtime_fn: "ArraySortNumeric".to_owned(),
                                args: vec![self.lower_expr(object)?],
                            });
                        }
                        return Err(unsupported_array_sort_diagnostic(Some(*span)));
                    }

                    if (method == "map" && self.is_known_array_expr(object))
                        || is_array_prototype_map_call_receiver(object, method)
                    {
                        return Err(unsupported_array_map_diagnostic(Some(*span)));
                    }

                    if matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                        let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "this.method(...) requires class context".to_owned(),
                            span: Some(*span),
                        })?;
                        let method_id =
                            self.resolve_class_method(class_name, method)
                                .ok_or_else(|| Diagnostic {
                                    code: DiagCode::UnsupportedSyntax,
                                    message: format!(
                                        "method `{}.{}` not found",
                                        class_name, method
                                    ),
                                    span: Some(*span),
                                })?;

                        let mut lowered_args =
                            vec![LoweredExpr::Local(self.resolve_local("this")?)];
                        lowered_args.extend(
                            args.iter()
                                .map(|e| self.lower_expr(e))
                                .collect::<Result<Vec<_>, _>>()?,
                        );
                        self.append_class_method_captures(method_id, &mut lowered_args)?;
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    let receiver_name = match object.as_ref() {
                        ResolvedExpr::Ident(name) => name,
                        _ => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-211: method `{}` requires an identifier receiver",
                                    method
                                ),
                                span: Some(*span),
                            });
                        }
                    };

                    if let Ok(obj_local) = self.resolve_local(receiver_name)
                        && let Some(class_name) = self.local_classes.get(&obj_local)
                        && let Some(runtime_fn) = collection_method_runtime_fn(class_name, method)
                    {
                        if class_name == "RegExp" && args.len() != 1 {
                            return Err(Diagnostic {
                                code: DiagCode::ArityMismatch,
                                message: format!(
                                    "RegExp.prototype.{method} expects 1 argument, got {}",
                                    args.len()
                                ),
                                span: Some(*span),
                            });
                        }
                        let mut lowered_args = vec![LoweredExpr::Local(obj_local)];
                        lowered_args.extend(
                            args.iter()
                                .map(|e| self.lower_expr(e))
                                .collect::<Result<Vec<_>, _>>()?,
                        );
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: runtime_fn.to_owned(),
                            args: lowered_args,
                        });
                    }

                    if receiver_name == "super" {
                        let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super.method(...) requires class context".to_owned(),
                            span: Some(*span),
                        })?;
                        let parent_name = self
                            .class_parents
                            .get(class_name)
                            .and_then(|p| p.clone())
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "super.method(...) used in class without extends"
                                    .to_owned(),
                                span: Some(*span),
                            })?;
                        let method_id = self
                            .resolve_class_method(&parent_name, method)
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "super method `{}.{}` not found",
                                    parent_name, method
                                ),
                                span: Some(*span),
                            })?;

                        let mut lowered_args =
                            vec![LoweredExpr::Local(self.resolve_local("this")?)];
                        lowered_args.extend(
                            args.iter()
                                .map(|e| self.lower_expr(e))
                                .collect::<Result<Vec<_>, _>>()?,
                        );
                        self.append_class_method_captures(method_id, &mut lowered_args)?;
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    if let Some(method_id) = self
                        .class_static_method_ids
                        .get(&(receiver_name.clone(), method.clone()))
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
                        });
                    }

                    if receiver_name == "Object"
                        && method == "getOwnPropertyDescriptor"
                        && self.resolve_local(receiver_name).is_err()
                    {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-291: Object.getOwnPropertyDescriptor is not implemented in the current Object global binding slice".to_owned(),
                            span: Some(*span),
                        });
                    }

                    let obj_local = self.resolve_local(receiver_name)?;

                    let class_name =
                        self.local_classes
                            .get(&obj_local)
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-211: unknown receiver class for method `{}`",
                                    method
                                ),
                                span: Some(*span),
                            })?;

                    let method_id =
                        self.resolve_class_method(class_name, method)
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!("method `{}.{}` not found", class_name, method),
                                span: Some(*span),
                            })?;

                    let mut lowered_args = vec![LoweredExpr::Local(obj_local)];
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);
                    self.append_class_method_captures(method_id, &mut lowered_args)?;

                    Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(method_id),
                        args: lowered_args,
                    })
                }
            }
            ResolvedExpr::PropertyAssign {
                object,
                key,
                value,
                span,
            } => {
                if let ResolvedExpr::Ident(name) = object.as_ref()
                    && let Ok(local_id) = self.resolve_local(name)
                {
                    self.invalidate_static_object_literal_local(local_id);
                }
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(Some(*span)));
                }
                if is_set_prototype_property(object, key, "add") {
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "SetPrototypeAddSet".to_owned(),
                        args: vec![self.lower_set_prototype_add_assignment_value(value)?],
                    });
                }
                if is_set_prototype_property(object, key, "originalAdd")
                    && is_set_prototype_property_expr(value, "add")
                {
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "SetPrototypeAddGet".to_owned(),
                        args: Vec::new(),
                    });
                }
                if key.starts_with('#') {
                    if let Some(local_name) = self.current_static_private_field_local_name(key) {
                        if self.is_same_class_static_private_receiver(object) {
                            let local = self.resolve_local(&local_name).map_err(|_| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-352: static private field `{key}` cannot be accessed before its declaration in class static initialization order"
                                ),
                                span: Some(*span),
                            })?;
                            let expr = Box::new(self.lower_expr(value)?);
                            return Ok(if self.env_cell_locals.contains(&local) {
                                LoweredExpr::EnvCellSet { cell: local, expr }
                            } else {
                                LoweredExpr::Assign { local, expr }
                            });
                        }
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: static private field `{key}` assignment is currently supported only as `this.{key} = value` inside static methods or `Class.{key} = value` inside the declaring class"
                            ),
                            span: Some(*span),
                        });
                    }
                    if let Some(setter_id) = self.current_static_private_setter_id(key) {
                        if self.is_same_class_static_private_receiver(object) {
                            return Ok(LoweredExpr::Call {
                                kind: FunctionCallKind::User(setter_id),
                                args: vec![self.lower_expr(value)?],
                            });
                        }
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: static private setter `{key}` assignment is currently supported only as `this.{key} = value` inside static methods or `Class.{key} = value` inside the declaring class"
                            ),
                            span: Some(*span),
                        });
                    }
                    if let Some(setter_id) = self.current_private_setter_id(key) {
                        if !matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-255: private setter `{key}` assignment is currently supported only as `this.{key} = value` inside the declaring class"
                                ),
                                span: Some(*span),
                            });
                        }
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(setter_id),
                            args: vec![
                                LoweredExpr::Local(self.resolve_local("this")?),
                                self.lower_expr(value)?,
                            ],
                        });
                    }
                    if let Some(class_name) = self.infer_class_for_expr(object)
                        && self.private_setter_id_for_class(&class_name, key).is_some()
                    {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private setter `{key}` external assignment is not supported in this private setter runtime slice"
                            ),
                            span: Some(*span),
                        });
                    }
                    let slot = self.private_field_slot(object, key, *span)?;
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "PrivateFieldSet".to_owned(),
                        args: vec![
                            self.lower_expr(object)?,
                            LoweredExpr::Number(slot as i32),
                            self.lower_expr(value)?,
                        ],
                    });
                }
                Ok(LoweredExpr::PropertySet {
                    object: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                    value: Box::new(self.lower_expr(value)?),
                })
            }
            ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
                if let ResolvedExpr::Ident(name) = object.as_ref()
                    && let Ok(local_id) = self.resolve_local(name)
                {
                    self.invalidate_static_object_literal_local(local_id);
                    self.update_static_function_array_like_index(local_id, key, value);
                }
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::PropertySetDynamic {
                    object: Box::new(self.lower_expr(object)?),
                    index: Box::new(self.lower_expr(key)?),
                    value: Box::new(self.lower_expr(value)?),
                })
            }
            ResolvedExpr::New {
                class_name,
                args,
                span: _,
            } => {
                if class_name == "RegExp" {
                    return Ok(LoweredExpr::String(regexp_constructor_literal(args)?));
                }
                if class_name == "Date" {
                    if args.is_empty() {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "DateNewLive".to_owned(),
                            args: vec![],
                        });
                    }
                    if args.len() != 1 {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-050: only deterministic new Date(<epoch-ms integer>) is supported in this slice"
                                .to_owned(),
                            span: None,
                        });
                    }
                    let epoch_ms = &args[0];
                    if !is_date_constructor_epoch_arg(epoch_ms) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-050: Date constructor currently requires an integer epoch millisecond literal".to_owned(),
                            span: None,
                        });
                    }
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "DateNew".to_owned(),
                        args: vec![self.lower_expr(epoch_ms)?],
                    });
                }
                if class_name == "Map" || class_name == "Set" {
                    if args.is_empty() {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: format!("{class_name}New"),
                            args: Vec::new(),
                        });
                    }
                    if class_name == "Set" && args.len() == 1 && self.is_known_array_expr(&args[0])
                    {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "SetFromArray".to_owned(),
                            args: vec![self.lower_expr(&args[0])?],
                        });
                    }
                    if class_name == "Set" {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-276: new Set(iterable) currently supports only known dense array inputs".to_owned(),
                            span: None,
                        });
                    }
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!("issue-049: new {class_name}(iterable) is not supported yet"),
                        span: None,
                    });
                }
                if let Some(constructor) = BuiltinErrorConstructor::from_name(class_name) {
                    let message = match args.first() {
                        Some(message) => LoweredExpr::RuntimeCall {
                            runtime_fn: "ErrorMessage".to_owned(),
                            args: vec![self.lower_expr(message)?],
                        },
                        None => LoweredExpr::String(String::new()),
                    };
                    return Ok(LoweredExpr::ErrorNew {
                        constructor,
                        message: Box::new(message),
                    });
                }

                let prototype = self.class_prototype_ref(class_name)?;

                let lowered_args = args
                    .iter()
                    .map(|arg| self.lower_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(LoweredExpr::New {
                    constructor: prototype.constructor,
                    prototype,
                    args: lowered_args,
                    base_local: self.alloc_temp(),
                    private_slot_count: self.private_slot_count(class_name),
                })
            }
            ResolvedExpr::ModuleLoad { specifier } => Ok(LoweredExpr::ModuleLoad {
                module_id: self.module_id_for_specifier(specifier),
            }),
            ResolvedExpr::ArrowFn { params, body } => self.lower_arrow_fn(params, body),
            ResolvedExpr::FunctionExpr { name, params, body } => {
                self.lower_named_function_expr(name, params, body)
            }
        }
    }

}
