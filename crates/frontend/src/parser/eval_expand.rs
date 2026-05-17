/// Post-parse pass: expand legacy literal `Function("...", "body")` calls.
///
/// Literal eval expression classification is resolver-owned; statement-level
/// direct eval expansion remains in the statement parser until EvalFragment
/// lowering replaces it.
pub(super) fn expand_eval_in_statements(
    stmts: Vec<Stmt>,
    strict_mode: bool,
    possible_function_shadowing: bool,
) -> Vec<Stmt> {
    stmts
        .into_iter()
        .map(|stmt| expand_eval_in_stmt(stmt, strict_mode, possible_function_shadowing))
        .collect()
}

fn expand_eval_in_stmt(stmt: Stmt, strict_mode: bool, possible_function_shadowing: bool) -> Stmt {
    match stmt {
        Stmt::Expr { expr, span } => Stmt::Expr {
            expr: expand_eval_in_expr(expr, strict_mode, possible_function_shadowing),
            span,
        },
        Stmt::Let {
            name,
            expr,
            is_var,
            span,
        } => Stmt::Let {
            name,
            expr: expand_eval_in_expr(expr, strict_mode, possible_function_shadowing),
            is_var,
            span,
        },
        Stmt::Assign { name, expr, span } => Stmt::Assign {
            name,
            expr: expand_eval_in_expr(expr, strict_mode, possible_function_shadowing),
            span,
        },
        Stmt::Return { expr, span } => Stmt::Return {
            expr: expand_eval_in_expr(expr, strict_mode, possible_function_shadowing),
            span,
        },
        Stmt::Throw { expr, span } => Stmt::Throw {
            expr: expand_eval_in_expr(expr, strict_mode, possible_function_shadowing),
            span,
        },
        Stmt::If {
            condition,
            then_body,
            else_body,
            span,
        } => Stmt::If {
            condition: expand_eval_in_expr(condition, strict_mode, possible_function_shadowing),
            then_body: then_body
                .into_iter()
                .map(|s| expand_eval_in_stmt(s, strict_mode, possible_function_shadowing))
                .collect(),
            else_body: else_body
                .into_iter()
                .map(|s| expand_eval_in_stmt(s, strict_mode, possible_function_shadowing))
                .collect(),
            span,
        },
        Stmt::While { condition, body, span } => Stmt::While {
            condition: expand_eval_in_expr(condition, strict_mode, possible_function_shadowing),
            body: body
                .into_iter()
                .map(|s| expand_eval_in_stmt(s, strict_mode, possible_function_shadowing))
                .collect(),
            span,
        },
        Stmt::Block { statements, span } => Stmt::Block {
            statements: statements
                .into_iter()
                .map(|s| expand_eval_in_stmt(s, strict_mode, possible_function_shadowing))
                .collect(),
            span,
        },
        Stmt::DoWhile { body, condition, span } => Stmt::DoWhile {
            body: body
                .into_iter()
                .map(|s| expand_eval_in_stmt(s, strict_mode, possible_function_shadowing))
                .collect(),
            condition: expand_eval_in_expr(condition, strict_mode, possible_function_shadowing),
            span,
        },
        stmt => stmt,
    }
}

