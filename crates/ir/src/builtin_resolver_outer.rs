use super::*;
use ts2wasm_syntax::ArrayLiteralElement;

pub(super) fn collect_top_level_bindings(program: &[Stmt]) -> Result<HashSet<String>, Diagnostic> {
    let mut bindings = HashSet::new();
    for stmt in program {
        collect_stmt_declared_bindings(stmt, &mut bindings)?;
    }
    Ok(bindings)
}

/// Collects class and enum declaration names from the top-level program.
pub(super) fn collect_top_level_class_names(program: &[Stmt]) -> HashSet<String> {
    let mut names = HashSet::new();
    for stmt in program {
        match stmt {
            Stmt::ClassDecl { name, .. } | Stmt::EnumDecl { name, .. } => {
                names.insert(name.clone());
            }
            _ => {}
        }
    }
    names
}

pub(super) fn collect_stmt_declared_bindings(
    stmt: &Stmt,
    bindings: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    match stmt {
        Stmt::Let { name, span, .. } => {
            collect_binding_names(name, Some(*span), bindings)?;
        }
        Stmt::Function { name, .. }
        | Stmt::ClassDecl { name, .. }
        | Stmt::EnumDecl { name, .. } => {
            bindings.insert(name.clone());
        }
        Stmt::TryCatch {
            catch_param,
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            if let Some(param) = catch_param {
                bindings.insert(param.clone());
            }
            collect_stmt_declared_bindings_in_block(try_block, bindings)?;
            if let Some(block) = catch_block {
                collect_stmt_declared_bindings_in_block(block, bindings)?;
            }
            if let Some(block) = finally_block {
                collect_stmt_declared_bindings_in_block(block, bindings)?;
            }
        }
        Stmt::If {
            then_body,
            else_body,
            ..
        } => {
            collect_stmt_declared_bindings_in_block(then_body, bindings)?;
            collect_stmt_declared_bindings_in_block(else_body, bindings)?;
        }
        Stmt::While { body, .. }
        | Stmt::DoWhile { body, .. }
        | Stmt::For { body, .. }
        | Stmt::ForIn { body, .. }
        | Stmt::ForOf { body, .. } => {
            collect_stmt_declared_bindings_in_block(body, bindings)?;
            if let Stmt::ForIn { var, .. } | Stmt::ForOf { var, .. } = stmt {
                bindings.insert(var.clone());
            }
        }
        Stmt::Switch { cases, .. } => {
            for (_, body) in cases {
                collect_stmt_declared_bindings_in_block(body, bindings)?;
            }
        }
        Stmt::Labeled { body, .. } => collect_stmt_declared_bindings(body, bindings)?,
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
        | Stmt::AmbientValueDecl { .. }
        | Stmt::Assign { .. }
        | Stmt::Expr { .. }
        | Stmt::Return { .. }
        | Stmt::Throw { .. }
        | Stmt::Break { .. }
        | Stmt::Continue { .. } => {}
        Stmt::Block { .. } => {}
    }
    Ok(())
}

pub(super) fn collect_stmt_declared_bindings_in_block(
    block: &[Stmt],
    bindings: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    for stmt in block {
        collect_stmt_declared_bindings(stmt, bindings)?;
    }
    Ok(())
}

pub(super) fn collect_binding_names(
    name: &str,
    span: Option<Span>,
    bindings: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    if let Some(pattern) = parse_binding_pattern(name, span)? {
        for binding_name in pattern.names() {
            bindings.insert(binding_name.to_owned());
        }
    } else {
        bindings.insert(name.to_owned());
    }
    Ok(())
}

