use super::*;

#[allow(clippy::type_complexity)]
pub(super) fn resolve_private_elements(
    class_name: &str,
    _extends_name: Option<&String>,
    private_elements: &[ClassPrivateElement],
) -> Result<
    (
        Vec<String>,
        Vec<(String, ResolvedExpr, Span)>,
        Vec<ResolvedStmt>,
        Vec<ClassMethod>,
    ),
    Diagnostic,
> {
    let mut fields = Vec::new();
    let mut static_fields = Vec::new();
    let mut initializers = Vec::new();
    let mut methods = Vec::new();
    let mut seen = HashSet::new();

    for element in private_elements {
        match element {
            ClassPrivateElement::Field {
                name,
                name_span,
                value,
                is_static,
                span,
            } => {
                if *is_static {
                    if !seen.insert(name.clone()) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: duplicate private field `#{name}` in class `{class_name}`"
                            ),
                            span: Some(*span),
                        });
                    }
                    static_fields.push((
                        name.clone(),
                        value
                            .as_ref()
                            .map(resolve_expr)
                            .transpose()?
                            .unwrap_or(ResolvedExpr::Undefined),
                        *span,
                    ));
                    continue;
                }
                if !seen.insert(name.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: duplicate private field `#{name}` in class `{class_name}`"
                        ),
                        span: Some(*span),
                    });
                }
                fields.push(name.clone());
                initializers.push(ResolvedStmt::Expr(ResolvedExpr::PropertyAssign {
                    object: Box::new(ResolvedExpr::This { span: *name_span }),
                    key: format!("#{name}"),
                    value: Box::new(
                        value
                            .as_ref()
                            .map(resolve_expr)
                            .transpose()?
                            .unwrap_or(ResolvedExpr::Undefined),
                    ),
                    span: *span,
                }));
            }
            ClassPrivateElement::Method {
                name,
                params,
                body,
                is_static,
                span,
                ..
            } => {
                if !seen.insert(name.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: duplicate private element `#{name}` in class `{class_name}`"
                        ),
                        span: Some(*span),
                    });
                }
                methods.push(ClassMethod {
                    name: if *is_static {
                        format!("static::#{name}")
                    } else {
                        format!("#{name}")
                    },
                    params: params
                        .iter()
                        .map(|(param_name, default, is_rest)| {
                            Ok(ResolvedParam {
                                name: param_name.clone(),
                                default: default.as_ref().map(resolve_expr).transpose()?,
                                is_rest: *is_rest,
                                span: Some(*span),
                            })
                        })
                        .collect::<Result<Vec<_>, Diagnostic>>()?,
                    body: body
                        .iter()
                        .map(resolve_stmt)
                        .collect::<Result<Vec<_>, _>>()?,
                    captures: Vec::new(),
                });
            }
            ClassPrivateElement::Getter {
                name,
                body,
                is_static,
                span,
                ..
            } => {
                if !seen.insert(name.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: duplicate private element `#{name}` in class `{class_name}`"
                        ),
                        span: Some(*span),
                    });
                }
                methods.push(ClassMethod {
                    name: if *is_static {
                        format!("static::{}", private_getter_method_name(name))
                    } else {
                        private_getter_method_name(name)
                    },
                    params: Vec::new(),
                    body: body
                        .iter()
                        .map(resolve_stmt)
                        .collect::<Result<Vec<_>, _>>()?,
                    captures: Vec::new(),
                });
            }
            ClassPrivateElement::Setter {
                name,
                param,
                body,
                is_static,
                span,
                ..
            } => {
                if block_contains_return_stmt(body) {
                    return Err(unsupported_private_element(
                        "private setters with explicit return are not supported in this private setter runtime slice",
                        *span,
                    ));
                }
                if !seen.insert(name.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: duplicate private element `#{name}` in class `{class_name}`"
                        ),
                        span: Some(*span),
                    });
                }
                let mut resolved_body = body
                    .iter()
                    .map(resolve_stmt)
                    .collect::<Result<Vec<_>, _>>()?;
                resolved_body.push(ResolvedStmt::Return(ResolvedExpr::Ident(param.clone())));
                methods.push(ClassMethod {
                    name: if *is_static {
                        format!("static::{}", private_setter_method_name(name))
                    } else {
                        private_setter_method_name(name)
                    },
                    params: vec![ResolvedParam {
                        name: param.clone(),
                        default: None,
                        is_rest: false,
                        span: Some(*span),
                    }],
                    body: resolved_body,
                    captures: Vec::new(),
                });
            }
        }
    }

    Ok((fields, static_fields, initializers, methods))
}

