use crate::{DiagCode, Diagnostic, Expr, Stmt};

use super::builtin::BuiltinId;
use super::builtin::BuiltinPropertyId;
use super::builtin_resolved::{ResolvedExpr, ResolvedStmt};

pub(crate) fn resolve_builtins(program: &[Stmt]) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    program.iter().map(resolve_stmt).collect()
}

fn resolve_stmt(stmt: &Stmt) -> Result<ResolvedStmt, Diagnostic> {
    match stmt {
        Stmt::Let { name, expr, .. } => Ok(ResolvedStmt::Let(name.clone(), resolve_expr(expr)?)),
        Stmt::Assign { name, expr, .. } => {
            Ok(ResolvedStmt::Assign(name.clone(), resolve_expr(expr)?))
        }
        Stmt::Expr { expr, .. } => Ok(ResolvedStmt::Expr(resolve_expr(expr)?)),
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => Ok(ResolvedStmt::If {
            condition: resolve_expr(condition)?,
            then_body: then_body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            else_body: else_body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::While {
            condition, body, ..
        } => Ok(ResolvedStmt::While {
            condition: resolve_expr(condition)?,
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::Return { expr, .. } => Ok(ResolvedStmt::Return(resolve_expr(expr)?)),
        Stmt::Function {
            name, params, body, ..
        } => Ok(ResolvedStmt::Function {
            name: name.clone(),
            params: params.clone(),
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
    }
}

fn resolve_expr(expr: &Expr) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
        Expr::Number { value, .. } => Ok(ResolvedExpr::Number(*value)),
        Expr::String { value, .. } => Ok(ResolvedExpr::String(value.clone())),
        Expr::Bool { value, .. } => Ok(ResolvedExpr::Bool(*value)),
        Expr::Null { .. } => Ok(ResolvedExpr::Null),
        Expr::Undefined { .. } => Ok(ResolvedExpr::Undefined),
        Expr::Ident { name, .. } => Ok(ResolvedExpr::Ident(name.clone())),
        Expr::Unary { op, expr, .. } => Ok(ResolvedExpr::Unary {
            op: *op,
            expr: Box::new(resolve_expr(expr)?),
        }),
        Expr::Binary {
            left, op, right, ..
        } => Ok(ResolvedExpr::Binary {
            left: Box::new(resolve_expr(left)?),
            op: *op,
            right: Box::new(resolve_expr(right)?),
        }),
        Expr::Call { callee, args, .. } => {
            let resolved_args = args
                .iter()
                .map(resolve_expr)
                .collect::<Result<Vec<_>, _>>()?;
            if let Some(builtin) = resolve_builtin_call(callee.as_ref(), args)? {
                let builtin_args = if matches!(builtin, BuiltinId::ReadStdinUtf8) {
                    Vec::new()
                } else {
                    resolved_args
                };
                Ok(ResolvedExpr::BuiltinCall {
                    builtin,
                    args: builtin_args,
                })
            } else {
                Ok(ResolvedExpr::Call {
                    callee: Box::new(resolve_expr(callee)?),
                    args: resolved_args,
                })
            }
        }
        Expr::Member {
            object, property, ..
        } => {
            let resolved_object = Box::new(resolve_expr(object)?);
            if property == "length" {
                Ok(ResolvedExpr::BuiltinProperty {
                    builtin: BuiltinPropertyId::Length,
                    object: resolved_object,
                })
            } else {
                Ok(ResolvedExpr::PropertyAccess {
                    object: resolved_object,
                    key: property.clone(),
                })
            }
        }
        Expr::Array { elements, .. } => Ok(ResolvedExpr::Array(
            elements
                .iter()
                .map(resolve_expr)
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Expr::Object { props, .. } => Ok(ResolvedExpr::Object(
            props
                .iter()
                .map(|(k, v)| Ok((k.clone(), resolve_expr(v)?)))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Expr::Index { object, index, .. } => Ok(ResolvedExpr::ComputedIndex {
            object: Box::new(resolve_expr(object)?),
            index: Box::new(resolve_expr(index)?),
        }),
    }
}

fn resolve_builtin_call(
    callee: &Expr,
    call_args: &[Expr],
) -> Result<Option<BuiltinId>, Diagnostic> {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return Ok(None);
    };

    let Expr::Ident {
        name: object_name, ..
    } = object.as_ref()
    else {
        if is_require_fs_read_file_sync_callee(callee) {
            validate_read_stdin_utf8_args(call_args, callee)?;
            return Ok(Some(BuiltinId::ReadStdinUtf8));
        }
        return Ok(None);
    };

    if object_name == "console" && property == "log" {
        return Ok(Some(BuiltinId::ConsoleLog));
    }

    Ok(None)
}

fn is_require_fs_read_file_sync_callee(callee: &Expr) -> bool {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return false;
    };
    if property != "readFileSync" {
        return false;
    }
    let Expr::Call {
        callee: require_callee,
        args,
        ..
    } = object.as_ref()
    else {
        return false;
    };
    let Expr::Ident {
        name: require_name, ..
    } = require_callee.as_ref()
    else {
        return false;
    };
    if require_name != "require" {
        return false;
    }
    matches!(args.as_slice(), [Expr::String { value, .. }] if value == "fs")
}

fn validate_read_stdin_utf8_args(args: &[Expr], callee: &Expr) -> Result<(), Diagnostic> {
    if args.len() != 2 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "require(\"fs\").readFileSync expects 2 arguments in this milestone, got {}",
                args.len()
            ),
            span: span_of_expr(callee),
        });
    }
    let fd_expr = &args[0];
    let encoding_expr = &args[1];

    match fd_expr {
        Expr::Number { value: 0, .. } => {}
        _ => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "require(\"fs\").readFileSync currently supports only fd 0 as first argument"
                        .to_owned(),
                span: span_of_expr(fd_expr),
            });
        }
    }

    match encoding_expr {
        Expr::String { value, .. } if value == "utf8" => Ok(()),
        _ => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "require(\"fs\").readFileSync currently supports only \"utf8\" encoding"
                .to_owned(),
            span: span_of_expr(encoding_expr),
        }),
    }
}

