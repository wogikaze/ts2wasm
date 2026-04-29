pub fn lower_program(program: &[ResolvedStmt]) -> Result<LoweredProgram, Diagnostic> {
    let function_ids = collect_function_ids(program)?;
    let class_parents = collect_class_parents(program);
    let mut next_func_id = function_ids.len();
    let mut functions_by_id = vec![None; function_ids.len()];
    let mut generated_functions = Vec::new();

    for stmt in program {
        match stmt {
            ResolvedStmt::Function { name, params, body } => {
                let func_id = function_ids[name];
                let lowered = lower_function(
                    func_id,
                    params,
                    body,
                    &function_ids,
                    class_parents.clone(),
                    LowerFunctionOptions {
                        current_class: None,
                        in_constructor: false,
                        next_func_id,
                    },
                )?;
                next_func_id = lowered.next_func_id;
                functions_by_id[func_id.0] = Some(lowered.function);
                generated_functions.extend(lowered.generated_functions);
            }
            ResolvedStmt::ClassDecl {
                name,
                constructor,
                methods,
                ..
            } => {
                let ctor_key = class_constructor_key(name);
                let ctor_id = function_ids[&ctor_key];

                let (ctor_params, ctor_body) = if let Some((params, body)) = constructor {
                    (params.clone(), body.clone())
                } else {
                    (Vec::new(), Vec::new())
                };

                let mut ctor_params_with_this: Vec<ResolvedParam> =
                    vec![("this".to_owned(), None, false)];
                ctor_params_with_this.extend(ctor_params.clone());

                let lowered = lower_function(
                    ctor_id,
                    &ctor_params_with_this,
                    &ctor_body,
                    &function_ids,
                    class_parents.clone(),
                    LowerFunctionOptions {
                        current_class: Some(name),
                        in_constructor: true,
                        next_func_id,
                    },
                )?;
                next_func_id = lowered.next_func_id;
                functions_by_id[ctor_id.0] = Some(lowered.function);
                generated_functions.extend(lowered.generated_functions);

                for method in methods {
                    let method_key = class_method_key(name, &method.name);
                    let method_id = function_ids[&method_key];
                    let method_params_with_this: Vec<(String, Option<ResolvedExpr>, bool)> =
                        if method.name.starts_with("static::") {
                            method.params.clone()
                        } else {
                            let mut params = vec![("this".to_owned(), None, false)];
                            params.extend(method.params.clone());
                            params
                        };
                    let lowered = lower_function(
                        method_id,
                        &method_params_with_this,
                        &method.body,
                        &function_ids,
                        class_parents.clone(),
                        LowerFunctionOptions {
                            current_class: Some(name),
                            in_constructor: false,
                            next_func_id,
                        },
                    )?;
                    next_func_id = lowered.next_func_id;
                    functions_by_id[method_id.0] = Some(lowered.function);
                    generated_functions.extend(lowered.generated_functions);
                }
            }
            _ => {}
        }
    }

    let mut resolver = Resolver::new(&function_ids, class_parents.clone(), next_func_id);
    let mut top_level_statements = Vec::new();
    for stmt in program {
        match stmt {
            ResolvedStmt::Function { .. } | ResolvedStmt::ClassDecl { .. } => {}
            _ => top_level_statements.push(resolver.lower_stmt(stmt)?),
        }
    }
    generated_functions.extend(resolver.generated_functions);

    let mut functions = functions_by_id
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "function id allocation left an unfilled function slot".to_owned(),
            span: None,
        })?;
    generated_functions.sort_by_key(|function| function.id.0);
    functions.extend(generated_functions);

    Ok(LoweredProgram {
        top_level_statements,
        top_level_locals: resolver.locals,
        functions,
        modules: resolver.modules,
    })
}

struct FunctionLowering {
    function: LoweredFunction,
    generated_functions: Vec<LoweredFunction>,
    next_func_id: usize,
}

