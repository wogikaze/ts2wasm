use std::collections::HashSet;

use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_ir::binding_pattern::{BindingPattern, parse_binding_pattern};
use ts2wasm_ir::builtin_resolved::{
    ClassMethod, EvalCompletionPlan, EvalCompletionStep, EvalDeclarationPlan,
    EvalForHeadVarLanding, EvalFragmentPlan, EvalFunctionHoist, EvalKind, EvalSource,
    FunctionConstructorHostPolicy, FunctionConstructorKind, FunctionConstructorParseGoal,
    FunctionConstructorParseGoals, FunctionConstructorPlan, ResolvedArrayElement, ResolvedExpr,
    ResolvedObjectProp, ResolvedParam, ResolvedStmt,
};
use ts2wasm_ir::builtin_resolver::resolve_builtins;
use ts2wasm_ir::name_resolver::resolve_names;
use ts2wasm_ir::name_resolver::resolve_names_with_outer_bindings;
use ts2wasm_source::Span;
use ts2wasm_syntax::{Expr, FunctionExprOrigin, Stmt};
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
/// constructor calls are left for the host.function.* lane.
///
/// Runtime-source eval is left as-is for the host lane.
pub fn expand_static_eval_fragments(
    resolved: Vec<ResolvedStmt>,
) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    let mut ctx = EvalExpansionContext::new();
    ctx.set_strict_context(resolved_block_has_use_strict_directive(&resolved));
    expand_stmts(resolved, &mut ctx)
}

#[derive(Debug, Default)]
struct EvalExpansionContext {
    scopes: Vec<HashSet<String>>,
    strict_contexts: Vec<bool>,
}

impl EvalExpansionContext {
    fn new() -> Self {
        Self {
            scopes: vec![HashSet::new()],
            strict_contexts: vec![false],
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashSet::new());
        let strict = self.is_strict_context();
        self.strict_contexts.push(strict);
    }

    fn enter_strict_scope(&mut self, strict_context: bool) {
        self.scopes.push(HashSet::new());
        self.strict_contexts
            .push(self.is_strict_context() || strict_context);
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
        self.strict_contexts.pop();
    }

    fn set_strict_context(&mut self, strict_context: bool) {
        if let Some(current) = self.strict_contexts.last_mut() {
            *current |= strict_context;
        }
    }

    fn is_strict_context(&self) -> bool {
        self.strict_contexts.last().copied().unwrap_or(false)
    }

    fn declare(&mut self, name: impl Into<String>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.into());
        }
    }

    fn declare_params(&mut self, params: &[ResolvedParam]) {
        for param in params {
            self.declare(param.name.clone());
        }
    }

    fn visible_bindings(&self) -> Vec<String> {
        let mut bindings = Vec::new();
        for scope in &self.scopes {
            for name in scope {
                bindings.push(name.clone());
            }
        }
        bindings
    }
}

