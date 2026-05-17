/// Post-parse pass: expand `eval("expr")` and `Function("...", "body")` calls.
pub(super) fn expand_eval_in_statements(stmts: Vec<Stmt>, strict_mode: bool) -> Vec<Stmt> {
    stmts
        .into_iter()
        .map(|stmt| expand_eval_in_stmt(stmt, strict_mode))
        .collect()
}

fn expand_eval_in_stmt(stmt: Stmt, strict_mode: bool) -> Stmt {
    match stmt {
        Stmt::Expr { expr, span } => Stmt::Expr {
            expr: expand_eval_in_expr(expr, strict_mode),
            span,
        },
        Stmt::Let {
            name,
            expr,
            is_var,
            span,
        } => Stmt::Let {
            name,
            expr: expand_eval_in_expr(expr, strict_mode),
            is_var,
            span,
        },
        Stmt::Assign { name, expr, span } => Stmt::Assign {
            name,
            expr: expand_eval_in_expr(expr, strict_mode),
            span,
        },
        Stmt::Return { expr, span } => Stmt::Return {
            expr: expand_eval_in_expr(expr, strict_mode),
            span,
        },
        Stmt::Throw { expr, span } => Stmt::Throw {
            expr: expand_eval_in_expr(expr, strict_mode),
            span,
        },
        Stmt::If {
            condition,
            then_body,
            else_body,
            span,
        } => Stmt::If {
            condition: expand_eval_in_expr(condition, strict_mode),
            then_body: then_body.into_iter().map(|s| expand_eval_in_stmt(s, strict_mode)).collect(),
            else_body: else_body.into_iter().map(|s| expand_eval_in_stmt(s, strict_mode)).collect(),
            span,
        },
        Stmt::While { condition, body, span } => Stmt::While {
            condition: expand_eval_in_expr(condition, strict_mode),
            body: body.into_iter().map(|s| expand_eval_in_stmt(s, strict_mode)).collect(),
            span,
        },
        Stmt::Block { statements, span } => Stmt::Block {
            statements: statements.into_iter().map(|s| expand_eval_in_stmt(s, strict_mode)).collect(),
            span,
        },
        Stmt::DoWhile { body, condition, span } => Stmt::DoWhile {
            body: body.into_iter().map(|s| expand_eval_in_stmt(s, strict_mode)).collect(),
            condition: expand_eval_in_expr(condition, strict_mode),
            span,
        },
        stmt => stmt,
    }
}

fn expand_eval_in_expr(expr: Expr, strict_mode: bool) -> Expr {
    match expr {
        Expr::Call { callee, args, span } => {
            if let Expr::Ident { name, .. } = callee.as_ref()
                && name == "eval"
                && let [Expr::String { value, .. }] = args.as_slice()
                && let Some(parsed) = try_parse_eval_source(value, strict_mode)
            {
                return parsed;
            }
            if let Expr::Ident { name, .. } = callee.as_ref()
                && name == "Function"
                && let Some(parsed) = try_expand_function_constructor(&args, span, strict_mode)
            {
                return parsed;
            }
            Expr::Call {
                callee: Box::new(expand_eval_in_expr(*callee, strict_mode)),
                args: args.into_iter().map(|a| expand_eval_in_expr(a, strict_mode)).collect(),
                span,
            }
        }
        Expr::Binary { left, op, right, span } => Expr::Binary {
            left: Box::new(expand_eval_in_expr(*left, strict_mode)),
            op,
            right: Box::new(expand_eval_in_expr(*right, strict_mode)),
            span,
        },
        Expr::Unary { op, expr, span } => Expr::Unary {
            op,
            expr: Box::new(expand_eval_in_expr(*expr, strict_mode)),
            span,
        },
        Expr::Ternary { condition, then_expr, else_expr, span } => Expr::Ternary {
            condition: Box::new(expand_eval_in_expr(*condition, strict_mode)),
            then_expr: Box::new(expand_eval_in_expr(*then_expr, strict_mode)),
            else_expr: Box::new(expand_eval_in_expr(*else_expr, strict_mode)),
            span,
        },
        Expr::Member { object, property, span } => Expr::Member {
            object: Box::new(expand_eval_in_expr(*object, strict_mode)),
            property,
            span,
        },
        Expr::Index { object, index, span } => Expr::Index {
            object: Box::new(expand_eval_in_expr(*object, strict_mode)),
            index: Box::new(expand_eval_in_expr(*index, strict_mode)),
            span,
        },
        Expr::OptionalCall { callee, args, span } => {
            if let Expr::Ident { name, .. } = callee.as_ref()
                && (name == "eval" || name == "Function")
                && let [Expr::String { value, .. }] = args.as_slice()
            {
                if name == "eval"
                    && let Some(parsed) = try_parse_eval_source(value, strict_mode)
                {
                    return parsed;
                }
                if name == "Function"
                    && let Some(parsed) = try_expand_function_constructor(&args, span, strict_mode)
                {
                    return parsed;
                }
            }
            Expr::OptionalCall {
                callee: Box::new(expand_eval_in_expr(*callee, strict_mode)),
                args: args.into_iter().map(|a| expand_eval_in_expr(a, strict_mode)).collect(),
                span,
            }
        }
        Expr::New { expr, args, span } => {
            if let Expr::Ident { name, .. } = expr.as_ref()
                && name == "Function"
                && let Some(parsed) = try_expand_function_constructor(&args, span, strict_mode)
            {
                return parsed;
            }
            Expr::New {
                expr: Box::new(expand_eval_in_expr(*expr, strict_mode)),
                args: args.into_iter().map(|a| expand_eval_in_expr(a, strict_mode)).collect(),
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
    if strings.len() != args.len() || args.is_empty() {
        return None;
    }
    let body_source = strings[strings.len() - 1];
    let param_names = &strings[..strings.len() - 1];
    let params_str = param_names.join(", ");
    let function_source = format!("function anonymous({params_str}) {{\n{body_source}\n}}");
    let tokens = crate::Lexer::new_with_strict_mode(&function_source, strict_mode)
        .tokenize().ok()?;
    let mut parser = Parser::new_with_strict_mode(tokens, strict_mode, &function_source);
    let stmts = parser.parse_program().ok()?;
    for stmt in stmts {
        if let Stmt::Function { name, params, body, is_generator, .. } = stmt {
            return Some(Expr::FunctionExpr { name, params, body, is_generator, span, source_text: function_source });
        }
    }
    None
}

fn try_parse_eval_source(source: &str, strict_mode: bool) -> Option<Expr> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return None;
    }
    let tokens = crate::Lexer::new_with_strict_mode(trimmed, strict_mode)
        .tokenize().ok()?;
    let mut parser = Parser::new_with_strict_mode(tokens, strict_mode, trimmed);
    let stmts = parser.parse_program().ok()?;
    let mut completion: Option<Expr> = None;
    for stmt in stmts {
        match stmt {
            Stmt::Expr { expr, .. } => completion = Some(expr),
            _ => return None,
        }
    }
    completion
}