fn collect_function_ids(program: &[ResolvedStmt]) -> Result<HashMap<String, FuncId>, Diagnostic> {
    let mut function_ids = HashMap::new();
    let mut next_func_id = 0;

    for stmt in program {
        match stmt {
            ResolvedStmt::Function { name, .. } => {
                if function_ids.contains_key(name.as_str()) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateFunction,
                        message: format!("duplicate function definition: `{name}`"),
                        span: None,
                    });
                }
                function_ids.insert(name.clone(), FuncId(next_func_id));
                next_func_id += 1;
            }
            ResolvedStmt::ClassDecl {
                name,
                constructor,
                methods,
                ..
            } => {
                let ctor_key = class_constructor_key(name);
                if function_ids.contains_key(&ctor_key) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateFunction,
                        message: format!("duplicate constructor definition: `{name}`"),
                        span: None,
                    });
                }
                function_ids.insert(ctor_key, FuncId(next_func_id));
                next_func_id += 1;

                if constructor.is_some() {
                    // constructor body is lowered into the constructor function ID above.
                }

                for method in methods {
                    let method_key = class_method_key(name, &method.name);
                    if function_ids.contains_key(&method_key) {
                        return Err(Diagnostic {
                            code: DiagCode::DuplicateFunction,
                            message: format!(
                                "duplicate method definition: `{}.{}`",
                                name, method.name
                            ),
                            span: None,
                        });
                    }
                    function_ids.insert(method_key, FuncId(next_func_id));
                    next_func_id += 1;
                }
            }
            _ => {}
        }
    }

    Ok(function_ids)
}

fn class_constructor_key(class_name: &str) -> String {
    format!("class::{class_name}::constructor")
}

fn class_method_key(class_name: &str, method_name: &str) -> String {
    format!("class::{class_name}::{method_name}")
}

fn collect_class_parents(program: &[ResolvedStmt]) -> HashMap<String, Option<String>> {
    let mut parents = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::ClassDecl { name, extends, .. } = stmt {
            parents.insert(name.clone(), extends.clone());
        }
    }
    parents
}

struct LowerFunctionOptions<'a> {
    current_class: Option<&'a str>,
    in_constructor: bool,
    next_func_id: usize,
}

fn lower_function(
    id: FuncId,
    params: &[ResolvedParam],
    body: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
    class_parents: HashMap<String, Option<String>>,
    options: LowerFunctionOptions<'_>,
) -> Result<FunctionLowering, Diagnostic> {
    let (mut resolver, param_ids) = Resolver::with_params(
        function_ids,
        params
            .iter()
            .map(|(name, _, _)| name.clone())
            .collect::<Vec<_>>()
            .as_slice(),
        class_parents,
        options.current_class,
        options.in_constructor,
        options.next_func_id,
    )?;

    let rest_param_index = params.iter().position(|(_, _, is_rest)| *is_rest);

    // Insert default parameter assignments at the start of the body.
    let mut body_with_defaults = Vec::new();
    for (param_name, default_expr, is_rest) in params {
        if *is_rest {
            // Rest parameters are populated by call lowering/emission.
            continue;
        } else if let Some(default) = default_expr {
            let param_local = resolver.resolve_local(param_name)?;
            let lowered_default = resolver.lower_expr(default)?;
            // Generate: if (param === undefined) { param = default; }
            body_with_defaults.push(LoweredStmt::If {
                condition: LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(param_local)),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined),
                },
                then_body: vec![LoweredStmt::Assign(param_local, lowered_default)],
                else_body: vec![],
            });
        }
    }
    body_with_defaults.extend(resolver.lower_block(body)?);

    let min_required = params
        .iter()
        .filter(|(_, default, is_rest)| default.is_none() && !*is_rest)
        .count();
    Ok(FunctionLowering {
        function: LoweredFunction {
            id,
            params: param_ids,
            min_required_params: min_required,
            rest_param_index,
            locals: resolver.locals,
            body: body_with_defaults,
        },
        generated_functions: resolver.generated_functions,
        next_func_id: resolver.next_func_id,
    })
}

