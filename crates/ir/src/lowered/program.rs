use crate::builtin_resolved::ResolvedArrayElement;
#[path = "program_builtins.rs"]
mod program_builtins;
#[path = "program_captures.rs"]
mod program_captures;
#[path = "program_direct_eval.rs"]
mod program_direct_eval;
use program_builtins::*;
use program_captures::*;
use program_direct_eval::*;
pub fn lower_program(program: &[ResolvedStmt]) -> Result<LoweredProgram, Diagnostic> {
    let function_ids = collect_function_ids(program)?;
    let generator_function_names = collect_generator_function_names(program);
    let function_signatures = collect_function_signatures(program, &function_ids);
    let top_level_local_names = collect_top_level_local_names(program)?;
    let map_callback_function_names = collect_array_map_callback_function_names(program);
    let function_captures = collect_callback_function_captures(
        program,
        &function_ids,
        &top_level_local_names,
        &map_callback_function_names,
    )?;
    let function_mutable_captures =
        collect_callback_function_mutable_captures(program, &function_captures);
    let class_method_captures = collect_class_method_captures(program, &function_ids);
    let class_method_mutable_captures =
        collect_class_method_mutable_captures(program, &function_ids);
    let mutable_class_capture_names = collect_mutable_class_capture_names(program);
    let direct_eval_env = collect_direct_eval_block_function_env(program);
    let env_cell_names = mutable_class_capture_names
        .union(&collect_mutable_function_capture_names(
            &function_mutable_captures,
        ))
        .cloned()
        .collect::<HashSet<_>>()
        .union(&direct_eval_env.env_cell_names)
        .cloned()
        .collect::<HashSet<_>>();
    let class_parents = collect_class_parents(program);
    let class_private_fields = collect_class_private_fields(program);
    let class_static_private_fields = collect_class_static_private_fields(program);
    let mut next_func_id = function_ids.len();
    let mut functions_by_id = vec![None; function_ids.len()];
    let mut generated_functions = Vec::new();

    for stmt in program {
        match stmt {
            ResolvedStmt::Function {
                name, params, body, ..
            } => {
                let func_id = function_ids[name];
                let params_with_captures = function_params_with_captures(
                    params,
                    function_captures
                        .get(&func_id)
                        .map(Vec::as_slice)
                        .unwrap_or(&[]),
                );
                let function_env_cell_names = function_mutable_captures
                    .get(&func_id)
                    .map(|names| names.iter().cloned().collect::<HashSet<_>>())
                    .unwrap_or_default();
                let lowered = lower_function(
                    func_id,
                    &params_with_captures,
                    body,
                    &function_ids,
                    &function_signatures,
                    &function_captures,
                    &function_mutable_captures,
                    &class_method_captures,
                    &class_method_mutable_captures,
                    &function_env_cell_names,
                    &HashSet::new(),
                    class_parents.clone(),
                    class_private_fields.clone(),
                    class_static_private_fields.clone(),
                    LowerFunctionOptions {
                        current_class: None,
                        in_constructor: false,
                        next_func_id,
                        self_closure: None,
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

                let mut ctor_params_with_this: Vec<ResolvedParam> = vec![ResolvedParam {
                    name: "this".to_owned(),
                    default: None,
                    is_rest: false,
                    span: None,
                }];
                ctor_params_with_this.extend(ctor_params.clone());

                let lowered = lower_function(
                    ctor_id,
                    &ctor_params_with_this,
                    &ctor_body,
                    &function_ids,
                    &function_signatures,
                    &function_captures,
                    &function_mutable_captures,
                    &class_method_captures,
                    &class_method_mutable_captures,
                    &HashSet::new(),
                    &HashSet::new(),
                    class_parents.clone(),
                    class_private_fields.clone(),
                    class_static_private_fields.clone(),
                    LowerFunctionOptions {
                        current_class: Some(name),
                        in_constructor: true,
                        next_func_id,
                        self_closure: None,
                    },
                )?;
                next_func_id = lowered.next_func_id;
                functions_by_id[ctor_id.0] = Some(lowered.function);
                generated_functions.extend(lowered.generated_functions);

                for method in methods {
                    let method_key = class_method_key(name, &method.name);
                    let method_id = function_ids[&method_key];
                    let mut method_params_with_this: Vec<ResolvedParam> =
                        if method.name.starts_with("static::") {
                            method.params.clone()
                        } else {
                            let mut params = vec![ResolvedParam {
                                name: "this".to_owned(),
                                default: None,
                                is_rest: false,
                                span: None,
                            }];
                            params.extend(method.params.clone());
                            params
                        };
                    method_params_with_this.extend(method.captures.iter().map(|name| {
                        ResolvedParam {
                            name: name.clone(),
                            default: None,
                            is_rest: false,
                            span: None,
                        }
                    }));
                    let method_env_cell_names = class_method_mutable_captures
                        .get(&method_id)
                        .map(|names| names.iter().cloned().collect::<HashSet<_>>())
                        .unwrap_or_default();
                    let lowered = lower_function(
                        method_id,
                        &method_params_with_this,
                        &method.body,
                        &function_ids,
                        &function_signatures,
                        &function_captures,
                        &function_mutable_captures,
                        &class_method_captures,
                        &class_method_mutable_captures,
                        &method_env_cell_names,
                        &HashSet::new(),
                        class_parents.clone(),
                        class_private_fields.clone(),
                        class_static_private_fields.clone(),
                        LowerFunctionOptions {
                            current_class: Some(name),
                            in_constructor: false,
                            next_func_id,
                            self_closure: None,
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

    let mut resolver = Resolver::new(
        &function_ids,
        &function_signatures,
        &function_captures,
        &function_mutable_captures,
        &class_method_captures,
        &class_method_mutable_captures,
        &env_cell_names,
        &direct_eval_env.heap_closure_names,
        class_parents.clone(),
        class_private_fields,
        class_static_private_fields,
        generator_function_names,
        next_func_id,
    );
    let mut top_level_statements = Vec::new();
    for stmt in program {
        match stmt {
            ResolvedStmt::Function { .. } => {}
            ResolvedStmt::ClassDecl {
                name,
                extends,
                constructor: _,
                methods,
                private_fields,
                static_private_fields,
                static_blocks,
                ..
            } => {
                // Look up FuncIds for constructor and methods (pre-computed in phase 1)
                // constructor is always Some because Phase 1 always allocates a FuncId
                let ctor_key = class_constructor_key(name);
                let ctor_id = Some(function_ids[&ctor_key]);
                let mut instance_methods = Vec::new();
                let mut static_methods = Vec::new();
                for method in methods {
                    let key = class_method_key(name, &method.name);
                    let method_id = function_ids[&key];
                    if let Some(stripped) = method.name.strip_prefix("static::") {
                        static_methods.push((stripped.to_owned(), method_id));
                    } else {
                        instance_methods.push((method.name.clone(), method_id));
                    }
                }

                // Emit class binding first (JS semantics: class visible before static init)
                top_level_statements.push(LoweredStmt::ClassDecl {
                    name: name.clone(),
                    extends: extends.clone(),
                    constructor: ctor_id,
                    methods: instance_methods,
                    static_methods,
                    private_fields: private_fields.clone(),
                });
                let mut initializers = Vec::new();
                for (field, initializer, span) in static_private_fields {
                    initializers.push(ClassStaticInitializer::PrivateField {
                        span_start: span.start,
                        field,
                        initializer,
                    });
                }
                for (span, block) in static_blocks {
                    initializers.push(ClassStaticInitializer::Block {
                        span_start: span.start,
                        block,
                    });
                }
                initializers.sort_by_key(ClassStaticInitializer::span_start);
                for initializer in initializers {
                    match initializer {
                        ClassStaticInitializer::PrivateField {
                            field, initializer, ..
                        } => {
                            top_level_statements.push(
                                resolver.lower_class_static_private_field(name, field, initializer)?,
                            );
                        }
                        ClassStaticInitializer::Block { block, .. } => {
                            top_level_statements
                                .extend(resolver.lower_class_static_block(name, block)?);
                        }
                    }
                }
            }
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

enum ClassStaticInitializer<'a> {
    PrivateField {
        span_start: usize,
        field: &'a str,
        initializer: &'a ResolvedExpr,
    },
    Block {
        span_start: usize,
        block: &'a [ResolvedStmt],
    },
}

impl ClassStaticInitializer<'_> {
    fn span_start(&self) -> usize {
        match self {
            Self::PrivateField { span_start, .. } | Self::Block { span_start, .. } => *span_start,
        }
    }
}

struct FunctionLowering {
    function: LoweredFunction,
    generated_functions: Vec<LoweredFunction>,
    next_func_id: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct FunctionSignature {
    explicit_params: usize,
    needs_receiver: bool,
    needs_arguments: bool,
    has_rest: bool,
    metadata_length: Option<usize>,
    returns_heap_closure: bool,
    returns_dense_array: bool,
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

fn collect_generator_function_names(program: &[ResolvedStmt]) -> HashSet<String> {
    program
        .iter()
        .filter_map(|stmt| match stmt {
            ResolvedStmt::Function {
                name,
                is_generator: true,
                ..
            } => Some(name.clone()),
            _ => None,
        })
        .collect()
}

fn collect_top_level_local_names(program: &[ResolvedStmt]) -> Result<HashSet<String>, Diagnostic> {
    let mut names = HashSet::new();
    for stmt in program {
        match stmt {
            ResolvedStmt::Let(name, _) => {
                if let Some(pattern) = parse_binding_pattern(name, None)? {
                    names.extend(pattern.names().into_iter().map(ToOwned::to_owned));
                } else {
                    names.insert(name.clone());
                }
            }
            ResolvedStmt::DestructureLet { pattern, .. } => {
                names.extend(pattern.names().into_iter().map(ToOwned::to_owned));
            }
            _ => {}
        }
    }
    Ok(names)
}

fn collect_array_map_callback_function_names(program: &[ResolvedStmt]) -> HashSet<String> {
    let mut names = HashSet::new();
    for stmt in program {
        collect_array_map_callback_function_names_in_stmt(stmt, &mut names);
    }
    names
}

fn collect_array_map_callback_function_names_in_stmt(
    stmt: &ResolvedStmt,
    names: &mut HashSet<String>,
) {
    match stmt {
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::Assign(_, expr)
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr) => {
            collect_array_map_callback_function_names_in_expr(expr, names);
        }
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            collect_array_map_callback_function_names_in_expr(condition, names);
            for stmt in then_body.iter().chain(else_body) {
                collect_array_map_callback_function_names_in_stmt(stmt, names);
            }
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
            collect_array_map_callback_function_names_in_expr(condition, names);
            for stmt in body {
                collect_array_map_callback_function_names_in_stmt(stmt, names);
            }
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            if let Some(init) = init {
                collect_array_map_callback_function_names_in_stmt(init, names);
            }
            if let Some(condition) = condition {
                collect_array_map_callback_function_names_in_expr(condition, names);
            }
            if let Some(update) = update {
                collect_array_map_callback_function_names_in_expr(update, names);
            }
            for stmt in body {
                collect_array_map_callback_function_names_in_stmt(stmt, names);
            }
        }
        ResolvedStmt::ForIn { iter, body, .. } | ResolvedStmt::ForOf { iter, body, .. } => {
            collect_array_map_callback_function_names_in_expr(iter, names);
            for stmt in body {
                collect_array_map_callback_function_names_in_stmt(stmt, names);
            }
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            for stmt in try_block {
                collect_array_map_callback_function_names_in_stmt(stmt, names);
            }
            if let Some(block) = catch_block {
                for stmt in block {
                    collect_array_map_callback_function_names_in_stmt(stmt, names);
                }
            }
            if let Some(block) = finally_block {
                for stmt in block {
                    collect_array_map_callback_function_names_in_stmt(stmt, names);
                }
            }
        }
        ResolvedStmt::Switch { expr, cases } => {
            collect_array_map_callback_function_names_in_expr(expr, names);
            for (case_expr, body) in cases {
                if let Some(case_expr) = case_expr {
                    collect_array_map_callback_function_names_in_expr(case_expr, names);
                }
                for stmt in body {
                    collect_array_map_callback_function_names_in_stmt(stmt, names);
                }
            }
        }
        ResolvedStmt::Labeled { body, .. } => {
            collect_array_map_callback_function_names_in_stmt(body, names);
        }
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            collect_array_map_callback_function_names_in_expr(expr, names);
        }
        ResolvedStmt::Block { statements, .. } => {
            for stmt in statements {
                collect_array_map_callback_function_names_in_stmt(stmt, names);
            }
        }
        ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => {}
    }
}

fn collect_array_map_callback_function_names_in_expr(
    expr: &ResolvedExpr,
    names: &mut HashSet<String>,
) {
    match expr {
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } => {
            collect_array_map_callback_function_names_in_expr(object, names);
            if method == "map"
                && let Some(ResolvedExpr::Ident(callback)) = args.first()
            {
                names.insert(callback.clone());
            }
            for arg in args {
                collect_array_map_callback_function_names_in_expr(arg, names);
            }
        }
        ResolvedExpr::Unary { expr, .. }
        | ResolvedExpr::Spread(expr)
        | ResolvedExpr::BuiltinProperty { object: expr, .. }
        | ResolvedExpr::PropertyAccess { object: expr, .. }
        | ResolvedExpr::OptionalPropertyAccess { object: expr, .. } => {
            collect_array_map_callback_function_names_in_expr(expr, names);
        }
        ResolvedExpr::Binary { left, right, .. }
        | ResolvedExpr::ComputedIndex {
            object: left,
            index: right,
        } => {
            collect_array_map_callback_function_names_in_expr(left, names);
            collect_array_map_callback_function_names_in_expr(right, names);
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            collect_array_map_callback_function_names_in_expr(callee, names);
            for arg in args {
                collect_array_map_callback_function_names_in_expr(arg, names);
            }
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
            collect_array_map_callback_function_names_in_expr(expr, names);
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            collect_array_map_callback_function_names_in_expr(object, names);
            collect_array_map_callback_function_names_in_expr(expr, names);
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            collect_array_map_callback_function_names_in_expr(key, names);
            collect_array_map_callback_function_names_in_expr(expr, names);
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            collect_array_map_callback_function_names_in_expr(object, names);
            collect_array_map_callback_function_names_in_expr(key, names);
            collect_array_map_callback_function_names_in_expr(expr, names);
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                if let ResolvedArrayElement::Present(expr) = element {
                    collect_array_map_callback_function_names_in_expr(expr, names);
                }
            }
        }
        ResolvedExpr::Object(props) => {
            for (_, value) in props {
                collect_array_map_callback_function_names_in_expr(value, names);
            }
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            for arg in args {
                collect_array_map_callback_function_names_in_expr(arg, names);
            }
        }
        ResolvedExpr::OptionalComputedIndex { object, index, .. }
        | ResolvedExpr::PropertyAssignDynamic {
            object,
            key: index,
            ..
        } => {
            collect_array_map_callback_function_names_in_expr(object, names);
            collect_array_map_callback_function_names_in_expr(index, names);
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            collect_array_map_callback_function_names_in_expr(object, names);
            collect_array_map_callback_function_names_in_expr(value, names);
        }
        ResolvedExpr::ArrowFn { body, .. } => {
            collect_array_map_callback_function_names_in_expr(body, names);
        }
        ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::This { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. } => {}
    }
}

fn collect_callback_function_captures(
    program: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
    top_level_local_names: &HashSet<String>,
    callback_names: &HashSet<String>,
) -> Result<HashMap<FuncId, Vec<String>>, Diagnostic> {
    let mut captures = HashMap::new();
    if callback_names.is_empty() || top_level_local_names.is_empty() {
        return Ok(captures);
    }

    for stmt in program {
        let ResolvedStmt::Function {
            name, params, body, ..
        } = stmt
        else {
            continue;
        };
        if !callback_names.contains(name) {
            continue;
        }

        let mut excluded = HashSet::new();
        excluded.insert(name.clone());
        for param in params {
            if let Some(pattern) = parse_binding_pattern(&param.name, param.span)? {
                excluded.extend(pattern.names().into_iter().map(ToOwned::to_owned));
            } else {
                excluded.insert(param.name.clone());
            }
        }
        collect_declared_names_in_stmts(body, &mut excluded);

        let mut found = Vec::new();
        collect_stmt_captures(body, &excluded, &mut found);
        let found = found
            .into_iter()
            .filter(|capture| top_level_local_names.contains(capture))
            .collect::<Vec<_>>();
        if !found.is_empty() {
            captures.insert(function_ids[name], found);
        }
    }

    Ok(captures)
}

fn collect_callback_function_mutable_captures(
    program: &[ResolvedStmt],
    function_captures: &HashMap<FuncId, Vec<String>>,
) -> HashMap<FuncId, Vec<String>> {
    let mut mutable_captures = HashMap::new();
    if function_captures.is_empty() {
        return mutable_captures;
    }

    let function_ids = collect_function_ids(program).unwrap_or_default();
    for stmt in program {
        let ResolvedStmt::Function { name, body, .. } = stmt else {
            continue;
        };
        let Some(func_id) = function_ids.get(name).copied() else {
            continue;
        };
        let Some(captures) = function_captures.get(&func_id) else {
            continue;
        };
        let mutable = captures
            .iter()
            .filter(|capture| block_assigns_any_name(body, std::slice::from_ref(capture)))
            .cloned()
            .collect::<Vec<_>>();
        if !mutable.is_empty() {
            mutable_captures.insert(func_id, mutable);
        }
    }

    mutable_captures
}

fn collect_mutable_function_capture_names(
    function_mutable_captures: &HashMap<FuncId, Vec<String>>,
) -> HashSet<String> {
    function_mutable_captures
        .values()
        .flat_map(|captures| captures.iter().cloned())
        .collect()
}

fn function_params_with_captures(
    params: &[ResolvedParam],
    captures: &[String],
) -> Vec<ResolvedParam> {
    let mut params = params.to_vec();
    params.extend(captures.iter().map(|capture| ResolvedParam {
        name: capture.clone(),
        default: None,
        is_rest: false,
        span: None,
    }));
    params
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

fn collect_class_private_fields(program: &[ResolvedStmt]) -> ClassPrivateFieldSlots {
    let mut fields = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::ClassDecl {
            name,
            private_fields,
            ..
        } = stmt
        {
            fields.insert(
                name.clone(),
                private_fields
                    .iter()
                    .enumerate()
                    .map(|(slot, field)| (field.clone(), slot))
                    .collect(),
            );
        }
    }
    fields
}

fn collect_class_static_private_fields(program: &[ResolvedStmt]) -> ClassStaticPrivateFields {
    let mut fields = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::ClassDecl {
            name,
            static_private_fields,
            ..
        } = stmt
        {
            fields.insert(
                name.clone(),
                static_private_fields
                    .iter()
                    .map(|(field, _, _)| {
                        (
                            field.clone(),
                            crate::builtin_resolver::static_private_field_local_name(name, field),
                        )
                    })
                    .collect(),
            );
        }
    }
    fields
}

fn collect_function_signatures(
    program: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
) -> HashMap<FuncId, FunctionSignature> {
    let mut signatures = HashMap::new();

    for stmt in program {
        match stmt {
            ResolvedStmt::Function {
                name, params, body, ..
            } => {
                signatures.insert(
                    function_ids[name],
                    FunctionSignature {
                        explicit_params: params.len(),
                        needs_receiver: block_contains_this(body),
                        needs_arguments: block_contains_arguments(body)
                            && !params.iter().any(|param| param.name == "arguments"),
                        has_rest: params.iter().any(|param| param.is_rest),
                        metadata_length: fixed_arity_metadata_length(params),
                        returns_heap_closure: block_returns_declared_function(body),
                        returns_dense_array: block_returns_dense_array_local(body),
                    },
                );
            }
            ResolvedStmt::ClassDecl {
                name,
                constructor,
                methods,
                ..
            } => {
                let ctor_key = class_constructor_key(name);
                let ctor_params_len = constructor
                    .as_ref()
                    .map(|(params, _)| params.len())
                    .unwrap_or_default()
                    + 1;
                let ctor_has_rest = constructor
                    .as_ref()
                    .is_some_and(|(params, _)| params.iter().any(|param| param.is_rest));
                let ctor_returns_heap_closure = constructor
                    .as_ref()
                    .is_some_and(|(_, body)| block_returns_declared_function(body));
                let ctor_returns_dense_array = constructor
                    .as_ref()
                    .is_some_and(|(_, body)| block_returns_dense_array_local(body));
                signatures.insert(
                    function_ids[&ctor_key],
                    FunctionSignature {
                        explicit_params: ctor_params_len,
                        has_rest: ctor_has_rest,
                        returns_heap_closure: ctor_returns_heap_closure,
                        returns_dense_array: ctor_returns_dense_array,
                        ..FunctionSignature::default()
                    },
                );

                for method in methods {
                    let method_key = class_method_key(name, &method.name);
                    let receiver_param_count = usize::from(!method.name.starts_with("static::"));
                    signatures.insert(
                        function_ids[&method_key],
                        FunctionSignature {
                            explicit_params: method.params.len() + receiver_param_count,
                            has_rest: method.params.iter().any(|param| param.is_rest),
                            returns_heap_closure: block_returns_declared_function(&method.body),
                            returns_dense_array: block_returns_dense_array_local(&method.body),
                            ..FunctionSignature::default()
                        },
                    );
                }
            }
            _ => {}
        }
    }

    signatures
}

fn collect_class_method_captures(
    program: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
) -> HashMap<FuncId, Vec<String>> {
    let mut captures = HashMap::new();

    for stmt in program {
        if let ResolvedStmt::ClassDecl { name, methods, .. } = stmt {
            for method in methods {
                if method.captures.is_empty() {
                    continue;
                }
                let method_key = class_method_key(name, &method.name);
                if let Some(func_id) = function_ids.get(&method_key) {
                    captures.insert(*func_id, method.captures.clone());
                }
            }
        }
    }

    captures
}

fn collect_class_method_mutable_captures(
    program: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
) -> HashMap<FuncId, Vec<String>> {
    let mut captures = HashMap::new();

    for stmt in program {
        if let ResolvedStmt::ClassDecl { name, methods, .. } = stmt {
            for method in methods {
                let mut mutable = Vec::new();
                for capture in &method.captures {
                    if is_static_private_field_local_name(capture)
                        || block_assigns_any_name(&method.body, std::slice::from_ref(capture))
                    {
                        mutable.push(capture.clone());
                    }
                }
                if mutable.is_empty() {
                    continue;
                }
                let method_key = class_method_key(name, &method.name);
                if let Some(func_id) = function_ids.get(&method_key) {
                    captures.insert(*func_id, mutable);
                }
            }
        }
    }

    captures
}

fn collect_mutable_class_capture_names(program: &[ResolvedStmt]) -> HashSet<String> {
    let mut names = HashSet::new();
    for stmt in program {
        if let ResolvedStmt::ClassDecl { methods, .. } = stmt {
            for method in methods {
                for capture in &method.captures {
                    if is_static_private_field_local_name(capture)
                        || block_assigns_any_name(&method.body, std::slice::from_ref(capture))
                    {
                        names.insert(capture.clone());
                    }
                }
            }
        }
    }
    names
}

fn is_static_private_field_local_name(name: &str) -> bool {
    name.starts_with("__ts2wasm_static_private::")
}

#[derive(Default)]
struct DirectEvalBlockFunctionEnv {
    env_cell_names: HashSet<String>,
    heap_closure_names: HashSet<String>,
}

fn block_contains_this(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_contains_this)
}

fn fixed_arity_metadata_length(params: &[ResolvedParam]) -> Option<usize> {
    if params
        .iter()
        .any(|param| param.default.is_some() || param.is_rest)
    {
        None
    } else {
        Some(params.len())
    }
}

fn block_returns_declared_function(stmts: &[ResolvedStmt]) -> bool {
    let mut function_names = HashSet::new();
    collect_declared_function_names(stmts, &mut function_names);
    !function_names.is_empty() && block_returns_any_name(stmts, &function_names)
}

fn block_returns_dense_array_local(stmts: &[ResolvedStmt]) -> bool {
    let mut dense_locals = HashSet::new();
    let mut saw_return = false;
    let mut all_returns_dense = true;
    scan_dense_array_returns(
        stmts,
        &mut dense_locals,
        &mut saw_return,
        &mut all_returns_dense,
    );
    saw_return && all_returns_dense
}

fn scan_dense_array_returns(
    stmts: &[ResolvedStmt],
    dense_locals: &mut HashSet<String>,
    saw_return: &mut bool,
    all_returns_dense: &mut bool,
) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Let(name, expr) | ResolvedStmt::Assign(name, expr) => {
                if expr_is_dense_array_seed(expr) {
                    dense_locals.insert(name.clone());
                } else {
                    dense_locals.remove(name);
                }
            }
            ResolvedStmt::Expr(expr) => {
                if let Some(receiver) = pushed_dense_array_local(expr)
                    && dense_locals.contains(receiver)
                {
                    continue;
                }
            }
            ResolvedStmt::Return(expr) => {
                *saw_return = true;
                if !expr_is_known_dense_array_return(expr, dense_locals) {
                    *all_returns_dense = false;
                }
            }
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                scan_dense_array_returns(then_body, dense_locals, saw_return, all_returns_dense);
                scan_dense_array_returns(else_body, dense_locals, saw_return, all_returns_dense);
            }
            ResolvedStmt::While { body, .. }
            | ResolvedStmt::DoWhile { body, .. }
            | ResolvedStmt::ForIn { body, .. }
            | ResolvedStmt::ForOf { body, .. } => {
                scan_dense_array_returns(body, dense_locals, saw_return, all_returns_dense);
            }
            ResolvedStmt::For { init, body, .. } => {
                if let Some(init) = init {
                    scan_dense_array_returns(
                        std::slice::from_ref(init.as_ref()),
                        dense_locals,
                        saw_return,
                        all_returns_dense,
                    );
                }
                scan_dense_array_returns(body, dense_locals, saw_return, all_returns_dense);
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                scan_dense_array_returns(try_block, dense_locals, saw_return, all_returns_dense);
                if let Some(block) = catch_block {
                    scan_dense_array_returns(block, dense_locals, saw_return, all_returns_dense);
                }
                if let Some(block) = finally_block {
                    scan_dense_array_returns(block, dense_locals, saw_return, all_returns_dense);
                }
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    scan_dense_array_returns(body, dense_locals, saw_return, all_returns_dense);
                }
            }
            ResolvedStmt::Labeled { body, .. } => {
                scan_dense_array_returns(
                    std::slice::from_ref(body.as_ref()),
                    dense_locals,
                    saw_return,
                    all_returns_dense,
                );
            }
            ResolvedStmt::Block { statements, .. } => {
                scan_dense_array_returns(statements, dense_locals, saw_return, all_returns_dense);
            }
            ResolvedStmt::Function { .. }
            | ResolvedStmt::DestructureLet { .. }
            | ResolvedStmt::Throw(_)
            | ResolvedStmt::Export { .. }
            | ResolvedStmt::ModuleExportsAssign { .. }
            | ResolvedStmt::ClassDecl { .. }
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. } => {}
        }
    }
}

