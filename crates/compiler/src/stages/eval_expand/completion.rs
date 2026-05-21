use crate::stages::eval_expand::*;

pub(super) fn eval_completion_steps(
    source: &str,
    ast_stmts: &[Stmt],
    stmts: Vec<ResolvedStmt>,
    var_landing: EvalVarLanding,
) -> Vec<EvalCompletionStep> {
    stmts
        .into_iter()
        .enumerate()
        .map(|(idx, stmt)| {
            eval_statement_completion_step(source, ast_stmts.get(idx), stmt, var_landing)
        })
        .collect()
}

pub(super) fn eval_statement_completion_step(
    source: &str,
    ast_stmt: Option<&Stmt>,
    stmt: ResolvedStmt,
    var_landing: EvalVarLanding,
) -> EvalCompletionStep {
    match stmt {
        ResolvedStmt::Expr(expr) => EvalCompletionStep::Value(expr),
        ResolvedStmt::Assign(name, expr) => EvalCompletionStep::Value(ResolvedExpr::Assign {
            name,
            expr: Box::new(expr),
        }),
        ResolvedStmt::Let(name, expr)
            if var_landing == EvalVarLanding::Caller
                && matches!(ast_stmt, Some(Stmt::Let { is_var: true, .. })) =>
        {
            EvalCompletionStep::VarLet { name, init: expr }
        }
        ResolvedStmt::Let(name, expr)
            if var_landing == EvalVarLanding::Global
                && matches!(ast_stmt, Some(Stmt::Let { is_var: true, .. })) =>
        {
            EvalCompletionStep::GlobalVarLet { name, init: expr }
        }
        ResolvedStmt::Let(name, expr) => EvalCompletionStep::LexicalLet { name, init: expr },
        ResolvedStmt::DestructureLet { pattern, expr }
            if matches!(ast_stmt, Some(Stmt::Let { is_var: true, .. }))
                && matches!(var_landing, EvalVarLanding::Caller | EvalVarLanding::Global) =>
        {
            EvalCompletionStep::DestructureVarLet {
                pattern,
                init: expr,
                var_landing: match var_landing {
                    EvalVarLanding::Caller => EvalForHeadVarLanding::Caller,
                    EvalVarLanding::Global => EvalForHeadVarLanding::Global,
                    EvalVarLanding::Lexical => EvalForHeadVarLanding::Local,
                },
            }
        }
        ResolvedStmt::DestructureLet { pattern, expr } => EvalCompletionStep::DestructureLet {
            pattern,
            init: expr,
        },
        ResolvedStmt::Block { statements } => {
            let ast_statements = match ast_stmt {
                Some(Stmt::Block { statements, .. }) => statements.as_slice(),
                _ => &[],
            };
            EvalCompletionStep::Block(eval_completion_steps(
                source,
                ast_statements,
                statements,
                var_landing,
            ))
        }
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let (ast_then, ast_else) = match ast_stmt {
                Some(Stmt::If {
                    then_body,
                    else_body,
                    ..
                }) => (then_body.as_slice(), else_body.as_slice()),
                _ => (&[][..], &[][..]),
            };
            EvalCompletionStep::If {
                condition,
                then_steps: eval_completion_steps(source, ast_then, then_body, var_landing),
                else_steps: eval_completion_steps(source, ast_else, else_body, var_landing),
            }
        }
        ResolvedStmt::While { condition, body } => {
            let ast_body = match ast_stmt {
                Some(Stmt::While { body, .. }) => body.as_slice(),
                _ => &[],
            };
            EvalCompletionStep::While {
                condition,
                body_steps: eval_completion_steps(source, ast_body, body, var_landing),
            }
        }
        ResolvedStmt::DoWhile { body, condition } => {
            let ast_body = match ast_stmt {
                Some(Stmt::DoWhile { body, .. }) => body.as_slice(),
                _ => &[],
            };
            EvalCompletionStep::DoWhile {
                body_steps: eval_completion_steps(source, ast_body, body, var_landing),
                condition,
            }
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            let (ast_init, ast_body) = match ast_stmt {
                Some(Stmt::For { init, body, .. }) => (init.as_deref(), body.as_slice()),
                _ => (None, &[][..]),
            };
            EvalCompletionStep::For {
                init: init.map(|stmt| {
                    Box::new(eval_statement_completion_step(
                        source,
                        ast_init,
                        *stmt,
                        var_landing,
                    ))
                }),
                condition,
                update,
                body_steps: eval_completion_steps(source, ast_body, body, var_landing),
            }
        }
        ResolvedStmt::ForOf { var, iter, body } => {
            let (ast_body, head_landing, head_pattern) = match ast_stmt {
                Some(Stmt::ForOf { body, span, .. }) => (
                    body.as_slice(),
                    eval_for_head_var_landing(source, *span, "of", &var, var_landing),
                    eval_for_head_var_pattern(source, *span, "of"),
                ),
                _ => (&[][..], EvalForHeadVarLanding::Local, None),
            };
            let (ast_body, strip_parser_shim) = eval_for_head_body_without_parser_shim(
                ast_body,
                body.as_slice(),
                &var,
                head_pattern.is_some(),
            );
            let body = if strip_parser_shim {
                body.into_iter().skip(1).collect()
            } else {
                body
            };
            EvalCompletionStep::ForOf {
                var,
                var_landing: head_landing,
                var_pattern: head_pattern,
                iter,
                body_steps: eval_completion_steps(source, ast_body, body, var_landing),
            }
        }
        ResolvedStmt::ForIn { var, iter, body } => {
            let (ast_body, head_landing, head_pattern) = match ast_stmt {
                Some(Stmt::ForIn { body, span, .. }) => (
                    body.as_slice(),
                    eval_for_head_var_landing(source, *span, "in", &var, var_landing),
                    eval_for_head_var_pattern(source, *span, "in"),
                ),
                _ => (&[][..], EvalForHeadVarLanding::Local, None),
            };
            let (ast_body, strip_parser_shim) = eval_for_head_body_without_parser_shim(
                ast_body,
                body.as_slice(),
                &var,
                head_pattern.is_some(),
            );
            let body = if strip_parser_shim {
                body.into_iter().skip(1).collect()
            } else {
                body
            };
            EvalCompletionStep::ForIn {
                var,
                var_landing: head_landing,
                var_pattern: head_pattern,
                iter,
                body_steps: eval_completion_steps(source, ast_body, body, var_landing),
            }
        }
        ResolvedStmt::Switch { expr, cases } => {
            let ast_cases = match ast_stmt {
                Some(Stmt::Switch { cases, .. }) => cases.as_slice(),
                _ => &[],
            };
            let cases = cases
                .into_iter()
                .enumerate()
                .map(|(idx, (case_expr, body))| {
                    let ast_body = ast_cases
                        .get(idx)
                        .map(|(_, body)| body.as_slice())
                        .unwrap_or(&[]);
                    (
                        case_expr,
                        eval_completion_steps(source, ast_body, body, var_landing),
                    )
                })
                .collect();
            EvalCompletionStep::Switch { expr, cases }
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
        } => {
            let (ast_try, ast_catch, ast_finally) = match ast_stmt {
                Some(Stmt::TryCatch {
                    try_block,
                    catch_block,
                    finally_block,
                    ..
                }) => (
                    try_block.as_slice(),
                    catch_block.as_deref(),
                    finally_block.as_deref(),
                ),
                _ => (&[][..], None, None),
            };
            EvalCompletionStep::TryCatch {
                try_steps: eval_completion_steps(source, ast_try, try_block, var_landing),
                catch_param,
                catch_steps: catch_block.map(|block| {
                    eval_completion_steps(source, ast_catch.unwrap_or(&[]), block, var_landing)
                }),
                finally_steps: finally_block.map(|block| {
                    eval_completion_steps(source, ast_finally.unwrap_or(&[]), block, var_landing)
                }),
            }
        }
        ResolvedStmt::Labeled { label, body } => {
            let ast_body = match ast_stmt {
                Some(Stmt::Labeled { body, .. }) => Some(body.as_ref()),
                _ => None,
            };
            EvalCompletionStep::Labeled {
                label,
                body: Box::new(eval_statement_completion_step(
                    source,
                    ast_body,
                    *body,
                    var_landing,
                )),
            }
        }
        ResolvedStmt::Function {
            name,
            params,
            body,
            is_async,
            ..
        } if var_landing == EvalVarLanding::Caller
            && matches!(ast_stmt, Some(Stmt::Function { .. })) =>
        {
            EvalCompletionStep::FunctionDecl {
                name,
                params,
                body,
                is_async,
            }
        }
        ResolvedStmt::Function {
            name,
            params,
            body,
            is_generator,
            is_async,
            source_text,
            ..
        } if var_landing == EvalVarLanding::Global
            && matches!(ast_stmt, Some(Stmt::Function { .. })) =>
        {
            EvalCompletionStep::GlobalFunctionDecl {
                name,
                params,
                body,
                is_generator,
                is_async,
                source_text,
            }
        }
        ResolvedStmt::ClassDecl {
            name,
            extends,
            constructor,
            methods,
            static_blocks,
            private_fields,
            static_private_fields,
            ..
        } if matches!(ast_stmt, Some(Stmt::ClassDecl { .. })) => EvalCompletionStep::ClassDecl {
            name,
            extends,
            constructor,
            methods,
            private_fields,
            static_private_fields,
            static_blocks,
        },
        ResolvedStmt::Break { label } => EvalCompletionStep::Break { label },
        ResolvedStmt::Continue { label } => EvalCompletionStep::Continue { label },
        ResolvedStmt::Throw(expr) => EvalCompletionStep::Throw(expr),
        ResolvedStmt::Return(expr) => EvalCompletionStep::Value(expr),
        _ => EvalCompletionStep::Empty(None),
    }
}
