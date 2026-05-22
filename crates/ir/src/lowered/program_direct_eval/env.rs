use crate::lowered::program_direct_eval::*;

pub(super) fn is_direct_eval_env_binding_name(name: &str) -> bool {
    !name.is_empty()
}

pub(super) fn dynamic_direct_eval_known_source<'a>(
    expr: &'a ResolvedExpr,
    known_sources: &'a HashMap<String, String>,
) -> Option<&'a str> {
    let ResolvedExpr::Eval { plan } = expr else {
        return None;
    };
    if plan.kind != EvalKind::Direct {
        return None;
    }
    if plan.caller_is_strict {
        return None;
    }
    let EvalSource::Runtime(source) = &plan.source else {
        return None;
    };
    match source.as_ref() {
        ResolvedExpr::Ident(name) => known_sources.get(name).map(String::as_str),
        ResolvedExpr::String(value) => Some(value.as_str()),
        _ => None,
    }
}

pub(super) fn collect_unassigned_string_bindings(
    stmts: &[ResolvedStmt],
) -> HashMap<String, String> {
    let mut candidates = HashMap::new();
    let mut assigned = HashSet::new();
    collect_unassigned_string_bindings_from_stmts(stmts, &mut candidates, &mut assigned);
    for name in assigned {
        candidates.remove(&name);
    }
    candidates
}

pub(super) fn collect_unassigned_string_bindings_from_stmts(
    stmts: &[ResolvedStmt],
    candidates: &mut HashMap<String, String>,
    assigned: &mut HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Let(name, ResolvedExpr::String(value)) => {
                candidates.insert(name.clone(), value.clone());
            }
            ResolvedStmt::Let(name, _) | ResolvedStmt::Assign(name, _) => {
                candidates.remove(name);
                assigned.insert(name.clone());
            }
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_unassigned_string_bindings_from_stmts(then_body, candidates, assigned);
                collect_unassigned_string_bindings_from_stmts(else_body, candidates, assigned);
            }
            ResolvedStmt::While { body, .. }
            | ResolvedStmt::DoWhile { body, .. }
            | ResolvedStmt::Block { statements: body } => {
                collect_unassigned_string_bindings_from_stmts(body, candidates, assigned);
            }
            ResolvedStmt::For { init, body, .. } => {
                if let Some(init) = init {
                    collect_unassigned_string_bindings_from_stmts(
                        std::slice::from_ref(init.as_ref()),
                        candidates,
                        assigned,
                    );
                }
                collect_unassigned_string_bindings_from_stmts(body, candidates, assigned);
            }
            ResolvedStmt::ForIn { body, .. }
            | ResolvedStmt::ForOf { body, .. }
            | ResolvedStmt::ForAwaitOf { body, .. } => {
                collect_unassigned_string_bindings_from_stmts(body, candidates, assigned);
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_unassigned_string_bindings_from_stmts(body, candidates, assigned);
                }
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_unassigned_string_bindings_from_stmts(try_block, candidates, assigned);
                if let Some(catch_block) = catch_block {
                    collect_unassigned_string_bindings_from_stmts(
                        catch_block,
                        candidates,
                        assigned,
                    );
                }
                if let Some(finally_block) = finally_block {
                    collect_unassigned_string_bindings_from_stmts(
                        finally_block,
                        candidates,
                        assigned,
                    );
                }
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_unassigned_string_bindings_from_stmts(
                    std::slice::from_ref(body.as_ref()),
                    candidates,
                    assigned,
                );
            }
            _ => {}
        }
    }
}

pub(super) fn collect_eval_var_function_names(source: &str, names: &mut HashSet<String>) {
    names.extend(eval_var_and_function_names(source));
}

pub(super) fn collect_eval_function_names(source: &str, names: &mut HashSet<String>) {
    names.extend(eval_function_names(source));
}