fn expand_stmt(
    stmt: ResolvedStmt,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedStmt, Diagnostic> {
    match stmt {
        ResolvedStmt::Expr(expr) => Ok(ResolvedStmt::Expr(expand_expr(expr, ctx)?)),
        ResolvedStmt::Let(name, expr) => {
            let expr = expand_expr(expr, ctx)?;
            ctx.declare(name.clone());
            Ok(ResolvedStmt::Let(name, expr))
        }
        ResolvedStmt::Assign(name, expr) => Ok(ResolvedStmt::Assign(name, expand_expr(expr, ctx)?)),
        ResolvedStmt::Return(expr) => Ok(ResolvedStmt::Return(expand_expr(expr, ctx)?)),
        ResolvedStmt::Throw(expr) => Ok(ResolvedStmt::Throw(expand_expr(expr, ctx)?)),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let condition = expand_expr(condition, ctx)?;
            ctx.enter_scope();
            let then_body = expand_stmts(then_body, ctx)?;
            ctx.exit_scope();
            ctx.enter_scope();
            let else_body = expand_stmts(else_body, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            })
        }
        ResolvedStmt::While { condition, body } => Ok(ResolvedStmt::While {
            condition: expand_expr(condition, ctx)?,
            body: {
                ctx.enter_scope();
                let body = expand_stmts(body, ctx)?;
                ctx.exit_scope();
                body
            },
        }),
        ResolvedStmt::DoWhile { body, condition } => {
            ctx.enter_scope();
            let body = expand_stmts(body, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedStmt::DoWhile {
                body,
                condition: expand_expr(condition, ctx)?,
            })
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            ctx.enter_scope();
            let init = match init {
                Some(boxed) => Some(Box::new(expand_stmt(*boxed, ctx)?)),
                None => None,
            };
            let condition = condition.map(|expr| expand_expr(expr, ctx)).transpose()?;
            let update = update.map(|expr| expand_expr(expr, ctx)).transpose()?;
            let body = expand_stmts(body, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            })
        }
        ResolvedStmt::ForIn { var, iter, body } => {
            let iter = expand_expr(iter, ctx)?;
            ctx.enter_scope();
            ctx.declare(var.clone());
            let body = expand_stmts(body, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedStmt::ForIn { var, iter, body })
        }
        ResolvedStmt::ForOf { var, iter, body } => {
            let iter = expand_expr(iter, ctx)?;
            ctx.enter_scope();
            ctx.declare(var.clone());
            let body = expand_stmts(body, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedStmt::ForOf { var, iter, body })
        }
        ResolvedStmt::ForAwaitOf { var, iter, body } => {
            let iter = expand_expr(iter, ctx)?;
            ctx.enter_scope();
            ctx.declare(var.clone());
            let body = expand_stmts(body, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedStmt::ForAwaitOf { var, iter, body })
        }
        ResolvedStmt::Switch { expr, cases } => {
            let mut expanded_cases = Vec::new();
            let expr = expand_expr(expr, ctx)?;
            for (cond, body) in cases {
                let expanded_cond = cond.map(|expr| expand_expr(expr, ctx)).transpose()?;
                ctx.enter_scope();
                let expanded_body = expand_stmts(body, ctx)?;
                ctx.exit_scope();
                expanded_cases.push((expanded_cond, expanded_body));
            }
            Ok(ResolvedStmt::Switch {
                expr,
                cases: expanded_cases,
            })
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
        } => {
            ctx.enter_scope();
            let try_block = expand_stmts(try_block, ctx)?;
            ctx.exit_scope();
            let catch_block = catch_block
                .map(|b| {
                    ctx.enter_scope();
                    if let Some(catch_param) = &catch_param {
                        ctx.declare(catch_param.clone());
                    }
                    let result = expand_stmts(b, ctx);
                    ctx.exit_scope();
                    result
                })
                .transpose()?;
            let finally_block = finally_block
                .map(|b| {
                    ctx.enter_scope();
                    let result = expand_stmts(b, ctx);
                    ctx.exit_scope();
                    result
                })
                .transpose()?;
            Ok(ResolvedStmt::TryCatch {
                try_block,
                catch_param,
                catch_block,
                finally_block,
            })
        }
        ResolvedStmt::Block { statements } => {
            ctx.enter_scope();
            let statements = expand_stmts(statements, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedStmt::Block { statements })
        }
        ResolvedStmt::Labeled { label, body } => Ok(ResolvedStmt::Labeled {
            label,
            body: Box::new(expand_stmt(*body, ctx)?),
        }),
        ResolvedStmt::Break { label } => Ok(ResolvedStmt::Break { label }),
        ResolvedStmt::Continue { label } => Ok(ResolvedStmt::Continue { label }),
        ResolvedStmt::DestructureLet { pattern, expr } => {
            let expr = expand_expr(expr, ctx)?;
            for name in pattern.names() {
                ctx.declare(name.to_owned());
            }
            Ok(ResolvedStmt::DestructureLet { pattern, expr })
        }
        ResolvedStmt::Export { name, expr } => Ok(ResolvedStmt::Export {
            name,
            expr: Box::new(expand_expr(*expr, ctx)?),
        }),
        ResolvedStmt::ModuleExportsAssign { expr } => Ok(ResolvedStmt::ModuleExportsAssign {
            expr: Box::new(expand_expr(*expr, ctx)?),
        }),
        ResolvedStmt::Function {
            name,
            params,
            body,
            is_generator,
            is_async,
            is_ambient,
            source_text,
        } => {
            ctx.declare(name.clone());
            let function_is_strict = resolved_block_has_use_strict_directive(&body);
            ctx.enter_strict_scope(function_is_strict);
            ctx.declare("arguments");
            let params = expand_params(params, ctx)?;
            ctx.declare_params(&params);
            let body = expand_stmts(body, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedStmt::Function {
                name,
                params,
                body,
                is_generator,
                is_async,
                is_ambient,
                source_text,
            })
        }
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
            ctx.declare(name.clone());
            let constructor = constructor
                .map(|ctor| expand_constructor(ctor, ctx))
                .transpose()?;
            let methods = methods
                .into_iter()
                .map(|method| expand_class_method(method, ctx))
                .collect::<Result<Vec<_>, _>>()?;
            let statics = statics
                .into_iter()
                .map(|(name, expr)| Ok((name, expand_expr(expr, ctx)?)))
                .collect::<Result<Vec<_>, Diagnostic>>()?;
            let static_blocks = static_blocks
                .into_iter()
                .map(|(span, body)| {
                    ctx.enter_scope();
                    let result = expand_stmts(body, ctx);
                    ctx.exit_scope();
                    Ok((span, result?))
                })
                .collect::<Result<Vec<_>, Diagnostic>>()?;
            let static_private_fields = static_private_fields
                .into_iter()
                .map(|(name, expr, span)| Ok((name, expand_expr(expr, ctx)?, span)))
                .collect::<Result<Vec<_>, Diagnostic>>()?;
            Ok(ResolvedStmt::ClassDecl {
                name,
                extends,
                constructor,
                methods,
                statics,
                static_blocks,
                private_fields,
                static_private_fields,
            })
        }
        ResolvedStmt::AmbientValue(_) => Ok(stmt),
    }
}

fn expand_stmts(
    stmts: Vec<ResolvedStmt>,
    ctx: &mut EvalExpansionContext,
) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    for stmt in &stmts {
        match stmt {
            ResolvedStmt::Function { name, .. } | ResolvedStmt::ClassDecl { name, .. } => {
                ctx.declare(name.clone());
            }
            _ => {}
        }
    }
    stmts
        .into_iter()
        .map(|stmt| expand_stmt(stmt, ctx))
        .collect()
}

fn expand_params(
    params: Vec<ts2wasm_ir::builtin_resolved::ResolvedParam>,
    ctx: &mut EvalExpansionContext,
) -> Result<Vec<ts2wasm_ir::builtin_resolved::ResolvedParam>, Diagnostic> {
    params
        .into_iter()
        .map(|mut param| {
            param.default = param
                .default
                .map(|expr| expand_expr(expr, ctx))
                .transpose()?;
            Ok(param)
        })
        .collect()
}

fn expand_constructor(
    (params, body): ts2wasm_ir::builtin_resolved::ResolvedConstructor,
    ctx: &mut EvalExpansionContext,
) -> Result<ts2wasm_ir::builtin_resolved::ResolvedConstructor, Diagnostic> {
    ctx.enter_strict_scope(true);
    ctx.declare("arguments");
    let params = expand_params(params, ctx)?;
    ctx.declare_params(&params);
    let body = expand_stmts(body, ctx)?;
    ctx.exit_scope();
    Ok((params, body))
}

