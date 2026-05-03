#[path = "resolver_expr.rs"]
mod resolver_expr;
#[path = "resolver_extra.rs"]
mod resolver_extra;
struct Resolver<'a> {
    function_ids: &'a HashMap<String, FuncId>,
    function_signatures: &'a HashMap<FuncId, FunctionSignature>,
    function_captures: &'a HashMap<FuncId, Vec<String>>,
    function_mutable_captures: &'a HashMap<FuncId, Vec<String>>,
    class_method_captures: &'a HashMap<FuncId, Vec<String>>,
    class_method_mutable_captures: &'a HashMap<FuncId, Vec<String>>,
    env_cell_names: HashSet<String>,
    env_cell_locals: HashSet<LocalId>,
    heap_closure_names: HashSet<String>,
    scopes: Vec<HashMap<String, LocalId>>,
    next_local_id: usize,
    locals: Vec<LocalId>,
    next_func_id: usize,
    generated_functions: Vec<LoweredFunction>,
    arrow_locals: HashMap<LocalId, ArrowClosure>,
    heap_closure_locals: HashSet<LocalId>,
    nullish_locals: HashSet<LocalId>,
    module_ids: HashMap<String, usize>,
    modules: Vec<ModuleInfo>,
    class_constructor_ids: HashMap<String, FuncId>,
    class_method_ids: HashMap<(String, String), FuncId>,
    class_static_method_ids: HashMap<(String, String), FuncId>,
    class_parents: HashMap<String, Option<String>>,
    class_private_fields: ClassPrivateFieldSlots,
    class_static_private_fields: ClassStaticPrivateFields,
    local_classes: HashMap<LocalId, String>,
    object_function_props: HashMap<LocalId, HashMap<String, FuncId>>,
    regexp_literal_locals: HashSet<LocalId>,
    bigint_locals: HashSet<LocalId>,
    control_flow_bigint_div_rem_locals: HashSet<LocalId>,
    control_flow_mixed_bigint_locals: HashSet<LocalId>,
    array_locals: HashSet<LocalId>,
    static_array_slots: HashMap<LocalId, Vec<ResolvedArrayElement>>,
    symbol_iterator_object_locals: HashSet<LocalId>,
    static_object_literal_locals: HashMap<LocalId, Vec<(String, ResolvedExpr)>>,
    static_object_literal_alias_sources: HashMap<LocalId, HashSet<LocalId>>,
    static_function_array_like_locals: HashMap<LocalId, StaticFunctionArrayLike>,
    string_literal_locals: HashMap<LocalId, String>,
    native_set_add_locals: HashSet<LocalId>,
    generator_function_names: HashSet<String>,
    current_class: Option<String>,
    in_constructor: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArrowClosure {
    func_id: FuncId,
    captures: Vec<LocalId>,
}

#[derive(Debug, Clone)]
struct StaticFunctionArrayLike {
    elements: Vec<Option<ResolvedExpr>>,
}

impl ArrowClosure {
    fn to_expr(&self, representation: ClosureRepresentation) -> LoweredExpr {
        LoweredExpr::ArrowFn {
            func_id: self.func_id,
            captures: self.captures.clone(),
            representation,
        }
    }
}