fn expr_is_dense_array_seed(expr: &ResolvedExpr) -> bool {
    matches!(expr, ResolvedExpr::Array(_))
}

fn pushed_dense_array_local(expr: &ResolvedExpr) -> Option<&str> {
    let ResolvedExpr::MethodCall { object, method, .. } = expr else {
        return None;
    };
    if method != "push" {
        return None;
    }
    let ResolvedExpr::Ident(name) = object.as_ref() else {
        return None;
    };
    Some(name)
}

fn expr_is_known_dense_array_return(expr: &ResolvedExpr, dense_locals: &HashSet<String>) -> bool {
    match expr {
        ResolvedExpr::Array(_) => true,
        ResolvedExpr::Ident(name) => dense_locals.contains(name),
        _ => false,
    }
}

fn collect_declared_function_names(stmts: &[ResolvedStmt], names: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Function { name, .. } => {
                names.insert(name.clone());
            }
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_declared_function_names(then_body, names);
                collect_declared_function_names(else_body, names);
            }
            ResolvedStmt::While { body, .. }
            | ResolvedStmt::DoWhile { body, .. }
            | ResolvedStmt::ForIn { body, .. }
            | ResolvedStmt::ForOf { body, .. } => collect_declared_function_names(body, names),
            ResolvedStmt::For { init, body, .. } => {
                if let Some(init) = init {
                    collect_declared_function_names(std::slice::from_ref(init.as_ref()), names);
                }
                collect_declared_function_names(body, names);
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_declared_function_names(try_block, names);
                if let Some(block) = catch_block {
                    collect_declared_function_names(block, names);
                }
                if let Some(block) = finally_block {
                    collect_declared_function_names(block, names);
                }
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_declared_function_names(body, names);
                }
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_declared_function_names(std::slice::from_ref(body.as_ref()), names);
            }
            ResolvedStmt::Block { statements, .. } => {
                collect_declared_function_names(statements, names);
            }
            ResolvedStmt::Let(_, _)
            | ResolvedStmt::DestructureLet { .. }
            | ResolvedStmt::Assign(_, _)
            | ResolvedStmt::Expr(_)
            | ResolvedStmt::Return(_)
            | ResolvedStmt::Throw(_)
            | ResolvedStmt::Export { .. }
            | ResolvedStmt::ModuleExportsAssign { .. }
            | ResolvedStmt::ClassDecl { .. }
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. } => {}
        }
    }
}