pub(super) fn collect_block_declared_names(stmts: &[ResolvedStmt], names: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::AmbientValue(name)
            | ResolvedStmt::Let(name, _)
            | ResolvedStmt::Assign(name, _)
            | ResolvedStmt::Function { name, .. }
            | ResolvedStmt::ForIn { var: name, .. }
            | ResolvedStmt::ForOf { var: name, .. }
            | ResolvedStmt::ForAwaitOf { var: name, .. }
            | ResolvedStmt::Export { name, .. } => {
                if is_direct_eval_env_binding_name(name) {
                    names.insert(name.clone());
                }
            }
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_block_declared_names(then_body, names);
                collect_block_declared_names(else_body, names);
            }
            ResolvedStmt::While { body, .. }
            | ResolvedStmt::DoWhile { body, .. }
            | ResolvedStmt::Block {
                statements: body, ..
            } => {
                collect_block_declared_names(body, names);
            }
            ResolvedStmt::For { init, body, .. } => {
                if let Some(init) = init {
                    collect_block_declared_names(std::slice::from_ref(init.as_ref()), names);
                }
                collect_block_declared_names(body, names);
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_param,
                catch_block,
                finally_block,
            } => {
                collect_block_declared_names(try_block, names);
                if let Some(block) = catch_block {
                    if let Some(catch_param) = catch_param
                        && block_contains_dynamic_direct_eval(block)
                        && is_direct_eval_env_binding_name(catch_param)
                    {
                        names.insert(catch_param.to_string());
                    }
                    collect_block_declared_names(block, names);
                }
                if let Some(block) = finally_block {
                    collect_block_declared_names(block, names);
                }
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_block_declared_names(body, names);
                }
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_block_declared_names(std::slice::from_ref(body.as_ref()), names);
            }
            ResolvedStmt::ClassDecl { name, .. } => {
                if is_direct_eval_env_binding_name(name) {
                    names.insert(name.clone());
                }
            }
            ResolvedStmt::DestructureLet { .. }
            | ResolvedStmt::DestructureAssign { .. }
            | ResolvedStmt::Expr(_)
            | ResolvedStmt::Return(_)
            | ResolvedStmt::Throw(_)
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. }
            | ResolvedStmt::ModuleExportsAssign { .. } => {}
        }
    }
}

pub(crate) fn block_contains_dynamic_direct_eval(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_contains_dynamic_direct_eval)
}

pub(super) fn stmt_contains_dynamic_direct_eval(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::Assign(_, expr)
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr)
        | ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::DestructureAssign { expr, .. } => expr_contains_dynamic_direct_eval(expr),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_contains_dynamic_direct_eval(condition)
                || block_contains_dynamic_direct_eval(then_body)
                || block_contains_dynamic_direct_eval(else_body)
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
            expr_contains_dynamic_direct_eval(condition) || block_contains_dynamic_direct_eval(body)
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_contains_dynamic_direct_eval(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block_contains_dynamic_direct_eval(block))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block_contains_dynamic_direct_eval(block))
        }
        ResolvedStmt::Switch { expr, cases } => {
            expr_contains_dynamic_direct_eval(expr)
                || cases.iter().any(|(case_expr, body)| {
                    case_expr
                        .as_ref()
                        .is_some_and(expr_contains_dynamic_direct_eval)
                        || block_contains_dynamic_direct_eval(body)
                })
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            init.as_ref()
                .is_some_and(|init| stmt_contains_dynamic_direct_eval(init))
                || condition
                    .as_ref()
                    .is_some_and(expr_contains_dynamic_direct_eval)
                || update
                    .as_ref()
                    .is_some_and(expr_contains_dynamic_direct_eval)
                || block_contains_dynamic_direct_eval(body)
        }
        ResolvedStmt::ForIn { iter, body, .. }
        | ResolvedStmt::ForOf { iter, body, .. }
        | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
            expr_contains_dynamic_direct_eval(iter) || block_contains_dynamic_direct_eval(body)
        }
        ResolvedStmt::Labeled { body, .. } => stmt_contains_dynamic_direct_eval(body),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_contains_dynamic_direct_eval(expr)
        }
        ResolvedStmt::Block { statements } => block_contains_dynamic_direct_eval(statements),
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