impl<'a> Resolver<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        function_ids: &'a HashMap<String, FuncId>,
        function_signatures: &'a HashMap<FuncId, FunctionSignature>,
        function_captures: &'a HashMap<FuncId, Vec<String>>,
        function_mutable_captures: &'a HashMap<FuncId, Vec<String>>,
        class_method_captures: &'a HashMap<FuncId, Vec<String>>,
        class_method_mutable_captures: &'a HashMap<FuncId, Vec<String>>,
        env_cell_names: &HashSet<String>,
        heap_closure_names: &HashSet<String>,
        class_parents: HashMap<String, Option<String>>,
        class_private_fields: ClassPrivateFieldSlots,
        class_static_private_fields: ClassStaticPrivateFields,
        generator_function_names: HashSet<String>,
        next_func_id: usize,
    ) -> Self {
        let (class_constructor_ids, class_method_ids, class_static_method_ids) =
            class_maps(function_ids);
        Self {
            function_ids,
            function_signatures,
            function_captures,
            function_mutable_captures,
            class_method_captures,
            class_method_mutable_captures,
            env_cell_names: env_cell_names.clone(),
            env_cell_locals: HashSet::new(),
            heap_closure_names: heap_closure_names.clone(),
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            locals: Vec::new(),
            next_func_id,
            generated_functions: Vec::new(),
            arrow_locals: HashMap::new(),
            heap_closure_locals: HashSet::new(),
            nullish_locals: HashSet::new(),
            module_ids: HashMap::new(),
            modules: Vec::new(),
            class_constructor_ids,
            class_method_ids,
            class_static_method_ids,
            class_parents,
            class_private_fields,
            class_static_private_fields,
            local_classes: HashMap::new(),
            object_function_props: HashMap::new(),
            regexp_literal_locals: HashSet::new(),
            bigint_locals: HashSet::new(),
            control_flow_bigint_div_rem_locals: HashSet::new(),
            control_flow_mixed_bigint_locals: HashSet::new(),
            array_locals: HashSet::new(),
            static_array_slots: HashMap::new(),
            symbol_iterator_object_locals: HashSet::new(),
            static_object_literal_locals: HashMap::new(),
            static_object_literal_alias_sources: HashMap::new(),
            static_function_array_like_locals: HashMap::new(),
            string_literal_locals: HashMap::new(),
            native_set_add_locals: HashSet::new(),
            generator_function_names,
            current_class: None,
            in_constructor: false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn with_params(
        function_ids: &'a HashMap<String, FuncId>,
        function_signatures: &'a HashMap<FuncId, FunctionSignature>,
        function_captures: &'a HashMap<FuncId, Vec<String>>,
        function_mutable_captures: &'a HashMap<FuncId, Vec<String>>,
        class_method_captures: &'a HashMap<FuncId, Vec<String>>,
        class_method_mutable_captures: &'a HashMap<FuncId, Vec<String>>,
        env_cell_names: &HashSet<String>,
        heap_closure_names: &HashSet<String>,
        params: &[String],
        class_parents: HashMap<String, Option<String>>,
        class_private_fields: ClassPrivateFieldSlots,
        class_static_private_fields: ClassStaticPrivateFields,
        current_class: Option<&str>,
        in_constructor: bool,
        next_func_id: usize,
    ) -> Result<(Self, Vec<LocalId>), Diagnostic> {
        let (class_constructor_ids, class_method_ids, class_static_method_ids) =
            class_maps(function_ids);
        let mut resolver = Self {
            function_ids,
            function_signatures,
            function_captures,
            function_mutable_captures,
            class_method_captures,
            class_method_mutable_captures,
            env_cell_names: env_cell_names.clone(),
            env_cell_locals: HashSet::new(),
            heap_closure_names: heap_closure_names.clone(),
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            locals: Vec::new(),
            next_func_id,
            generated_functions: Vec::new(),
            arrow_locals: HashMap::new(),
            heap_closure_locals: HashSet::new(),
            nullish_locals: HashSet::new(),
            module_ids: HashMap::new(),
            modules: Vec::new(),
            class_constructor_ids,
            class_method_ids,
            class_static_method_ids,
            class_parents,
            class_private_fields,
            class_static_private_fields,
            local_classes: HashMap::new(),
            object_function_props: HashMap::new(),
            regexp_literal_locals: HashSet::new(),
            bigint_locals: HashSet::new(),
            control_flow_bigint_div_rem_locals: HashSet::new(),
            control_flow_mixed_bigint_locals: HashSet::new(),
            array_locals: HashSet::new(),
            static_array_slots: HashMap::new(),
            symbol_iterator_object_locals: HashSet::new(),
            static_object_literal_locals: HashMap::new(),
            static_object_literal_alias_sources: HashMap::new(),
            static_function_array_like_locals: HashMap::new(),
            string_literal_locals: HashMap::new(),
            native_set_add_locals: HashSet::new(),
            generator_function_names: HashSet::new(),
            current_class: current_class.map(ToOwned::to_owned),
            in_constructor,
        };
        let mut param_ids = Vec::new();
        let mut seen_params = HashMap::new();

        for param in params {
            if seen_params.contains_key(param) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateParameter,
                    message: format!("duplicate parameter name: `{param}`"),
                    span: None,
                });
            }
            seen_params.insert(param.clone(), ());
            let local_id = LocalId(resolver.next_local_id);
            resolver.next_local_id += 1;
            resolver
                .scopes
                .last_mut()
                .expect("function scope must exist")
                .insert(param.clone(), local_id);
            if resolver.env_cell_names.contains(param) {
                resolver.env_cell_locals.insert(local_id);
            }
            if resolver.heap_closure_names.contains(param) {
                resolver.heap_closure_locals.insert(local_id);
            }
            if let Some(current_class) = current_class
                && param == "this"
            {
                resolver
                    .local_classes
                    .insert(local_id, current_class.to_owned());
            }
            param_ids.push(local_id);
        }

        Ok((resolver, param_ids))
    }

    fn lower_block(&mut self, statements: &[ResolvedStmt]) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let mut lowered = Vec::with_capacity(statements.len());
        for statement in statements {
            lowered.push(self.lower_stmt(statement)?);
        }
        Ok(lowered)
    }

    fn lower_nested_block(
        &mut self,
        statements: &[ResolvedStmt],
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        self.scopes.push(HashMap::new());
        let lowered = self.lower_block(statements);
        self.scopes.pop();
        lowered
    }

    fn lower_direct_iife_stmt(
        &mut self,
        expr: &ResolvedExpr,
    ) -> Result<Option<LoweredStmt>, Diagnostic> {
        let ResolvedExpr::Call { callee, args, span } = expr else {
            return Ok(None);
        };
        let ResolvedExpr::FunctionExpr { params, body, .. } = callee.as_ref() else {
            return Ok(None);
        };
        if !args.is_empty() || !params.is_empty() {
            return Ok(None);
        }
        if !direct_iife_body_has_static_eval_block_function_binding(body) {
            return Ok(None);
        }
        if direct_iife_body_has_unsupported_return(body) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-302: direct eval IIFE lowering does not support function returns"
                        .to_owned(),
                span: Some(*span),
            });
        }
        if block_contains_this(body) || block_contains_arguments(body) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-302: direct eval IIFE lowering does not support `this` or `arguments`"
                        .to_owned(),
                span: Some(*span),
            });
        }

        self.scopes.push(HashMap::new());
        let lowered = self.lower_block(body);
        self.scopes.pop();
        lowered.map(|statements| Some(LoweredStmt::Block(statements)))
    }

    fn lower_stmt(&mut self, stmt: &ResolvedStmt) -> Result<LoweredStmt, Diagnostic> {
        match stmt {
            ResolvedStmt::DestructureLet { pattern, expr } => {
                let value_local = self.alloc_temp();
                let mut statements = vec![LoweredStmt::Let(value_local, self.lower_expr(expr)?)];
                statements.extend(
                    self.lower_binding_pattern_declarations(
                        pattern,
                        LoweredExpr::Local(value_local),
                        Some(expr),
                    )?,
                );
                Ok(LoweredStmt::Block(statements))
            }
            ResolvedStmt::Let(name, expr) => {
                let local_id = self.declare_local(name)?;
                let function_props = self.function_props_for_object_expr(expr);
                let lowered = if let ResolvedExpr::ArrowFn { params, body, .. } = expr {
                    self.lower_arrow_fn_with_self(params, body, Some(name))?
                } else {
                    self.lower_expr(expr)?
                };
                let lowered = if self.env_cell_names.contains(name) {
                    self.env_cell_locals.insert(local_id);
                    LoweredExpr::EnvCellNew(Box::new(lowered))
                } else {
                    lowered
                };
                if let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = &lowered
                {
                    self.arrow_locals.insert(
                        local_id,
                        ArrowClosure {
                            func_id: *func_id,
                            captures: captures.clone(),
                        },
                    );
                } else {
                    self.arrow_locals.remove(&local_id);
                }
                self.update_heap_closure_local(local_id, expr, &lowered);
                if self.heap_closure_names.contains(name) {
                    self.heap_closure_locals.insert(local_id);
                }
                self.update_nullish_local(local_id, expr);
                self.update_bigint_local(local_id, expr);
                self.update_control_flow_bigint_assignment(local_id);
                self.update_array_local(local_id, expr);
                self.update_symbol_iterator_object_local(local_id, expr);
                self.update_static_object_literal_local_on_let(local_id, expr);
                self.update_static_function_array_like_local_on_let(local_id, expr);
                self.update_string_literal_local(local_id, expr);
                self.update_native_set_add_local(local_id, expr);
                if let Some(props) = function_props {
                    self.object_function_props.insert(local_id, props);
                } else {
                    self.object_function_props.remove(&local_id);
                }
                let expr_class = self.infer_class_for_expr(expr);
                if let Some(class_name) = expr_class {
                    self.local_classes.insert(local_id, class_name);
                } else {
                    self.local_classes.remove(&local_id);
                }
                self.update_regexp_literal_local(local_id, expr);
                Ok(LoweredStmt::Let(local_id, lowered))
            }
            ResolvedStmt::Assign(name, expr) => {
                let local_id = self.resolve_local(name)?;
                self.invalidate_static_object_literal_local(local_id);
                let function_props = self.function_props_for_object_expr(expr);
                let lowered = self.lower_expr(expr)?;
                if let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = &lowered
                {
                    self.arrow_locals.insert(
                        local_id,
                        ArrowClosure {
                            func_id: *func_id,
                            captures: captures.clone(),
                        },
                    );
                } else {
                    self.arrow_locals.remove(&local_id);
                }
                self.update_heap_closure_local(local_id, expr, &lowered);
                self.update_nullish_local(local_id, expr);
                self.update_bigint_local(local_id, expr);
                self.update_control_flow_bigint_assignment(local_id);
                self.update_array_local(local_id, expr);
                self.update_symbol_iterator_object_local(local_id, expr);
                self.invalidate_static_function_array_like_local(local_id);
                self.update_string_literal_local(local_id, expr);
                self.update_native_set_add_local(local_id, expr);
                if let Some(props) = function_props {
                    self.object_function_props.insert(local_id, props);
                } else {
                    self.object_function_props.remove(&local_id);
                }
                let expr_class = self.infer_class_for_expr(expr);
                if let Some(class_name) = expr_class {
                    self.local_classes.insert(local_id, class_name);
                } else {
                    self.local_classes.remove(&local_id);
                }
                self.update_regexp_literal_local(local_id, expr);
                if self.env_cell_locals.contains(&local_id) {
                    Ok(LoweredStmt::Expr(LoweredExpr::EnvCellSet {
                        cell: local_id,
                        expr: Box::new(lowered),
                    }))
                } else {
                    Ok(LoweredStmt::Assign(local_id, lowered))
                }
            }
            ResolvedStmt::Expr(expr) => {
                if let Some(lowered) = self.lower_direct_iife_stmt(expr)? {
                    return Ok(lowered);
                }
                self.update_static_array_slot_assignment(expr);
                if let ResolvedExpr::MethodCall {
                    object,
                    method,
                    args,
                    ..
                } = expr
                    && method == "push"
                    && args.len() == 1
                    && let ResolvedExpr::Ident(name) = object.as_ref()
                    && let Ok(local_id) = self.resolve_local(name)
                    && self.array_locals.contains(&local_id)
                {
                    return Ok(LoweredStmt::Assign(
                        local_id,
                        LoweredExpr::RuntimeCall {
                            runtime_fn: "ArrayPushGrow".to_owned(),
                            args: vec![LoweredExpr::Local(local_id), self.lower_expr(&args[0])?],
                        },
                    ));
                }
                Ok(LoweredStmt::Expr(self.lower_expr(expr)?))
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let condition = self.lower_expr(condition)?;
                let incoming_bigint_locals = self.bigint_locals.clone();
                let incoming_div_rem_locals = self.control_flow_bigint_div_rem_locals.clone();
                let incoming_mixed_locals = self.control_flow_mixed_bigint_locals.clone();

                let then_body = self.lower_nested_block(then_body)?;
                let then_add_sub_bigint_locals = self.bigint_locals.clone();
                let then_div_rem_bigint_locals = self.bigint_div_rem_candidate_locals();
                let then_mixed_locals = self.control_flow_mixed_bigint_locals.clone();

                self.bigint_locals = incoming_bigint_locals.clone();
                self.control_flow_bigint_div_rem_locals = incoming_div_rem_locals.clone();
                self.control_flow_mixed_bigint_locals = incoming_mixed_locals.clone();

                let else_body = self.lower_nested_block(else_body)?;
                let else_add_sub_bigint_locals = self.bigint_locals.clone();
                let else_div_rem_bigint_locals = self.bigint_div_rem_candidate_locals();
                let else_mixed_locals = self.control_flow_mixed_bigint_locals.clone();

                self.bigint_locals = then_add_sub_bigint_locals
                    .intersection(&else_add_sub_bigint_locals)
                    .copied()
                    .collect();
                let definite_div_rem_locals = then_div_rem_bigint_locals
                    .intersection(&else_div_rem_bigint_locals)
                    .copied()
                    .collect::<HashSet<_>>();
                let branch_mixed_locals = then_div_rem_bigint_locals
                    .symmetric_difference(&else_div_rem_bigint_locals)
                    .copied()
                    .chain(then_mixed_locals.union(&else_mixed_locals).copied())
                    .filter(|local| !definite_div_rem_locals.contains(local))
                    .collect::<HashSet<_>>();

                self.control_flow_bigint_div_rem_locals = definite_div_rem_locals
                    .difference(&self.bigint_locals)
                    .copied()
                    .collect();
                self.control_flow_mixed_bigint_locals = branch_mixed_locals;
                Ok(LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,
                })
            }
            ResolvedStmt::While { condition, body } => Ok(LoweredStmt::While {
                condition: self.lower_expr(condition)?,
                body: self.lower_nested_block(body)?,
            }),
            ResolvedStmt::Return(expr) => {
                if let ResolvedExpr::Ident(name) = expr
                    && let Some(closure) = self
                        .resolve_local(name)
                        .ok()
                        .and_then(|local| self.arrow_locals.get(&local))
                {
                    return Ok(LoweredStmt::Return(
                        closure.to_expr(ClosureRepresentation::HeapObject),
                    ));
                }
                Ok(LoweredStmt::Return(self.lower_expr(expr)?))
            }
            ResolvedStmt::Function {
                name, params, body, ..
            } => {
                let local_id = self.declare_local(name)?;
                if self.env_cell_names.contains(name) {
                    self.env_cell_locals.insert(local_id);
                }
                let closure = self.lower_nested_function(name, params, body)?;
                if let LoweredExpr::ArrowFn {
                    func_id,
                    captures,
                    representation,
                } = &closure
                {
                    if matches!(representation, ClosureRepresentation::HeapObject) {
                        self.heap_closure_locals.insert(local_id);
                    } else {
                        self.arrow_locals.insert(
                            local_id,
                            ArrowClosure {
                                func_id: *func_id,
                                captures: captures.clone(),
                            },
                        );
                    }
                }
                self.nullish_locals.remove(&local_id);
                if self.env_cell_locals.contains(&local_id) {
                    Ok(LoweredStmt::Block(vec![
                        LoweredStmt::Let(
                            local_id,
                            LoweredExpr::EnvCellNew(Box::new(LoweredExpr::Undefined)),
                        ),
                        LoweredStmt::Expr(LoweredExpr::EnvCellSet {
                            cell: local_id,
                            expr: Box::new(closure),
                        }),
                    ]))
                } else {
                    Ok(LoweredStmt::Let(local_id, closure))
                }
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_param,
                catch_block,
                finally_block,
            } => {
                let catch_var = if let Some(param) = catch_param {
                    Some(self.declare_local(param)?)
                } else {
                    None
                };
                Ok(LoweredStmt::TryCatch {
                    try_body: self.lower_nested_block(try_block)?,
                    catch_var,
                    catch_body: catch_block
                        .as_ref()
                        .map(|b| self.lower_nested_block(b))
                        .transpose()?,
                    finally_body: finally_block
                        .as_ref()
                        .map(|b| self.lower_nested_block(b))
                        .transpose()?,
                })
            }
            ResolvedStmt::Throw(expr) => Ok(LoweredStmt::Throw(self.lower_expr(expr)?)),
            ResolvedStmt::Switch { expr, cases } => {
                let resolved_cases = cases
                    .iter()
                    .map(|(cond, body)| {
                        Ok((
                            cond.as_ref().map(|e| self.lower_expr(e)).transpose()?,
                            self.lower_nested_block(body)?,
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(LoweredStmt::Switch {
                    expr: self.lower_expr(expr)?,
                    cases: resolved_cases,
                })
            }
            ResolvedStmt::DoWhile { body, condition } => Ok(LoweredStmt::DoWhile {
                body: self.lower_nested_block(body)?,
                condition: self.lower_expr(condition)?,
            }),
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                self.scopes.push(HashMap::new());
                let resolved = (|| {
                    let resolved_init = if let Some(i) = init {
                        Some(Box::new(self.lower_stmt(i)?))
                    } else {
                        None
                    };
                    Ok(LoweredStmt::For {
                        init: resolved_init,
                        condition: condition.as_ref().map(|c| self.lower_expr(c)).transpose()?,
                        update: update.as_ref().map(|u| self.lower_expr(u)).transpose()?,
                        body: self.lower_nested_block(body)?,
                    })
                })();
                self.scopes.pop();
                resolved
            }
            ResolvedStmt::ForIn { var, iter, body } => {
                let var_id = self.declare_local(var)?;
                Ok(LoweredStmt::ForIn {
                    var: var_id,
                    iter: self.lower_expr(iter)?,
                    iter_local: self.alloc_temp(),
                    index_local: self.alloc_temp(),
                    len_local: self.alloc_temp(),
                    body: self.lower_nested_block(body)?,
                })
            }
            ResolvedStmt::ForOf { var, iter, body } => {
                let var_id = self.declare_local(var)?;
                let lowered_iter = if let ResolvedExpr::Ident(name) = iter
                    && let Ok(local_id) = self.resolve_local(name)
                {
                    let class_name = self.local_classes.get(&local_id);
                    if class_name.is_some_and(|c| c == "Set") {
                        LoweredExpr::RuntimeCall {
                            runtime_fn: "SetValuesArray".to_owned(),
                            args: vec![LoweredExpr::Local(local_id)],
                        }
                    } else if class_name.is_some_and(|c| c == "Map") {
                        LoweredExpr::RuntimeCall {
                            runtime_fn: "MapValuesArray".to_owned(),
                            args: vec![LoweredExpr::Local(local_id)],
                        }
                    } else {
                        self.lower_expr(iter)?
                    }
                } else {
                    self.lower_expr(iter)?
                };
                Ok(LoweredStmt::ForOf {
                    var: var_id,
                    iter: lowered_iter,
                    iter_local: self.alloc_temp(),
                    index_local: self.alloc_temp(),
                    len_local: self.alloc_temp(),
                    body: self.lower_nested_block(body)?,
                })
            }
            ResolvedStmt::Labeled { label, body } => Ok(LoweredStmt::Labeled {
                label: label.clone(),
                body: Box::new(self.lower_stmt(body)?),
            }),
            ResolvedStmt::Break { label } => Ok(LoweredStmt::Break {
                label: label.clone(),
            }),
            ResolvedStmt::Continue { label } => Ok(LoweredStmt::Continue {
                label: label.clone(),
            }),
            ResolvedStmt::Export { name, expr } => Ok(LoweredStmt::Export {
                name: name.clone(),
                expr: self.lower_expr(expr)?,
            }),
            ResolvedStmt::ModuleExportsAssign { expr } => Ok(LoweredStmt::ModuleExportsAssign {
                expr: self.lower_expr(expr)?,
            }),
            ResolvedStmt::ClassDecl { .. } => Ok(LoweredStmt::Expr(LoweredExpr::Undefined)),
            ResolvedStmt::Block { statements, .. } => {
                Ok(LoweredStmt::Block(self.lower_block(statements)?))
            }
        }
    }

    pub(super) fn lower_class_static_private_field(
        &mut self,
        class_name: &str,
        field: &str,
        initializer: &ResolvedExpr,
    ) -> Result<LoweredStmt, Diagnostic> {
        let local_name = crate::builtin_resolver::static_private_field_local_name(class_name, field);
        self.with_current_class(class_name, |resolver| {
            resolver.lower_stmt(&ResolvedStmt::Let(local_name, initializer.clone()))
        })
    }

    pub(super) fn lower_class_static_block(
        &mut self,
        class_name: &str,
        block: &[ResolvedStmt],
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        self.with_current_class(class_name, |resolver| resolver.lower_nested_block(block))
    }

    fn with_current_class<T>(
        &mut self,
        class_name: &str,
        f: impl FnOnce(&mut Self) -> Result<T, Diagnostic>,
    ) -> Result<T, Diagnostic> {
        let previous = self.current_class.replace(class_name.to_owned());
        let result = f(self);
        self.current_class = previous;
        result
    }

}

fn class_maps(
    function_ids: &HashMap<String, FuncId>,
) -> (ClassConstructorMap, ClassMethodMap, ClassMethodMap) {
    let mut ctor_ids = HashMap::new();
    let mut method_ids = HashMap::new();
    let mut static_method_ids = HashMap::new();

    for (name, id) in function_ids {
        if let Some(rest) = name.strip_prefix("class::") {
            let mut parts = rest.splitn(2, "::");
            let class = parts.next().unwrap_or_default();
            let member = parts.next().unwrap_or_default();
            if member == "constructor" {
                ctor_ids.insert(class.to_owned(), *id);
            } else if let Some(static_name) = member.strip_prefix("static::") {
                static_method_ids.insert((class.to_owned(), static_name.to_owned()), *id);
            } else if !class.is_empty() && !member.is_empty() {
                method_ids.insert((class.to_owned(), member.to_owned()), *id);
            }
        }
    }

    (ctor_ids, method_ids, static_method_ids)
}

fn lowered_binding_default(default: &BindingDefault) -> LoweredExpr {
    match default {
        BindingDefault::Number(value) => LoweredExpr::Number(*value),
        BindingDefault::String(value) => LoweredExpr::String(value.clone()),
        BindingDefault::Bool(value) => LoweredExpr::Bool(*value),
        BindingDefault::Null => LoweredExpr::Null,
        BindingDefault::Undefined => LoweredExpr::Undefined,
    }
}

fn binding_param_names<'a>(
    params: impl Iterator<Item = (&'a str, Option<Span>)>,
) -> Result<Vec<String>, Diagnostic> {
    let mut names = Vec::new();
    for (param, span) in params {
        if let Some(inner) = param.strip_prefix("...") {
            // Rest param: extract inner names from binding pattern
            if let Some(pattern) = parse_binding_pattern(inner, span)? {
                names.extend(pattern.names().into_iter().map(ToOwned::to_owned));
            } else {
                names.push(param.to_owned());
            }
        } else if let Some(pattern) = parse_binding_pattern(param, span)? {
            names.extend(pattern.names().into_iter().map(ToOwned::to_owned));
        } else {
            names.push(param.to_owned());
        }
    }
    Ok(names)
}

const PRIVATE_FIELD_STORAGE_PREFIX: &str = "__ts2wasm_private::";

fn is_private_field_storage_key(key: &str) -> bool {
    key.starts_with(PRIVATE_FIELD_STORAGE_PREFIX)
}

fn private_storage_observable_access_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-255: private field backing storage is not accessible through ordinary property access in this private field runtime slice".to_owned(),
        span,
    }
}

