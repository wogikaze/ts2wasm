use ts2wasm_frontend::{DiagCode, Diagnostic, Expr, Span, Stmt, UnaryOp};

use super::builtin::BuiltinId;
use super::builtin::BuiltinPropertyId;
use super::builtin_resolved::{ClassMethod, ResolvedExpr, ResolvedStmt};

pub fn resolve_builtins(program: &[Stmt]) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    program.iter().map(resolve_stmt).collect()
}

fn resolve_stmt(stmt: &Stmt) -> Result<ResolvedStmt, Diagnostic> {
    match stmt {
        Stmt::Let { name, expr, .. } => Ok(ResolvedStmt::Let(name.clone(), resolve_expr(expr)?)),
        Stmt::Assign { name, expr, .. } => {
            Ok(ResolvedStmt::Assign(name.clone(), resolve_expr(expr)?))
        }
        Stmt::Expr { expr, .. } => {
            // Detect exports.X = ... and module.exports = ... patterns
            if let Expr::PropertyAssign {
                object,
                property,
                value,
                ..
            } = expr
            {
                if let Expr::Ident { name, .. } = object.as_ref() {
                    if name == "exports" {
                        return Ok(ResolvedStmt::Export {
                            name: property.clone(),
                            expr: Box::new(resolve_expr(value)?),
                        });
                    }
                    if name == "module" && property == "exports" {
                        return Ok(ResolvedStmt::ModuleExportsAssign {
                            expr: Box::new(resolve_expr(value)?),
                        });
                    }
                }
            }
            Ok(ResolvedStmt::Expr(resolve_expr(expr)?))
        }
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
        } => {
            let resolved_params = params
                .iter()
                .map(|(param_name, default)| {
                    Ok((
                        param_name.clone(),
                        default.as_ref().map(|d| resolve_expr(d)).transpose()?,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ResolvedStmt::Function {
                name: name.clone(),
                params: resolved_params,
                body: body
                    .iter()
                    .map(resolve_stmt)
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
        Stmt::ClassDecl {
            name,
            extends,
            body,
            ..
        } => {
            // Parse extends (must be an identifier for now)
            let extends_name = match extends {
                Some(ext_expr) => match ext_expr.as_ref() {
                    Expr::Ident { name: parent, .. } => Some(parent.clone()),
                    _ => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "only simple inheritance (extends ClassName) is supported"
                                .to_owned(),
                            span: None,
                        });
                    }
                },
                None => None,
            };

            // Parse class body to extract constructor and methods
            let mut constructor = None;
            let mut methods = Vec::new();
            let mut statics = Vec::new();

            for stmt in body {
                match stmt {
                    // Constructor method (identified by being a Function named "constructor")
                    Stmt::Function {
                        name: method_name,
                        params,
                        body: method_body,
                        ..
                    } if method_name == "constructor" => {
                        if constructor.is_some() {
                            return Err(Diagnostic {
                                code: DiagCode::DuplicateFunction,
                                message: "duplicate constructor definition".to_owned(),
                                span: None,
                            });
                        }
                        let resolved_params = params
                            .iter()
                            .map(|(param_name, default)| {
                                Ok((
                                    param_name.clone(),
                                    default.as_ref().map(|d| resolve_expr(d)).transpose()?,
                                ))
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        let resolved_body = method_body.iter().map(resolve_stmt).collect::<Result<
                            Vec<_>,
                            _,
                        >>(
                        )?;
                        constructor = Some((resolved_params, resolved_body));
                    }
                    // Regular methods
                    Stmt::Function {
                        name: method_name,
                        params,
                        body: method_body,
                        ..
                    } => {
                        let resolved_params = params
                            .iter()
                            .map(|(param_name, default)| {
                                Ok((
                                    param_name.clone(),
                                    default.as_ref().map(|d| resolve_expr(d)).transpose()?,
                                ))
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        let resolved_body = method_body.iter().map(resolve_stmt).collect::<Result<
                            Vec<_>,
                            _,
                        >>(
                        )?;
                        if let Some(stripped) = method_name.strip_prefix("static::") {
                            statics.push((stripped.to_owned(), ResolvedExpr::Undefined));
                            methods.push(ClassMethod {
                                name: method_name.clone(),
                                params: resolved_params,
                                body: resolved_body,
                            });
                        } else {
                            methods.push(ClassMethod {
                                name: method_name.clone(),
                                params: resolved_params,
                                body: resolved_body,
                            });
                        }
                    }
                    // Static members (for now, we'll just skip them - not yet supported)
                    _ => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "class body may only contain methods and constructors"
                                .to_owned(),
                            span: None,
                        });
                    }
                }
            }

            Ok(ResolvedStmt::ClassDecl {
                name: name.clone(),
                extends: extends_name,
                constructor,
                methods,
                statics,
            })
        }
        Stmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
            ..
        } => Ok(ResolvedStmt::TryCatch {
            try_block: try_block
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            catch_param: catch_param.clone(),
            catch_block: catch_block
                .as_ref()
                .map(|b| b.iter().map(resolve_stmt).collect::<Result<Vec<_>, _>>())
                .transpose()?,
            finally_block: finally_block
                .as_ref()
                .map(|b| b.iter().map(resolve_stmt).collect::<Result<Vec<_>, _>>())
                .transpose()?,
        }),
        Stmt::Throw { expr, .. } => Ok(ResolvedStmt::Throw(resolve_expr(expr)?)),
        Stmt::Switch { expr, cases, .. } => {
            let resolved_cases = cases
                .iter()
                .map(|(cond, body)| {
                    Ok((
                        cond.as_ref().map(resolve_expr).transpose()?,
                        body.iter()
                            .map(resolve_stmt)
                            .collect::<Result<Vec<_>, _>>()?,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ResolvedStmt::Switch {
                expr: resolve_expr(expr)?,
                cases: resolved_cases,
            })
        }
        Stmt::DoWhile {
            body, condition, ..
        } => Ok(ResolvedStmt::DoWhile {
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            condition: resolve_expr(condition)?,
        }),
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            let resolved_init = if let Some(i) = init {
                Some(Box::new(resolve_stmt(i)?))
            } else {
                None
            };
            let resolved_condition = if let Some(cond) = condition {
                Some(resolve_expr(cond)?)
            } else {
                None
            };
            let resolved_update = if let Some(upd) = update {
                Some(resolve_expr(upd)?)
            } else {
                None
            };
            Ok(ResolvedStmt::For {
                init: resolved_init,
                condition: resolved_condition,
                update: resolved_update,
                body: body
                    .iter()
                    .map(resolve_stmt)
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
        Stmt::ForIn {
            var, iter, body, ..
        } => Ok(ResolvedStmt::ForIn {
            var: var.clone(),
            iter: resolve_expr(iter)?,
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::ForOf {
            var, iter, body, ..
        } => Ok(ResolvedStmt::ForOf {
            var: var.clone(),
            iter: resolve_expr(iter)?,
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::Break { .. } => Ok(ResolvedStmt::Break),
        Stmt::Continue { .. } => Ok(ResolvedStmt::Continue),
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
        Expr::Call { callee, args, .. } if is_require_call(callee, args) => {
            if let [
                Expr::String {
                    value: specifier, ..
                },
            ] = args.as_slice()
            {
                Ok(ResolvedExpr::ModuleLoad {
                    specifier: specifier.clone(),
                })
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "require() expects a string literal argument".to_owned(),
                    span: None,
                })
            }
        }
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
            } else if let Expr::Member {
                object, property, ..
            } = callee.as_ref()
            {
                Ok(ResolvedExpr::MethodCall {
                    object: Box::new(resolve_expr(object)?),
                    method: property.clone(),
                    args: resolved_args,
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
            if let Expr::Ident { name, .. } = object.as_ref() {
                if name == "process" {
                    return match property.as_str() {
                        "argv" => Ok(ResolvedExpr::BuiltinCall {
                            builtin: BuiltinId::ProcessArgv,
                            args: Vec::new(),
                        }),
                        "env" => Ok(ResolvedExpr::BuiltinCall {
                            builtin: BuiltinId::ProcessEnv,
                            args: Vec::new(),
                        }),
                        _ => Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "process.{} is not supported in this milestone",
                                property
                            ),
                            span: span_of_expr(expr),
                        }),
                    };
                }
            }

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
        Expr::Index { object, index, .. } => {
            // For string literal keys, use PropertyAccess (object property semantics)
            // For other expressions, use ComputedIndex (array indexing semantics)
            if let Expr::String { value, .. } = index.as_ref() {
                Ok(ResolvedExpr::PropertyAccess {
                    object: Box::new(resolve_expr(object)?),
                    key: value.clone(),
                })
            } else {
                Ok(ResolvedExpr::ComputedIndex {
                    object: Box::new(resolve_expr(object)?),
                    index: Box::new(resolve_expr(index)?),
                })
            }
        }
        Expr::New {
            expr: new_expr,
            args,
            ..
        } => {
            // Extract class name from identifier
            if let Expr::Ident {
                name: class_name, ..
            } = new_expr.as_ref()
            {
                let resolved_args = args
                    .iter()
                    .map(resolve_expr)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ResolvedExpr::New {
                    class_name: class_name.clone(),
                    args: resolved_args,
                })
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "only new ClassName(...) is supported".to_owned(),
                    span: None,
                })
            }
        }
        Expr::PropertyAssign {
            object,
            property,
            value,
            ..
        } => Ok(ResolvedExpr::PropertyAssign {
            object: Box::new(resolve_expr(object)?),
            key: property.clone(),
            value: Box::new(resolve_expr(value)?),
        }),
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => Ok(ResolvedExpr::PropertyAssignDynamic {
            object: Box::new(resolve_expr(object)?),
            key: Box::new(resolve_expr(index)?),
            value: Box::new(resolve_expr(value)?),
        }),
        // New expression types (not yet supported in resolver
        Expr::InstanceOf { span, .. }
        | Expr::Ternary { span, .. }
        | Expr::ArrowFn { span, .. }
        | Expr::Spread { span, .. } => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "expression type not yet supported in builtin resolver".to_owned(),
            span: Some(*span),
        }),
        Expr::TypeOf { expr, .. } => Ok(ResolvedExpr::Unary {
            op: UnaryOp::TypeOf,
            expr: Box::new(resolve_expr(expr)?),
        }),
    }
}

/// Check if an expression is a `require("...")` call.
fn is_require_call(callee: &Expr, args: &[Expr]) -> bool {
    let Expr::Ident { name, .. } = callee else {
        return false;
    };
    name == "require" && args.len() == 1 && matches!(&args[0], Expr::String { .. })
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

    if let Expr::Ident {
        name: object_name, ..
    } = object.as_ref()
    {
        if object_name == "console" {
            return if property == "log" {
                Ok(Some(BuiltinId::ConsoleLog))
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("console.{} is not supported in this milestone", property),
                    span: span_of_expr(callee),
                })
            };
        }
        if object_name == "process" {
            return if property == "exit" {
                Ok(Some(BuiltinId::ProcessExit))
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("process.{} is not supported in this milestone", property),
                    span: span_of_expr(callee),
                })
            };
        }
    }

    if let Some(builtin) = resolve_require_module_builtin(object.as_ref(), property, call_args)? {
        return Ok(Some(builtin));
    }

    Ok(None)
}