pub(super) fn expr_contains_dynamic_direct_eval(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Eval {
            plan:
                crate::builtin_resolved::EvalFragmentPlan {
                    kind: EvalKind::Direct,
                    source: EvalSource::Runtime(_),
                    ..
                },
        } => true,
        ResolvedExpr::Eval { plan } => match &plan.source {
            EvalSource::Runtime(expr) => expr_contains_dynamic_direct_eval(expr),
            EvalSource::StaticLiteral(_) | EvalSource::NonStringStatic(_) => false,
        },
        ResolvedExpr::Await { expr }
        | ResolvedExpr::Spread(expr)
        | ResolvedExpr::Unary { expr, .. }
        | ResolvedExpr::Yield {
            expr: Some(expr), ..
        } => expr_contains_dynamic_direct_eval(expr),
        ResolvedExpr::Binary { left, right, .. } => {
            expr_contains_dynamic_direct_eval(left) || expr_contains_dynamic_direct_eval(right)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_dynamic_direct_eval(condition)
                || expr_contains_dynamic_direct_eval(then_expr)
                || expr_contains_dynamic_direct_eval(else_expr)
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            expr_contains_dynamic_direct_eval(callee)
                || args.iter().any(expr_contains_dynamic_direct_eval)
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            expr_contains_dynamic_direct_eval(object)
                || args.iter().any(expr_contains_dynamic_direct_eval)
        }
        ResolvedExpr::Array(elements) => elements.iter().any(|element| {
            matches!(element, ResolvedArrayElement::Present(expr) if expr_contains_dynamic_direct_eval(expr))
        }),
        ResolvedExpr::Object(props) => props.iter().any(|prop| {
            prop.computed_key()
                .is_some_and(expr_contains_dynamic_direct_eval)
                || expr_contains_dynamic_direct_eval(prop.value())
        }),
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            expr_contains_dynamic_direct_eval(object) || expr_contains_dynamic_direct_eval(index)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(expr_contains_dynamic_direct_eval)
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            plan.args.iter().any(expr_contains_dynamic_direct_eval)
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
            expr_contains_dynamic_direct_eval(object)
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            expr_contains_dynamic_direct_eval(object) || expr_contains_dynamic_direct_eval(value)
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            expr_contains_dynamic_direct_eval(object)
                || expr_contains_dynamic_direct_eval(key)
                || expr_contains_dynamic_direct_eval(value)
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
            expr_contains_dynamic_direct_eval(expr)
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_contains_dynamic_direct_eval(object) || expr_contains_dynamic_direct_eval(expr)
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_contains_dynamic_direct_eval(key) || expr_contains_dynamic_direct_eval(expr)
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            expr_contains_dynamic_direct_eval(object)
                || expr_contains_dynamic_direct_eval(key)
                || expr_contains_dynamic_direct_eval(expr)
        }
        ResolvedExpr::Sequence(exprs) => exprs.iter().any(expr_contains_dynamic_direct_eval),
        ResolvedExpr::EvalCompletion(steps) => steps
            .iter()
            .filter_map(|step| step.expr())
            .any(expr_contains_dynamic_direct_eval),
        ResolvedExpr::ArrowFn {
            body, body_stmts, ..
        } => {
            block_contains_dynamic_direct_eval(body_stmts) || expr_contains_dynamic_direct_eval(body)
        }
        ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::Yield { expr: None, .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin_resolved::{EvalFragmentPlan, EvalKind};
    use ts2wasm_source::Span;

    #[test]
    fn collects_created_var_from_known_runtime_direct_eval_source() {
        let body = vec![
            ResolvedStmt::Let(
                "source".to_owned(),
                ResolvedExpr::String("var created = 7; created".to_owned()),
            ),
            ResolvedStmt::Expr(ResolvedExpr::MethodCall {
                object: Box::new(ResolvedExpr::Ident("console".to_owned())),
                method: "log".to_owned(),
                args: vec![ResolvedExpr::Eval {
                    plan: EvalFragmentPlan::new(
                        EvalKind::Direct,
                        EvalSource::Runtime(Box::new(ResolvedExpr::Ident("source".to_owned()))),
                        false,
                        Span::generated("eval"),
                    ),
                }],
                span: Span::generated("call"),
            }),
        ];

        let names = collect_dynamic_direct_eval_created_binding_names(&body);
        assert!(names.contains("created"));
    }

    #[test]
    fn collects_created_destructuring_vars_from_known_runtime_direct_eval_source() {
        let body = vec![
            ResolvedStmt::Let(
                "source".to_owned(),
                ResolvedExpr::String(
                    "var {value: created, nested: {leaf}} = {value: 4, nested: {leaf: 5}}; created + leaf".to_owned(),
                ),
            ),
            ResolvedStmt::Expr(ResolvedExpr::Eval {
                plan: EvalFragmentPlan::new(
                    EvalKind::Direct,
                    EvalSource::Runtime(Box::new(ResolvedExpr::Ident("source".to_owned()))),
                    false,
                    Span::generated("eval"),
                ),
            }),
        ];

        let names = collect_dynamic_direct_eval_created_binding_names(&body);
        assert!(names.contains("created"));
        assert!(names.contains("leaf"));
    }

    #[test]
    fn collects_created_function_from_known_runtime_direct_eval_source() {
        let body = vec![
            ResolvedStmt::Let(
                "source".to_owned(),
                ResolvedExpr::String("function created() { return 7; } created()".to_owned()),
            ),
            ResolvedStmt::Expr(ResolvedExpr::Eval {
                plan: EvalFragmentPlan::new(
                    EvalKind::Direct,
                    EvalSource::Runtime(Box::new(ResolvedExpr::Ident("source".to_owned()))),
                    false,
                    Span::generated("eval"),
                ),
            }),
        ];

        let names = collect_dynamic_direct_eval_created_function_names(&body);
        assert!(names.contains("created"));
    }

    #[test]
    fn skips_created_names_from_strict_caller_runtime_direct_eval_source() {
        let body = vec![
            ResolvedStmt::Let(
                "source".to_owned(),
                ResolvedExpr::String("var hidden = 7; function eval() {}".to_owned()),
            ),
            ResolvedStmt::Expr(ResolvedExpr::Eval {
                plan: EvalFragmentPlan::new(
                    EvalKind::Direct,
                    EvalSource::Runtime(Box::new(ResolvedExpr::Ident("source".to_owned()))),
                    true,
                    Span::generated("eval"),
                ),
            }),
        ];

        let binding_names = collect_dynamic_direct_eval_created_binding_names(&body);
        assert!(!binding_names.contains("hidden"));
        assert!(!binding_names.contains("eval"));

        let function_names = collect_dynamic_direct_eval_created_function_names(&body);
        assert!(!function_names.contains("eval"));
    }
}
pub(crate) fn collect_direct_eval_block_function_env_from_stmts(
    stmts: &[ResolvedStmt],
    env: &mut DirectEvalBlockFunctionEnv,
) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Expr(ResolvedExpr::Call { callee, args, .. }) => {
                if let ResolvedExpr::FunctionExpr { params, body, .. } = callee.as_ref()
                    && params.is_empty()
                    && args.is_empty()
                    && direct_iife_body_has_static_eval_block_function_binding(body)
                {
                    collect_direct_eval_block_function_iife_env(body, env);
                }
            }
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_direct_eval_block_function_env_from_stmts(then_body, env);
                collect_direct_eval_block_function_env_from_stmts(else_body, env);
            }
            ResolvedStmt::While { body, .. }
            | ResolvedStmt::DoWhile { body, .. }
            | ResolvedStmt::ForIn { body, .. }
            | ResolvedStmt::ForOf { body, .. }
            | ResolvedStmt::ForAwaitOf { body, .. } => {
                collect_direct_eval_block_function_env_from_stmts(body, env);
            }
            ResolvedStmt::For { init, body, .. } => {
                if let Some(init) = init {
                    collect_direct_eval_block_function_env_from_stmts(
                        std::slice::from_ref(init.as_ref()),
                        env,
                    );
                }
                collect_direct_eval_block_function_env_from_stmts(body, env);
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_direct_eval_block_function_env_from_stmts(try_block, env);
                if let Some(block) = catch_block {
                    collect_direct_eval_block_function_env_from_stmts(block, env);
                }
                if let Some(block) = finally_block {
                    collect_direct_eval_block_function_env_from_stmts(block, env);
                }
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_direct_eval_block_function_env_from_stmts(body, env);
                }
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_direct_eval_block_function_env_from_stmts(
                    std::slice::from_ref(body.as_ref()),
                    env,
                );
            }
            ResolvedStmt::Block { statements, .. } => {
                collect_direct_eval_block_function_env_from_stmts(statements, env);
            }
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Function { .. }
            | ResolvedStmt::ClassDecl { .. }
            | ResolvedStmt::Let(_, _)
            | ResolvedStmt::DestructureLet { .. }
            | ResolvedStmt::DestructureAssign { .. }
            | ResolvedStmt::Assign(_, _)
            | ResolvedStmt::Expr(_)
            | ResolvedStmt::Return(_)
            | ResolvedStmt::Throw(_)
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. }
            | ResolvedStmt::Export { .. }
            | ResolvedStmt::ModuleExportsAssign { .. } => {}
        }
    }
}