fn lower_binary_op(op: BinaryOp) -> Result<LoweredBinaryOp, Diagnostic> {
    match op {
        BinaryOp::Add => Ok(LoweredBinaryOp::Add),
        BinaryOp::Subtract => Ok(LoweredBinaryOp::Subtract),
        BinaryOp::Multiply => Ok(LoweredBinaryOp::Multiply),
        BinaryOp::Divide => Ok(LoweredBinaryOp::Divide),
        BinaryOp::Modulo => Ok(LoweredBinaryOp::Modulo),
        BinaryOp::Less => Ok(LoweredBinaryOp::Less),
        BinaryOp::LessEqual => Ok(LoweredBinaryOp::LessEqual),
        BinaryOp::Greater => Ok(LoweredBinaryOp::Greater),
        BinaryOp::GreaterEqual => Ok(LoweredBinaryOp::GreaterEqual),
        BinaryOp::StrictEqual => Ok(LoweredBinaryOp::StrictEqual),
        BinaryOp::EqualEqual => Ok(LoweredBinaryOp::EqualEqual),
        BinaryOp::BangEqual => Ok(LoweredBinaryOp::BangEqual),
        BinaryOp::StrictNotEqual => Ok(LoweredBinaryOp::StrictNotEqual),
        BinaryOp::And => Ok(LoweredBinaryOp::And),
        BinaryOp::Or => Ok(LoweredBinaryOp::Or),
        BinaryOp::Power
        | BinaryOp::BitwiseAnd
        | BinaryOp::BitwiseOr
        | BinaryOp::BitwiseXor
        | BinaryOp::LeftShift
        | BinaryOp::RightShift
        | BinaryOp::UnsignedRightShift
        | BinaryOp::In
        | BinaryOp::InstanceOf => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("binary operator {:?} not yet supported", op),
            span: None,
        }),
    }
}

fn lower_logical_assign_op(op: LogicalAssignOp) -> LoweredLogicalAssignOp {
    match op {
        LogicalAssignOp::And => LoweredLogicalAssignOp::And,
        LogicalAssignOp::Or => LoweredLogicalAssignOp::Or,
        LogicalAssignOp::Nullish => LoweredLogicalAssignOp::Nullish,
    }
}

fn resolve_method_to_runtime_fn(object: &ResolvedExpr, method: &str) -> Option<String> {
    if let ResolvedExpr::Ident(name) = object {
        if name == "Math" {
            return match method {
                "floor" => Some("MathFloor".to_owned()),
                "ceil" => Some("MathCeil".to_owned()),
                "round" => Some("MathRound".to_owned()),
                "abs" => Some("MathAbs".to_owned()),
                "max" => Some("MathMax".to_owned()),
                "min" => Some("MathMin".to_owned()),
                "random" => Some("MathRandom".to_owned()),
                _ => None,
            };
        }
        if name == "JSON" {
            return match method {
                "stringify" => Some("JsonStringify".to_owned()),
                "parse" => Some("JsonParse".to_owned()),
                _ => None,
            };
        }
        if name == "Object" {
            return match method {
                "keys" => Some("ObjectKeys".to_owned()),
                "values" => Some("ObjectValues".to_owned()),
                "entries" => Some("ObjectEntries".to_owned()),
                "getPrototypeOf" => Some("ObjectGetPrototypeOf".to_owned()),
                "setPrototypeOf" => Some("ObjectSetPrototypeOf".to_owned()),
                _ => None,
            };
        }
        if name == "String" {
            return match method {
                "fromCharCode" => Some("StringFromCharCode".to_owned()),
                _ => None,
            };
        }
    }
    match method {
        "charAt" => Some("StringCharAt".to_owned()),
        "substring" => Some("StringSubstring".to_owned()),
        "slice" => Some("StringSlice".to_owned()),
        "indexOf" => Some("StringIndexOf".to_owned()),
        "split" => Some("StringSplit".to_owned()),
        "trim" => Some("StringTrim".to_owned()),
        "toUpperCase" => Some("StringToUpperCase".to_owned()),
        "toLowerCase" => Some("StringToLowerCase".to_owned()),
        "charCodeAt" => Some("StringCharCodeAt".to_owned()),
        "push" => Some("ArrayPush".to_owned()),
        "pop" => Some("ArrayPop".to_owned()),
        "concat" => Some("ArrayConcat".to_owned()),
        "join" => Some("ArrayJoin".to_owned()),
        "reverse" => Some("ArrayReverse".to_owned()),
        _ => None,
    }
}