fn is_static_copy_safe_object_prop_value(expr: &ResolvedExpr) -> bool {
    matches!(
        expr,
        ResolvedExpr::Number(_)
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined
    )
}

fn is_set_prototype_property(object: &ResolvedExpr, key: &str, expected_key: &str) -> bool {
    key == expected_key && matches_set_prototype_object(object)
}

fn is_set_prototype_property_expr(expr: &ResolvedExpr, expected_key: &str) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    is_set_prototype_property(object, key, expected_key)
}

fn is_array_prototype_push_property(object: &ResolvedExpr, key: &str) -> bool {
    key == "push" && matches_array_prototype_object(object)
}

fn is_array_prototype_push_expr(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    is_array_prototype_push_property(object, key)
}

fn matches_array_prototype_object(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Array"
        )
}

fn matches_set_prototype_object(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Set"
        )
}

fn unsupported_array_map_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-270: Array.prototype.map requires callback dispatch and new array allocation semantics that are not supported in this runtime slice".to_owned(),
        span,
    }
}

fn unsupported_array_sort_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-299: Array.prototype.sort is currently supported only for dense numeric arrays with comparator `(a, b) => a - b`".to_owned(),
        span,
    }
}

fn is_array_prototype_map_call_receiver(object: &ResolvedExpr, method: &str) -> bool {
    method == "call" && matches_array_prototype_map_property(object)
}

