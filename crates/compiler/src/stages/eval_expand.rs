use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_ir::builtin_resolved::{EvalKind, EvalSource, ResolvedExpr, ResolvedStmt};
use ts2wasm_ir::builtin_resolver::resolve_builtins;
use ts2wasm_ir::name_resolver::resolve_names;
use ts2wasm_ir::name_resolver::{
    INTRINSIC_FUNCTION_CONSTRUCTOR_CALL, INTRINSIC_FUNCTION_CONSTRUCTOR_NEW,
};
use ts2wasm_syntax::FunctionExprOrigin;
/// Expand static literal dynamic-code expressions at compile time.
///
/// For direct or indirect eval("literal") where the source is a compile-time string literal:
/// 1. Parse the source with the frontend parser
/// 2. Run name resolution on the parsed AST
/// 3. Run builtin resolution
/// 4. Replace the Eval node with the resolved expression
///
/// Literal-only Function(...) / new Function(...) follows the same parse and resolve path,
/// but produces a generated non-capturing FunctionExpr. Runtime-source Function
/// constructor calls are rejected until the host.function.* lane exists.
///
/// Runtime-source eval is left as-is for the host lane.
pub(crate) fn expand_static_eval_fragments(
    resolved: Vec<ResolvedStmt>,
) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    resolved.into_iter().map(expand_stmt).collect()
}

