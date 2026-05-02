pub fn validate_lowered(program: &LoweredProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let num_funcs = program.functions.len();

    validate_functions(program, &mut errors);

    validate_stmts(
        &program.top_level_statements,
        program.top_level_locals.len(),
        num_funcs,
        program,
        &mut errors,
    );

    for func in &program.functions {
        let local_count = func.params.len() + func.locals.len();
        validate_stmts(&func.body, local_count, num_funcs, program, &mut errors);
    }

    for module in &program.modules {
        validate_stmts(
            &module.statements,
            module.locals_count,
            num_funcs,
            program,
            &mut errors,
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_functions(program: &LoweredProgram, errors: &mut Vec<Diagnostic>) {
    for (idx, function) in program.functions.iter().enumerate() {
        if function.id.0 != idx {
            errors.push(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "function id {} does not match its index {}",
                    function.id.0, idx
                ),
                span: None,
            });
        }

        for (param_index, local_id) in function.params.iter().enumerate() {
            if local_id.0 != param_index {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "parameter LocalId {} must match parameter index {}",
                        local_id.0, param_index
                    ),
                    span: None,
                });
            }
        }

        if let Some(rest_param_index) = function.rest_param_index {
            if rest_param_index >= function.params.len() {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "rest parameter index {} is out of range (function has {} parameter(s))",
                        rest_param_index,
                        function.params.len()
                    ),
                    span: None,
                });
            } else if rest_param_index + 1 != function.params.len() {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "rest parameter index {} must be the final parameter",
                        rest_param_index
                    ),
                    span: None,
                });
            }
        }

        let base = function.params.len();
        for (local_index, local_id) in function.locals.iter().enumerate() {
            let expected = base + local_index;
            if local_id.0 != expected {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "local LocalId {} must be contiguous and start at {}",
                        local_id.0, base
                    ),
                    span: None,
                });
            }
        }
    }
}

fn validate_stmts(
    stmts: &[LoweredStmt],
    local_count: usize,
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
) {
    for stmt in stmts {
        validate_stmt(stmt, local_count, num_funcs, program, errors);
    }
}