fn resolve_require_module_builtin(
    object: &Expr,
    property: &str,
    call_args: &[Expr],
) -> Result<Option<BuiltinId>, Diagnostic> {
    let Expr::Call {
        callee: require_callee,
        args: require_args,
        ..
    } = object
    else {
        return Ok(None);
    };
    let Expr::Ident {
        name: require_name, ..
    } = require_callee.as_ref()
    else {
        return Ok(None);
    };
    if require_name != "require" {
        return Ok(None);
    }
    let module_name = match require_args.as_slice() {
        [Expr::String { value, .. }] => value.as_str(),
        _ => return Ok(None),
    };

    let builtin = match (module_name, property) {
        ("fs", "readFileSync") => {
            if matches!(call_args.first(), Some(Expr::Number { .. })) {
                validate_read_stdin_utf8_args(call_args, object)?;
                BuiltinId::ReadStdinUtf8
            } else {
                BuiltinId::FsReadFileSync
            }
        }
        ("fs", "writeFileSync") => BuiltinId::FsWriteFileSync,
        ("fs", "appendFileSync") => BuiltinId::FsAppendFileSync,
        ("path", "join") => BuiltinId::PathJoin,
        ("path", "resolve") => BuiltinId::PathResolve,
        ("path", "basename") => BuiltinId::PathBasename,
        ("path", "dirname") => BuiltinId::PathDirname,
        ("crypto", "randomBytes") => BuiltinId::CryptoRandomBytes,
        ("fs", unsupported)
        | ("path", unsupported)
        | ("crypto", unsupported)
        | ("util", unsupported) => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "require(\"{}\").{} is not supported in this milestone",
                    module_name, unsupported
                ),
                span: span_of_expr(object),
            });
        }
        _ => return Ok(None),
    };

    if !matches!(builtin, BuiltinId::ReadStdinUtf8) && call_args.len() != builtin.expected_arity() {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "builtin call expects {} arguments, got {}",
                builtin.expected_arity(),
                call_args.len()
            ),
            span: span_of_expr(object),
        });
    }

    Ok(Some(builtin))
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

fn span_of_expr(expr: &Expr) -> Option<Span> {
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
        | Expr::Index { span, .. }
        | Expr::New { span, .. }
        | Expr::TypeOf { span, .. }
        | Expr::InstanceOf { span, .. }
        | Expr::Ternary { span, .. }
        | Expr::ArrowFn { span, .. }
        | Expr::Spread { span, .. }
        | Expr::PropertyAssign { span, .. }
        | Expr::IndexAssign { span, .. } => Some(*span),
    }
}