fn block_returns_any_name(stmts: &[ResolvedStmt], names: &HashSet<String>) -> bool {
    stmts.iter().any(|stmt| stmt_returns_any_name(stmt, names))
}

fn stmt_returns_any_name(stmt: &ResolvedStmt, names: &HashSet<String>) -> bool {
    match stmt {
        ResolvedStmt::Return(ResolvedExpr::Ident(name)) => names.contains(name),
        ResolvedStmt::If {
            then_body,
            else_body,
            ..
        } => block_returns_any_name(then_body, names) || block_returns_any_name(else_body, names),
        ResolvedStmt::While { body, .. }
        | ResolvedStmt::DoWhile { body, .. }
        | ResolvedStmt::ForIn { body, .. }
        | ResolvedStmt::ForOf { body, .. } => block_returns_any_name(body, names),
        ResolvedStmt::For { init, body, .. } => {
            init.as_ref()
                .is_some_and(|stmt| stmt_returns_any_name(stmt, names))
                || block_returns_any_name(body, names)
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_returns_any_name(try_block, names)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block_returns_any_name(block, names))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block_returns_any_name(block, names))
        }
        ResolvedStmt::Switch { cases, .. } => cases
            .iter()
            .any(|(_, body)| block_returns_any_name(body, names)),
        ResolvedStmt::Labeled { body, .. } => stmt_returns_any_name(body, names),
        ResolvedStmt::Block { statements, .. } => block_returns_any_name(statements, names),
        ResolvedStmt::Function { .. }
        | ResolvedStmt::Let(_, _)
        | ResolvedStmt::DestructureLet { .. }
        | ResolvedStmt::Assign(_, _)
        | ResolvedStmt::Expr(_)
        | ResolvedStmt::Return(_)
        | ResolvedStmt::Throw(_)
        | ResolvedStmt::Export { .. }
        | ResolvedStmt::ModuleExportsAssign { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

fn stmt_contains_this(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::Assign(_, expr)
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr) => expr_contains_this(expr),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_contains_this(condition)
                || block_contains_this(then_body)
                || block_contains_this(else_body)
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
            expr_contains_this(condition) || block_contains_this(body)
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_contains_this(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block_contains_this(block))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block_contains_this(block))
        }
        ResolvedStmt::Switch { expr, cases } => {
            expr_contains_this(expr)
                || cases.iter().any(|(case_expr, body)| {
                    case_expr.as_ref().is_some_and(expr_contains_this)
                        || block_contains_this(body)
                })
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            init.as_ref().is_some_and(|stmt| stmt_contains_this(stmt))
                || condition.as_ref().is_some_and(expr_contains_this)
                || update.as_ref().is_some_and(expr_contains_this)
                || block_contains_this(body)
        }
        ResolvedStmt::ForIn { iter, body, .. } | ResolvedStmt::ForOf { iter, body, .. } => {
            expr_contains_this(iter) || block_contains_this(body)
        }
        ResolvedStmt::Labeled { body, .. } => stmt_contains_this(body),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_contains_this(expr)
        }
        ResolvedStmt::Block { statements, .. } => block_contains_this(statements),
        ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

