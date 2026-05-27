use super::types::*;
use crate::binding_pattern::parse_binding_pattern;
use crate::binding_pattern::{ArrayBinding, BindingPattern, BindingTarget};
use crate::builtin_resolved::{
    ClassMethodKind, EvalCompletionStep, ResolvedArrayElement, ResolvedExpr, ResolvedObjectProp,
    ResolvedParam, ResolvedStmt,
};
use crate::lowered::classes::{ObjectAccessorKey, ObjectAccessorProp};
use crate::lowered::facts::{
    GeneratorObjectResumePlan, GeneratorYieldStep, HostExternalKind, IntlNumberFormatOptions,
};
use crate::lowered::source_text::strip_typescript_function_source;
use crate::lowered::symbols::FunctionSignature;
use std::collections::{HashMap, HashSet};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::{BinaryOp, LogicalAssignOp, UnaryOp};

pub(crate) const SYNTHETIC_NEW_TARGET_PARAM: &str = "__ts2wasm_new_target";

#[path = "program_builtins.rs"]
pub(crate) mod program_builtins;
#[path = "program_captures.rs"]
pub(crate) mod program_captures;
#[path = "program_direct_eval.rs"]
pub(crate) mod program_direct_eval;
pub(crate) use program_builtins::*;
pub(crate) use program_captures::*;
pub(crate) use program_direct_eval::*;
pub fn lower_program(program: &[ResolvedStmt]) -> Result<LoweredProgram, Diagnostic> {
    lower_program_with_module_url(program, "<entry>")
}

pub fn lower_program_with_module_url(
    program: &[ResolvedStmt],
    module_url: impl Into<String>,
) -> Result<LoweredProgram, Diagnostic> {
    lower_program_inner(program, module_url, None, None)
}

/// Like `lower_program_with_module_url`, but pre-populates the module environment
/// with known specifier-to-module-id mappings from the module graph.
///
/// This ensures dynamic `import()` expressions in function bodies are assigned
/// the correct module graph IDs during lowering, rather than getting synthetic
/// placeholder IDs that conflict with `populate_static_module_exports_for_build`.
pub fn lower_program_with_module_specs(
    program: &[ResolvedStmt],
    module_url: impl Into<String>,
    module_specs: HashMap<String, usize>,
    type_maps: Option<(
        HashMap<String, ts2wasm_syntax::TypeRef>,
        HashMap<String, Vec<(String, ts2wasm_syntax::TypeRef)>>,
    )>,
) -> Result<LoweredProgram, Diagnostic> {
    lower_program_inner(program, module_url, Some(module_specs), type_maps)
}

