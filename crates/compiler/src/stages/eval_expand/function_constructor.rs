use crate::stages::eval_expand::*;

pub(super) fn expand_function_constructor(
    plan: FunctionConstructorPlan,
) -> Result<ResolvedExpr, Diagnostic> {
    if !plan.static_source_is_consistent() {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedEval,
            message: "Function constructor static source metadata does not match plan".to_owned(),
            span: Some(plan.span),
            phase: None,
        });
    }
    let FunctionConstructorPlan {
        kind,
        args,
        static_source,
        host_policy,
        span,
    } = plan;
    let expected_host_policy = FunctionConstructorHostPolicy::for_static_source(&static_source);
    if host_policy != expected_host_policy {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedEval,
            message: format!(
                "Function constructor host policy {host_policy:?} does not match source classification"
            ),
            span: Some(span),
            phase: None,
        });
    }
    if host_policy != FunctionConstructorHostPolicy::AotOnly {
        return Ok(function_constructor_host_lane(kind, args, span));
    }

    let Some(static_source) = static_source else {
        return Ok(function_constructor_host_lane(kind, args, span));
    };
    validate_function_constructor_parse_goals(static_source.parse_goals, span)?;
    let function_name = static_source.generated_function.name.clone();
    let generated_source_text = static_source.generated_source_text();
    let function_source = static_source.synthetic_function_source();

    let tokens = ts2wasm_frontend::Lexer::new(&function_source)
        .tokenize()
        .map_err(|e| {
            Diagnostic::unsupported_at(span, format!("Function constructor source lex error: {e}"))
        })?;
    let program = ts2wasm_frontend::Parser::new(tokens, &function_source)
        .parse_program()
        .map_err(|e| {
            Diagnostic::unsupported_at(
                span,
                format!("Function constructor source parse error: {e}"),
            )
        })?;
    validate_static_function_constructor_wrapper_shape(&program, span)?;
    validate_static_function_constructor_early_errors(&program, span)?;

    let name_resolved = resolve_names(&program)?;
    let builtin_resolved = resolve_builtins(&name_resolved)?;
    for stmt in builtin_resolved {
        if let ResolvedStmt::Function {
            name: _,
            params,
            body,
            is_generator,
            source_text: _,
            ..
        } = stmt
        {
            let generated_function = static_source
                .generated_function
                .with_length(function_constructor_length_metadata(&params));
            let mut function_ctx = EvalExpansionContext::new();
            function_ctx.declare_params(&params);
            let body = expand_stmts(body, &mut function_ctx)?;
            return Ok(ResolvedExpr::FunctionExpr {
                name: function_name,
                params,
                body,
                is_generator,
                origin: FunctionExprOrigin::FunctionConstructor,
                constructor_metadata: Some(generated_function),
                source_text: generated_source_text,
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

pub(super) fn validate_static_function_constructor_wrapper_shape(
    program: &[Stmt],
    span: ts2wasm_source::Span,
) -> Result<(), Diagnostic> {
    if !matches!(program, [Stmt::Function { .. }]) {
        return Err(function_constructor_syntax_error(
            "Function constructor parameters must parse as a single FormalParameters list",
            span,
        ));
    }
    Ok(())
}

pub(super) fn validate_function_constructor_parse_goals(
    parse_goals: FunctionConstructorParseGoals,
    span: ts2wasm_source::Span,
) -> Result<(), Diagnostic> {
    if parse_goals.params != FunctionConstructorParseGoal::FormalParameters {
        return Err(function_constructor_syntax_error(
            "Function constructor parameters must use the FormalParameters parse goal",
            span,
        ));
    }
    if parse_goals.body != FunctionConstructorParseGoal::FunctionBody {
        return Err(function_constructor_syntax_error(
            "Function constructor body must use the FunctionBody parse goal",
            span,
        ));
    }
    Ok(())
}

pub(super) fn function_constructor_host_lane(
    kind: FunctionConstructorKind,
    args: Vec<ResolvedExpr>,
    span: ts2wasm_source::Span,
) -> ResolvedExpr {
    ResolvedExpr::FunctionConstructor {
        plan: FunctionConstructorPlan::new(kind, args, span),
    }
}

pub(super) fn function_constructor_length_metadata(params: &[ResolvedParam]) -> usize {
    params
        .iter()
        .take_while(|param| param.default.is_none() && !param.is_rest)
        .count()
}

pub(super) fn validate_static_function_constructor_early_errors(
    program: &[Stmt],
    span: ts2wasm_source::Span,
) -> Result<(), Diagnostic> {
    let Some(Stmt::Function { params, body, .. }) = program.first() else {
        return Ok(());
    };

    let has_non_simple_params = params.iter().any(|(name, default, is_rest)| {
        default.is_some() || *is_rest || !is_simple_identifier(name)
    });
    let mut seen = HashSet::new();
    if has_non_simple_params {
        for (name, _, _) in params {
            for bound_name in function_constructor_bound_names(name) {
                if !seen.insert(bound_name) {
                    return Err(function_constructor_syntax_error(
                        "Duplicate parameter name not allowed in this context",
                        span,
                    ));
                }
            }
        }
    }

    if !block_has_use_strict_directive(body) {
        return Ok(());
    }

    seen.clear();
    for (name, default, is_rest) in params {
        if default.is_some() || *is_rest || !is_simple_identifier(name) {
            return Err(function_constructor_syntax_error(
                "Illegal 'use strict' directive in function with non-simple parameter list",
                span,
            ));
        }
        if matches!(name.as_str(), "eval" | "arguments") {
            return Err(function_constructor_syntax_error(
                "Unexpected eval or arguments in strict mode",
                span,
            ));
        }
        if !seen.insert(name.to_owned()) {
            return Err(function_constructor_syntax_error(
                "Duplicate parameter name not allowed in this context",
                span,
            ));
        }
    }

    Ok(())
}

pub(super) fn function_constructor_bound_names(param: &str) -> Vec<String> {
    let text = param.trim();
    if is_simple_identifier(text) {
        return vec![text.to_owned()];
    }
    if let Some(inner) = text.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
        return inner
            .split(',')
            .filter_map(object_binding_name)
            .collect::<Vec<_>>();
    }
    if let Some(inner) = text.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        return inner
            .split(',')
            .filter_map(array_binding_name)
            .collect::<Vec<_>>();
    }
    Vec::new()
}

pub(super) fn object_binding_name(part: &str) -> Option<String> {
    let mut text = part.trim().trim_start_matches("...").trim();
    if text.is_empty() {
        return None;
    }
    if let Some((_, binding)) = text.split_once(':') {
        text = binding.trim();
    }
    if let Some((binding, _)) = text.split_once('=') {
        text = binding.trim();
    }
    is_simple_identifier(text).then(|| text.to_owned())
}

pub(super) fn array_binding_name(part: &str) -> Option<String> {
    let text = part
        .trim()
        .trim_start_matches("...")
        .split_once('=')
        .map_or_else(
            || part.trim().trim_start_matches("..."),
            |(binding, _)| binding.trim(),
        );
    is_simple_identifier(text).then(|| text.to_owned())
}

pub(super) fn block_has_use_strict_directive(stmts: &[Stmt]) -> bool {
    for stmt in stmts {
        match stmt {
            Stmt::Expr {
                expr: Expr::String { value, .. },
                ..
            } if value == "use strict" => return true,
            Stmt::Expr {
                expr: Expr::String { .. },
                ..
            } => continue,
            _ => return false,
        }
    }
    false
}

pub(super) fn is_simple_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first == '$' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch == '$' || ch.is_ascii_alphanumeric())
}