fn expr_contains_this(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::This { .. } => true,
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => expr_contains_this(expr),
        ResolvedExpr::Binary { left, right, .. } => {
            expr_contains_this(left) || expr_contains_this(right)
        }
        ResolvedExpr::Call { callee, args, .. } => {
            expr_contains_this(callee) || args.iter().any(expr_contains_this)
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => expr_contains_this(expr),
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_contains_this(object) || expr_contains_this(expr)
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_contains_this(key) || expr_contains_this(expr)
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => expr_contains_this(object) || expr_contains_this(key) || expr_contains_this(expr),
        ResolvedExpr::Array(elements) => elements.iter().any(|element| match element {
            ResolvedArrayElement::Present(expr) => expr_contains_this(expr),
            ResolvedArrayElement::Hole => false,
        }),
        ResolvedExpr::Object(props) => props.iter().any(|(_, value)| expr_contains_this(value)),
        ResolvedExpr::ComputedIndex { object, index } => {
            expr_contains_this(object) || expr_contains_this(index)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(expr_contains_this)
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => expr_contains_this(object),
        ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            expr_contains_this(object) || expr_contains_this(index)
        }
        ResolvedExpr::OptionalCall { callee, args, .. } => {
            expr_contains_this(callee) || args.iter().any(expr_contains_this)
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            expr_contains_this(object) || args.iter().any(expr_contains_this)
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            expr_contains_this(object) || expr_contains_this(value)
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            expr_contains_this(object) || expr_contains_this(key) || expr_contains_this(value)
        }
        ResolvedExpr::ArrowFn { body, .. } => expr_contains_this(body),
        ResolvedExpr::FunctionExpr { .. } => false,
        ResolvedExpr::ClassExpr { .. } => false,
        ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. } => false,
    }
}