fn expand_stmt(stmt: ResolvedStmt) -> Result<ResolvedStmt, Diagnostic> {
    match stmt {
        ResolvedStmt::Expr(expr) => Ok(ResolvedStmt::Expr(expand_expr(expr)?)),
        ResolvedStmt::Let(name, expr) => Ok(ResolvedStmt::Let(name, expand_expr(expr)?)),
        ResolvedStmt::Assign(name, expr) => Ok(ResolvedStmt::Assign(name, expand_expr(expr)?)),
        ResolvedStmt::Return(expr) => Ok(ResolvedStmt::Return(expand_expr(expr)?)),
        ResolvedStmt::Throw(expr) => Ok(ResolvedStmt::Throw(expand_expr(expr)?)),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => Ok(ResolvedStmt::If {
            condition: expand_expr(condition)?,
            then_body: then_body
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            else_body: else_body
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ResolvedStmt::While { condition, body } => Ok(ResolvedStmt::While {
            condition: expand_expr(condition)?,
            body: body
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ResolvedStmt::DoWhile { body, condition } => Ok(ResolvedStmt::DoWhile {
            body: body
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            condition: expand_expr(condition)?,
        }),
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => Ok(ResolvedStmt::For {
            init: match init {
                Some(boxed) => Some(Box::new(expand_stmt(*boxed)?)),
                None => None,
            },
            condition: condition.map(expand_expr).transpose()?,
            update: update.map(expand_expr).transpose()?,
            body: body
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ResolvedStmt::ForIn { var, iter, body } => Ok(ResolvedStmt::ForIn {
            var,
            iter: expand_expr(iter)?,
            body: body
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ResolvedStmt::ForOf { var, iter, body } => Ok(ResolvedStmt::ForOf {
            var,
            iter: expand_expr(iter)?,
            body: body
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ResolvedStmt::ForAwaitOf { var, iter, body } => Ok(ResolvedStmt::ForAwaitOf {
            var,
            iter: expand_expr(iter)?,
            body: body
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ResolvedStmt::Switch { expr, cases } => {
            let mut expanded_cases = Vec::new();
            for (cond, body) in cases {
                let expanded_cond = cond.map(expand_expr).transpose()?;
                let expanded_body = body
                    .into_iter()
                    .map(expand_stmt)
                    .collect::<Result<Vec<_>, _>>()?;
                expanded_cases.push((expanded_cond, expanded_body));
            }
            Ok(ResolvedStmt::Switch {
                expr: expand_expr(expr)?,
                cases: expanded_cases,
            })
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
        } => Ok(ResolvedStmt::TryCatch {
            try_block: try_block
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            catch_param,
            catch_block: catch_block
                .map(|b| {
                    b.into_iter()
                        .map(expand_stmt)
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            finally_block: finally_block
                .map(|b| {
                    b.into_iter()
                        .map(expand_stmt)
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        }),
        ResolvedStmt::Block { statements } => Ok(ResolvedStmt::Block {
            statements: statements
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ResolvedStmt::Labeled { label, body } => Ok(ResolvedStmt::Labeled {
            label,
            body: Box::new(expand_stmt(*body)?),
        }),
        ResolvedStmt::Break { label } => Ok(ResolvedStmt::Break { label }),
        ResolvedStmt::Continue { label } => Ok(ResolvedStmt::Continue { label }),
        ResolvedStmt::DestructureLet { pattern, expr } => Ok(ResolvedStmt::DestructureLet {
            pattern,
            expr: expand_expr(expr)?,
        }),
        ResolvedStmt::Export { name, expr } => Ok(ResolvedStmt::Export {
            name,
            expr: Box::new(expand_expr(*expr)?),
        }),
        ResolvedStmt::ModuleExportsAssign { expr } => Ok(ResolvedStmt::ModuleExportsAssign {
            expr: Box::new(expand_expr(*expr)?),
        }),
        ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::AmbientValue(_) => Ok(stmt),
    }
}

fn expand_expr(expr: ResolvedExpr) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
        ResolvedExpr::Eval {
            kind: EvalKind::Direct | EvalKind::Indirect,
            source: EvalSource::StaticLiteral(ref src),
            ..
        } => expand_static_eval_source(src),
        ResolvedExpr::Eval { .. } => {
            // Non-expandable eval (indirect or runtime source) — keep as-is.
            Ok(expr)
        }
        ResolvedExpr::Call { callee, args, span }
            if matches!(
                callee.as_ref(),
                ResolvedExpr::Ident(name) if name == INTRINSIC_FUNCTION_CONSTRUCTOR_CALL
            ) =>
        {
            expand_static_function_constructor(args, span)
        }
        ResolvedExpr::New {
            class_name,
            args,
            span,
        } if class_name == INTRINSIC_FUNCTION_CONSTRUCTOR_NEW => {
            expand_static_function_constructor(args, span)
        }
        // Recursively expand eval in sub-expressions.
        ResolvedExpr::Unary { op, expr: inner } => Ok(ResolvedExpr::Unary {
            op,
            expr: Box::new(expand_expr(*inner)?),
        }),
        ResolvedExpr::Binary { left, op, right } => Ok(ResolvedExpr::Binary {
            left: Box::new(expand_expr(*left)?),
            op,
            right: Box::new(expand_expr(*right)?),
        }),
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            span,
        } => Ok(ResolvedExpr::Ternary {
            condition: Box::new(expand_expr(*condition)?),
            then_expr: Box::new(expand_expr(*then_expr)?),
            else_expr: Box::new(expand_expr(*else_expr)?),
            span,
        }),
        ResolvedExpr::Call { callee, args, span } => Ok(ResolvedExpr::Call {
            callee: Box::new(expand_expr(*callee)?),
            args: args
                .into_iter()
                .map(expand_expr)
                .collect::<Result<Vec<_>, _>>()?,
            span,
        }),
        ResolvedExpr::New {
            class_name,
            args,
            span,
        } => Ok(ResolvedExpr::New {
            class_name,
            args: args
                .into_iter()
                .map(expand_expr)
                .collect::<Result<Vec<_>, _>>()?,
            span,
        }),
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            span,
        } => Ok(ResolvedExpr::MethodCall {
            object: Box::new(expand_expr(*object)?),
            method,
            args: args
                .into_iter()
                .map(expand_expr)
                .collect::<Result<Vec<_>, _>>()?,
            span,
        }),
        ResolvedExpr::PropertyAccess { object, key, span } => Ok(ResolvedExpr::PropertyAccess {
            object: Box::new(expand_expr(*object)?),
            key,
            span,
        }),
        ResolvedExpr::OptionalPropertyAccess { object, key, span } => {
            Ok(ResolvedExpr::OptionalPropertyAccess {
                object: Box::new(expand_expr(*object)?),
                key,
                span,
            })
        }
        ResolvedExpr::ComputedIndex { object, index } => Ok(ResolvedExpr::ComputedIndex {
            object: Box::new(expand_expr(*object)?),
            index: Box::new(expand_expr(*index)?),
        }),
        ResolvedExpr::OptionalComputedIndex {
            object,
            index,
            span,
        } => Ok(ResolvedExpr::OptionalComputedIndex {
            object: Box::new(expand_expr(*object)?),
            index: Box::new(expand_expr(*index)?),
            span,
        }),
        ResolvedExpr::Assign { name, expr: inner } => Ok(ResolvedExpr::Assign {
            name,
            expr: Box::new(expand_expr(*inner)?),
        }),
        ResolvedExpr::Array(elements) => {
            let expanded = elements
                .into_iter()
                .map(|el| match el {
                    ts2wasm_ir::builtin_resolved::ResolvedArrayElement::Present(e) => {
                        Ok(ts2wasm_ir::builtin_resolved::ResolvedArrayElement::Present(
                            expand_expr(e)?,
                        ))
                    }
                    hole @ ts2wasm_ir::builtin_resolved::ResolvedArrayElement::Hole => Ok(hole),
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ResolvedExpr::Array(expanded))
        }
        // Leaf expressions and other types — no recursive expansion needed.
        other => Ok(other),
    }
}

fn expand_static_eval_source(src: &str) -> Result<ResolvedExpr, Diagnostic> {
    let tokens = ts2wasm_frontend::Lexer::new(src)
        .tokenize()
        .map_err(|e| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("eval source lex error: {e}"),
            span: None,
            phase: None,
        })?;
    let program = ts2wasm_frontend::Parser::new(tokens, src)
        .parse_program()
        .map_err(|e| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("eval source parse error: {e}"),
            span: None,
            phase: None,
        })?;

    let name_resolved = resolve_names(&program)?;
    let builtin_resolved = resolve_builtins(&name_resolved)?;

    extract_completion_value(builtin_resolved)
}

fn expand_static_function_constructor(
    args: Vec<ResolvedExpr>,
    span: ts2wasm_source::Span,
) -> Result<ResolvedExpr, Diagnostic> {
    let mut strings = Vec::with_capacity(args.len());
    for arg in args {
        match arg {
            ResolvedExpr::String(value) => strings.push(value),
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedBuiltin,
                    message: "dynamic Function constructor requires the host.function.* lane"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            }
        }
    }

    let (body_source, param_names): (&str, &[String]) = match strings.split_last() {
        Some((body, params)) => (body.as_str(), params),
        None => ("", &[]),
    };
    let params_source = param_names
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    let function_source = format!("function anonymous({params_source}) {{\n{body_source}\n}}");

    let tokens = ts2wasm_frontend::Lexer::new(&function_source)
        .tokenize()
        .map_err(|e| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("Function constructor source lex error: {e}"),
            span: Some(span),
            phase: None,
        })?;
    let program = ts2wasm_frontend::Parser::new(tokens, &function_source)
        .parse_program()
        .map_err(|e| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("Function constructor source parse error: {e}"),
            span: Some(span),
            phase: None,
        })?;

    let name_resolved = resolve_names(&program)?;
    let builtin_resolved = resolve_builtins(&name_resolved)?;
    for stmt in builtin_resolved {
        if let ResolvedStmt::Function {
            name,
            params,
            body,
            is_generator,
            source_text,
            ..
        } = stmt
        {
            let body = body
                .into_iter()
                .map(expand_stmt)
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(ResolvedExpr::FunctionExpr {
                name,
                params,
                body,
                is_generator,
                origin: FunctionExprOrigin::FunctionConstructor,
                source_text,
            });
        }
    }

    Err(Diagnostic {
        code: DiagCode::InvariantViolation,
        message: "Function constructor expansion did not produce a function".to_owned(),
        span: Some(span),
        phase: None,
    })
}

/// Extract the completion value from a resolved program body.
///
/// * Empty block → `ResolvedExpr::Undefined`
/// * Single expression statement → the expression itself
/// * Multiple statements → the last statement's completion value
fn extract_completion_value(stmts: Vec<ResolvedStmt>) -> Result<ResolvedExpr, Diagnostic> {
    let mut last_expr: Option<ResolvedExpr> = None;
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Expr(expr) => last_expr = Some(expr),
            ResolvedStmt::Let(_, expr)
            | ResolvedStmt::Assign(_, expr)
            | ResolvedStmt::Return(expr) => {
                last_expr = Some(expr);
            }
            _ => last_expr = Some(ResolvedExpr::Undefined),
        }
    }
    Ok(last_expr.unwrap_or(ResolvedExpr::Undefined))
}