fn expand_eval_in_expr(expr: Expr, strict_mode: bool, possible_function_shadowing: bool) -> Expr {
    match expr {
        Expr::Call { callee, args, span } => {
            if let Expr::Ident { name, .. } = callee.as_ref()
                && name == "Function"
                && !possible_function_shadowing
                && let Some(parsed) = try_expand_function_constructor(&args, span, strict_mode)
            {
                return parsed;
            }
            Expr::Call {
                callee: Box::new(expand_eval_in_expr(
                    *callee,
                    strict_mode,
                    possible_function_shadowing,
                )),
                args: args
                    .into_iter()
                    .map(|a| expand_eval_in_expr(a, strict_mode, possible_function_shadowing))
                    .collect(),
                span,
            }
        }
        Expr::Binary { left, op, right, span } => Expr::Binary {
            left: Box::new(expand_eval_in_expr(
                *left,
                strict_mode,
                possible_function_shadowing,
            )),
            op,
            right: Box::new(expand_eval_in_expr(
                *right,
                strict_mode,
                possible_function_shadowing,
            )),
            span,
        },
        Expr::Unary { op, expr, span } => Expr::Unary {
            op,
            expr: Box::new(expand_eval_in_expr(
                *expr,
                strict_mode,
                possible_function_shadowing,
            )),
            span,
        },
        Expr::Ternary { condition, then_expr, else_expr, span } => Expr::Ternary {
            condition: Box::new(expand_eval_in_expr(
                *condition,
                strict_mode,
                possible_function_shadowing,
            )),
            then_expr: Box::new(expand_eval_in_expr(
                *then_expr,
                strict_mode,
                possible_function_shadowing,
            )),
            else_expr: Box::new(expand_eval_in_expr(
                *else_expr,
                strict_mode,
                possible_function_shadowing,
            )),
            span,
        },
        Expr::Member { object, property, span } => Expr::Member {
            object: Box::new(expand_eval_in_expr(
                *object,
                strict_mode,
                possible_function_shadowing,
            )),
            property,
            span,
        },
        Expr::Index { object, index, span } => Expr::Index {
            object: Box::new(expand_eval_in_expr(
                *object,
                strict_mode,
                possible_function_shadowing,
            )),
            index: Box::new(expand_eval_in_expr(
                *index,
                strict_mode,
                possible_function_shadowing,
            )),
            span,
        },
        Expr::OptionalCall { callee, args, span } => {
            if let Expr::Ident { name, .. } = callee.as_ref()
                && name == "Function"
                && !possible_function_shadowing
                && let Some(parsed) = try_expand_function_constructor(&args, span, strict_mode)
            {
                return parsed;
            }
            Expr::OptionalCall {
                callee: Box::new(expand_eval_in_expr(
                    *callee,
                    strict_mode,
                    possible_function_shadowing,
                )),
                args: args
                    .into_iter()
                    .map(|a| expand_eval_in_expr(a, strict_mode, possible_function_shadowing))
                    .collect(),
                span,
            }
        }
        Expr::New { expr, args, span } => {
            if let Expr::Ident { name, .. } = expr.as_ref()
                && name == "Function"
                && !possible_function_shadowing
                && let Some(parsed) = try_expand_function_constructor(&args, span, strict_mode)
            {
                return parsed;
            }
            Expr::New {
                expr: Box::new(expand_eval_in_expr(
                    *expr,
                    strict_mode,
                    possible_function_shadowing,
                )),
                args: args
                    .into_iter()
                    .map(|a| expand_eval_in_expr(a, strict_mode, possible_function_shadowing))
                    .collect(),
                span,
            }
        }
        other => other,
    }
}

/// Try to expand `Function("param1", ..., "body")` into a FunctionExpr.
fn try_expand_function_constructor(args: &[Expr], span: Span, strict_mode: bool) -> Option<Expr> {
    let strings: Vec<&str> = args.iter().filter_map(|arg| match arg {
        Expr::String { value, .. } => Some(value.as_str()),
        _ => None,
    }).collect();
    if strings.len() != args.len() {
        return None;
    }
    let (body_source, param_names): (&str, &[&str]) = match strings.split_last() {
        Some((body, params)) => (*body, params),
        None => ("", &[]),
    };
    let params_str = param_names.join(", ");
    let function_source = format!("function anonymous({params_str}) {{\n{body_source}\n}}");
    let tokens = crate::Lexer::new_with_strict_mode(&function_source, strict_mode)
        .tokenize().ok()?;
    let mut parser = Parser::new_with_strict_mode(tokens, strict_mode, &function_source);
    let stmts = parser.parse_program().ok()?;
    for stmt in stmts {
        if let Stmt::Function { name, params, body, is_generator, .. } = stmt {
            return Some(Expr::FunctionExpr {
                name,
                params,
                body,
                is_generator,
                origin: ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor,
                span,
                source_text: function_source,
            });
        }
    }
    None
}