fn expand_class_method(
    method: ts2wasm_ir::builtin_resolved::ClassMethod,
    ctx: &mut EvalExpansionContext,
) -> Result<ts2wasm_ir::builtin_resolved::ClassMethod, Diagnostic> {
    ctx.enter_strict_scope(true);
    ctx.declare("arguments");
    let params = expand_params(method.params, ctx)?;
    ctx.declare_params(&params);
    let body = expand_stmts(method.body, ctx)?;
    ctx.exit_scope();
    Ok(ts2wasm_ir::builtin_resolved::ClassMethod {
        name: method.name,
        kind: method.kind,
        params,
        body,
        captures: method.captures,
    })
}

fn expand_expr(
    expr: ResolvedExpr,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
        ResolvedExpr::Eval { plan }
            if plan.kind == EvalKind::Direct
                && matches!(plan.source, EvalSource::StaticLiteral(_)) =>
        {
            let EvalSource::StaticLiteral(src) = &plan.source else {
                unreachable!();
            };
            let caller_is_strict = plan.caller_is_strict || ctx.is_strict_context();
            let expanded =
                expand_static_eval_source(src, &ctx.visible_bindings(), &plan, caller_is_strict)?;
            for name in &expanded.caller_var_declarations {
                ctx.declare(name.clone());
            }
            Ok(expanded.expr)
        }
        ResolvedExpr::Eval { plan }
            if plan.kind == EvalKind::Indirect
                && matches!(plan.source, EvalSource::StaticLiteral(_)) =>
        {
            let EvalSource::StaticLiteral(src) = &plan.source else {
                unreachable!();
            };
            let caller_is_strict = plan.caller_is_strict || ctx.is_strict_context();
            let caller_bindings = ctx.visible_bindings();
            let expanded =
                expand_static_eval_source(src, &caller_bindings, &plan, caller_is_strict)?;
            let mut global_bindings = caller_bindings;
            for name in &expanded.global_declaration_names {
                if !global_bindings.contains(name) {
                    global_bindings.push(name.clone());
                }
            }
            Ok(rewrite_indirect_eval_caller_binding_collisions(
                expanded.expr,
                &global_bindings,
            ))
        }
        ResolvedExpr::Eval {
            plan:
                EvalFragmentPlan {
                    source: EvalSource::NonStringStatic(value),
                    ..
                },
        } => expand_expr(*value, ctx),
        ResolvedExpr::Eval { .. } => {
            // Non-expandable eval (indirect or runtime source) — keep as-is.
            Ok(expr)
        }
        ResolvedExpr::FunctionConstructor { plan } => expand_function_constructor(plan),
        // Recursively expand eval in sub-expressions.
        ResolvedExpr::Unary { op, expr: inner } => Ok(ResolvedExpr::Unary {
            op,
            expr: Box::new(expand_expr(*inner, ctx)?),
        }),
        ResolvedExpr::Binary { left, op, right } => Ok(ResolvedExpr::Binary {
            left: Box::new(expand_expr(*left, ctx)?),
            op,
            right: Box::new(expand_expr(*right, ctx)?),
        }),
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            span,
        } => Ok(ResolvedExpr::Ternary {
            condition: Box::new(expand_expr(*condition, ctx)?),
            then_expr: Box::new(expand_expr(*then_expr, ctx)?),
            else_expr: Box::new(expand_expr(*else_expr, ctx)?),
            span,
        }),
        ResolvedExpr::Call { callee, args, span } => Ok(ResolvedExpr::Call {
            callee: Box::new(expand_expr(*callee, ctx)?),
            args: args
                .into_iter()
                .map(|expr| expand_expr(expr, ctx))
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
                .map(|expr| expand_expr(expr, ctx))
                .collect::<Result<Vec<_>, _>>()?,
            span,
        }),
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            span,
        } => Ok(ResolvedExpr::MethodCall {
            object: Box::new(expand_expr(*object, ctx)?),
            method,
            args: args
                .into_iter()
                .map(|expr| expand_expr(expr, ctx))
                .collect::<Result<Vec<_>, _>>()?,
            span,
        }),
        ResolvedExpr::PropertyAccess { object, key, span } => Ok(ResolvedExpr::PropertyAccess {
            object: Box::new(expand_expr(*object, ctx)?),
            key,
            span,
        }),
        ResolvedExpr::OptionalPropertyAccess { object, key, span } => {
            Ok(ResolvedExpr::OptionalPropertyAccess {
                object: Box::new(expand_expr(*object, ctx)?),
                key,
                span,
            })
        }
        ResolvedExpr::ComputedIndex { object, index } => Ok(ResolvedExpr::ComputedIndex {
            object: Box::new(expand_expr(*object, ctx)?),
            index: Box::new(expand_expr(*index, ctx)?),
        }),
        ResolvedExpr::OptionalComputedIndex {
            object,
            index,
            span,
        } => Ok(ResolvedExpr::OptionalComputedIndex {
            object: Box::new(expand_expr(*object, ctx)?),
            index: Box::new(expand_expr(*index, ctx)?),
            span,
        }),
        ResolvedExpr::Assign { name, expr: inner } => Ok(ResolvedExpr::Assign {
            name,
            expr: Box::new(expand_expr(*inner, ctx)?),
        }),
        ResolvedExpr::LogicalAssign {
            name,
            op,
            expr: inner,
        } => Ok(ResolvedExpr::LogicalAssign {
            name,
            op,
            expr: Box::new(expand_expr(*inner, ctx)?),
        }),
        ResolvedExpr::LogicalPropertyAssign {
            object,
            key,
            op,
            expr: inner,
        } => Ok(ResolvedExpr::LogicalPropertyAssign {
            object,
            key,
            op,
            expr: Box::new(expand_expr(*inner, ctx)?),
        }),
        ResolvedExpr::LogicalComputedPropertyAssign {
            object,
            key,
            op,
            expr: inner,
        } => Ok(ResolvedExpr::LogicalComputedPropertyAssign {
            object,
            key: Box::new(expand_expr(*key, ctx)?),
            op,
            expr: Box::new(expand_expr(*inner, ctx)?),
        }),
        ResolvedExpr::LogicalComputedMemberAssign {
            object,
            key,
            op,
            expr: inner,
        } => Ok(ResolvedExpr::LogicalComputedMemberAssign {
            object: Box::new(expand_expr(*object, ctx)?),
            key: Box::new(expand_expr(*key, ctx)?),
            op,
            expr: Box::new(expand_expr(*inner, ctx)?),
        }),
        ResolvedExpr::LogicalMemberAssign {
            object,
            key,
            op,
            expr: inner,
        } => Ok(ResolvedExpr::LogicalMemberAssign {
            object: Box::new(expand_expr(*object, ctx)?),
            key,
            op,
            expr: Box::new(expand_expr(*inner, ctx)?),
        }),
        ResolvedExpr::Array(elements) => {
            let expanded = elements
                .into_iter()
                .map(|el| match el {
                    ts2wasm_ir::builtin_resolved::ResolvedArrayElement::Present(e) => {
                        Ok(ts2wasm_ir::builtin_resolved::ResolvedArrayElement::Present(
                            expand_expr(e, ctx)?,
                        ))
                    }
                    hole @ ts2wasm_ir::builtin_resolved::ResolvedArrayElement::Hole => Ok(hole),
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ResolvedExpr::Array(expanded))
        }
        ResolvedExpr::Object(props) => Ok(ResolvedExpr::Object(
            props
                .into_iter()
                .map(|prop| expand_object_prop(prop, ctx))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        ResolvedExpr::BuiltinCall { builtin, args } => Ok(ResolvedExpr::BuiltinCall {
            builtin,
            args: args
                .into_iter()
                .map(|expr| expand_expr(expr, ctx))
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ResolvedExpr::BuiltinProperty {
            builtin,
            object,
            span,
        } => Ok(ResolvedExpr::BuiltinProperty {
            builtin,
            object: Box::new(expand_expr(*object, ctx)?),
            span,
        }),
        ResolvedExpr::OptionalCall { callee, args, span } => Ok(ResolvedExpr::OptionalCall {
            callee: Box::new(expand_expr(*callee, ctx)?),
            args: args
                .into_iter()
                .map(|expr| expand_expr(expr, ctx))
                .collect::<Result<Vec<_>, _>>()?,
            span,
        }),
        ResolvedExpr::PropertyAssign {
            object,
            key,
            value,
            span,
        } => Ok(ResolvedExpr::PropertyAssign {
            object: Box::new(expand_expr(*object, ctx)?),
            key,
            value: Box::new(expand_expr(*value, ctx)?),
            span,
        }),
        ResolvedExpr::Spread(inner) => {
            Ok(ResolvedExpr::Spread(Box::new(expand_expr(*inner, ctx)?)))
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            Ok(ResolvedExpr::PropertyAssignDynamic {
                object: Box::new(expand_expr(*object, ctx)?),
                key: Box::new(expand_expr(*key, ctx)?),
                value: Box::new(expand_expr(*value, ctx)?),
            })
        }
        ResolvedExpr::Await { expr: inner } => Ok(ResolvedExpr::Await {
            expr: Box::new(expand_expr(*inner, ctx)?),
        }),
        ResolvedExpr::Yield {
            expr: Some(inner),
            delegate,
        } => Ok(ResolvedExpr::Yield {
            expr: Some(Box::new(expand_expr(*inner, ctx)?)),
            delegate,
        }),
        ResolvedExpr::ArrowFn {
            params,
            body,
            body_stmts,
            source_text,
        } => {
            let function_is_strict = resolved_block_has_use_strict_directive(&body_stmts);
            ctx.enter_strict_scope(function_is_strict);
            for param in &params {
                ctx.declare(param.clone());
            }
            let body = Box::new(expand_expr(*body, ctx)?);
            let body_stmts = expand_stmts(body_stmts, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedExpr::ArrowFn {
                params,
                body,
                body_stmts,
                source_text,
            })
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
            let function_is_strict = resolved_block_has_use_strict_directive(&body);
            ctx.enter_strict_scope(function_is_strict);
            if !name.is_empty() {
                ctx.declare(name.clone());
            }
            ctx.declare("arguments");
            let params = expand_params(params, ctx)?;
            ctx.declare_params(&params);
            let body = expand_stmts(body, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedExpr::FunctionExpr {
                name,
                params,
                body,
                is_generator,
                origin,
                constructor_metadata,
                source_text,
            })
        }
        ResolvedExpr::ClassExpr { name, body } => {
            ctx.enter_strict_scope(true);
            if !name.is_empty() {
                ctx.declare(name.clone());
            }
            let body = expand_stmts(body, ctx)?;
            ctx.exit_scope();
            Ok(ResolvedExpr::ClassExpr { name, body })
        }
        ResolvedExpr::Sequence(exprs) => Ok(ResolvedExpr::Sequence(
            exprs
                .into_iter()
                .map(|expr| expand_expr(expr, ctx))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        // Leaf expressions and other types — no recursive expansion needed.
        other => Ok(other),
    }
}

fn expand_object_prop(
    prop: ts2wasm_ir::builtin_resolved::ResolvedObjectProp,
    ctx: &mut EvalExpansionContext,
) -> Result<ts2wasm_ir::builtin_resolved::ResolvedObjectProp, Diagnostic> {
    use ts2wasm_ir::builtin_resolved::ResolvedObjectProp;

    match prop {
        ResolvedObjectProp::KeyValue { key, value } => Ok(ResolvedObjectProp::KeyValue {
            key,
            value: expand_expr(value, ctx)?,
        }),
        ResolvedObjectProp::Shorthand { key, value } => Ok(ResolvedObjectProp::Shorthand {
            key,
            value: expand_expr(value, ctx)?,
        }),
        ResolvedObjectProp::ComputedKey { key, value } => Ok(ResolvedObjectProp::ComputedKey {
            key: Box::new(expand_expr(*key, ctx)?),
            value: expand_expr(value, ctx)?,
        }),
        ResolvedObjectProp::MethodShorthand { key, value } => {
            Ok(ResolvedObjectProp::MethodShorthand {
                key,
                value: expand_expr(value, ctx)?,
            })
        }
    }
}

struct StaticEvalExpansion {
    expr: ResolvedExpr,
    caller_var_declarations: Vec<String>,
    global_declaration_names: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EvalVarLanding {
    Caller,
    Global,
    Lexical,
}

fn expand_static_eval_source(
    src: &str,
    outer_bindings: &[String],
    fragment_plan: &EvalFragmentPlan,
    caller_is_strict: bool,
) -> Result<StaticEvalExpansion, Diagnostic> {
    let tokens = if caller_is_strict {
        ts2wasm_frontend::Lexer::new_with_strict_mode(src, true)
    } else {
        ts2wasm_frontend::Lexer::new(src)
    }
    .tokenize()
    .map_err(|e| Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("eval source lex error: {e}"),
        span: None,
        phase: None,
    })?;
    let program = if caller_is_strict {
        ts2wasm_frontend::Parser::new_with_strict_mode(tokens, true, src)
    } else {
        ts2wasm_frontend::Parser::new(tokens, src)
    }
    .parse_program()
    .map_err(|e| Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("eval source parse error: {e}"),
        span: None,
        phase: None,
    })?;
    validate_static_eval_source(&program)?;

    let eval_is_strict = caller_is_strict || block_has_use_strict_directive(&program);
    let leak_var_declarations = !eval_is_strict;
    let mut caller_landing_declarations = Vec::new();
    let mut global_var_hoists = Vec::new();
    let mut global_declaration_names = Vec::new();
    if leak_var_declarations {
        collect_eval_var_declaration_names(src, &program, &mut caller_landing_declarations);
        collect_eval_var_let_declaration_names(src, &program, &mut global_var_hoists);
        collect_eval_var_declaration_names(src, &program, &mut global_declaration_names);
    }
    let direct_caller_scope = fragment_plan.kind == EvalKind::Direct;
    let var_landing = match (direct_caller_scope, leak_var_declarations) {
        (true, true) => EvalVarLanding::Caller,
        (false, true) => EvalVarLanding::Global,
        _ => EvalVarLanding::Lexical,
    };
    let caller_var_declarations = if var_landing == EvalVarLanding::Caller {
        caller_landing_declarations.clone()
    } else {
        Vec::new()
    };
    if var_landing != EvalVarLanding::Global {
        global_var_hoists.clear();
        global_declaration_names.clear();
    }

    let mut effective_outer_bindings = outer_bindings.to_vec();
    for name in caller_var_declarations
        .iter()
        .chain(global_declaration_names.iter())
    {
        if !effective_outer_bindings.contains(name) {
            effective_outer_bindings.push(name.clone());
        }
    }

    let name_resolved = if effective_outer_bindings.is_empty() {
        resolve_names(&program)?
    } else {
        resolve_names_with_outer_bindings(&program, &effective_outer_bindings)?
    };
    let builtin_resolved = resolve_builtins(&name_resolved)?;
    let mut nested_ctx = EvalExpansionContext::new();
    nested_ctx.set_strict_context(eval_is_strict);
    for binding in &effective_outer_bindings {
        nested_ctx.declare(binding.clone());
    }
    let builtin_resolved = expand_stmts(builtin_resolved, &mut nested_ctx)?;
    let mut function_hoists = Vec::new();
    if matches!(var_landing, EvalVarLanding::Caller | EvalVarLanding::Global) {
        collect_eval_function_hoists(&program, &builtin_resolved, src, &mut function_hoists);
    }
    let (caller_function_hoists, global_function_hoists) = if var_landing == EvalVarLanding::Global
    {
        (Vec::new(), function_hoists)
    } else {
        (function_hoists, Vec::new())
    };

    Ok(StaticEvalExpansion {
        expr: extract_completion_value(
            src,
            fragment_plan,
            &program,
            builtin_resolved,
            caller_is_strict,
            eval_is_strict,
            var_landing,
            &caller_var_declarations,
            caller_function_hoists,
            &global_var_hoists,
            global_function_hoists,
        )?,
        caller_var_declarations,
        global_declaration_names,
    })
}

fn resolved_block_has_use_strict_directive(stmts: &[ResolvedStmt]) -> bool {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Expr(ResolvedExpr::String(value)) if value == "use strict" => {
                return true;
            }
            ResolvedStmt::Expr(_) => continue,
            _ => return false,
        }
    }
    false
}

fn validate_static_eval_source(program: &[Stmt]) -> Result<(), Diagnostic> {
    if let Some(span) = eval_source_illegal_return_span(program) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "return statement is not valid in eval source".to_owned(),
            span: Some(span),
            phase: None,
        });
    }
    Ok(())
}

