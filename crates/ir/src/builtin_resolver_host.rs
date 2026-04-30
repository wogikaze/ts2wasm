use super::*;

pub(super) fn resolve_test262_assert_stmt(expr: &Expr) -> Result<Option<ResolvedStmt>, Diagnostic> {
    let Expr::Call { callee, args, .. } = expr else {
        return Ok(None);
    };
    let Some(op) = test262_assert_failure_op(callee, args) else {
        return Ok(None);
    };
    let [actual, expected, ..] = args.as_slice() else {
        return Ok(None);
    };
    Ok(Some(ResolvedStmt::If {
        condition: ResolvedExpr::Binary {
            left: Box::new(resolve_expr(actual)?),
            op,
            right: Box::new(resolve_expr(expected)?),
        },
        then_body: vec![ResolvedStmt::Expr(ResolvedExpr::BuiltinCall {
            builtin: BuiltinId::ConsoleLog,
            args: vec![ResolvedExpr::String(
                "__TS2WASM_TEST262_ASSERT_FAIL__".to_owned(),
            )],
        })],
        else_body: vec![],
    }))
}

pub(super) fn test262_assert_failure_op(callee: &Expr, args: &[Expr]) -> Option<BinaryOp> {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return None;
    };
    if !matches!(object.as_ref(), Expr::Ident { name, .. } if name == "assert") {
        return None;
    }
    match (property.as_str(), args.len()) {
        ("sameValue", 2 | 3) => Some(BinaryOp::StrictNotEqual),
        ("notSameValue", 2 | 3) => Some(BinaryOp::StrictEqual),
        _ => None,
    }
}

pub(super) fn is_test262_assert_reference_error_probe(callee: &Expr, args: &[Expr]) -> bool {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return false;
    };
    if !matches!(object.as_ref(), Expr::Ident { name, .. } if name == "assert")
        || property != "throws"
    {
        return false;
    }
    let [
        Expr::Ident {
            name: error_name, ..
        },
        callback,
        ..,
    ] = args
    else {
        return false;
    };
    if error_name != "ReferenceError" {
        return false;
    }
    matches!(
        callback,
        Expr::FunctionExpr { params, body, .. }
            if params.is_empty()
                && matches!(
                    body.as_slice(),
                    [Stmt::Expr {
                        expr: Expr::Ident { .. },
                        ..
                    }]
                )
    )
}

/// Check if an expression is a `require("...")` call.
pub(super) fn is_require_call(callee: &Expr, args: &[Expr]) -> bool {
    let Expr::Ident { name, .. } = callee else {
        return false;
    };
    name == "require" && args.len() == 1 && matches!(&args[0], Expr::String { .. })
}

pub(super) fn resolve_builtin_call(
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
        if object_name == "BigInt" && matches!(property.as_str(), "asIntN" | "asUintN") {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-280: BigInt.asIntN/asUintN require literal bit width and BigInt value inputs in this builtin slice"
                        .to_owned(),
                span: span_of_expr(callee),
            });
        }
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
        if object_name == "Math" && property == "pow" {
            return Ok(Some(BuiltinId::MathPow));
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
    if let Some(builtin) = resolve_bun_file_text_builtin(object.as_ref(), property, call_args)? {
        return Ok(Some(builtin));
    }

    Ok(None)
}

pub(super) fn resolve_bun_file_text_builtin(
    object: &Expr,
    property: &str,
    call_args: &[Expr],
) -> Result<Option<BuiltinId>, Diagnostic> {
    if property != "text" {
        return Ok(None);
    }
    if !call_args.is_empty() {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "Bun.file(\"/dev/stdin\").text expects 0 arguments in this milestone, got {}",
                call_args.len()
            ),
            span: span_of_expr(object),
        });
    }
    let Expr::Call {
        callee: file_callee,
        args: file_args,
        ..
    } = object
    else {
        return Ok(None);
    };
    let Expr::Member {
        object: bun_object,
        property: file_property,
        ..
    } = file_callee.as_ref()
    else {
        return Ok(None);
    };
    let Expr::Ident {
        name: object_name, ..
    } = bun_object.as_ref()
    else {
        return Ok(None);
    };
    if object_name != "Bun" || file_property != "file" {
        return Ok(None);
    }
    match file_args.as_slice() {
        [Expr::String { value, .. }] if value == "/dev/stdin" => Ok(Some(BuiltinId::ReadStdinUtf8)),
        [arg] => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "Bun.file(...).text() currently supports only \"/dev/stdin\" stdin lowering"
                .to_owned(),
            span: span_of_expr(arg),
        }),
        _ => Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "Bun.file expects 1 argument in this milestone, got {}",
                file_args.len()
            ),
            span: span_of_expr(object),
        }),
    }
}

pub(super) fn resolve_require_module_builtin(
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

pub(super) fn validate_read_stdin_utf8_args(
    args: &[Expr],
    callee: &Expr,
) -> Result<(), Diagnostic> {
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

pub(super) fn span_of_expr(expr: &Expr) -> Option<Span> {
    match expr {
        Expr::Number { span, .. }
        | Expr::BigInt { span, .. }
        | Expr::String { span, .. }
        | Expr::Bool { span, .. }
        | Expr::Null { span }
        | Expr::This { span }
        | Expr::Undefined { span }
        | Expr::Await { span, .. }
        | Expr::Ident { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Member { span, .. }
        | Expr::OptionalMember { span, .. }
        | Expr::Call { span, .. }
        | Expr::OptionalCall { span, .. }
        | Expr::Assign { span, .. }
        | Expr::LogicalAssign { span, .. }
        | Expr::LogicalPropertyAssign { span, .. }
        | Expr::Array { span, .. }
        | Expr::Object { span, .. }
        | Expr::Index { span, .. }
        | Expr::OptionalIndex { span, .. }
        | Expr::New { span, .. }
        | Expr::TypeOf { span, .. }
        | Expr::InstanceOf { span, .. }
        | Expr::Ternary { span, .. }
        | Expr::ArrowFn { span, .. }
        | Expr::FunctionExpr { span, .. }
        | Expr::Spread { span, .. }
        | Expr::PropertyAssign { span, .. }
        | Expr::IndexAssign { span, .. } => Some(*span),
    }
}
