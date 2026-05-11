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
use ts2wasm_shared::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl<'a> super::super::Resolver {
    pub(crate) fn lower_set_for_each_method(
        &mut self,
        receiver: LoweredExpr,
        _resolved_receiver: &ResolvedExpr,
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
                if params.len() > 3 {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "Set.prototype.forEach callbacks with more than 3 parameters are not supported"
                                .to_owned(),
                        span: Some(span),
                        phase: None,
                    });
                }
                let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = self.lower_arrow_fn(params, body, body_stmts)?
                else {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "failed to lower Set.prototype.forEach arrow callback".to_owned(),
                        span: Some(span),
                        phase: None,
                    });
                };
                (func_id, captures, params.len())
            }
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "non-arrow-function callbacks are not yet supported for Set.prototype.forEach"
                            .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            }
        };

        let receiver_local = match &receiver {
            LoweredExpr::Local(id, _) => *id,
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "non-identifier receiver not yet supported for Set.prototype.forEach"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            }
        };

        let values = self.alloc_temp();
        let values_len = self.alloc_temp();
        let i = self.alloc_temp();

        let mut stmts = Vec::new();

        // values = RuntimeCall("SetValuesArray", [receiver])
        stmts.push(LoweredStmt::Let(
            values,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetValuesArray,
                args: vec![LoweredExpr::Local(receiver_local, Span::generated("local"))],
                span: Span::generated("runtime_call"),
            },
            Span::generated("Let"),
        ));

        // values_len = GetLength(values)
        stmts.push(LoweredStmt::Let(
            values_len,
            LoweredExpr::GetLength(
                Box::new(LoweredExpr::Local(values, Span::generated("local"))),
                Span::generated("get_length"),
            ),
            Span::generated("Let"),
        ));

        let mut while_body = Vec::new();

        let val = self.alloc_temp();

        // val = ArrayGet(values, i)
        while_body.push(LoweredStmt::Let(
            val,
            LoweredExpr::ArrayGet {
                arr: Box::new(LoweredExpr::Local(values, Span::generated("local"))),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        // Call callback(value, value, set) — key === value per spec
        let call_args = {
            let explicit_args = vec![
                LoweredExpr::Local(val, Span::generated("local")),
                LoweredExpr::Local(val, Span::generated("local")),
                LoweredExpr::Local(receiver_local, Span::generated("local")),
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

        // i += 1
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

        // i = 0
        stmts.push(LoweredStmt::Let(
            i,
            LoweredExpr::Number(0, Span::generated("num")),
            Span::generated("Let"),
        ));

        // While(i < values_len, body)
        stmts.push(LoweredStmt::While {
            condition: LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Less,
                right: Box::new(LoweredExpr::Local(values_len, Span::generated("local"))),
                span: Span::generated("binary"),
            },
            body: while_body,
            span: Span::generated("while"),
        });

        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
            span: Span::generated("block"),
        })
    }

    pub(crate) fn lower_map_for_each_method(
        &mut self,
        receiver: LoweredExpr,
        _resolved_receiver: &ResolvedExpr,
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
                if params.len() > 3 {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "Map.prototype.forEach callbacks with more than 3 parameters are not supported"
                                .to_owned(),
                        span: Some(span),
                        phase: None,
                    });
                }
                let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = self.lower_arrow_fn(params, body, body_stmts)?
                else {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "failed to lower Map.prototype.forEach arrow callback".to_owned(),
                        span: Some(span),
                        phase: None,
                    });
                };
                (func_id, captures, params.len())
            }
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "non-arrow-function callbacks are not yet supported for Map.prototype.forEach"
                            .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            }
        };

        let receiver_local = match &receiver {
            LoweredExpr::Local(id, _) => *id,
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "non-identifier receiver not yet supported for Map.prototype.forEach"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            }
        };

        let entries = self.alloc_temp();
        let entries_len = self.alloc_temp();
        let i = self.alloc_temp();

        let mut stmts = Vec::new();

        // entries = RuntimeCall("MapEntriesArray", [receiver])
        stmts.push(LoweredStmt::Let(
            entries,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapEntriesArray,
                args: vec![LoweredExpr::Local(receiver_local, Span::generated("local"))],
                span: Span::generated("runtime_call"),
            },
            Span::generated("Let"),
        ));

        // entries_len = GetLength(entries)
        stmts.push(LoweredStmt::Let(
            entries_len,
            LoweredExpr::GetLength(
                Box::new(LoweredExpr::Local(entries, Span::generated("local"))),
                Span::generated("get_length"),
            ),
            Span::generated("Let"),
        ));

        let mut while_body = Vec::new();

        let key = self.alloc_temp();
        let val = self.alloc_temp();

        // key = ArrayGet(entries, i)
        while_body.push(LoweredStmt::Let(
            key,
            LoweredExpr::ArrayGet {
                arr: Box::new(LoweredExpr::Local(entries, Span::generated("local"))),
                index: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        // val = ArrayGet(entries, i + 1)
        while_body.push(LoweredStmt::Let(
            val,
            LoweredExpr::ArrayGet {
                arr: Box::new(LoweredExpr::Local(entries, Span::generated("local"))),
                index: Box::new(LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                    op: LoweredBinaryOp::Add,
                    right: Box::new(LoweredExpr::Number(1, Span::generated("num"))),
                    span: Span::generated("binary"),
                }),
                span: Span::generated("array_get"),
            },
            Span::generated("Let"),
        ));

        // Call callback(value, key, map) — value is first arg per spec
        let call_args = {
            let explicit_args = vec![
                LoweredExpr::Local(val, Span::generated("local")),
                LoweredExpr::Local(key, Span::generated("local")),
                LoweredExpr::Local(receiver_local, Span::generated("local")),
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

        // i += 2
        while_body.push(LoweredStmt::Assign(
            i,
            LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Add,
                right: Box::new(LoweredExpr::Number(2, Span::generated("num"))),
                span: Span::generated("binary"),
            },
            Span::generated("Assign"),
        ));

        // i = 0
        stmts.push(LoweredStmt::Let(
            i,
            LoweredExpr::Number(0, Span::generated("num")),
            Span::generated("Let"),
        ));

        // While(i < entries_len, body)
        stmts.push(LoweredStmt::While {
            condition: LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(i, Span::generated("local"))),
                op: LoweredBinaryOp::Less,
                right: Box::new(LoweredExpr::Local(entries_len, Span::generated("local"))),
                span: Span::generated("binary"),
            },
            body: while_body,
            span: Span::generated("while"),
        });

        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
            span: Span::generated("block"),
        })
    }
}
