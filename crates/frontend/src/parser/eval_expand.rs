/// Post-parse pass: expand `eval("expr")` in expression position.
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
            {
                if let [Expr::String { value, .. }] = args.as_slice() {
                    if let Some(parsed) = try_parse_eval_source(value, strict_mode) {
                        return parsed;
                    }
                }
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
        Expr::OptionalCall { callee, args, span } => Expr::OptionalCall {
            callee: Box::new(expand_eval_in_expr(*callee, strict_mode)),
            args: args.into_iter().map(|a| expand_eval_in_expr(a, strict_mode)).collect(),
            span,
        },
        Expr::New { expr, args, span } => Expr::New {
            expr: Box::new(expand_eval_in_expr(*expr, strict_mode)),
            args: args.into_iter().map(|a| expand_eval_in_expr(a, strict_mode)).collect(),
            span,
        },
        other => other,
    }
}

fn try_parse_eval_source(source: &str, strict_mode: bool) -> Option<Expr> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return None;
    }
    let tokens = crate::Lexer::new_with_strict_mode(trimmed, strict_mode)
        .tokenize()
        .ok()?;
    let mut parser = Parser::new_with_strict_mode(tokens, strict_mode, trimmed);
    let stmts = parser.parse_program().ok()?;
    let mut iter = stmts.into_iter();
    let first = iter.next()?;
    if iter.next().is_some() {
        return None;
    }
    match first {
        Stmt::Expr { expr, .. } => Some(expr),
        _ => None,
    }
}
