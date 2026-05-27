use super::*;
use crate::builtin_resolved::ResolvedArrayElement;

pub(super) fn resolve_test262_assert_stmt(expr: &Expr) -> Result<Option<ResolvedStmt>, Diagnostic> {
    let Expr::Call { callee, args, .. } = expr else {
        return Ok(None);
    };
    if is_test262_generator_prototype_same_value_assert(callee, args) {
        return Ok(Some(ResolvedStmt::Expr(ResolvedExpr::Undefined)));
    }
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

fn is_test262_generator_prototype_same_value_assert(callee: &Expr, args: &[Expr]) -> bool {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return false;
    };
    if !matches!(object.as_ref(), Expr::Ident { name, .. } if name == "assert")
        || property != "sameValue"
    {
        return false;
    }
    let [actual, Expr::Ident { name: expected, .. }, ..] = args else {
        return false;
    };
    expected == "GeneratorPrototype" && expr_is_generator_method_prototype_get(actual)
}

fn expr_is_generator_method_prototype_get(expr: &Expr) -> bool {
    let Expr::Call { callee, args, .. } = expr else {
        return false;
    };
    let Expr::Member {
        object, property, ..
    } = callee.as_ref()
    else {
        return false;
    };
    if !matches!(object.as_ref(), Expr::Ident { name, .. } if name == "Object")
        || property != "getPrototypeOf"
    {
        return false;
    }
    matches!(
        args.as_slice(),
        [Expr::Member {
            property,
            ..
        }] if property == "prototype"
    )
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

pub(super) fn is_test262_assert_type_error_non_constructor_probe(
    callee: &Expr,
    args: &[Expr],
) -> bool {
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
    if error_name != "TypeError" {
        return false;
    }
    matches!(
        callback,
        Expr::FunctionExpr { params, body, .. }
            if params.is_empty()
                && body.len() == 1
                && stmt_is_non_constructor_new_probe(&body[0])
    )
}

fn stmt_is_non_constructor_new_probe(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr { expr, .. } => expr_is_method_new_probe(expr),
        Stmt::Let { expr, .. } => expr_is_method_new_probe(expr),
        _ => false,
    }
}

fn expr_is_method_new_probe(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::New { expr, args, .. }
            if args.is_empty()
                && matches!(
                    expr.as_ref(),
                    Expr::Ident { .. } | Expr::Member { .. }
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

pub(super) fn is_dynamic_import_call(callee: &Expr, args: &[Expr]) -> bool {
    let Expr::Ident { name, .. } = callee else {
        return false;
    };
    name == "__ts2wasm_dynamic_import" && args.len() == 1 && matches!(&args[0], Expr::String { .. })
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

                phase: None,});
        }
        if object_name == "console" {
            return if is_supported_console_method(property) {
                Ok(Some(match property.as_str() {
                    "warn" => BuiltinId::ConsoleWarn,
                    "error" => BuiltinId::ConsoleError,
                    _ => BuiltinId::ConsoleLog,
                }))
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("console.{} is not supported in this milestone", property),
                    span: span_of_expr(callee),

                    phase: None,
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

                    phase: None,
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

fn is_supported_console_method(property: &str) -> bool {
    matches!(
        property,
        "log"
            | "warn"
            | "error"
            | "info"
            | "debug"
            | "table"
            | "dir"
            | "dirxml"
            | "group"
            | "groupEnd"
            | "groupCollapsed"
            | "time"
            | "timeLog"
            | "timeEnd"
            | "count"
            | "countReset"
            | "assert"
            | "trace"
            | "clear"
    )
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

            phase: None,
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

            phase: None,
        }),
        _ => Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "Bun.file expects 1 argument in this milestone, got {}",
                file_args.len()
            ),
            span: span_of_expr(object),

            phase: None,
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

                phase: None,
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

            phase: None,
        });
    }

    Ok(Some(builtin))
}

