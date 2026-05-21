use crate::stages::eval_expand::*;

pub(super) fn rewrite_indirect_eval_caller_binding_collisions(
    expr: ResolvedExpr,
    caller_bindings: &[String],
) -> ResolvedExpr {
    if caller_bindings.is_empty() {
        return expr;
    }
    let collisions = caller_bindings.iter().cloned().collect::<HashSet<_>>();
    let mut scopes = vec![HashSet::new()];
    rewrite_eval_expr_global_collisions(expr, &collisions, &mut scopes)
}

pub(super) fn rewrite_eval_expr_global_collisions(
    expr: ResolvedExpr,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> ResolvedExpr {
    match expr {
        ResolvedExpr::Ident(name)
            if collisions.contains(&name) && !eval_name_is_scoped(&name, scopes) =>
        {
            eval_global_property(name)
        }
        ResolvedExpr::Await { expr } => ResolvedExpr::Await {
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedExpr::Yield { expr, delegate } => ResolvedExpr::Yield {
            expr: expr.map(|expr| {
                Box::new(rewrite_eval_expr_global_collisions(
                    *expr, collisions, scopes,
                ))
            }),
            delegate,
        },
        ResolvedExpr::Unary { op, expr } => ResolvedExpr::Unary {
            op,
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedExpr::Binary { left, op, right } => ResolvedExpr::Binary {
            left: Box::new(rewrite_eval_expr_global_collisions(
                *left, collisions, scopes,
            )),
            op,
            right: Box::new(rewrite_eval_expr_global_collisions(
                *right, collisions, scopes,
            )),
        },
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            span,
        } => ResolvedExpr::Ternary {
            condition: Box::new(rewrite_eval_expr_global_collisions(
                *condition, collisions, scopes,
            )),
            then_expr: Box::new(rewrite_eval_expr_global_collisions(
                *then_expr, collisions, scopes,
            )),
            else_expr: Box::new(rewrite_eval_expr_global_collisions(
                *else_expr, collisions, scopes,
            )),
            span,
        },
        ResolvedExpr::Call { callee, args, span } => match *callee {
            ResolvedExpr::Ident(name)
                if collisions.contains(&name) && !eval_name_is_scoped(&name, scopes) =>
            {
                ResolvedExpr::MethodCall {
                    object: Box::new(ResolvedExpr::Ident("globalThis".to_owned())),
                    method: name,
                    args: rewrite_eval_exprs_global_collisions(args, collisions, scopes),
                    span,
                }
            }
            callee => ResolvedExpr::Call {
                callee: Box::new(rewrite_eval_expr_global_collisions(
                    callee, collisions, scopes,
                )),
                args: rewrite_eval_exprs_global_collisions(args, collisions, scopes),
                span,
            },
        },
        ResolvedExpr::Assign { name, expr }
            if collisions.contains(&name) && !eval_name_is_scoped(&name, scopes) =>
        {
            ResolvedExpr::PropertyAssign {
                object: Box::new(ResolvedExpr::Ident("globalThis".to_owned())),
                key: name,
                value: Box::new(rewrite_eval_expr_global_collisions(
                    *expr, collisions, scopes,
                )),
                span: Span::generated("static_indirect_eval_global_assign"),
            }
        }
        ResolvedExpr::Assign { name, expr } => ResolvedExpr::Assign {
            name,
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedExpr::LogicalAssign { name, op, expr } => ResolvedExpr::LogicalAssign {
            name,
            op,
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedExpr::LogicalPropertyAssign {
            object,
            key,
            op,
            expr,
        } => ResolvedExpr::LogicalPropertyAssign {
            object,
            key,
            op,
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedExpr::LogicalComputedPropertyAssign {
            object,
            key,
            op,
            expr,
        } => ResolvedExpr::LogicalComputedPropertyAssign {
            object,
            key: Box::new(rewrite_eval_expr_global_collisions(
                *key, collisions, scopes,
            )),
            op,
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedExpr::LogicalComputedMemberAssign {
            object,
            key,
            op,
            expr,
        } => ResolvedExpr::LogicalComputedMemberAssign {
            object: Box::new(rewrite_eval_expr_global_collisions(
                *object, collisions, scopes,
            )),
            key: Box::new(rewrite_eval_expr_global_collisions(
                *key, collisions, scopes,
            )),
            op,
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedExpr::LogicalMemberAssign {
            object,
            key,
            op,
            expr,
        } => ResolvedExpr::LogicalMemberAssign {
            object: Box::new(rewrite_eval_expr_global_collisions(
                *object, collisions, scopes,
            )),
            key,
            op,
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedExpr::Array(elements) => ResolvedExpr::Array(
            elements
                .into_iter()
                .map(|element| match element {
                    ResolvedArrayElement::Present(expr) => ResolvedArrayElement::Present(
                        rewrite_eval_expr_global_collisions(expr, collisions, scopes),
                    ),
                    ResolvedArrayElement::Hole => ResolvedArrayElement::Hole,
                })
                .collect(),
        ),
        ResolvedExpr::Object(props) => ResolvedExpr::Object(
            props
                .into_iter()
                .map(|prop| rewrite_eval_object_prop_global_collisions(prop, collisions, scopes))
                .collect(),
        ),
        ResolvedExpr::ComputedIndex { object, index } => ResolvedExpr::ComputedIndex {
            object: Box::new(rewrite_eval_expr_global_collisions(
                *object, collisions, scopes,
            )),
            index: Box::new(rewrite_eval_expr_global_collisions(
                *index, collisions, scopes,
            )),
        },
        ResolvedExpr::BuiltinCall { builtin, args } => ResolvedExpr::BuiltinCall {
            builtin,
            args: rewrite_eval_exprs_global_collisions(args, collisions, scopes),
        },
        ResolvedExpr::BuiltinProperty {
            builtin,
            object,
            span,
        } => ResolvedExpr::BuiltinProperty {
            builtin,
            object: Box::new(rewrite_eval_expr_global_collisions(
                *object, collisions, scopes,
            )),
            span,
        },
        ResolvedExpr::PropertyAccess { object, key, span } => ResolvedExpr::PropertyAccess {
            object: Box::new(rewrite_eval_expr_global_collisions(
                *object, collisions, scopes,
            )),
            key,
            span,
        },
        ResolvedExpr::OptionalPropertyAccess { object, key, span } => {
            ResolvedExpr::OptionalPropertyAccess {
                object: Box::new(rewrite_eval_expr_global_collisions(
                    *object, collisions, scopes,
                )),
                key,
                span,
            }
        }
        ResolvedExpr::OptionalComputedIndex {
            object,
            index,
            span,
        } => ResolvedExpr::OptionalComputedIndex {
            object: Box::new(rewrite_eval_expr_global_collisions(
                *object, collisions, scopes,
            )),
            index: Box::new(rewrite_eval_expr_global_collisions(
                *index, collisions, scopes,
            )),
            span,
        },
        ResolvedExpr::OptionalCall { callee, args, span } => ResolvedExpr::OptionalCall {
            callee: Box::new(rewrite_eval_expr_global_collisions(
                *callee, collisions, scopes,
            )),
            args: rewrite_eval_exprs_global_collisions(args, collisions, scopes),
            span,
        },
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            span,
        } => ResolvedExpr::MethodCall {
            object: Box::new(rewrite_eval_expr_global_collisions(
                *object, collisions, scopes,
            )),
            method,
            args: rewrite_eval_exprs_global_collisions(args, collisions, scopes),
            span,
        },
        ResolvedExpr::PropertyAssign {
            object,
            key,
            value,
            span,
        } => ResolvedExpr::PropertyAssign {
            object: Box::new(rewrite_eval_expr_global_collisions(
                *object, collisions, scopes,
            )),
            key,
            value: Box::new(rewrite_eval_expr_global_collisions(
                *value, collisions, scopes,
            )),
            span,
        },
        ResolvedExpr::Spread(expr) => ResolvedExpr::Spread(Box::new(
            rewrite_eval_expr_global_collisions(*expr, collisions, scopes),
        )),
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            ResolvedExpr::PropertyAssignDynamic {
                object: Box::new(rewrite_eval_expr_global_collisions(
                    *object, collisions, scopes,
                )),
                key: Box::new(rewrite_eval_expr_global_collisions(
                    *key, collisions, scopes,
                )),
                value: Box::new(rewrite_eval_expr_global_collisions(
                    *value, collisions, scopes,
                )),
            }
        }
        ResolvedExpr::New {
            class_name,
            args,
            span,
        } => ResolvedExpr::New {
            class_name,
            args: rewrite_eval_exprs_global_collisions(args, collisions, scopes),
            span,
        },
        ResolvedExpr::FunctionConstructor { plan } => ResolvedExpr::FunctionConstructor {
            plan: FunctionConstructorPlan::new(
                plan.kind,
                rewrite_eval_exprs_global_collisions(plan.args, collisions, scopes),
                plan.span,
            ),
        },
        ResolvedExpr::ArrowFn {
            params,
            body,
            body_stmts,
            source_text,
        } => {
            scopes.push(params.iter().cloned().collect());
            let body = Box::new(rewrite_eval_expr_global_collisions(
                *body, collisions, scopes,
            ));
            let body_stmts = rewrite_eval_stmts_global_collisions(body_stmts, collisions, scopes);
            scopes.pop();
            ResolvedExpr::ArrowFn {
                params,
                body,
                body_stmts,
                source_text,
            }
        }
        ResolvedExpr::FunctionExpr {
            name,
            params,
            body,
            is_generator,
            origin,
            constructor_metadata,
            source_text,
        } => {
            scopes.push(HashSet::new());
            if !name.is_empty() {
                eval_declare_name(&name, scopes);
            }
            let params = rewrite_eval_params_global_collisions(params, collisions, scopes);
            let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
            scopes.pop();
            ResolvedExpr::FunctionExpr {
                name,
                params,
                body,
                is_generator,
                origin,
                constructor_metadata,
                source_text,
            }
        }
        ResolvedExpr::ClassExpr { name, body } => {
            scopes.push(HashSet::new());
            if !name.is_empty() {
                eval_declare_name(&name, scopes);
            }
            let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
            scopes.pop();
            ResolvedExpr::ClassExpr { name, body }
        }
        ResolvedExpr::Sequence(exprs) => ResolvedExpr::Sequence(
            rewrite_eval_exprs_global_collisions(exprs, collisions, scopes),
        ),
        ResolvedExpr::EvalCompletion(plan) => {
            let declarations = rewrite_eval_declaration_plan_global_collisions(
                plan.declarations,
                collisions,
                scopes,
            );
            ResolvedExpr::EvalCompletion(EvalCompletionPlan::with_eval_context(
                plan.scope_mode,
                plan.caller_is_strict,
                plan.eval_is_strict,
                declarations,
                rewrite_eval_steps_global_collisions(plan.steps, collisions, scopes),
            ))
        }
        ResolvedExpr::Eval { plan } => ResolvedExpr::Eval {
            plan: EvalFragmentPlan {
                source: match plan.source {
                    EvalSource::Runtime(expr) => EvalSource::Runtime(Box::new(
                        rewrite_eval_expr_global_collisions(*expr, collisions, scopes),
                    )),
                    EvalSource::StaticLiteral(src) => EvalSource::StaticLiteral(src),
                    EvalSource::NonStringStatic(expr) => EvalSource::NonStringStatic(Box::new(
                        rewrite_eval_expr_global_collisions(*expr, collisions, scopes),
                    )),
                },
                ..plan
            },
        },
        ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. } => expr,
    }
}

pub(super) fn rewrite_eval_exprs_global_collisions(
    exprs: Vec<ResolvedExpr>,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> Vec<ResolvedExpr> {
    exprs
        .into_iter()
        .map(|expr| rewrite_eval_expr_global_collisions(expr, collisions, scopes))
        .collect()
}

pub(super) fn rewrite_eval_object_prop_global_collisions(
    prop: ResolvedObjectProp,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> ResolvedObjectProp {
    match prop {
        ResolvedObjectProp::KeyValue { key, value } => ResolvedObjectProp::KeyValue {
            key,
            value: rewrite_eval_expr_global_collisions(value, collisions, scopes),
        },
        ResolvedObjectProp::Shorthand { key, value } => ResolvedObjectProp::Shorthand {
            key,
            value: rewrite_eval_expr_global_collisions(value, collisions, scopes),
        },
        ResolvedObjectProp::ComputedKey { key, value } => ResolvedObjectProp::ComputedKey {
            key: Box::new(rewrite_eval_expr_global_collisions(
                *key, collisions, scopes,
            )),
            value: rewrite_eval_expr_global_collisions(value, collisions, scopes),
        },
        ResolvedObjectProp::MethodShorthand { key, value } => ResolvedObjectProp::MethodShorthand {
            key,
            value: rewrite_eval_expr_global_collisions(value, collisions, scopes),
        },
    }
}

pub(super) fn rewrite_eval_params_global_collisions(
    params: Vec<ResolvedParam>,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> Vec<ResolvedParam> {
    params
        .into_iter()
        .map(|mut param| {
            param.default = param
                .default
                .map(|expr| rewrite_eval_expr_global_collisions(expr, collisions, scopes));
            eval_declare_name(&param.name, scopes);
            param
        })
        .collect()
}

pub(super) fn rewrite_eval_stmts_global_collisions(
    stmts: Vec<ResolvedStmt>,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> Vec<ResolvedStmt> {
    stmts
        .into_iter()
        .map(|stmt| rewrite_eval_stmt_global_collisions(stmt, collisions, scopes))
        .collect()
}

pub(super) fn rewrite_eval_stmt_global_collisions(
    stmt: ResolvedStmt,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> ResolvedStmt {
    match stmt {
        ResolvedStmt::Let(name, expr) => {
            let expr = rewrite_eval_expr_global_collisions(expr, collisions, scopes);
            eval_declare_name(&name, scopes);
            ResolvedStmt::Let(name, expr)
        }
        ResolvedStmt::DestructureLet { pattern, expr } => {
            let expr = rewrite_eval_expr_global_collisions(expr, collisions, scopes);
            for name in pattern.names() {
                eval_declare_name(name, scopes);
            }
            ResolvedStmt::DestructureLet { pattern, expr }
        }
        ResolvedStmt::Assign(name, expr) => ResolvedStmt::Assign(
            name,
            rewrite_eval_expr_global_collisions(expr, collisions, scopes),
        ),
        ResolvedStmt::Expr(expr) => ResolvedStmt::Expr(rewrite_eval_expr_global_collisions(
            expr, collisions, scopes,
        )),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => ResolvedStmt::If {
            condition: rewrite_eval_expr_global_collisions(condition, collisions, scopes),
            then_body: rewrite_eval_scoped_stmts_global_collisions(then_body, collisions, scopes),
            else_body: rewrite_eval_scoped_stmts_global_collisions(else_body, collisions, scopes),
        },
        ResolvedStmt::While { condition, body } => ResolvedStmt::While {
            condition: rewrite_eval_expr_global_collisions(condition, collisions, scopes),
            body: rewrite_eval_scoped_stmts_global_collisions(body, collisions, scopes),
        },
        ResolvedStmt::Return(expr) => ResolvedStmt::Return(rewrite_eval_expr_global_collisions(
            expr, collisions, scopes,
        )),
        ResolvedStmt::Function {
            name,
            params,
            body,
            is_generator,
            is_async,
            is_ambient,
            source_text,
        } => {
            eval_declare_name(&name, scopes);
            scopes.push(HashSet::new());
            let params = rewrite_eval_params_global_collisions(params, collisions, scopes);
            let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
            scopes.pop();
            ResolvedStmt::Function {
                name,
                params,
                body,
                is_generator,
                is_async,
                is_ambient,
                source_text,
            }
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
        } => ResolvedStmt::TryCatch {
            try_block: rewrite_eval_scoped_stmts_global_collisions(try_block, collisions, scopes),
            catch_block: catch_block.map(|block| {
                scopes.push(HashSet::new());
                if let Some(param) = &catch_param {
                    eval_declare_name(param, scopes);
                }
                let block = rewrite_eval_stmts_global_collisions(block, collisions, scopes);
                scopes.pop();
                block
            }),
            catch_param,
            finally_block: finally_block.map(|block| {
                rewrite_eval_scoped_stmts_global_collisions(block, collisions, scopes)
            }),
        },
        ResolvedStmt::Throw(expr) => ResolvedStmt::Throw(rewrite_eval_expr_global_collisions(
            expr, collisions, scopes,
        )),
        ResolvedStmt::Switch { expr, cases } => ResolvedStmt::Switch {
            expr: rewrite_eval_expr_global_collisions(expr, collisions, scopes),
            cases: cases
                .into_iter()
                .map(|(case_expr, body)| {
                    (
                        case_expr.map(|expr| {
                            rewrite_eval_expr_global_collisions(expr, collisions, scopes)
                        }),
                        rewrite_eval_scoped_stmts_global_collisions(body, collisions, scopes),
                    )
                })
                .collect(),
        },
        ResolvedStmt::DoWhile { body, condition } => ResolvedStmt::DoWhile {
            body: rewrite_eval_scoped_stmts_global_collisions(body, collisions, scopes),
            condition: rewrite_eval_expr_global_collisions(condition, collisions, scopes),
        },
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            scopes.push(HashSet::new());
            let init = init.map(|stmt| {
                Box::new(rewrite_eval_stmt_global_collisions(
                    *stmt, collisions, scopes,
                ))
            });
            let condition =
                condition.map(|expr| rewrite_eval_expr_global_collisions(expr, collisions, scopes));
            let update =
                update.map(|expr| rewrite_eval_expr_global_collisions(expr, collisions, scopes));
            let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
            scopes.pop();
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            }
        }
        ResolvedStmt::ForIn { var, iter, body } => {
            let iter = rewrite_eval_expr_global_collisions(iter, collisions, scopes);
            scopes.push(HashSet::new());
            eval_declare_name(&var, scopes);
            let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
            scopes.pop();
            ResolvedStmt::ForIn { var, iter, body }
        }
        ResolvedStmt::ForOf { var, iter, body } => {
            let iter = rewrite_eval_expr_global_collisions(iter, collisions, scopes);
            scopes.push(HashSet::new());
            eval_declare_name(&var, scopes);
            let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
            scopes.pop();
            ResolvedStmt::ForOf { var, iter, body }
        }
        ResolvedStmt::ForAwaitOf { var, iter, body } => {
            let iter = rewrite_eval_expr_global_collisions(iter, collisions, scopes);
            scopes.push(HashSet::new());
            eval_declare_name(&var, scopes);
            let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
            scopes.pop();
            ResolvedStmt::ForAwaitOf { var, iter, body }
        }
        ResolvedStmt::Labeled { label, body } => ResolvedStmt::Labeled {
            label,
            body: Box::new(rewrite_eval_stmt_global_collisions(
                *body, collisions, scopes,
            )),
        },
        ResolvedStmt::Export { name, expr } => ResolvedStmt::Export {
            name,
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedStmt::ModuleExportsAssign { expr } => ResolvedStmt::ModuleExportsAssign {
            expr: Box::new(rewrite_eval_expr_global_collisions(
                *expr, collisions, scopes,
            )),
        },
        ResolvedStmt::ClassDecl {
            name,
            extends,
            constructor,
            methods,
            statics,
            static_blocks,
            private_fields,
            static_private_fields,
        } => {
            eval_declare_name(&name, scopes);
            ResolvedStmt::ClassDecl {
                name,
                extends,
                constructor: constructor.map(|(params, body)| {
                    scopes.push(HashSet::new());
                    let params = rewrite_eval_params_global_collisions(params, collisions, scopes);
                    let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
                    scopes.pop();
                    (params, body)
                }),
                methods: methods
                    .into_iter()
                    .map(|method| {
                        rewrite_eval_class_method_global_collisions(method, collisions, scopes)
                    })
                    .collect(),
                statics: statics
                    .into_iter()
                    .map(|(name, expr)| {
                        (
                            name,
                            rewrite_eval_expr_global_collisions(expr, collisions, scopes),
                        )
                    })
                    .collect(),
                static_blocks: static_blocks
                    .into_iter()
                    .map(|(span, body)| {
                        (
                            span,
                            rewrite_eval_scoped_stmts_global_collisions(body, collisions, scopes),
                        )
                    })
                    .collect(),
                private_fields,
                static_private_fields: static_private_fields
                    .into_iter()
                    .map(|(name, expr, span)| {
                        (
                            name,
                            rewrite_eval_expr_global_collisions(expr, collisions, scopes),
                            span,
                        )
                    })
                    .collect(),
            }
        }
        ResolvedStmt::Block { statements } => ResolvedStmt::Block {
            statements: rewrite_eval_scoped_stmts_global_collisions(statements, collisions, scopes),
        },
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => stmt,
    }
}

pub(super) fn rewrite_eval_class_method_global_collisions(
    method: ClassMethod,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> ClassMethod {
    scopes.push(HashSet::new());
    let params = rewrite_eval_params_global_collisions(method.params, collisions, scopes);
    let body = rewrite_eval_stmts_global_collisions(method.body, collisions, scopes);
    scopes.pop();
    ClassMethod {
        name: method.name,
        kind: method.kind,
        params,
        body,
        captures: method.captures,
    }
}

pub(super) fn rewrite_eval_scoped_stmts_global_collisions(
    stmts: Vec<ResolvedStmt>,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> Vec<ResolvedStmt> {
    scopes.push(HashSet::new());
    let stmts = rewrite_eval_stmts_global_collisions(stmts, collisions, scopes);
    scopes.pop();
    stmts
}

pub(super) fn rewrite_eval_steps_global_collisions(
    steps: Vec<EvalCompletionStep>,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> Vec<EvalCompletionStep> {
    steps
        .into_iter()
        .map(|step| rewrite_eval_step_global_collisions(step, collisions, scopes))
        .collect()
}

pub(super) fn rewrite_eval_declaration_plan_global_collisions(
    plan: EvalDeclarationPlan,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> EvalDeclarationPlan {
    for name in &plan.var_names {
        eval_declare_name(name, scopes);
    }
    let function_hoists = plan
        .function_hoists
        .into_iter()
        .map(|mut hoist| {
            eval_declare_name(&hoist.name, scopes);
            scopes.push(HashSet::new());
            hoist.params = rewrite_eval_params_global_collisions(hoist.params, collisions, scopes);
            hoist.body = rewrite_eval_stmts_global_collisions(hoist.body, collisions, scopes);
            scopes.pop();
            hoist
        })
        .collect();
    EvalDeclarationPlan {
        var_names: plan.var_names,
        function_hoists,
    }
}

pub(super) fn rewrite_eval_step_global_collisions(
    step: EvalCompletionStep,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> EvalCompletionStep {
    match step {
        EvalCompletionStep::Value(expr) => EvalCompletionStep::Value(
            rewrite_eval_expr_global_collisions(expr, collisions, scopes),
        ),
        EvalCompletionStep::Empty(expr) => EvalCompletionStep::Empty(
            expr.map(|expr| rewrite_eval_expr_global_collisions(expr, collisions, scopes)),
        ),
        EvalCompletionStep::VarLet { name, init } => {
            let init = rewrite_eval_expr_global_collisions(init, collisions, scopes);
            eval_declare_name(&name, scopes);
            EvalCompletionStep::VarLet { name, init }
        }
        EvalCompletionStep::GlobalVarLet { name, init } => {
            let init = rewrite_eval_expr_global_collisions(init, collisions, scopes);
            EvalCompletionStep::GlobalVarLet { name, init }
        }
        EvalCompletionStep::LexicalLet { name, init } => {
            let init = rewrite_eval_expr_global_collisions(init, collisions, scopes);
            eval_declare_name(&name, scopes);
            EvalCompletionStep::LexicalLet { name, init }
        }
        EvalCompletionStep::DestructureLet { pattern, init } => {
            let init = rewrite_eval_expr_global_collisions(init, collisions, scopes);
            for name in pattern.names() {
                eval_declare_name(name, scopes);
            }
            EvalCompletionStep::DestructureLet { pattern, init }
        }
        EvalCompletionStep::DestructureVarLet {
            pattern,
            init,
            var_landing,
        } => {
            let init = rewrite_eval_expr_global_collisions(init, collisions, scopes);
            if var_landing == EvalForHeadVarLanding::Caller {
                for name in pattern.names() {
                    eval_declare_name(name, scopes);
                }
            }
            EvalCompletionStep::DestructureVarLet {
                pattern,
                init,
                var_landing,
            }
        }
        EvalCompletionStep::FunctionDecl {
            name,
            params,
            body,
            is_async,
        } => {
            eval_declare_name(&name, scopes);
            scopes.push(HashSet::new());
            let params = rewrite_eval_params_global_collisions(params, collisions, scopes);
            let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
            scopes.pop();
            EvalCompletionStep::FunctionDecl {
                name,
                params,
                body,
                is_async,
            }
        }
        EvalCompletionStep::GlobalFunctionDecl {
            name,
            params,
            body,
            is_generator,
            is_async,
            source_text,
        } => {
            scopes.push(HashSet::new());
            let params = rewrite_eval_params_global_collisions(params, collisions, scopes);
            let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
            scopes.pop();
            EvalCompletionStep::GlobalFunctionDecl {
                name,
                params,
                body,
                is_generator,
                is_async,
                source_text,
            }
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
            eval_declare_name(&name, scopes);
            EvalCompletionStep::ClassDecl {
                name,
                extends,
                constructor: constructor.map(|(params, body)| {
                    scopes.push(HashSet::new());
                    let params = rewrite_eval_params_global_collisions(params, collisions, scopes);
                    let body = rewrite_eval_stmts_global_collisions(body, collisions, scopes);
                    scopes.pop();
                    (params, body)
                }),
                methods: methods
                    .into_iter()
                    .map(|method| {
                        rewrite_eval_class_method_global_collisions(method, collisions, scopes)
                    })
                    .collect(),
                private_fields,
                static_private_fields: static_private_fields
                    .into_iter()
                    .map(|(name, expr, span)| {
                        (
                            name,
                            rewrite_eval_expr_global_collisions(expr, collisions, scopes),
                            span,
                        )
                    })
                    .collect(),
                static_blocks: static_blocks
                    .into_iter()
                    .map(|(span, body)| {
                        (
                            span,
                            rewrite_eval_scoped_stmts_global_collisions(body, collisions, scopes),
                        )
                    })
                    .collect(),
            }
        }
        EvalCompletionStep::Block(steps) => EvalCompletionStep::Block({
            scopes.push(HashSet::new());
            let steps = rewrite_eval_steps_global_collisions(steps, collisions, scopes);
            scopes.pop();
            steps
        }),
        EvalCompletionStep::If {
            condition,
            then_steps,
            else_steps,
        } => EvalCompletionStep::If {
            condition: rewrite_eval_expr_global_collisions(condition, collisions, scopes),
            then_steps: {
                scopes.push(HashSet::new());
                let steps = rewrite_eval_steps_global_collisions(then_steps, collisions, scopes);
                scopes.pop();
                steps
            },
            else_steps: {
                scopes.push(HashSet::new());
                let steps = rewrite_eval_steps_global_collisions(else_steps, collisions, scopes);
                scopes.pop();
                steps
            },
        },
        EvalCompletionStep::While {
            condition,
            body_steps,
        } => EvalCompletionStep::While {
            condition: rewrite_eval_expr_global_collisions(condition, collisions, scopes),
            body_steps: {
                scopes.push(HashSet::new());
                let steps = rewrite_eval_steps_global_collisions(body_steps, collisions, scopes);
                scopes.pop();
                steps
            },
        },
        EvalCompletionStep::DoWhile {
            body_steps,
            condition,
        } => EvalCompletionStep::DoWhile {
            body_steps: {
                scopes.push(HashSet::new());
                let steps = rewrite_eval_steps_global_collisions(body_steps, collisions, scopes);
                scopes.pop();
                steps
            },
            condition: rewrite_eval_expr_global_collisions(condition, collisions, scopes),
        },
        EvalCompletionStep::For {
            init,
            condition,
            update,
            body_steps,
        } => {
            scopes.push(HashSet::new());
            let init = init.map(|step| {
                Box::new(rewrite_eval_step_global_collisions(
                    *step, collisions, scopes,
                ))
            });
            let condition =
                condition.map(|expr| rewrite_eval_expr_global_collisions(expr, collisions, scopes));
            let update =
                update.map(|expr| rewrite_eval_expr_global_collisions(expr, collisions, scopes));
            let body_steps = rewrite_eval_steps_global_collisions(body_steps, collisions, scopes);
            scopes.pop();
            EvalCompletionStep::For {
                init,
                condition,
                update,
                body_steps,
            }
        }
        EvalCompletionStep::ForOf {
            var,
            var_landing,
            var_pattern,
            iter,
            body_steps,
        } => {
            let iter = rewrite_eval_expr_global_collisions(iter, collisions, scopes);
            scopes.push(HashSet::new());
            eval_declare_name(&var, scopes);
            let body_steps = rewrite_eval_steps_global_collisions(body_steps, collisions, scopes);
            scopes.pop();
            EvalCompletionStep::ForOf {
                var,
                var_landing,
                var_pattern,
                iter,
                body_steps,
            }
        }
        EvalCompletionStep::ForIn {
            var,
            var_landing,
            var_pattern,
            iter,
            body_steps,
        } => {
            let iter = rewrite_eval_expr_global_collisions(iter, collisions, scopes);
            scopes.push(HashSet::new());
            eval_declare_name(&var, scopes);
            let body_steps = rewrite_eval_steps_global_collisions(body_steps, collisions, scopes);
            scopes.pop();
            EvalCompletionStep::ForIn {
                var,
                var_landing,
                var_pattern,
                iter,
                body_steps,
            }
        }
        EvalCompletionStep::Switch { expr, cases } => EvalCompletionStep::Switch {
            expr: rewrite_eval_expr_global_collisions(expr, collisions, scopes),
            cases: cases
                .into_iter()
                .map(|(case_expr, steps)| {
                    (
                        case_expr.map(|expr| {
                            rewrite_eval_expr_global_collisions(expr, collisions, scopes)
                        }),
                        {
                            scopes.push(HashSet::new());
                            let steps =
                                rewrite_eval_steps_global_collisions(steps, collisions, scopes);
                            scopes.pop();
                            steps
                        },
                    )
                })
                .collect(),
        },
        EvalCompletionStep::TryCatch {
            try_steps,
            catch_param,
            catch_steps,
            finally_steps,
        } => EvalCompletionStep::TryCatch {
            try_steps: {
                scopes.push(HashSet::new());
                let steps = rewrite_eval_steps_global_collisions(try_steps, collisions, scopes);
                scopes.pop();
                steps
            },
            catch_steps: catch_steps.map(|steps| {
                scopes.push(HashSet::new());
                if let Some(param) = &catch_param {
                    eval_declare_name(param, scopes);
                }
                let steps = rewrite_eval_steps_global_collisions(steps, collisions, scopes);
                scopes.pop();
                steps
            }),
            catch_param,
            finally_steps: finally_steps.map(|steps| {
                scopes.push(HashSet::new());
                let steps = rewrite_eval_steps_global_collisions(steps, collisions, scopes);
                scopes.pop();
                steps
            }),
        },
        EvalCompletionStep::Labeled { label, body } => EvalCompletionStep::Labeled {
            label,
            body: Box::new(rewrite_eval_step_global_collisions(
                *body, collisions, scopes,
            )),
        },
        EvalCompletionStep::Throw(expr) => EvalCompletionStep::Throw(
            rewrite_eval_expr_global_collisions(expr, collisions, scopes),
        ),
        EvalCompletionStep::Break { .. } | EvalCompletionStep::Continue { .. } => step,
    }
}

pub(super) fn eval_name_is_scoped(name: &str, scopes: &[HashSet<String>]) -> bool {
    scopes.iter().rev().any(|scope| scope.contains(name))
}

pub(super) fn eval_declare_name(name: &str, scopes: &mut [HashSet<String>]) {
    if let Some(scope) = scopes.last_mut() {
        scope.insert(name.to_owned());
    }
}

pub(super) fn eval_global_property(name: String) -> ResolvedExpr {
    ResolvedExpr::PropertyAccess {
        object: Box::new(ResolvedExpr::Ident("globalThis".to_owned())),
        key: name,
        span: Span::generated("static_indirect_eval_global"),
    }
}