fn matches_array_prototype_map_property(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "map" && matches_array_prototype_property(object)
}

fn matches_array_prototype_property(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Array"
        )
}

fn is_string_split_result_expr(expr: &ResolvedExpr) -> bool {
    matches!(
        expr,
        ResolvedExpr::MethodCall { method, .. } if method == "split"
    )
}

fn is_identity_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [param] = params.as_slice() else {
        return false;
    };
    matches!(body.as_ref(), ResolvedExpr::Ident(name) if name == param)
}

fn is_number_double_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [param] = params.as_slice() else {
        return false;
    };
    let ResolvedExpr::Binary {
        left,
        op: BinaryOp::Multiply,
        right,
    } = body.as_ref()
    else {
        return false;
    };
    matches!(
        (left.as_ref(), right.as_ref()),
        (ResolvedExpr::Ident(name), ResolvedExpr::Number(2))
            if name == param
    ) || matches!(
        (left.as_ref(), right.as_ref()),
        (ResolvedExpr::Number(2), ResolvedExpr::Ident(name))
            if name == param
    )
}

fn string_split_arrow_separator(args: &[ResolvedExpr]) -> Option<&ResolvedExpr> {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return None;
    };
    let [param] = params.as_slice() else {
        return None;
    };
    let ResolvedExpr::MethodCall {
        object,
        method,
        args,
        ..
    } = body.as_ref()
    else {
        return None;
    };
    if method != "split" {
        return None;
    }
    let ResolvedExpr::Ident(name) = object.as_ref() else {
        return None;
    };
    if name != param {
        return None;
    }
    let [separator] = args.as_slice() else {
        return None;
    };
    Some(separator)
}

fn string_constructor_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [param] = params.as_slice() else {
        return false;
    };
    let ResolvedExpr::Call { callee, args, .. } = body.as_ref() else {
        return false;
    };
    if !matches!(callee.as_ref(), ResolvedExpr::Ident(name) if name == "String") {
        return false;
    }
    let [ResolvedExpr::Ident(arg_name)] = args.as_slice() else {
        return false;
    };
    arg_name == param
}

fn unary_plus_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [param] = params.as_slice() else {
        return false;
    };
    let ResolvedExpr::Unary { op, expr } = body.as_ref() else {
        return false;
    };
    *op == UnaryOp::Plus && matches!(expr.as_ref(), ResolvedExpr::Ident(name) if name == param)
}

fn numeric_ascending_sort_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [left_param, right_param] = params.as_slice() else {
        return false;
    };
    let ResolvedExpr::Binary {
        left,
        op: BinaryOp::Subtract,
        right,
    } = body.as_ref()
    else {
        return false;
    };
    matches!(left.as_ref(), ResolvedExpr::Ident(name) if name == left_param)
        && matches!(right.as_ref(), ResolvedExpr::Ident(name) if name == right_param)
}