fn span_of_expr(expr: &Expr) -> Option<crate::Span> {
    match expr {
        Expr::Number { span, .. }
        | Expr::String { span, .. }
        | Expr::Bool { span, .. }
        | Expr::Null { span }
        | Expr::Undefined { span }
        | Expr::Ident { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Member { span, .. }
        | Expr::Call { span, .. }
        | Expr::Array { span, .. }
        | Expr::Object { span, .. }
        | Expr::Index { span, .. } => Some(*span),
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_builtins;
    use crate::DiagCode;
    use crate::ir::builtin::{BuiltinId, BuiltinPropertyId};
    use crate::ir::builtin_resolved::{ResolvedExpr, ResolvedStmt};

    #[test]
    fn console_log_resolves_to_builtin_call() {
        let program = crate::parse_program("console.log(1);").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[0] {
            ResolvedStmt::Expr(ResolvedExpr::BuiltinCall { builtin, args }) => {
                assert_eq!(*builtin, BuiltinId::ConsoleLog);
                assert_eq!(args.len(), 1);
            }
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }

    #[test]
    fn member_length_resolves_to_builtin_property() {
        let program = crate::parse_program("let a = [1]; let b = a.length;").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[1] {
            ResolvedStmt::Let(_, ResolvedExpr::BuiltinProperty { builtin, .. }) => {
                assert_eq!(*builtin, BuiltinPropertyId::Length);
            }
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }

    #[test]
    fn member_access_resolves_to_property_access() {
        let program = crate::parse_program("let o = { a: 1 }; let v = o.a;").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[1] {
            ResolvedStmt::Let(_, ResolvedExpr::PropertyAccess { key, .. }) => {
                assert_eq!(key, "a");
            }
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }

    #[test]
    fn index_access_resolves_to_computed_index() {
        let program = crate::parse_program("let o = { a: 1 }; let v = o[\"a\"];").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[1] {
            ResolvedStmt::Let(_, ResolvedExpr::ComputedIndex { .. }) => {}
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }

    #[test]
    fn read_file_sync_stdin_utf8_idiom_resolves_to_builtin_call() {
        let program =
            crate::parse_program("let s = require(\"fs\").readFileSync(0, \"utf8\");").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[0] {
            ResolvedStmt::Let(_, ResolvedExpr::BuiltinCall { builtin, args }) => {
                assert_eq!(*builtin, BuiltinId::ReadStdinUtf8);
                assert!(args.is_empty());
            }
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }

    #[test]
    fn read_file_sync_with_nonzero_fd_is_rejected() {
        let program = crate::parse_program("require(\"fs\").readFileSync(1, \"utf8\");").unwrap();
        let err = resolve_builtins(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("fd 0"));
    }

    #[test]
    fn read_file_sync_with_missing_encoding_is_rejected() {
        let program = crate::parse_program("require(\"fs\").readFileSync(0);").unwrap();
        let err = resolve_builtins(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::ArityMismatch);
    }

    #[test]
    fn read_file_sync_with_non_utf8_encoding_is_rejected() {
        let program = crate::parse_program("require(\"fs\").readFileSync(0, \"ascii\");").unwrap();
        let err = resolve_builtins(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("utf8"));
    }

    #[test]
    fn non_fs_read_file_sync_is_not_misclassified() {
        let program =
            crate::parse_program("let s = require(\"path\").readFileSync(0, \"utf8\");").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[0] {
            ResolvedStmt::Let(_, ResolvedExpr::Call { .. }) => {}
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }
}
