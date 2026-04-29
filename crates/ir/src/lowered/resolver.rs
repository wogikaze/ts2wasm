struct Resolver<'a> {
    function_ids: &'a HashMap<String, FuncId>,
    function_signatures: &'a HashMap<FuncId, FunctionSignature>,
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
    local_classes: HashMap<LocalId, String>,
    object_function_props: HashMap<LocalId, HashMap<String, FuncId>>,
    regexp_literal_locals: HashSet<LocalId>,
    bigint_locals: HashSet<LocalId>,
    current_class: Option<String>,
    in_constructor: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArrowClosure {
    func_id: FuncId,
    captures: Vec<LocalId>,
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
    fn new(
        function_ids: &'a HashMap<String, FuncId>,
        function_signatures: &'a HashMap<FuncId, FunctionSignature>,
        class_parents: HashMap<String, Option<String>>,
        class_private_fields: ClassPrivateFieldSlots,
        next_func_id: usize,
    ) -> Self {
        let (class_constructor_ids, class_method_ids, class_static_method_ids) =
            class_maps(function_ids);
        Self {
            function_ids,
            function_signatures,
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
            local_classes: HashMap::new(),
            object_function_props: HashMap::new(),
            regexp_literal_locals: HashSet::new(),
            bigint_locals: HashSet::new(),
            current_class: None,
            in_constructor: false,
        }
    }

    fn with_params(
        function_ids: &'a HashMap<String, FuncId>,
        function_signatures: &'a HashMap<FuncId, FunctionSignature>,
        params: &[String],
        class_parents: HashMap<String, Option<String>>,
        class_private_fields: ClassPrivateFieldSlots,
        current_class: Option<&str>,
        in_constructor: bool,
        next_func_id: usize,
    ) -> Result<(Self, Vec<LocalId>), Diagnostic> {
        let (class_constructor_ids, class_method_ids, class_static_method_ids) =
            class_maps(function_ids);
        let mut resolver = Self {
            function_ids,
            function_signatures,
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
            local_classes: HashMap::new(),
            object_function_props: HashMap::new(),
            regexp_literal_locals: HashSet::new(),
            bigint_locals: HashSet::new(),
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
                Ok(LoweredStmt::Assign(local_id, lowered))
            }
            ResolvedStmt::Expr(expr) => Ok(LoweredStmt::Expr(self.lower_expr(expr)?)),
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => Ok(LoweredStmt::If {
                condition: self.lower_expr(condition)?,
                then_body: self.lower_nested_block(then_body)?,
                else_body: self.lower_nested_block(else_body)?,
            }),
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
            ResolvedStmt::Function { name, params, body } => {
                let local_id = self.declare_local(name)?;
                let closure = self.lower_nested_function(name, params, body)?;
                if let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = &closure
                {
                    self.arrow_locals.insert(
                        local_id,
                        ArrowClosure {
                            func_id: *func_id,
                            captures: captures.clone(),
                        },
                    );
                }
                self.nullish_locals.remove(&local_id);
                Ok(LoweredStmt::Let(local_id, closure))
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
                Ok(LoweredStmt::ForOf {
                    var: var_id,
                    iter: self.lower_expr(iter)?,
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
        }
    }

    fn lower_expr(&mut self, expr: &ResolvedExpr) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            ResolvedExpr::Number(value) => Ok(LoweredExpr::Number(*value)),
            ResolvedExpr::BigIntLiteral {
                decimal,
                sign,
                limb_low,
                limb_high,
            } => Ok(LoweredExpr::BigIntLiteral {
                decimal: decimal.clone(),
                sign: *sign,
                limb_low: *limb_low,
                limb_high: *limb_high,
            }),
            ResolvedExpr::String(value) => Ok(LoweredExpr::String(value.clone())),
            ResolvedExpr::Bool(value) => Ok(LoweredExpr::Bool(*value)),
            ResolvedExpr::Null => Ok(LoweredExpr::Null),
            ResolvedExpr::Undefined => Ok(LoweredExpr::Undefined),
            ResolvedExpr::This { span } => match self.resolve_local("this") {
                Ok(local) => Ok(LoweredExpr::Local(local)),
                Err(_) => Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-062d: `this` is only supported inside receiver-bound functions, class constructors, and instance methods in this milestone".to_owned(),
                    span: Some(*span),
                }),
            },
            ResolvedExpr::Ident(name) => match self.resolve_local(name) {
                Ok(local) => Ok(LoweredExpr::Local(local)),
                Err(_) if name == "arguments" => Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-062d: `arguments` is only supported inside non-arrow functions in this milestone".to_owned(),
                    span: None,
                }),
                Err(err) => Err(err),
            },
            ResolvedExpr::Spread(_) => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "spread expressions are only supported in call arguments".to_owned(),
                span: None,
            }),
            ResolvedExpr::Unary { op, expr } => {
                if *op == UnaryOp::Negate && self.resolved_expr_is_bigint(expr) {
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "BigIntUnaryMinus".to_owned(),
                        args: vec![self.lower_expr(expr)?],
                    });
                }
                if *op == UnaryOp::Delete {
                    // Lower delete to PropertyDelete or PropertyDeleteDynamic
                    match expr.as_ref() {
                        ResolvedExpr::PropertyAccess { object, key, .. } => {
                            Ok(LoweredExpr::PropertyDelete {
                                object: Box::new(self.lower_expr(object)?),
                                key: key.clone(),
                            })
                        }
                        ResolvedExpr::ComputedIndex { object, index } => {
                            Ok(LoweredExpr::PropertyDeleteDynamic {
                                object: Box::new(self.lower_expr(object)?),
                                key: Box::new(self.lower_expr(index)?),
                            })
                        }
                        _ => Ok(LoweredExpr::Unary {
                            op: lower_unary_op(*op)?,
                            expr: Box::new(self.lower_expr(expr)?),
                        }),
                    }
                } else {
                    Ok(LoweredExpr::Unary {
                        op: lower_unary_op(*op)?,
                        expr: Box::new(self.lower_expr(expr)?),
                    })
                }
            }
            ResolvedExpr::Binary { left, op, right } => {
                if *op == BinaryOp::InstanceOf {
                    let prototype = match right.as_ref() {
                        ResolvedExpr::Ident(name) => {
                            if let Some(constructor) = BuiltinErrorConstructor::from_name(name) {
                                LoweredExpr::BuiltinErrorPrototype(constructor)
                            } else {
                                self.class_prototype_ref(name)
                                    .map(LoweredExpr::ClassPrototype)?
                            }
                        }
                        _ => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "issue-207: instanceof right-hand side must be a supported class constructor".to_owned(),
                                span: None,
                            });
                        }
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "$instanceof".to_string(),
                        args: vec![self.lower_expr(left)?, prototype],
                    })
                } else if *op == BinaryOp::In {
                    // Lower in to PropertyIn or PropertyInDynamic
                    // key in object -> check if key exists in object
                    match left.as_ref() {
                        ResolvedExpr::String(key) => Ok(LoweredExpr::PropertyIn {
                            obj: Box::new(self.lower_expr(right)?),
                            key: key.clone(),
                        }),
                        _ => Ok(LoweredExpr::PropertyInDynamic {
                            obj: Box::new(self.lower_expr(right)?),
                            key: Box::new(self.lower_expr(left)?),
                        }),
                    }
                } else if matches!(
                    op,
                    BinaryOp::Add
                        | BinaryOp::Subtract
                        | BinaryOp::Multiply
                        | BinaryOp::Divide
                        | BinaryOp::Modulo
                )
                    && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
                {
                    let runtime_fn = match op {
                        BinaryOp::Add => "BigIntAdd",
                        BinaryOp::Subtract => "BigIntSub",
                        BinaryOp::Multiply => "BigIntMul",
                        BinaryOp::Divide => "BigIntDiv",
                        BinaryOp::Modulo => "BigIntRem",
                        _ => unreachable!("checked above"),
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: runtime_fn.to_owned(),
                        args: vec![self.lower_expr(left)?, self.lower_expr(right)?],
                    })
                } else {
                    Ok(LoweredExpr::Binary {
                        left: Box::new(self.lower_expr(left)?),
                        op: lower_binary_op(*op)?,
                        right: Box::new(self.lower_expr(right)?),
                    })
                }
            }
            ResolvedExpr::Assign { name, expr } => {
                let local = self.resolve_local(name)?;
                Ok(LoweredExpr::Assign {
                    local,
                    expr: Box::new(self.lower_expr(expr)?),
                })
            }
            ResolvedExpr::LogicalAssign { name, op, expr } => {
                let local = self.resolve_local(name)?;
                Ok(LoweredExpr::LogicalAssign {
                    local,
                    op: lower_logical_assign_op(*op),
                    expr: Box::new(self.lower_expr(expr)?),
                })
            }
            ResolvedExpr::LogicalPropertyAssign {
                object,
                key,
                op,
                expr,
            } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                let object = self.resolve_local(object)?;
                Ok(LoweredExpr::LogicalPropertyAssign {
                    object,
                    key: key.clone(),
                    op: lower_logical_assign_op(*op),
                    expr: Box::new(self.lower_expr(expr)?),
                })
            }
            ResolvedExpr::LogicalComputedPropertyAssign {
                object,
                key,
                op,
                expr,
            } => {
                let object = self.resolve_local(object)?;
                if self.local_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::LogicalComputedPropertyAssign {
                    object,
                    key: Box::new(self.lower_expr(key)?),
                    op: lower_logical_assign_op(*op),
                    expr: Box::new(self.lower_expr(expr)?),
                })
            }
            ResolvedExpr::LogicalComputedMemberAssign {
                object,
                key,
                op,
                expr,
            } => Ok(LoweredExpr::LogicalComputedMemberAssign {
                object: {
                    if self.expr_has_private_progress_storage(object) {
                        return Err(private_storage_observable_access_diagnostic(None));
                    }
                    Box::new(self.lower_expr(object)?)
                },
                key: Box::new(self.lower_expr(key)?),
                op: lower_logical_assign_op(*op),
                expr: Box::new(self.lower_expr(expr)?),
            }),
            ResolvedExpr::LogicalMemberAssign {
                object,
                key,
                op,
                expr,
            } => Ok(LoweredExpr::LogicalMemberAssign {
                object: Box::new(self.lower_expr(object)?),
                key: key.clone(),
                op: lower_logical_assign_op(*op),
                expr: Box::new(self.lower_expr(expr)?),
            }),
            ResolvedExpr::Call { callee, args, span } => {
                let func_name = match callee.as_ref() {
                    ResolvedExpr::Ident(name) => name,
                    _ => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "only identifier calls are supported in expression context"
                                .to_owned(),
                            span: Some(*span),
                        });
                    }
                };

                if let Ok(local_id) = self.resolve_local(func_name)
                    && let Some(closure) = self.arrow_locals.get(&local_id).cloned()
                {
                    let mut lowered_args = self.lower_call_args(args)?;
                    lowered_args.extend(closure.captures.iter().copied().map(LoweredExpr::Local));
                    return Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(closure.func_id),
                        args: lowered_args,
                    });
                }

                if let Ok(local_id) = self.resolve_local(func_name)
                    && self.heap_closure_locals.contains(&local_id)
                {
                    let mut lowered_args = vec![LoweredExpr::Local(local_id)];
                    lowered_args.extend(self.lower_call_args(args)?);
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "HeapClosureCall".to_owned(),
                        args: lowered_args,
                    });
                }

                if func_name == "super" {
                    if !self.in_constructor {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super(...) is only supported in constructors".to_owned(),
                            span: None,
                        });
                    }
                    let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "super(...) requires class context".to_owned(),
                        span: None,
                    })?;
                    let parent_name = self
                        .class_parents
                        .get(class_name)
                        .and_then(|p| p.clone())
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super(...) used in class without extends".to_owned(),
                            span: None,
                        })?;
                    let parent_ctor = self
                        .class_constructor_ids
                        .get(&parent_name)
                        .copied()
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "super class constructor for `{}` not found",
                                parent_name
                            ),
                            span: None,
                        })?;

                    let mut lowered_args = vec![LoweredExpr::Local(self.resolve_local("this")?)];
                    lowered_args.extend(
                        args.iter()
                            .map(|arg| self.lower_expr(arg))
                            .collect::<Result<Vec<_>, _>>()?,
                    );

                    return Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(parent_ctor),
                        args: lowered_args,
                    });
                }

                if func_name == "String" {
                    if let [arg] = args.as_slice()
                        && self.resolved_expr_is_bigint(arg)
                    {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "BigIntToString".to_owned(),
                            args: vec![self.lower_expr(arg)?],
                        });
                    }
                }

                if func_name == "Boolean" {
                    if let [ResolvedExpr::BigIntLiteral { .. }] = args.as_slice() {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "BigIntToBoolean".to_owned(),
                            args: vec![self.lower_expr(&args[0])?],
                        });
                    }
                }

                let func_id = match self.resolve_func(func_name) {
                    Ok(func_id) => func_id,
                    Err(_) if self.resolve_local(func_name).is_ok() => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-211: function-valued local calls such as extracted method `{func_name}(...)` are not supported; call receiver.method(...) directly"
                            ),
                            span: Some(*span),
                        });
                    }
                    Err(err) => return Err(err),
                };
                if self
                    .function_signatures
                    .get(&func_id)
                    .is_some_and(|signature| signature.needs_receiver)
                {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-062d: direct call `{func_name}(...)` cannot bind a supported receiver for `this`; call through a supported receiver object"
                        ),
                        span: Some(*span),
                    });
                }
                let lowered_args =
                    self.lower_function_call_args(func_id, LoweredExpr::Undefined, args)?;

                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(func_id),
                    args: lowered_args,
                })
            }
            ResolvedExpr::BuiltinCall { builtin, args } => {
                let lowered_args = args
                    .iter()
                    .map(|arg| self.lower_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(*builtin),
                    args: lowered_args,
                })
            }
            ResolvedExpr::BuiltinProperty {
                builtin,
                object,
                span,
            } => match builtin {
                BuiltinPropertyId::Length => match object.as_ref() {
                    ResolvedExpr::Ident(name) if self.resolve_func(name).is_ok() => {
                        self.lower_function_metadata_property(name, "length", *span)
                    }
                    _ => Ok(LoweredExpr::GetLength(Box::new(self.lower_expr(object)?))),
                },
            },
            ResolvedExpr::PropertyAccess { object, key, span } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(Some(*span)));
                }
                if key.starts_with('#') {
                    if self.current_private_method_id(key).is_some() {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private method `{key}` extraction is not supported in this private method runtime slice; call it directly as `this.{key}(...)`"
                            ),
                            span: Some(*span),
                        });
                    }
                    if let Some(getter_id) = self.current_private_getter_id(key) {
                        if !matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-255: private getter `{key}` access is currently supported only as `this.{key}` inside the declaring class"
                                ),
                                span: Some(*span),
                            });
                        }
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(getter_id),
                            args: vec![LoweredExpr::Local(self.resolve_local("this")?)],
                        });
                    }
                    if let Some(class_name) = self.infer_class_for_expr(object)
                        && self.private_getter_id_for_class(&class_name, key).is_some()
                    {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private getter `{key}` external access is not supported in this private accessor runtime slice"
                            ),
                            span: Some(*span),
                        });
                    }
                    let slot = self.private_field_slot(object, key, *span)?;
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "PrivateFieldGet".to_owned(),
                        args: vec![self.lower_expr(object)?, LoweredExpr::Number(slot as i32)],
                    });
                }
                if let ResolvedExpr::Ident(name) = object.as_ref()
                    && self.resolve_func(name).is_ok()
                {
                    return self.lower_function_metadata_property(name, key, *span);
                }
                Ok(LoweredExpr::PropertyGet {
                    obj: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                })
            },
            ResolvedExpr::OptionalPropertyAccess { object, key, .. } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::OptionalPropertyGet {
                    obj: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                })
            }
            ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::OptionalIndex {
                    object: Box::new(self.lower_expr(object)?),
                    index: Box::new(self.lower_expr(index)?),
                })
            }
            ResolvedExpr::OptionalCall { callee, args, span } => {
                self.lower_optional_call(callee, args, *span)
            }
            ResolvedExpr::ComputedIndex { object, index } => {
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                // Lower the object first to determine its type
                let lowered_object = self.lower_expr(object)?;
                let lowered_index = self.lower_expr(index)?;

                // Keep obvious array literals on the compact array helper. Unknown
                // receivers must use the generic index helper so object[stringKey]
                // and array[numberIndex] both preserve JavaScript semantics.
                if matches!(object.as_ref(), ResolvedExpr::String(_)) {
                    Ok(LoweredExpr::Index {
                        object: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                    })
                } else if matches!(object.as_ref(), ResolvedExpr::Array(_))
                    || matches!(lowered_object, LoweredExpr::ArrayNew { .. })
                {
                    Ok(LoweredExpr::ArrayGet {
                        arr: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                    })
                } else {
                    Ok(LoweredExpr::Index {
                        object: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                    })
                }
            }
            ResolvedExpr::Array(elements) => {
                let lowered = elements
                    .iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(LoweredExpr::ArrayNew { elements: lowered })
            }
            ResolvedExpr::Object(props) => {
                let mut lowered = Vec::new();
                for (key, value) in props {
                    if self.is_function_identifier(value) {
                        continue;
                    }
                    lowered.push((key.clone(), self.lower_expr(value)?));
                }
                Ok(LoweredExpr::ObjectNew { props: lowered })
            }
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                span,
            } => {
                if method.starts_with('#') {
                    if let Some(method_id) = self.current_static_private_method_id(method) {
                        let same_class_static_receiver = match object.as_ref() {
                            ResolvedExpr::This { .. } => self.resolve_local("this").is_err(),
                            ResolvedExpr::Ident(name) => {
                                self.current_class.as_deref() == Some(name.as_str())
                            }
                            _ => false,
                        };
                        if same_class_static_receiver {
                            let lowered_args = args
                                .iter()
                                .map(|e| self.lower_expr(e))
                                .collect::<Result<Vec<_>, _>>()?;
                            return Ok(LoweredExpr::Call {
                                kind: FunctionCallKind::User(method_id),
                                args: lowered_args,
                            });
                        }
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: static private method `{method}` calls are currently supported only as `this.{method}(...)` inside static methods or `Class.{method}(...)` inside the declaring class"
                            ),
                            span: Some(*span),
                        });
                    }
                    if !matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private method `{method}` calls are currently supported only as `this.{method}(...)` inside the declaring class"
                            ),
                            span: Some(*span),
                        });
                    }
                    let method_id = self.current_private_method_id(method).ok_or_else(|| {
                        Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private method `{method}` is not declared in this class"
                            ),
                            span: Some(*span),
                        }
                    })?;
                    let mut lowered_args = vec![LoweredExpr::Local(self.resolve_local("this")?)];
                    lowered_args.extend(
                        args.iter()
                            .map(|e| self.lower_expr(e))
                            .collect::<Result<Vec<_>, _>>()?,
                    );
                    return Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(method_id),
                        args: lowered_args,
                    });
                }
                if is_json_static_call(object, method) {
                    validate_json_stringify_args(
                        args,
                        *span,
                        self.function_ids,
                        self.function_signatures,
                    )?;
                    let mut lowered_args = Vec::with_capacity(3);
                    let value = if let (
                        ResolvedExpr::Object(props),
                        Some(replacer_keys),
                    ) = (&args[0], json_stringify_replacer_keys(args, self.function_ids))
                    {
                        let mut lowered_props = Vec::new();
                        for allowed_key in replacer_keys {
                            if lowered_props
                                .iter()
                                .any(|(key, _): &(String, LoweredExpr)| key == &allowed_key)
                            {
                                continue;
                            }

                            if let Some((key, value)) =
                                props.iter().rev().find(|(key, _)| key == &allowed_key)
                            {
                                lowered_props.push((key.clone(), self.lower_expr(value)?));
                            }
                        }
                        LoweredExpr::ObjectNew {
                            props: lowered_props,
                        }
                    } else {
                        self.lower_expr(&args[0])?
                    };
                    lowered_args.push(value);
                    lowered_args.push(match args.get(1) {
                        Some(ResolvedExpr::Array(_)) => LoweredExpr::Null,
                        Some(replacer) => {
                            if let Some(func_id) =
                                json_stringify_function_replacer_id(replacer, self.function_ids)
                            {
                                LoweredExpr::Number(func_id.0 as i32)
                            } else {
                                self.lower_expr(replacer)?
                            }
                        }
                        None => LoweredExpr::Undefined,
                    });
                    lowered_args.push(match args.get(2) {
                        Some(space)
                            if should_ignore_json_stringify_space(space, self.function_ids) =>
                        {
                            LoweredExpr::Undefined
                        }
                        Some(space) => {
                            if let Some(boxed_space) = json_stringify_boxed_space_value(space) {
                                self.lower_expr(boxed_space)?
                            } else {
                                self.lower_expr(space)?
                            }
                        }
                        None => LoweredExpr::Undefined,
                    });
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "JsonStringify".to_owned(),
                        args: lowered_args,
                    })
                } else if is_date_now_live_time_call(object, method) {
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "DateNow".to_owned(),
                        args: vec![],
                    })
                } else if self.is_unsupported_regexp_compile_receiver(object, method) {
                    Err(unsupported_regexp_compile_diagnostic(Some(*span)))
                } else if self.is_object_key_enumeration_leak(object, method, args) {
                    Err(private_storage_observable_access_diagnostic(Some(*span)))
                } else if let Some(regexp_args) = regexp_test_runtime(object, method, args, *span)?
                {
                    let lowered_args = regexp_args
                        .iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "RegExpTest".to_owned(),
                        args: lowered_args,
                    })
                } else if let Some(regexp_args) = regexp_exec_runtime(object, method, args, *span)?
                {
                    let lowered_args = regexp_args
                        .iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "RegExpMatch".to_owned(),
                        args: lowered_args,
                    })
                } else if let Some(regexp_args) =
                    regexp_string_match_runtime(object, method, args, *span)?
                {
                    let lowered_args = regexp_args
                        .iter()
                        .map(|e| self.lower_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "RegExpMatch".to_owned(),
                        args: lowered_args,
                    })
                } else if matches!(method.as_str(), "getTime" | "valueOf")
                    && self.is_date_receiver(object)
                {
                    if !args.is_empty() {
                        return Err(Diagnostic {
                            code: DiagCode::ArityMismatch,
                            message: format!(
                                "Date.prototype.{method} expects 0 arguments, got {}",
                                args.len()
                            ),
                            span: Some(*span),
                        });
                    }
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "DateGetTime".to_owned(),
                        args: vec![self.lower_expr(object)?],
                    })
                } else if is_annex_b_date_method(method) && self.is_date_receiver(object) {
                    Err(unsupported_annex_b_date_method_diagnostic(
                        method,
                        Some(*span),
                    ))
                } else if method == "toString" && self.is_date_receiver(object) {
                    Err(unsupported_date_timezone_diagnostic(method, Some(*span)))
                } else if matches!(object.as_ref(), ResolvedExpr::String(_)) {
                    if let Some(diagnostic) = unsupported_annex_b_string_method(method, *span) {
                        Err(diagnostic)
                    } else if let Some(runtime_fn) = resolve_method_to_runtime_fn(object, method) {
                        let mut lowered_args = vec![self.lower_expr(object)?];
                        lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<
                            Result<Vec<_>, _>,
                        >(
                        )?);
                        Ok(LoweredExpr::RuntimeCall {
                            runtime_fn,
                            args: lowered_args,
                        })
                    } else {
                        Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "String.prototype.{method} is not supported in this milestone"
                            ),
                            span: Some(*span),
                        })
                    }
                } else if let Some(runtime_fn) = resolve_method_to_runtime_fn(object, method) {
                    let mut lowered_args = Vec::new();
                    let is_static_call = matches!(
                        object.as_ref(),
                        ResolvedExpr::Ident(name) if name == "Math" || name == "JSON" || name == "Object" || name == "String"
                    );
                    if !is_static_call {
                        lowered_args.push(self.lower_expr(object)?);
                    }
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn,
                        args: lowered_args,
                    })
                } else {
                    if let ResolvedExpr::Ident(receiver_name) = object.as_ref()
                        && let Ok(obj_local) = self.resolve_local(receiver_name)
                        && let Some(method_id) = self
                            .object_function_props
                            .get(&obj_local)
                            .and_then(|props| props.get(method))
                            .copied()
                    {
                        let lowered_args = self.lower_function_call_args(
                            method_id,
                            LoweredExpr::Local(obj_local),
                            args,
                        )?;
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    if matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                        let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "this.method(...) requires class context".to_owned(),
                            span: Some(*span),
                        })?;
                        let method_id =
                            self.resolve_class_method(class_name, method)
                                .ok_or_else(|| Diagnostic {
                                    code: DiagCode::UnsupportedSyntax,
                                    message: format!(
                                        "method `{}.{}` not found",
                                        class_name, method
                                    ),
                                    span: Some(*span),
                                })?;

                        let mut lowered_args =
                            vec![LoweredExpr::Local(self.resolve_local("this")?)];
                        lowered_args.extend(
                            args.iter()
                                .map(|e| self.lower_expr(e))
                                .collect::<Result<Vec<_>, _>>()?,
                        );
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    let receiver_name = match object.as_ref() {
                        ResolvedExpr::Ident(name) => name,
                        _ => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-211: method `{}` requires an identifier receiver",
                                    method
                                ),
                                span: Some(*span),
                            });
                        }
                    };

                    if let Ok(obj_local) = self.resolve_local(receiver_name)
                        && let Some(class_name) = self.local_classes.get(&obj_local)
                        && let Some(runtime_fn) = collection_method_runtime_fn(class_name, method)
                    {
                        if class_name == "RegExp" && args.len() != 1 {
                            return Err(Diagnostic {
                                code: DiagCode::ArityMismatch,
                                message: format!(
                                    "RegExp.prototype.{method} expects 1 argument, got {}",
                                    args.len()
                                ),
                                span: Some(*span),
                            });
                        }
                        let mut lowered_args = vec![LoweredExpr::Local(obj_local)];
                        lowered_args.extend(
                            args.iter()
                                .map(|e| self.lower_expr(e))
                                .collect::<Result<Vec<_>, _>>()?,
                        );
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: runtime_fn.to_owned(),
                            args: lowered_args,
                        });
                    }

                    if receiver_name == "super" {
                        let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super.method(...) requires class context".to_owned(),
                            span: Some(*span),
                        })?;
                        let parent_name = self
                            .class_parents
                            .get(class_name)
                            .and_then(|p| p.clone())
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "super.method(...) used in class without extends"
                                    .to_owned(),
                                span: Some(*span),
                            })?;
                        let method_id = self
                            .resolve_class_method(&parent_name, method)
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "super method `{}.{}` not found",
                                    parent_name, method
                                ),
                                span: Some(*span),
                            })?;

                        let mut lowered_args =
                            vec![LoweredExpr::Local(self.resolve_local("this")?)];
                        lowered_args.extend(
                            args.iter()
                                .map(|e| self.lower_expr(e))
                                .collect::<Result<Vec<_>, _>>()?,
                        );
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    if let Some(method_id) = self
                        .class_static_method_ids
                        .get(&(receiver_name.clone(), method.clone()))
                        .copied()
                    {
                        let lowered_args = args
                            .iter()
                            .map(|e| self.lower_expr(e))
                            .collect::<Result<Vec<_>, _>>()?;
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    let obj_local = self.resolve_local(receiver_name)?;

                    let class_name =
                        self.local_classes
                            .get(&obj_local)
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-211: unknown receiver class for method `{}`",
                                    method
                                ),
                                span: Some(*span),
                            })?;

                    let method_id =
                        self.resolve_class_method(class_name, method)
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!("method `{}.{}` not found", class_name, method),
                                span: Some(*span),
                            })?;

                    let mut lowered_args = vec![LoweredExpr::Local(obj_local)];
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);

                    Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(method_id),
                        args: lowered_args,
                    })
                }
            }
            ResolvedExpr::PropertyAssign {
                object,
                key,
                value,
                span,
            } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(Some(*span)));
                }
                if key.starts_with('#') {
                    if let Some(setter_id) = self.current_private_setter_id(key) {
                        if !matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-255: private setter `{key}` assignment is currently supported only as `this.{key} = value` inside the declaring class"
                                ),
                                span: Some(*span),
                            });
                        }
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(setter_id),
                            args: vec![
                                LoweredExpr::Local(self.resolve_local("this")?),
                                self.lower_expr(value)?,
                            ],
                        });
                    }
                    if let Some(class_name) = self.infer_class_for_expr(object)
                        && self.private_setter_id_for_class(&class_name, key).is_some()
                    {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private setter `{key}` external assignment is not supported in this private setter runtime slice"
                            ),
                            span: Some(*span),
                        });
                    }
                    let slot = self.private_field_slot(object, key, *span)?;
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "PrivateFieldSet".to_owned(),
                        args: vec![
                            self.lower_expr(object)?,
                            LoweredExpr::Number(slot as i32),
                            self.lower_expr(value)?,
                        ],
                    });
                }
                Ok(LoweredExpr::PropertySet {
                    object: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                    value: Box::new(self.lower_expr(value)?),
                })
            }
            ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::PropertySetDynamic {
                    object: Box::new(self.lower_expr(object)?),
                    index: Box::new(self.lower_expr(key)?),
                    value: Box::new(self.lower_expr(value)?),
                })
            }
            ResolvedExpr::New {
                class_name,
                args,
                span: _,
            } => {
                if class_name == "RegExp" {
                    return Ok(LoweredExpr::String(regexp_constructor_literal(args)?));
                }
                if class_name == "Date" {
                    if args.is_empty() {
                        return Ok(LoweredExpr::RuntimeCall {
                            runtime_fn: "DateNewLive".to_owned(),
                            args: vec![],
                        });
                    }
                    if args.len() != 1 {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-050: only deterministic new Date(<epoch-ms integer>) is supported in this slice"
                                .to_owned(),
                            span: None,
                        });
                    }
                    let epoch_ms = &args[0];
                    if !is_date_constructor_epoch_arg(epoch_ms) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-050: Date constructor currently requires an integer epoch millisecond literal".to_owned(),
                            span: None,
                        });
                    }
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "DateNew".to_owned(),
                        args: vec![self.lower_expr(epoch_ms)?],
                    });
                }
                if class_name == "Map" || class_name == "Set" {
                    if !args.is_empty() {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-049: new {class_name}(iterable) is not supported yet"
                            ),
                            span: None,
                        });
                    }
                    return Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: format!("{class_name}New"),
                        args: Vec::new(),
                    });
                }
                if let Some(constructor) = BuiltinErrorConstructor::from_name(class_name) {
                    let message = match args.first() {
                        Some(message) => LoweredExpr::RuntimeCall {
                            runtime_fn: "ErrorMessage".to_owned(),
                            args: vec![self.lower_expr(message)?],
                        },
                        None => LoweredExpr::String(String::new()),
                    };
                    return Ok(LoweredExpr::ErrorNew {
                        constructor,
                        message: Box::new(message),
                    });
                }

                let prototype = self.class_prototype_ref(class_name)?;

                let lowered_args = args
                    .iter()
                    .map(|arg| self.lower_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(LoweredExpr::New {
                    constructor: prototype.constructor,
                    prototype,
                    args: lowered_args,
                    base_local: self.alloc_temp(),
                    private_slot_count: self.private_slot_count(class_name),
                })
            }
            ResolvedExpr::ModuleLoad { specifier } => Ok(LoweredExpr::ModuleLoad {
                module_id: self.module_id_for_specifier(specifier),
            }),
            ResolvedExpr::ArrowFn { params, body } => self.lower_arrow_fn(params, body),
        }
    }

    fn lower_call_args(&mut self, args: &[ResolvedExpr]) -> Result<Vec<LoweredExpr>, Diagnostic> {
        let mut lowered_args = Vec::new();
        for arg in args {
            match arg {
                ResolvedExpr::Spread(spread_expr) => {
                    if let ResolvedExpr::Array(elements) = spread_expr.as_ref() {
                        for elem in elements {
                            lowered_args.push(self.lower_expr(elem)?);
                        }
                    } else {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message:
                                "spread arguments are only supported for literal arrays in this milestone"
                                    .to_owned(),
                            span: None,
                        });
                    }
                }
                _ => lowered_args.push(self.lower_expr(arg)?),
            }
        }
        Ok(lowered_args)
    }

    fn lower_binding_pattern_declarations(
        &mut self,
        pattern: &BindingPattern,
        value: LoweredExpr,
        source: Option<&ResolvedExpr>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        match pattern {
            BindingPattern::Array(bindings) => {
                let mut statements = Vec::new();
                for binding in bindings {
                    statements.extend(self.lower_array_binding_declaration(binding, &value)?);
                }
                Ok(statements)
            }
            BindingPattern::Object(bindings) => {
                let mut statements = Vec::new();
                for binding in bindings {
                    statements.extend(self.lower_object_binding_declaration(
                        binding, bindings, &value, source,
                    )?);
                }
                Ok(statements)
            }
        }
    }

    fn lower_array_binding_declaration(
        &mut self,
        binding: &ArrayBinding,
        value: &LoweredExpr,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let element_value = if binding.is_rest {
            LoweredExpr::RuntimeCall {
                runtime_fn: "ArraySlice".to_owned(),
                args: vec![
                    value.clone(),
                    LoweredExpr::Number(binding.index as i32),
                    LoweredExpr::GetLength(Box::new(value.clone())),
                ],
            }
        } else {
            LoweredExpr::Index {
                object: Box::new(value.clone()),
                index: Box::new(LoweredExpr::Number(binding.index as i32)),
            }
        };
        let Some(name) = binding.target.identifier() else {
            if let Some(pattern) = binding.target.pattern() {
                return self.lower_binding_pattern_declarations(pattern, element_value, None);
            }
            unreachable!("binding target must be identifier or pattern");
        };
        let local_id = self.declare_local(name)?;
        if binding.is_rest {
            return Ok(vec![LoweredStmt::Let(local_id, element_value)]);
        }
        self.lower_binding_declaration_with_default(
            local_id,
            element_value,
            binding.default.as_ref(),
        )
    }

    fn lower_object_binding_declaration(
        &mut self,
        binding: &ObjectBinding,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        source: Option<&ResolvedExpr>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let property_value = LoweredExpr::PropertyGet {
            obj: Box::new(value.clone()),
            key: binding.key.clone(),
        };
        let Some(name) = binding.target.identifier() else {
            if let Some(pattern) = binding.target.pattern() {
                return self.lower_binding_pattern_declarations(pattern, property_value, None);
            }
            unreachable!("binding target must be identifier or pattern");
        };
        let local_id = self.declare_local(name)?;
        if binding.is_rest {
            return self.lower_object_rest_binding_declaration(
                local_id,
                siblings,
                value,
                source,
                binding.span,
            );
        }
        self.lower_binding_declaration_with_default(
            local_id,
            property_value,
            binding.default.as_ref(),
        )
    }

    fn lower_object_rest_binding_declaration(
        &mut self,
        local_id: LocalId,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        source: Option<&ResolvedExpr>,
        span: Option<Span>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let Some(ResolvedExpr::Object(props)) = source else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-251: object rest binding currently requires a static object literal source in this runtime slice".to_owned(),
                span,
            });
        };
        let excluded_keys = siblings
            .iter()
            .filter(|binding| !binding.is_rest)
            .map(|binding| binding.key.as_str())
            .collect::<HashSet<_>>();
        let rest_props = props
            .iter()
            .filter(|(key, _)| !excluded_keys.contains(key.as_str()))
            .map(|(key, _)| {
                (
                    key.clone(),
                    LoweredExpr::PropertyGet {
                        obj: Box::new(value.clone()),
                        key: key.clone(),
                    },
                )
            })
            .collect();
        Ok(vec![LoweredStmt::Let(
            local_id,
            LoweredExpr::ObjectNew { props: rest_props },
        )])
    }

    fn lower_binding_declaration_with_default(
        &mut self,
        local_id: LocalId,
        value: LoweredExpr,
        default: Option<&BindingDefault>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let Some(default) = default else {
            return Ok(vec![LoweredStmt::Let(local_id, value)]);
        };
        let temp_id = self.alloc_temp();
        Ok(vec![
            LoweredStmt::Let(temp_id, value),
            LoweredStmt::Let(local_id, LoweredExpr::Local(temp_id)),
            LoweredStmt::If {
                condition: LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(temp_id)),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined),
                },
                then_body: vec![LoweredStmt::Assign(
                    local_id,
                    lowered_binding_default(default),
                )],
                else_body: vec![],
            },
        ])
    }

    fn lower_optional_call(
        &mut self,
        callee: &ResolvedExpr,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let func_name = match callee {
            ResolvedExpr::Ident(name) => name,
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-253: optional calls are currently supported only for identifier callees"
                            .to_owned(),
                    span: Some(span),
                });
            }
        };

        if let Ok(local_id) = self.resolve_local(func_name) {
            if self.nullish_locals.contains(&local_id) {
                return Ok(LoweredExpr::Undefined);
            }

            if let Some(closure) = self.arrow_locals.get(&local_id).cloned() {
                let mut lowered_args = self.lower_call_args(args)?;
                lowered_args.extend(closure.captures.iter().copied().map(LoweredExpr::Local));
                return Ok(LoweredExpr::OptionalCall {
                    callee: Box::new(LoweredExpr::Local(local_id)),
                    call: Box::new(LoweredExpr::Call {
                        kind: FunctionCallKind::User(closure.func_id),
                        args: lowered_args,
                    }),
                });
            }

            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-253: optional call `{func_name}?.(...)` is supported only for known functions or nullish locals"
                ),
                span: Some(span),
            });
        }

        let func_id = self.resolve_func(func_name)?;
        if self
            .function_signatures
            .get(&func_id)
            .is_some_and(|signature| signature.needs_receiver)
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062d: optional direct call `{func_name}?.(...)` cannot bind a supported receiver for `this`; call through a supported receiver object"
                ),
                span: Some(span),
            });
        }
        let lowered_args = self.lower_function_call_args(func_id, LoweredExpr::Undefined, args)?;
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: lowered_args,
        })
    }

    fn lower_function_call_args(
        &mut self,
        func_id: FuncId,
        receiver: LoweredExpr,
        args: &[ResolvedExpr],
    ) -> Result<Vec<LoweredExpr>, Diagnostic> {
        let signature = self
            .function_signatures
            .get(&func_id)
            .copied()
            .unwrap_or_default();
        let explicit_args = self.lower_call_args(args)?;
        let mut lowered_args = Vec::new();

        if signature.needs_receiver {
            lowered_args.push(receiver);
        }

        if signature.has_rest {
            lowered_args.extend(explicit_args.iter().cloned());
        } else {
            lowered_args.extend(explicit_args.iter().take(signature.explicit_params).cloned());
            for _ in explicit_args.len()..signature.explicit_params {
                lowered_args.push(LoweredExpr::Undefined);
            }
        }

        if signature.needs_arguments {
            lowered_args.push(LoweredExpr::ArrayNew {
                elements: explicit_args,
            });
        }

        Ok(lowered_args)
    }

    fn function_props_for_object_expr(
        &self,
        expr: &ResolvedExpr,
    ) -> Option<HashMap<String, FuncId>> {
        let ResolvedExpr::Object(props) = expr else {
            return None;
        };
        let function_props = props
            .iter()
            .filter_map(|(key, value)| {
                if let ResolvedExpr::Ident(name) = value {
                    self.resolve_func(name).ok().map(|func_id| (key.clone(), func_id))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();
        if function_props.is_empty() {
            None
        } else {
            Some(function_props)
        }
    }

    fn is_function_identifier(&self, expr: &ResolvedExpr) -> bool {
        matches!(expr, ResolvedExpr::Ident(name) if self.resolve_func(name).is_ok())
    }

    fn lower_function_metadata_property(
        &self,
        name: &str,
        key: &str,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let func_id = self.resolve_func(name)?;
        match key {
            "name" => Ok(LoweredExpr::String(name.to_owned())),
            "length" => {
                let signature = self
                    .function_signatures
                    .get(&func_id)
                    .copied()
                    .unwrap_or_default();
                if let Some(length) = signature.metadata_length {
                    Ok(LoweredExpr::Number(length as i32))
                } else {
                    Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-062f: function `{name}` length metadata is only supported for fixed-arity function declarations"
                        ),
                        span: Some(span),
                    })
                }
            }
            "prototype" => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062f: function `{name}` prototype metadata is not supported in this slice"
                ),
                span: Some(span),
            }),
            _ => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062f: function `{name}` metadata property `{key}` is not supported"
                ),
                span: Some(span),
            }),
        }
    }

    fn lower_arrow_fn(
        &mut self,
        params: &[String],
        body: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        let capture_names = self.arrow_capture_names(params, body)?;
        let captures = capture_names
            .iter()
            .map(|name| self.resolve_local(name))
            .collect::<Result<Vec<_>, _>>()?;
        let mut lowered_params = params
            .iter()
            .map(|name| ResolvedParam {
                name: name.clone(),
                default: None,
                is_rest: false,
                span: None,
            })
            .collect::<Vec<_>>();
        lowered_params.extend(capture_names.iter().map(|name| ResolvedParam {
            name: name.clone(),
            default: None,
            is_rest: false,
            span: None,
        }));

        let func_id = FuncId(self.next_func_id);
        self.next_func_id += 1;
        let body_stmts = vec![ResolvedStmt::Return((*body).clone())];
        let lowered = lower_function(
            func_id,
            &lowered_params,
            &body_stmts,
            self.function_ids,
            self.function_signatures,
            self.class_parents.clone(),
            self.class_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.current_class.as_deref(),
                in_constructor: false,
                next_func_id: self.next_func_id,
                self_closure: None,
            },
        )?;
        self.next_func_id = lowered.next_func_id;
        self.generated_functions.push(lowered.function);
        self.generated_functions.extend(lowered.generated_functions);

        Ok(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: ClosureRepresentation::DirectLocalToken,
        })
    }

    fn lower_nested_function(
        &mut self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
    ) -> Result<LoweredExpr, Diagnostic> {
        if params
            .iter()
            .any(|param| param.default.is_some() || param.is_rest)
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` closure parameters with defaults or rest are not supported in this slice"
                ),
                span: None,
            });
        }
        if block_contains_this(body) || block_contains_arguments(body) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` closures with `this` or `arguments` are not supported in this slice"
                ),
                span: None,
            });
        }

        let capture_names = self.nested_function_capture_names(name, params, body)?;
        if block_assigns_any_name(body, &capture_names) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062e: nested function `{name}` mutates a captured outer local; mutable closure environments require heap environment support"
                ),
                span: None,
            });
        }
        let captures = capture_names
            .iter()
            .map(|capture| self.resolve_local(capture))
            .collect::<Result<Vec<_>, _>>()?;
        let mut lowered_params = params.to_vec();
        lowered_params.extend(
            capture_names
                .iter()
                .map(|capture| ResolvedParam {
                    name: capture.clone(),
                    default: None,
                    is_rest: false,
                    span: None,
                }),
        );

        let func_id = FuncId(self.next_func_id);
        self.next_func_id += 1;
        let lowered = lower_function(
            func_id,
            &lowered_params,
            body,
            self.function_ids,
            self.function_signatures,
            self.class_parents.clone(),
            self.class_private_fields.clone(),
            LowerFunctionOptions {
                current_class: self.current_class.as_deref(),
                in_constructor: false,
                next_func_id: self.next_func_id,
                self_closure: Some(SelfClosureOptions {
                    name,
                    func_id,
                    capture_names: &capture_names,
                }),
            },
        )?;
        self.next_func_id = lowered.next_func_id;
        self.generated_functions.push(lowered.function);
        self.generated_functions.extend(lowered.generated_functions);

        Ok(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: ClosureRepresentation::DirectLocalToken,
        })
    }

    fn arrow_capture_names(
        &self,
        params: &[String],
        body: &ResolvedExpr,
    ) -> Result<Vec<String>, Diagnostic> {
        let excluded = binding_param_names(params.iter().map(|param| (param.as_str(), None)))?;
        let mut captures = Vec::new();
        collect_arrow_captures(body, &excluded, &mut captures);
        Ok(captures
            .into_iter()
            .filter(|name| self.resolve_local(name).is_ok())
            .collect())
    }

    fn nested_function_capture_names(
        &self,
        name: &str,
        params: &[ResolvedParam],
        body: &[ResolvedStmt],
    ) -> Result<Vec<String>, Diagnostic> {
        let mut excluded = binding_param_names(
            params
                .iter()
                .map(|param| (param.name.as_str(), param.span)),
        )?
        .into_iter()
        .collect::<HashSet<_>>();
        excluded.insert(name.to_owned());
        collect_declared_names_in_stmts(body, &mut excluded);

        let mut captures = Vec::new();
        collect_stmt_captures(body, &excluded, &mut captures);
        Ok(captures
            .into_iter()
            .filter(|capture| self.resolve_local(capture).is_ok())
            .collect())
    }

    fn declare_local(&mut self, name: &str) -> Result<LocalId, Diagnostic> {
        let scope = self.scopes.last_mut().expect("scope must exist");
        if scope.contains_key(name) {
            return Err(Diagnostic {
                code: DiagCode::DuplicateLocal,
                message: format!("duplicate local binding: `{name}`"),
                span: None,
            });
        }
        let local_id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(local_id);
        scope.insert(name.to_owned(), local_id);
        Ok(local_id)
    }

    fn declare_self_closure(
        &mut self,
        name: &str,
        func_id: FuncId,
        capture_names: &[String],
    ) -> Result<(), Diagnostic> {
        let local_id = self.declare_local(name)?;
        let captures = capture_names
            .iter()
            .map(|capture| self.resolve_local(capture))
            .collect::<Result<Vec<_>, _>>()?;
        self.arrow_locals
            .insert(local_id, ArrowClosure { func_id, captures });
        Ok(())
    }

    fn alloc_temp(&mut self) -> LocalId {
        let id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(id);
        id
    }

    fn resolve_local(&self, name: &str) -> Result<LocalId, Diagnostic> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedName,
                message: format!("unresolved name: `{name}`"),
                span: None,
            })
    }

    fn resolve_func(&self, name: &str) -> Result<FuncId, Diagnostic> {
        self.function_ids
            .get(name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedFunction,
                message: format!("unresolved function: `{name}`"),
                span: None,
            })
    }

    fn module_id_for_specifier(&mut self, specifier: &str) -> usize {
        if let Some(id) = self.module_ids.get(specifier) {
            return *id;
        }

        let id = self.modules.len() + 1;
        self.module_ids.insert(specifier.to_owned(), id);
        self.modules.push(ModuleInfo {
            id,
            specifier: specifier.to_owned(),
            statements: Vec::new(),
            locals_count: 0,
        });
        id
    }

    fn resolve_class_method(&self, class_name: &str, method: &str) -> Option<FuncId> {
        let mut current = Some(class_name.to_owned());
        while let Some(class) = current {
            if let Some(id) = self
                .class_method_ids
                .get(&(class.clone(), method.to_owned()))
                .copied()
            {
                return Some(id);
            }
            current = self.class_parents.get(&class).and_then(|p| p.clone());
        }
        None
    }

    fn current_private_method_id(&self, method: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        self.class_method_ids
            .get(&(class_name.clone(), method.to_owned()))
            .copied()
    }

    fn current_static_private_method_id(&self, method: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        self.class_static_method_ids
            .get(&(class_name.clone(), method.to_owned()))
            .copied()
    }

    fn current_private_getter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        self.private_getter_id_for_class(class_name, key)
    }

    fn private_getter_id_for_class(&self, class_name: &str, key: &str) -> Option<FuncId> {
        let getter_name = key.strip_prefix('#')?;
        self.class_method_ids
            .get(&(class_name.to_owned(), format!("#get::{getter_name}")))
            .copied()
    }

    fn current_private_setter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.current_class.as_ref()?;
        self.private_setter_id_for_class(class_name, key)
    }

    fn private_setter_id_for_class(&self, class_name: &str, key: &str) -> Option<FuncId> {
        let setter_name = key.strip_prefix('#')?;
        self.class_method_ids
            .get(&(class_name.to_owned(), format!("#set::{setter_name}")))
            .copied()
    }

    fn private_field_slot(
        &self,
        object: &ResolvedExpr,
        key: &str,
        span: Span,
    ) -> Result<usize, Diagnostic> {
        let Some(field_name) = key.strip_prefix('#') else {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!("private field slot lookup requires private key, got `{key}`"),
                span: Some(span),
            });
        };
        if !matches!(object, ResolvedExpr::This { .. }) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-255: private field `#{field_name}` access is currently supported only as `this.#{field_name}` inside class methods and constructors"
                ),
                span: Some(span),
            });
        }
        let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-255: private field `#{field_name}` access requires class context"
            ),
            span: Some(span),
        })?;
        let Some(slot) = self
            .class_private_fields
            .get(class_name)
            .and_then(|fields| fields.get(field_name))
            .copied()
        else
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-255: private field `#{field_name}` is not declared in class `{class_name}`"
                ),
                span: Some(span),
            });
        };
        Ok(slot)
    }

    fn private_slot_count(&self, class_name: &str) -> usize {
        self.class_private_fields
            .get(class_name)
            .map_or(0, HashMap::len)
    }

    fn is_object_key_enumeration_leak(
        &self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
    ) -> bool {
        matches!(object, ResolvedExpr::Ident(name) if name == "Object")
            && matches!(method, "keys" | "values" | "entries")
            && args
                .first()
                .is_some_and(|arg| self.expr_has_private_progress_storage(arg))
    }

    fn expr_has_private_progress_storage(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::This { .. } => self
                .current_class
                .as_ref()
                .is_some_and(|class_name| self.class_has_private_progress_storage(class_name)),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local| self.local_has_private_progress_storage(local)),
            ResolvedExpr::New { class_name, .. } => self.class_has_private_progress_storage(class_name),
            _ => false,
        }
    }

    fn local_has_private_progress_storage(&self, local: LocalId) -> bool {
        self.local_classes
            .get(&local)
            .is_some_and(|class_name| self.class_has_private_progress_storage(class_name))
    }

    fn class_has_private_progress_storage(&self, class_name: &str) -> bool {
        self.class_private_fields
            .get(class_name)
            .is_some_and(|fields| !fields.is_empty())
    }

    fn is_date_receiver(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::New { class_name, .. } => class_name == "Date",
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .and_then(|local_id| self.local_classes.get(&local_id))
                .is_some_and(|class_name| class_name == "Date"),
            _ => false,
        }
    }

    fn is_unsupported_regexp_compile_receiver(&self, expr: &ResolvedExpr, method: &str) -> bool {
        if method != "compile" {
            return false;
        }
        match expr {
            ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => true,
            ResolvedExpr::New { class_name, .. } => class_name == "RegExp",
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local_id| {
                self.regexp_literal_locals.contains(&local_id)
                    || self
                        .local_classes
                        .get(&local_id)
                        .is_some_and(|class_name| class_name == "RegExp")
            }),
            _ => false,
        }
    }

    fn update_regexp_literal_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if matches!(expr, ResolvedExpr::String(raw) if looks_like_regexp_literal(raw)) {
            self.regexp_literal_locals.insert(local_id);
        } else {
            self.regexp_literal_locals.remove(&local_id);
        }
    }

    fn update_bigint_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if self.resolved_expr_is_bigint(expr) {
            self.bigint_locals.insert(local_id);
        } else {
            self.bigint_locals.remove(&local_id);
        }
    }

    fn update_heap_closure_local(
        &mut self,
        local_id: LocalId,
        expr: &ResolvedExpr,
        lowered: &LoweredExpr,
    ) {
        if self.expr_is_known_heap_closure(expr)
            || matches!(
                lowered,
                LoweredExpr::ArrowFn {
                    representation: ClosureRepresentation::HeapObject,
                    ..
                }
            )
        {
            self.heap_closure_locals.insert(local_id);
        } else {
            self.heap_closure_locals.remove(&local_id);
        }
    }

    fn update_nullish_local(&mut self, local_id: LocalId, expr: &ResolvedExpr) {
        if self.resolved_expr_is_nullish(expr) {
            self.nullish_locals.insert(local_id);
        } else {
            self.nullish_locals.remove(&local_id);
        }
    }

    fn resolved_expr_is_nullish(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Null | ResolvedExpr::Undefined => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.nullish_locals.contains(&local_id)),
            _ => false,
        }
    }

    fn expr_is_known_heap_closure(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Call { callee, .. } => match callee.as_ref() {
                ResolvedExpr::Ident(name) => self
                    .resolve_func(name)
                    .ok()
                    .and_then(|func_id| self.function_signatures.get(&func_id))
                    .is_some_and(|signature| signature.returns_heap_closure),
                _ => false,
            },
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.heap_closure_locals.contains(&local_id)),
            _ => false,
        }
    }

    fn resolved_expr_is_bigint(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::BigIntLiteral { .. } => true,
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local_id| self.bigint_locals.contains(&local_id)),
            ResolvedExpr::Unary { op, expr } => {
                *op == UnaryOp::Negate && self.resolved_expr_is_bigint(expr)
            }
            ResolvedExpr::Binary { left, op, right } => {
                matches!(
                    op,
                    BinaryOp::Add
                        | BinaryOp::Subtract
                        | BinaryOp::Multiply
                        | BinaryOp::Divide
                        | BinaryOp::Modulo
                )
                    && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
            }
            _ => false,
        }
    }

    fn class_prototype_ref(&self, class_name: &str) -> Result<ClassPrototypeRef, Diagnostic> {
        let constructor = self
            .class_constructor_ids
            .get(class_name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-207: instanceof right-hand side must be a supported class constructor `{}`",
                    class_name
                ),
                span: None,
            })?;

        let mut parent_constructors = Vec::new();
        let mut current = self.class_parents.get(class_name).and_then(|p| p.clone());
        while let Some(parent) = current {
            let parent_constructor = self
                .class_constructor_ids
                .get(&parent)
                .copied()
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-207: superclass constructor `{}` is not available for instanceof",
                        parent
                    ),
                    span: None,
                })?;
            parent_constructors.push(parent_constructor);
            current = self.class_parents.get(&parent).and_then(|p| p.clone());
        }

        Ok(ClassPrototypeRef {
            constructor,
            parent_constructors,
        })
    }

    fn infer_class_for_expr(&self, expr: &ResolvedExpr) -> Option<String> {
        match expr {
            ResolvedExpr::New { class_name, .. } => Some(class_name.clone()),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .and_then(|local_id| self.local_classes.get(&local_id).cloned()),
            _ => None,
        }
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
        if let Some(pattern) = parse_binding_pattern(param, span)? {
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