fn lower_program_inner(
    program: &[ResolvedStmt],
    module_url: impl Into<String>,
    module_specs: Option<HashMap<String, usize>>,
    type_maps: Option<(
        HashMap<String, ts2wasm_syntax::TypeRef>,
        HashMap<String, Vec<(String, ts2wasm_syntax::TypeRef)>>,
    )>,
) -> Result<LoweredProgram, Diagnostic> {
    let module_url = module_url.into();
    let program_is_strict = block_has_use_strict_directive(program);
    let function_ids = collect_function_ids(program, program_is_strict)?;
    let generator_function_names = collect_generator_function_names(program);
    let generator_function_yields = collect_generator_function_yields(program);
    let (generator_function_steps, generator_function_completion_steps) =
        collect_generator_function_steps(program);
    let generator_function_object_resume_plans = collect_generator_object_resume_plans(program);
    let mut function_signatures =
        collect_function_signatures(program, &function_ids, program_is_strict);
    let function_sources = collect_function_sources(program, &function_ids);
    let top_level_local_names = collect_top_level_local_names(program)?;
    let function_captures =
        collect_top_level_function_captures(program, &function_ids, &top_level_local_names)?;
    let function_mutable_captures =
        collect_callback_function_mutable_captures(program, &function_captures)?;
    let class_method_captures = collect_class_method_captures(program, &function_ids);
    let class_method_mutable_captures =
        collect_class_method_mutable_captures(program, &function_ids);
    let mutable_class_capture_names = collect_mutable_class_capture_names(program);
    let mutable_object_method_capture_names =
        collect_block_object_method_mutable_captures(program)?;
    let mutable_nested_function_capture_names =
        collect_block_nested_function_mutable_captures(program)?;
    let direct_eval_env = collect_direct_eval_block_function_env(program);
    let dynamic_direct_eval_env_cell_names =
        collect_dynamic_direct_eval_env_cell_names(&[], program, false, false);
    let dynamic_direct_eval_created_binding_names =
        collect_dynamic_direct_eval_created_binding_names(program);
    let dynamic_direct_eval_created_function_names =
        collect_dynamic_direct_eval_created_function_names(program);
    let env_cell_names = mutable_class_capture_names
        .union(&mutable_object_method_capture_names)
        .cloned()
        .collect::<HashSet<_>>()
        .union(&collect_mutable_function_capture_names(
            &function_mutable_captures,
        ))
        .cloned()
        .collect::<HashSet<_>>()
        .union(&mutable_nested_function_capture_names)
        .cloned()
        .collect::<HashSet<_>>()
        .union(&direct_eval_env.env_cell_names)
        .cloned()
        .collect::<HashSet<_>>()
        .union(&dynamic_direct_eval_env_cell_names)
        .cloned()
        .collect::<HashSet<_>>()
        .union(&dynamic_direct_eval_created_binding_names)
        .cloned()
        .collect::<HashSet<_>>();
    let class_parents = collect_class_parents(program);
    let class_private_fields = collect_class_private_fields(program);
    let class_static_private_fields = collect_class_static_private_fields(program);
    let function_recursion_depths = compute_recursion_depths(program, &function_ids);
    let mut next_func_id = function_ids.len();
    let preindexed_function_properties = collect_preindexed_function_properties(
        program,
        &mut next_func_id,
        &mut function_signatures,
        program_is_strict,
    );
    let function_property_assignments =
        function_property_assignment_map(&preindexed_function_properties);
    let mut functions_by_id = vec![None; function_ids.len()];
    let mut generated_functions = Vec::new();

    let (type_aliases, interface_definitions) = type_maps.unwrap_or_default();

    for stmt in program {
        match stmt {
            ResolvedStmt::Function {
                name,
                params,
                body,
                is_generator,
                is_async,
                ..
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
                // Also scan for arrow function mutable captures (e.g., returning
                // a closure that mutates captured locals). These aren't tracked
                // in function_mutable_captures because arrow functions aren't
                // pre-registered like declared functions.
                let arrow_mutable_captures = collect_block_arrow_fn_mutable_captures(body);
                let object_method_mutable_captures =
                    collect_block_object_method_mutable_captures(body)?;
                let nested_function_mutable_captures =
                    collect_block_nested_function_mutable_captures(body)?;
                let dynamic_direct_eval_env_cell_names =
                    collect_dynamic_direct_eval_env_cell_names(params, body, true, false);
                let dynamic_direct_eval_created_binding_names =
                    collect_dynamic_direct_eval_created_binding_names(body);
                let dynamic_direct_eval_created_function_names =
                    collect_dynamic_direct_eval_created_function_names(body);
                let function_env_cell_names = function_env_cell_names
                    .union(&arrow_mutable_captures)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .union(&object_method_mutable_captures)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .union(&nested_function_mutable_captures)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .union(&dynamic_direct_eval_env_cell_names)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .union(&dynamic_direct_eval_created_binding_names)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .union(&dynamic_direct_eval_created_function_names)
                    .cloned()
                    .collect::<HashSet<_>>();
                let self_closure = top_level_function_body_references_name(params, body, name)?
                    .then_some(SelfClosureOptions {
                        name,
                        func_id,
                        capture_names: &[],
                        object_function_props: function_property_assignments.get(name),
                    });
                let lowered = lower_function(
                    func_id,
                    &params_with_captures,
                    body,
                    *is_generator,
                    *is_async,
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
                        in_static_method: false,
                        next_func_id,
                        self_closure,
                        capture_facts: FunctionCaptureFacts::default(),
                        recursion_depth: *function_recursion_depths.get(&func_id).unwrap_or(&0),
                        new_target_class: None,
                        module_url: module_url.as_str(),
                        strict_context: program_is_strict,
                        type_aliases: &type_aliases,
                        interface_definitions: &interface_definitions,
                    },
                )?;
                next_func_id = lowered.next_func_id;
                functions_by_id[func_id.0] = Some(lowered.function);
                generated_functions.extend(lowered.generated_functions);
            }
            ResolvedStmt::ClassDecl {
                name,
                constructor,
                extends,
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

                let mut ctor_params_for_lowering = ctor_params.clone();

                // Derived classes without explicit constructors have an implicit
                // default constructor that accepts any number of arguments
                // (constructor(...args: any[]) { super(...args); }).
                // Add a rest parameter to match JavaScript semantics so that
                // new Derived(arg) passes arity validation.
                if constructor.is_none() && extends.is_some() {
                    ctor_params_for_lowering.push(ResolvedParam {
                        name: "...args".to_owned(),
                        default: None,
                        is_rest: true,
                        span: None,
                    });
                }
                let mut ctor_params_with_this: Vec<ResolvedParam> = vec![ResolvedParam {
                    name: "this".to_owned(),
                    default: None,
                    is_rest: false,
                    span: None,
                }];
                ctor_params_with_this.extend(ctor_params_for_lowering.clone());

                let constructor_object_method_mutable_captures =
                    collect_block_object_method_mutable_captures(&ctor_body)?;
                let constructor_nested_function_mutable_captures =
                    collect_block_nested_function_mutable_captures(&ctor_body)?;
                let dynamic_direct_eval_env_cell_names = collect_dynamic_direct_eval_env_cell_names(
                    &ctor_params_with_this,
                    &ctor_body,
                    true,
                    false,
                );
                let dynamic_direct_eval_created_binding_names =
                    collect_dynamic_direct_eval_created_binding_names(&ctor_body);
                let dynamic_direct_eval_created_function_names =
                    collect_dynamic_direct_eval_created_function_names(&ctor_body);
                let constructor_env_cell_names = constructor_object_method_mutable_captures
                    .union(&constructor_nested_function_mutable_captures)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .union(&dynamic_direct_eval_env_cell_names)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .union(&dynamic_direct_eval_created_binding_names)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .union(&dynamic_direct_eval_created_function_names)
                    .cloned()
                    .collect::<HashSet<_>>();
                let lowered = lower_function(
                    ctor_id,
                    &ctor_params_for_lowering,
                    &ctor_body,
                    false,
                    false,
                    &function_ids,
                    &function_signatures,
                    &function_captures,
                    &function_mutable_captures,
                    &class_method_captures,
                    &class_method_mutable_captures,
                    &constructor_env_cell_names,
                    &HashSet::new(),
                    class_parents.clone(),
                    class_private_fields.clone(),
                    class_static_private_fields.clone(),
                    LowerFunctionOptions {
                        current_class: Some(name),
                        in_constructor: true,
                        in_static_method: false,
                        next_func_id,
                        self_closure: None,
                        capture_facts: FunctionCaptureFacts::default(),
                        recursion_depth: *function_recursion_depths.get(&ctor_id).unwrap_or(&0),
                        new_target_class: Some(name),
                        module_url: module_url.as_str(),
                        strict_context: true,
                        type_aliases: &type_aliases,
                        interface_definitions: &interface_definitions,
                    },
                )?;
                next_func_id = lowered.next_func_id;
                functions_by_id[ctor_id.0] = Some(lowered.function);
                generated_functions.extend(lowered.generated_functions);

                for method in methods {
                    let method_key = class_method_key(name, &method.name);
                    let method_id = function_ids[&method_key];
                    let mut method_params_for_lowering = method.params.clone();
                    method_params_for_lowering.extend(method.captures.iter().map(|name| {
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
                    let method_object_method_mutable_captures =
                        collect_block_object_method_mutable_captures(&method.body)?;
                    let method_nested_function_mutable_captures =
                        collect_block_nested_function_mutable_captures(&method.body)?;
                    let dynamic_direct_eval_env_cell_names =
                        collect_dynamic_direct_eval_env_cell_names(
                            &method_params_for_lowering,
                            &method.body,
                            true,
                            true,
                        );
                    let dynamic_direct_eval_created_binding_names =
                        collect_dynamic_direct_eval_created_binding_names(&method.body);
                    let dynamic_direct_eval_created_function_names =
                        collect_dynamic_direct_eval_created_function_names(&method.body);
                    let method_env_cell_names = method_env_cell_names
                        .union(&method_object_method_mutable_captures)
                        .cloned()
                        .collect::<HashSet<_>>()
                        .union(&method_nested_function_mutable_captures)
                        .cloned()
                        .collect::<HashSet<_>>()
                        .union(&dynamic_direct_eval_env_cell_names)
                        .cloned()
                        .collect::<HashSet<_>>()
                        .union(&dynamic_direct_eval_created_binding_names)
                        .cloned()
                        .collect::<HashSet<_>>()
                        .union(&dynamic_direct_eval_created_function_names)
                        .cloned()
                        .collect::<HashSet<_>>();
                    let lowered = lower_function(
                        method_id,
                        &method_params_for_lowering,
                        &method.body,
                        false,
                        false,
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
                            in_static_method: method.name.starts_with("static::"),
                            next_func_id,
                            self_closure: None,
                            capture_facts: FunctionCaptureFacts::default(),
                            recursion_depth: *function_recursion_depths
                                .get(&method_id)
                                .unwrap_or(&0),
                            new_target_class: None,
                            module_url: module_url.as_str(),
                            strict_context: true,
                            type_aliases: &type_aliases,
                            interface_definitions: &interface_definitions,
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

    for property_function in &preindexed_function_properties {
        let self_closure = (!property_function.name.is_empty()).then_some(SelfClosureOptions {
            name: property_function.name.as_str(),
            func_id: property_function.func_id,
            capture_names: &[],
            object_function_props: None,
        });
        let lowered = lower_function(
            property_function.func_id,
            &property_function.params,
            &property_function.body,
            false,
            false,
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
                current_class: None,
                in_constructor: false,
                in_static_method: false,
                next_func_id,
                self_closure,
                capture_facts: FunctionCaptureFacts::default(),
                recursion_depth: 0,
                new_target_class: None,
                module_url: module_url.as_str(),
                strict_context: program_is_strict,
                type_aliases: &type_aliases,
                interface_definitions: &interface_definitions,
            },
        )?;
        next_func_id = lowered.next_func_id;
        generated_functions.push(lowered.function);
        generated_functions.extend(lowered.generated_functions);
    }

    let mut resolver = crate::lowered::resolver::Resolver::new(
        &function_ids,
        &function_signatures,
        function_sources,
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
        module_url.as_str(),
        program_is_strict,
        type_aliases.clone(),
        interface_definitions.clone(),
    );

    // Register synthetic FuncIds for known builtin classes that appear as
    // extends targets (e.g., RegExp in `class MyRegExp extends RegExp {}`).
    // This lets class_prototype_ref find builtin constructors for instanceof
    // checks, and the backend can set up prototype chain links via synthetic
    // ClassDecl statements emitted below.
    let builtin_class_parents: &[(&str, Option<&str>)] = &[
        ("Object", None),
        ("RegExp", Some("Object")),
        ("Array", Some("Object")),
        ("Date", Some("Object")),
        ("Map", Some("Object")),
        ("Set", Some("Object")),
        ("WeakMap", Some("Object")),
        ("WeakSet", Some("Object")),
        ("Promise", Some("Object")),
        ("Error", Some("Object")),
        ("TypeError", Some("Error")),
        ("RangeError", Some("Error")),
        ("ReferenceError", Some("Error")),
        ("SyntaxError", Some("Error")),
        ("EvalError", Some("Error")),
        ("URIError", Some("Error")),
        ("AggregateError", Some("Error")),
    ];
    // Collect builtin names that are actually referenced as extends targets
    // in the user program (or are ancestors of such targets).
    let needed_builtins = collect_needed_builtins(&class_parents, builtin_class_parents);
    let builtin_ctor_base = function_ids.len();
    let builtin_ctor_ids = builtin_class_parents
        .iter()
        .filter(|(name, _)| needed_builtins.contains(*name))
        .enumerate()
        .map(|(idx, (name, _))| ((*name).to_string(), FuncId(builtin_ctor_base + idx)))
        .collect::<HashMap<_, _>>();
    let mut builtin_class_decls: Vec<LoweredStmt> = Vec::new();
    for (name, parent_name) in builtin_class_parents {
        if !needed_builtins.contains(*name) {
            continue;
        }
        let ctor_id = builtin_ctor_ids[*name];
        resolver
            .ctx
            .classes
            .class_constructor_ids
            .insert(name.to_string(), ctor_id);
        resolver
            .ctx
            .classes
            .class_parents
            .insert(name.to_string(), parent_name.map(|s| s.to_string()));
        builtin_class_decls.push(LoweredStmt::ClassDecl {
            name: name.to_string(),
            extends: parent_name.map(|s| s.to_string()),
            constructor: Some(ctor_id),
            methods: Vec::new(),
            static_methods: Vec::new(),
            private_fields: Vec::new(),
            span: Span::generated("synthetic"),
        });
    }
    // Pre-populate module IDs from the module graph so that dynamic import()
    // expressions during lowering get the correct module graph IDs, not
    // synthetic placeholder IDs that conflict with populate_static_module_exports_for_build.
    if let Some(specs) = module_specs {
        for (specifier, id) in specs {
            resolver.ctx.modules.module_ids.insert(specifier, id);
        }
    }

    resolver.ctx.facts.generator_function_yields = generator_function_yields;
    resolver.ctx.facts.generator_function_steps = generator_function_steps;
    resolver.ctx.facts.generator_function_completion_steps = generator_function_completion_steps;
    resolver.ctx.facts.generator_function_object_resume_plans =
        generator_function_object_resume_plans;
    let mut top_level_statements = Vec::new();
    // First pass: pre-declare all let/var/const names so forward references
    // (e.g., using a var before its declaration) work in the lowered resolver.
    // is_var was already discarded in builtin_resolver, so we pre-declare all.
    for stmt in program {
        if let ResolvedStmt::Let(name, _) = stmt {
            resolver.declare_local(name)?;
        }
    }
    let mut eval_created_names = dynamic_direct_eval_created_binding_names
        .into_iter()
        .collect::<Vec<_>>();
    eval_created_names.sort();
    for name in eval_created_names {
        if resolver.ctx.symbols.resolve(&name).is_some() {
            continue;
        }
        let local_id = resolver.declare_local(&name)?;
        if dynamic_direct_eval_created_function_names.contains(&name) {
            resolver
                .ctx
                .facts
                .mark_host_external(local_id, HostExternalKind::FunctionHandle, true);
        }
        if resolver.ctx.facts.env_cell_names.contains(&name) {
            resolver.ctx.facts.env_cell_locals.insert(local_id);
            resolver
                .ctx
                .facts
                .initialized_env_cell_locals
                .insert(local_id);
            top_level_statements.push(LoweredStmt::Let(
                local_id,
                LoweredExpr::EnvCellNew(
                    Box::new(LoweredExpr::Undefined(Span::generated("undefined"))),
                    Span::generated("env_cell_new"),
                ),
                Span::generated("direct_eval_created_binding"),
            ));
        } else {
            top_level_statements.push(LoweredStmt::Let(
                local_id,
                LoweredExpr::Undefined(Span::generated("undefined")),
                Span::generated("direct_eval_created_binding"),
            ));
        }
    }
    for stmt in program {
        match stmt {
            ResolvedStmt::AmbientValue(name) => {
                resolver.declare_local(name)?;
            }
            ResolvedStmt::Function {
                name,
                is_async,
                is_generator,
                ..
            } => {
                // Register function name in the lowered resolver's scope
                // so it can be referenced as a value (e.g., `let cb = myFunc`)
                // Call lowering falls through to resolve_func path, which handles
                // arguments/this correctly — do NOT add to arrow_locals.
                let local_id = resolver.declare_local(name)?;
                let func_id = resolver.resolve_func(name)?;
                if !is_async && !is_generator {
                    resolver
                        .ctx
                        .facts
                        .constructable_function_locals
                        .insert(local_id);
                }
                top_level_statements.push(LoweredStmt::Let(
                    local_id,
                    LoweredExpr::ArrowFn {
                        func_id,
                        captures: Vec::new(),
                        representation: ClosureRepresentation::DirectLocalToken,
                        span: Span::generated("arrow_fn"),
                    },
                    Span::generated("let"),
                ));
            }
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
                    } else if method.kind == ClassMethodKind::Method {
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
                    span: Span::generated("class_decl"),
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
                            top_level_statements.push(resolver.lower_class_static_private_field(
                                name,
                                field,
                                initializer,
                            )?);
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
    generated_functions.extend(resolver.ctx.functions.generated_functions);

    // Emit synthetic ClassDecl for builtin classes so the backend creates
    // prototype globals and sets up extends-based prototype chain links.
    top_level_statements.extend(builtin_class_decls);

    // Add stub function entries for synthetic builtin constructors so that
    // FuncIds referenced by ClassDecl and ClassPrototypeRef pass validation.
    let needed_count = needed_builtins.len();
    if needed_count > 0 {
        functions_by_id.resize(function_ids.len() + needed_count, None);
        for (name, _parent_name) in builtin_class_parents.iter() {
            if !needed_builtins.contains(*name) {
                continue;
            }
            let ctor_id = builtin_ctor_ids[*name];
            functions_by_id[ctor_id.0] = Some(LoweredFunction {
                id: ctor_id,
                params: Vec::new(),
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: Some((*name).to_owned()),
                locals: Vec::new(),
                body: Vec::new(),
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            });
        }
    }

    let mut functions = functions_by_id
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "function id allocation left an unfilled function slot".to_owned(),
            span: None,

            phase: None,
        })?;
    generated_functions.sort_by_key(|function| function.id.0);
    functions.extend(generated_functions);

    Ok(LoweredProgram {
        top_level_statements,
        top_level_locals: resolver.ctx.symbols.locals,
        functions,
        modules: resolver.ctx.modules.modules,
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

pub(crate) struct FunctionLowering {
    pub(crate) function: LoweredFunction,
    pub(crate) generated_functions: Vec<LoweredFunction>,
    pub(crate) next_func_id: usize,
}

fn collect_function_ids(
    program: &[ResolvedStmt],
    program_is_strict: bool,
) -> Result<HashMap<String, FuncId>, Diagnostic> {
    let mut function_ids = HashMap::new();
    let mut next_func_id = 0;

    // Pre-collect names with concrete (non-empty body) functions for
    // overload-group detection.
    let concrete_names: HashSet<&str> = program
        .iter()
        .filter_map(|s| match s {
            ResolvedStmt::Function { name, body, .. } if !body.is_empty() => Some(name.as_str()),
            _ => None,
        })
        .collect();

    for stmt in program {
        match stmt {
            ResolvedStmt::Function { name, body, .. } => {
                if function_ids.contains_key(name.as_str()) {
                    // Allow bodyless overloads to reuse an existing name.
                    // Only body-ful (concrete function body) duplicates are errors,
                    // and only in strict mode (ES spec §14.1.1).
                    if !body.is_empty() && program_is_strict {
                        return Err(Diagnostic {
                            code: DiagCode::DuplicateFunction,
                            message: format!("duplicate function definition: `{name}`"),
                            span: None,

                            phase: None,
                        });
                    }
                    continue;
                }
                // Skip bodyless overloads that have a concrete implementation.
                if body.is_empty() && concrete_names.contains(name.as_str()) {
                    continue;
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

                        phase: None,
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

                            phase: None,
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

fn collect_generator_function_yields(
    program: &[ResolvedStmt],
) -> HashMap<String, Vec<ResolvedExpr>> {
    let mut yields = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::Function {
            name,
            body,
            is_generator: true,
            ..
        } = stmt
        {
            let values = evaluate_straight_line_generator_yields(body).unwrap_or_else(|| {
                let mut values = Vec::new();
                collect_generator_yield_values(body, &mut values);
                values
            });
            yields.insert(name.clone(), values);
        }
    }
    yields
}

fn collect_generator_function_steps(
    program: &[ResolvedStmt],
) -> (
    HashMap<String, Vec<GeneratorYieldStep>>,
    HashMap<String, Vec<ResolvedStmt>>,
) {
    let mut steps = HashMap::new();
    let mut completion_steps = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::Function {
            name,
            body,
            is_generator: true,
            ..
        } = stmt
            && let Some(function_steps) = collect_straight_line_generator_steps(body)
        {
            steps.insert(name.clone(), function_steps.steps);
            completion_steps.insert(name.clone(), function_steps.completion);
        }
    }
    (steps, completion_steps)
}

fn collect_generator_object_resume_plans(
    program: &[ResolvedStmt],
) -> HashMap<String, GeneratorObjectResumePlan> {
    let mut plans = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::Function {
            name,
            body,
            is_generator: true,
            ..
        } = stmt
            && let Some(plan) = collect_generator_object_resume_plan(body)
        {
            plans.insert(name.clone(), plan);
        }
    }
    plans
}

fn collect_generator_object_resume_plan(
    stmts: &[ResolvedStmt],
) -> Option<GeneratorObjectResumePlan> {
    let [ResolvedStmt::Assign(target, ResolvedExpr::Object(props))] = stmts else {
        return None;
    };
    let mut yield_values = Vec::new();
    for prop in props {
        match prop {
            ResolvedObjectProp::ComputedKey { key, .. } => {
                let ResolvedExpr::Yield { expr, delegate } = key.as_ref() else {
                    if contains_generator_yield_expr(key.as_ref()) {
                        return None;
                    }
                    continue;
                };
                if *delegate {
                    return None;
                }
                yield_values.push(
                    expr.as_ref()
                        .map(|expr| expr.as_ref().clone())
                        .unwrap_or(ResolvedExpr::Undefined),
                );
            }
            _ if contains_generator_yield_expr(prop.value()) => return None,
            _ => {}
        }
    }
    (!yield_values.is_empty()).then(|| GeneratorObjectResumePlan {
        target: target.clone(),
        props: props.clone(),
        yield_values,
    })
}

fn collect_straight_line_generator_steps(stmts: &[ResolvedStmt]) -> Option<GeneratorStepPlan> {
    let mut collector = GeneratorStepCollector::default();
    collector.collect_stmts(stmts)?;
    if !collector.steps.is_empty() {
        Some(GeneratorStepPlan {
            steps: collector.steps,
            completion: collector.pending,
        })
    } else {
        None
    }
}

struct GeneratorStepPlan {
    steps: Vec<GeneratorYieldStep>,
    completion: Vec<ResolvedStmt>,
}

#[derive(Default)]
struct GeneratorStepCollector {
    pending: Vec<ResolvedStmt>,
    steps: Vec<GeneratorYieldStep>,
}

impl GeneratorStepCollector {
    fn collect_stmts(&mut self, stmts: &[ResolvedStmt]) -> Option<()> {
        for stmt in stmts {
            self.collect_stmt(stmt)?;
        }
        Some(())
    }

    fn collect_stmt(&mut self, stmt: &ResolvedStmt) -> Option<()> {
        match stmt {
            ResolvedStmt::Let(name, ResolvedExpr::Object(props)) => {
                if let Some((value, resumed_props)) =
                    split_object_literal_computed_yield_step(props)
                {
                    self.steps.push(GeneratorYieldStep {
                        statements: std::mem::take(&mut self.pending),
                        value,
                    });
                    self.pending.push(ResolvedStmt::Let(
                        name.clone(),
                        ResolvedExpr::Object(resumed_props),
                    ));
                    return Some(());
                }
                self.pending.push(stmt.clone());
                Some(())
            }
            ResolvedStmt::Expr(ResolvedExpr::Yield { expr, delegate }) => {
                if *delegate {
                    return None;
                }
                let value = expr
                    .as_ref()
                    .map(|expr| expr.as_ref().clone())
                    .unwrap_or(ResolvedExpr::Undefined);
                self.steps.push(GeneratorYieldStep {
                    statements: std::mem::take(&mut self.pending),
                    value,
                });
                Some(())
            }
            ResolvedStmt::Block { statements } => self.collect_stmts(statements),
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let then_expr = single_generator_yield_value(then_body)?;
                let else_expr = single_generator_yield_value(else_body)?;
                self.steps.push(GeneratorYieldStep {
                    statements: std::mem::take(&mut self.pending),
                    value: ResolvedExpr::Ternary {
                        condition: Box::new(condition.clone()),
                        then_expr: Box::new(then_expr),
                        else_expr: Box::new(else_expr),
                        span: Span::generated("ternary"),
                    },
                });
                Some(())
            }
            ResolvedStmt::For {
                init: Some(init),
                condition: Some(condition),
                update: Some(update),
                body,
            } => self.collect_static_counter_for(init, condition, update, body),
            ResolvedStmt::Assign(..)
            | ResolvedStmt::Expr(..)
            | ResolvedStmt::Return(..)
            | ResolvedStmt::DestructureLet { .. }
            | ResolvedStmt::DestructureAssign { .. } => {
                self.pending.push(stmt.clone());
                Some(())
            }
            _ => None,
        }
    }

    fn collect_static_counter_for(
        &mut self,
        init: &ResolvedStmt,
        condition: &ResolvedExpr,
        update: &ResolvedExpr,
        body: &[ResolvedStmt],
    ) -> Option<()> {
        let (var, start) = parse_generator_for_init(init)?;
        let end = parse_generator_for_less_than(condition, &var)?;
        parse_generator_for_increment(update, &var)?;
        let value = single_generator_yield_value(body)?;
        if start > end || end - start > 1024 {
            return None;
        }

        let mut first_statements = std::mem::take(&mut self.pending);
        first_statements.push(init.clone());
        for index in start..end {
            let statements = if index == start {
                std::mem::take(&mut first_statements)
            } else {
                vec![ResolvedStmt::Expr(update.clone())]
            };
            self.steps.push(GeneratorYieldStep {
                statements,
                value: value.clone(),
            });
        }
        self.pending.push(ResolvedStmt::Expr(update.clone()));
        Some(())
    }
}

fn split_object_literal_computed_yield_step(
    props: &[ResolvedObjectProp],
) -> Option<(ResolvedExpr, Vec<ResolvedObjectProp>)> {
    let mut resumed_props = Vec::with_capacity(props.len());
    let mut yielded_value = None;
    for prop in props {
        match prop {
            ResolvedObjectProp::ComputedKey { key, value }
                if yielded_value.is_none()
                    && matches!(
                        key.as_ref(),
                        ResolvedExpr::Yield {
                            delegate: false,
                            ..
                        }
                    ) =>
            {
                let ResolvedExpr::Yield { expr, .. } = key.as_ref() else {
                    unreachable!("matches! above guarantees a yield expression");
                };
                yielded_value = Some(
                    expr.as_ref()
                        .map(|expr| expr.as_ref().clone())
                        .unwrap_or(ResolvedExpr::Undefined),
                );
                resumed_props.push(ResolvedObjectProp::ComputedKey {
                    key: Box::new(ResolvedExpr::Undefined),
                    value: value.clone(),
                });
            }
            ResolvedObjectProp::ComputedKey { key, .. }
                if contains_generator_yield_expr(key.as_ref()) =>
            {
                return None;
            }
            _ => resumed_props.push(prop.clone()),
        }
    }
    yielded_value.map(|value| (value, resumed_props))
}

fn contains_generator_yield_expr(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Yield { .. } => true,
        ResolvedExpr::Await { expr }
        | ResolvedExpr::Spread(expr)
        | ResolvedExpr::Unary { expr, .. } => contains_generator_yield_expr(expr),
        ResolvedExpr::Binary { left, right, .. } => {
            contains_generator_yield_expr(left) || contains_generator_yield_expr(right)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            contains_generator_yield_expr(condition)
                || contains_generator_yield_expr(then_expr)
                || contains_generator_yield_expr(else_expr)
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            contains_generator_yield_expr(callee) || args.iter().any(contains_generator_yield_expr)
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            contains_generator_yield_expr(object) || args.iter().any(contains_generator_yield_expr)
        }
        ResolvedExpr::Object(props) => props.iter().any(|prop| {
            prop.computed_key()
                .is_some_and(contains_generator_yield_expr)
                || contains_generator_yield_expr(prop.value())
        }),
        ResolvedExpr::Array(elements) => elements.iter().any(|element| match element {
            ResolvedArrayElement::Present(expr) => contains_generator_yield_expr(expr),
            ResolvedArrayElement::Hole => false,
        }),
        ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
            contains_generator_yield_expr(object)
        }
        ResolvedExpr::Assign { expr, .. } | ResolvedExpr::LogicalAssign { expr, .. } => {
            contains_generator_yield_expr(expr)
        }
        ResolvedExpr::LogicalPropertyAssign { expr, .. } => contains_generator_yield_expr(expr),
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. }
        | ResolvedExpr::LogicalComputedMemberAssign { key, expr, .. } => {
            contains_generator_yield_expr(key) || contains_generator_yield_expr(expr)
        }
        ResolvedExpr::PropertyAssignDynamic {
            object,
            key,
            value: expr,
        } => {
            contains_generator_yield_expr(object)
                || contains_generator_yield_expr(key)
                || contains_generator_yield_expr(expr)
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            contains_generator_yield_expr(object) || contains_generator_yield_expr(expr)
        }
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            contains_generator_yield_expr(object) || contains_generator_yield_expr(index)
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            contains_generator_yield_expr(object) || contains_generator_yield_expr(value)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(contains_generator_yield_expr)
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            plan.args.iter().any(contains_generator_yield_expr)
        }
        ResolvedExpr::ArrowFn {
            body, body_stmts, ..
        } => {
            contains_generator_yield_expr(body)
                || body_stmts.iter().any(stmt_contains_generator_yield_expr)
        }
        ResolvedExpr::FunctionExpr { .. } => false,
        _ => false,
    }
}

fn stmt_contains_generator_yield_expr(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::Assign(_, expr)
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr) => contains_generator_yield_expr(expr),
        ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::DestructureAssign { expr, .. } => contains_generator_yield_expr(expr),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            contains_generator_yield_expr(expr)
        }
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            contains_generator_yield_expr(condition)
                || then_body.iter().any(stmt_contains_generator_yield_expr)
                || else_body.iter().any(stmt_contains_generator_yield_expr)
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { condition, body } => {
            contains_generator_yield_expr(condition)
                || body.iter().any(stmt_contains_generator_yield_expr)
        }
        ResolvedStmt::Block { statements } => {
            statements.iter().any(stmt_contains_generator_yield_expr)
        }
        ResolvedStmt::Function { .. } => false,
        _ => false,
    }
}

fn single_generator_yield_value(stmts: &[ResolvedStmt]) -> Option<ResolvedExpr> {
    let [stmt] = stmts else {
        return None;
    };
    match stmt {
        ResolvedStmt::Expr(ResolvedExpr::Yield { expr, delegate }) if !*delegate => Some(
            expr.as_ref()
                .map(|expr| expr.as_ref().clone())
                .unwrap_or(ResolvedExpr::Undefined),
        ),
        ResolvedStmt::Block { statements } => single_generator_yield_value(statements),
        _ => None,
    }
}

fn parse_generator_for_init(stmt: &ResolvedStmt) -> Option<(String, i32)> {
    match stmt {
        ResolvedStmt::Let(name, ResolvedExpr::Number(value))
        | ResolvedStmt::Assign(name, ResolvedExpr::Number(value)) => Some((name.clone(), *value)),
        _ => None,
    }
}

fn parse_generator_for_less_than(expr: &ResolvedExpr, var: &str) -> Option<i32> {
    match expr {
        ResolvedExpr::Binary { left, op, right }
            if *op == BinaryOp::Less
                && matches!(left.as_ref(), ResolvedExpr::Ident(name) if name == var) =>
        {
            match right.as_ref() {
                ResolvedExpr::Number(value) => Some(*value),
                _ => None,
            }
        }
        _ => None,
    }
}

fn parse_generator_for_increment(expr: &ResolvedExpr, var: &str) -> Option<()> {
    match expr {
        ResolvedExpr::Assign { name, expr } if name == var => match expr.as_ref() {
            ResolvedExpr::Binary { left, op, right }
                if *op == BinaryOp::Add
                    && matches!(left.as_ref(), ResolvedExpr::Ident(name) if name == var)
                    && matches!(right.as_ref(), ResolvedExpr::Number(1)) =>
            {
                Some(())
            }
            _ => None,
        },
        _ => None,
    }
}

fn evaluate_straight_line_generator_yields(stmts: &[ResolvedStmt]) -> Option<Vec<ResolvedExpr>> {
    let mut evaluator = GeneratorYieldEvaluator::default();
    evaluator.eval_stmts(stmts)?;
    Some(evaluator.yields)
}

#[derive(Default)]
struct GeneratorYieldEvaluator {
    locals: HashMap<String, ResolvedExpr>,
    yields: Vec<ResolvedExpr>,
}

impl GeneratorYieldEvaluator {
    fn eval_stmts(&mut self, stmts: &[ResolvedStmt]) -> Option<()> {
        for stmt in stmts {
            self.eval_stmt(stmt)?;
        }
        Some(())
    }

    fn eval_stmt(&mut self, stmt: &ResolvedStmt) -> Option<()> {
        match stmt {
            ResolvedStmt::Let(name, expr) | ResolvedStmt::Assign(name, expr) => {
                let value = self.eval_expr(expr)?;
                self.locals.insert(name.clone(), value);
                Some(())
            }
            ResolvedStmt::Expr(ResolvedExpr::Assign { name, expr }) => {
                let value = self.eval_expr(expr)?;
                self.locals.insert(name.clone(), value);
                Some(())
            }
            ResolvedStmt::Expr(ResolvedExpr::Yield { expr, delegate }) => {
                if *delegate {
                    return None;
                }
                let value = expr
                    .as_ref()
                    .map(|expr| self.eval_expr(expr))
                    .unwrap_or(Some(ResolvedExpr::Undefined))?;
                self.yields.push(value);
                Some(())
            }
            ResolvedStmt::Block { statements } => self.eval_stmts(statements),
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                if self.eval_bool_expr(condition)? {
                    self.eval_stmts(then_body)
                } else {
                    self.eval_stmts(else_body)
                }
            }
            _ => None,
        }
    }

    fn eval_expr(&self, expr: &ResolvedExpr) -> Option<ResolvedExpr> {
        match expr {
            ResolvedExpr::Number(_)
            | ResolvedExpr::DecimalNumber(_)
            | ResolvedExpr::BigIntLiteral { .. }
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined => Some(expr.clone()),
            ResolvedExpr::Ident(name) => self.locals.get(name).cloned(),
            ResolvedExpr::Binary { left, op, right } => self.eval_binary_expr(left, *op, right),
            _ => None,
        }
    }

    fn eval_bool_expr(&self, expr: &ResolvedExpr) -> Option<bool> {
        match self.eval_expr(expr)? {
            ResolvedExpr::Bool(value) => Some(value),
            _ => None,
        }
    }

    fn eval_binary_expr(
        &self,
        left: &ResolvedExpr,
        op: BinaryOp,
        right: &ResolvedExpr,
    ) -> Option<ResolvedExpr> {
        let left = self.eval_expr(left)?;
        let right = self.eval_expr(right)?;
        match (left, op, right) {
            (ResolvedExpr::Number(left), BinaryOp::Add, ResolvedExpr::Number(right)) => {
                Some(ResolvedExpr::Number(left.checked_add(right)?))
            }
            (ResolvedExpr::Number(left), BinaryOp::Subtract, ResolvedExpr::Number(right)) => {
                Some(ResolvedExpr::Number(left.checked_sub(right)?))
            }
            (ResolvedExpr::Number(left), BinaryOp::Multiply, ResolvedExpr::Number(right)) => {
                Some(ResolvedExpr::Number(left.checked_mul(right)?))
            }
            (ResolvedExpr::Number(left), BinaryOp::Divide, ResolvedExpr::Number(right))
                if right != 0 =>
            {
                Some(ResolvedExpr::Number(left / right))
            }
            (ResolvedExpr::String(left), BinaryOp::Add, ResolvedExpr::String(right)) => {
                Some(ResolvedExpr::String(format!("{left}{right}")))
            }
            _ => None,
        }
    }
}

fn collect_generator_yield_values(stmts: &[ResolvedStmt], values: &mut Vec<ResolvedExpr>) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Expr(ResolvedExpr::Yield { expr, delegate }) => {
                if *delegate {
                    return;
                }
                values.push(
                    expr.as_ref()
                        .map(|expr| expr.as_ref().clone())
                        .unwrap_or(ResolvedExpr::Undefined),
                );
            }
            ResolvedStmt::Block { statements } => {
                collect_generator_yield_values(statements, values)
            }
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_generator_yield_values(then_body, values);
                collect_generator_yield_values(else_body, values);
            }
            ResolvedStmt::While { body, .. }
            | ResolvedStmt::DoWhile { body, .. }
            | ResolvedStmt::ForIn { body, .. }
            | ResolvedStmt::ForOf { body, .. }
            | ResolvedStmt::ForAwaitOf { body, .. } => collect_generator_yield_values(body, values),
            ResolvedStmt::For { init, body, .. } => {
                if let Some(init) = init {
                    collect_generator_yield_values(std::slice::from_ref(init.as_ref()), values);
                }
                collect_generator_yield_values(body, values);
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_generator_yield_values(try_block, values);
                if let Some(catch_block) = catch_block {
                    collect_generator_yield_values(catch_block, values);
                }
                if let Some(finally_block) = finally_block {
                    collect_generator_yield_values(finally_block, values);
                }
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_generator_yield_values(body, values);
                }
            }
            _ => {}
        }
    }
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
            ResolvedStmt::AmbientValue(name) => {
                names.insert(name.clone());
            }
            ResolvedStmt::DestructureLet { pattern, .. } => {
                names.extend(pattern.names().into_iter().map(ToOwned::to_owned));
            }
            _ => {}
        }
    }
    Ok(names)
}

