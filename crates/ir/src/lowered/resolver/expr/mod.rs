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
use crate::lowered::facts::ArrowClosure;
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
            ResolvedExpr::FunctionConstructor { plan } => {
                self.lower_dynamic_function_constructor_host_compile(&plan.args, plan.span)
            }
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
                constructor_metadata,
                source_text,
                ..
            } => self.lower_named_function_expr(
                name,
                params,
                body,
                *is_generator,
                *origin,
                constructor_metadata.as_ref(),
                source_text,
            ),
            ResolvedExpr::ClassExpr { .. } => Ok(LoweredExpr::Undefined(Span::generated("undef"))),
            ResolvedExpr::Sequence(exprs) => self.lower_sequence_expr(exprs),
            ResolvedExpr::EvalCompletion(plan) => self.lower_eval_completion_expr(plan),
            ResolvedExpr::Eval { plan } => {
                let crate::builtin_resolved::EvalSource::Runtime(source_expr) = &plan.source else {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedEval,
                        message: "static eval fragment reached lowering without AOT expansion"
                            .to_owned(),
                        span: Some(plan.span),
                        phase: None,
                    });
                };
                let source_expr = self.lower_expr(source_expr)?;
                let intrinsic = match plan.host_policy {
                    crate::builtin_resolved::EvalHostPolicy::DirectHost => {
                        self.ensure_direct_eval_env_descriptor_initialized(plan.span)?;
                        RuntimeFn::EvalDirectHost
                    }
                    crate::builtin_resolved::EvalHostPolicy::IndirectHost => {
                        RuntimeFn::EvalIndirectHost
                    }
                    crate::builtin_resolved::EvalHostPolicy::AotOnly => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedEval,
                            message: "AOT-only eval fragment cannot use a runtime host eval lane"
                                .to_owned(),
                            span: Some(plan.span),
                            phase: None,
                        });
                    }
                };
                let args = match plan.host_policy {
                    crate::builtin_resolved::EvalHostPolicy::DirectHost => vec![
                        source_expr,
                        self.lower_direct_eval_env_descriptor(plan.caller_is_strict),
                    ],
                    crate::builtin_resolved::EvalHostPolicy::IndirectHost => vec![source_expr],
                    crate::builtin_resolved::EvalHostPolicy::AotOnly => unreachable!(),
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
        plan: &crate::builtin_resolved::EvalCompletionPlan,
    ) -> Result<LoweredExpr, Diagnostic> {
        if plan.scope_mode != crate::builtin_resolved::EvalScopeMode::Caller
            && (!plan.declarations.is_empty()
                || eval_completion_steps_have_caller_landings(plan.steps()))
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedEval,
                message: "global eval completion cannot land declarations in the caller scope"
                    .to_owned(),
                span: Some(Span::generated("eval_completion_scope")),
                phase: None,
            });
        }
        let completion = self.alloc_temp();
        self.ctx.symbols.scopes.push(HashMap::new());
        let saved_class_constructor_ids = self.ctx.classes.class_constructor_ids.clone();
        let saved_class_method_ids = self.ctx.classes.class_method_ids.clone();
        let saved_class_static_method_ids = self.ctx.classes.class_static_method_ids.clone();
        let saved_class_parents = self.ctx.classes.class_parents.clone();
        let saved_class_private_fields = self.ctx.classes.class_private_fields.clone();
        let saved_class_static_private_fields =
            self.ctx.classes.class_static_private_fields.clone();
        let caller_scope_index = self.ctx.symbols.scopes.len().saturating_sub(2);
        let lowered = (|| {
            let mut stmts = vec![LoweredStmt::Let(
                completion,
                LoweredExpr::Undefined(Span::generated("eval_completion_init")),
                Span::generated("eval_completion_let"),
            )];
            self.lower_eval_declaration_plan_into(
                &plan.declarations,
                caller_scope_index,
                &mut stmts,
            )?;
            self.lower_eval_completion_steps_into(
                plan.steps(),
                completion,
                caller_scope_index,
                &mut stmts,
            )?;
            Ok(LoweredExpr::Block {
                stmts,
                result: Box::new(LoweredExpr::Local(
                    completion,
                    Span::generated("eval_completion_result"),
                )),
                span: Span::generated("eval_completion"),
            })
        })();
        self.ctx.classes.class_constructor_ids = saved_class_constructor_ids;
        self.ctx.classes.class_method_ids = saved_class_method_ids;
        self.ctx.classes.class_static_method_ids = saved_class_static_method_ids;
        self.ctx.classes.class_parents = saved_class_parents;
        self.ctx.classes.class_private_fields = saved_class_private_fields;
        self.ctx.classes.class_static_private_fields = saved_class_static_private_fields;
        self.ctx.symbols.scopes.pop();
        lowered
    }

    fn lower_eval_declaration_plan_into(
        &mut self,
        declarations: &crate::builtin_resolved::EvalDeclarationPlan,
        caller_scope_index: usize,
        stmts: &mut Vec<LoweredStmt>,
    ) -> Result<(), Diagnostic> {
        self.lower_eval_hoisted_vars_into(&declarations.var_names, caller_scope_index, stmts)?;
        self.lower_eval_hoisted_functions_into(
            &declarations.function_hoists,
            caller_scope_index,
            stmts,
        )
    }

    fn lower_eval_hoisted_vars_into(
        &mut self,
        names: &[String],
        caller_scope_index: usize,
        stmts: &mut Vec<LoweredStmt>,
    ) -> Result<(), Diagnostic> {
        for name in names {
            let (local, existed) =
                self.declare_eval_var_in_caller_scope(name, caller_scope_index)?;
            if self.ctx.facts.env_cell_names.contains(name) {
                self.ctx.facts.env_cell_locals.insert(local);
                self.ctx.facts.initialized_env_cell_locals.insert(local);
            }
            if !existed {
                let init = if self.ctx.facts.env_cell_locals.contains(&local) {
                    LoweredExpr::EnvCellNew(
                        Box::new(LoweredExpr::Undefined(Span::generated("eval_var_hoist"))),
                        Span::generated("eval_var_hoist_cell"),
                    )
                } else {
                    LoweredExpr::Undefined(Span::generated("eval_var_hoist"))
                };
                stmts.push(LoweredStmt::Let(
                    local,
                    init,
                    Span::generated("eval_var_hoist_let"),
                ));
            }
        }
        Ok(())
    }

    fn lower_eval_hoisted_functions_into(
        &mut self,
        functions: &[crate::builtin_resolved::EvalFunctionHoist],
        caller_scope_index: usize,
        stmts: &mut Vec<LoweredStmt>,
    ) -> Result<(), Diagnostic> {
        for function in functions {
            stmts.push(self.lower_eval_function_decl_in_caller_scope(
                &function.name,
                &function.params,
                &function.body,
                function.is_async,
                caller_scope_index,
            )?);
        }
        Ok(())
    }

    fn lower_eval_completion_steps_into(
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
                    stmts.extend(self.lower_eval_class_decl(super::EvalClassDeclParts {
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
                    iter,
                    body_steps,
                } => {
                    let var_id = self.declare_local(var)?;
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
                    let body = self.lower_eval_completion_steps_scoped(
                        body_steps,
                        completion,
                        caller_scope_index,
                    )?;
                    stmts.push(LoweredStmt::ForOf {
                        var: var_id,
                        iter: lowered_iter,
                        iter_local: self.alloc_temp(),
                        index_local: self.alloc_temp(),
                        len_local: self.alloc_temp(),
                        body,
                        span: Span::generated("eval_completion_for_of"),
                    });
                }
                EvalCompletionStep::ForIn {
                    var,
                    iter,
                    body_steps,
                } => {
                    let var_id = self.declare_local(var)?;
                    let iter = self.lower_expr(iter)?;
                    let body = self.lower_eval_completion_steps_scoped(
                        body_steps,
                        completion,
                        caller_scope_index,
                    )?;
                    stmts.push(LoweredStmt::ForIn {
                        var: var_id,
                        iter,
                        iter_local: self.alloc_temp(),
                        index_local: self.alloc_temp(),
                        len_local: self.alloc_temp(),
                        body,
                        span: Span::generated("eval_completion_for_in"),
                    });
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

    fn lower_eval_completion_steps_scoped(
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

    fn lower_eval_non_completion_steps_scoped(
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

    fn lower_eval_non_completion_step(
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
            EvalCompletionStep::LexicalLet { name, init } => Ok(Some(
                self.lower_stmt(&ResolvedStmt::Let(name.clone(), init.clone()))?,
            )),
            EvalCompletionStep::DestructureLet { pattern, init } => {
                Ok(Some(self.lower_stmt(&ResolvedStmt::DestructureLet {
                    pattern: pattern.clone(),
                    expr: init.clone(),
                })?))
            }
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
                self.lower_eval_class_decl(super::EvalClassDeclParts {
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

    fn declare_eval_var_in_caller_scope(
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

    fn lower_static_global_eval_var_landing(
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

    fn lower_eval_function_decl_in_caller_scope(
        &mut self,
        name: &str,
        params: &[crate::builtin_resolved::ResolvedParam],
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

    fn lower_direct_eval_env_descriptor(&self, caller_is_strict: bool) -> LoweredExpr {
        self.collect_direct_eval_env_descriptor(caller_is_strict)
            .into_lowered_expr()
    }

    fn collect_direct_eval_env_descriptor(
        &self,
        caller_is_strict: bool,
    ) -> DirectEvalEnvDescriptor {
        let mut seen = HashSet::new();
        let mut bindings = Vec::new();
        for scope in self.ctx.symbols.scopes.iter().rev() {
            let mut scope_bindings = scope.iter().collect::<Vec<_>>();
            scope_bindings.sort_by(|(left, _), (right, _)| left.cmp(right));
            for (name, local) in scope_bindings {
                if !seen.insert(name.clone()) {
                    continue;
                }
                if self.ctx.facts.env_cell_locals.contains(local)
                    && self.ctx.facts.initialized_env_cell_locals.contains(local)
                {
                    bindings.push(DirectEvalEnvBinding {
                        name: name.clone(),
                        local: *local,
                    });
                }
            }
        }

        DirectEvalEnvDescriptor {
            caller_is_strict,
            bindings,
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

struct DirectEvalEnvDescriptor {
    caller_is_strict: bool,
    bindings: Vec<DirectEvalEnvBinding>,
}

struct DirectEvalEnvBinding {
    name: String,
    local: LocalId,
}

impl DirectEvalEnvDescriptor {
    fn into_lowered_expr(self) -> LoweredExpr {
        if self.bindings.is_empty() && !self.caller_is_strict {
            return LoweredExpr::Undefined(Span::generated("eval_env"));
        }

        let mut elements = Vec::with_capacity(2 + self.bindings.len() * 2);
        elements.push(LoweredExpr::String(
            "__ts2wasm_eval_caller_strict".to_owned(),
            Span::generated("eval_env_strict"),
        ));
        elements.push(LoweredExpr::Bool(
            self.caller_is_strict,
            Span::generated("eval_env_strict"),
        ));
        for binding in self.bindings {
            elements.push(LoweredExpr::String(
                binding.name,
                Span::generated("eval_env_name"),
            ));
            elements.push(LoweredExpr::Local(
                binding.local,
                Span::generated("eval_env_cell"),
            ));
        }
        LoweredExpr::ArrayNew {
            elements,
            span: Span::generated("eval_env"),
        }
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

fn eval_completion_steps_have_caller_landings(
    steps: &[crate::builtin_resolved::EvalCompletionStep],
) -> bool {
    use crate::builtin_resolved::EvalCompletionStep;

    steps.iter().any(|step| match step {
        EvalCompletionStep::VarLet { .. } | EvalCompletionStep::FunctionDecl { .. } => true,
        EvalCompletionStep::Block(steps)
        | EvalCompletionStep::While {
            body_steps: steps, ..
        }
        | EvalCompletionStep::DoWhile {
            body_steps: steps, ..
        }
        | EvalCompletionStep::ForOf {
            body_steps: steps, ..
        }
        | EvalCompletionStep::ForIn {
            body_steps: steps, ..
        } => eval_completion_steps_have_caller_landings(steps),
        EvalCompletionStep::If {
            then_steps,
            else_steps,
            ..
        } => {
            eval_completion_steps_have_caller_landings(then_steps)
                || eval_completion_steps_have_caller_landings(else_steps)
        }
        EvalCompletionStep::For {
            init, body_steps, ..
        } => {
            init.as_deref().is_some_and(|step| {
                eval_completion_steps_have_caller_landings(std::slice::from_ref(step))
            }) || eval_completion_steps_have_caller_landings(body_steps)
        }
        EvalCompletionStep::Switch { cases, .. } => cases
            .iter()
            .any(|(_, steps)| eval_completion_steps_have_caller_landings(steps)),
        EvalCompletionStep::TryCatch {
            try_steps,
            catch_steps,
            finally_steps,
            ..
        } => {
            eval_completion_steps_have_caller_landings(try_steps)
                || catch_steps
                    .as_deref()
                    .is_some_and(eval_completion_steps_have_caller_landings)
                || finally_steps
                    .as_deref()
                    .is_some_and(eval_completion_steps_have_caller_landings)
        }
        EvalCompletionStep::Labeled { body, .. } => {
            eval_completion_steps_have_caller_landings(std::slice::from_ref(body.as_ref()))
        }
        EvalCompletionStep::Value(_)
        | EvalCompletionStep::Empty(_)
        | EvalCompletionStep::GlobalVarLet { .. }
        | EvalCompletionStep::ClassDecl { .. }
        | EvalCompletionStep::Throw(_)
        | EvalCompletionStep::Break { .. }
        | EvalCompletionStep::Continue { .. }
        | EvalCompletionStep::LexicalLet { .. }
        | EvalCompletionStep::DestructureLet { .. } => false,
    })
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