fn unsupported_annex_b_string_method(method: &str, span: Span) -> Option<Diagnostic> {
    match method {
        "anchor" | "fontcolor" | "fontsize" | "link" | "substr" => Some(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("issue-067: Annex B String.prototype.{method} is not supported yet"),
            span: Some(span),
        }),
        _ => None,
    }
}

fn collection_method_runtime_fn(class_name: &str, method: &str) -> Option<&'static str> {
    match (class_name, method) {
        ("Map", "get") => Some("MapGet"),
        ("Map", "set") => Some("MapSet"),
        ("Map", "has") => Some("MapHas"),
        ("Map", "delete") => Some("MapDelete"),
        ("Set", "add") => Some("SetAdd"),
        ("Set", "has") => Some("SetHas"),
        ("Set", "delete") => Some("SetDelete"),
        ("RegExp", "test") => Some("RegExpTest"),
        ("RegExp", "exec") => Some("RegExpMatch"),
        _ => None,
    }
}

fn is_date_constructor_epoch_arg(arg: &ResolvedExpr) -> bool {
    match arg {
        ResolvedExpr::Number(_) => true,
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Negate => {
            matches!(expr.as_ref(), ResolvedExpr::Number(_))
        }
        _ => false,
    }
}

fn is_json_static_call(object: &ResolvedExpr, method: &str) -> bool {
    matches!(object, ResolvedExpr::Ident(name) if name == "JSON") && method == "stringify"
}

fn validate_json_stringify_args(
    args: &[ResolvedExpr],
    span: Span,
    function_ids: &HashMap<String, FuncId>,
) -> Result<(), Diagnostic> {
    if args.is_empty() || args.len() > 3 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "JSON.stringify expects 1 to 3 arguments, got {}",
                args.len()
            ),
            span: Some(span),
        });
    }

    if let Some(replacer) = args.get(1) {
        match replacer {
            ResolvedExpr::Null | ResolvedExpr::Undefined => {}
            ResolvedExpr::ArrowFn { .. } => {
                return Err(json_stringify_replacer_diagnostic(
                    "function replacer callbacks",
                    span,
                ));
            }
            ResolvedExpr::Ident(name) if function_ids.contains_key(name) => {
                return Err(json_stringify_replacer_diagnostic(
                    "function replacer callbacks",
                    span,
                ));
            }
            ResolvedExpr::Array(elements)
                if matches!(args.first(), Some(ResolvedExpr::Object(_)))
                    && is_supported_json_stringify_replacer_array(elements) => {}
            ResolvedExpr::Array(_) => {
                return Err(json_stringify_replacer_diagnostic(
                    "array replacer property lists outside the string/numeric-literal object subset",
                    span,
                ));
            }
            _ => {
                return Err(json_stringify_replacer_diagnostic("replacer values", span));
            }
        }
    }

    if let Some(space) = args.get(2)
        && !is_supported_json_stringify_space(space, function_ids)
    {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "JSON.stringify space currently supports numeric/string values and ignored object/function values"
                    .to_owned(),
            span: Some(span),
        });
    }

    Ok(())
}

fn is_supported_json_stringify_space(
    space: &ResolvedExpr,
    function_ids: &HashMap<String, FuncId>,
) -> bool {
    match space {
        ResolvedExpr::Number(_)
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::Object(_)
        | ResolvedExpr::ArrowFn { .. } => true,
        ResolvedExpr::Ident(name) => {
            function_ids.contains_key(name) || is_ignored_json_stringify_space_ident(name)
        }
        ResolvedExpr::Call { callee, args, .. } => {
            is_ignored_json_stringify_space_call(callee, args)
        }
        ResolvedExpr::New {
            class_name, args, ..
        } => is_supported_json_stringify_boxed_space(class_name, args),
        _ => false,
    }
}

fn is_supported_json_stringify_boxed_space(class_name: &str, args: &[ResolvedExpr]) -> bool {
    match (class_name, args) {
        ("Number", [arg]) => is_json_stringify_number_space_arg(arg),
        ("Number", []) => true,
        ("String", [ResolvedExpr::String(_)]) | ("String", []) => true,
        ("Boolean", []) => true,
        ("Boolean", [arg]) => is_json_stringify_primitive_space_arg(arg),
        ("Object", []) => true,
        _ => false,
    }
}