fn top_level_function_body_references_name(
    params: &[ResolvedParam],
    body: &[ResolvedStmt],
    name: &str,
) -> Result<bool, Diagnostic> {
    let mut excluded = HashSet::new();
    for param in params {
        if let Some(pattern) = parse_binding_pattern(&param.name, param.span)? {
            excluded.extend(pattern.names().into_iter().map(ToOwned::to_owned));
        } else {
            excluded.insert(param.name.clone());
        }
    }
    collect_declared_names_in_stmts(body, &mut excluded);

    let mut captures = Vec::new();
    collect_stmt_captures(body, &excluded, &mut captures);
    Ok(captures.iter().any(|capture| capture == name))
}

fn collect_top_level_function_captures(
    program: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
    top_level_local_names: &HashSet<String>,
) -> Result<HashMap<FuncId, Vec<String>>, Diagnostic> {
    let mut captures = HashMap::new();
    let mut direct_callee_graph = Vec::new();
    if top_level_local_names.is_empty() {
        return Ok(captures);
    }

    for stmt in program {
        let ResolvedStmt::Function {
            name, params, body, ..
        } = stmt
        else {
            continue;
        };
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
        collect_nested_function_captures_in_stmts(body, &excluded, &mut found)?;

        let mut direct_callees = found
            .iter()
            .filter(|name| function_ids.contains_key(name.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        collect_direct_function_call_targets_in_stmts(body, &mut direct_callees);
        let direct_callees = direct_callees
            .into_iter()
            .filter_map(|callee| function_ids.get(&callee).copied())
            .collect::<Vec<_>>();
        direct_callee_graph.push((function_ids[name], direct_callees));

        let found = found
            .into_iter()
            .filter(|capture| top_level_local_names.contains(capture))
            .collect::<Vec<_>>();
        if !found.is_empty() {
            captures.insert(function_ids[name], found);
        }
    }

    loop {
        let mut changed = false;
        for (caller, callees) in &direct_callee_graph {
            let mut caller_captures = captures.get(caller).cloned().unwrap_or_default();
            for callee in callees {
                let Some(callee_captures) = captures.get(callee) else {
                    continue;
                };
                for capture in callee_captures {
                    if !caller_captures.contains(capture) {
                        caller_captures.push(capture.clone());
                        changed = true;
                    }
                }
            }
            if caller_captures.is_empty() {
                captures.remove(caller);
            } else {
                captures.insert(*caller, caller_captures);
            }
        }
        if !changed {
            break;
        }
    }

    Ok(captures)
}

pub(crate) fn collect_nested_function_captures_in_stmts(
    stmts: &[ResolvedStmt],
    outer_excluded: &HashSet<String>,
    captures: &mut Vec<String>,
) -> Result<(), Diagnostic> {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Let(_, expr)
            | ResolvedStmt::DestructureLet { expr, .. }
            | ResolvedStmt::DestructureAssign { expr, .. }
            | ResolvedStmt::Assign(_, expr)
            | ResolvedStmt::Expr(expr)
            | ResolvedStmt::Return(expr)
            | ResolvedStmt::Throw(expr) => {
                collect_nested_function_captures_in_expr(expr, outer_excluded, captures)?;
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_nested_function_captures_in_expr(condition, outer_excluded, captures)?;
                collect_nested_function_captures_in_stmts(then_body, outer_excluded, captures)?;
                collect_nested_function_captures_in_stmts(else_body, outer_excluded, captures)?;
            }
            ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { condition, body } => {
                collect_nested_function_captures_in_expr(condition, outer_excluded, captures)?;
                collect_nested_function_captures_in_stmts(body, outer_excluded, captures)?;
            }
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init) = init {
                    collect_nested_function_captures_in_stmts(
                        std::slice::from_ref(init.as_ref()),
                        outer_excluded,
                        captures,
                    )?;
                }
                if let Some(condition) = condition {
                    collect_nested_function_captures_in_expr(condition, outer_excluded, captures)?;
                }
                if let Some(update) = update {
                    collect_nested_function_captures_in_expr(update, outer_excluded, captures)?;
                }
                collect_nested_function_captures_in_stmts(body, outer_excluded, captures)?;
            }
            ResolvedStmt::ForIn { iter, body, .. }
            | ResolvedStmt::ForOf { iter, body, .. }
            | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
                collect_nested_function_captures_in_expr(iter, outer_excluded, captures)?;
                collect_nested_function_captures_in_stmts(body, outer_excluded, captures)?;
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_nested_function_captures_in_stmts(try_block, outer_excluded, captures)?;
                if let Some(block) = catch_block {
                    collect_nested_function_captures_in_stmts(block, outer_excluded, captures)?;
                }
                if let Some(block) = finally_block {
                    collect_nested_function_captures_in_stmts(block, outer_excluded, captures)?;
                }
            }
            ResolvedStmt::Switch { expr, cases } => {
                collect_nested_function_captures_in_expr(expr, outer_excluded, captures)?;
                for (case_expr, body) in cases {
                    if let Some(case_expr) = case_expr {
                        collect_nested_function_captures_in_expr(
                            case_expr,
                            outer_excluded,
                            captures,
                        )?;
                    }
                    collect_nested_function_captures_in_stmts(body, outer_excluded, captures)?;
                }
            }
            ResolvedStmt::Block { statements, .. } => {
                collect_nested_function_captures_in_stmts(statements, outer_excluded, captures)?;
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_nested_function_captures_in_stmts(
                    std::slice::from_ref(body.as_ref()),
                    outer_excluded,
                    captures,
                )?;
            }
            ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
                collect_nested_function_captures_in_expr(expr, outer_excluded, captures)?;
            }
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Function { .. }
            | ResolvedStmt::ClassDecl { .. }
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. } => {}
        }
    }
    Ok(())
}

pub(crate) fn collect_nested_function_captures_in_expr(
    expr: &ResolvedExpr,
    outer_excluded: &HashSet<String>,
    captures: &mut Vec<String>,
) -> Result<(), Diagnostic> {
    match expr {
        ResolvedExpr::FunctionExpr { params, body, .. } => {
            let mut nested_excluded = outer_excluded.clone();
            nested_excluded.extend(resolved_param_names(params)?);
            collect_declared_names_in_stmts(body, &mut nested_excluded);
            collect_stmt_captures(body, &nested_excluded, captures);
            collect_nested_function_captures_in_stmts(body, &nested_excluded, captures)?;
        }
        ResolvedExpr::Object(props) => {
            for prop in props {
                if let Some(key) = prop.computed_key() {
                    collect_nested_function_captures_in_expr(key, outer_excluded, captures)?;
                }
                collect_nested_function_captures_in_expr(prop.value(), outer_excluded, captures)?;
            }
        }
        ResolvedExpr::Await { expr }
        | ResolvedExpr::Yield {
            expr: Some(expr), ..
        }
        | ResolvedExpr::Unary { expr, .. }
        | ResolvedExpr::Spread(expr) => {
            collect_nested_function_captures_in_expr(expr, outer_excluded, captures)?;
        }
        ResolvedExpr::Yield { expr: None, .. } => {}
        ResolvedExpr::Binary { left, right, .. } => {
            collect_nested_function_captures_in_expr(left, outer_excluded, captures)?;
            collect_nested_function_captures_in_expr(right, outer_excluded, captures)?;
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_nested_function_captures_in_expr(condition, outer_excluded, captures)?;
            collect_nested_function_captures_in_expr(then_expr, outer_excluded, captures)?;
            collect_nested_function_captures_in_expr(else_expr, outer_excluded, captures)?;
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            collect_nested_function_captures_in_expr(callee, outer_excluded, captures)?;
            for arg in args {
                collect_nested_function_captures_in_expr(arg, outer_excluded, captures)?;
            }
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
            collect_nested_function_captures_in_expr(expr, outer_excluded, captures)?;
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. }
        | ResolvedExpr::LogicalComputedMemberAssign { key, expr, .. } => {
            collect_nested_function_captures_in_expr(key, outer_excluded, captures)?;
            collect_nested_function_captures_in_expr(expr, outer_excluded, captures)?;
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            collect_nested_function_captures_in_expr(object, outer_excluded, captures)?;
            collect_nested_function_captures_in_expr(expr, outer_excluded, captures)?;
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                if let ResolvedArrayElement::Present(expr) = element {
                    collect_nested_function_captures_in_expr(expr, outer_excluded, captures)?;
                }
            }
        }
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            collect_nested_function_captures_in_expr(object, outer_excluded, captures)?;
            collect_nested_function_captures_in_expr(index, outer_excluded, captures)?;
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            for arg in args {
                collect_nested_function_captures_in_expr(arg, outer_excluded, captures)?;
            }
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            for arg in &plan.args {
                collect_nested_function_captures_in_expr(arg, outer_excluded, captures)?;
            }
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
            collect_nested_function_captures_in_expr(object, outer_excluded, captures)?;
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            collect_nested_function_captures_in_expr(object, outer_excluded, captures)?;
            for arg in args {
                collect_nested_function_captures_in_expr(arg, outer_excluded, captures)?;
            }
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            collect_nested_function_captures_in_expr(object, outer_excluded, captures)?;
            collect_nested_function_captures_in_expr(value, outer_excluded, captures)?;
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            collect_nested_function_captures_in_expr(object, outer_excluded, captures)?;
            collect_nested_function_captures_in_expr(key, outer_excluded, captures)?;
            collect_nested_function_captures_in_expr(value, outer_excluded, captures)?;
        }
        ResolvedExpr::Sequence(exprs) => {
            for e in exprs {
                collect_nested_function_captures_in_expr(e, outer_excluded, captures)?;
            }
        }
        ResolvedExpr::EvalCompletion(steps) => {
            for step in steps {
                collect_nested_function_captures_in_eval_step(step, outer_excluded, captures)?;
            }
        }
        ResolvedExpr::ArrowFn { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined => {}
        ResolvedExpr::Eval { .. } => {}
    }
    Ok(())
}

