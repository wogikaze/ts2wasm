use std::collections::HashSet;

use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_ir::builtin_resolved::{
    EvalKind, EvalSource, ResolvedExpr, ResolvedParam, ResolvedStmt,
};
use ts2wasm_ir::builtin_resolver::resolve_builtins;
use ts2wasm_ir::name_resolver::resolve_names;
use ts2wasm_ir::name_resolver::{
    INTRINSIC_FUNCTION_CONSTRUCTOR_CALL, INTRINSIC_FUNCTION_CONSTRUCTOR_NEW,
    resolve_names_with_outer_bindings,
};
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
pub(crate) fn expand_static_eval_fragments(
    resolved: Vec<ResolvedStmt>,
) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    let mut ctx = EvalExpansionContext::new();
    expand_stmts(resolved, &mut ctx)
}

#[derive(Debug, Default)]
struct EvalExpansionContext {
    scopes: Vec<HashSet<String>>,
}

impl EvalExpansionContext {
    fn new() -> Self {
        Self {
            scopes: vec![HashSet::new()],
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
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
            ctx.enter_scope();
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
    ctx.enter_scope();
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
    ctx.enter_scope();
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
        ResolvedExpr::Eval {
            kind: EvalKind::Direct,
            source: EvalSource::StaticLiteral(ref src),
            ..
        } => expand_static_eval_source(src, &ctx.visible_bindings()),
        ResolvedExpr::Eval {
            kind: EvalKind::Indirect,
            source: EvalSource::StaticLiteral(ref src),
            ..
        } => expand_static_eval_source(src, &[]),
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
            expand_function_constructor(
                FunctionConstructorKind::Call,
                INTRINSIC_FUNCTION_CONSTRUCTOR_CALL,
                args,
                span,
            )
        }
        ResolvedExpr::New {
            class_name,
            args,
            span,
        } if class_name == INTRINSIC_FUNCTION_CONSTRUCTOR_NEW => expand_function_constructor(
            FunctionConstructorKind::New,
            INTRINSIC_FUNCTION_CONSTRUCTOR_NEW,
            args,
            span,
        ),
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
            ctx.enter_scope();
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
            source_text,
        } => {
            ctx.enter_scope();
            if !name.is_empty() {
                ctx.declare(name.clone());
            }
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
                source_text,
            })
        }
        ResolvedExpr::ClassExpr { name, body } => {
            ctx.enter_scope();
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

fn expand_static_eval_source(
    src: &str,
    outer_bindings: &[String],
) -> Result<ResolvedExpr, Diagnostic> {
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

    let name_resolved = if outer_bindings.is_empty() {
        resolve_names(&program)?
    } else {
        resolve_names_with_outer_bindings(&program, outer_bindings)?
    };
    let builtin_resolved = resolve_builtins(&name_resolved)?;

    extract_completion_value(builtin_resolved)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionConstructorKind {
    Call,
    New,
}

fn expand_function_constructor(
    kind: FunctionConstructorKind,
    intrinsic_name: &str,
    args: Vec<ResolvedExpr>,
    span: ts2wasm_source::Span,
) -> Result<ResolvedExpr, Diagnostic> {
    let mut strings = Vec::with_capacity(args.len());
    for arg in &args {
        match arg {
            ResolvedExpr::String(value) => strings.push(value.clone()),
            _ => {
                return Ok(function_constructor_host_lane(
                    kind,
                    intrinsic_name,
                    args,
                    span,
                ));
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
    validate_static_function_constructor_early_errors(&program, span)?;

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
            let mut function_ctx = EvalExpansionContext::new();
            function_ctx.declare_params(&params);
            let body = expand_stmts(body, &mut function_ctx)?;
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

fn function_constructor_host_lane(
    kind: FunctionConstructorKind,
    intrinsic_name: &str,
    args: Vec<ResolvedExpr>,
    span: ts2wasm_source::Span,
) -> ResolvedExpr {
    match kind {
        FunctionConstructorKind::Call => ResolvedExpr::Call {
            callee: Box::new(ResolvedExpr::Ident(intrinsic_name.to_owned())),
            args,
            span,
        },
        FunctionConstructorKind::New => ResolvedExpr::New {
            class_name: intrinsic_name.to_owned(),
            args,
            span,
        },
    }
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
/// * Empty block → `ResolvedExpr::Undefined`
/// * Single expression statement → the expression itself
/// * Multiple statements → the last statement's completion value
fn extract_completion_value(stmts: Vec<ResolvedStmt>) -> Result<ResolvedExpr, Diagnostic> {
    let mut exprs = Vec::new();
    for stmt in stmts {
        exprs.push(eval_statement_completion_expr(stmt));
    }
    Ok(match exprs.len() {
        0 => ResolvedExpr::Undefined,
        1 => exprs.pop().expect("single completion expression exists"),
        _ => ResolvedExpr::Sequence(exprs),
    })
}

fn eval_statement_completion_expr(stmt: ResolvedStmt) -> ResolvedExpr {
    match stmt {
        ResolvedStmt::Expr(expr) => expr,
        ResolvedStmt::Assign(name, expr) => ResolvedExpr::Assign {
            name,
            expr: Box::new(expr),
        },
        ResolvedStmt::Let(_, expr) | ResolvedStmt::Return(expr) => expr,
        _ => ResolvedExpr::Undefined,
    }
}