fn validate_stmt(
    stmt: &LoweredStmt,
    local_count: usize,
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
) {
    match stmt {
        LoweredStmt::Block(stmts) => {
            validate_stmts(stmts, local_count, num_funcs, program, errors);
        }
        LoweredStmt::Let(id, expr) | LoweredStmt::Assign(id, expr) => {
            check_local_id(*id, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::Expr(expr) => {
            validate_expr(expr, local_count, num_funcs, program, errors, false);
        }
        LoweredStmt::Return(expr) => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::Throw(expr) => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            validate_expr(condition, local_count, num_funcs, program, errors, true);
            validate_stmts(then_body, local_count, num_funcs, program, errors);
            validate_stmts(else_body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::While { condition, body } => {
            validate_expr(condition, local_count, num_funcs, program, errors, true);
            validate_stmts(body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
        } => {
            validate_stmts(try_body, local_count, num_funcs, program, errors);
            if let Some(var_id) = catch_var {
                check_local_id(*var_id, local_count, errors);
            }
            if let Some(body) = catch_body {
                validate_stmts(body, local_count, num_funcs, program, errors);
            }
            if let Some(body) = finally_body {
                validate_stmts(body, local_count, num_funcs, program, errors);
            }
            if catch_body.is_none() && finally_body.is_none() {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "try-catch must have at least a catch or finally block".to_owned(),
                    span: None,
                });
            }
        }
        LoweredStmt::Switch { expr, cases } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
            for (cond, body) in cases {
                if let Some(c) = cond {
                    validate_expr(c, local_count, num_funcs, program, errors, true);
                }
                validate_stmts(body, local_count, num_funcs, program, errors);
            }
        }
        LoweredStmt::DoWhile { body, condition } => {
            validate_stmts(body, local_count, num_funcs, program, errors);
            validate_expr(condition, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            if let Some(i) = init {
                validate_stmt(i, local_count, num_funcs, program, errors);
            }
            if let Some(c) = condition {
                validate_expr(c, local_count, num_funcs, program, errors, true);
            }
            if let Some(u) = update {
                validate_expr(u, local_count, num_funcs, program, errors, true);
            }
            validate_stmts(body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::ForIn {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
        } => {
            check_local_id(*var, local_count, errors);
            check_local_id(*iter_local, local_count, errors);
            check_local_id(*index_local, local_count, errors);
            check_local_id(*len_local, local_count, errors);
            validate_expr(iter, local_count, num_funcs, program, errors, true);
            validate_stmts(body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::ForOf {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
        } => {
            check_local_id(*var, local_count, errors);
            check_local_id(*iter_local, local_count, errors);
            check_local_id(*index_local, local_count, errors);
            check_local_id(*len_local, local_count, errors);
            validate_expr(iter, local_count, num_funcs, program, errors, true);
            validate_stmts(body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::Labeled { body, .. } => {
            validate_stmt(body, local_count, num_funcs, program, errors)
        }
        LoweredStmt::Break { .. } | LoweredStmt::Continue { .. } => {}
        LoweredStmt::Export { expr, .. } | LoweredStmt::ModuleExportsAssign { expr } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::ClassDecl { .. } => {
            // Dead variant: ResolvedStmt::ClassDecl is handled directly in program.rs:216
            // (constructor/methods extracted as functions, statement dropped). It never reaches
            // the lowered resolver or the validator. This variant is a future-class-support
            // placeholder. See the LIMITATION comment in program.rs for details.
        }
    }
}

fn validate_expr(
    expr: &LoweredExpr,
    local_count: usize,
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
    value_required: bool,
) {
    match expr {
        LoweredExpr::Number(_) => {}
        LoweredExpr::BigIntLiteral { .. } => {}
        LoweredExpr::Local(id) => check_local_id(*id, local_count, errors),
        LoweredExpr::EnvCellNew(expr) => validate_expr(expr, local_count, num_funcs, program, errors, true),
        LoweredExpr::EnvCellGet(cell) => check_local_id(*cell, local_count, errors),
        LoweredExpr::EnvCellSet { cell, expr } => {
            check_local_id(*cell, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::Unary { expr, .. } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::Assign { local, expr } => {
            check_local_id(*local, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::LogicalAssign { local, expr, .. } => {
            check_local_id(*local, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::LogicalPropertyAssign { object, expr, .. } => {
            check_local_id(*object, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::LogicalMemberAssign { object, expr, .. } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(key, local_count, num_funcs, program, errors, true);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::LogicalComputedPropertyAssign {
            object, key, expr, ..
        } => {
            check_local_id(*object, local_count, errors);
            validate_expr(key, local_count, num_funcs, program, errors, true);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::Binary { left, right, .. } => {
            validate_expr(left, local_count, num_funcs, program, errors, true);
            validate_expr(right, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::Call { kind, args } => {
            for arg in args {
                validate_expr(arg, local_count, num_funcs, program, errors, true);
            }
            match kind {
                FunctionCallKind::User(func_id) => {
                    if func_id.0 >= num_funcs {
                        errors.push(Diagnostic {
                            code: DiagCode::InvariantViolation,
                            message: format!(
                                "FuncId {} is out of range (program has {} function(s))",
                                func_id.0, num_funcs
                            ),
                            span: None,
                        });
                    } else {
                        let func = &program.functions[func_id.0];
                        let min_required = func.min_required_params;
                        if args.len() < min_required {
                            errors.push(Diagnostic {
                                code: DiagCode::ArityMismatch,
                                message: format!(
                                    "function {} expects at least {} argument(s), got {}",
                                    func_id.0,
                                    min_required,
                                    args.len()
                                ),
                                span: None,
                            });
                        } else if func.rest_param_index.is_none() {
                            let max_allowed = func.params.len();
                            if args.len() > max_allowed {
                                errors.push(Diagnostic {
                                    code: DiagCode::ArityMismatch,
                                    message: format!(
                                        "function {} expects between {} and {} argument(s), got {}",
                                        func_id.0,
                                        min_required,
                                        max_allowed,
                                        args.len()
                                    ),
                                    span: None,
                                });
                            }
                        }
                    }
                }
                FunctionCallKind::Builtin(builtin) => {
                    let expected = builtin.expected_arity();
                    if args.len() != expected {
                        errors.push(Diagnostic {
                            code: DiagCode::ArityMismatch,
                            message: format!(
                                "builtin {:?} expects {} argument(s), got {}",
                                builtin,
                                expected,
                                args.len()
                            ),
                            span: None,
                        });
                    }
                    if value_required && matches!(builtin.result(), BuiltinResult::EffectOnly) {
                        errors.push(Diagnostic {
                            code: DiagCode::InvariantViolation,
                            message: format!(
                                "builtin {:?} is effect-only and cannot be used in a value context",
                                builtin
                            ),
                            span: None,
                        });
                    }
                }
            }
        }
        LoweredExpr::RuntimeCall { runtime_fn, args } => {
            for arg in args {
                validate_expr(arg, local_count, num_funcs, program, errors, true);
            }
            if runtime_fn == "ArrayPushMany" && args.is_empty() {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "ArrayPushMany must include an array receiver argument".to_owned(),
                    span: None,
                });
            }
            if runtime_fn == "ArrayPushGrow" && args.len() != 2 {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "ArrayPushGrow must include an array receiver and value".to_owned(),
                    span: None,
                });
            }
            if runtime_fn == "HeapClosureCall" && args.is_empty() {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "HeapClosureCall must include a closure receiver argument"
                        .to_owned(),
                    span: None,
                });
            }
            if runtime_fn == "PrivateFieldGet"
                && !matches!(args.as_slice(), [_, LoweredExpr::Number(brand), LoweredExpr::Number(slot)] if *brand > 0 && *slot >= 0)
            {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "PrivateFieldGet must include an object, positive private brand, and non-negative private slot"
                        .to_owned(),
                    span: None,
                });
            }
            if runtime_fn == "PrivateFieldSet"
                && !matches!(args.as_slice(), [_, LoweredExpr::Number(brand), LoweredExpr::Number(slot), _] if *brand > 0 && *slot >= 0)
            {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message:
                        "PrivateFieldSet must include an object, positive private brand, non-negative private slot, and value"
                            .to_owned(),
                    span: None,
                });
            }
            if runtime_fn == "PrivateBrandCheck"
                && !matches!(args.as_slice(), [_, LoweredExpr::Number(brand)] if *brand > 0)
            {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "PrivateBrandCheck must include an object and positive private brand"
                        .to_owned(),
                    span: None,
                });
            }
        }
        LoweredExpr::ArrayNew { elements } => {
            for elem in elements {
                validate_expr(elem, local_count, num_funcs, program, errors, true);
            }
        }
        LoweredExpr::ArrayNewSparse { slots } => {
            for slot in slots {
                if let LoweredArraySlot::Present(elem) = slot {
                    validate_expr(elem, local_count, num_funcs, program, errors, true);
                }
            }
        }
        LoweredExpr::ArrayGet { arr, index } => {
            validate_expr(arr, local_count, num_funcs, program, errors, true);
            validate_expr(index, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::Index { object, index } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(index, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::GetLength(expr) => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::ObjectNew { props } => {
            for (_, val) in props {
                validate_expr(val, local_count, num_funcs, program, errors, true);
            }
        }
        LoweredExpr::ErrorNew { message, .. } => {
            validate_expr(message, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::PropertyGet { obj, .. } => {
            validate_expr(obj, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::PropertySet { object, value, .. } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(value, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
        } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(index, local_count, num_funcs, program, errors, true);
            validate_expr(value, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::New {
            constructor,
            prototype,
            args,
            base_local,
            private_brand,
            private_slot_count,
        } => {
            check_func_id(*constructor, num_funcs, errors);
            check_func_id(prototype.constructor, num_funcs, errors);
            for parent in &prototype.parent_constructors {
                check_func_id(*parent, num_funcs, errors);
            }
            check_local_id(*base_local, local_count, errors);
            if *private_slot_count > 0 && !private_brand.is_some_and(|brand| brand > 0) {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "class instances with private slots must include a positive private brand".to_owned(),
                    span: None,
                });
            }
            if *private_slot_count > u16::MAX as usize {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "class private slot count exceeds runtime metadata capacity".to_owned(),
                    span: None,
                });
            }
            validate_constructor_arity(*constructor, args, num_funcs, program, errors);
            for arg in args {
                validate_expr(arg, local_count, num_funcs, program, errors, true);
            }
        }
        LoweredExpr::ClassPrototype(prototype) => {
            check_func_id(prototype.constructor, num_funcs, errors);
            for parent in &prototype.parent_constructors {
                check_func_id(*parent, num_funcs, errors);
            }
        }
        LoweredExpr::BuiltinErrorPrototype(_) => {}
        LoweredExpr::This => {
            errors.push(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-211: residual `this` must be resolved to an active receiver local before backend emission".to_owned(),
                span: None,
            });
        }
        LoweredExpr::MethodCall { object, .. } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            errors.push(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "MethodCall must be resolved before backend; residual MethodCall is unsupported"
                        .to_owned(),
                span: None,
            });
        }
        LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: _,
        } => {
            check_func_id(*func_id, num_funcs, errors);
            for capture in captures {
                check_local_id(*capture, local_count, errors);
            }
        }
        LoweredExpr::String(_) => {}
        LoweredExpr::Bool(_) => {}
        LoweredExpr::Null => {}
        LoweredExpr::Undefined => {}
        LoweredExpr::PropertyIn { obj, .. } => {
            validate_expr(obj, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::PropertyInDynamic { obj, key } => {
            validate_expr(obj, local_count, num_funcs, program, errors, true);
            validate_expr(key, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::PropertyDelete { object, .. } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::PropertyDeleteDynamic { object, key } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(key, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::PropertyGetDynamic { obj, key } => {
            validate_expr(obj, local_count, num_funcs, program, errors, true);
            validate_expr(key, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::OptionalPropertyGet { obj, .. } => {
            validate_expr(obj, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::OptionalIndex { object, index } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(index, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::OptionalCall { callee, call } => {
            validate_expr(callee, local_count, num_funcs, program, errors, true);
            validate_expr(call, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::ModuleLoad { module_id } => {
            if program.modules.iter().all(|m| m.id != *module_id) && *module_id != 0 {
                // module_id 0 refers to the entry module; other IDs must exist in program.modules
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "ModuleLoad references module_id {} which is not in the program's module list",
                        module_id
                    ),
                    span: None,
                });
            }
        }
    }
}

fn check_local_id(id: LocalId, local_count: usize, errors: &mut Vec<Diagnostic>) {
    if id.0 >= local_count {
        errors.push(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "LocalId {} is out of range (scope has {} local(s))",
                id.0, local_count
            ),
            span: None,
        });
    }
}

fn check_func_id(id: FuncId, num_funcs: usize, errors: &mut Vec<Diagnostic>) {
    if id.0 >= num_funcs {
        errors.push(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "FuncId {} is out of range (program has {} function(s))",
                id.0, num_funcs
            ),
            span: None,
        });
    }
}

fn validate_constructor_arity(
    constructor: FuncId,
    args: &[LoweredExpr],
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
) {
    if constructor.0 >= num_funcs {
        return;
    }
    let func = &program.functions[constructor.0];
    let min_required = func.min_required_params.saturating_sub(1);
    if args.len() < min_required {
        errors.push(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "constructor {} expects at least {} argument(s), got {}",
                constructor.0,
                min_required,
                args.len()
            ),
            span: None,
        });
    } else if func.rest_param_index.is_none() {
        let max_allowed = func.params.len().saturating_sub(1);
        if args.len() > max_allowed {
            errors.push(Diagnostic {
                code: DiagCode::ArityMismatch,
                message: format!(
                    "constructor {} expects between {} and {} argument(s), got {}",
                    constructor.0,
                    min_required,
                    max_allowed,
                    args.len()
                ),
                span: None,
            });
        }
    }
}