fn collect_nested_function_captures_in_eval_step(
    step: &EvalCompletionStep,
    outer_excluded: &HashSet<String>,
    captures: &mut Vec<String>,
) -> Result<(), Diagnostic> {
    match step {
        EvalCompletionStep::FunctionDecl { params, body, .. } => {
            let mut nested_excluded = outer_excluded.clone();
            nested_excluded.extend(resolved_param_names(params)?);
            collect_declared_names_in_stmts(body, &mut nested_excluded);
            collect_stmt_captures(body, &nested_excluded, captures);
            collect_nested_function_captures_in_stmts(body, &nested_excluded, captures)?;
        }
        EvalCompletionStep::Block(steps) => {
            for step in steps {
                collect_nested_function_captures_in_eval_step(step, outer_excluded, captures)?;
            }
        }
        EvalCompletionStep::If {
            then_steps,
            else_steps,
            ..
        } => {
            for step in then_steps.iter().chain(else_steps) {
                collect_nested_function_captures_in_eval_step(step, outer_excluded, captures)?;
            }
        }
        EvalCompletionStep::While { body_steps, .. }
        | EvalCompletionStep::DoWhile { body_steps, .. }
        | EvalCompletionStep::ForIn { body_steps, .. }
        | EvalCompletionStep::ForOf { body_steps, .. } => {
            for step in body_steps {
                collect_nested_function_captures_in_eval_step(step, outer_excluded, captures)?;
            }
        }
        EvalCompletionStep::For {
            init, body_steps, ..
        } => {
            if let Some(init) = init {
                collect_nested_function_captures_in_eval_step(init, outer_excluded, captures)?;
            }
            for step in body_steps {
                collect_nested_function_captures_in_eval_step(step, outer_excluded, captures)?;
            }
        }
        EvalCompletionStep::Switch { cases, .. } => {
            for (_, steps) in cases {
                for step in steps {
                    collect_nested_function_captures_in_eval_step(step, outer_excluded, captures)?;
                }
            }
        }
        EvalCompletionStep::TryCatch {
            try_steps,
            catch_steps,
            finally_steps,
            ..
        } => {
            for step in try_steps {
                collect_nested_function_captures_in_eval_step(step, outer_excluded, captures)?;
            }
            if let Some(steps) = catch_steps {
                for step in steps {
                    collect_nested_function_captures_in_eval_step(step, outer_excluded, captures)?;
                }
            }
            if let Some(steps) = finally_steps {
                for step in steps {
                    collect_nested_function_captures_in_eval_step(step, outer_excluded, captures)?;
                }
            }
        }
        EvalCompletionStep::Labeled { body, .. } => {
            collect_nested_function_captures_in_eval_step(body, outer_excluded, captures)?;
        }
        EvalCompletionStep::Value(_)
        | EvalCompletionStep::Empty(_)
        | EvalCompletionStep::VarLet { .. }
        | EvalCompletionStep::GlobalVarLet { .. }
        | EvalCompletionStep::GlobalFunctionDecl { .. }
        | EvalCompletionStep::DestructureLet { .. }
        | EvalCompletionStep::DestructureVarLet { .. }
        | EvalCompletionStep::LexicalLet { .. }
        | EvalCompletionStep::ClassDecl { .. }
        | EvalCompletionStep::Throw(_)
        | EvalCompletionStep::Break { .. }
        | EvalCompletionStep::Continue { .. } => {}
    }
    if let Some(expr) = step.expr() {
        collect_nested_function_captures_in_expr(expr, outer_excluded, captures)?;
    }
    Ok(())
}

fn collect_direct_function_call_targets_in_stmts(
    stmts: &[ResolvedStmt],
    targets: &mut Vec<String>,
) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Let(_, expr)
            | ResolvedStmt::DestructureLet { expr, .. }
            | ResolvedStmt::DestructureAssign { expr, .. }
            | ResolvedStmt::Assign(_, expr)
            | ResolvedStmt::Expr(expr)
            | ResolvedStmt::Return(expr)
            | ResolvedStmt::Throw(expr) => {
                collect_direct_function_call_targets_in_expr(expr, targets);
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_direct_function_call_targets_in_expr(condition, targets);
                collect_direct_function_call_targets_in_stmts(then_body, targets);
                collect_direct_function_call_targets_in_stmts(else_body, targets);
            }
            ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
                collect_direct_function_call_targets_in_expr(condition, targets);
                collect_direct_function_call_targets_in_stmts(body, targets);
            }
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init) = init {
                    collect_direct_function_call_targets_in_stmts(
                        std::slice::from_ref(init.as_ref()),
                        targets,
                    );
                }
                if let Some(condition) = condition {
                    collect_direct_function_call_targets_in_expr(condition, targets);
                }
                if let Some(update) = update {
                    collect_direct_function_call_targets_in_expr(update, targets);
                }
                collect_direct_function_call_targets_in_stmts(body, targets);
            }
            ResolvedStmt::ForIn { iter, body, .. }
            | ResolvedStmt::ForOf { iter, body, .. }
            | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
                collect_direct_function_call_targets_in_expr(iter, targets);
                collect_direct_function_call_targets_in_stmts(body, targets);
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_direct_function_call_targets_in_stmts(try_block, targets);
                if let Some(block) = catch_block {
                    collect_direct_function_call_targets_in_stmts(block, targets);
                }
                if let Some(block) = finally_block {
                    collect_direct_function_call_targets_in_stmts(block, targets);
                }
            }
            ResolvedStmt::Switch { expr, cases } => {
                collect_direct_function_call_targets_in_expr(expr, targets);
                for (case_expr, body) in cases {
                    if let Some(case_expr) = case_expr {
                        collect_direct_function_call_targets_in_expr(case_expr, targets);
                    }
                    collect_direct_function_call_targets_in_stmts(body, targets);
                }
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_direct_function_call_targets_in_stmts(
                    std::slice::from_ref(body.as_ref()),
                    targets,
                );
            }
            ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
                collect_direct_function_call_targets_in_expr(expr, targets);
            }
            ResolvedStmt::Block { statements, .. } => {
                collect_direct_function_call_targets_in_stmts(statements, targets);
            }
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Function { .. }
            | ResolvedStmt::ClassDecl { .. }
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. } => {}
        }
    }
}

fn collect_direct_function_call_targets_in_expr(expr: &ResolvedExpr, targets: &mut Vec<String>) {
    match expr {
        ResolvedExpr::Await { expr } | ResolvedExpr::Spread(expr) => {
            collect_direct_function_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::Yield { expr, .. } => {
            if let Some(expr) = expr {
                collect_direct_function_call_targets_in_expr(expr, targets);
            }
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            if let ResolvedExpr::Ident(name) = callee.as_ref()
                && !targets.contains(name)
            {
                targets.push(name.clone());
            }
            collect_direct_function_call_targets_in_expr(callee, targets);
            for arg in args {
                collect_direct_function_call_targets_in_expr(arg, targets);
            }
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            collect_direct_function_call_targets_in_expr(object, targets);
            for arg in args {
                collect_direct_function_call_targets_in_expr(arg, targets);
            }
        }
        ResolvedExpr::Unary { expr, .. }
        | ResolvedExpr::BuiltinProperty { object: expr, .. }
        | ResolvedExpr::PropertyAccess { object: expr, .. }
        | ResolvedExpr::OptionalPropertyAccess { object: expr, .. } => {
            collect_direct_function_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::Binary { left, right, .. }
        | ResolvedExpr::ComputedIndex {
            object: left,
            index: right,
        } => {
            collect_direct_function_call_targets_in_expr(left, targets);
            collect_direct_function_call_targets_in_expr(right, targets);
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_direct_function_call_targets_in_expr(condition, targets);
            collect_direct_function_call_targets_in_expr(then_expr, targets);
            collect_direct_function_call_targets_in_expr(else_expr, targets);
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
            collect_direct_function_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            collect_direct_function_call_targets_in_expr(object, targets);
            collect_direct_function_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            collect_direct_function_call_targets_in_expr(key, targets);
            collect_direct_function_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            collect_direct_function_call_targets_in_expr(object, targets);
            collect_direct_function_call_targets_in_expr(key, targets);
            collect_direct_function_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                if let ResolvedArrayElement::Present(expr) = element {
                    collect_direct_function_call_targets_in_expr(expr, targets);
                }
            }
        }
        ResolvedExpr::Object(props) => {
            for prop in props {
                if let Some(key) = prop.computed_key() {
                    collect_direct_function_call_targets_in_expr(key, targets);
                }
                collect_direct_function_call_targets_in_expr(prop.value(), targets);
            }
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            for arg in args {
                collect_direct_function_call_targets_in_expr(arg, targets);
            }
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            for arg in &plan.args {
                collect_direct_function_call_targets_in_expr(arg, targets);
            }
        }
        ResolvedExpr::OptionalComputedIndex { object, index, .. }
        | ResolvedExpr::PropertyAssignDynamic {
            object, key: index, ..
        } => {
            collect_direct_function_call_targets_in_expr(object, targets);
            collect_direct_function_call_targets_in_expr(index, targets);
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            collect_direct_function_call_targets_in_expr(object, targets);
            collect_direct_function_call_targets_in_expr(value, targets);
        }
        ResolvedExpr::Sequence(exprs) => {
            for e in exprs {
                collect_direct_function_call_targets_in_expr(e, targets);
            }
        }
        ResolvedExpr::EvalCompletion(steps) => {
            for step in steps {
                if let Some(expr) = step.expr() {
                    collect_direct_function_call_targets_in_expr(expr, targets);
                }
            }
        }
        ResolvedExpr::ArrowFn { .. }
        | ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. } => {}
        ResolvedExpr::Eval { .. } => {}
    }
}

fn collect_callback_function_mutable_captures(
    program: &[ResolvedStmt],
    function_captures: &HashMap<FuncId, Vec<String>>,
) -> Result<HashMap<FuncId, Vec<String>>, Diagnostic> {
    let mut mutable_captures = HashMap::new();
    if function_captures.is_empty() {
        return Ok(mutable_captures);
    }

    let function_ids = collect_function_ids(program, false).unwrap_or_default();
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
        let object_method_mutable_captures = collect_block_object_method_mutable_captures(body)?;
        let mutable = captures
            .iter()
            .filter(|capture| {
                block_assigns_any_name(body, std::slice::from_ref(capture))
                    || object_method_mutable_captures.contains(*capture)
            })
            .cloned()
            .collect::<Vec<_>>();
        if !mutable.is_empty() {
            mutable_captures.insert(func_id, mutable);
        }
    }

    Ok(mutable_captures)
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

/// Collect names that are mutably captured by arrow functions within the
/// given block. This detects patterns like:
/// ```typescript
/// function makeCounter() {
///   let count = 0;
///   return () => { count = count + 1; return count; };
/// }
/// ```
/// where `count` is captured and mutated by the returned arrow function,
/// requiring env-cell allocation in the enclosing function.
fn collect_block_arrow_fn_mutable_captures(stmts: &[ResolvedStmt]) -> HashSet<String> {
    let mut mutable_captures = HashSet::new();
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Return(expr) if matches!(expr, ResolvedExpr::ArrowFn { .. }) => {
                if let ResolvedExpr::ArrowFn {
                    body, body_stmts, ..
                } = expr
                {
                    let mut arrow_captures = Vec::new();
                    collect_expr_captures(body, &HashSet::new(), &mut arrow_captures);
                    collect_stmt_captures(body_stmts, &HashSet::new(), &mut arrow_captures);
                    for capture in &arrow_captures {
                        if block_assigns_any_name(body_stmts, core::slice::from_ref(capture)) {
                            mutable_captures.insert(capture.clone());
                        }
                    }
                }
            }
            ResolvedStmt::Expr(expr) if matches!(expr, ResolvedExpr::ArrowFn { .. }) => {
                if let ResolvedExpr::ArrowFn {
                    body, body_stmts, ..
                } = expr
                {
                    let mut arrow_captures = Vec::new();
                    collect_expr_captures(body, &HashSet::new(), &mut arrow_captures);
                    collect_stmt_captures(body_stmts, &HashSet::new(), &mut arrow_captures);
                    for capture in &arrow_captures {
                        if block_assigns_any_name(body_stmts, core::slice::from_ref(capture)) {
                            mutable_captures.insert(capture.clone());
                        }
                    }
                }
            }
            ResolvedStmt::Return(..) | ResolvedStmt::Expr(..) | ResolvedStmt::Throw(..) => {}
            ResolvedStmt::Let(_, expr) => {
                if let ResolvedExpr::ArrowFn {
                    body, body_stmts, ..
                } = expr
                {
                    let mut arrow_captures = Vec::new();
                    collect_expr_captures(body, &HashSet::new(), &mut arrow_captures);
                    collect_stmt_captures(body_stmts, &HashSet::new(), &mut arrow_captures);
                    for capture in &arrow_captures {
                        if block_assigns_any_name(body_stmts, core::slice::from_ref(capture)) {
                            mutable_captures.insert(capture.clone());
                        }
                    }
                }
            }
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                mutable_captures.extend(collect_block_arrow_fn_mutable_captures(then_body));
                mutable_captures.extend(collect_block_arrow_fn_mutable_captures(else_body));
            }
            ResolvedStmt::While { body, .. } | ResolvedStmt::DoWhile { body, .. } => {
                mutable_captures.extend(collect_block_arrow_fn_mutable_captures(body));
            }
            ResolvedStmt::For { body, .. }
            | ResolvedStmt::ForIn { body, .. }
            | ResolvedStmt::ForOf { body, .. }
            | ResolvedStmt::ForAwaitOf { body, .. } => {
                mutable_captures.extend(collect_block_arrow_fn_mutable_captures(body));
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                mutable_captures.extend(collect_block_arrow_fn_mutable_captures(try_block));
                if let Some(block) = catch_block {
                    mutable_captures.extend(collect_block_arrow_fn_mutable_captures(block));
                }
                if let Some(block) = finally_block {
                    mutable_captures.extend(collect_block_arrow_fn_mutable_captures(block));
                }
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    mutable_captures.extend(collect_block_arrow_fn_mutable_captures(body));
                }
            }
            ResolvedStmt::Block { statements, .. } => {
                mutable_captures.extend(collect_block_arrow_fn_mutable_captures(statements));
            }
            ResolvedStmt::Labeled { body, .. } => {
                mutable_captures.extend(collect_block_arrow_fn_mutable_captures(
                    std::slice::from_ref(body),
                ));
            }
            ResolvedStmt::Assign(..)
            | ResolvedStmt::DestructureLet { .. }
            | ResolvedStmt::DestructureAssign { .. }
            | ResolvedStmt::AmbientValue(..)
            | ResolvedStmt::Function { .. }
            | ResolvedStmt::ClassDecl { .. }
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. }
            | ResolvedStmt::Export { .. }
            | ResolvedStmt::ModuleExportsAssign { .. } => {}
        }
    }
    mutable_captures
}

fn collect_block_nested_function_mutable_captures(
    stmts: &[ResolvedStmt],
) -> Result<HashSet<String>, Diagnostic> {
    let mut mutable_captures = HashSet::new();
    for stmt in stmts {
        collect_stmt_nested_function_mutable_captures(stmt, &mut mutable_captures)?;
    }
    Ok(mutable_captures)
}

fn collect_stmt_nested_function_mutable_captures(
    stmt: &ResolvedStmt,
    mutable_captures: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    match stmt {
        ResolvedStmt::Function { params, body, .. } => {
            mutable_captures.extend(collect_function_expr_mutable_captures(params, body)?);
            collect_block_nested_function_mutable_captures_into(body, mutable_captures)?;
        }
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::DestructureAssign { expr, .. }
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr)
        | ResolvedStmt::Assign(_, expr) => {
            collect_expr_nested_function_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            collect_expr_nested_function_mutable_captures(condition, mutable_captures)?;
            collect_block_nested_function_mutable_captures_into(then_body, mutable_captures)?;
            collect_block_nested_function_mutable_captures_into(else_body, mutable_captures)?;
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
            collect_expr_nested_function_mutable_captures(condition, mutable_captures)?;
            collect_block_nested_function_mutable_captures_into(body, mutable_captures)?;
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            if let Some(init) = init {
                collect_stmt_nested_function_mutable_captures(init, mutable_captures)?;
            }
            if let Some(condition) = condition {
                collect_expr_nested_function_mutable_captures(condition, mutable_captures)?;
            }
            if let Some(update) = update {
                collect_expr_nested_function_mutable_captures(update, mutable_captures)?;
            }
            collect_block_nested_function_mutable_captures_into(body, mutable_captures)?;
        }
        ResolvedStmt::ForIn { iter, body, .. }
        | ResolvedStmt::ForOf { iter, body, .. }
        | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
            collect_expr_nested_function_mutable_captures(iter, mutable_captures)?;
            collect_block_nested_function_mutable_captures_into(body, mutable_captures)?;
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            collect_block_nested_function_mutable_captures_into(try_block, mutable_captures)?;
            if let Some(block) = catch_block {
                collect_block_nested_function_mutable_captures_into(block, mutable_captures)?;
            }
            if let Some(block) = finally_block {
                collect_block_nested_function_mutable_captures_into(block, mutable_captures)?;
            }
        }
        ResolvedStmt::Switch { expr, cases } => {
            collect_expr_nested_function_mutable_captures(expr, mutable_captures)?;
            for (case_expr, body) in cases {
                if let Some(case_expr) = case_expr {
                    collect_expr_nested_function_mutable_captures(case_expr, mutable_captures)?;
                }
                collect_block_nested_function_mutable_captures_into(body, mutable_captures)?;
            }
        }
        ResolvedStmt::Block { statements, .. } => {
            collect_block_nested_function_mutable_captures_into(statements, mutable_captures)?;
        }
        ResolvedStmt::Labeled { body, .. } => {
            collect_stmt_nested_function_mutable_captures(body, mutable_captures)?;
        }
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            collect_expr_nested_function_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => {}
    }
    Ok(())
}

fn collect_block_nested_function_mutable_captures_into(
    stmts: &[ResolvedStmt],
    mutable_captures: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    for stmt in stmts {
        collect_stmt_nested_function_mutable_captures(stmt, mutable_captures)?;
    }
    Ok(())
}

