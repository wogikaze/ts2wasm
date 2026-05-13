use crate::builtin::{BuiltinId, BuiltinResult};
use crate::lowered::{
    FuncId, FunctionCallKind, LocalId, MirArraySlot, MirExpr, MirFunction, MirProgram, MirStmt,
    RuntimeFn,
};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};

/// Validate a native MIR program without converting it back to `LoweredProgram`.
pub fn validate_mir(program: &MirProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let num_funcs = program.functions.len();

    validate_functions(program, &mut errors);
    validate_modules(program, &mut errors);
    validate_top_level_locals(program, &mut errors);

    validate_stmts(
        &program.top_level_statements,
        program.top_level_locals.len(),
        num_funcs,
        program,
        ScopeKind::TopLevel,
        &mut errors,
    );

    for func in &program.functions {
        let local_count = func.params.len() + func.locals.len();
        validate_stmts(
            &func.body,
            local_count,
            num_funcs,
            program,
            ScopeKind::Function,
            &mut errors,
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScopeKind {
    TopLevel,
    Function,
}

fn invariant(message: impl Into<String>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::InvariantViolation,
        message: message.into(),
        span: None,
        phase: None,
    }
}

fn arity(message: impl Into<String>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::ArityMismatch,
        message: message.into(),
        span: None,
        phase: None,
    }
}

fn validate_functions(program: &MirProgram, errors: &mut Vec<Diagnostic>) {
    for (idx, function) in program.functions.iter().enumerate() {
        if function.id.0 != idx {
            errors.push(invariant(format!(
                "function id {} does not match its index {}",
                function.id.0, idx
            )));
        }

        for (param_index, local_id) in function.params.iter().enumerate() {
            if local_id.0 != param_index {
                errors.push(invariant(format!(
                    "parameter LocalId {} must match parameter index {}",
                    local_id.0, param_index
                )));
            }
        }

        if let Some(rest_param_index) = function.rest_param_index {
            if rest_param_index >= function.params.len() {
                errors.push(invariant(format!(
                    "rest parameter index {} is out of range (function has {} parameter(s))",
                    rest_param_index,
                    function.params.len()
                )));
            } else if rest_param_index + 1 != function.params.len() {
                errors.push(invariant(format!(
                    "rest parameter index {} must be the final parameter",
                    rest_param_index
                )));
            }
        }

        let base = function.params.len();
        for (local_index, local_id) in function.locals.iter().enumerate() {
            let expected = base + local_index;
            if local_id.0 != expected {
                errors.push(invariant(format!(
                    "local LocalId {} must be contiguous and start at {}",
                    local_id.0, base
                )));
            }
        }

        validate_generator_state(function, errors);
    }
}

fn validate_generator_state(function: &MirFunction, errors: &mut Vec<Diagnostic>) {
    let Some(state) = &function.generator_state else {
        return;
    };
    if !function.is_generator {
        errors.push(invariant(format!(
            "function {} has generator state but is not marked generator",
            function.id.0
        )));
    }
    for point in &state.suspend_points {
        if point.resume_state == state.completed_state {
            errors.push(invariant(format!(
                "function {} suspend point {} uses completed resume state {}",
                function.id.0, point.index, point.resume_state
            )));
        }
    }
}

fn validate_modules(program: &MirProgram, errors: &mut Vec<Diagnostic>) {
    let mut seen = std::collections::BTreeSet::new();
    for module in &program.modules {
        if !seen.insert(module.id) {
            errors.push(invariant(format!("duplicate module id {}", module.id)));
        }
    }
}

fn validate_top_level_locals(program: &MirProgram, errors: &mut Vec<Diagnostic>) {
    for (index, local_id) in program.top_level_locals.iter().enumerate() {
        if local_id.0 != index {
            errors.push(invariant(format!(
                "top_level_locals LocalId {} must match its index {}",
                local_id.0, index
            )));
        }
    }
}

fn validate_stmts(
    stmts: &[MirStmt],
    local_count: usize,
    num_funcs: usize,
    program: &MirProgram,
    scope: ScopeKind,
    errors: &mut Vec<Diagnostic>,
) {
    for stmt in stmts {
        validate_stmt(stmt, local_count, num_funcs, program, scope, errors);
    }
}

fn validate_stmt(
    stmt: &MirStmt,
    local_count: usize,
    num_funcs: usize,
    program: &MirProgram,
    scope: ScopeKind,
    errors: &mut Vec<Diagnostic>,
) {
    match stmt {
        MirStmt::Block(stmts, _) => {
            validate_stmts(stmts, local_count, num_funcs, program, scope, errors);
        }
        MirStmt::Let(id, expr, _) | MirStmt::Assign(id, expr, _) => {
            check_local_id(*id, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirStmt::Expr(expr, _) => {
            validate_expr(expr, local_count, num_funcs, program, errors, false);
        }
        MirStmt::Yield(expr, _) => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirStmt::Return(expr, _) => {
            if scope == ScopeKind::TopLevel {
                errors.push(invariant("top-level return is invalid in MIR"));
            }
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirStmt::Throw(expr, _) => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            validate_expr(condition, local_count, num_funcs, program, errors, true);
            validate_stmts(then_body, local_count, num_funcs, program, scope, errors);
            validate_stmts(else_body, local_count, num_funcs, program, scope, errors);
        }
        MirStmt::While {
            condition, body, ..
        } => {
            validate_expr(condition, local_count, num_funcs, program, errors, true);
            validate_stmts(body, local_count, num_funcs, program, scope, errors);
        }
        MirStmt::TryFinally {
            try_body,
            finally_body,
            ..
        } => {
            validate_stmts(try_body, local_count, num_funcs, program, scope, errors);
            validate_stmts(finally_body, local_count, num_funcs, program, scope, errors);
        }
        MirStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
            ..
        } => {
            validate_stmts(try_body, local_count, num_funcs, program, scope, errors);
            if let Some(var_id) = catch_var {
                check_local_id(*var_id, local_count, errors);
            }
            if let Some(body) = catch_body {
                validate_stmts(body, local_count, num_funcs, program, scope, errors);
            }
            if let Some(body) = finally_body {
                validate_stmts(body, local_count, num_funcs, program, scope, errors);
            }
            if catch_body.is_none() && finally_body.is_none() {
                errors.push(invariant(
                    "try-catch must have at least a catch or finally block",
                ));
            }
        }
        MirStmt::Switch { expr, cases, .. } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
            for (cond, body) in cases {
                if let Some(c) = cond {
                    validate_expr(c, local_count, num_funcs, program, errors, true);
                }
                validate_stmts(body, local_count, num_funcs, program, scope, errors);
            }
        }
        MirStmt::DoWhile {
            body, condition, ..
        } => {
            validate_stmts(body, local_count, num_funcs, program, scope, errors);
            validate_expr(condition, local_count, num_funcs, program, errors, true);
        }
        MirStmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            if let Some(i) = init {
                validate_stmt(i, local_count, num_funcs, program, scope, errors);
            }
            if let Some(c) = condition {
                validate_expr(c, local_count, num_funcs, program, errors, true);
            }
            if let Some(u) = update {
                validate_expr(u, local_count, num_funcs, program, errors, true);
            }
            validate_stmts(body, local_count, num_funcs, program, scope, errors);
        }
        MirStmt::ForIn {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            ..
        }
        | MirStmt::ForOf {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            ..
        } => {
            check_local_id(*var, local_count, errors);
            check_local_id(*iter_local, local_count, errors);
            check_local_id(*index_local, local_count, errors);
            check_local_id(*len_local, local_count, errors);
            validate_expr(iter, local_count, num_funcs, program, errors, true);
            validate_stmts(body, local_count, num_funcs, program, scope, errors);
        }
        MirStmt::ForAwaitOfLower {
            var,
            iter,
            async_iter_local,
            next_result_local,
            done_local,
            value_local,
            body,
            ..
        } => {
            check_local_id(*var, local_count, errors);
            check_local_id(*async_iter_local, local_count, errors);
            check_local_id(*next_result_local, local_count, errors);
            check_local_id(*done_local, local_count, errors);
            check_local_id(*value_local, local_count, errors);
            validate_expr(iter, local_count, num_funcs, program, errors, true);
            validate_stmts(body, local_count, num_funcs, program, scope, errors);
        }
        MirStmt::Labeled { body, .. } => {
            validate_stmt(body, local_count, num_funcs, program, scope, errors)
        }
        MirStmt::Break { .. } | MirStmt::Continue { .. } => {}
        MirStmt::Export { expr, .. } | MirStmt::ModuleExportsAssign { expr, .. } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirStmt::ModuleExportsUpdate { local, .. } => {
            check_local_id(*local, local_count, errors);
        }
        MirStmt::ClassDecl {
            constructor,
            methods,
            static_methods,
            ..
        } => {
            if let Some(ctor_id) = constructor {
                check_func_id(*ctor_id, num_funcs, errors);
            }
            for (_, method_id) in methods.iter().chain(static_methods.iter()) {
                check_func_id(*method_id, num_funcs, errors);
            }
        }
    }
}