pub(super) fn place_private_field_initializers(
    initializers: &[ResolvedStmt],
    body: Vec<ResolvedStmt>,
    is_derived: bool,
) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    if initializers.is_empty() {
        return Ok(body);
    }
    if is_derived {
        let Some(super_index) = body.iter().position(is_direct_super_call_stmt) else {
            return Err(unsupported_private_element(
                "derived classes with private instance fields require a direct top-level super() call before constructor body initialization in this runtime slice",
                Span { start: 0, end: 0 },
            ));
        };
        let mut merged = Vec::with_capacity(body.len() + initializers.len());
        merged.extend_from_slice(&body[..=super_index]);
        merged.extend_from_slice(initializers);
        merged.extend_from_slice(&body[super_index + 1..]);
        return Ok(merged);
    }
    let mut body = body;
    let mut merged = initializers.to_vec();
    merged.append(&mut body);
    Ok(merged)
}

pub(super) fn implicit_derived_private_field_constructor_body(
    initializers: &[ResolvedStmt],
) -> Vec<ResolvedStmt> {
    let mut body = vec![ResolvedStmt::Expr(ResolvedExpr::Call {
        callee: Box::new(ResolvedExpr::Ident("super".to_owned())),
        args: Vec::new(),
        span: Span { start: 0, end: 0 },
    })];
    body.extend_from_slice(initializers);
    body
}

fn is_direct_super_call_stmt(stmt: &ResolvedStmt) -> bool {
    matches!(
        stmt,
        ResolvedStmt::Expr(ResolvedExpr::Call { callee, .. })
            if matches!(callee.as_ref(), ResolvedExpr::Ident(name) if name == "super")
    )
}

pub(super) fn unsupported_private_element(detail: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("issue-255: {detail}"),
        span: Some(span),
    }
}

pub(super) fn is_private_member_key(key: &str) -> bool {
    key.starts_with('#')
}

pub(super) fn private_getter_method_name(name: &str) -> String {
    format!("#get::{name}")
}

pub(super) fn private_setter_method_name(name: &str) -> String {
    format!("#set::{name}")
}

pub(crate) fn static_private_field_local_name(class_name: &str, field_name: &str) -> String {
    format!("__ts2wasm_static_private::{class_name}::{field_name}")
}

pub(super) fn block_contains_return_stmt(statements: &[Stmt]) -> bool {
    statements.iter().any(stmt_contains_return_stmt)
}

pub(super) fn stmt_contains_return_stmt(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Return { .. } => true,
        Stmt::If {
            then_body,
            else_body,
            ..
        } => block_contains_return_stmt(then_body) || block_contains_return_stmt(else_body),
        Stmt::While { body, .. }
        | Stmt::DoWhile { body, .. }
        | Stmt::ForIn { body, .. }
        | Stmt::ForOf { body, .. } => block_contains_return_stmt(body),
        Stmt::For { init, body, .. } => {
            init.as_deref().is_some_and(stmt_contains_return_stmt)
                || block_contains_return_stmt(body)
        }
        Stmt::Block { statements: body, .. } => block_contains_return_stmt(body),
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_contains_return_stmt(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|body| block_contains_return_stmt(body))
                || finally_block
                    .as_ref()
                    .is_some_and(|body| block_contains_return_stmt(body))
        }
        Stmt::Switch { cases, .. } => cases
            .iter()
            .any(|(_, body)| block_contains_return_stmt(body)),
        Stmt::Labeled { body, .. } => stmt_contains_return_stmt(body),
        Stmt::Function { .. } | Stmt::ClassDecl { .. } | Stmt::AmbientValueDecl { .. } => false,
        Stmt::ImportSideEffect { .. }
        | Stmt::ImportNamed { .. }
        | Stmt::ImportDefault { .. }
        | Stmt::ImportDefaultNamed { .. }
        | Stmt::ImportNamespace { .. }
        | Stmt::ImportDefaultNamespace { .. }
        | Stmt::ExportNamed { .. }
        | Stmt::ExportNamedFrom { .. }
        | Stmt::ExportAllFrom { .. }
        | Stmt::ExportNamespaceFrom { .. }
        | Stmt::ExportDecl { .. }
        | Stmt::ExportDefault { .. }
        | Stmt::ExportAssignment { .. }
        | Stmt::Let { .. }
        | Stmt::Assign { .. }
        | Stmt::Expr { .. }
        | Stmt::Throw { .. }
        | Stmt::Break { .. }
        | Stmt::Continue { .. } => false,
        Stmt::Block { .. } => false,
    }
}