fn collect_expr_nested_function_mutable_captures(
    expr: &ResolvedExpr,
    mutable_captures: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    match expr {
        ResolvedExpr::FunctionExpr { params, body, .. } => {
            mutable_captures.extend(collect_function_expr_mutable_captures(params, body)?);
            collect_block_nested_function_mutable_captures_into(body, mutable_captures)?;
        }
        ResolvedExpr::Object(props) => {
            for prop in props {
                if let Some(key) = prop.computed_key() {
                    collect_expr_nested_function_mutable_captures(key, mutable_captures)?;
                }
                collect_expr_nested_function_mutable_captures(prop.value(), mutable_captures)?;
            }
        }
        ResolvedExpr::Await { expr }
        | ResolvedExpr::Yield {
            expr: Some(expr), ..
        }
        | ResolvedExpr::Unary { expr, .. }
        | ResolvedExpr::Spread(expr) => {
            collect_expr_nested_function_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedExpr::Yield { expr: None, .. } => {}
        ResolvedExpr::Binary { left, right, .. } => {
            collect_expr_nested_function_mutable_captures(left, mutable_captures)?;
            collect_expr_nested_function_mutable_captures(right, mutable_captures)?;
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_expr_nested_function_mutable_captures(condition, mutable_captures)?;
            collect_expr_nested_function_mutable_captures(then_expr, mutable_captures)?;
            collect_expr_nested_function_mutable_captures(else_expr, mutable_captures)?;
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            collect_expr_nested_function_mutable_captures(callee, mutable_captures)?;
            for arg in args {
                collect_expr_nested_function_mutable_captures(arg, mutable_captures)?;
            }
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
            collect_expr_nested_function_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. }
        | ResolvedExpr::LogicalComputedMemberAssign { key, expr, .. } => {
            collect_expr_nested_function_mutable_captures(key, mutable_captures)?;
            collect_expr_nested_function_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            collect_expr_nested_function_mutable_captures(object, mutable_captures)?;
            collect_expr_nested_function_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                if let ResolvedArrayElement::Present(expr) = element {
                    collect_expr_nested_function_mutable_captures(expr, mutable_captures)?;
                }
            }
        }
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            collect_expr_nested_function_mutable_captures(object, mutable_captures)?;
            collect_expr_nested_function_mutable_captures(index, mutable_captures)?;
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            for arg in args {
                collect_expr_nested_function_mutable_captures(arg, mutable_captures)?;
            }
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            for arg in &plan.args {
                collect_expr_nested_function_mutable_captures(arg, mutable_captures)?;
            }
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
            collect_expr_nested_function_mutable_captures(object, mutable_captures)?;
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            collect_expr_nested_function_mutable_captures(object, mutable_captures)?;
            for arg in args {
                collect_expr_nested_function_mutable_captures(arg, mutable_captures)?;
            }
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            collect_expr_nested_function_mutable_captures(object, mutable_captures)?;
            collect_expr_nested_function_mutable_captures(value, mutable_captures)?;
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            collect_expr_nested_function_mutable_captures(object, mutable_captures)?;
            collect_expr_nested_function_mutable_captures(key, mutable_captures)?;
            collect_expr_nested_function_mutable_captures(value, mutable_captures)?;
        }
        ResolvedExpr::ArrowFn {
            body, body_stmts, ..
        } => {
            collect_expr_nested_function_mutable_captures(body, mutable_captures)?;
            collect_block_nested_function_mutable_captures_into(body_stmts, mutable_captures)?;
        }
        ResolvedExpr::Sequence(exprs) => {
            for e in exprs {
                collect_expr_nested_function_mutable_captures(e, mutable_captures)?;
            }
        }
        ResolvedExpr::EvalCompletion(steps) => {
            for step in steps {
                collect_eval_step_nested_function_mutable_captures(step, mutable_captures)?;
            }
        }
        ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Eval { .. } => {}
        ResolvedExpr::Undefined => {}
    }
    Ok(())
}

fn collect_eval_step_nested_function_mutable_captures(
    step: &EvalCompletionStep,
    mutable_captures: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    match step {
        EvalCompletionStep::FunctionDecl { params, body, .. } => {
            mutable_captures.extend(collect_function_expr_mutable_captures(params, body)?);
            collect_block_nested_function_mutable_captures_into(body, mutable_captures)?;
        }
        EvalCompletionStep::Block(steps) => {
            for step in steps {
                collect_eval_step_nested_function_mutable_captures(step, mutable_captures)?;
            }
        }
        EvalCompletionStep::If {
            then_steps,
            else_steps,
            ..
        } => {
            for step in then_steps.iter().chain(else_steps) {
                collect_eval_step_nested_function_mutable_captures(step, mutable_captures)?;
            }
        }
        EvalCompletionStep::While { body_steps, .. }
        | EvalCompletionStep::DoWhile { body_steps, .. }
        | EvalCompletionStep::ForIn { body_steps, .. }
        | EvalCompletionStep::ForOf { body_steps, .. } => {
            for step in body_steps {
                collect_eval_step_nested_function_mutable_captures(step, mutable_captures)?;
            }
        }
        EvalCompletionStep::For {
            init, body_steps, ..
        } => {
            if let Some(init) = init {
                collect_eval_step_nested_function_mutable_captures(init, mutable_captures)?;
            }
            for step in body_steps {
                collect_eval_step_nested_function_mutable_captures(step, mutable_captures)?;
            }
        }
        EvalCompletionStep::Switch { cases, .. } => {
            for (_, steps) in cases {
                for step in steps {
                    collect_eval_step_nested_function_mutable_captures(step, mutable_captures)?;
                }
            }
        }
        EvalCompletionStep::TryCatch {
            try_steps,
            catch_steps,
            finally_steps,
            ..
        } => {
            for step in try_steps {
                collect_eval_step_nested_function_mutable_captures(step, mutable_captures)?;
            }
            if let Some(steps) = catch_steps {
                for step in steps {
                    collect_eval_step_nested_function_mutable_captures(step, mutable_captures)?;
                }
            }
            if let Some(steps) = finally_steps {
                for step in steps {
                    collect_eval_step_nested_function_mutable_captures(step, mutable_captures)?;
                }
            }
        }
        EvalCompletionStep::Labeled { body, .. } => {
            collect_eval_step_nested_function_mutable_captures(body, mutable_captures)?;
        }
        EvalCompletionStep::Value(_)
        | EvalCompletionStep::Empty(_)
        | EvalCompletionStep::VarLet { .. }
        | EvalCompletionStep::GlobalVarLet { .. }
        | EvalCompletionStep::GlobalFunctionDecl { .. }
        | EvalCompletionStep::DestructureLet { .. }
        | EvalCompletionStep::DestructureVarLet { .. }
        | EvalCompletionStep::LexicalLet { .. }
        | EvalCompletionStep::ClassDecl { .. }
        | EvalCompletionStep::Throw(_)
        | EvalCompletionStep::Break { .. }
        | EvalCompletionStep::Continue { .. } => {}
    }
    if let Some(expr) = step.expr() {
        collect_expr_nested_function_mutable_captures(expr, mutable_captures)?;
    }
    Ok(())
}

fn collect_block_object_method_mutable_captures(
    stmts: &[ResolvedStmt],
) -> Result<HashSet<String>, Diagnostic> {
    let mut mutable_captures = HashSet::new();
    for stmt in stmts {
        collect_stmt_object_method_mutable_captures(stmt, &mut mutable_captures)?;
    }
    Ok(mutable_captures)
}

fn collect_stmt_object_method_mutable_captures(
    stmt: &ResolvedStmt,
    mutable_captures: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    match stmt {
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::DestructureAssign { expr, .. }
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr)
        | ResolvedStmt::Assign(_, expr) => {
            collect_expr_object_method_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            collect_expr_object_method_mutable_captures(condition, mutable_captures)?;
            collect_block_object_method_mutable_captures_into(then_body, mutable_captures)?;
            collect_block_object_method_mutable_captures_into(else_body, mutable_captures)?;
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
            collect_expr_object_method_mutable_captures(condition, mutable_captures)?;
            collect_block_object_method_mutable_captures_into(body, mutable_captures)?;
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            if let Some(init) = init {
                collect_stmt_object_method_mutable_captures(init, mutable_captures)?;
            }
            if let Some(condition) = condition {
                collect_expr_object_method_mutable_captures(condition, mutable_captures)?;
            }
            if let Some(update) = update {
                collect_expr_object_method_mutable_captures(update, mutable_captures)?;
            }
            collect_block_object_method_mutable_captures_into(body, mutable_captures)?;
        }
        ResolvedStmt::ForIn { iter, body, .. }
        | ResolvedStmt::ForOf { iter, body, .. }
        | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
            collect_expr_object_method_mutable_captures(iter, mutable_captures)?;
            collect_block_object_method_mutable_captures_into(body, mutable_captures)?;
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            collect_block_object_method_mutable_captures_into(try_block, mutable_captures)?;
            if let Some(block) = catch_block {
                collect_block_object_method_mutable_captures_into(block, mutable_captures)?;
            }
            if let Some(block) = finally_block {
                collect_block_object_method_mutable_captures_into(block, mutable_captures)?;
            }
        }
        ResolvedStmt::Switch { expr, cases } => {
            collect_expr_object_method_mutable_captures(expr, mutable_captures)?;
            for (case_expr, body) in cases {
                if let Some(case_expr) = case_expr {
                    collect_expr_object_method_mutable_captures(case_expr, mutable_captures)?;
                }
                collect_block_object_method_mutable_captures_into(body, mutable_captures)?;
            }
        }
        ResolvedStmt::Block { statements, .. } => {
            collect_block_object_method_mutable_captures_into(statements, mutable_captures)?;
        }
        ResolvedStmt::Labeled { body, .. } => {
            collect_stmt_object_method_mutable_captures(body, mutable_captures)?;
        }
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            collect_expr_object_method_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => {}
    }
    Ok(())
}

fn collect_block_object_method_mutable_captures_into(
    stmts: &[ResolvedStmt],
    mutable_captures: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    for stmt in stmts {
        collect_stmt_object_method_mutable_captures(stmt, mutable_captures)?;
    }
    Ok(())
}

fn collect_expr_object_method_mutable_captures(
    expr: &ResolvedExpr,
    mutable_captures: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    match expr {
        ResolvedExpr::Object(props) => {
            for prop in props {
                if let Some(key) = prop.computed_key() {
                    collect_expr_object_method_mutable_captures(key, mutable_captures)?;
                }
                if let Some((params, body)) = object_prop_function_value(prop) {
                    mutable_captures.extend(collect_function_expr_mutable_captures(params, body)?);
                } else {
                    collect_expr_object_method_mutable_captures(prop.value(), mutable_captures)?;
                }
            }
        }
        ResolvedExpr::Await { expr }
        | ResolvedExpr::Yield {
            expr: Some(expr), ..
        }
        | ResolvedExpr::Unary { expr, .. }
        | ResolvedExpr::Spread(expr) => {
            collect_expr_object_method_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedExpr::Yield { expr: None, .. } => {}
        ResolvedExpr::Binary { left, right, .. } => {
            collect_expr_object_method_mutable_captures(left, mutable_captures)?;
            collect_expr_object_method_mutable_captures(right, mutable_captures)?;
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_expr_object_method_mutable_captures(condition, mutable_captures)?;
            collect_expr_object_method_mutable_captures(then_expr, mutable_captures)?;
            collect_expr_object_method_mutable_captures(else_expr, mutable_captures)?;
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            collect_expr_object_method_mutable_captures(callee, mutable_captures)?;
            for arg in args {
                collect_expr_object_method_mutable_captures(arg, mutable_captures)?;
            }
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
            collect_expr_object_method_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. }
        | ResolvedExpr::LogicalComputedMemberAssign { key, expr, .. } => {
            collect_expr_object_method_mutable_captures(key, mutable_captures)?;
            collect_expr_object_method_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            collect_expr_object_method_mutable_captures(object, mutable_captures)?;
            collect_expr_object_method_mutable_captures(expr, mutable_captures)?;
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                if let ResolvedArrayElement::Present(expr) = element {
                    collect_expr_object_method_mutable_captures(expr, mutable_captures)?;
                }
            }
        }
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            collect_expr_object_method_mutable_captures(object, mutable_captures)?;
            collect_expr_object_method_mutable_captures(index, mutable_captures)?;
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            for arg in args {
                collect_expr_object_method_mutable_captures(arg, mutable_captures)?;
            }
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            for arg in &plan.args {
                collect_expr_object_method_mutable_captures(arg, mutable_captures)?;
            }
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
            collect_expr_object_method_mutable_captures(object, mutable_captures)?;
        }
        ResolvedExpr::MethodCall { object, args, .. } => {
            collect_expr_object_method_mutable_captures(object, mutable_captures)?;
            for arg in args {
                collect_expr_object_method_mutable_captures(arg, mutable_captures)?;
            }
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            collect_expr_object_method_mutable_captures(object, mutable_captures)?;
            collect_expr_object_method_mutable_captures(value, mutable_captures)?;
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            collect_expr_object_method_mutable_captures(object, mutable_captures)?;
            collect_expr_object_method_mutable_captures(key, mutable_captures)?;
            collect_expr_object_method_mutable_captures(value, mutable_captures)?;
        }
        ResolvedExpr::Sequence(exprs) => {
            for e in exprs {
                collect_expr_object_method_mutable_captures(e, mutable_captures)?;
            }
        }
        ResolvedExpr::EvalCompletion(steps) => {
            for step in steps {
                if let Some(expr) = step.expr() {
                    collect_expr_object_method_mutable_captures(expr, mutable_captures)?;
                }
            }
        }
        ResolvedExpr::ArrowFn { .. }
        | ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Eval { .. } => {}
        ResolvedExpr::Null | ResolvedExpr::Undefined => {}
    }
    Ok(())
}

fn object_prop_function_value(
    prop: &ResolvedObjectProp,
) -> Option<(&[ResolvedParam], &[ResolvedStmt])> {
    match prop.value() {
        ResolvedExpr::FunctionExpr { params, body, .. } => Some((params, body)),
        _ => None,
    }
}

fn collect_function_expr_mutable_captures(
    params: &[ResolvedParam],
    body: &[ResolvedStmt],
) -> Result<HashSet<String>, Diagnostic> {
    let mut excluded = resolved_param_names(params)?;
    collect_declared_names_in_stmts(body, &mut excluded);

    let mut captures = Vec::new();
    collect_stmt_captures(body, &excluded, &mut captures);
    Ok(captures
        .into_iter()
        .filter(|capture| block_assigns_any_name(body, std::slice::from_ref(capture)))
        .collect())
}