fn validate_expr(
    expr: &MirExpr,
    local_count: usize,
    num_funcs: usize,
    program: &MirProgram,
    errors: &mut Vec<Diagnostic>,
    value_required: bool,
) {
    match expr {
        MirExpr::Number(_, _)
        | MirExpr::DecimalNumber(_, _)
        | MirExpr::BigIntLiteral { .. }
        | MirExpr::String(_, _)
        | MirExpr::Bool(_, _)
        | MirExpr::Null(_)
        | MirExpr::Undefined(_)
        | MirExpr::BuiltinErrorPrototype(_, _) => {}
        MirExpr::Local(id, _) => check_local_id(*id, local_count, errors),
        MirExpr::EnvCellNew(expr, _) => {
            validate_expr(expr, local_count, num_funcs, program, errors, true)
        }
        MirExpr::EnvCellGet(cell, _) => check_local_id(*cell, local_count, errors),
        MirExpr::EnvCellSet { cell, expr, .. } => {
            check_local_id(*cell, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirExpr::Unary { expr, .. } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirExpr::Assign { local, expr, .. } | MirExpr::LogicalAssign { local, expr, .. } => {
            check_local_id(*local, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirExpr::LogicalPropertyAssign { object, expr, .. } => {
            check_local_id(*object, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirExpr::LogicalComputedPropertyAssign {
            object, key, expr, ..
        } => {
            check_local_id(*object, local_count, errors);
            validate_expr(key, local_count, num_funcs, program, errors, true);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirExpr::LogicalMemberAssign { object, expr, .. } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(key, local_count, num_funcs, program, errors, true);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirExpr::Binary { left, right, .. } => {
            validate_expr(left, local_count, num_funcs, program, errors, true);
            validate_expr(right, local_count, num_funcs, program, errors, true);
        }
        MirExpr::PropertyIn { obj, .. }
        | MirExpr::PropertyGet { obj, .. }
        | MirExpr::OptionalPropertyGet { obj, .. }
        | MirExpr::PropertyDelete { object: obj, .. }
        | MirExpr::MethodCall { object: obj, .. } => {
            validate_expr(obj, local_count, num_funcs, program, errors, true);
            if matches!(expr, MirExpr::MethodCall { .. }) {
                errors.push(invariant(
                    "MethodCall must be resolved before backend; residual MethodCall is unsupported",
                ));
            }
        }
        MirExpr::PropertyInDynamic { obj, key, .. }
        | MirExpr::PropertyGetDynamic { obj, key, .. }
        | MirExpr::PropertyDeleteDynamic {
            object: obj, key, ..
        } => {
            validate_expr(obj, local_count, num_funcs, program, errors, true);
            validate_expr(key, local_count, num_funcs, program, errors, true);
        }
        MirExpr::Call { kind, args, .. } => {
            for arg in args {
                validate_expr(arg, local_count, num_funcs, program, errors, true);
            }
            validate_call_kind(
                *kind,
                args.len(),
                num_funcs,
                program,
                errors,
                value_required,
            );
        }
        MirExpr::RuntimeCall {
            intrinsic, args, ..
        } => {
            for arg in args {
                validate_expr(arg, local_count, num_funcs, program, errors, true);
            }
            validate_runtime_call(*intrinsic, args, errors);
        }
        MirExpr::ArrayNew { elements, .. } => {
            for elem in elements {
                validate_expr(elem, local_count, num_funcs, program, errors, true);
            }
        }
        MirExpr::ArrayNewSparse { slots, .. } => {
            for slot in slots {
                if let MirArraySlot::Present(elem) = slot {
                    validate_expr(elem, local_count, num_funcs, program, errors, true);
                }
            }
        }
        MirExpr::ArrayGet { arr, index, .. }
        | MirExpr::Index {
            object: arr, index, ..
        }
        | MirExpr::OptionalIndex {
            object: arr, index, ..
        } => {
            validate_expr(arr, local_count, num_funcs, program, errors, true);
            validate_expr(index, local_count, num_funcs, program, errors, true);
        }
        MirExpr::OptionalCall { callee, call, .. } => {
            validate_expr(callee, local_count, num_funcs, program, errors, true);
            validate_expr(call, local_count, num_funcs, program, errors, true);
        }
        MirExpr::GetLength(expr, _) | MirExpr::PromiseGetValue { promise: expr, .. } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        MirExpr::ObjectNew { props, .. } => {
            for (_, val) in props {
                validate_expr(val, local_count, num_funcs, program, errors, true);
            }
        }
        MirExpr::ErrorNew { message, .. } => {
            validate_expr(message, local_count, num_funcs, program, errors, true);
        }
        MirExpr::PropertySet { object, value, .. } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(value, local_count, num_funcs, program, errors, true);
        }
        MirExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(index, local_count, num_funcs, program, errors, true);
            validate_expr(value, local_count, num_funcs, program, errors, true);
        }
        MirExpr::New {
            constructor,
            prototype,
            args,
            base_local,
            private_brand,
            private_slot_count,
            ..
        } => {
            check_func_id(*constructor, num_funcs, errors);
            check_func_id(prototype.constructor, num_funcs, errors);
            for parent in &prototype.parent_constructors {
                check_func_id(*parent, num_funcs, errors);
            }
            check_local_id(*base_local, local_count, errors);
            if *private_slot_count > 0 && !private_brand.is_some_and(|brand| brand > 0) {
                errors.push(invariant(
                    "class instances with private slots must include a positive private brand",
                ));
            }
            if *private_slot_count > u16::MAX as usize {
                errors.push(invariant(
                    "class private slot count exceeds runtime metadata capacity",
                ));
            }
            validate_constructor_arity(*constructor, args, num_funcs, program, errors);
            for arg in args {
                validate_expr(arg, local_count, num_funcs, program, errors, true);
            }
        }
        MirExpr::ClassPrototype(prototype, _) => {
            check_func_id(prototype.constructor, num_funcs, errors);
            for parent in &prototype.parent_constructors {
                check_func_id(*parent, num_funcs, errors);
            }
        }
        MirExpr::ModuleLoad { module_id, .. } => {
            if program.modules.iter().all(|m| m.id != *module_id) && *module_id != 0 {
                errors.push(invariant(format!(
                    "ModuleLoad references module_id {} which is not in the program's module list",
                    module_id
                )));
            }
        }
        MirExpr::Block { stmts, result, .. } => {
            validate_stmts(
                stmts,
                local_count,
                num_funcs,
                program,
                ScopeKind::Function,
                errors,
            );
            validate_expr(
                result,
                local_count,
                num_funcs,
                program,
                errors,
                value_required,
            );
        }
        MirExpr::This(_) => {
            errors.push(invariant(
                "issue-211: residual `this` must be resolved to an active receiver local before backend emission",
            ));
        }
        MirExpr::ArrowFn {
            func_id, captures, ..
        } => {
            check_func_id(*func_id, num_funcs, errors);
            for capture in captures {
                check_local_id(*capture, local_count, errors);
            }
        }
    }
}

fn validate_call_kind(
    kind: FunctionCallKind,
    arg_count: usize,
    num_funcs: usize,
    program: &MirProgram,
    errors: &mut Vec<Diagnostic>,
    value_required: bool,
) {
    match kind {
        FunctionCallKind::User(func_id) => {
            if func_id.0 >= num_funcs {
                errors.push(invariant(format!(
                    "FuncId {} is out of range (program has {} function(s))",
                    func_id.0, num_funcs
                )));
            } else {
                let func = &program.functions[func_id.0];
                if arg_count < func.min_required_params {
                    errors.push(arity(format!(
                        "function {} expects at least {} argument(s), got {}",
                        func_id.0, func.min_required_params, arg_count
                    )));
                }
            }
        }
        FunctionCallKind::Builtin(builtin) => {
            let expected = builtin.expected_arity();
            let min_required = builtin.min_arity();
            if arg_count < min_required || arg_count > expected {
                errors.push(arity(format!(
                    "builtin {:?} expects {}-{} argument(s), got {}",
                    builtin, min_required, expected, arg_count
                )));
            }
            if value_required
                && matches!(builtin.result(), BuiltinResult::EffectOnly)
                && !matches!(builtin, BuiltinId::ConsoleLog)
            {
                errors.push(invariant(format!(
                    "builtin {:?} is effect-only and cannot be used in a value context",
                    builtin
                )));
            }
        }
    }
}

fn validate_runtime_call(intrinsic: RuntimeFn, args: &[MirExpr], errors: &mut Vec<Diagnostic>) {
    if intrinsic == RuntimeFn::ArrayPushMany && args.is_empty() {
        errors.push(invariant(
            "ArrayPushMany must include an array receiver argument",
        ));
    }
    if intrinsic == RuntimeFn::ArrayPushGrow && args.len() != 2 {
        errors.push(invariant(
            "ArrayPushGrow must include an array receiver and value",
        ));
    }
    if intrinsic == RuntimeFn::HeapClosureCall && args.is_empty() {
        errors.push(invariant(
            "HeapClosureCall must include a closure receiver argument",
        ));
    }
    if intrinsic == RuntimeFn::PrivateFieldGet
        && !matches!(args, [_, MirExpr::Number(brand, _), MirExpr::Number(slot, _)] if *brand > 0 && *slot >= 0)
    {
        errors.push(invariant(
            "PrivateFieldGet must include an object, positive private brand, and non-negative private slot",
        ));
    }
    if intrinsic == RuntimeFn::PrivateFieldSet
        && !matches!(args, [_, MirExpr::Number(brand, _), MirExpr::Number(slot, _), _] if *brand > 0 && *slot >= 0)
    {
        errors.push(invariant(
            "PrivateFieldSet must include an object, positive private brand, non-negative private slot, and value",
        ));
    }
    if intrinsic == RuntimeFn::PrivateBrandCheck
        && !matches!(args, [_, MirExpr::Number(brand, _)] if *brand > 0)
    {
        errors.push(invariant(
            "PrivateBrandCheck must include an object and positive private brand",
        ));
    }
    if intrinsic == RuntimeFn::ObjectDefineProperty && args.len() != 3 {
        errors.push(invariant(
            "ObjectDefineProperty must include an object, key, and descriptor",
        ));
    }
    if intrinsic == RuntimeFn::ObjectGetOwnPropertyDescriptor && args.len() != 2 {
        errors.push(invariant(
            "ObjectGetOwnPropertyDescriptor must include an object and key",
        ));
    }
    if intrinsic == RuntimeFn::ObjectGetPrototypeOf && args.len() != 1 {
        errors.push(invariant(
            "ObjectGetPrototypeOf must include an object argument",
        ));
    }
    if intrinsic == RuntimeFn::ObjectSetPrototypeOf && args.len() != 2 {
        errors.push(invariant(
            "ObjectSetPrototypeOf must include an object and prototype",
        ));
    }
}

fn check_local_id(id: LocalId, local_count: usize, errors: &mut Vec<Diagnostic>) {
    if id.0 >= local_count {
        errors.push(invariant(format!(
            "LocalId {} is out of range (scope has {} local(s))",
            id.0, local_count
        )));
    }
}

fn check_func_id(id: FuncId, num_funcs: usize, errors: &mut Vec<Diagnostic>) {
    if id.0 >= num_funcs {
        errors.push(invariant(format!(
            "FuncId {} is out of range (program has {} function(s))",
            id.0, num_funcs
        )));
    }
}

fn validate_constructor_arity(
    constructor: FuncId,
    args: &[MirExpr],
    num_funcs: usize,
    program: &MirProgram,
    errors: &mut Vec<Diagnostic>,
) {
    if constructor.0 >= num_funcs {
        return;
    }
    let func = &program.functions[constructor.0];
    let min_required = func.min_required_params.saturating_sub(1);
    if args.len() < min_required {
        errors.push(arity(format!(
            "constructor {} expects at least {} argument(s), got {}",
            constructor.0,
            min_required,
            args.len()
        )));
    } else if func.rest_param_index.is_none() {
        let max_allowed = func.params.len().saturating_sub(1);
        if args.len() > max_allowed {
            errors.push(arity(format!(
                "constructor {} expects between {} and {} argument(s), got {}",
                constructor.0,
                min_required,
                max_allowed,
                args.len()
            )));
        }
    }
}