pub(crate) fn collect_direct_eval_block_function_iife_env(
    body: &[ResolvedStmt],
    env: &mut DirectEvalBlockFunctionEnv,
) {
    for stmt in body {
        let ResolvedStmt::Function {
            name,
            params,
            body: function_body,
            ..
        } = stmt
        else {
            continue;
        };
        if !params.is_empty()
            || block_contains_this(function_body)
            || block_contains_arguments(function_body)
        {
            continue;
        }
        env.env_cell_names.insert(name.clone());
        env.heap_closure_names.insert(name.clone());
        collect_direct_eval_function_assignment_env(name, function_body, env);
    }
}

pub(crate) fn collect_direct_eval_function_assignment_env(
    function_name: &str,
    body: &[ResolvedStmt],
    env: &mut DirectEvalBlockFunctionEnv,
) {
    for stmt in body {
        match stmt {
            ResolvedStmt::Assign(name, expr) => {
                env.env_cell_names.insert(name.clone());
                if matches!(expr, ResolvedExpr::Ident(value) if value == function_name) {
                    env.heap_closure_names.insert(name.clone());
                }
                collect_direct_eval_function_assignment_expr(function_name, expr, env);
            }
            ResolvedStmt::Expr(expr) | ResolvedStmt::Return(expr) | ResolvedStmt::Throw(expr) => {
                collect_direct_eval_function_assignment_expr(function_name, expr, env);
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_direct_eval_function_assignment_expr(function_name, condition, env);
                collect_direct_eval_function_assignment_env(function_name, then_body, env);
                collect_direct_eval_function_assignment_env(function_name, else_body, env);
            }
            ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
                collect_direct_eval_function_assignment_expr(function_name, condition, env);
                collect_direct_eval_function_assignment_env(function_name, body, env);
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_direct_eval_function_assignment_env(function_name, try_block, env);
                if let Some(block) = catch_block {
                    collect_direct_eval_function_assignment_env(function_name, block, env);
                }
                if let Some(block) = finally_block {
                    collect_direct_eval_function_assignment_env(function_name, block, env);
                }
            }
            ResolvedStmt::Switch { expr, cases } => {
                collect_direct_eval_function_assignment_expr(function_name, expr, env);
                for (case_expr, body) in cases {
                    if let Some(case_expr) = case_expr {
                        collect_direct_eval_function_assignment_expr(function_name, case_expr, env);
                    }
                    collect_direct_eval_function_assignment_env(function_name, body, env);
                }
            }
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init) = init {
                    collect_direct_eval_function_assignment_env(
                        function_name,
                        std::slice::from_ref(init.as_ref()),
                        env,
                    );
                }
                if let Some(condition) = condition {
                    collect_direct_eval_function_assignment_expr(function_name, condition, env);
                }
                if let Some(update) = update {
                    collect_direct_eval_function_assignment_expr(function_name, update, env);
                }
                collect_direct_eval_function_assignment_env(function_name, body, env);
            }
            ResolvedStmt::ForIn { iter, body, .. }
            | ResolvedStmt::ForOf { iter, body, .. }
            | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
                collect_direct_eval_function_assignment_expr(function_name, iter, env);
                collect_direct_eval_function_assignment_env(function_name, body, env);
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_direct_eval_function_assignment_env(
                    function_name,
                    std::slice::from_ref(body.as_ref()),
                    env,
                );
            }
            ResolvedStmt::Let(_, expr)
            | ResolvedStmt::DestructureLet { expr, .. }
            | ResolvedStmt::DestructureAssign { expr, .. } => {
                collect_direct_eval_function_assignment_expr(function_name, expr, env);
            }
            ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
                collect_direct_eval_function_assignment_expr(function_name, expr, env);
            }
            ResolvedStmt::Block { statements, .. } => {
                collect_direct_eval_function_assignment_env(function_name, statements, env);
            }
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Function { .. }
            | ResolvedStmt::ClassDecl { .. }
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. } => {}
        }
    }
}