fn resolved_param_names(params: &[ResolvedParam]) -> Result<HashSet<String>, Diagnostic> {
    let mut names = HashSet::new();
    for param in params {
        if let Some(inner) = param.name.strip_prefix("...") {
            if let Some(pattern) = parse_binding_pattern(inner, param.span)? {
                names.extend(pattern.names().into_iter().map(ToOwned::to_owned));
            } else {
                names.insert(inner.to_owned());
            }
        } else if let Some(pattern) = parse_binding_pattern(&param.name, param.span)? {
            names.extend(pattern.names().into_iter().map(ToOwned::to_owned));
        } else {
            names.insert(param.name.clone());
        }
    }
    Ok(names)
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
    parent_is_strict: bool,
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
                        is_strict: function_body_is_strict(parent_is_strict, body),
                        needs_arguments: (block_contains_arguments(body)
                            || block_contains_dynamic_direct_eval(body))
                            && !params.iter().any(|param| param.name == "arguments"),
                        needs_new_target: false,
                        has_rest: params.iter().any(|param| param.is_rest),
                        metadata_length: fixed_arity_metadata_length(params),
                        metadata_name: Some(name.clone()),
                        returns_heap_closure: block_returns_declared_function(body),
                        returns_dense_array: block_returns_dense_array_local(body),
                        returns_first_param_identity: body_returns_first_param_identity(
                            params, body,
                        ),
                        returns_static_string: body_returns_static_string(body),
                    },
                );
            }
            ResolvedStmt::ClassDecl {
                name,
                constructor,
                extends,
                methods,
                ..
            } => {
                let ctor_key = class_constructor_key(name);
                let ctor_params_len = constructor
                    .as_ref()
                    .map(|(params, _)| params.len())
                    .unwrap_or_default();
                let ctor_has_rest = constructor
                    .as_ref()
                    .is_some_and(|(params, _)| params.iter().any(|param| param.is_rest))
                    // Derived classes without explicit constructors accept any
                    // number of arguments (implicit rest).
                    || (constructor.is_none() && extends.is_some());
                let ctor_returns_heap_closure = constructor
                    .as_ref()
                    .is_some_and(|(_, body)| block_returns_declared_function(body));
                let ctor_returns_dense_array = constructor
                    .as_ref()
                    .is_some_and(|(_, body)| block_returns_dense_array_local(body));
                let ctor_returns_static_string = constructor
                    .as_ref()
                    .and_then(|(_, body)| body_returns_static_string(body));
                let ctor_needs_arguments = constructor.as_ref().is_some_and(|(params, body)| {
                    (block_contains_arguments(body) || block_contains_dynamic_direct_eval(body))
                        && !params.iter().any(|param| param.name == "arguments")
                });
                signatures.insert(
                    function_ids[&ctor_key],
                    FunctionSignature {
                        explicit_params: ctor_params_len,
                        needs_receiver: true,
                        needs_arguments: ctor_needs_arguments,
                        has_rest: ctor_has_rest,
                        is_strict: true,
                        returns_heap_closure: ctor_returns_heap_closure,
                        returns_dense_array: ctor_returns_dense_array,
                        returns_static_string: ctor_returns_static_string,
                        ..FunctionSignature::default()
                    },
                );

                for method in methods {
                    let method_key = class_method_key(name, &method.name);
                    let is_static_method = method.name.starts_with("static::");
                    signatures.insert(
                        function_ids[&method_key],
                        FunctionSignature {
                            explicit_params: method.params.len(),
                            needs_receiver: block_contains_this(&method.body)
                                || block_contains_dynamic_direct_eval(&method.body)
                                || (!is_static_method && block_contains_super(&method.body)),
                            needs_arguments: (block_contains_arguments(&method.body)
                                || block_contains_dynamic_direct_eval(&method.body))
                                && !method.params.iter().any(|param| param.name == "arguments"),
                            has_rest: method.params.iter().any(|param| param.is_rest),
                            is_strict: true,
                            returns_heap_closure: block_returns_declared_function(&method.body),
                            returns_dense_array: block_returns_dense_array_local(&method.body),
                            returns_static_string: body_returns_static_string(&method.body),
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

fn collect_function_sources(
    program: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
) -> HashMap<FuncId, String> {
    let mut sources = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::Function {
            name, source_text, ..
        } = stmt
            && !source_text.is_empty()
        {
            sources.insert(
                function_ids[name],
                strip_typescript_function_source(source_text),
            );
        }
    }
    sources
}

#[derive(Debug, Clone)]
struct PreindexedFunctionProperty {
    receiver: String,
    key: String,
    name: String,
    func_id: FuncId,
    params: Vec<ResolvedParam>,
    body: Vec<ResolvedStmt>,
}

fn collect_preindexed_function_properties(
    program: &[ResolvedStmt],
    next_func_id: &mut usize,
    function_signatures: &mut HashMap<FuncId, FunctionSignature>,
    parent_is_strict: bool,
) -> Vec<PreindexedFunctionProperty> {
    let mut properties = Vec::new();
    for stmt in program {
        let ResolvedStmt::Expr(ResolvedExpr::PropertyAssign {
            object, key, value, ..
        }) = stmt
        else {
            continue;
        };
        let ResolvedExpr::Ident(receiver) = object.as_ref() else {
            continue;
        };
        let ResolvedExpr::FunctionExpr {
            name, params, body, ..
        } = value.as_ref()
        else {
            continue;
        };
        let func_id = FuncId(*next_func_id);
        *next_func_id += 1;
        function_signatures.insert(
            func_id,
            function_signature_for_params_body(params, body, parent_is_strict),
        );
        properties.push(PreindexedFunctionProperty {
            receiver: receiver.clone(),
            key: key.clone(),
            name: name.clone(),
            func_id,
            params: params.clone(),
            body: body.clone(),
        });
    }
    properties
}

fn function_property_assignment_map(
    properties: &[PreindexedFunctionProperty],
) -> HashMap<String, HashMap<ObjectAccessorKey, FuncId>> {
    let mut by_receiver: HashMap<String, HashMap<ObjectAccessorKey, FuncId>> = HashMap::new();
    for property in properties {
        by_receiver
            .entry(property.receiver.clone())
            .or_default()
            .insert(
                ObjectAccessorKey::Property(property.key.clone()),
                property.func_id,
            );
    }
    by_receiver
}

fn function_signature_for_params_body(
    params: &[ResolvedParam],
    body: &[ResolvedStmt],
    parent_is_strict: bool,
) -> FunctionSignature {
    FunctionSignature {
        explicit_params: params.len(),
        needs_receiver: block_contains_this(body),
        is_strict: function_body_is_strict(parent_is_strict, body),
        needs_arguments: (block_contains_arguments(body)
            || block_contains_dynamic_direct_eval(body))
            && !params.iter().any(|param| param.name == "arguments"),
        needs_new_target: false,
        has_rest: params.iter().any(|param| param.is_rest),
        metadata_length: fixed_arity_metadata_length(params),
        metadata_name: None,
        returns_heap_closure: block_returns_declared_function(body),
        returns_dense_array: block_returns_dense_array_local(body),
        returns_first_param_identity: body_returns_first_param_identity(params, body),
        returns_static_string: body_returns_static_string(body),
    }
}

fn body_returns_first_param_identity(params: &[ResolvedParam], body: &[ResolvedStmt]) -> bool {
    let [param] = params else {
        return false;
    };
    if param.is_rest || param.default.is_some() {
        return false;
    }

    let mut first_body_stmt = 0;
    while matches!(
        body.get(first_body_stmt),
        Some(ResolvedStmt::Expr(ResolvedExpr::String(_)))
    ) {
        first_body_stmt += 1;
    }

    matches!(
        &body[first_body_stmt..],
        [ResolvedStmt::Return(ResolvedExpr::Ident(name))] if name == &param.name
    )
}

pub(crate) fn body_returns_static_string(body: &[ResolvedStmt]) -> Option<String> {
    let mut first_body_stmt = 0;
    while matches!(
        body.get(first_body_stmt),
        Some(ResolvedStmt::Expr(ResolvedExpr::String(_)))
    ) {
        first_body_stmt += 1;
    }

    let body = &body[first_body_stmt..];
    let (last, prefix) = body.split_last()?;
    if prefix.iter().any(stmt_contains_return) {
        return None;
    }
    match last {
        ResolvedStmt::Return(ResolvedExpr::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn stmt_contains_return(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Return(_) => true,
        ResolvedStmt::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(stmt_contains_return) || else_body.iter().any(stmt_contains_return)
        }
        ResolvedStmt::While { body, .. }
        | ResolvedStmt::DoWhile { body, .. }
        | ResolvedStmt::For { body, .. }
        | ResolvedStmt::ForIn { body, .. }
        | ResolvedStmt::ForOf { body, .. }
        | ResolvedStmt::ForAwaitOf { body, .. } => body.iter().any(stmt_contains_return),
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            try_block.iter().any(stmt_contains_return)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block.iter().any(stmt_contains_return))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block.iter().any(stmt_contains_return))
        }
        ResolvedStmt::Switch { cases, .. } => cases
            .iter()
            .any(|(_, stmts)| stmts.iter().any(stmt_contains_return)),
        ResolvedStmt::Labeled { body, .. } => stmt_contains_return(body),
        ResolvedStmt::Block { statements } => statements.iter().any(stmt_contains_return),
        _ => false,
    }
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
                        || block_assigns_any_name(&method.body, &[capture.to_string()])
                    {
                        mutable.push(capture.to_string());
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
                        || block_assigns_any_name(&method.body, &[capture.to_string()])
                    {
                        names.insert(capture.to_string());
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
pub(crate) struct DirectEvalBlockFunctionEnv {
    env_cell_names: HashSet<String>,
    heap_closure_names: HashSet<String>,
}

pub(super) fn block_contains_this(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_contains_this)
}

pub(super) fn block_contains_super(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_contains_super)
}

pub(crate) fn function_body_is_strict(parent_is_strict: bool, stmts: &[ResolvedStmt]) -> bool {
    parent_is_strict || block_has_use_strict_directive(stmts)
}

pub(crate) fn block_has_use_strict_directive(stmts: &[ResolvedStmt]) -> bool {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Expr(ResolvedExpr::String(value)) => {
                if value == "use strict" {
                    return true;
                }
            }
            _ => return false,
        }
    }
    false
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
            | ResolvedStmt::ForOf { body, .. }
            | ResolvedStmt::ForAwaitOf { body, .. } => {
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
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Function { .. }
            | ResolvedStmt::DestructureLet { .. }
            | ResolvedStmt::DestructureAssign { .. }
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

/// Compute recursion depth for each function by analyzing the call graph.
///
/// A function has recursion_depth >= 1 if it is part of a recursive cycle
/// (directly calls itself, or is part of a cycle through other functions).
/// Depth 0 means the function is not recursive. Depths above 1 indicate the
/// cycle length (2 for mutual recursion between two functions, etc.).
///
/// This is used by ABC451 runtime tracking to distinguish top-level array
/// growth (depth 0) from nested/recursive array growth (depth 1+).
fn compute_recursion_depths(
    program: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
) -> HashMap<FuncId, usize> {
    // Step 1: Build the call graph: for each FuncId, which function *names* does it call?
    let mut call_graph: HashMap<FuncId, HashSet<String>> = HashMap::new();
    for stmt in program {
        match stmt {
            ResolvedStmt::Function { name, body, .. } => {
                if let Some(&func_id) = function_ids.get(name.as_str()) {
                    let mut targets = HashSet::new();
                    collect_call_targets_in_stmts(body, &mut targets);
                    call_graph.insert(func_id, targets);
                }
            }
            ResolvedStmt::ClassDecl {
                name,
                constructor,
                methods,
                ..
            } => {
                let ctor_key = class_constructor_key(name);
                if let Some(&ctor_id) = function_ids.get(&ctor_key) {
                    let mut targets = HashSet::new();
                    if let Some((_, body)) = constructor {
                        collect_call_targets_in_stmts(body, &mut targets);
                    }
                    call_graph.insert(ctor_id, targets);
                }
                for method in methods {
                    let method_key = class_method_key(name, &method.name);
                    if let Some(&method_id) = function_ids.get(&method_key) {
                        let mut targets = HashSet::new();
                        collect_call_targets_in_stmts(&method.body, &mut targets);
                        call_graph.insert(method_id, targets);
                    }
                }
            }
            _ => {}
        }
    }

    // Step 2: Find strongly connected components (cycles) via DFS.
    // Functions in a non-trivial SCC (size > 1 or self-loop) are recursive.
    // Map FuncId -> node index (0..n)
    let mut func_to_idx: HashMap<FuncId, usize> = HashMap::new();
    let mut next_idx = 0;
    for (name, &id) in function_ids {
        if (call_graph.contains_key(&id)
            || program.iter().any(|stmt| match stmt {
                ResolvedStmt::Function { name: n, .. } => n == name,
                ResolvedStmt::ClassDecl { name: n, .. } => n == name,
                _ => false,
            }))
            && let std::collections::hash_map::Entry::Vacant(e) = func_to_idx.entry(id)
        {
            e.insert(next_idx);
            next_idx += 1;
        }
    }

    let n = func_to_idx.len();
    let mut idx_to_func: Vec<FuncId> = vec![FuncId(0); n];
    for (&id, &i) in &func_to_idx {
        idx_to_func[i] = id;
    }

    // Build adjacency list: node index -> [node indices of called functions]
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (&caller_id, callee_names) in &call_graph {
        if let Some(&caller_idx) = func_to_idx.get(&caller_id) {
            for callee_name in callee_names {
                if let Some(&callee_id) = function_ids.get(callee_name.as_str())
                    && let Some(&callee_idx) = func_to_idx.get(&callee_id)
                    && !adj[caller_idx].contains(&callee_idx)
                {
                    adj[caller_idx].push(callee_idx);
                }
            }
        }
    }

    // Kosaraju-style SCC detection via iterative Tarjan-like DFS.
    // We use a simpler approach: find self-loops first, then find 2-node cycles, etc.
    let mut recursion_depth: HashMap<FuncId, usize> = HashMap::new();

    // Helper: recursive DFS to find reachable nodes that lead back to the start.
    fn dfs_find_cycle(
        node: usize,
        adj: &[Vec<usize>],
        visited: &mut [bool],
        stack: &mut Vec<usize>,
        in_stack: &mut [bool],
        depths: &mut HashMap<FuncId, usize>,
        idx_to_func: &[FuncId],
    ) {
        visited[node] = true;
        stack.push(node);
        in_stack[node] = true;

        for &next in &adj[node] {
            if !visited[next] {
                dfs_find_cycle(next, adj, visited, stack, in_stack, depths, idx_to_func);
            } else if in_stack[next] {
                // Found a back edge: next -> ... -> node -> next is a cycle
                // Mark all nodes in the cycle as recursive (depth 1).
                let mut in_cycle = false;
                for &s in stack.iter() {
                    if s == next {
                        in_cycle = true;
                    }
                    if in_cycle {
                        depths.insert(idx_to_func[s], 1);
                    }
                    if s == node && in_cycle {
                        break;
                    }
                }
            }
        }

        stack.pop();
        in_stack[node] = false;
    }

    let mut visited = vec![false; n];
    let mut stack = Vec::new();
    let mut in_stack = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            dfs_find_cycle(
                i,
                &adj,
                &mut visited,
                &mut stack,
                &mut in_stack,
                &mut recursion_depth,
                &idx_to_func,
            );
        }
    }

    // Default depth 0 for functions not in cycles
    for &id in idx_to_func.iter() {
        recursion_depth.entry(id).or_insert(0);
    }

    recursion_depth
}

/// Collect all function names that are called from within a sequence of statements.
fn collect_call_targets_in_stmts(stmts: &[ResolvedStmt], targets: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            ResolvedStmt::Let(_, expr)
            | ResolvedStmt::Assign(_, expr)
            | ResolvedStmt::Expr(expr)
            | ResolvedStmt::Return(expr)
            | ResolvedStmt::Throw(expr) => {
                collect_call_targets_in_expr(expr, targets);
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_call_targets_in_expr(condition, targets);
                collect_call_targets_in_stmts(then_body, targets);
                collect_call_targets_in_stmts(else_body, targets);
            }
            ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
                collect_call_targets_in_expr(condition, targets);
                collect_call_targets_in_stmts(body, targets);
            }
            ResolvedStmt::For {
                init,
                condition,
                body,
                ..
            } => {
                if let Some(init) = init {
                    collect_call_targets_in_stmts(std::slice::from_ref(init.as_ref()), targets);
                }
                if let Some(condition) = condition {
                    collect_call_targets_in_expr(condition, targets);
                }
                collect_call_targets_in_stmts(body, targets);
            }
            ResolvedStmt::ForIn { iter, body, .. }
            | ResolvedStmt::ForOf { iter, body, .. }
            | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
                collect_call_targets_in_expr(iter, targets);
                collect_call_targets_in_stmts(body, targets);
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_call_targets_in_stmts(try_block, targets);
                if let Some(block) = catch_block {
                    collect_call_targets_in_stmts(block, targets);
                }
                if let Some(block) = finally_block {
                    collect_call_targets_in_stmts(block, targets);
                }
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_call_targets_in_stmts(body, targets);
                }
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_call_targets_in_stmts(std::slice::from_ref(body.as_ref()), targets);
            }
            ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
                collect_call_targets_in_expr(expr, targets);
            }
            ResolvedStmt::Block { statements, .. } => {
                collect_call_targets_in_stmts(statements, targets);
            }
            ResolvedStmt::DestructureLet { expr, .. }
            | ResolvedStmt::DestructureAssign { expr, .. } => {
                collect_call_targets_in_expr(expr, targets);
            }
            ResolvedStmt::Function { body, .. } => {
                // Nested functions: walk their body too
                collect_call_targets_in_stmts(body, targets);
            }
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. }
            | ResolvedStmt::ClassDecl { .. } => {}
        }
    }
}