pub(super) fn validate_static_block_supported(block: &ClassStaticBlock) -> Result<(), Diagnostic> {
    for stmt in &block.body {
        validate_static_block_stmt(stmt)?;
    }
    Ok(())
}

pub(super) fn validate_static_block_stmt(stmt: &Stmt) -> Result<(), Diagnostic> {
    match stmt {
        Stmt::Return { span, .. } => Err(static_block_unsupported(
            "return statements are not valid in class static blocks",
            *span,
        )),
        Stmt::Block { statements: body, .. } => {
            for stmt in body {
                validate_static_block_stmt(stmt)?;
            }
            Ok(())
        }
        Stmt::Let { expr, .. }
        | Stmt::Assign { expr, .. }
        | Stmt::Expr { expr, .. }
        | Stmt::Throw { expr, .. } => validate_static_block_expr(expr),
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            validate_static_block_expr(condition)?;
            validate_static_block_stmts(then_body)?;
            validate_static_block_stmts(else_body)
        }
        Stmt::While {
            condition, body, ..
        }
        | Stmt::DoWhile {
            condition, body, ..
        } => {
            validate_static_block_expr(condition)?;
            validate_static_block_stmts(body)
        }
        Stmt::Function { params, body, .. } => {
            for (_, default, _) in params {
                if let Some(default) = default {
                    validate_static_block_expr(default)?;
                }
            }
            validate_static_block_stmts(body)
        }
        Stmt::ClassDecl {
            extends,
            body,
            static_blocks,
            ..
        } => {
            if let Some(extends) = extends {
                validate_static_block_expr(extends)?;
            }
            validate_static_block_stmts(body)?;
            for block in static_blocks {
                validate_static_block_supported(block)?;
            }
            Ok(())
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            validate_static_block_stmts(try_block)?;
            if let Some(block) = catch_block {
                validate_static_block_stmts(block)?;
            }
            if let Some(block) = finally_block {
                validate_static_block_stmts(block)?;
            }
            Ok(())
        }
        Stmt::Switch { expr, cases, .. } => {
            validate_static_block_expr(expr)?;
            for (case, body) in cases {
                if let Some(case) = case {
                    validate_static_block_expr(case)?;
                }
                validate_static_block_stmts(body)?;
            }
            Ok(())
        }
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            if let Some(init) = init {
                validate_static_block_stmt(init)?;
            }
            if let Some(condition) = condition {
                validate_static_block_expr(condition)?;
            }
            if let Some(update) = update {
                validate_static_block_expr(update)?;
            }
            validate_static_block_stmts(body)
        }
        Stmt::ForIn { iter, body, .. } | Stmt::ForOf { iter, body, .. } => {
            validate_static_block_expr(iter)?;
            validate_static_block_stmts(body)
        }
        Stmt::Labeled { body, .. } => validate_static_block_stmt(body),
        Stmt::Block { .. }
        | Stmt::Break { .. }
        | Stmt::Continue { .. }
        | Stmt::AmbientValueDecl { .. } => Ok(()),
        Stmt::ImportSideEffect { span, .. }
        | Stmt::ImportNamed { span, .. }
        | Stmt::ImportDefault { span, .. }
        | Stmt::ImportDefaultNamed { span, .. }
        | Stmt::ImportNamespace { span, .. }
        | Stmt::ImportDefaultNamespace { span, .. }
        | Stmt::ExportNamed { span, .. }
        | Stmt::ExportNamedFrom { span, .. }
        | Stmt::ExportAllFrom { span, .. }
        | Stmt::ExportNamespaceFrom { span, .. }
        | Stmt::ExportDecl { span, .. }
        | Stmt::ExportAssignment { span, .. }
        | Stmt::ExportDefault { span, .. } => Err(static_block_unsupported(
            "module declarations inside class static blocks are not supported",
            *span,
        )),
    }
}