pub(super) fn function_constructor_syntax_error(
    message: &str,
    span: ts2wasm_source::Span,
) -> Diagnostic {
    Diagnostic::unsupported_at(
        span,
        format!("Function constructor source parse error: SyntaxError: {message}"),
    )
}

/// Extract the completion value from a resolved program body.
///
/// Produces a completion-plan expression so lower IR can evaluate all eval-code
/// side effects while preserving the last non-empty completion value exactly once.
pub(super) fn extract_completion_value(
    source: &str,
    fragment_plan: &EvalFragmentPlan,
    ast_stmts: &[Stmt],
    stmts: Vec<ResolvedStmt>,
    caller_is_strict: bool,
    eval_is_strict: bool,
    var_landing: EvalVarLanding,
    eval_declarations: &[String],
    function_hoists: Vec<EvalFunctionHoist>,
    global_var_hoists: &[String],
    global_function_hoists: Vec<EvalFunctionHoist>,
) -> Result<ResolvedExpr, Diagnostic> {
    let mut steps = Vec::new();
    steps.extend(
        global_var_hoists
            .iter()
            .cloned()
            .map(|name| EvalCompletionStep::GlobalVarLet {
                name,
                init: ResolvedExpr::Undefined,
            }),
    );
    steps.extend(global_function_hoists.into_iter().map(|hoist| {
        EvalCompletionStep::GlobalFunctionDecl {
            name: hoist.name,
            params: hoist.params,
            body: hoist.body,
            is_generator: hoist.is_generator,
            is_async: hoist.is_async,
            source_text: hoist.source_text,
        }
    }));
    steps.extend(eval_completion_steps(source, ast_stmts, stmts, var_landing));
    Ok(fragment_plan.completion_expr_with_context(
        caller_is_strict,
        eval_is_strict,
        EvalDeclarationPlan {
            var_names: eval_declarations.to_vec(),
            function_hoists: function_hoists.clone(),
        },
        steps,
    ))
}