pub(crate) fn collect_direct_eval_function_assignment_expr(
    function_name: &str,
    expr: &ResolvedExpr,
    env: &mut DirectEvalBlockFunctionEnv,
) {
    match expr {
        ResolvedExpr::Assign { name, expr } | ResolvedExpr::LogicalAssign { name, expr, .. } => {
            env.env_cell_names.insert(name.clone());
            if matches!(expr.as_ref(), ResolvedExpr::Ident(value) if value == function_name) {
                env.heap_closure_names.insert(name.clone());
            }
            collect_direct_eval_function_assignment_expr(function_name, expr, env);
        }
        ResolvedExpr::Await { expr } => {
            collect_direct_eval_function_assignment_expr(function_name, expr, env);
        }
        ResolvedExpr::Yield { expr, .. } => {
            if let Some(expr) = expr {
                collect_direct_eval_function_assignment_expr(function_name, expr, env);
            }
        }
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => {
            collect_direct_eval_function_assignment_expr(function_name, expr, env);
        }
        ResolvedExpr::Binary { left, right, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, left, env);
            collect_direct_eval_function_assignment_expr(function_name, right, env);
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_direct_eval_function_assignment_expr(function_name, condition, env);
            collect_direct_eval_function_assignment_expr(function_name, then_expr, env);
            collect_direct_eval_function_assignment_expr(function_name, else_expr, env);
        }
        ResolvedExpr::Call { callee, args, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, callee, env);
            for arg in args {
                collect_direct_eval_function_assignment_expr(function_name, arg, env);
            }
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                if let ResolvedArrayElement::Present(expr) = element {
                    collect_direct_eval_function_assignment_expr(function_name, expr, env);
                }
            }
        }
        ResolvedExpr::Object(props) => {
            for prop in props {
                if let Some(key) = prop.computed_key() {
                    collect_direct_eval_function_assignment_expr(function_name, key, env);
                }
                collect_direct_eval_function_assignment_expr(function_name, prop.value(), env);
            }
        }
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, object, env);
            collect_direct_eval_function_assignment_expr(function_name, index, env);
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            for arg in args {
                collect_direct_eval_function_assignment_expr(function_name, arg, env);
            }
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            for arg in &plan.args {
                collect_direct_eval_function_assignment_expr(function_name, arg, env);
            }
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, object, env);
        }
        ResolvedExpr::OptionalCall { callee, args, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, callee, env);
            for arg in args {
                collect_direct_eval_function_assignment_expr(function_name, arg, env);
            }
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, object, env);
            for arg in args {
                collect_direct_eval_function_assignment_expr(function_name, arg, env);
            }
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, object, env);
            collect_direct_eval_function_assignment_expr(function_name, value, env);
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            collect_direct_eval_function_assignment_expr(function_name, object, env);
            collect_direct_eval_function_assignment_expr(function_name, key, env);
            collect_direct_eval_function_assignment_expr(function_name, value, env);
        }
        ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, expr, env);
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, object, env);
            collect_direct_eval_function_assignment_expr(function_name, expr, env);
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            collect_direct_eval_function_assignment_expr(function_name, key, env);
            collect_direct_eval_function_assignment_expr(function_name, expr, env);
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            collect_direct_eval_function_assignment_expr(function_name, object, env);
            collect_direct_eval_function_assignment_expr(function_name, key, env);
            collect_direct_eval_function_assignment_expr(function_name, expr, env);
        }
        ResolvedExpr::Sequence(exprs) => {
            for e in exprs {
                collect_direct_eval_function_assignment_expr(function_name, e, env);
            }
        }
        ResolvedExpr::EvalCompletion(steps) => {
            for step in steps {
                if let Some(expr) = step.expr() {
                    collect_direct_eval_function_assignment_expr(function_name, expr, env);
                }
            }
        }
        ResolvedExpr::ArrowFn { .. }
        | ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Eval { .. } => {}
        ResolvedExpr::Undefined => {}
    }
}