pub(super) fn reject_class_method_outer_local_references(
    class_name: &str,
    method_name: &str,
    params: &[(String, Option<Expr>, bool)],
    body: &[Stmt],
    outer_bindings: &HashSet<String>,
    class_names: &HashSet<String>,
) -> Result<(), Diagnostic> {
    let captures = class_method_outer_local_captures(
        class_name,
        method_name,
        params,
        body,
        outer_bindings,
        class_names,
    )?;
    if let Some(name) = captures.first() {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-289: class constructor `{method_name}` references outer local `{name}`; class constructor lexical captures require environment support"
            ),
            span: first_outer_local_reference_in_stmts(
                body,
                outer_bindings,
                &class_method_local_names(class_name, params, body)?,
                class_names,
            )
            .map(|(_, span)| span),

            phase: None,
        });
    }

    Ok(())
}

pub(super) fn class_method_outer_local_captures(
    class_name: &str,
    method_name: &str,
    params: &[(String, Option<Expr>, bool)],
    body: &[Stmt],
    outer_bindings: &HashSet<String>,
    class_names: &HashSet<String>,
) -> Result<Vec<String>, Diagnostic> {
    if outer_bindings.is_empty() {
        return Ok(Vec::new());
    }

    let mut method_locals = class_method_local_names(class_name, params, body)?;

    let mut capture_names = Vec::new();
    while let Some((name, span)) =
        first_outer_local_reference_in_stmts(body, outer_bindings, &method_locals, class_names)
    {
        if params.iter().any(|(_, _, is_rest)| *is_rest) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-289: class method `{method_name}` captures outer local `{name}` with a rest parameter; hidden capture parameters after rest require a broader call ABI",
                ),
                span: Some(span),

                phase: None,
            });
        }

        method_locals.insert(name.clone());
        capture_names.push(name);
    }

    Ok(capture_names)
}

pub(super) fn class_method_local_names(
    class_name: &str,
    params: &[(String, Option<Expr>, bool)],
    body: &[Stmt],
) -> Result<HashSet<String>, Diagnostic> {
    let mut method_locals = HashSet::new();
    method_locals.insert(class_name.to_owned());
    for (param, default, _) in params {
        collect_binding_names(param, default.as_ref().map(Expr::span), &mut method_locals)?;
    }
    collect_stmt_declared_bindings_in_block(body, &mut method_locals)?;
    Ok(method_locals)
}

pub(super) fn first_outer_local_reference_in_stmts(
    stmts: &[Stmt],
    outer_bindings: &HashSet<String>,
    method_locals: &HashSet<String>,
    class_names: &HashSet<String>,
) -> Option<(String, Span)> {
    stmts.iter().find_map(|stmt| {
        first_outer_local_reference_in_stmt(stmt, outer_bindings, method_locals, class_names)
    })
}