fn is_json_stringify_number_space_arg(arg: &ResolvedExpr) -> bool {
    matches!(arg, ResolvedExpr::Number(_))
        || matches!(
            arg,
            ResolvedExpr::Unary { op, expr }
                if *op == UnaryOp::Negate && matches!(expr.as_ref(), ResolvedExpr::Number(_))
        )
}

fn is_json_stringify_primitive_space_arg(arg: &ResolvedExpr) -> bool {
    matches!(
        arg,
        ResolvedExpr::Number(_)
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined
    )
}

fn is_supported_json_stringify_replacer_array(elements: &[ResolvedExpr]) -> bool {
    elements
        .iter()
        .all(|element| json_stringify_replacer_key(element).is_some())
}

fn json_stringify_replacer_key(element: &ResolvedExpr) -> Option<String> {
    match element {
        ResolvedExpr::String(key) => Some(key.clone()),
        ResolvedExpr::Number(value) => Some(value.to_string()),
        ResolvedExpr::Unary { op, expr }
            if *op == UnaryOp::Negate && matches!(expr.as_ref(), ResolvedExpr::Number(_)) =>
        {
            match expr.as_ref() {
                ResolvedExpr::Number(0) => Some("0".to_owned()),
                ResolvedExpr::Number(value) => Some(format!("-{value}")),
                _ => None,
            }
        }
        _ => None,
    }
}

fn json_stringify_replacer_keys(args: &[ResolvedExpr]) -> Option<Vec<String>> {
    match args.get(1) {
        Some(ResolvedExpr::Array(elements)) => {
            elements.iter().map(json_stringify_replacer_key).collect()
        }
        _ => None,
    }
}

fn should_ignore_json_stringify_space(
    space: &ResolvedExpr,
    function_ids: &HashMap<String, FuncId>,
) -> bool {
    matches!(
        space,
        ResolvedExpr::Object(_) | ResolvedExpr::ArrowFn { .. }
    ) || matches!(
        space,
        ResolvedExpr::Ident(name)
            if function_ids.contains_key(name) || is_ignored_json_stringify_space_ident(name)
    ) || matches!(
        space,
        ResolvedExpr::Call { callee, args, .. }
            if is_ignored_json_stringify_space_call(callee, args)
    ) || is_ignored_json_stringify_boxed_space(space)
}

fn is_ignored_json_stringify_space_ident(name: &str) -> bool {
    matches!(name, "Symbol" | "Number" | "String" | "Boolean" | "Object")
}

fn is_ignored_json_stringify_space_call(callee: &ResolvedExpr, args: &[ResolvedExpr]) -> bool {
    matches!(callee, ResolvedExpr::Ident(name) if name == "Symbol")
        && args.iter().all(is_json_stringify_primitive_space_arg)
}

fn is_ignored_json_stringify_boxed_space(space: &ResolvedExpr) -> bool {
    matches!(
        space,
        ResolvedExpr::New {
            class_name,
            args,
            ..
        } if matches!(class_name.as_str(), "Boolean" | "Object")
            || (matches!(class_name.as_str(), "Number" | "String") && args.is_empty())
    )
}

fn json_stringify_boxed_space_value(space: &ResolvedExpr) -> Option<&ResolvedExpr> {
    match space {
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "Number" && args.len() == 1 => args.first(),
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "String" && args.len() == 1 => args.first(),
        _ => None,
    }
}

fn json_stringify_replacer_diagnostic(kind: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-052: JSON.stringify {kind} are not supported yet; pass null or undefined until replacer semantics are implemented"
        ),
        span: Some(span),
    }
}

fn is_date_now_live_time_call(object: &ResolvedExpr, method: &str) -> bool {
    matches!(object, ResolvedExpr::Ident(name) if name == "Date") && method == "now"
}

fn unsupported_date_timezone_diagnostic(method: &str, span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-050: Date.prototype.{method}() requires timezone/host formatting policy; use getTime() or valueOf() for deterministic epoch milliseconds"
        ),
        span,
    }
}

fn is_annex_b_date_method(method: &str) -> bool {
    matches!(method, "getYear" | "setYear" | "toGMTString")
}

