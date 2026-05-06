use crate::builtin_resolved::ResolvedArrayElement;
use super::*;

pub(super) fn collect_direct_eval_block_function_env(program: &[ResolvedStmt]) -> DirectEvalBlockFunctionEnv {
    let mut env = DirectEvalBlockFunctionEnv::default();
    collect_direct_eval_block_function_env_from_stmts(program, &mut env);
    env
}
pub(super) fn collect_direct_eval_block_function_env_from_stmts(
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
            | ResolvedStmt::ForOf { body, .. } => {
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

pub(super) fn collect_direct_eval_block_function_iife_env(
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
        if !params.is_empty() || block_contains_this(function_body) || block_contains_arguments(function_body) {
            continue;
        }
        env.env_cell_names.insert(name.clone());
        env.heap_closure_names.insert(name.clone());
        collect_direct_eval_function_assignment_env(name, function_body, env);
    }
}

pub(super) fn collect_direct_eval_function_assignment_env(
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
            ResolvedStmt::ForIn { iter, body, .. } | ResolvedStmt::ForOf { iter, body, .. } => {
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
            ResolvedStmt::Let(_, expr) | ResolvedStmt::DestructureLet { expr, .. } => {
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

pub(super) fn collect_direct_eval_function_assignment_expr(
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
            for (_, value) in props {
                collect_direct_eval_function_assignment_expr(function_name, value, env);
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
        ResolvedExpr::ArrowFn { .. }
        | ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined => {}
    }
}
