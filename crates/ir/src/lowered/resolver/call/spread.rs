use super::super::{
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
use ts2wasm_syntax::SYMBOL_ITERATOR_OBJECT_KEY;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl<'a> super::super::Resolver {
    pub(crate) fn lower_function_call_args(
        &mut self,
        func_id: FuncId,
        receiver: LoweredExpr,
        args: &[ResolvedExpr],
    ) -> Result<Vec<LoweredExpr>, Diagnostic> {
        let signature = self
            .ctx
            .symbols
            .function_signatures
            .get(&func_id)
            .copied()
            .unwrap_or_default();
        let explicit_args = if !signature.has_rest && !signature.needs_arguments {
            if let Some(local_id) = self.single_dense_array_local_spread_arg(args) {
                (0..signature.explicit_params)
                    .map(|index| LoweredExpr::ArrayGet {
                        arr: Box::new(LoweredExpr::Local(local_id, Span::generated("local"))),
                        index: Box::new(LoweredExpr::Number(index as i32, Span::generated("num"))),

                        span: Span::generated("array_get"),
                    })
                    .collect()
            } else if let Some(local_id) = self.single_set_local_spread_arg(args) {
                (0..signature.explicit_params)
                    .map(|index| LoweredExpr::ArrayGet {
                        arr: Box::new(LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::SetValuesArray,
                            args: vec![LoweredExpr::Local(local_id, Span::generated("local"))],

                            span: Span::generated("runtime_call"),
                        }),
                        index: Box::new(LoweredExpr::Number(index as i32, Span::generated("num"))),

                        span: Span::generated("array_get"),
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
            lowered_args.extend(
                explicit_args
                    .iter()
                    .take(signature.explicit_params)
                    .cloned(),
            );
            for _ in explicit_args.len()..signature.explicit_params {
                lowered_args.push(LoweredExpr::Undefined(Span::generated("undef")));
            }
        }

        if signature.needs_arguments {
            let argument_count = explicit_args.len();
            let mut props = explicit_args
                .into_iter()
                .enumerate()
                .map(|(index, arg)| (index.to_string(), arg))
                .collect::<Vec<_>>();
            let length_index = props.len(); // index of "length" after push
            props.push((
                "length".to_owned(),
                LoweredExpr::Number(argument_count as i32, Span::generated("num")),
            ));
            lowered_args.push(LoweredExpr::ObjectNew {
                props,
                non_enumerable: 1 << length_index, // length is non-enumerable
                span: Span::generated("object_new"),
            });
        }

        self.append_function_captures(func_id, &mut lowered_args)?;

        Ok(lowered_args)
    }

    pub(crate) fn single_dense_array_local_spread_arg(
        &self,
        args: &[ResolvedExpr],
    ) -> Option<LocalId> {
        let [ResolvedExpr::Spread(spread_expr)] = args else {
            return None;
        };
        let ResolvedExpr::Ident(name) = spread_expr.as_ref() else {
            return None;
        };
        let local_id = self.resolve_local(name).ok()?;
        if self.ctx.facts.array_locals.contains(&local_id)
            && !self.ctx.facts.env_cell_locals.contains(&local_id)
        {
            Some(local_id)
        } else {
            None
        }
    }

    pub(crate) fn single_set_local_spread_arg(&self, args: &[ResolvedExpr]) -> Option<LocalId> {
        let [ResolvedExpr::Spread(spread_expr)] = args else {
            return None;
        };
        let ResolvedExpr::Ident(name) = spread_expr.as_ref() else {
            return None;
        };
        let local_id = self.resolve_local(name).ok()?;
        if self.ctx.facts.env_cell_locals.contains(&local_id) {
            return None;
        }
        self.ctx
            .classes
            .local_classes
            .get(&local_id)
            .is_some_and(|class_name| class_name == "Set")
            .then_some(local_id)
    }

    pub(crate) fn append_function_captures(
        &self,
        func_id: FuncId,
        lowered_args: &mut Vec<LoweredExpr>,
    ) -> Result<(), Diagnostic> {
        let Some(captures) = self.ctx.functions.function_captures.get(&func_id) else {
            return Ok(());
        };
        let mutable_captures = self
            .ctx
            .functions
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

                phase: None,})?;
            if mutable_captures.contains(capture)
                && !self.ctx.facts.env_cell_locals.contains(&local)
            {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-404: mutable callback capture `{capture}` is not available as an environment cell at this call site"
                    ),
                    span: None,

                    phase: None,
                });
            }
            lowered_args.push(LoweredExpr::Local(local, Span::generated("local")));
        }

        Ok(())
    }
    pub(crate) fn lower_string_match_all_literal(
        &mut self,
        object: &ResolvedExpr,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if args.len() != 1 {
            return Err(Diagnostic {
                code: DiagCode::ArityMismatch,
                message: format!(
                    "String.prototype.matchAll expects 1 argument, got {}",
                    args.len()
                ),
                span: Some(span),

                phase: None,
            });
        }

        let Some(input) = self.resolved_expr_static_string_value(object) else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-5129: String.prototype.matchAll currently requires a static string receiver"
                    .to_owned(),
                span: Some(span),

                phase: None,});
        };
        if !input.is_ascii() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-5129: String.prototype.matchAll currently supports ASCII input only"
                        .to_owned(),
                span: Some(span),

                phase: None,
            });
        }

        let ResolvedExpr::String(raw_pattern) = &args[0] else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-5129: String.prototype.matchAll currently requires a RegExp literal argument"
                    .to_owned(),
                span: Some(span),

                phase: None,});
        };
        if !looks_like_regexp_literal(raw_pattern) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-5129: String.prototype.matchAll currently requires a RegExp literal argument"
                    .to_owned(),
                span: Some(span),

                phase: None,});
        }
        validate_regexp_plain_literal(raw_pattern, "String.prototype.matchAll literal")?;
        let delimiter = raw_pattern
            .rfind('/')
            .expect("regexp literal has delimiter");
        let flags = &raw_pattern[delimiter + 1..];
        if !flags.contains('g') {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-5129: String.prototype.matchAll requires a global RegExp literal in this slice"
                    .to_owned(),
                span: Some(span),

                phase: None,});
        }

        let pattern = &raw_pattern[1..delimiter];
        let mut elements = Vec::new();
        for (index, ch) in input.char_indices() {
            let matches = match pattern {
                r"\w" => ch.is_ascii_alphanumeric() || ch == '_',
                "." => ch != '\n' && ch != '\r',
                literal if literal.len() == 1 => literal.as_bytes()[0] == ch as u8,
                _ => {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "issue-5129: String.prototype.matchAll currently supports /\\w/g, /./g, and one-byte literal patterns"
                                .to_owned(),
                        span: Some(span),

                        phase: None,});
                }
            };
            if matches {
                elements.push(LoweredExpr::ObjectNew {
                    props: vec![
                        (
                            "0".to_owned(),
                            LoweredExpr::String(ch.to_string(), Span::generated("str")),
                        ),
                        (
                            "index".to_owned(),
                            LoweredExpr::Number(index as i32, Span::generated("num")),
                        ),
                        (
                            "input".to_owned(),
                            LoweredExpr::String(input.clone(), Span::generated("str")),
                        ),
                    ],
                    non_enumerable: 0,

                    span: Span::generated("object_new"),
                });
            }
        }

        Ok(LoweredExpr::ArrayNew {
            elements,
            span: Span::generated("array_new"),
        })
    }

    pub(crate) fn lower_native_set_add_call(
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

                phase: None,
            });
        }
        Ok(LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::SetAdd,
            args: vec![self.lower_expr(&args[0])?, self.lower_expr(&args[1])?],

            span: Span::generated("runtime_call"),
        })
    }

    pub(crate) fn lower_call_args(
        &mut self,
        args: &[ResolvedExpr],
    ) -> Result<Vec<LoweredExpr>, Diagnostic> {
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
                                    lowered_args
                                        .push(LoweredExpr::Undefined(Span::generated("undef")));
                                }
                            }
                        }
                    } else if let Some(value) =
                        self.static_string_spread_value(spread_expr.as_ref())
                    {
                        lowered_args.extend(Self::lower_ascii_string_spread_chars(&value)?);
                    } else if self.is_generator_call_spread_operand(spread_expr.as_ref()) {
                        return Err(Self::unsupported_generator_spread_diagnostic());
                    } else if self.resolved_expr_has_symbol_iterator_property(spread_expr.as_ref())
                    {
                        return Err(Self::unsupported_symbol_iterator_spread_diagnostic());
                    } else if let Some(map_array) =
                        self.lower_map_spread_operand(spread_expr.as_ref())?
                    {
                        lowered_args.push(map_array);
                    } else {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message:
                                "issue-274: spread arguments are only supported for literal arrays and ASCII literal-derived strings in this milestone"
                                    .to_owned(),
                            span: None,

                            phase: None,});
                    }
                }
                _ => lowered_args.push(self.lower_expr(arg)?),
            }
        }
        Ok(lowered_args)
    }

    pub(crate) fn lower_spread_via_iterator(
        &mut self,
        spread_expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        let sentinel_key = SYMBOL_ITERATOR_OBJECT_KEY.to_owned();
        let span = Span::generated("spread_via_iterator");
        let iterable = self.lower_expr(spread_expr)?;
        let iter_fn = self.alloc_temp();
        let iterator = self.alloc_temp();
        let result_arr = self.alloc_temp();
        let done_val = self.alloc_temp();
        let mut stmts = vec![LoweredStmt::Let(
            iter_fn,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(iterable),
                key: Box::new(LoweredExpr::String(sentinel_key, Span::generated("str"))),
                span,
            },
            span,
        )];
        stmts.push(LoweredStmt::Let(
            iterator,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::HeapClosureCall,
                args: vec![LoweredExpr::Local(iter_fn, Span::generated("local"))],
                span,
            },
            span,
        ));
        stmts.push(LoweredStmt::Let(
            result_arr,
            LoweredExpr::ArrayNew {
                elements: vec![],
                span,
            },
            span,
        ));
        stmts.push(LoweredStmt::Let(
            done_val,
            LoweredExpr::Bool(false, Span::generated("bool")),
            span,
        ));
        let next_fn = self.alloc_temp();
        let r = self.alloc_temp();
        let value = self.alloc_temp();
        let mut body = Vec::new();
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
        body.push(LoweredStmt::Let(
            r,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::HeapClosureCall,
                args: vec![LoweredExpr::Local(next_fn, Span::generated("local"))],
                span,
            },
            span,
        ));
        body.push(LoweredStmt::Let(
            done_val,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(r, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "done".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        ));
        let mut push_body = vec![LoweredStmt::Let(
            value,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(r, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "value".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        )];
        push_body.push(LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayPush,
                args: vec![
                    LoweredExpr::Local(result_arr, Span::generated("local")),
                    LoweredExpr::Local(value, Span::generated("local")),
                ],
                span,
            },
            span,
        ));
        body.push(LoweredStmt::If {
            condition: LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Local(done_val, Span::generated("local"))),
                span,
            },
            then_body: push_body,
            else_body: vec![],
            span,
        });
        stmts.push(LoweredStmt::DoWhile {
            body,
            condition: LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Local(done_val, Span::generated("local"))),
                span,
            },
            span,
        });
        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(LoweredExpr::Local(result_arr, Span::generated("local"))),
            span,
        })
    }

    pub(crate) fn lower_for_of_via_iterator(
        &mut self,
        var_id: LocalId,
        iter_expr: &ResolvedExpr,
        body_stmts: &[ResolvedStmt],
    ) -> Result<LoweredStmt, Diagnostic> {
        let sentinel_key = SYMBOL_ITERATOR_OBJECT_KEY.to_owned();
        let span = Span::generated("for_of_via_iterator");
        let iterable = self.lower_expr(iter_expr)?;
        let iter_fn = self.alloc_temp();
        let iterator = self.alloc_temp();
        let done_val = self.alloc_temp();
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
            LoweredStmt::Let(
                done_val,
                LoweredExpr::Bool(false, Span::generated("bool")),
                span,
            ),
        ];
        let next_fn = self.alloc_temp();
        let r = self.alloc_temp();
        let mut body = Vec::new();
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
        body.push(LoweredStmt::Let(
            r,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::HeapClosureCall,
                args: vec![LoweredExpr::Local(next_fn, Span::generated("local"))],
                span,
            },
            span,
        ));
        body.push(LoweredStmt::Let(
            done_val,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(r, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "done".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        ));
        let mut if_body = vec![LoweredStmt::Assign(
            var_id,
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(r, Span::generated("local"))),
                key: Box::new(LoweredExpr::String(
                    "value".to_owned(),
                    Span::generated("str"),
                )),
                span,
            },
            span,
        )];
        if_body.extend(self.lower_nested_block(body_stmts)?);
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
        stmts.push(LoweredStmt::DoWhile {
            body,
            condition: LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Local(done_val, Span::generated("local"))),
                span,
            },
            span,
        });
        Ok(LoweredStmt::Block(stmts, span))
    }
}
