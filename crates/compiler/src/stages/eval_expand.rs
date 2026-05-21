use std::collections::HashSet;

use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_ir::binding_pattern::{BindingPattern, parse_binding_pattern};
use ts2wasm_ir::builtin_resolved::{
    ClassMethod, EvalCompletionPlan, EvalCompletionStep, EvalDeclarationPlan,
    EvalForHeadVarLanding, EvalFragmentPlan, EvalFunctionHoist, EvalKind, EvalSource,
    FunctionConstructorHostPolicy, FunctionConstructorKind, FunctionConstructorParseGoal,
    FunctionConstructorParseGoals, FunctionConstructorPlan, ResolvedArrayElement,
    ResolvedConstructor, ResolvedExpr, ResolvedObjectProp, ResolvedParam, ResolvedStmt,
};
use ts2wasm_ir::builtin_resolver::resolve_builtins;
use ts2wasm_ir::name_resolver::resolve_names;
use ts2wasm_ir::name_resolver::resolve_names_with_outer_bindings;
use ts2wasm_source::Span;
use ts2wasm_syntax::{Expr, FunctionExprOrigin, Stmt};

mod collision;
mod completion;
mod declarations;
mod function_constructor;

use collision::*;
use completion::*;
use declarations::*;
use function_constructor::*;

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
    global_eval_bindings: HashSet<String>,
}

impl EvalExpansionContext {
    fn new() -> Self {
        Self {
            scopes: vec![HashSet::new()],
            strict_contexts: vec![false],
            global_eval_bindings: HashSet::new(),
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

    fn declare_global_eval_binding(&mut self, name: impl Into<String>) {
        self.global_eval_bindings.insert(name.into());
    }

    fn is_global_eval_binding(&self, name: &str) -> bool {
        self.global_eval_bindings.contains(name)
    }

    fn has_visible_binding(&self, name: &str) -> bool {
        self.scopes.iter().any(|scope| scope.contains(name))
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
        } => expand_try_catch_stmt(try_block, catch_param, catch_block, finally_block, ctx),
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
        } => expand_function_stmt(
            name,
            params,
            body,
            is_generator,
            is_async,
            is_ambient,
            source_text,
            ctx,
        ),
        ResolvedStmt::ClassDecl {
            name,
            extends,
            constructor,
            methods,
            statics,
            static_blocks,
            private_fields,
            static_private_fields,
        } => expand_class_decl_stmt(
            name,
            extends,
            constructor,
            methods,
            statics,
            static_blocks,
            private_fields,
            static_private_fields,
            ctx,
        ),
        ResolvedStmt::AmbientValue(_) => Ok(stmt),
    }
}