fn block_contains_arguments(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_contains_arguments)
}

fn direct_iife_body_has_static_eval_block_function_binding(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(|stmt| {
        matches!(
            stmt,
            ResolvedStmt::Let(_, ResolvedExpr::Undefined) | ResolvedStmt::Function { .. }
        )
    })
}

fn direct_iife_body_has_unsupported_return(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_has_direct_return)
}

fn stmt_has_direct_return(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Return(_) => true,
        ResolvedStmt::If {
            then_body,
            else_body,
            ..
        } => {
            direct_iife_body_has_unsupported_return(then_body)
                || direct_iife_body_has_unsupported_return(else_body)
        }
        ResolvedStmt::While { body, .. } | ResolvedStmt::DoWhile { body, .. } => {
            direct_iife_body_has_unsupported_return(body)
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            direct_iife_body_has_unsupported_return(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| direct_iife_body_has_unsupported_return(block))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| direct_iife_body_has_unsupported_return(block))
        }
        ResolvedStmt::Switch { cases, .. } => cases
            .iter()
            .any(|(_, body)| direct_iife_body_has_unsupported_return(body)),
        ResolvedStmt::For { body, .. }
        | ResolvedStmt::ForIn { body, .. }
        | ResolvedStmt::ForOf { body, .. } => direct_iife_body_has_unsupported_return(body),
        ResolvedStmt::Labeled { body, .. } => stmt_has_direct_return(body),
        ResolvedStmt::Block { statements, .. } => {
            direct_iife_body_has_unsupported_return(statements)
        }
        ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Let(_, _)
        | ResolvedStmt::DestructureLet { .. }
        | ResolvedStmt::Assign(_, _)
        | ResolvedStmt::Expr(_)
        | ResolvedStmt::Throw(_)
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. }
        | ResolvedStmt::Export { .. }
        | ResolvedStmt::ModuleExportsAssign { .. } => false,
    }
}