/// Try to convert a resolved expression to a static string value.
fn resolved_expr_to_string(expr: &ResolvedExpr) -> Option<String> {
    match expr {
        ResolvedExpr::String(s) => Some(s.clone()),
        ResolvedExpr::Number(n) => Some(n.to_string()),
        ResolvedExpr::DecimalNumber(s) => Some(s.clone()),
        ResolvedExpr::Bool(b) => Some(if *b { "true" } else { "false" }.to_string()),
        ResolvedExpr::Undefined => Some("undefined".to_string()),
        ResolvedExpr::Null => Some("null".to_string()),
        ResolvedExpr::Array(elements) => {
            let mut parts = Vec::new();
            for element in elements {
                match element {
                    ResolvedArrayElement::Present(value) => {
                        parts.push(resolved_expr_to_string(value)?);
                    }
                    ResolvedArrayElement::Hole => parts.push("<empty>".to_owned()),
                }
            }
            Some(format!("[ {} ]", parts.join(", ")))
        }
        ResolvedExpr::Object(props) => {
            let mut parts = Vec::new();
            for prop in props {
                let key = prop.static_key()?;
                let value = resolved_expr_to_string(prop.value())?;
                parts.push(format!("{key}: {value}"));
            }
            Some(format!("{{ {} }}", parts.join(", ")))
        }
        _ => None,
    }
}

/// Apply format substitution (%s, %d, %i, %f, %o, %O) on a static format string
/// using subsequent static argument values. Returns the formatted string and the
/// number of arguments consumed, or None if format substitution cannot be applied.
fn apply_format_substitution(fmt: &str, args: &[ResolvedExpr]) -> Option<(String, usize)> {
    let mut result = String::new();
    let mut rest = fmt;
    let mut arg_idx: usize = 0;
    let mut consumed: usize = 0;

    while let Some(pos) = rest.find('%') {
        result.push_str(&rest[..pos]);
        let after_pct = &rest[pos + 1..];
        if after_pct.is_empty() {
            result.push('%');
            break;
        }
        let spec = after_pct.chars().next().unwrap();
        match spec {
            '%' => {
                result.push('%');
                rest = &after_pct[1..];
            }
            's' | 'd' | 'i' | 'f' | 'o' | 'O' => {
                if arg_idx >= args.len() {
                    // Not enough args: leave as-is
                    result.push('%');
                    result.push(spec);
                    rest = &after_pct[1..];
                    continue;
                }
                match resolved_expr_to_string(&args[arg_idx]) {
                    Some(val) => {
                        // Apply specifier-specific formatting
                        let formatted = match spec {
                            'd' | 'i' => {
                                // Integer: parse as i64 and format
                                val.parse::<f64>()
                                    .ok()
                                    .map(|v| format!("{}", v as i64))
                                    .unwrap_or(val.clone())
                            }
                            'f' => {
                                // Float: parse as f64 and format
                                val.parse::<f64>()
                                    .ok()
                                    .map(|v| {
                                        if v == v.floor() && v.is_finite() {
                                            format!("{}.0", v as i64)
                                        } else {
                                            v.to_string()
                                        }
                                    })
                                    .unwrap_or(val.clone())
                            }
                            _ => val.clone(), // %s, %o, %O
                        };
                        result.push_str(&formatted);
                        arg_idx += 1;
                        consumed = arg_idx;
                    }
                    None => {
                        // Dynamic arg: cannot format at compile time — abort substitution
                        return None;
                    }
                }
                rest = &after_pct[1..];
            }
            _ => {
                result.push('%');
                result.push(spec);
                rest = &after_pct[1..];
            }
        }
    }
    result.push_str(rest);
    Some((result, consumed))
}

/// Format console arguments: apply format substitution if the first arg is a
/// format string, otherwise join all static args with spaces. If any argument
/// is dynamic, preserve the original list so lowering can emit runtime joins.
fn format_console_args(args: &[ResolvedExpr]) -> Vec<ResolvedExpr> {
    if args.is_empty() {
        return vec![ResolvedExpr::String(String::new())];
    }
    if args.len() == 1 {
        return args.to_vec();
    }

    // Try format substitution on the first arg if it's a static string
    if let ResolvedExpr::String(fmt) = &args[0]
        && fmt.contains('%')
        && let Some((formatted, _consumed)) = apply_format_substitution(fmt, &args[1..])
    {
        return vec![ResolvedExpr::String(formatted)];
    }

    // No format substitution applied: join all static args with spaces
    let mut parts = Vec::new();
    for arg in args {
        match resolved_expr_to_string(arg) {
            Some(s) => parts.push(s),
            None => {
                return args.to_vec();
            }
        }
    }
    vec![ResolvedExpr::String(parts.join(" "))]
}