fn unsupported_annex_b_date_method_diagnostic(method: &str, span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-241: Date.prototype.{method} is Annex B legacy Date behavior and is not supported in the deterministic Date epoch slice"
        ),
        span,
    }
}

fn regexp_constructor_literal(args: &[ResolvedExpr]) -> Result<String, Diagnostic> {
    if !(1..=2).contains(&args.len()) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-051: RegExp constructor supports 1 string literal pattern and optional string literal flags in this subset, got {}",
                args.len()
            ),
            span: None,
        });
    }
    let ResolvedExpr::String(pattern) = &args[0] else {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-051: RegExp constructor pattern must be a string literal in this subset"
                    .to_owned(),
            span: None,
        });
    };
    let flags = match args.get(1) {
        Some(ResolvedExpr::String(flags)) => flags.as_str(),
        Some(_) => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-051: RegExp constructor flags must be a string literal in this subset"
                        .to_owned(),
                span: None,
            });
        }
        None => "",
    };
    let raw = format!("/{pattern}/{flags}");
    validate_regexp_plain_literal(&raw, "RegExp constructor")?;
    Ok(raw)
}

fn regexp_test_runtime(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
    span: ts2wasm_frontend::Span,
) -> Result<Option<Vec<ResolvedExpr>>, Diagnostic> {
    if method != "test" {
        return Ok(None);
    }
    if args.len() != 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "RegExp.prototype.test expects 1 argument, got {}",
                args.len()
            ),
            span: Some(span),
        });
    }
    match object {
        ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
            validate_regexp_plain_literal(raw, "RegExp.prototype.test literal")?;
            Ok(Some(vec![object.clone(), args[0].clone()]))
        }
        ResolvedExpr::New {
            class_name,
            args: ctor_args,
            ..
        } if class_name == "RegExp" => {
            regexp_constructor_literal(ctor_args)?;
            Ok(Some(vec![object.clone(), args[0].clone()]))
        }
        _ => Ok(None),
    }
}

fn regexp_string_match_runtime(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
    span: ts2wasm_frontend::Span,
) -> Result<Option<Vec<ResolvedExpr>>, Diagnostic> {
    if method != "match" {
        return Ok(None);
    }
    if args.len() != 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "String.prototype.match expects 1 argument, got {}",
                args.len()
            ),
            span: Some(span),
        });
    }
    if !matches!(object, ResolvedExpr::String(_) | ResolvedExpr::Ident(_)) {
        return Ok(None);
    }
    match &args[0] {
        ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
            validate_regexp_plain_literal(raw, "String.prototype.match literal")?;
        }
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "RegExp" => {
            regexp_constructor_literal(args)?;
        }
        _ => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-051: String.prototype.match supports only RegExp literal or new RegExp(\"plain\") arguments in this subset"
                        .to_owned(),
                span: Some(span),
            });
        }
    }
    Ok(Some(vec![args[0].clone(), object.clone()]))
}

fn regexp_exec_runtime(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
    span: ts2wasm_frontend::Span,
) -> Result<Option<Vec<ResolvedExpr>>, Diagnostic> {
    if method != "exec" {
        return Ok(None);
    }
    if args.len() != 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "RegExp.prototype.exec expects 1 argument, got {}",
                args.len()
            ),
            span: Some(span),
        });
    }
    match object {
        ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
            validate_regexp_plain_literal(raw, "RegExp.prototype.exec literal")?;
            Ok(Some(vec![object.clone(), args[0].clone()]))
        }
        ResolvedExpr::New {
            class_name,
            args: ctor_args,
            ..
        } if class_name == "RegExp" => {
            regexp_constructor_literal(ctor_args)?;
            Ok(Some(vec![object.clone(), args[0].clone()]))
        }
        _ => Ok(None),
    }
}

fn looks_like_regexp_literal(raw: &str) -> bool {
    raw.starts_with('/') && raw[1..].contains('/')
}