fn stmt_contains_arguments(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::Assign(_, expr)
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr) => expr_contains_arguments(expr),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_contains_arguments(condition)
                || block_contains_arguments(then_body)
                || block_contains_arguments(else_body)
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
            expr_contains_arguments(condition) || block_contains_arguments(body)
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_contains_arguments(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block_contains_arguments(block))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block_contains_arguments(block))
        }
        ResolvedStmt::Switch { expr, cases } => {
            expr_contains_arguments(expr)
                || cases.iter().any(|(case_expr, body)| {
                    case_expr.as_ref().is_some_and(expr_contains_arguments)
                        || block_contains_arguments(body)
                })
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            init.as_ref().is_some_and(|stmt| stmt_contains_arguments(stmt))
                || condition.as_ref().is_some_and(expr_contains_arguments)
                || update.as_ref().is_some_and(expr_contains_arguments)
                || block_contains_arguments(body)
        }
        ResolvedStmt::ForIn { iter, body, .. } | ResolvedStmt::ForOf { iter, body, .. } => {
            expr_contains_arguments(iter) || block_contains_arguments(body)
        }
        ResolvedStmt::Labeled { body, .. } => stmt_contains_arguments(body),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_contains_arguments(expr)
        }
        ResolvedStmt::Block { statements, .. } => block_contains_arguments(statements),
        ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

