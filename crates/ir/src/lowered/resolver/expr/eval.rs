use crate::binding_pattern::{ArrayBinding, BindingPattern, BindingTarget, ObjectBinding};
use crate::builtin_resolved::{EvalForHeadVarLanding, ResolvedExpr, ResolvedStmt};
use crate::lowered::resolver::EvalClassDeclParts;
use crate::lowered::resolver::expr::is_eval_for_head_identifier_key;
use crate::lowered::*;
use std::collections::HashMap;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_eval_completion_steps_into(
        &mut self,
        steps: &[crate::builtin_resolved::EvalCompletionStep],
        completion: LocalId,
        caller_scope_index: usize,
        stmts: &mut Vec<LoweredStmt>,
    ) -> Result<(), Diagnostic> {
        use crate::builtin_resolved::EvalCompletionStep;

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
                EvalCompletionStep::VarLet { name, init } => {
                    let (local, existed) =
                        self.declare_eval_var_in_caller_scope(name, caller_scope_index)?;
                    let init = self.lower_expr(init)?;
                    if existed {
                        stmts.push(LoweredStmt::Assign(
                            local,
                            init,
                            Span::generated("eval_var_assign"),
                        ));
                    } else {
                        stmts.push(LoweredStmt::Let(
                            local,
                            init,
                            Span::generated("eval_var_let"),
                        ));
                    }
                }
                EvalCompletionStep::GlobalVarLet { name, init } => {
                    stmts.push(LoweredStmt::Expr(
                        self.lower_static_global_eval_var_landing(name, init)?,
                        Span::generated("eval_global_var"),
                    ));
                }
                EvalCompletionStep::GlobalFunctionDecl {
                    name,
                    params,
                    body,
                    is_generator,
                    is_async,
                    source_text,
                } => {
                    stmts.push(LoweredStmt::Expr(
                        self.lower_static_global_eval_function_landing(
                            name,
                            params,
                            body,
                            *is_generator,
                            *is_async,
                            source_text,
                        )?,
                        Span::generated("eval_global_function"),
                    ));
                }
                EvalCompletionStep::FunctionDecl {
                    name,
                    params,
                    body,
                    is_async,
                } => {
                    stmts.push(self.lower_eval_function_decl_in_caller_scope(
                        name,
                        params,
                        body,
                        *is_async,
                        caller_scope_index,
                    )?);
                }
                EvalCompletionStep::ClassDecl {
                    name,
                    extends,
                    constructor,
                    methods,
                    private_fields,
                    static_private_fields,
                    static_blocks,
                } => {
                    stmts.extend(self.lower_eval_class_decl(EvalClassDeclParts {
                        name,
                        extends,
                        constructor,
                        methods,
                        private_fields,
                        static_private_fields,
                        static_blocks,
                    })?);
                }
                EvalCompletionStep::Block(block_steps) => {
                    self.ctx.symbols.scopes.push(HashMap::new());
                    let lowered = (|| {
                        let mut block_stmts = Vec::new();
                        self.lower_eval_completion_steps_into(
                            block_steps,
                            completion,
                            caller_scope_index,
                            &mut block_stmts,
                        )?;
                        Ok(LoweredStmt::Block(
                            block_stmts,
                            Span::generated("eval_completion_block"),
                        ))
                    })();
                    self.ctx.symbols.scopes.pop();
                    stmts.push(lowered?);
                }
                EvalCompletionStep::If {
                    condition,
                    then_steps,
                    else_steps,
                } => {
                    let condition = self.lower_expr(condition)?;
                    self.ctx.symbols.scopes.push(HashMap::new());
                    let then_body = (|| {
                        let mut then_stmts = Vec::new();
                        self.lower_eval_completion_steps_into(
                            then_steps,
                            completion,
                            caller_scope_index,
                            &mut then_stmts,
                        )?;
                        Ok(then_stmts)
                    })();
                    self.ctx.symbols.scopes.pop();
                    self.ctx.symbols.scopes.push(HashMap::new());
                    let else_body = (|| {
                        let mut else_stmts = Vec::new();
                        self.lower_eval_completion_steps_into(
                            else_steps,
                            completion,
                            caller_scope_index,
                            &mut else_stmts,
                        )?;
                        Ok(else_stmts)
                    })();
                    self.ctx.symbols.scopes.pop();
                    stmts.push(LoweredStmt::If {
                        condition,
                        then_body: then_body?,
                        else_body: else_body?,
                        span: Span::generated("eval_completion_if"),
                    });
                }
                EvalCompletionStep::While {
                    condition,
                    body_steps,
                } => {
                    let condition = self.lower_expr(condition)?;
                    self.ctx.symbols.scopes.push(HashMap::new());
                    let body = (|| {
                        let mut body_stmts = Vec::new();
                        self.lower_eval_completion_steps_into(
                            body_steps,
                            completion,
                            caller_scope_index,
                            &mut body_stmts,
                        )?;
                        Ok(body_stmts)
                    })();
                    self.ctx.symbols.scopes.pop();
                    stmts.push(LoweredStmt::While {
                        condition,
                        body: body?,
                        span: Span::generated("eval_completion_while"),
                    });
                }
                EvalCompletionStep::DoWhile {
                    body_steps,
                    condition,
                } => {
                    self.ctx.symbols.scopes.push(HashMap::new());
                    let body = (|| {
                        let mut body_stmts = Vec::new();
                        self.lower_eval_completion_steps_into(
                            body_steps,
                            completion,
                            caller_scope_index,
                            &mut body_stmts,
                        )?;
                        Ok(body_stmts)
                    })();
                    self.ctx.symbols.scopes.pop();
                    stmts.push(LoweredStmt::DoWhile {
                        body: body?,
                        condition: self.lower_expr(condition)?,
                        span: Span::generated("eval_completion_do_while"),
                    });
                }
                EvalCompletionStep::For {
                    init,
                    condition,
                    update,
                    body_steps,
                } => {
                    self.ctx.symbols.scopes.push(HashMap::new());
                    let lowered = (|| {
                        let init = init
                            .as_deref()
                            .map(|step| {
                                self.lower_eval_non_completion_step(step, caller_scope_index)
                            })
                            .transpose()?
                            .flatten()
                            .map(Box::new);
                        let condition =
                            condition.as_ref().map(|c| self.lower_expr(c)).transpose()?;
                        let update = update.as_ref().map(|u| self.lower_expr(u)).transpose()?;
                        let mut body = Vec::new();
                        self.lower_eval_completion_steps_into(
                            body_steps,
                            completion,
                            caller_scope_index,
                            &mut body,
                        )?;
                        Ok(LoweredStmt::For {
                            init,
                            condition,
                            update,
                            body,
                            span: Span::generated("eval_completion_for"),
                        })
                    })();
                    self.ctx.symbols.scopes.pop();
                    stmts.push(lowered?);
                }
                EvalCompletionStep::ForOf {
                    var,
                    var_landing,
                    var_pattern,
                    iter,
                    body_steps,
                } => {
                    let should_land_global = matches!(var_landing, EvalForHeadVarLanding::Global);
                    let var_id = if var_pattern.is_some() {
                        self.declare_local(var)?
                    } else {
                        match var_landing {
                            EvalForHeadVarLanding::Caller => {
                                let (local, existed) =
                                    self.declare_eval_var_in_caller_scope(var, caller_scope_index)?;
                                if !existed {
                                    stmts.push(LoweredStmt::Let(
                                        local,
                                        LoweredExpr::Undefined(Span::generated(
                                            "eval_for_var_hoist",
                                        )),
                                        Span::generated("eval_for_var_hoist_let"),
                                    ));
                                }
                                local
                            }
                            EvalForHeadVarLanding::Local | EvalForHeadVarLanding::Global => {
                                self.declare_local(var)?
                            }
                        }
                    };
                    let lowered_iter = if let ResolvedExpr::Ident(name) = iter
                        && let Ok(local_id) = self.resolve_local(name)
                    {
                        let class_name = self.ctx.classes.local_classes.get(&local_id);
                        if class_name.is_some_and(|c| c == "Set") {
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::SetValuesArray,
                                args: vec![LoweredExpr::Local(local_id, Span::generated("local"))],
                                span: Span::generated("runtime_call"),
                            }
                        } else if class_name.is_some_and(|c| c == "Map") {
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::MapEntryPairsArray,
                                args: vec![LoweredExpr::Local(local_id, Span::generated("local"))],
                                span: Span::generated("runtime_call"),
                            }
                        } else {
                            self.lower_expr(iter)?
                        }
                    } else {
                        self.lower_expr(iter)?
                    };
                    let mut body = self.lower_eval_completion_steps_scoped(
                        body_steps,
                        completion,
                        caller_scope_index,
                    )?;
                    if let Some(pattern) = var_pattern {
                        let writes = self.lower_eval_for_head_pattern_writes(
                            pattern,
                            LoweredExpr::Local(var_id, Span::generated("eval_for_pattern_value")),
                            *var_landing,
                            caller_scope_index,
                        )?;
                        body.splice(0..0, writes);
                    }
                    stmts.push(LoweredStmt::ForOf {
                        var: var_id,
                        iter: lowered_iter,
                        iter_local: self.alloc_temp(),
                        index_local: self.alloc_temp(),
                        len_local: self.alloc_temp(),
                        body,
                        span: Span::generated("eval_completion_for_of"),
                    });
                    if should_land_global && var_pattern.is_none() {
                        stmts.push(LoweredStmt::Expr(
                            self.lower_static_global_eval_var_landing(
                                var,
                                &ResolvedExpr::Ident(var.clone()),
                            )?,
                            Span::generated("eval_for_global_var"),
                        ));
                    }
                }
                EvalCompletionStep::ForIn {
                    var,
                    var_landing,
                    var_pattern,
                    iter,
                    body_steps,
                } => {
                    let should_land_global = matches!(var_landing, EvalForHeadVarLanding::Global);
                    let var_id = if var_pattern.is_some() {
                        self.declare_local(var)?
                    } else {
                        match var_landing {
                            EvalForHeadVarLanding::Caller => {
                                let (local, existed) =
                                    self.declare_eval_var_in_caller_scope(var, caller_scope_index)?;
                                if !existed {
                                    stmts.push(LoweredStmt::Let(
                                        local,
                                        LoweredExpr::Undefined(Span::generated(
                                            "eval_for_var_hoist",
                                        )),
                                        Span::generated("eval_for_var_hoist_let"),
                                    ));
                                }
                                local
                            }
                            EvalForHeadVarLanding::Local | EvalForHeadVarLanding::Global => {
                                self.declare_local(var)?
                            }
                        }
                    };
                    let iter = self.lower_expr(iter)?;
                    let mut body = self.lower_eval_completion_steps_scoped(
                        body_steps,
                        completion,
                        caller_scope_index,
                    )?;
                    if let Some(pattern) = var_pattern {
                        let writes = self.lower_eval_for_head_pattern_writes(
                            pattern,
                            LoweredExpr::Local(var_id, Span::generated("eval_for_pattern_value")),
                            *var_landing,
                            caller_scope_index,
                        )?;
                        body.splice(0..0, writes);
                    }
                    stmts.push(LoweredStmt::ForIn {
                        var: var_id,
                        iter,
                        iter_local: self.alloc_temp(),
                        index_local: self.alloc_temp(),
                        len_local: self.alloc_temp(),
                        body,
                        span: Span::generated("eval_completion_for_in"),
                    });
                    if should_land_global && var_pattern.is_none() {
                        stmts.push(LoweredStmt::Expr(
                            self.lower_static_global_eval_var_landing(
                                var,
                                &ResolvedExpr::Ident(var.clone()),
                            )?,
                            Span::generated("eval_for_global_var"),
                        ));
                    }
                }
                EvalCompletionStep::Switch { expr, cases } => {
                    let expr = self.lower_expr(expr)?;
                    let cases = cases
                        .iter()
                        .map(|(cond, body_steps)| {
                            self.ctx.symbols.scopes.push(HashMap::new());
                            let lowered = (|| {
                                let cond = cond.as_ref().map(|c| self.lower_expr(c)).transpose()?;
                                let mut body = Vec::new();
                                self.lower_eval_completion_steps_into(
                                    body_steps,
                                    completion,
                                    caller_scope_index,
                                    &mut body,
                                )?;
                                Ok((cond, body))
                            })();
                            self.ctx.symbols.scopes.pop();
                            lowered
                        })
                        .collect::<Result<Vec<_>, Diagnostic>>()?;
                    stmts.push(LoweredStmt::Switch {
                        expr,
                        cases,
                        span: Span::generated("eval_completion_switch"),
                    });
                }
                EvalCompletionStep::TryCatch {
                    try_steps,
                    catch_param,
                    catch_steps,
                    finally_steps,
                } => {
                    let try_body = self.lower_eval_completion_steps_scoped(
                        try_steps,
                        completion,
                        caller_scope_index,
                    )?;
                    let (catch_var, catch_body) = if let Some(catch_steps) = catch_steps {
                        self.ctx.symbols.scopes.push(HashMap::new());
                        let lowered = (|| {
                            let catch_var = if let Some(param) = catch_param {
                                let local_id = self.declare_local(param)?;
                                if self.ctx.facts.env_cell_names.contains(param) {
                                    self.ctx.facts.env_cell_locals.insert(local_id);
                                    self.ctx.facts.initialized_env_cell_locals.insert(local_id);
                                }
                                Some(local_id)
                            } else {
                                None
                            };
                            let mut catch_body = Vec::new();
                            self.lower_eval_completion_steps_into(
                                catch_steps,
                                completion,
                                caller_scope_index,
                                &mut catch_body,
                            )?;
                            if let Some(local_id) = catch_var
                                && self.ctx.facts.env_cell_locals.contains(&local_id)
                            {
                                catch_body.insert(
                                    0,
                                    LoweredStmt::Assign(
                                        local_id,
                                        LoweredExpr::EnvCellNew(
                                            Box::new(LoweredExpr::Local(
                                                local_id,
                                                Span::generated("catch_binding"),
                                            )),
                                            Span::generated("env_cell_new"),
                                        ),
                                        Span::generated("assign"),
                                    ),
                                );
                            }
                            Ok((catch_var, Some(catch_body)))
                        })();
                        self.ctx.symbols.scopes.pop();
                        lowered?
                    } else {
                        (None, None)
                    };
                    let finally_body = finally_steps
                        .as_deref()
                        .map(|steps| {
                            self.lower_eval_non_completion_steps_scoped(steps, caller_scope_index)
                        })
                        .transpose()?;
                    if catch_body.is_none() && finally_body.is_some() {
                        stmts.push(LoweredStmt::TryFinally {
                            try_body,
                            finally_body: finally_body.unwrap_or_default(),
                            span: Span::generated("eval_completion_try_finally"),
                        });
                    } else {
                        stmts.push(LoweredStmt::TryCatch {
                            try_body,
                            catch_var,
                            catch_body,
                            finally_body,
                            span: Span::generated("eval_completion_try_catch"),
                        });
                    }
                }
                EvalCompletionStep::LexicalLet { name, init } => {
                    stmts.push(self.lower_stmt(&ResolvedStmt::Let(name.clone(), init.clone()))?);
                }
                EvalCompletionStep::DestructureLet { pattern, init } => {
                    stmts.push(self.lower_stmt(&ResolvedStmt::DestructureLet {
                        pattern: pattern.clone(),
                        expr: init.clone(),
                    })?);
                }
                EvalCompletionStep::DestructureVarLet {
                    pattern,
                    init,
                    var_landing,
                } => {
                    stmts.extend(self.lower_eval_destructure_var_landing(
                        pattern,
                        init,
                        *var_landing,
                        caller_scope_index,
                    )?);
                }
                EvalCompletionStep::Throw(expr) => {
                    stmts.push(LoweredStmt::Throw(
                        self.lower_expr(expr)?,
                        Span::generated("eval_completion_throw"),
                    ));
                }
                EvalCompletionStep::Labeled { label, body } => {
                    let mut labeled_body = Vec::new();
                    self.lower_eval_completion_steps_into(
                        std::slice::from_ref(body.as_ref()),
                        completion,
                        caller_scope_index,
                        &mut labeled_body,
                    )?;
                    let body = if labeled_body.len() == 1 {
                        labeled_body.remove(0)
                    } else {
                        LoweredStmt::Block(
                            labeled_body,
                            Span::generated("eval_completion_labeled_block"),
                        )
                    };
                    stmts.push(LoweredStmt::Labeled {
                        label: label.clone(),
                        body: Box::new(body),
                        span: Span::generated("eval_completion_labeled"),
                    });
                }
                EvalCompletionStep::Break { label } => {
                    stmts.push(LoweredStmt::Break {
                        label: label.clone(),
                        span: Span::generated("eval_completion_break"),
                    });
                }
                EvalCompletionStep::Continue { label } => {
                    stmts.push(LoweredStmt::Continue {
                        label: label.clone(),
                        span: Span::generated("eval_completion_continue"),
                    });
                }
                EvalCompletionStep::Empty(None) => {}
            }
        }
        Ok(())
    }

    pub(super) fn lower_eval_completion_steps_scoped(
        &mut self,
        steps: &[crate::builtin_resolved::EvalCompletionStep],
        completion: LocalId,
        caller_scope_index: usize,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        self.ctx.symbols.scopes.push(HashMap::new());
        let lowered = (|| {
            let mut body = Vec::new();
            self.lower_eval_completion_steps_into(
                steps,
                completion,
                caller_scope_index,
                &mut body,
            )?;
            Ok(body)
        })();
        self.ctx.symbols.scopes.pop();
        lowered
    }

    pub(super) fn lower_eval_non_completion_steps_scoped(
        &mut self,
        steps: &[crate::builtin_resolved::EvalCompletionStep],
        caller_scope_index: usize,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        self.ctx.symbols.scopes.push(HashMap::new());
        let lowered = (|| {
            let mut body = Vec::new();
            for step in steps {
                if let Some(stmt) = self.lower_eval_non_completion_step(step, caller_scope_index)? {
                    body.push(stmt);
                }
            }
            Ok(body)
        })();
        self.ctx.symbols.scopes.pop();
        lowered
    }

    pub(super) fn lower_eval_non_completion_step(
        &mut self,
        step: &crate::builtin_resolved::EvalCompletionStep,
        caller_scope_index: usize,
    ) -> Result<Option<LoweredStmt>, Diagnostic> {
        use crate::builtin_resolved::EvalCompletionStep;

        match step {
            EvalCompletionStep::Value(expr) | EvalCompletionStep::Empty(Some(expr)) => {
                Ok(Some(LoweredStmt::Expr(
                    self.lower_expr(expr)?,
                    Span::generated("eval_non_completion_expr"),
                )))
            }
            EvalCompletionStep::VarLet { name, init } => {
                let (local, existed) =
                    self.declare_eval_var_in_caller_scope(name, caller_scope_index)?;
                let init = self.lower_expr(init)?;
                if existed {
                    Ok(Some(LoweredStmt::Assign(
                        local,
                        init,
                        Span::generated("eval_non_completion_var_assign"),
                    )))
                } else {
                    Ok(Some(LoweredStmt::Let(
                        local,
                        init,
                        Span::generated("eval_non_completion_var_let"),
                    )))
                }
            }
            EvalCompletionStep::GlobalVarLet { name, init } => Ok(Some(LoweredStmt::Expr(
                self.lower_static_global_eval_var_landing(name, init)?,
                Span::generated("eval_non_completion_global_var"),
            ))),
            EvalCompletionStep::GlobalFunctionDecl {
                name,
                params,
                body,
                is_generator,
                is_async,
                source_text,
            } => Ok(Some(LoweredStmt::Expr(
                self.lower_static_global_eval_function_landing(
                    name,
                    params,
                    body,
                    *is_generator,
                    *is_async,
                    source_text,
                )?,
                Span::generated("eval_non_completion_global_function"),
            ))),
            EvalCompletionStep::LexicalLet { name, init } => Ok(Some(
                self.lower_stmt(&ResolvedStmt::Let(name.clone(), init.clone()))?,
            )),
            EvalCompletionStep::DestructureLet { pattern, init } => {
                Ok(Some(self.lower_stmt(&ResolvedStmt::DestructureLet {
                    pattern: pattern.clone(),
                    expr: init.clone(),
                })?))
            }
            EvalCompletionStep::DestructureVarLet {
                pattern,
                init,
                var_landing,
            } => Ok(Some(LoweredStmt::Block(
                self.lower_eval_destructure_var_landing(
                    pattern,
                    init,
                    *var_landing,
                    caller_scope_index,
                )?,
                Span::generated("eval_non_completion_destructure_var"),
            ))),
            EvalCompletionStep::FunctionDecl {
                name,
                params,
                body,
                is_async,
            } => Ok(Some(self.lower_eval_function_decl_in_caller_scope(
                name,
                params,
                body,
                *is_async,
                caller_scope_index,
            )?)),
            EvalCompletionStep::ClassDecl {
                name,
                extends,
                constructor,
                methods,
                private_fields,
                static_private_fields,
                static_blocks,
            } => Ok(Some(LoweredStmt::Block(
                self.lower_eval_class_decl(EvalClassDeclParts {
                    name,
                    extends,
                    constructor,
                    methods,
                    private_fields,
                    static_private_fields,
                    static_blocks,
                })?,
                Span::generated("eval_non_completion_class_decl"),
            ))),
            EvalCompletionStep::Throw(expr) => Ok(Some(LoweredStmt::Throw(
                self.lower_expr(expr)?,
                Span::generated("eval_non_completion_throw"),
            ))),
            EvalCompletionStep::Labeled { label, body } => {
                let body = self
                    .lower_eval_non_completion_step(body, caller_scope_index)?
                    .unwrap_or_else(|| {
                        LoweredStmt::Block(
                            Vec::new(),
                            Span::generated("eval_non_completion_labeled_empty"),
                        )
                    });
                Ok(Some(LoweredStmt::Labeled {
                    label: label.clone(),
                    body: Box::new(body),
                    span: Span::generated("eval_non_completion_labeled"),
                }))
            }
            EvalCompletionStep::Break { label } => Ok(Some(LoweredStmt::Break {
                label: label.clone(),
                span: Span::generated("eval_non_completion_break"),
            })),
            EvalCompletionStep::Continue { label } => Ok(Some(LoweredStmt::Continue {
                label: label.clone(),
                span: Span::generated("eval_non_completion_continue"),
            })),
            EvalCompletionStep::Empty(None) => Ok(None),
            other => {
                let temp = self.alloc_temp();
                let mut stmts = vec![LoweredStmt::Let(
                    temp,
                    LoweredExpr::Undefined(Span::generated("eval_non_completion_temp")),
                    Span::generated("eval_non_completion_temp_let"),
                )];
                self.lower_eval_completion_steps_into(
                    std::slice::from_ref(other),
                    temp,
                    caller_scope_index,
                    &mut stmts,
                )?;
                Ok(Some(LoweredStmt::Block(
                    stmts,
                    Span::generated("eval_non_completion_block"),
                )))
            }
        }
    }

    pub(super) fn declare_eval_var_in_caller_scope(
        &mut self,
        name: &str,
        scope_index: usize,
    ) -> Result<(LocalId, bool), Diagnostic> {
        if self.ctx.is_strict_context() && matches!(name, "eval" | "arguments") {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-450: {:?} strict mode forbids binding local `{name}`",
                    crate::lowered::ctx::StrictModeCheck::StrictEval
                ),
                span: None,
                phase: None,
            });
        }
        let scope = self
            .ctx
            .symbols
            .scopes
            .get_mut(scope_index)
            .expect("eval caller scope must exist");
        if let Some(&existing) = scope.get(name) {
            return Ok((existing, true));
        }
        let local = LocalId(self.ctx.symbols.next_local_id);
        self.ctx.symbols.next_local_id += 1;
        self.ctx.symbols.locals.push(local);
        scope.insert(name.to_owned(), local);
        Ok((local, false))
    }

    pub(super) fn lower_eval_for_head_pattern_writes(
        &mut self,
        pattern: &BindingPattern,
        value: LoweredExpr,
        landing: EvalForHeadVarLanding,
        caller_scope_index: usize,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        if landing == EvalForHeadVarLanding::Local {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedEval,
                message: "static eval for-head destructuring currently requires caller or global var landing".to_owned(),
                span: Some(Span::generated("eval_for_pattern_landing")),
                phase: None,
            });
        }
        match pattern {
            BindingPattern::Array(bindings) => {
                let mut stmts = Vec::new();
                for binding in bindings {
                    stmts.extend(self.lower_eval_for_head_array_binding_write(
                        binding,
                        &value,
                        landing,
                        caller_scope_index,
                    )?);
                }
                Ok(stmts)
            }
            BindingPattern::Object(bindings) => {
                let mut stmts = Vec::new();
                for binding in bindings {
                    stmts.extend(self.lower_eval_for_head_object_binding_write(
                        binding,
                        bindings,
                        &value,
                        landing,
                        caller_scope_index,
                    )?);
                }
                Ok(stmts)
            }
        }
    }

    pub(super) fn lower_eval_destructure_var_landing(
        &mut self,
        pattern: &BindingPattern,
        init: &ResolvedExpr,
        landing: EvalForHeadVarLanding,
        caller_scope_index: usize,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let temp = self.alloc_temp();
        let mut stmts = vec![LoweredStmt::Let(
            temp,
            self.lower_expr(init)?,
            Span::generated("eval_destructure_var_temp"),
        )];
        stmts.extend(self.lower_eval_for_head_pattern_writes(
            pattern,
            LoweredExpr::Local(temp, Span::generated("eval_destructure_var_temp")),
            landing,
            caller_scope_index,
        )?);
        Ok(stmts)
    }

    pub(super) fn lower_eval_for_head_array_binding_write(
        &mut self,
        binding: &ArrayBinding,
        value: &LoweredExpr,
        landing: EvalForHeadVarLanding,
        caller_scope_index: usize,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let element_value = if binding.is_rest {
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArraySlice,
                args: vec![
                    value.clone(),
                    LoweredExpr::Number(binding.index as i32, Span::generated("num")),
                    LoweredExpr::GetLength(Box::new(value.clone()), Span::generated("get_length")),
                ],
                span: Span::generated("runtime_call"),
            }
        } else {
            LoweredExpr::Index {
                object: Box::new(value.clone()),
                index: Box::new(LoweredExpr::Number(
                    binding.index as i32,
                    Span::generated("num"),
                )),
                span: Span::generated("index"),
            }
        };
        self.lower_eval_for_head_binding_target_write(
            &binding.target,
            element_value,
            binding.default.as_ref(),
            landing,
            caller_scope_index,
        )
    }

    pub(super) fn lower_eval_for_head_object_binding_write(
        &mut self,
        binding: &ObjectBinding,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        landing: EvalForHeadVarLanding,
        caller_scope_index: usize,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        if binding.is_rest {
            return self.lower_eval_for_head_object_rest_binding_write(
                binding,
                siblings,
                value,
                landing,
                caller_scope_index,
            );
        }
        let property_value = if binding.computed {
            let key_raw = binding.key.trim_start_matches('[').trim_end_matches(']');
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(value.clone()),
                key: Box::new(self.lower_eval_for_head_computed_key_expr(key_raw, landing)?),
                span: Span::generated("prop_get_dynamic"),
            }
        } else {
            LoweredExpr::PropertyGet {
                obj: Box::new(value.clone()),
                key: binding.key.clone(),
                span: Span::generated("prop_get"),
            }
        };
        self.lower_eval_for_head_binding_target_write(
            &binding.target,
            property_value,
            binding.default.as_ref(),
            landing,
            caller_scope_index,
        )
    }

    pub(super) fn lower_eval_for_head_object_rest_binding_write(
        &mut self,
        binding: &ObjectBinding,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        landing: EvalForHeadVarLanding,
        caller_scope_index: usize,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let rest_temp = self.alloc_temp();
        let mut rest_args = vec![value.clone()];
        for sibling in siblings.iter().filter(|sibling| !sibling.is_rest) {
            if sibling.computed {
                let key_raw = sibling.key.trim_start_matches('[').trim_end_matches(']');
                rest_args.push(self.lower_eval_for_head_computed_key_expr(key_raw, landing)?);
            } else {
                rest_args.push(LoweredExpr::String(
                    sibling.key.clone(),
                    Span::generated("str"),
                ));
            }
        }
        let mut stmts = vec![LoweredStmt::Let(
            rest_temp,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::RestObject,
                args: rest_args,
                span: Span::generated("eval_for_object_rest"),
            },
            Span::generated("eval_for_object_rest_temp"),
        )];
        stmts.extend(self.lower_eval_for_head_binding_target_write(
            &binding.target,
            LoweredExpr::Local(rest_temp, Span::generated("eval_for_object_rest")),
            None,
            landing,
            caller_scope_index,
        )?);
        Ok(stmts)
    }

    pub(super) fn lower_eval_for_head_computed_key_expr(
        &mut self,
        key_raw: &str,
        landing: EvalForHeadVarLanding,
    ) -> Result<LoweredExpr, Diagnostic> {
        if landing == EvalForHeadVarLanding::Global && is_eval_for_head_identifier_key(key_raw) {
            return Ok(LoweredExpr::PropertyGet {
                obj: Box::new(self.lower_expr(&ResolvedExpr::Ident("globalThis".to_owned()))?),
                key: key_raw.to_owned(),
                span: Span::generated("eval_for_pattern_global_key"),
            });
        }
        self.lower_computed_object_binding_key_expr(key_raw)
    }

    pub(super) fn lower_eval_for_head_binding_target_write(
        &mut self,
        target: &BindingTarget,
        value: LoweredExpr,
        default: Option<&crate::binding_pattern::BindingDefault>,
        landing: EvalForHeadVarLanding,
        caller_scope_index: usize,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let value = if let Some(default) = default {
            let default_expr = self.lower_binding_default_expr(default)?;
            let temp = self.alloc_temp();
            return Ok({
                let mut stmts = vec![
                    LoweredStmt::Let(temp, value, Span::generated("eval_for_pattern_temp")),
                    LoweredStmt::If {
                        condition: LoweredExpr::Binary {
                            left: Box::new(LoweredExpr::Local(
                                temp,
                                Span::generated("eval_for_pattern_temp"),
                            )),
                            op: LoweredBinaryOp::StrictEqual,
                            right: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                            span: Span::generated("binary"),
                        },
                        then_body: vec![LoweredStmt::Assign(
                            temp,
                            default_expr,
                            Span::generated("eval_for_pattern_default"),
                        )],
                        else_body: vec![],
                        span: Span::generated("eval_for_pattern_default_if"),
                    },
                ];
                stmts.extend(self.lower_eval_for_head_binding_target_write(
                    target,
                    LoweredExpr::Local(temp, Span::generated("eval_for_pattern_temp")),
                    None,
                    landing,
                    caller_scope_index,
                )?);
                stmts
            });
        } else {
            value
        };
        match target {
            BindingTarget::Identifier(name) => match landing {
                EvalForHeadVarLanding::Caller => {
                    let (local, _) =
                        self.declare_eval_var_in_caller_scope(name, caller_scope_index)?;
                    Ok(vec![LoweredStmt::Assign(
                        local,
                        value,
                        Span::generated("eval_for_pattern_assign"),
                    )])
                }
                EvalForHeadVarLanding::Global => Ok(vec![LoweredStmt::Expr(
                    LoweredExpr::PropertySet {
                        object: Box::new(
                            self.lower_expr(&ResolvedExpr::Ident("globalThis".to_owned()))?,
                        ),
                        key: name.clone(),
                        value: Box::new(value),
                        span: Span::generated("eval_for_pattern_global_assign"),
                    },
                    Span::generated("eval_for_pattern_global_assign"),
                )]),
                EvalForHeadVarLanding::Local => unreachable!("local landing rejected above"),
            },
            BindingTarget::Pattern(pattern) => {
                self.lower_eval_for_head_pattern_writes(pattern, value, landing, caller_scope_index)
            }
        }
    }

    pub(super) fn lower_static_global_eval_var_landing(
        &mut self,
        name: &str,
        init: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        self.lower_property_assign_expr(
            &ResolvedExpr::Ident("globalThis".to_owned()),
            name,
            init,
            Span::generated("static_indirect_eval_global_var"),
        )
    }

    pub(super) fn lower_static_global_eval_function_landing(
        &mut self,
        name: &str,
        params: &[crate::builtin_resolved::ResolvedParam],
        body: &[ResolvedStmt],
        is_generator: bool,
        _is_async: bool,
        source_text: &str,
    ) -> Result<LoweredExpr, Diagnostic> {
        let function = ResolvedExpr::FunctionExpr {
            name: name.to_owned(),
            params: params.to_vec(),
            body: body.to_vec(),
            is_generator,
            origin: ts2wasm_syntax::FunctionExprOrigin::User,
            constructor_metadata: None,
            source_text: source_text.to_owned(),
        };
        self.lower_property_assign_expr(
            &ResolvedExpr::Ident("globalThis".to_owned()),
            name,
            &function,
            Span::generated("static_indirect_eval_global_function"),
        )
    }
}