/// Collect function call targets from a resolved expression tree.
fn collect_call_targets_in_expr(expr: &ResolvedExpr, targets: &mut HashSet<String>) {
    match expr {
        ResolvedExpr::Await { expr } => {
            collect_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::Yield { expr, .. } => {
            if let Some(expr) = expr {
                collect_call_targets_in_expr(expr, targets);
            }
        }
        ResolvedExpr::Call { callee, args, .. } => {
            // Record the callee if it's a direct function reference
            if let ResolvedExpr::Ident(name) = callee.as_ref() {
                targets.insert(name.clone());
            }
            for arg in args {
                collect_call_targets_in_expr(arg, targets);
            }
        }
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } => {
            // Record method name as a potential call target
            // (methods could be called on any object, including `this`)
            targets.insert(method.clone());
            collect_call_targets_in_expr(object, targets);
            for arg in args {
                collect_call_targets_in_expr(arg, targets);
            }
        }
        ResolvedExpr::Unary { expr, .. }
        | ResolvedExpr::BuiltinProperty { object: expr, .. }
        | ResolvedExpr::PropertyAccess { object: expr, .. }
        | ResolvedExpr::OptionalPropertyAccess { object: expr, .. } => {
            collect_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::Binary { left, right, .. }
        | ResolvedExpr::ComputedIndex {
            object: left,
            index: right,
        } => {
            collect_call_targets_in_expr(left, targets);
            collect_call_targets_in_expr(right, targets);
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_call_targets_in_expr(condition, targets);
            collect_call_targets_in_expr(then_expr, targets);
            collect_call_targets_in_expr(else_expr, targets);
        }
        ResolvedExpr::OptionalCall { callee, args, .. } => {
            if let ResolvedExpr::Ident(name) = callee.as_ref() {
                targets.insert(name.clone());
            }
            for arg in args {
                collect_call_targets_in_expr(arg, targets);
            }
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
            collect_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            collect_call_targets_in_expr(object, targets);
            collect_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            collect_call_targets_in_expr(key, targets);
            collect_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            collect_call_targets_in_expr(object, targets);
            collect_call_targets_in_expr(key, targets);
            collect_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::Array(elements) => {
            for element in elements {
                if let ResolvedArrayElement::Present(element_expr) = element {
                    collect_call_targets_in_expr(element_expr, targets);
                }
            }
        }
        ResolvedExpr::Object(props) => {
            for prop in props {
                if let Some(key) = prop.computed_key() {
                    collect_call_targets_in_expr(key, targets);
                }
                collect_call_targets_in_expr(prop.value(), targets);
            }
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            for arg in args {
                collect_call_targets_in_expr(arg, targets);
            }
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            for arg in &plan.args {
                collect_call_targets_in_expr(arg, targets);
            }
        }
        ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            collect_call_targets_in_expr(object, targets);
            collect_call_targets_in_expr(index, targets);
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            collect_call_targets_in_expr(object, targets);
            collect_call_targets_in_expr(value, targets);
        }
        ResolvedExpr::ArrowFn { body, .. } => {
            collect_call_targets_in_expr(body, targets);
        }
        ResolvedExpr::FunctionExpr { name, body, .. } => {
            targets.insert(name.clone());
            collect_call_targets_in_stmts(body, targets);
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            collect_call_targets_in_expr(object, targets);
            collect_call_targets_in_expr(key, targets);
            collect_call_targets_in_expr(value, targets);
        }
        ResolvedExpr::Spread(expr) => {
            collect_call_targets_in_expr(expr, targets);
        }
        ResolvedExpr::Sequence(exprs) => {
            for e in exprs {
                collect_call_targets_in_expr(e, targets);
            }
        }
        ResolvedExpr::EvalCompletion(steps) => {
            for step in steps {
                if let Some(expr) = step.expr() {
                    collect_call_targets_in_expr(expr, targets);
                }
            }
        }
        ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::ModuleLoad { .. }
        | ResolvedExpr::ClassExpr { .. } => {}
        ResolvedExpr::Eval { .. } => {}
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
            | ResolvedStmt::ForOf { body, .. }
            | ResolvedStmt::ForAwaitOf { body, .. } => collect_declared_function_names(body, names),
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
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Let(_, _)
            | ResolvedStmt::DestructureLet { .. }
            | ResolvedStmt::DestructureAssign { .. }
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
        | ResolvedStmt::ForOf { body, .. }
        | ResolvedStmt::ForAwaitOf { body, .. } => block_returns_any_name(body, names),
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
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::Let(_, _)
        | ResolvedStmt::DestructureLet { .. }
        | ResolvedStmt::DestructureAssign { .. }
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
        | ResolvedStmt::DestructureAssign { expr, .. }
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
                    case_expr.as_ref().is_some_and(expr_contains_this) || block_contains_this(body)
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
        ResolvedStmt::ForIn { iter, body, .. }
        | ResolvedStmt::ForOf { iter, body, .. }
        | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
            expr_contains_this(iter) || block_contains_this(body)
        }
        ResolvedStmt::Labeled { body, .. } => stmt_contains_this(body),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_contains_this(expr)
        }
        ResolvedStmt::Block { statements, .. } => block_contains_this(statements),
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

fn expr_contains_this(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Await { expr } => expr_contains_this(expr),
        ResolvedExpr::Yield { expr, .. } => expr.as_deref().is_some_and(expr_contains_this),
        ResolvedExpr::This { .. } => true,
        ResolvedExpr::NewTarget { .. } => false,
        ResolvedExpr::ImportMeta { .. } => false,
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => expr_contains_this(expr),
        ResolvedExpr::Binary { left, right, .. } => {
            expr_contains_this(left) || expr_contains_this(right)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_this(condition)
                || expr_contains_this(then_expr)
                || expr_contains_this(else_expr)
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
        ResolvedExpr::Object(props) => props.iter().any(|prop| {
            prop.computed_key().is_some_and(expr_contains_this) || expr_contains_this(prop.value())
        }),
        ResolvedExpr::ComputedIndex { object, index } => {
            expr_contains_this(object) || expr_contains_this(index)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(expr_contains_this)
        }
        ResolvedExpr::FunctionConstructor { plan } => plan.args.iter().any(expr_contains_this),
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
        ResolvedExpr::Sequence(exprs) => exprs.iter().any(expr_contains_this),
        ResolvedExpr::EvalCompletion(steps) => steps
            .iter()
            .filter_map(|step| step.expr())
            .any(expr_contains_this),
        ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Eval { .. } => false,
        ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. } => false,
    }
}

fn stmt_contains_super(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::Assign(_, expr)
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr) => expr_contains_super(expr),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_contains_super(condition)
                || block_contains_super(then_body)
                || block_contains_super(else_body)
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { condition, body } => {
            expr_contains_super(condition) || block_contains_super(body)
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            init.as_ref().is_some_and(|stmt| stmt_contains_super(stmt))
                || condition.as_ref().is_some_and(expr_contains_super)
                || update.as_ref().is_some_and(expr_contains_super)
                || block_contains_super(body)
        }
        ResolvedStmt::ForOf { iter, body, .. }
        | ResolvedStmt::ForIn { iter, body, .. }
        | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
            expr_contains_super(iter) || block_contains_super(body)
        }
        ResolvedStmt::Block { statements, .. } => block_contains_super(statements),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_contains_super(expr)
        }
        ResolvedStmt::Switch { expr, cases } => {
            expr_contains_super(expr)
                || cases.iter().any(|(case_expr, body)| {
                    case_expr.as_ref().is_some_and(expr_contains_super)
                        || block_contains_super(body)
                })
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_contains_super(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block_contains_super(block))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block_contains_super(block))
        }
        ResolvedStmt::Labeled { body, .. } => stmt_contains_super(body),
        ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::DestructureAssign { expr, .. } => expr_contains_super(expr),
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

fn expr_contains_super(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Ident(name) => name == "super",
        ResolvedExpr::Await { expr } => expr_contains_super(expr),
        ResolvedExpr::Yield { expr, .. } => expr.as_deref().is_some_and(expr_contains_super),
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => expr_contains_super(expr),
        ResolvedExpr::Binary { left, right, .. } => {
            expr_contains_super(left) || expr_contains_super(right)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_super(condition)
                || expr_contains_super(then_expr)
                || expr_contains_super(else_expr)
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            expr_contains_super(callee) || args.iter().any(expr_contains_super)
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => expr_contains_super(expr),
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_contains_super(object) || expr_contains_super(expr)
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_contains_super(key) || expr_contains_super(expr)
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => expr_contains_super(object) || expr_contains_super(key) || expr_contains_super(expr),
        ResolvedExpr::Array(elements) => elements.iter().any(|element| match element {
            ResolvedArrayElement::Present(expr) => expr_contains_super(expr),
            ResolvedArrayElement::Hole => false,
        }),
        ResolvedExpr::Object(props) => props.iter().any(|prop| {
            prop.computed_key().is_some_and(expr_contains_super)
                || expr_contains_super(prop.value())
        }),
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            expr_contains_super(object) || expr_contains_super(index)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(expr_contains_super)
        }
        ResolvedExpr::FunctionConstructor { plan } => plan.args.iter().any(expr_contains_super),
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => expr_contains_super(object),
        ResolvedExpr::MethodCall { object, args, .. } => {
            expr_contains_super(object) || args.iter().any(expr_contains_super)
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            expr_contains_super(object) || expr_contains_super(value)
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            expr_contains_super(object) || expr_contains_super(key) || expr_contains_super(value)
        }
        ResolvedExpr::ArrowFn { body, .. } => expr_contains_super(body),
        ResolvedExpr::Sequence(exprs) => exprs.iter().any(expr_contains_super),
        ResolvedExpr::EvalCompletion(steps) => steps
            .iter()
            .filter_map(|step| step.expr())
            .any(expr_contains_super),
        ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Eval { .. }
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::ModuleLoad { .. } => false,
    }
}

pub(super) fn block_contains_arguments(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_contains_arguments)
}

pub(super) fn block_contains_new_target(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_contains_new_target)
}

pub(crate) fn direct_iife_body_has_static_eval_block_function_binding(
    stmts: &[ResolvedStmt],
) -> bool {
    stmts.iter().any(|stmt| {
        matches!(
            stmt,
            ResolvedStmt::Let(_, ResolvedExpr::Undefined) | ResolvedStmt::Function { .. }
        )
    })
}

pub(crate) fn direct_iife_body_has_unsupported_return(stmts: &[ResolvedStmt]) -> bool {
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
        | ResolvedStmt::ForOf { body, .. }
        | ResolvedStmt::ForAwaitOf { body, .. } => direct_iife_body_has_unsupported_return(body),
        ResolvedStmt::Labeled { body, .. } => stmt_has_direct_return(body),
        ResolvedStmt::Block { statements, .. } => {
            direct_iife_body_has_unsupported_return(statements)
        }
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Let(_, _)
        | ResolvedStmt::DestructureLet { .. }
        | ResolvedStmt::DestructureAssign { .. }
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
        | ResolvedStmt::DestructureAssign { expr, .. }
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
            init.as_ref()
                .is_some_and(|stmt| stmt_contains_arguments(stmt))
                || condition.as_ref().is_some_and(expr_contains_arguments)
                || update.as_ref().is_some_and(expr_contains_arguments)
                || block_contains_arguments(body)
        }
        ResolvedStmt::ForIn { iter, body, .. }
        | ResolvedStmt::ForOf { iter, body, .. }
        | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
            expr_contains_arguments(iter) || block_contains_arguments(body)
        }
        ResolvedStmt::Labeled { body, .. } => stmt_contains_arguments(body),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_contains_arguments(expr)
        }
        ResolvedStmt::Block { statements, .. } => block_contains_arguments(statements),
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

fn stmt_contains_new_target(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::DestructureLet { expr, .. }
        | ResolvedStmt::DestructureAssign { expr, .. }
        | ResolvedStmt::Assign(_, expr)
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr) => expr_contains_new_target(expr),
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_contains_new_target(condition)
                || block_contains_new_target(then_body)
                || block_contains_new_target(else_body)
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { body, condition } => {
            expr_contains_new_target(condition) || block_contains_new_target(body)
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            init.as_ref()
                .is_some_and(|stmt| stmt_contains_new_target(stmt))
                || condition.as_ref().is_some_and(expr_contains_new_target)
                || update.as_ref().is_some_and(expr_contains_new_target)
                || block_contains_new_target(body)
        }
        ResolvedStmt::ForIn { iter, body, .. }
        | ResolvedStmt::ForOf { iter, body, .. }
        | ResolvedStmt::ForAwaitOf { iter, body, .. } => {
            expr_contains_new_target(iter) || block_contains_new_target(body)
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_contains_new_target(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block_contains_new_target(block))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block_contains_new_target(block))
        }
        ResolvedStmt::Switch { expr, cases } => {
            expr_contains_new_target(expr)
                || cases.iter().any(|(case_expr, body)| {
                    case_expr.as_ref().is_some_and(expr_contains_new_target)
                        || block_contains_new_target(body)
                })
        }
        ResolvedStmt::Labeled { body, .. } => stmt_contains_new_target(body),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_contains_new_target(expr)
        }
        ResolvedStmt::Block { statements, .. } => block_contains_new_target(statements),
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Function { .. }
        | ResolvedStmt::ClassDecl { .. }
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
    }
}

fn expr_contains_new_target(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::NewTarget { .. } => true,
        ResolvedExpr::Await { expr } => expr_contains_new_target(expr),
        ResolvedExpr::Yield { expr, .. } => expr.as_deref().is_some_and(expr_contains_new_target),
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => {
            expr_contains_new_target(expr)
        }
        ResolvedExpr::Binary { left, right, .. } => {
            expr_contains_new_target(left) || expr_contains_new_target(right)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_new_target(condition)
                || expr_contains_new_target(then_expr)
                || expr_contains_new_target(else_expr)
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            expr_contains_new_target(callee) || args.iter().any(expr_contains_new_target)
        }
        ResolvedExpr::Assign { expr, .. }
        | ResolvedExpr::LogicalAssign { expr, .. }
        | ResolvedExpr::LogicalPropertyAssign { expr, .. } => expr_contains_new_target(expr),
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_contains_new_target(object) || expr_contains_new_target(expr)
        }
        ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_contains_new_target(key) || expr_contains_new_target(expr)
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            expr_contains_new_target(object)
                || expr_contains_new_target(key)
                || expr_contains_new_target(expr)
        }
        ResolvedExpr::Array(elements) => elements.iter().any(|element| match element {
            ResolvedArrayElement::Present(expr) => expr_contains_new_target(expr),
            ResolvedArrayElement::Hole => false,
        }),
        ResolvedExpr::Object(props) => props.iter().any(|prop| {
            prop.computed_key().is_some_and(expr_contains_new_target)
                || expr_contains_new_target(prop.value())
        }),
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            expr_contains_new_target(object) || expr_contains_new_target(index)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(expr_contains_new_target)
        }
        ResolvedExpr::FunctionConstructor { plan } => {
            plan.args.iter().any(expr_contains_new_target)
        }
        ResolvedExpr::BuiltinProperty { object, .. }
        | ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => expr_contains_new_target(object),
        ResolvedExpr::MethodCall { object, args, .. } => {
            expr_contains_new_target(object) || args.iter().any(expr_contains_new_target)
        }
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            expr_contains_new_target(object) || expr_contains_new_target(value)
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            expr_contains_new_target(object)
                || expr_contains_new_target(key)
                || expr_contains_new_target(value)
        }
        ResolvedExpr::ArrowFn { body, .. } => expr_contains_new_target(body),
        ResolvedExpr::Sequence(exprs) => exprs.iter().any(expr_contains_new_target),
        ResolvedExpr::EvalCompletion(steps) => steps
            .iter()
            .filter_map(|step| step.expr())
            .any(expr_contains_new_target),
        ResolvedExpr::FunctionExpr { .. }
        | ResolvedExpr::ClassExpr { .. }
        | ResolvedExpr::This { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Eval { .. }
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::Ident(_)
        | ResolvedExpr::ModuleLoad { .. } => false,
    }
}