fn eval_source_illegal_return_span(stmts: &[Stmt]) -> Option<ts2wasm_source::Span> {
    for stmt in stmts {
        if let Some(span) = eval_stmt_illegal_return_span(stmt) {
            return Some(span);
        }
    }
    None
}

fn eval_stmt_illegal_return_span(stmt: &Stmt) -> Option<ts2wasm_source::Span> {
    match stmt {
        Stmt::Return { span, .. } => Some(*span),
        Stmt::Block { statements, .. } => eval_source_illegal_return_span(statements),
        Stmt::If {
            then_body,
            else_body,
            ..
        } => eval_source_illegal_return_span(then_body)
            .or_else(|| eval_source_illegal_return_span(else_body)),
        Stmt::While { body, .. }
        | Stmt::DoWhile { body, .. }
        | Stmt::ForIn { body, .. }
        | Stmt::ForOf { body, .. }
        | Stmt::ForAwaitOf { body, .. } => eval_source_illegal_return_span(body),
        Stmt::For { init, body, .. } => init
            .as_deref()
            .and_then(eval_stmt_illegal_return_span)
            .or_else(|| eval_source_illegal_return_span(body)),
        Stmt::Switch { cases, .. } => cases
            .iter()
            .find_map(|(_, body)| eval_source_illegal_return_span(body)),
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => eval_source_illegal_return_span(try_block)
            .or_else(|| {
                catch_block
                    .as_deref()
                    .and_then(eval_source_illegal_return_span)
            })
            .or_else(|| {
                finally_block
                    .as_deref()
                    .and_then(eval_source_illegal_return_span)
            }),
        Stmt::Labeled { body, .. } => eval_stmt_illegal_return_span(body),
        Stmt::ExportDecl { declaration, .. } => eval_stmt_illegal_return_span(declaration),
        Stmt::Function { .. } | Stmt::ClassDecl { .. } => None,
        _ => None,
    }
}