fn expand_try_catch_stmt(
    try_block: Vec<ResolvedStmt>,
    catch_param: Option<String>,
    catch_block: Option<Vec<ResolvedStmt>>,
    finally_block: Option<Vec<ResolvedStmt>>,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedStmt, Diagnostic> {
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

fn expand_function_stmt(
    name: String,
    params: Vec<ResolvedParam>,
    body: Vec<ResolvedStmt>,
    is_generator: bool,
    is_async: bool,
    is_ambient: bool,
    source_text: String,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedStmt, Diagnostic> {
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

fn expand_class_decl_stmt(
    name: String,
    extends: Option<String>,
    constructor: Option<ResolvedConstructor>,
    methods: Vec<ClassMethod>,
    statics: Vec<(String, ResolvedExpr)>,
    static_blocks: Vec<(Span, Vec<ResolvedStmt>)>,
    private_fields: Vec<String>,
    static_private_fields: Vec<(String, ResolvedExpr, Span)>,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedStmt, Diagnostic> {
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
        ResolvedExpr::Eval { plan } => expand_eval_expr(plan, ctx),
        ResolvedExpr::FunctionConstructor { plan } => expand_function_constructor(plan),
        ResolvedExpr::Ident(name)
            if ctx.is_global_eval_binding(&name) && !ctx.has_visible_binding(&name) =>
        {
            Ok(eval_global_property(name))
        }
        ResolvedExpr::Unary { .. } | ResolvedExpr::Binary { .. } | ResolvedExpr::Ternary { .. } => {
            expand_operator_expr(expr, ctx)
        }
        ResolvedExpr::Call { .. }
        | ResolvedExpr::New { .. }
        | ResolvedExpr::MethodCall { .. }
        | ResolvedExpr::PropertyAccess { .. }
        | ResolvedExpr::OptionalPropertyAccess { .. }
        | ResolvedExpr::ComputedIndex { .. }
        | ResolvedExpr::OptionalComputedIndex { .. }
        | ResolvedExpr::BuiltinCall { .. }
        | ResolvedExpr::BuiltinProperty { .. }
        | ResolvedExpr::OptionalCall { .. }
        | ResolvedExpr::Spread(_)
        | ResolvedExpr::Await { .. }
        | ResolvedExpr::Yield { expr: Some(_), .. } => expand_access_call_expr(expr, ctx),
        ResolvedExpr::Assign { .. }
        | ResolvedExpr::LogicalAssign { .. }
        | ResolvedExpr::LogicalPropertyAssign { .. }
        | ResolvedExpr::LogicalComputedPropertyAssign { .. }
        | ResolvedExpr::LogicalComputedMemberAssign { .. }
        | ResolvedExpr::LogicalMemberAssign { .. }
        | ResolvedExpr::PropertyAssign { .. }
        | ResolvedExpr::PropertyAssignDynamic { .. } => expand_assignment_expr(expr, ctx),
        ResolvedExpr::Array(_)
        | ResolvedExpr::Object(_)
        | ResolvedExpr::ArrowFn { .. }
        | ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::Sequence(_) => expand_constructed_expr(expr, ctx),
        other => Ok(other),
    }
}

fn expand_eval_expr(
    plan: EvalFragmentPlan,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedExpr, Diagnostic> {
    if plan.kind == EvalKind::Direct && matches!(plan.source, EvalSource::StaticLiteral(_)) {
        validate_eval_fragment_plan(&plan)?;
        let EvalSource::StaticLiteral(src) = &plan.source else {
            unreachable!();
        };
        let caller_is_strict = plan.caller_is_strict || ctx.is_strict_context();
        let expanded =
            expand_static_eval_source(src, &ctx.visible_bindings(), &plan, caller_is_strict)?;
        for name in &expanded.caller_var_declarations {
            ctx.declare(name.clone());
        }
        return Ok(expanded.expr);
    }

    if plan.kind == EvalKind::Indirect && matches!(plan.source, EvalSource::StaticLiteral(_)) {
        validate_eval_fragment_plan(&plan)?;
        let EvalSource::StaticLiteral(src) = &plan.source else {
            unreachable!();
        };
        let caller_is_strict = plan.caller_is_strict || ctx.is_strict_context();
        let caller_bindings = ctx.visible_bindings();
        let expanded = expand_static_eval_source(src, &caller_bindings, &plan, caller_is_strict)?;
        let mut global_bindings = caller_bindings;
        for name in &expanded.global_declaration_names {
            if !global_bindings.contains(name) {
                global_bindings.push(name.clone());
            }
            ctx.declare_global_eval_binding(name.clone());
        }
        return Ok(rewrite_indirect_eval_caller_binding_collisions(
            expanded.expr,
            &global_bindings,
        ));
    }

    if matches!(plan.source, EvalSource::NonStringStatic(_)) {
        validate_eval_fragment_plan(&plan)?;
        let EvalSource::NonStringStatic(value) = plan.source else {
            unreachable!();
        };
        return expand_expr(*value, ctx);
    }

    Ok(ResolvedExpr::Eval { plan })
}

fn expand_operator_expr(
    expr: ResolvedExpr,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
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
        other => Ok(other),
    }
}

fn expand_access_call_expr(
    expr: ResolvedExpr,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
        ResolvedExpr::Call { callee, args, span } => Ok(ResolvedExpr::Call {
            callee: Box::new(expand_expr(*callee, ctx)?),
            args: expand_exprs(args, ctx)?,
            span,
        }),
        ResolvedExpr::New {
            class_name,
            args,
            span,
        } => Ok(ResolvedExpr::New {
            class_name,
            args: expand_exprs(args, ctx)?,
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
            args: expand_exprs(args, ctx)?,
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
        ResolvedExpr::BuiltinCall { builtin, args } => Ok(ResolvedExpr::BuiltinCall {
            builtin,
            args: expand_exprs(args, ctx)?,
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
            args: expand_exprs(args, ctx)?,
            span,
        }),
        ResolvedExpr::Spread(inner) => {
            Ok(ResolvedExpr::Spread(Box::new(expand_expr(*inner, ctx)?)))
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
        other => Ok(other),
    }
}

fn expand_assignment_expr(
    expr: ResolvedExpr,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
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
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            Ok(ResolvedExpr::PropertyAssignDynamic {
                object: Box::new(expand_expr(*object, ctx)?),
                key: Box::new(expand_expr(*key, ctx)?),
                value: Box::new(expand_expr(*value, ctx)?),
            })
        }
        other => Ok(other),
    }
}

fn expand_constructed_expr(
    expr: ResolvedExpr,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
        ResolvedExpr::Array(elements) => {
            let expanded = elements
                .into_iter()
                .map(|el| match el {
                    ResolvedArrayElement::Present(e) => {
                        Ok(ResolvedArrayElement::Present(expand_expr(e, ctx)?))
                    }
                    hole @ ResolvedArrayElement::Hole => Ok(hole),
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
        ResolvedExpr::ArrowFn {
            params,
            body,
            body_stmts,
            source_text,
        } => expand_arrow_fn_expr(params, body, body_stmts, source_text, ctx),
        ResolvedExpr::FunctionExpr {
            name,
            params,
            body,
            is_generator,
            origin,
            constructor_metadata,
            source_text,
        } => expand_function_expr(
            name,
            params,
            body,
            is_generator,
            origin,
            constructor_metadata,
            source_text,
            ctx,
        ),
        ResolvedExpr::ClassExpr { name, body } => expand_class_expr(name, body, ctx),
        ResolvedExpr::Sequence(exprs) => Ok(ResolvedExpr::Sequence(expand_exprs(exprs, ctx)?)),
        other => Ok(other),
    }
}

fn expand_exprs(
    exprs: Vec<ResolvedExpr>,
    ctx: &mut EvalExpansionContext,
) -> Result<Vec<ResolvedExpr>, Diagnostic> {
    exprs
        .into_iter()
        .map(|expr| expand_expr(expr, ctx))
        .collect::<Result<Vec<_>, _>>()
}

fn expand_arrow_fn_expr(
    params: Vec<String>,
    body: Box<ResolvedExpr>,
    body_stmts: Vec<ResolvedStmt>,
    source_text: String,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedExpr, Diagnostic> {
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

fn expand_function_expr(
    name: String,
    params: Vec<ResolvedParam>,
    body: Vec<ResolvedStmt>,
    is_generator: bool,
    origin: FunctionExprOrigin,
    constructor_metadata: Option<
        ts2wasm_ir::builtin_resolved::FunctionConstructorGeneratedFunction,
    >,
    source_text: String,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedExpr, Diagnostic> {
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

fn expand_class_expr(
    name: String,
    body: Vec<ResolvedStmt>,
    ctx: &mut EvalExpansionContext,
) -> Result<ResolvedExpr, Diagnostic> {
    ctx.enter_strict_scope(true);
    if !name.is_empty() {
        ctx.declare(name.clone());
    }
    let body = expand_stmts(body, ctx)?;
    ctx.exit_scope();
    Ok(ResolvedExpr::ClassExpr { name, body })
}

fn validate_eval_fragment_plan(plan: &EvalFragmentPlan) -> Result<(), Diagnostic> {
    if !plan.scope_mode_is_consistent() {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedEval,
            message: format!(
                "eval scope mode {:?} does not match {:?} eval",
                plan.scope_mode, plan.kind
            ),
            span: Some(plan.span),
            phase: None,
        });
    }
    if !plan.host_policy_is_consistent() {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedEval,
            message: format!(
                "eval host policy {:?} does not match {:?} eval source",
                plan.host_policy, plan.kind
            ),
            span: Some(plan.span),
            phase: None,
        });
    }
    if !plan.completion_state_is_consistent() {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedEval,
            message: "eval completion/declaration plan does not match fragment plan".to_owned(),
            span: Some(plan.span),
            phase: None,
        });
    }
    Ok(())
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