fn expr_contains_arguments(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Await { expr } => expr_contains_arguments(expr),
        ResolvedExpr::Yield { expr, .. } => expr.as_deref().is_some_and(expr_contains_arguments),
        ResolvedExpr::Ident(name) => name == "arguments",
        ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => {
            expr_contains_arguments(expr)
        }
        ResolvedExpr::Binary { left, right, .. } => {
            expr_contains_arguments(left) || expr_contains_arguments(right)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_arguments(condition)
                || expr_contains_arguments(then_expr)
                || expr_contains_arguments(else_expr)
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
        ResolvedExpr::LogicalComputedPropertyAssign {
            object, key, expr, ..
        } => object == "arguments" || expr_contains_arguments(key) || expr_contains_arguments(expr),
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
        ResolvedExpr::Object(props) => props.iter().any(|prop| {
            prop.computed_key().is_some_and(expr_contains_arguments)
                || expr_contains_arguments(prop.value())
        }),
        ResolvedExpr::ComputedIndex { object, index } => {
            expr_contains_arguments(object) || expr_contains_arguments(index)
        }
        ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
            args.iter().any(expr_contains_arguments)
        }
        ResolvedExpr::FunctionConstructor { plan } => plan.args.iter().any(expr_contains_arguments),
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
        ResolvedExpr::Sequence(exprs) => exprs.iter().any(expr_contains_arguments),
        ResolvedExpr::EvalCompletion(steps) => steps
            .iter()
            .filter_map(|step| step.expr())
            .any(expr_contains_arguments),
        ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::ImportMeta { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::DecimalNumber(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::Eval { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::ModuleLoad { .. } => false,
    }
}

pub(crate) struct LowerFunctionOptions<'a> {
    pub(crate) current_class: Option<&'a str>,
    pub(crate) in_constructor: bool,
    pub(crate) in_static_method: bool,
    pub(crate) next_func_id: usize,
    pub(crate) self_closure: Option<SelfClosureOptions<'a>>,
    pub(crate) capture_facts: FunctionCaptureFacts,
    /// Recursion depth for this function (0 = not recursive, 1+ = recursive).
    pub(crate) recursion_depth: usize,
    pub(crate) new_target_class: Option<&'a str>,
    pub(crate) module_url: &'a str,
    pub(crate) strict_context: bool,
    pub(crate) type_aliases: &'a HashMap<String, ts2wasm_syntax::TypeRef>,
    pub(crate) interface_definitions: &'a HashMap<String, Vec<(String, ts2wasm_syntax::TypeRef)>>,
}

pub(crate) struct SelfClosureOptions<'a> {
    pub(crate) name: &'a str,
    pub(crate) func_id: FuncId,
    pub(crate) capture_names: &'a [String],
    pub(crate) object_function_props: Option<&'a HashMap<ObjectAccessorKey, FuncId>>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct FunctionCaptureFacts {
    pub(crate) local_classes: HashMap<String, String>,
    pub(crate) intl_number_format_options: HashMap<String, IntlNumberFormatOptions>,
    pub(crate) object_function_props: HashMap<String, HashMap<ObjectAccessorKey, FuncId>>,
    pub(crate) object_accessor_props:
        HashMap<String, HashMap<ObjectAccessorKey, ObjectAccessorProp>>,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn lower_function(
    id: FuncId,
    params: &[ResolvedParam],
    body: &[ResolvedStmt],
    is_generator: bool,
    is_async: bool,
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
    let signature = function_signatures.get(&id).cloned().unwrap_or_default();
    let min_required_params = params
        .iter()
        .filter(|param| param.default.is_none() && !param.is_rest)
        .count()
        + usize::from(signature.needs_receiver)
        + usize::from(signature.needs_arguments);
    let mut lowered_params = Vec::new();
    if signature.needs_receiver {
        lowered_params.push(ResolvedParam {
            name: "this".to_owned(),
            default: None,
            is_rest: false,
            span: None,
        });
    }
    let mut rest_param = None;
    for param in params {
        if param.is_rest {
            rest_param = Some(param.clone());
        } else {
            lowered_params.push(param.clone());
        }
    }
    if signature.needs_new_target {
        lowered_params.push(ResolvedParam {
            name: SYNTHETIC_NEW_TARGET_PARAM.to_owned(),
            default: None,
            is_rest: false,
            span: None,
        });
    }
    if signature.needs_arguments {
        let synthetic_arguments_param_index = lowered_params.len();
        lowered_params.push(ResolvedParam {
            name: "arguments".to_owned(),
            default: None,
            is_rest: false,
            span: None,
        });
        let rest_param_index = rest_param.as_ref().map(|_| lowered_params.len());
        if let Some(param) = rest_param {
            lowered_params.push(param);
        }
        return lower_function_with_resolved_params(
            id,
            lowered_params,
            rest_param_index,
            Some(synthetic_arguments_param_index),
            min_required_params,
            body,
            is_generator,
            is_async,
            function_ids,
            function_signatures,
            function_captures,
            function_mutable_captures,
            class_method_captures,
            class_method_mutable_captures,
            env_cell_names,
            heap_closure_names,
            class_parents,
            class_private_fields,
            class_static_private_fields,
            options,
        );
    }
    let rest_param_index = rest_param.as_ref().map(|_| lowered_params.len());
    if let Some(param) = rest_param {
        lowered_params.push(param);
    }
    let synthetic_arguments_param_index =
        lowered_params
            .iter()
            .enumerate()
            .find_map(|(index, param)| {
                let name = param.name.strip_prefix("...").unwrap_or(&param.name);
                (index >= signature.explicit_params && name == "arguments").then_some(index)
            });

    lower_function_with_resolved_params(
        id,
        lowered_params,
        rest_param_index,
        synthetic_arguments_param_index,
        min_required_params,
        body,
        is_generator,
        is_async,
        function_ids,
        function_signatures,
        function_captures,
        function_mutable_captures,
        class_method_captures,
        class_method_mutable_captures,
        env_cell_names,
        heap_closure_names,
        class_parents,
        class_private_fields,
        class_static_private_fields,
        options,
    )
}

#[allow(clippy::too_many_arguments)]
fn lower_function_with_resolved_params(
    id: FuncId,
    lowered_params: Vec<ResolvedParam>,
    rest_param_index: Option<usize>,
    synthetic_arguments_param_index: Option<usize>,
    min_required_params: usize,
    body: &[ResolvedStmt],
    is_generator: bool,
    is_async: bool,
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
    let signature = function_signatures.get(&id).cloned().unwrap_or_default();
    let is_strict_context = function_body_is_strict(options.strict_context, body);
    let (mut resolver, param_ids) = crate::lowered::resolver::Resolver::with_params(
        function_ids,
        function_signatures,
        HashMap::new(),
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
        id,
        synthetic_arguments_param_index,
        class_parents,
        class_private_fields,
        class_static_private_fields,
        options.current_class,
        options.in_constructor,
        options.new_target_class,
        options.next_func_id,
        options.module_url,
        is_strict_context,
        options.type_aliases.clone(),
        options.interface_definitions.clone(),
    )?;
    resolver.ctx.classes.in_static_method = options.in_static_method;

    if let Some(self_closure) = options.self_closure {
        resolver.declare_self_closure(
            self_closure.name,
            self_closure.func_id,
            self_closure.capture_names,
            self_closure.object_function_props,
        )?;
    }

    for (name, class_name) in &options.capture_facts.local_classes {
        if let Ok(local_id) = resolver.resolve_local(name) {
            resolver
                .ctx
                .classes
                .local_classes
                .insert(local_id, class_name.clone());
        }
    }
    for (name, options) in &options.capture_facts.intl_number_format_options {
        if let Ok(local_id) = resolver.resolve_local(name) {
            resolver
                .ctx
                .facts
                .intl_number_format_locals
                .insert(local_id, options.clone());
        }
    }
    for (name, props) in &options.capture_facts.object_function_props {
        if let Ok(local_id) = resolver.resolve_local(name) {
            resolver
                .ctx
                .classes
                .object_function_props
                .insert(local_id, props.clone());
        }
    }
    for (name, props) in &options.capture_facts.object_accessor_props {
        if let Ok(local_id) = resolver.resolve_local(name) {
            resolver
                .ctx
                .classes
                .object_accessor_props
                .insert(local_id, props.clone());
        }
    }

    let capture_param_names = function_captures
        .get(&id)
        .into_iter()
        .chain(class_method_captures.get(&id))
        .flat_map(|names| names.iter())
        .collect::<HashSet<_>>();
    let mut body_with_defaults = lower_function_param_initializers(&lowered_params, &mut resolver)?;
    lower_function_param_env_cells(
        &lowered_params,
        &param_ids,
        &capture_param_names,
        &mut resolver,
        &mut body_with_defaults,
    );
    predeclare_function_body_locals(body, &mut resolver)?;
    lower_dynamic_direct_eval_created_bindings(body, &mut resolver, &mut body_with_defaults)?;
    body_with_defaults.extend(resolver.lower_block(body)?);
    let generator_state = if is_generator {
        Some(generator_state_for_body(&body_with_defaults))
    } else {
        None
    };

    Ok(FunctionLowering {
        function: LoweredFunction {
            id,
            params: param_ids,
            uses_receiver: signature.needs_receiver,
            min_required_params,
            rest_param_index,
            metadata_length: signature.metadata_length,
            metadata_name: signature.metadata_name.clone(),
            locals: resolver.ctx.symbols.locals,
            body: body_with_defaults,
            recursion_depth: options.recursion_depth,
            is_async,
            is_generator,
            generator_state,
        },
        generated_functions: resolver.ctx.functions.generated_functions,
        next_func_id: resolver.ctx.functions.next_func_id,
    })
}

fn lower_function_param_initializers(
    lowered_params: &[ResolvedParam],
    resolver: &mut crate::lowered::resolver::Resolver,
) -> Result<Vec<LoweredStmt>, Diagnostic> {
    let mut stmts = Vec::new();
    for param in lowered_params {
        let clean_name = param.name.strip_prefix("...").unwrap_or(&param.name);
        let param_local = resolver.resolve_local(clean_name)?;
        if let Some(default) = &param.default {
            stmts.push(default_param_assignment(
                param_local,
                resolver.lower_expr(default)?,
            ));
        }
        if let Some(pattern) = parse_binding_pattern(&param.name, param.span)? {
            if param.is_rest {
                // Handle rest pattern binding
                if let BindingPattern::Array(_) = pattern {
                    // Array rest: [...rest]
                    // Create a local for the rest array
                    let rest_local = resolver
                        .declare_local(param.name.strip_prefix("...").unwrap_or(&param.name))?;
                    // Use runtime ArraySlice for array rest binding
                    // We need to call lower_array_binding_declaration with a single ArrayBinding
                    // Create one ArrayBinding for the rest element
                    let rest_binding = ArrayBinding {
                        index: 0,
                        target: BindingTarget::Identifier(
                            param
                                .name
                                .strip_prefix("...")
                                .unwrap_or(&param.name)
                                .to_string(),
                        ),
                        default: None,
                        is_rest: true,
                    };
                    stmts.extend(resolver.lower_array_binding_declaration(
                        &rest_binding,
                        &LoweredExpr::Local(rest_local, Span::generated("local")),
                    )?);
                }
                continue;
            }
            stmts.extend(resolver.lower_binding_pattern_declarations(
                &pattern,
                LoweredExpr::Local(param_local, Span::generated("local")),
                None,
            )?);
        }
    }
    Ok(stmts)
}

fn default_param_assignment(param_local: LocalId, lowered_default: LoweredExpr) -> LoweredStmt {
    LoweredStmt::If {
        condition: LoweredExpr::Binary {
            left: Box::new(LoweredExpr::Local(param_local, Span::generated("local"))),
            op: LoweredBinaryOp::StrictEqual,
            right: Box::new(LoweredExpr::Undefined(Span::generated("undefined"))),
            span: Span::generated("binary"),
        },
        then_body: vec![LoweredStmt::Assign(
            param_local,
            lowered_default,
            Span::generated("assign"),
        )],
        else_body: vec![],
        span: Span::generated("if_stmt"),
    }
}

fn lower_function_param_env_cells(
    lowered_params: &[ResolvedParam],
    param_ids: &[LocalId],
    capture_param_names: &HashSet<&String>,
    resolver: &mut crate::lowered::resolver::Resolver,
    stmts: &mut Vec<LoweredStmt>,
) {
    for (param, param_id) in lowered_params.iter().zip(param_ids.iter().copied()) {
        let clean_name = param.name.strip_prefix("...").unwrap_or(&param.name);
        if capture_param_names.contains(&clean_name.to_owned()) {
            continue;
        }
        if resolver.ctx.facts.env_cell_locals.contains(&param_id) {
            stmts.push(LoweredStmt::Assign(
                param_id,
                LoweredExpr::EnvCellNew(
                    Box::new(LoweredExpr::Local(param_id, Span::generated("local"))),
                    Span::generated("env_cell_new"),
                ),
                Span::generated("assign"),
            ));
        }
    }
}

fn predeclare_function_body_locals(
    body: &[ResolvedStmt],
    resolver: &mut crate::lowered::resolver::Resolver,
) -> Result<(), Diagnostic> {
    for stmt in body {
        if let ResolvedStmt::Let(name, _) = stmt {
            resolver.declare_local(name)?;
        }
    }
    Ok(())
}

fn lower_dynamic_direct_eval_created_bindings(
    body: &[ResolvedStmt],
    resolver: &mut crate::lowered::resolver::Resolver,
    stmts: &mut Vec<LoweredStmt>,
) -> Result<(), Diagnostic> {
    let eval_created_function_names = collect_dynamic_direct_eval_created_function_names(body);
    let mut eval_created_names = collect_dynamic_direct_eval_created_binding_names(body)
        .into_iter()
        .collect::<Vec<_>>();
    eval_created_names.sort();
    for name in eval_created_names {
        if resolver.ctx.symbols.resolve(&name).is_some() {
            continue;
        }
        let local_id = resolver.declare_local(&name)?;
        if eval_created_function_names.contains(&name) {
            resolver
                .ctx
                .facts
                .mark_host_external(local_id, HostExternalKind::FunctionHandle, true);
        }
        stmts.push(dynamic_eval_created_binding_let(&name, local_id, resolver));
    }
    Ok(())
}

fn dynamic_eval_created_binding_let(
    name: &str,
    local_id: LocalId,
    resolver: &mut crate::lowered::resolver::Resolver,
) -> LoweredStmt {
    if resolver.ctx.facts.env_cell_names.contains(name) {
        resolver.ctx.facts.env_cell_locals.insert(local_id);
        resolver
            .ctx
            .facts
            .initialized_env_cell_locals
            .insert(local_id);
        LoweredStmt::Let(
            local_id,
            LoweredExpr::EnvCellNew(
                Box::new(LoweredExpr::Undefined(Span::generated("undefined"))),
                Span::generated("env_cell_new"),
            ),
            Span::generated("direct_eval_created_binding"),
        )
    } else {
        LoweredStmt::Let(
            local_id,
            LoweredExpr::Undefined(Span::generated("undefined")),
            Span::generated("direct_eval_created_binding"),
        )
    }
}

fn generator_state_for_body(body: &[LoweredStmt]) -> GeneratorState {
    let mut suspend_points = Vec::new();
    collect_suspend_points(body, &mut suspend_points);
    if suspend_points.is_empty() {
        GeneratorState::empty()
    } else {
        GeneratorState {
            completed_state: suspend_points.len() + 1,
            suspend_points,
        }
    }
}

fn collect_suspend_points(stmts: &[LoweredStmt], suspend_points: &mut Vec<SuspendPoint>) {
    for stmt in stmts {
        match stmt {
            LoweredStmt::Yield(_, _) => {
                let index = suspend_points.len();
                suspend_points.push(SuspendPoint {
                    index,
                    resume_state: index + 1,
                });
            }
            LoweredStmt::Block(stmts, _)
            | LoweredStmt::While { body: stmts, .. }
            | LoweredStmt::DoWhile { body: stmts, .. }
            | LoweredStmt::ForIn { body: stmts, .. }
            | LoweredStmt::ForOf { body: stmts, .. }
            | LoweredStmt::ForAwaitOfLower { body: stmts, .. } => {
                collect_suspend_points(stmts, suspend_points);
            }
            LoweredStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_suspend_points(then_body, suspend_points);
                collect_suspend_points(else_body, suspend_points);
            }
            LoweredStmt::For { init, body, .. } => {
                if let Some(init) = init {
                    collect_suspend_points(std::slice::from_ref(init.as_ref()), suspend_points);
                }
                collect_suspend_points(body, suspend_points);
            }
            LoweredStmt::TryFinally {
                try_body,
                finally_body,
                ..
            } => {
                collect_suspend_points(try_body, suspend_points);
                collect_suspend_points(finally_body, suspend_points);
            }
            LoweredStmt::TryCatch {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                collect_suspend_points(try_body, suspend_points);
                if let Some(catch_body) = catch_body {
                    collect_suspend_points(catch_body, suspend_points);
                }
                if let Some(finally_body) = finally_body {
                    collect_suspend_points(finally_body, suspend_points);
                }
            }
            LoweredStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_suspend_points(body, suspend_points);
                }
            }
            LoweredStmt::Labeled { body, .. } => {
                collect_suspend_points(std::slice::from_ref(body.as_ref()), suspend_points);
            }
            LoweredStmt::Let(_, _, _)
            | LoweredStmt::Assign(_, _, _)
            | LoweredStmt::Expr(_, _)
            | LoweredStmt::Return(_, _)
            | LoweredStmt::Throw(_, _)
            | LoweredStmt::Break { .. }
            | LoweredStmt::Continue { .. }
            | LoweredStmt::Export { .. }
            | LoweredStmt::ModuleExportsUpdate { .. }
            | LoweredStmt::ModuleExportsAssign { .. }
            | LoweredStmt::ClassDecl { .. } => {}
        }
    }
}

pub(super) fn lower_binary_op(op: BinaryOp) -> Result<LoweredBinaryOp, Diagnostic> {
    match op {
        BinaryOp::Add => Ok(LoweredBinaryOp::Add),
        BinaryOp::Subtract => Ok(LoweredBinaryOp::Subtract),
        BinaryOp::Multiply => Ok(LoweredBinaryOp::Multiply),
        BinaryOp::Power => Ok(LoweredBinaryOp::Power),
        BinaryOp::Divide => Ok(LoweredBinaryOp::Divide),
        BinaryOp::Modulo => Ok(LoweredBinaryOp::Modulo),
        BinaryOp::BitwiseAnd => Ok(LoweredBinaryOp::BitwiseAnd),
        BinaryOp::BitwiseXor => Ok(LoweredBinaryOp::BitwiseXor),
        BinaryOp::BitwiseOr => Ok(LoweredBinaryOp::BitwiseOr),
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
        BinaryOp::LeftShift
        | BinaryOp::RightShift
        | BinaryOp::UnsignedRightShift
        | BinaryOp::In
        | BinaryOp::InstanceOf
        | BinaryOp::Pipeline => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("binary operator {:?} not yet supported", op),
            span: None,

            phase: None,
        }),
    }
}

pub(super) fn lower_logical_assign_op(op: LogicalAssignOp) -> LoweredLogicalAssignOp {
    match op {
        LogicalAssignOp::And => LoweredLogicalAssignOp::And,
        LogicalAssignOp::Or => LoweredLogicalAssignOp::Or,
        LogicalAssignOp::Nullish => LoweredLogicalAssignOp::Nullish,
    }
}

pub(super) fn lower_unary_op(op: UnaryOp) -> Result<LoweredUnaryOp, Diagnostic> {
    match op {
        UnaryOp::Not => Ok(LoweredUnaryOp::Not),
        UnaryOp::Plus => Ok(LoweredUnaryOp::Plus),
        UnaryOp::Negate => Ok(LoweredUnaryOp::Negate),
        UnaryOp::TypeOf => Ok(LoweredUnaryOp::TypeOf),
        UnaryOp::Delete => Ok(LoweredUnaryOp::Delete),
        UnaryOp::Increment | UnaryOp::Decrement | UnaryOp::PreIncrement | UnaryOp::PreDecrement => {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("issue-268: unary operator {:?} not yet supported", op),
                span: None,

                phase: None,
            })
        }
        UnaryOp::BitwiseNot => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("unary operator {:?} not yet supported", op),
            span: None,

            phase: None,
        }),
        UnaryOp::Void => Ok(LoweredUnaryOp::Void),
    }
}

/// Collect the set of builtin class names that are needed as extends targets.
/// Includes all names that appear as a parent class in `class_parents` and
/// are known builtins, plus all of their transitive ancestors (e.g., if
/// "RegExp" is needed, "Object" is also needed because RegExp extends Object).
fn collect_needed_builtins(
    class_parents: &HashMap<String, Option<String>>,
    builtin_class_parents: &[(&str, Option<&str>)],
) -> HashSet<String> {
    let builtin_names: HashSet<&str> = builtin_class_parents.iter().map(|(n, _)| *n).collect();
    let mut needed: HashSet<String> = HashSet::new();
    // Find builtins that appear as extends targets.
    for (_, extends) in class_parents {
        if let Some(parent) = extends {
            if builtin_names.contains(parent.as_str()) {
                needed.insert(parent.clone());
            }
        }
    }
    // Add transitive ancestors.
    let builtin_parent_map: HashMap<&str, Option<&str>> = builtin_class_parents
        .iter()
        .map(|(n, p)| (*n, *p))
        .collect();
    let mut queue: Vec<String> = needed.iter().cloned().collect();
    while let Some(name) = queue.pop() {
        if let Some(Some(parent)) = builtin_parent_map.get(name.as_str()) {
            let parent_str = parent.to_string();
            if needed.insert(parent_str.clone()) {
                queue.push(parent_str);
            }
        }
    }
    needed
}
