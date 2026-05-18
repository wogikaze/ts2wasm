use super::super::*;
mod assignment;
mod binary;
mod binding;
mod control;
pub(crate) mod facts;
mod literal;
mod property;
mod ternary;
mod unary;

use crate::builtin_resolved::{ResolvedExpr, ResolvedStmt};
use std::collections::{HashMap, HashSet};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::Resolver {
    pub(crate) fn lower_expr(&mut self, expr: &ResolvedExpr) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            // Literals (trivial constructors)
            ResolvedExpr::Number(value) => Ok(LoweredExpr::Number(*value, Span::generated("num"))),
            ResolvedExpr::DecimalNumber(value) => Ok(LoweredExpr::DecimalNumber(
                value.clone(),
                Span::generated("num"),
            )),
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
                span: Span::generated("bigint"),
            }),
            ResolvedExpr::String(value) => {
                Ok(LoweredExpr::String(value.clone(), Span::generated("str")))
            }
            ResolvedExpr::Bool(value) => Ok(LoweredExpr::Bool(*value, Span::generated("bool"))),
            ResolvedExpr::Null => Ok(LoweredExpr::Null(Span::generated("null"))),
            ResolvedExpr::Undefined => Ok(LoweredExpr::Undefined(Span::generated("undef"))),

            // Control flow
            ResolvedExpr::Await { expr } => self.lower_await_expr(expr),
            ResolvedExpr::Yield { expr, delegate } => {
                if *delegate {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "yield* delegation is parsed but not lowered yet".to_owned(),
                        span: Some(Span::generated("yield_star")),
                        phase: None,
                    });
                }
                expr.as_ref()
                    .map(|expr| self.lower_expr(expr))
                    .transpose()
                    .map(|expr| {
                        expr.unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("yield")))
                    })
            }
            ResolvedExpr::This { .. } => self.lower_this_expr(),
            ResolvedExpr::NewTarget { span } => self.lower_new_target_expr(*span),
            ResolvedExpr::ImportMeta { span } => self.lower_import_meta_expr(*span),
            ResolvedExpr::Ident(name) => self.lower_ident_expr(name),
            ResolvedExpr::Spread(_) => self.lower_spread_expr(),

            // Unary / Binary / Ternary
            ResolvedExpr::Unary { op, expr } => self.lower_unary_expr(op, expr),
            ResolvedExpr::Binary { left, op, right } => self.lower_binary_expr(left, op, right),
            ResolvedExpr::Ternary {
                condition,
                then_expr,
                else_expr,
                ..
            } => self.lower_ternary_expr(condition, then_expr, else_expr),

            // Assignment expressions
            ResolvedExpr::Assign { name, expr } => self.lower_assign_expr(name, expr),
            ResolvedExpr::LogicalAssign { name, op, expr } => {
                self.lower_logical_assign_expr(name, op, expr)
            }
            ResolvedExpr::LogicalPropertyAssign {
                object,
                key,
                op,
                expr,
            } => self.lower_logical_property_assign_expr(object, key, op, expr),
            ResolvedExpr::LogicalComputedPropertyAssign {
                object,
                key,
                op,
                expr,
            } => self.lower_logical_computed_property_assign_expr(object, key, op, expr),
            ResolvedExpr::LogicalComputedMemberAssign {
                object,
                key,
                op,
                expr,
            } => self.lower_logical_computed_member_assign_expr(object, key, op, expr),
            ResolvedExpr::LogicalMemberAssign {
                object,
                key,
                op,
                expr,
            } => self.lower_logical_member_assign_expr(object, key, op, expr),
            ResolvedExpr::PropertyAssign {
                object,
                key,
                value,
                span,
            } => self.lower_property_assign_expr(object, key, value, *span),
            ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
                self.lower_property_assign_dynamic_expr(object, key, value)
            }

            // Property access
            ResolvedExpr::PropertyAccess { object, key, span } => {
                self.lower_property_access_expr(object, key, *span)
            }
            ResolvedExpr::OptionalPropertyAccess { object, key, .. } => {
                self.lower_optional_property_access_expr(object, key)
            }
            ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
                self.lower_optional_computed_index_expr(object, index)
            }
            ResolvedExpr::ComputedIndex { object, index } => {
                self.lower_computed_index_expr(object, index)
            }

            // Delegations to other modules
            ResolvedExpr::OptionalCall { callee, args, span } => {
                self.lower_optional_call(callee, args, *span)
            }
            ResolvedExpr::Call { callee, args, span } => self.lower_call_expr(callee, args, *span),
            ResolvedExpr::BuiltinCall { builtin, args } => {
                self.lower_builtin_call_expr(*builtin, args)
            }
            ResolvedExpr::BuiltinProperty {
                builtin,
                object,
                span,
            } => self.lower_builtin_property_expr(*builtin, object, *span),
            ResolvedExpr::Array(elements) => self.lower_array_literal(elements),
            ResolvedExpr::Object(props) => self.lower_object_literal_expr(props),
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                span,
            } => self.lower_method_call_expr(object, method, args, *span),
            ResolvedExpr::New {
                class_name,
                args,
                span,
            } => self.lower_new_expr(class_name, args, *span),
            ResolvedExpr::ModuleLoad {
                specifier,
                is_dynamic_import,
            } => Ok(LoweredExpr::ModuleLoad {
                module_id: self.module_id_for_specifier(specifier),
                // ModuleLoadDynamic: dynamic import() keeps async module-load identity in lowered IR.
                kind: if *is_dynamic_import {
                    ModuleLoadKind::DynamicImport
                } else {
                    ModuleLoadKind::StaticRequire
                },
                span: Span::generated("module_load"),
            }),
            ResolvedExpr::ArrowFn {
                params,
                body,
                body_stmts,
                ..
            } => self.lower_arrow_fn(params, body, body_stmts),
            ResolvedExpr::FunctionExpr {
                name,
                params,
                body,
                is_generator,
                origin,
                ..
            } => self.lower_named_function_expr(name, params, body, *is_generator, *origin),
            ResolvedExpr::ClassExpr { .. } => Ok(LoweredExpr::Undefined(Span::generated("undef"))),
            ResolvedExpr::Sequence(exprs) => self.lower_sequence_expr(exprs),
            ResolvedExpr::EvalCompletion(steps) => self.lower_eval_completion_expr(steps),
            ResolvedExpr::Eval {
                kind, source, span, ..
            } => {
                // Emit a runtime call for eval — the host shim handles execution.
                let source_expr = if let crate::builtin_resolved::EvalSource::StaticLiteral(s) =
                    &source
                {
                    LoweredExpr::String(s.clone(), Span::generated("eval"))
                } else if let crate::builtin_resolved::EvalSource::Runtime(source_expr) = &source {
                    self.lower_expr(source_expr)?
                } else {
                    LoweredExpr::Undefined(Span::generated("eval"))
                };
                let intrinsic = match kind {
                    crate::builtin_resolved::EvalKind::Direct => RuntimeFn::EvalDirectHost,
                    crate::builtin_resolved::EvalKind::Indirect => RuntimeFn::EvalIndirectHost,
                };
                let args = if matches!(kind, crate::builtin_resolved::EvalKind::Direct) {
                    if matches!(source, crate::builtin_resolved::EvalSource::Runtime(_)) {
                        self.ensure_direct_eval_env_descriptor_initialized(*span)?;
                    }
                    vec![source_expr, self.lower_direct_eval_env_descriptor()]
                } else {
                    vec![source_expr]
                };
                Ok(LoweredExpr::RuntimeCall {
                    intrinsic,
                    args,
                    span: Span::generated("eval"),
                })
            }
        }
    }

    pub(super) fn lower_import_meta_expr(&mut self, span: Span) -> Result<LoweredExpr, Diagnostic> {
        Ok(LoweredExpr::ObjectNew {
            props: vec![("url".to_owned(), self.lower_module_meta_url(span))],
            non_enumerable: 0,
            span,
        })
    }

    pub(super) fn lower_module_meta_url(&self, span: Span) -> LoweredExpr {
        // ModuleMetaUrl: import.meta.url resolves to the active module URL/specifier.
        LoweredExpr::String(self.ctx.current_module_url.clone(), span)
    }

    fn lower_sequence_expr(&mut self, exprs: &[ResolvedExpr]) -> Result<LoweredExpr, Diagnostic> {
        let mut stmts = Vec::new();
        let len = exprs.len();
        for (i, expr) in exprs.iter().enumerate() {
            let lowered = self.lower_expr(expr)?;
            if i < len - 1 {
                stmts.push(LoweredStmt::Expr(lowered, Span::generated("seq")));
            } else {
                return Ok(LoweredExpr::Block {
                    stmts,
                    result: Box::new(lowered),
                    span: Span::generated("seq"),
                });
            }
        }
        // Single-element sequence (shouldn't happen but handle gracefully)
        unreachable!("sequence with zero elements")
    }

    fn lower_eval_completion_expr(
        &mut self,
        steps: &[crate::builtin_resolved::EvalCompletionStep],
    ) -> Result<LoweredExpr, Diagnostic> {
        use crate::builtin_resolved::EvalCompletionStep;

        let completion = self.alloc_temp();
        self.ctx.symbols.scopes.push(HashMap::new());
        let lowered = (|| {
            let mut stmts = vec![LoweredStmt::Let(
                completion,
                LoweredExpr::Undefined(Span::generated("eval_completion_init")),
                Span::generated("eval_completion_let"),
            )];
            for step in steps {
                match step {
                    EvalCompletionStep::Value(expr) => {
                        let value = self.lower_expr(expr)?;
                        stmts.push(LoweredStmt::Assign(
                            completion,
                            value,
                            Span::generated("eval_completion_set"),
                        ));
                    }
                    EvalCompletionStep::Empty(Some(expr)) => {
                        let side_effect = self.lower_expr(expr)?;
                        stmts.push(LoweredStmt::Expr(
                            side_effect,
                            Span::generated("eval_completion_empty"),
                        ));
                    }
                    EvalCompletionStep::LexicalLet { name, init } => {
                        stmts
                            .push(self.lower_stmt(&ResolvedStmt::Let(name.clone(), init.clone()))?);
                    }
                    EvalCompletionStep::Empty(None) => {}
                }
            }
            Ok(LoweredExpr::Block {
                stmts,
                result: Box::new(LoweredExpr::Local(
                    completion,
                    Span::generated("eval_completion_result"),
                )),
                span: Span::generated("eval_completion"),
            })
        })();
        self.ctx.symbols.scopes.pop();
        lowered
    }

    fn lower_direct_eval_env_descriptor(&self) -> LoweredExpr {
        let mut seen = HashSet::new();
        let mut elements = Vec::new();
        for scope in self.ctx.symbols.scopes.iter().rev() {
            let mut bindings = scope.iter().collect::<Vec<_>>();
            bindings.sort_by(|(left, _), (right, _)| left.cmp(right));
            for (name, local) in bindings {
                if !seen.insert(name.clone()) {
                    continue;
                }
                if self.ctx.facts.env_cell_locals.contains(local)
                    && self.ctx.facts.initialized_env_cell_locals.contains(local)
                {
                    elements.push(LoweredExpr::String(
                        name.clone(),
                        Span::generated("eval_env_name"),
                    ));
                    elements.push(LoweredExpr::Local(*local, Span::generated("eval_env_cell")));
                }
            }
        }

        if elements.is_empty() {
            LoweredExpr::Undefined(Span::generated("eval_env"))
        } else {
            LoweredExpr::ArrayNew {
                elements,
                span: Span::generated("eval_env"),
            }
        }
    }

    fn ensure_direct_eval_env_descriptor_initialized(&self, span: Span) -> Result<(), Diagnostic> {
        let mut names = self
            .ctx
            .facts
            .env_cell_names
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        names.sort();

        for name in names {
            let Some(local) = self.ctx.symbols.resolve(&name) else {
                return Err(dynamic_direct_eval_tdz_diagnostic(&name, span));
            };
            if !self.ctx.facts.env_cell_locals.contains(&local)
                || !self.ctx.facts.initialized_env_cell_locals.contains(&local)
            {
                return Err(dynamic_direct_eval_tdz_diagnostic(&name, span));
            }
        }

        Ok(())
    }
}