pub(super) fn first_outer_local_reference_in_stmt(
    stmt: &Stmt,
    outer_bindings: &HashSet<String>,
    method_locals: &HashSet<String>,
    class_names: &HashSet<String>,
) -> Option<(String, Span)> {
    match stmt {
        Stmt::Let { expr, .. } | Stmt::Return { expr, .. } | Stmt::Throw { expr, .. } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals, class_names)
        }
        Stmt::Assign { name, expr, span } => {
            reference_if_outer(name, *span, outer_bindings, method_locals).or_else(|| {
                first_outer_local_reference_in_expr(
                    expr,
                    outer_bindings,
                    method_locals,
                    class_names,
                )
            })
        }
        Stmt::Expr { expr, .. } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals, class_names)
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => first_outer_local_reference_in_expr(
            condition,
            outer_bindings,
            method_locals,
            class_names,
        )
        .or_else(|| {
            first_outer_local_reference_in_stmts(
                then_body,
                outer_bindings,
                method_locals,
                class_names,
            )
        })
        .or_else(|| {
            first_outer_local_reference_in_stmts(
                else_body,
                outer_bindings,
                method_locals,
                class_names,
            )
        }),
        Stmt::While {
            condition, body, ..
        }
        | Stmt::DoWhile {
            body, condition, ..
        } => first_outer_local_reference_in_expr(
            condition,
            outer_bindings,
            method_locals,
            class_names,
        )
        .or_else(|| {
            first_outer_local_reference_in_stmts(body, outer_bindings, method_locals, class_names)
        }),
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => init
            .as_deref()
            .and_then(|stmt| {
                first_outer_local_reference_in_stmt(
                    stmt,
                    outer_bindings,
                    method_locals,
                    class_names,
                )
            })
            .or_else(|| {
                condition.as_ref().and_then(|expr| {
                    first_outer_local_reference_in_expr(
                        expr,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
            })
            .or_else(|| {
                update.as_ref().and_then(|expr| {
                    first_outer_local_reference_in_expr(
                        expr,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
            })
            .or_else(|| {
                first_outer_local_reference_in_stmts(
                    body,
                    outer_bindings,
                    method_locals,
                    class_names,
                )
            }),
        Stmt::ForIn { iter, body, .. } | Stmt::ForOf { iter, body, .. } => {
            first_outer_local_reference_in_expr(iter, outer_bindings, method_locals, class_names)
                .or_else(|| {
                    first_outer_local_reference_in_stmts(
                        body,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => first_outer_local_reference_in_stmts(
            try_block,
            outer_bindings,
            method_locals,
            class_names,
        )
        .or_else(|| {
            catch_block.as_ref().and_then(|block| {
                first_outer_local_reference_in_stmts(
                    block,
                    outer_bindings,
                    method_locals,
                    class_names,
                )
            })
        })
        .or_else(|| {
            finally_block.as_ref().and_then(|block| {
                first_outer_local_reference_in_stmts(
                    block,
                    outer_bindings,
                    method_locals,
                    class_names,
                )
            })
        }),
        Stmt::Switch { expr, cases, .. } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals, class_names)
                .or_else(|| {
                    cases.iter().find_map(|(case_expr, body)| {
                        case_expr
                            .as_ref()
                            .and_then(|expr| {
                                first_outer_local_reference_in_expr(
                                    expr,
                                    outer_bindings,
                                    method_locals,
                                    class_names,
                                )
                            })
                            .or_else(|| {
                                first_outer_local_reference_in_stmts(
                                    body,
                                    outer_bindings,
                                    method_locals,
                                    class_names,
                                )
                            })
                    })
                })
        }
        Stmt::Labeled { body, .. } => {
            first_outer_local_reference_in_stmt(body, outer_bindings, method_locals, class_names)
        }
        Stmt::Function { .. }
        | Stmt::ClassDecl { .. }
        | Stmt::EnumDecl { .. }
        | Stmt::AmbientValueDecl { .. } => None,
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
        | Stmt::Break { .. }
        | Stmt::Continue { .. } => None,
        Stmt::Block { .. } => None,
    }
}

pub(super) fn first_outer_local_reference_in_expr(
    expr: &Expr,
    outer_bindings: &HashSet<String>,
    method_locals: &HashSet<String>,
    class_names: &HashSet<String>,
) -> Option<(String, Span)> {
    match expr {
        Expr::Ident { name, span } => {
            reference_if_outer(name, *span, outer_bindings, method_locals)
        }
        Expr::Unary { expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Await { expr, .. }
        | Expr::Spread { expr, .. } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals, class_names)
        }
        Expr::Yield { expr, .. } => expr.as_deref().and_then(|expr| {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals, class_names)
        }),
        Expr::Binary { left, right, .. } => {
            first_outer_local_reference_in_expr(left, outer_bindings, method_locals, class_names)
                .or_else(|| {
                    first_outer_local_reference_in_expr(
                        right,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
        }
        Expr::Member { object, .. } | Expr::OptionalMember { object, .. } => {
            first_outer_local_reference_in_expr(object, outer_bindings, method_locals, class_names)
        }
        Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
            first_outer_local_reference_in_expr(callee, outer_bindings, method_locals, class_names)
                .or_else(|| {
                    args.iter().find_map(|arg| {
                        first_outer_local_reference_in_expr(
                            arg,
                            outer_bindings,
                            method_locals,
                            class_names,
                        )
                    })
                })
        }
        Expr::Assign { name, expr, span }
        | Expr::LogicalAssign {
            name, expr, span, ..
        } => reference_if_outer(name, *span, outer_bindings, method_locals).or_else(|| {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals, class_names)
        }),
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => object_expr
            .as_deref()
            .and_then(|object| {
                first_outer_local_reference_in_expr(
                    object,
                    outer_bindings,
                    method_locals,
                    class_names,
                )
            })
            .or_else(|| {
                computed_key.as_deref().and_then(|key| {
                    first_outer_local_reference_in_expr(
                        key,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
            })
            .or_else(|| {
                first_outer_local_reference_in_expr(
                    expr,
                    outer_bindings,
                    method_locals,
                    class_names,
                )
            }),
        Expr::Array { elements, .. } => elements.iter().find_map(|element| match element {
            ArrayLiteralElement::Present(expr) | ArrayLiteralElement::Spread(expr) => {
                first_outer_local_reference_in_expr(
                    expr,
                    outer_bindings,
                    method_locals,
                    class_names,
                )
            }
            ArrayLiteralElement::Hole(_) => None,
        }),
        Expr::Object { props, .. } => props.iter().find_map(|(_, value)| {
            first_outer_local_reference_in_expr(value, outer_bindings, method_locals, class_names)
        }),
        Expr::Index { object, index, .. } | Expr::OptionalIndex { object, index, .. } => {
            first_outer_local_reference_in_expr(object, outer_bindings, method_locals, class_names)
                .or_else(|| {
                    first_outer_local_reference_in_expr(
                        index,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
        }
        Expr::New { expr, args, .. } => {
            // Allow `new ClassName()` when ClassName is a class declaration
            // (forward references to later class declarations in the same scope)
            let callee_ref = match expr.as_ref() {
                Expr::Ident { name, .. } if class_names.contains(name) => None,
                _ => first_outer_local_reference_in_expr(
                    expr,
                    outer_bindings,
                    method_locals,
                    class_names,
                ),
            };
            callee_ref.or_else(|| {
                args.iter().find_map(|arg| {
                    first_outer_local_reference_in_expr(
                        arg,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
            })
        }
        Expr::InstanceOf {
            expr, type_expr, ..
        } => first_outer_local_reference_in_expr(expr, outer_bindings, method_locals, class_names)
            .or_else(|| {
                first_outer_local_reference_in_expr(
                    type_expr,
                    outer_bindings,
                    method_locals,
                    class_names,
                )
            }),
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => first_outer_local_reference_in_expr(
            condition,
            outer_bindings,
            method_locals,
            class_names,
        )
        .or_else(|| {
            first_outer_local_reference_in_expr(
                then_expr,
                outer_bindings,
                method_locals,
                class_names,
            )
        })
        .or_else(|| {
            first_outer_local_reference_in_expr(
                else_expr,
                outer_bindings,
                method_locals,
                class_names,
            )
        }),
        Expr::PropertyAssign { object, value, .. } => {
            first_outer_local_reference_in_expr(object, outer_bindings, method_locals, class_names)
                .or_else(|| {
                    first_outer_local_reference_in_expr(
                        value,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            first_outer_local_reference_in_expr(object, outer_bindings, method_locals, class_names)
                .or_else(|| {
                    first_outer_local_reference_in_expr(
                        index,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
                .or_else(|| {
                    first_outer_local_reference_in_expr(
                        value,
                        outer_bindings,
                        method_locals,
                        class_names,
                    )
                })
        }
        Expr::ArrowFn { .. } | Expr::FunctionExpr { .. } | Expr::ClassExpr { .. } => None,
        Expr::Number { .. }
        | Expr::BigInt { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::Undefined { .. }
        | Expr::This { .. } => None,
        Expr::NewTarget { .. } => None,
    }
}

pub(super) fn reference_if_outer(
    name: &str,
    span: Span,
    outer_bindings: &HashSet<String>,
    method_locals: &HashSet<String>,
) -> Option<(String, Span)> {
    if outer_bindings.contains(name) && !method_locals.contains(name) {
        Some((name.to_owned(), span))
    } else {
        None
    }
}