fn expand_function_constructor(plan: FunctionConstructorPlan) -> Result<ResolvedExpr, Diagnostic> {
    let FunctionConstructorPlan {
        kind,
        args,
        static_source,
        host_policy,
        span,
    } = plan;
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

fn validate_static_function_constructor_wrapper_shape(
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

fn validate_function_constructor_parse_goals(
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

fn function_constructor_host_lane(
    kind: FunctionConstructorKind,
    args: Vec<ResolvedExpr>,
    span: ts2wasm_source::Span,
) -> ResolvedExpr {
    ResolvedExpr::FunctionConstructor {
        plan: FunctionConstructorPlan::new(kind, args, span),
    }
}

fn function_constructor_length_metadata(params: &[ResolvedParam]) -> usize {
    params
        .iter()
        .take_while(|param| param.default.is_none() && !param.is_rest)
        .count()
}

fn validate_static_function_constructor_early_errors(
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

fn function_constructor_bound_names(param: &str) -> Vec<String> {
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

fn object_binding_name(part: &str) -> Option<String> {
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

fn array_binding_name(part: &str) -> Option<String> {
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

fn block_has_use_strict_directive(stmts: &[Stmt]) -> bool {
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

fn is_simple_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first == '$' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch == '$' || ch.is_ascii_alphanumeric())
}

fn function_constructor_syntax_error(message: &str, span: ts2wasm_source::Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("Function constructor source parse error: SyntaxError: {message}"),
        span: Some(span),
        phase: None,
    }
}

/// Extract the completion value from a resolved program body.
///
/// Produces a completion-plan expression so lower IR can evaluate all eval-code
/// side effects while preserving the last non-empty completion value exactly once.
fn extract_completion_value(
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

fn rewrite_indirect_eval_caller_binding_collisions(
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

fn rewrite_eval_expr_global_collisions(
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

fn rewrite_eval_exprs_global_collisions(
    exprs: Vec<ResolvedExpr>,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> Vec<ResolvedExpr> {
    exprs
        .into_iter()
        .map(|expr| rewrite_eval_expr_global_collisions(expr, collisions, scopes))
        .collect()
}

fn rewrite_eval_object_prop_global_collisions(
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

fn rewrite_eval_params_global_collisions(
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

fn rewrite_eval_stmts_global_collisions(
    stmts: Vec<ResolvedStmt>,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> Vec<ResolvedStmt> {
    stmts
        .into_iter()
        .map(|stmt| rewrite_eval_stmt_global_collisions(stmt, collisions, scopes))
        .collect()
}

fn rewrite_eval_stmt_global_collisions(
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

fn rewrite_eval_class_method_global_collisions(
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

fn rewrite_eval_scoped_stmts_global_collisions(
    stmts: Vec<ResolvedStmt>,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> Vec<ResolvedStmt> {
    scopes.push(HashSet::new());
    let stmts = rewrite_eval_stmts_global_collisions(stmts, collisions, scopes);
    scopes.pop();
    stmts
}

fn rewrite_eval_steps_global_collisions(
    steps: Vec<EvalCompletionStep>,
    collisions: &HashSet<String>,
    scopes: &mut Vec<HashSet<String>>,
) -> Vec<EvalCompletionStep> {
    steps
        .into_iter()
        .map(|step| rewrite_eval_step_global_collisions(step, collisions, scopes))
        .collect()
}

fn rewrite_eval_declaration_plan_global_collisions(
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

fn rewrite_eval_step_global_collisions(
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

fn eval_name_is_scoped(name: &str, scopes: &[HashSet<String>]) -> bool {
    scopes.iter().rev().any(|scope| scope.contains(name))
}

fn eval_declare_name(name: &str, scopes: &mut [HashSet<String>]) {
    if let Some(scope) = scopes.last_mut() {
        scope.insert(name.to_owned());
    }
}

fn eval_global_property(name: String) -> ResolvedExpr {
    ResolvedExpr::PropertyAccess {
        object: Box::new(ResolvedExpr::Ident("globalThis".to_owned())),
        key: name,
        span: Span::generated("static_indirect_eval_global"),
    }
}

fn eval_completion_steps(
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

fn eval_statement_completion_step(
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

fn collect_eval_var_declaration_names(source: &str, stmts: &[Stmt], out: &mut Vec<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let {
                name, is_var: true, ..
            }
            | Stmt::Function { name, .. } => {
                push_unique_eval_declaration(out, name);
            }
            Stmt::Block { statements, .. } => {
                collect_eval_var_declaration_names(source, statements, out)
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_eval_var_declaration_names(source, then_body, out);
                collect_eval_var_declaration_names(source, else_body, out);
            }
            Stmt::While { body, .. } | Stmt::DoWhile { body, .. } | Stmt::For { body, .. } => {
                collect_eval_var_declaration_names(source, body, out)
            }
            Stmt::ForIn {
                var, body, span, ..
            } => {
                collect_eval_for_head_var_declaration(source, *span, "in", var, out);
                collect_eval_var_declaration_names(source, body, out);
            }
            Stmt::ForOf {
                var, body, span, ..
            }
            | Stmt::ForAwaitOf {
                var, body, span, ..
            } => {
                collect_eval_for_head_var_declaration(source, *span, "of", var, out);
                collect_eval_var_declaration_names(source, body, out);
            }
            Stmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_eval_var_declaration_names(source, body, out);
                }
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_eval_var_declaration_names(source, try_block, out);
                if let Some(catch_block) = catch_block {
                    collect_eval_var_declaration_names(source, catch_block, out);
                }
                if let Some(finally_block) = finally_block {
                    collect_eval_var_declaration_names(source, finally_block, out);
                }
            }
            Stmt::Labeled { body, .. } => {
                collect_eval_var_declaration_names(
                    source,
                    std::slice::from_ref(body.as_ref()),
                    out,
                );
            }
            _ => {}
        }
    }
}

fn collect_eval_var_let_declaration_names(source: &str, stmts: &[Stmt], out: &mut Vec<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let {
                name, is_var: true, ..
            } => {
                push_unique_eval_declaration(out, name);
            }
            Stmt::Block { statements, .. } => {
                collect_eval_var_let_declaration_names(source, statements, out)
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_eval_var_let_declaration_names(source, then_body, out);
                collect_eval_var_let_declaration_names(source, else_body, out);
            }
            Stmt::While { body, .. } | Stmt::DoWhile { body, .. } | Stmt::For { body, .. } => {
                collect_eval_var_let_declaration_names(source, body, out)
            }
            Stmt::ForIn {
                var, body, span, ..
            } => {
                collect_eval_for_head_var_declaration(source, *span, "in", var, out);
                collect_eval_var_let_declaration_names(source, body, out);
            }
            Stmt::ForOf {
                var, body, span, ..
            }
            | Stmt::ForAwaitOf {
                var, body, span, ..
            } => {
                collect_eval_for_head_var_declaration(source, *span, "of", var, out);
                collect_eval_var_let_declaration_names(source, body, out);
            }
            Stmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_eval_var_let_declaration_names(source, body, out);
                }
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_eval_var_let_declaration_names(source, try_block, out);
                if let Some(catch_block) = catch_block {
                    collect_eval_var_let_declaration_names(source, catch_block, out);
                }
                if let Some(finally_block) = finally_block {
                    collect_eval_var_let_declaration_names(source, finally_block, out);
                }
            }
            Stmt::Labeled { body, .. } => {
                collect_eval_var_let_declaration_names(
                    source,
                    std::slice::from_ref(body.as_ref()),
                    out,
                );
            }
            _ => {}
        }
    }
}

fn collect_eval_for_head_var_declaration(
    source: &str,
    span: Span,
    separator: &str,
    fallback_var: &str,
    out: &mut Vec<String>,
) {
    let Some(binding) = eval_for_head_var_binding(source, span, separator) else {
        return;
    };
    if binding.starts_with(['{', '[']) {
        collect_binding_names_from_pattern(binding, out);
    } else if fallback_var != "_binding" {
        push_unique_eval_declaration(out, fallback_var);
    }
}

fn eval_for_head_var_landing(
    source: &str,
    span: Span,
    separator: &str,
    fallback_var: &str,
    var_landing: EvalVarLanding,
) -> EvalForHeadVarLanding {
    if !eval_for_head_uses_var(source, span, separator, fallback_var) {
        return EvalForHeadVarLanding::Local;
    }
    match var_landing {
        EvalVarLanding::Caller => EvalForHeadVarLanding::Caller,
        EvalVarLanding::Global => EvalForHeadVarLanding::Global,
        EvalVarLanding::Lexical => EvalForHeadVarLanding::Local,
    }
}

fn eval_for_head_uses_var(source: &str, span: Span, separator: &str, fallback_var: &str) -> bool {
    let Some(binding) = eval_for_head_var_binding(source, span, separator) else {
        return false;
    };
    fallback_var != "_binding" || binding.starts_with(['{', '['])
}

fn eval_for_head_var_pattern(source: &str, span: Span, separator: &str) -> Option<BindingPattern> {
    let binding = eval_for_head_var_binding(source, span, separator)?;
    parse_binding_pattern(binding, Some(span)).ok().flatten()
}

fn eval_for_head_var_binding<'a>(source: &'a str, span: Span, separator: &str) -> Option<&'a str> {
    let Some(loop_source) = source.get(span.start..) else {
        return None;
    };
    let Some(open_paren) = loop_source.find('(') else {
        return None;
    };
    let header = &loop_source[open_paren + 1..];
    let Some(separator_start) = top_level_loop_head_separator(header, separator) else {
        return None;
    };
    let binding = header[..separator_start].trim();
    let Some(binding) = binding.strip_prefix("var") else {
        return None;
    };
    if !binding
        .as_bytes()
        .first()
        .is_none_or(|byte| byte.is_ascii_whitespace() || matches!(byte, b'{' | b'['))
    {
        return None;
    }
    Some(strip_top_level_type_annotation(binding.trim()))
}

fn top_level_loop_head_separator(header: &str, separator: &str) -> Option<usize> {
    let bytes = header.as_bytes();
    let mut index = 0usize;
    let mut depth = 0usize;
    while index < header.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(header, index);
                continue;
            }
            b'(' | b'[' | b'{' => depth += 1,
            b')' if depth == 0 => return None,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            _ if depth == 0 && header[index..].starts_with(separator) => {
                let end = index + separator.len();
                if is_identifier_boundary(header, index, end) {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn strip_top_level_type_annotation(binding: &str) -> &str {
    let bytes = binding.as_bytes();
    let mut index = 0usize;
    let mut depth = 0usize;
    while index < binding.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(binding, index);
                continue;
            }
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            b':' if depth == 0 => return binding[..index].trim(),
            _ => {}
        }
        index += 1;
    }
    binding.trim()
}

fn collect_binding_names_from_pattern(pattern: &str, out: &mut Vec<String>) {
    let bytes = pattern.as_bytes();
    let mut index = 0usize;
    while index < pattern.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(pattern, index);
                continue;
            }
            b'[' => {
                if let Some(after_computed_key) = skip_computed_binding_key(pattern, index) {
                    index = after_computed_key;
                    continue;
                }
            }
            _ => {}
        }
        if bytes[index] == b'=' {
            index = skip_binding_initializer(pattern, index + 1);
            continue;
        }
        if !is_ident_start_byte(bytes[index]) {
            index += 1;
            continue;
        }
        let mut end = index + 1;
        while bytes.get(end).copied().is_some_and(is_ident_continue_byte) {
            end += 1;
        }
        let next = skip_ascii_ws(pattern, end);
        if pattern.as_bytes().get(next) == Some(&b':') {
            index = next + 1;
            continue;
        }
        push_unique_eval_declaration(out, &pattern[index..end]);
        index = end;
    }
}

fn skip_computed_binding_key(pattern: &str, start: usize) -> Option<usize> {
    let close = skip_balanced_bracket(pattern, start)?;
    let next = skip_ascii_ws(pattern, close);
    (pattern.as_bytes().get(next) == Some(&b':')).then_some(next + 1)
}

fn skip_balanced_bracket(pattern: &str, start: usize) -> Option<usize> {
    if pattern.as_bytes().get(start) != Some(&b'[') {
        return None;
    }
    let bytes = pattern.as_bytes();
    let mut index = start;
    let mut depth = 0usize;
    while index < pattern.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(pattern, index);
                continue;
            }
            b'[' => depth += 1,
            b']' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn skip_binding_initializer(pattern: &str, start: usize) -> usize {
    let mut index = start;
    let mut depth = 0usize;
    let bytes = pattern.as_bytes();
    while index < pattern.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(pattern, index);
                continue;
            }
            b'{' | b'[' | b'(' => depth += 1,
            b'}' | b']' | b')' if depth == 0 => return index,
            b'}' | b']' | b')' => depth -= 1,
            b',' if depth == 0 => return index,
            _ => {}
        }
        index += 1;
    }
    index
}

fn skip_quoted_source(source: &str, start: usize) -> usize {
    let bytes = source.as_bytes();
    let quote = bytes[start];
    let mut index = start + 1;
    while index < source.len() {
        if bytes[index] == b'\\' {
            index = (index + 2).min(source.len());
            continue;
        }
        if bytes[index] == quote {
            return index + 1;
        }
        index += 1;
    }
    source.len()
}

fn is_identifier_boundary(source: &str, start: usize, end: usize) -> bool {
    let before = start
        .checked_sub(1)
        .and_then(|pos| source.as_bytes().get(pos).copied())
        .is_none_or(|byte| !is_ident_continue_byte(byte));
    let after = source
        .as_bytes()
        .get(end)
        .copied()
        .is_none_or(|byte| !is_ident_continue_byte(byte));
    before && after
}

fn skip_ascii_ws(source: &str, mut index: usize) -> usize {
    while source
        .as_bytes()
        .get(index)
        .copied()
        .is_some_and(|byte| byte.is_ascii_whitespace())
    {
        index += 1;
    }
    index
}

fn is_ident_start_byte(byte: u8) -> bool {
    byte == b'_' || byte == b'$' || byte.is_ascii_alphabetic()
}

fn is_ident_continue_byte(byte: u8) -> bool {
    is_ident_start_byte(byte) || byte.is_ascii_digit()
}

fn push_unique_eval_declaration(out: &mut Vec<String>, name: &str) {
    if !out.iter().any(|existing| existing == name) {
        out.push(name.to_owned());
    }
}

fn collect_eval_function_hoists(
    ast_stmts: &[Stmt],
    stmts: &[ResolvedStmt],
    source: &str,
    out: &mut Vec<EvalFunctionHoist>,
) {
    for (index, stmt) in stmts.iter().enumerate() {
        match stmt {
            ResolvedStmt::Function {
                name,
                params,
                body,
                is_generator,
                is_async,
                source_text,
                ..
            } => {
                let is_block_function = matches!(
                    ast_stmts.get(index),
                    Some(Stmt::Function { span, .. })
                        if function_decl_is_preceded_by_block_open(source, *span)
                );
                if !is_block_function {
                    out.push(EvalFunctionHoist {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                        is_generator: *is_generator,
                        is_async: *is_async,
                        source_text: source_text.clone(),
                    });
                }
            }
            // Block-level function declarations are Annex B execution-time
            // bindings. Keep their caller var hoist as undefined, but do not
            // initialize them before preceding eval-code statements.
            ResolvedStmt::Block { .. } => {}
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_eval_function_hoists(&[], then_body, source, out);
                collect_eval_function_hoists(&[], else_body, source, out);
            }
            ResolvedStmt::While { body, .. }
            | ResolvedStmt::DoWhile { body, .. }
            | ResolvedStmt::For { body, .. }
            | ResolvedStmt::ForIn { body, .. }
            | ResolvedStmt::ForOf { body, .. }
            | ResolvedStmt::ForAwaitOf { body, .. } => {
                collect_eval_function_hoists(&[], body, source, out);
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_eval_function_hoists(&[], body, source, out);
                }
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_eval_function_hoists(&[], try_block, source, out);
                if let Some(catch_block) = catch_block {
                    collect_eval_function_hoists(&[], catch_block, source, out);
                }
                if let Some(finally_block) = finally_block {
                    collect_eval_function_hoists(&[], finally_block, source, out);
                }
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_eval_function_hoists(&[], std::slice::from_ref(body.as_ref()), source, out);
            }
            _ => {}
        }
    }
}

fn function_decl_is_preceded_by_block_open(source: &str, span: Span) -> bool {
    source
        .get(..span.start)
        .and_then(|prefix| prefix.chars().rev().find(|ch| !ch.is_whitespace()))
        == Some('{')
}
