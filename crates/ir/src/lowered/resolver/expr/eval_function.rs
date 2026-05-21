use crate::builtin_resolved::{ResolvedExpr, ResolvedParam, ResolvedStmt};
use crate::lowered::facts::ArrowClosure;
use crate::lowered::resolver::expr::{
    DirectEvalEnvBinding, DirectEvalEnvBindingKind, DirectEvalEnvDescriptor,
    focused_direct_eval_tdz_candidates,
};
use crate::lowered::*;
use std::collections::HashSet;
use ts2wasm_diagnostic::Diagnostic;
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_eval_function_decl_in_caller_scope(
        &mut self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
        is_async: bool,
        scope_index: usize,
    ) -> Result<LoweredStmt, Diagnostic> {
        let (local_id, existed) = self.declare_eval_var_in_caller_scope(name, scope_index)?;
        let self_mutates = block_assigns_any_name(body, &[name.to_owned()]);
        let mut env_cell_conversions = Vec::new();
        let capture_names = self.nested_function_capture_names(name, params, body)?;
        for capture in capture_names {
            if !block_assigns_any_name(body, std::slice::from_ref(&capture))
                || self.ctx.facts.env_cell_names.contains(&capture)
            {
                continue;
            }
            let capture_local = self.resolve_local(&capture)?;
            self.ctx.facts.env_cell_names.insert(capture);
            self.ctx.facts.env_cell_locals.insert(capture_local);
            self.ctx
                .facts
                .initialized_env_cell_locals
                .insert(capture_local);
            self.ctx.facts.nullish_locals.remove(&capture_local);
            env_cell_conversions.push(LoweredStmt::Assign(
                capture_local,
                LoweredExpr::EnvCellNew(
                    Box::new(LoweredExpr::Local(
                        capture_local,
                        Span::generated("eval_capture_current"),
                    )),
                    Span::generated("eval_capture_env_cell_new"),
                ),
                Span::generated("eval_capture_env_cell_assign"),
            ));
        }
        if self.ctx.facts.env_cell_names.contains(name) || self_mutates {
            self.ctx.facts.env_cell_names.insert(name.to_owned());
            self.ctx.facts.env_cell_locals.insert(local_id);
            self.ctx.facts.initialized_env_cell_locals.insert(local_id);
            self.ctx.facts.nullish_locals.remove(&local_id);
        }
        let closure = self.lower_nested_function(name, params, body, is_async)?;
        if let LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation,
            span: _,
        } = &closure
        {
            if matches!(representation, ClosureRepresentation::HeapObject) {
                self.ctx.facts.heap_closure_locals.insert(local_id);
            } else {
                self.ctx.facts.arrow_locals.insert(
                    local_id,
                    ArrowClosure {
                        func_id: *func_id,
                        captures: captures.clone(),
                    },
                );
            }
        }
        self.ctx.facts.nullish_locals.remove(&local_id);
        if self.ctx.facts.env_cell_locals.contains(&local_id) {
            self.lower_eval_function_env_cell_stmt(
                local_id,
                existed,
                self_mutates,
                env_cell_conversions,
                closure,
            )
        } else if existed {
            Ok(LoweredStmt::Assign(
                local_id,
                closure,
                Span::generated("eval_function_assign"),
            ))
        } else {
            Ok(LoweredStmt::Let(
                local_id,
                closure,
                Span::generated("eval_function_let"),
            ))
        }
    }

    fn lower_eval_function_env_cell_stmt(
        &mut self,
        local_id: LocalId,
        existed: bool,
        self_mutates: bool,
        mut env_cell_conversions: Vec<LoweredStmt>,
        closure: LoweredExpr,
    ) -> Result<LoweredStmt, Diagnostic> {
        if existed {
            let set_closure = LoweredStmt::Expr(
                LoweredExpr::EnvCellSet {
                    cell: local_id,
                    expr: Box::new(closure),
                    span: Span::generated("env_cell_set"),
                },
                Span::generated("expr_stmt"),
            );
            if self_mutates || !env_cell_conversions.is_empty() {
                if self_mutates {
                    env_cell_conversions.push(LoweredStmt::Assign(
                        local_id,
                        LoweredExpr::EnvCellNew(
                            Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                            Span::generated("env_cell_new"),
                        ),
                        Span::generated("eval_function_env_cell_assign"),
                    ));
                }
                env_cell_conversions.push(set_closure);
                Ok(LoweredStmt::Block(
                    env_cell_conversions,
                    Span::generated("block"),
                ))
            } else {
                Ok(set_closure)
            }
        } else {
            Ok(LoweredStmt::Block(
                vec![
                    LoweredStmt::Let(
                        local_id,
                        LoweredExpr::EnvCellNew(
                            Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                            Span::generated("env_cell_new"),
                        ),
                        Span::generated("let_stmt"),
                    ),
                    LoweredStmt::Expr(
                        LoweredExpr::EnvCellSet {
                            cell: local_id,
                            expr: Box::new(closure),
                            span: Span::generated("env_cell_set"),
                        },
                        Span::generated("expr_stmt"),
                    ),
                ],
                Span::generated("block"),
            ))
        }
    }

    pub(super) fn lower_direct_eval_env_descriptor(
        &self,
        caller_is_strict: bool,
        source_expr: &ResolvedExpr,
    ) -> LoweredExpr {
        self.collect_direct_eval_env_descriptor(caller_is_strict, source_expr)
            .into_lowered_expr()
    }

    pub(super) fn collect_direct_eval_env_descriptor(
        &self,
        caller_is_strict: bool,
        source_expr: &ResolvedExpr,
    ) -> DirectEvalEnvDescriptor {
        let mut seen = HashSet::new();
        let mut bindings = Vec::new();
        self.collect_visible_direct_eval_bindings(&mut seen, &mut bindings);
        self.collect_direct_eval_env_cell_tdz_bindings(&mut seen, &mut bindings);
        if let Some(source) = self.direct_eval_static_source_value(source_expr) {
            self.collect_focused_direct_eval_tdz_bindings(&source, &mut seen, &mut bindings);
        }

        DirectEvalEnvDescriptor {
            caller_is_strict,
            bindings,
        }
    }

    fn collect_visible_direct_eval_bindings(
        &self,
        seen: &mut HashSet<String>,
        bindings: &mut Vec<DirectEvalEnvBinding>,
    ) {
        for scope in self.ctx.symbols.scopes.iter().rev() {
            let mut scope_bindings = scope.iter().collect::<Vec<_>>();
            scope_bindings.sort_by(|(left, _), (right, _)| left.cmp(right));
            for (name, local) in scope_bindings {
                if !seen.insert(name.clone()) {
                    continue;
                }
                let kind = if self.ctx.facts.env_cell_locals.contains(local)
                    && self.ctx.facts.initialized_env_cell_locals.contains(local)
                {
                    DirectEvalEnvBindingKind::ReadWrite { local: *local }
                } else {
                    DirectEvalEnvBindingKind::Tdz
                };
                bindings.push(DirectEvalEnvBinding {
                    name: name.clone(),
                    kind,
                });
            }
        }
    }

    fn collect_direct_eval_env_cell_tdz_bindings(
        &self,
        seen: &mut HashSet<String>,
        bindings: &mut Vec<DirectEvalEnvBinding>,
    ) {
        let mut env_cell_names = self
            .ctx
            .facts
            .env_cell_names
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        env_cell_names.sort();
        for name in env_cell_names {
            if !seen.insert(name.clone()) {
                continue;
            }
            let needs_tdz_entry = match self.ctx.symbols.resolve(&name) {
                Some(local) => {
                    !self.ctx.facts.env_cell_locals.contains(&local)
                        || !self.ctx.facts.initialized_env_cell_locals.contains(&local)
                }
                None => true,
            };
            if needs_tdz_entry {
                bindings.push(DirectEvalEnvBinding {
                    name,
                    kind: DirectEvalEnvBindingKind::Tdz,
                });
            }
        }
    }

    fn collect_focused_direct_eval_tdz_bindings(
        &self,
        source: &str,
        seen: &mut HashSet<String>,
        bindings: &mut Vec<DirectEvalEnvBinding>,
    ) {
        for name in focused_direct_eval_tdz_candidates(source) {
            let is_known_binding = self.ctx.symbols.resolve(&name).is_some()
                || self.ctx.facts.env_cell_names.contains(&name);
            if is_known_binding && seen.insert(name.clone()) {
                bindings.push(DirectEvalEnvBinding {
                    name,
                    kind: DirectEvalEnvBindingKind::Tdz,
                });
            }
        }
    }

    pub(super) fn direct_eval_static_source_value(
        &self,
        source_expr: &ResolvedExpr,
    ) -> Option<String> {
        if let Some(value) = crate::lowered::resolver::string::resolved_expr_static_string_value(
            &self.ctx,
            source_expr,
        ) {
            return Some(value);
        }
        let ResolvedExpr::Ident(name) = source_expr else {
            return None;
        };
        let local_id = self.ctx.resolve_local(name).ok()?;
        self.ctx.facts.string_literal_locals.get(&local_id).cloned()
    }
}