pub(super) fn validate_static_block_stmts(stmts: &[Stmt]) -> Result<(), Diagnostic> {
    for stmt in stmts {
        validate_static_block_stmt(stmt)?;
    }
    Ok(())
}

pub(super) fn validate_static_block_expr(expr: &Expr) -> Result<(), Diagnostic> {
    match expr {
        Expr::This { span } => Err(static_block_unsupported(
            "`this` in class static blocks needs constructor-object binding support",
            *span,
        )),
        Expr::Ident { name, span } if name == "super" => Err(static_block_unsupported(
            "`super` in class static blocks is not supported",
            *span,
        )),
        Expr::Unary { expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Assign { expr, .. }
        | Expr::Await { expr, .. }
        | Expr::Spread { expr, .. } => validate_static_block_expr(expr),
        Expr::Binary {
            left: expr_left,
            right: expr_right,
            ..
        }
        | Expr::Index {
            object: expr_left,
            index: expr_right,
            ..
        }
        | Expr::PropertyAssign {
            object: expr_left,
            value: expr_right,
            ..
        } => {
            validate_static_block_expr(expr_left)?;
            validate_static_block_expr(expr_right)
        }
        Expr::InstanceOf {
            expr, type_expr, ..
        } => {
            validate_static_block_expr(expr)?;
            validate_static_block_expr(type_expr)
        }
        Expr::LogicalAssign { name: _, expr, .. } => validate_static_block_expr(expr),
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => {
            if let Some(object) = object_expr {
                validate_static_block_expr(object)?;
            }
            if let Some(key) = computed_key {
                validate_static_block_expr(key)?;
            }
            validate_static_block_expr(expr)
        }
        Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
            validate_static_block_expr(callee)?;
            validate_static_block_exprs(args)
        }
        Expr::Member { object, .. } | Expr::OptionalMember { object, .. } => {
            validate_static_block_expr(object)
        }
        Expr::OptionalIndex { object, index, .. } => {
            validate_static_block_expr(object)?;
            validate_static_block_expr(index)
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            validate_static_block_expr(object)?;
            validate_static_block_expr(index)?;
            validate_static_block_expr(value)
        }
        Expr::Array { elements, .. } => {
            for element in elements {
                match element {
                    ArrayLiteralElement::Present(expr) | ArrayLiteralElement::Spread(expr) => {
                        validate_static_block_expr(expr)?;
                    }
                    ArrayLiteralElement::Hole(_) => {}
                }
            }
            Ok(())
        }
        Expr::Object { props, .. } => {
            for (_, value) in props {
                validate_static_block_expr(value)?;
            }
            Ok(())
        }
        Expr::New { expr, args, .. } => {
            validate_static_block_expr(expr)?;
            validate_static_block_exprs(args)
        }
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            validate_static_block_expr(condition)?;
            validate_static_block_expr(then_expr)?;
            validate_static_block_expr(else_expr)
        }
        Expr::ArrowFn { body, .. } => validate_static_block_expr(body),
        Expr::FunctionExpr { body, .. } => validate_static_block_stmts(body),
        Expr::ClassExpr { body, .. } => validate_static_block_stmts(body),
        Expr::Number { .. }
        | Expr::BigInt { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::Undefined { .. }
        | Expr::Ident { .. } => Ok(()),
    }
}

pub(super) fn validate_static_block_exprs(exprs: &[Expr]) -> Result<(), Diagnostic> {
    for expr in exprs {
        validate_static_block_expr(expr)?;
    }
    Ok(())
}

pub(super) fn static_block_unsupported(detail: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("issue-254: {detail}"),
        span: Some(span),
    }
}