fn expr_contains_arguments(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Ident(name) => name == "arguments",
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => {
            expr_contains_arguments(expr)
        }
        ResolvedExpr::Binary { left, right, .. } => {
            expr_contains_arguments(left) || expr_contains_arguments(right)
        }
        ResolvedExpr::Call { callee, args, .. } => {
            expr_contains_arguments(callee) || args.iter().any(expr_contains_arguments)
        }
        ResolvedExpr::Assign { name, expr } | ResolvedExpr::LogicalAssign { name, expr, .. } => {
            name == "arguments" || expr_contains_arguments(expr)
        }
        ResolvedExpr::LogicalPropertyAssign { object, expr, .. } => {
            object == "arguments" || expr_contains_arguments(expr)
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_contains_arguments(object) || expr_contains_arguments(expr)
        }
        ResolvedExpr::LogicalComputedPropertyAssign { object, key, expr, .. } => {
            object == "arguments" || expr_contains_arguments(key) || expr_contains_arguments(expr)
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            expr_contains_arguments(object)
                || expr_contains_arguments(key)
                || expr_contains_arguments(expr)
        }
        ResolvedExpr::Array(elements) => elements.iter().any(|element| match element {
            ResolvedArrayElement::Present(expr) => expr_contains_arguments(expr),
            ResolvedArrayElement::Hole => false,
        }),
        ResolvedExpr::Object(props) => {
            props.iter().any(|(_, value)| expr_contains_arguments(value))
        }
        ResolvedExpr::ComputedIndex { object, index } => {
            expr_contains_arguments(object) || expr_contains_arguments(index)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(expr_contains_arguments)
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => expr_contains_arguments(object),
        ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            expr_contains_arguments(object) || expr_contains_arguments(index)
        }
        ResolvedExpr::OptionalCall { callee, args, .. } => {
            expr_contains_arguments(callee) || args.iter().any(expr_contains_arguments)
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            expr_contains_arguments(object) || args.iter().any(expr_contains_arguments)
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            expr_contains_arguments(object) || expr_contains_arguments(value)
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            expr_contains_arguments(object)
                || expr_contains_arguments(key)
                || expr_contains_arguments(value)
        }
        ResolvedExpr::ArrowFn { body, .. } => expr_contains_arguments(body),
        ResolvedExpr::FunctionExpr { .. } | ResolvedExpr::ClassExpr { .. } => false,
        ResolvedExpr::This { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::ModuleLoad { .. } => false,
    }
}