fn dynamic_direct_eval_tdz_diagnostic(name: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedEval,
        message: format!(
            "issue-429: dynamic direct eval cannot safely run before caller binding `{name}` is initialized; TDZ-aware env descriptors are not implemented"
        ),
        span: Some(span),
        phase: None,
    }
}

/// Check if a name is a known global builtin function (for metadata queries).
pub(super) fn is_global_builtin_function_name(name: &str) -> bool {
    matches!(
        name,
        "escape"
            | "unescape"
            | "isNaN"
            | "parseInt"
            | "parseFloat"
            | "isFinite"
            | "encodeURI"
            | "decodeURI"
            | "encodeURIComponent"
            | "decodeURIComponent"
            | "structuredClone"
            | "queueMicrotask"
    )
}

pub(super) fn lower_global_builtin_function_metadata_property(
    name: &str,
    key: &str,
) -> Result<LoweredExpr, Diagnostic> {
    match key {
        "name" => Ok(LoweredExpr::String(name.to_owned(), Span::generated("str"))),
        "length" => Ok(LoweredExpr::Number(
            global_builtin_function_length(name),
            Span::generated("num"),
        )),
        _ => unreachable!("caller filters global builtin function metadata property"),
    }
}

fn global_builtin_function_length(name: &str) -> i32 {
    match name {
        "parseInt" => 2,
        "escape" | "unescape" | "isNaN" | "parseFloat" | "isFinite" | "encodeURI" | "decodeURI"
        | "encodeURIComponent" | "decodeURIComponent" => 1,
        "structuredClone" => 1,
        "queueMicrotask" => 1,
        _ => 0,
    }
}