fn validate_regexp_plain_literal(raw: &str, context: &str) -> Result<(), Diagnostic> {
    let Some(delimiter) = raw.rfind('/') else {
        return Err(unsupported_regexp_literal(
            context,
            raw,
            "missing closing delimiter",
        ));
    };
    if delimiter == 0 {
        return Err(unsupported_regexp_literal(context, raw, "missing pattern"));
    }
    let flags = &raw[delimiter + 1..];
    if flags.chars().any(|ch| ch != 'g') || flags.chars().count() > 1 {
        return Err(unsupported_regexp_literal(
            context,
            raw,
            "only the empty flag set or `g` is supported",
        ));
    }
    let pattern = &raw[1..delimiter];
    if pattern.chars().any(|ch| {
        matches!(
            ch,
            '^' | '$' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\'
        )
    }) {
        return Err(unsupported_regexp_literal(
            context,
            raw,
            "only plain literal byte patterns are supported",
        ));
    }
    Ok(())
}

fn unsupported_regexp_literal(context: &str, raw: &str, reason: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("issue-051: {context} `{raw}` is not supported yet: {reason}"),
        span: None,
    }
}

fn unsupported_regexp_compile_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-051: RegExp.prototype.compile is not supported in this subset; create a new RegExp(\"plain\") value instead"
            .to_owned(),
        span,
    }
}

fn collect_arrow_captures(expr: &ResolvedExpr, params: &[String], captures: &mut Vec<String>) {
    match expr {
        ResolvedExpr::This { .. } => push_capture("this", params, captures),
        ResolvedExpr::Ident(name) => push_capture(name, params, captures),
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => {
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::Binary { left, right, .. } => {
            collect_arrow_captures(left, params, captures);
            collect_arrow_captures(right, params, captures);
        }
        ResolvedExpr::Call { callee, args, .. } => {
            collect_arrow_captures(callee, params, captures);
            for arg in args {
                collect_arrow_captures(arg, params, captures);
            }
        }
        ResolvedExpr::Assign { name, expr } => {
            push_capture(name, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalAssign { name, expr, .. } => {
            push_capture(name, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalPropertyAssign { object, expr, .. } => {
            push_capture(object, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalComputedPropertyAssign {
            object, key, expr, ..
        } => {
            push_capture(object, params, captures);
            collect_arrow_captures(key, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(key, params, captures);
            collect_arrow_captures(expr, params, captures);
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                collect_arrow_captures(element, params, captures);
            }
        }
        ResolvedExpr::Object(props) => {
            for (_, value) in props {
                collect_arrow_captures(value, params, captures);
            }
        }
        ResolvedExpr::ComputedIndex { object, index } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(index, params, captures);
        }
        ResolvedExpr::BuiltinCall { args, .. } => {
            for arg in args {
                collect_arrow_captures(arg, params, captures);
            }
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. } => {
            collect_arrow_captures(object, params, captures);
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            collect_arrow_captures(object, params, captures);
            for arg in args {
                collect_arrow_captures(arg, params, captures);
            }
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(value, params, captures);
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            collect_arrow_captures(object, params, captures);
            collect_arrow_captures(key, params, captures);
            collect_arrow_captures(value, params, captures);
        }
        ResolvedExpr::New { args, .. } => {
            for arg in args {
                collect_arrow_captures(arg, params, captures);
            }
        }
        ResolvedExpr::ArrowFn { .. }
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined => {}
    }
}

fn push_capture(name: &str, params: &[String], captures: &mut Vec<String>) {
    if params.iter().any(|param| param == name) || captures.iter().any(|capture| capture == name) {
        return;
    }
    captures.push(name.to_owned());
}

fn lower_unary_op(op: UnaryOp) -> Result<LoweredUnaryOp, Diagnostic> {
    match op {
        UnaryOp::Not => Ok(LoweredUnaryOp::Not),
        UnaryOp::Negate => Ok(LoweredUnaryOp::Negate),
        UnaryOp::TypeOf => Ok(LoweredUnaryOp::TypeOf),
        UnaryOp::Delete => Ok(LoweredUnaryOp::Delete),
        UnaryOp::Increment
        | UnaryOp::Decrement
        | UnaryOp::PreIncrement
        | UnaryOp::PreDecrement
        | UnaryOp::BitwiseNot
        | UnaryOp::Void => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("unary operator {:?} not yet supported", op),
            span: None,
        }),
    }
}