struct LowerFunctionOptions<'a> {
    current_class: Option<&'a str>,
    in_constructor: bool,
    next_func_id: usize,
    self_closure: Option<SelfClosureOptions<'a>>,
}

struct SelfClosureOptions<'a> {
    name: &'a str,
    func_id: FuncId,
    capture_names: &'a [String],
}

#[allow(clippy::too_many_arguments)]
fn lower_function(
    id: FuncId,
    params: &[ResolvedParam],
    body: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
    function_signatures: &HashMap<FuncId, FunctionSignature>,
    function_captures: &HashMap<FuncId, Vec<String>>,
    function_mutable_captures: &HashMap<FuncId, Vec<String>>,
    class_method_captures: &HashMap<FuncId, Vec<String>>,
    class_method_mutable_captures: &HashMap<FuncId, Vec<String>>,
    env_cell_names: &HashSet<String>,
    heap_closure_names: &HashSet<String>,
    class_parents: HashMap<String, Option<String>>,
    class_private_fields: ClassPrivateFieldSlots,
    class_static_private_fields: ClassStaticPrivateFields,
    options: LowerFunctionOptions<'_>,
) -> Result<FunctionLowering, Diagnostic> {
    let signature = function_signatures.get(&id).copied().unwrap_or_default();
    if signature.needs_arguments && signature.has_rest {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-062d: `arguments` together with rest parameters is not supported in this milestone".to_owned(),
            span: None,
        });
    }
    let mut lowered_params = Vec::new();
    if signature.needs_receiver {
        lowered_params.push(ResolvedParam {
            name: "this".to_owned(),
            default: None,
            is_rest: false,
            span: None,
        });
    }
    lowered_params.extend(params.iter().cloned());
    if signature.needs_arguments {
        lowered_params.push(ResolvedParam {
            name: "arguments".to_owned(),
            default: None,
            is_rest: false,
            span: None,
        });
    }

    let (mut resolver, param_ids) = Resolver::with_params(
        function_ids,
        function_signatures,
        function_captures,
        function_mutable_captures,
        class_method_captures,
        class_method_mutable_captures,
        env_cell_names,
        heap_closure_names,
        lowered_params
            .iter()
            .map(|param| param.name.clone())
            .collect::<Vec<_>>()
            .as_slice(),
        class_parents,
        class_private_fields,
        class_static_private_fields,
        options.current_class,
        options.in_constructor,
        options.next_func_id,
    )?;

    if let Some(self_closure) = options.self_closure {
        resolver.declare_self_closure(
            self_closure.name,
            self_closure.func_id,
            self_closure.capture_names,
        )?;
    }

    let rest_param_index = params
        .iter()
        .position(|param| param.is_rest)
        .map(|index| index + usize::from(signature.needs_receiver));

    // Insert default parameter assignments at the start of the body.
    let mut body_with_defaults = Vec::new();
    for param in params {
        if let Some(pattern) = parse_binding_pattern(&param.name, param.span)? {
            if param.is_rest {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-251: rest parameter binding patterns are not supported"
                        .to_owned(),
                    span: param.span,
                });
            }
            let param_local = resolver.resolve_local(&param.name)?;
            if let Some(default) = &param.default {
                let lowered_default = resolver.lower_expr(default)?;
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
            body_with_defaults.extend(
                resolver.lower_binding_pattern_declarations(
                    &pattern,
                    LoweredExpr::Local(param_local),
                    None,
                )?,
            );
            continue;
        }
        if param.is_rest {
            // Rest parameters are populated by call lowering/emission.
            // For rest params with binding patterns like (...[value]),
            // also generate destructuring code to extract inner bindings.
            if let Some(inner) = param.name.strip_prefix("...")
                && let Some(rest_pattern) = parse_binding_pattern(inner, param.span)?
            {
                    let param_local = resolver.resolve_local(&param.name)?;
                    body_with_defaults.extend(
                        resolver.lower_binding_pattern_declarations(
                            &rest_pattern,
                            LoweredExpr::Local(param_local),
                            None,
                        )?,
                    );
                }
            continue;
        } else if let Some(default) = &param.default {
            let param_local = resolver.resolve_local(&param.name)?;
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
        .filter(|param| param.default.is_none() && !param.is_rest)
        .count()
        + usize::from(signature.needs_receiver)
        + usize::from(signature.needs_arguments);
    Ok(FunctionLowering {
        function: LoweredFunction {
            id,
            params: param_ids,
            uses_receiver: signature.needs_receiver,
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
        BinaryOp::Power => Ok(LoweredBinaryOp::Power),
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
        BinaryOp::NullishCoalesce => Ok(LoweredBinaryOp::NullishCoalesce),
        BinaryOp::BitwiseAnd
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

fn lower_unary_op(op: UnaryOp) -> Result<LoweredUnaryOp, Diagnostic> {
    match op {
        UnaryOp::Not => Ok(LoweredUnaryOp::Not),
        UnaryOp::Plus => Ok(LoweredUnaryOp::Plus),
        UnaryOp::Negate => Ok(LoweredUnaryOp::Negate),
        UnaryOp::TypeOf => Ok(LoweredUnaryOp::TypeOf),
        UnaryOp::Delete => Ok(LoweredUnaryOp::Delete),
        UnaryOp::Increment
        | UnaryOp::Decrement
        | UnaryOp::PreIncrement
        | UnaryOp::PreDecrement => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("issue-268: unary operator {:?} not yet supported", op),
            span: None,
        }),
        UnaryOp::BitwiseNot | UnaryOp::Void => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("unary operator {:?} not yet supported", op),
            span: None,
        }),
    }
}