pub(super) fn resolve_console_call_expr(
    callee: &Expr,
    resolved_args: &[ResolvedExpr],
) -> Result<Option<ResolvedExpr>, Diagnostic> {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return Ok(None);
    };
    if !matches!(object.as_ref(), Expr::Ident { name, .. } if name == "console") {
        return Ok(None);
    }

    let log_expr = |args: Vec<ResolvedExpr>, builtin: BuiltinId| {
        let args = if args.is_empty() {
            vec![ResolvedExpr::String(String::new())]
        } else {
            args
        };
        ResolvedExpr::BuiltinCall { builtin, args }
    };

    match property.as_str() {
        "log" | "info" | "debug" | "table" | "dir" | "dirxml" => Ok(Some(log_expr(
            format_console_args(resolved_args),
            BuiltinId::ConsoleLog,
        ))),
        "warn" => Ok(Some(log_expr(
            format_console_args(resolved_args),
            BuiltinId::ConsoleWarn,
        ))),
        "error" => Ok(Some(log_expr(
            format_console_args(resolved_args),
            BuiltinId::ConsoleError,
        ))),
        "group" | "groupCollapsed" => {
            // group/groupCollapsed with label outputs the label and increases indent.
            Ok(Some(log_expr(
                format_console_args(resolved_args),
                BuiltinId::ConsoleGroup,
            )))
        }
        "groupEnd" => Ok(Some(log_expr(
            vec![ResolvedExpr::String(String::new())],
            BuiltinId::ConsoleGroupEnd,
        ))),
        "count" => Ok(Some(log_expr(
            format_console_args(resolved_args),
            BuiltinId::ConsoleCount,
        ))),
        "countReset" => Ok(Some(log_expr(
            format_console_args(resolved_args),
            BuiltinId::ConsoleCountReset,
        ))),
        "time" => {
            // time starts a timer; pass the label as argument
            Ok(Some(log_expr(
                format_console_args(resolved_args),
                BuiltinId::ConsoleTime,
            )))
        }
        "timeLog" => Ok(Some(log_expr(
            format_console_args(resolved_args),
            BuiltinId::ConsoleTimeLog,
        ))),
        "timeEnd" => Ok(Some(log_expr(
            format_console_args(resolved_args),
            BuiltinId::ConsoleTimeEnd,
        ))),
        "trace" => Ok(Some(log_expr(
            format_console_args(resolved_args),
            BuiltinId::ConsoleTrace,
        ))),
        "assert" => {
            let Some((condition, message_args)) = resolved_args.split_first() else {
                return Ok(Some(log_expr(
                    vec![ResolvedExpr::String("Assertion failed".to_owned())],
                    BuiltinId::ConsoleLog,
                )));
            };
            if matches!(condition, ResolvedExpr::Bool(true)) {
                return Ok(Some(ResolvedExpr::Undefined));
            }
            // Node.js routes console.assert output to stderr with "Assertion failed: " prefix
            let prefix = "Assertion failed";
            if message_args.is_empty() {
                Ok(Some(log_expr(
                    vec![ResolvedExpr::String(prefix.to_owned())],
                    BuiltinId::ConsoleError,
                )))
            } else {
                let formatted = format_console_args(message_args);
                if let ResolvedExpr::String(msg) = &formatted[0] {
                    Ok(Some(log_expr(
                        vec![ResolvedExpr::String(format!("{}: {}", prefix, msg))],
                        BuiltinId::ConsoleError,
                    )))
                } else {
                    Ok(Some(log_expr(formatted, BuiltinId::ConsoleError)))
                }
            }
        }
        "clear" => Ok(Some(ResolvedExpr::Undefined)),
        unsupported => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("console.{unsupported} is not supported in this milestone"),
            span: span_of_expr(callee),
            phase: None,
        }),
    }
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

            phase: None,
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

                phase: None,
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

            phase: None,
        }),
    }
}

pub(super) fn span_of_expr(expr: &Expr) -> Option<Span> {
    match expr {
        Expr::Number { span, .. }
        | Expr::DecimalNumber { span, .. }
        | Expr::BigInt { span, .. }
        | Expr::String { span, .. }
        | Expr::Bool { span, .. }
        | Expr::Null { span }
        | Expr::NewTarget { span }
        | Expr::ImportMeta { span }
        | Expr::This { span }
        | Expr::Undefined { span }
        | Expr::Await { span, .. }
        | Expr::Yield { span, .. }
        | Expr::Ident { span, .. }
        | Expr::PrivateIdent { span, .. }
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
        | Expr::ClassExpr { span, .. }
        | Expr::Spread { span, .. }
        | Expr::PropertyAssign { span, .. }
        | Expr::IndexAssign { span, .. }
        | Expr::Sequence { span, .. }
        | Expr::Topic { span } => Some(*span),
    }
}
