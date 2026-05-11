mod array;
mod call;
mod class;
mod expr;
mod extra;
mod function;
mod module;
mod object;
mod string;

use std::collections::{HashMap, HashSet};

use crate::binding_pattern::{BindingDefault, parse_binding_pattern};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedStmt};
use crate::lowered::*;
use ts2wasm_shared::{BinaryOp, UnaryOp};
use ts2wasm_shared::{DiagCode, Diagnostic, Span};

pub(crate) struct Symbols<'a> {
    pub(crate) function_ids: &'a HashMap<String, FuncId>,
    pub(crate) function_signatures: &'a HashMap<FuncId, FunctionSignature>,
}

pub(crate) struct Locals {
    pub(crate) scopes: Vec<HashMap<String, LocalId>>,
    pub(crate) next_local_id: usize,
    pub(crate) locals: Vec<LocalId>,
    pub(crate) param_locals: HashSet<LocalId>,
}

pub(crate) struct Functions<'a> {
    pub(crate) function_captures: &'a HashMap<FuncId, Vec<String>>,
    pub(crate) function_mutable_captures: &'a HashMap<FuncId, Vec<String>>,
    pub(crate) class_method_captures: &'a HashMap<FuncId, Vec<String>>,
    pub(crate) class_method_mutable_captures: &'a HashMap<FuncId, Vec<String>>,
    pub(crate) next_func_id: usize,
    pub(crate) generated_functions: Vec<LoweredFunction>,
}

pub(crate) struct Captures {
    pub(crate) env_cell_names: HashSet<String>,
    pub(crate) env_cell_locals: HashSet<LocalId>,
    pub(crate) heap_closure_names: HashSet<String>,
    pub(crate) heap_closure_locals: HashSet<LocalId>,
}

pub(crate) struct Classes {
    pub(crate) class_constructor_ids: HashMap<String, FuncId>,
    pub(crate) class_method_ids: HashMap<(String, String), FuncId>,
    pub(crate) class_static_method_ids: HashMap<(String, String), FuncId>,
    pub(crate) class_parents: HashMap<String, Option<String>>,
    pub(crate) class_private_fields: ClassPrivateFieldSlots,
    pub(crate) class_static_private_fields: ClassStaticPrivateFields,
    pub(crate) local_classes: HashMap<LocalId, String>,
    pub(crate) object_function_props: HashMap<LocalId, HashMap<String, FuncId>>,
    pub(crate) current_class: Option<String>,
    pub(crate) in_constructor: bool,
}

pub(crate) struct Modules {
    pub(crate) module_ids: HashMap<String, usize>,
    pub(crate) modules: Vec<ModuleInfo>,
}

pub(crate) struct Facts {
    pub(crate) arrow_locals: HashMap<LocalId, ArrowClosure>,
    #[allow(dead_code)]
    pub(crate) heap_closure_locals: HashSet<LocalId>,
    pub(crate) nullish_locals: HashSet<LocalId>,
    pub(crate) regexp_literal_locals: HashSet<LocalId>,
    pub(crate) invalid_date_locals: HashSet<LocalId>,
    pub(crate) bigint_locals: HashSet<LocalId>,
    pub(crate) control_flow_bigint_div_rem_locals: HashSet<LocalId>,
    pub(crate) control_flow_mixed_bigint_locals: HashSet<LocalId>,
    pub(crate) array_locals: HashSet<LocalId>,
    pub(crate) static_array_slots: HashMap<LocalId, Vec<ResolvedArrayElement>>,
    pub(crate) symbol_iterator_object_locals: HashSet<LocalId>,
    pub(crate) static_object_literal_locals: HashMap<LocalId, Vec<(String, ResolvedExpr)>>,
    pub(crate) static_object_literal_alias_sources: HashMap<LocalId, HashSet<LocalId>>,
    pub(crate) static_function_array_like_locals: HashMap<LocalId, StaticFunctionArrayLike>,
    pub(crate) string_literal_locals: HashMap<LocalId, String>,
    pub(crate) native_set_add_locals: HashSet<LocalId>,
    pub(crate) generator_function_names: HashSet<String>,
}

pub(super) struct Resolver<'a> {
    pub(crate) symbols: Symbols<'a>,
    pub(crate) locals: Locals,
    pub(crate) functions: Functions<'a>,
    pub(crate) captures: Captures,
    pub(crate) classes: Classes,
    pub(crate) modules: Modules,
    pub(crate) facts: Facts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArrowClosure {
    func_id: FuncId,
    captures: Vec<LocalId>,
}

#[derive(Debug, Clone)]
pub(crate) struct StaticFunctionArrayLike {
    elements: Vec<Option<ResolvedExpr>>,
}

impl ArrowClosure {
    fn to_expr(&self, representation: ClosureRepresentation) -> LoweredExpr {
        LoweredExpr::ArrowFn {
            func_id: self.func_id,
            captures: self.captures.clone(),
            representation,
            span: Span::generated("arrow_fn"),
        }
    }
}

impl<'a> Resolver<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
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
            symbols: Symbols {
                function_ids,
                function_signatures,
            },
            locals: Locals {
                scopes: vec![HashMap::new()],
                next_local_id: 0,
                locals: Vec::new(),
                param_locals: HashSet::new(),
            },
            functions: Functions {
                function_captures,
                function_mutable_captures,
                class_method_captures,
                class_method_mutable_captures,
                next_func_id,
                generated_functions: Vec::new(),
            },
            captures: Captures {
                env_cell_names: env_cell_names.clone(),
                env_cell_locals: HashSet::new(),
                heap_closure_names: heap_closure_names.clone(),
                heap_closure_locals: HashSet::new(),
            },
            classes: Classes {
                class_constructor_ids,
                class_method_ids,
                class_static_method_ids,
                class_parents,
                class_private_fields,
                class_static_private_fields,
                local_classes: HashMap::new(),
                object_function_props: HashMap::new(),
                current_class: None,
                in_constructor: false,
            },
            modules: Modules {
                module_ids: HashMap::new(),
                modules: Vec::new(),
            },
            facts: Facts {
                arrow_locals: HashMap::new(),
                heap_closure_locals: HashSet::new(),
                nullish_locals: HashSet::new(),
                regexp_literal_locals: HashSet::new(),
                invalid_date_locals: HashSet::new(),
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
            },
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn with_params(
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
            symbols: Symbols {
                function_ids,
                function_signatures,
            },
            locals: Locals {
                scopes: vec![HashMap::new()],
                next_local_id: 0,
                locals: Vec::new(),
                param_locals: HashSet::new(),
            },
            functions: Functions {
                function_captures,
                function_mutable_captures,
                class_method_captures,
                class_method_mutable_captures,
                next_func_id,
                generated_functions: Vec::new(),
            },
            captures: Captures {
                env_cell_names: env_cell_names.clone(),
                env_cell_locals: HashSet::new(),
                heap_closure_names: heap_closure_names.clone(),
                heap_closure_locals: HashSet::new(),
            },
            classes: Classes {
                class_constructor_ids,
                class_method_ids,
                class_static_method_ids,
                class_parents,
                class_private_fields,
                class_static_private_fields,
                local_classes: HashMap::new(),
                object_function_props: HashMap::new(),
                current_class: current_class.map(ToOwned::to_owned),
                in_constructor,
            },
            modules: Modules {
                module_ids: HashMap::new(),
                modules: Vec::new(),
            },
            facts: Facts {
                arrow_locals: HashMap::new(),
                heap_closure_locals: HashSet::new(),
                nullish_locals: HashSet::new(),
                regexp_literal_locals: HashSet::new(),
                invalid_date_locals: HashSet::new(),
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
            },
        };
        let mut param_ids = Vec::new();
        let mut seen_params = HashMap::new();

        for param in params {
            let clean_name = param.strip_prefix("...").unwrap_or(param.as_str());
            if seen_params.contains_key(clean_name) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateParameter,
                    message: format!("duplicate parameter name: `{clean_name}`"),
                    span: None,

                    phase: None,
                });
            }
            seen_params.insert(clean_name.to_owned(), ());
            let local_id = LocalId(resolver.locals.next_local_id);
            resolver.locals.next_local_id += 1;
            resolver
                .locals
                .scopes
                .last_mut()
                .expect("function scope must exist")
                .insert(clean_name.to_owned(), local_id);
            if resolver.captures.env_cell_names.contains(clean_name) {
                resolver.captures.env_cell_locals.insert(local_id);
            }
            if resolver.captures.heap_closure_names.contains(clean_name) {
                resolver.captures.heap_closure_locals.insert(local_id);
            }
            resolver.locals.param_locals.insert(local_id);
            if let Some(current_class) = current_class
                && param == "this"
            {
                resolver
                    .classes
                    .local_classes
                    .insert(local_id, current_class.to_owned());
            }
            param_ids.push(local_id);
        }

        Ok((resolver, param_ids))
    }

    pub(crate) fn lower_block(
        &mut self,
        statements: &[ResolvedStmt],
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
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
        self.locals.scopes.push(HashMap::new());
        let lowered = self.lower_block(statements);
        self.locals.scopes.pop();
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
                message: "issue-302: direct eval IIFE lowering does not support function returns"
                    .to_owned(),
                span: Some(*span),

                phase: None,
            });
        }
        if block_contains_this(body) || block_contains_arguments(body) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-302: direct eval IIFE lowering does not support `this` or `arguments`"
                        .to_owned(),
                span: Some(*span),

                phase: None,
            });
        }

        self.locals.scopes.push(HashMap::new());
        let lowered = self.lower_block(body);
        self.locals.scopes.pop();
        lowered.map(|statements| Some(LoweredStmt::Block(statements, Span::generated("block"))))
    }

    pub(crate) fn lower_stmt(&mut self, stmt: &ResolvedStmt) -> Result<LoweredStmt, Diagnostic> {
        match stmt {
            ResolvedStmt::AmbientValue(name) => {
                self.declare_local(name)?;
                Ok(LoweredStmt::Expr(
                    LoweredExpr::Undefined(Span::generated("undef")),
                    Span::generated("expr_stmt"),
                ))
            }
            ResolvedStmt::DestructureLet { pattern, expr } => {
                let value_local = self.alloc_temp();
                let mut statements = vec![LoweredStmt::Let(
                    value_local,
                    self.lower_expr(expr)?,
                    Span::generated("let_stmt"),
                )];
                statements.extend(self.lower_binding_pattern_declarations(
                    pattern,
                    LoweredExpr::Local(value_local, Span::generated("local")),
                    Some(expr),
                )?);
                Ok(LoweredStmt::Block(statements, Span::generated("block")))
            }
            ResolvedStmt::Let(name, expr) => {
                let local_id = self.declare_local(name)?;
                // Infer class before lowering so closures inside the initializer
                // can resolve the class of this local (e.g. `new Howl(...)` with a
                // callback that calls `instance.once(...)`).
                let expr_class = self.infer_class_for_expr(expr);
                if let Some(class_name) = &expr_class {
                    self.classes
                        .local_classes
                        .insert(local_id, class_name.clone());
                }
                let function_props = self.function_props_for_object_expr(expr);
                let lowered = if let ResolvedExpr::ArrowFn {
                    params,
                    body,
                    body_stmts,
                    ..
                } = expr
                {
                    self.lower_arrow_fn_with_self(params, body, body_stmts, Some(name))?
                } else {
                    self.lower_expr(expr)?
                };
                let lowered = if self.captures.env_cell_names.contains(name) {
                    self.captures.env_cell_locals.insert(local_id);
                    LoweredExpr::EnvCellNew(Box::new(lowered), Span::generated("env_cell_new"))
                } else {
                    lowered
                };
                if let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = &lowered
                {
                    self.facts.arrow_locals.insert(
                        local_id,
                        ArrowClosure {
                            func_id: *func_id,
                            captures: captures.clone(),
                        },
                    );
                } else {
                    self.facts.arrow_locals.remove(&local_id);
                }
                self.update_heap_closure_local(local_id, expr, &lowered);
                if self.captures.heap_closure_names.contains(name) {
                    self.captures.heap_closure_locals.insert(local_id);
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
                self.update_invalid_date_local(local_id, expr);
                if let Some(props) = function_props {
                    self.classes.object_function_props.insert(local_id, props);
                } else {
                    self.classes.object_function_props.remove(&local_id);
                }
                self.update_regexp_literal_local(local_id, expr);
                Ok(LoweredStmt::Let(
                    local_id,
                    lowered,
                    Span::generated("let_stmt"),
                ))
            }
            ResolvedStmt::Assign(name, expr) => {
                let local_id = self.resolve_local(name)?;
                self.invalidate_static_object_literal_local(local_id);
                // Infer class before lowering so closures inside the RHS
                // can resolve the class of this local.
                let expr_class = self.infer_class_for_expr(expr);
                if let Some(class_name) = &expr_class {
                    self.classes
                        .local_classes
                        .insert(local_id, class_name.clone());
                } else {
                    self.classes.local_classes.remove(&local_id);
                }
                let function_props = self.function_props_for_object_expr(expr);
                let lowered = self.lower_expr(expr)?;
                if let LoweredExpr::ArrowFn {
                    func_id, captures, ..
                } = &lowered
                {
                    self.facts.arrow_locals.insert(
                        local_id,
                        ArrowClosure {
                            func_id: *func_id,
                            captures: captures.clone(),
                        },
                    );
                } else {
                    self.facts.arrow_locals.remove(&local_id);
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
                self.update_invalid_date_local(local_id, expr);
                if let Some(props) = function_props {
                    self.classes.object_function_props.insert(local_id, props);
                } else {
                    self.classes.object_function_props.remove(&local_id);
                }
                self.update_regexp_literal_local(local_id, expr);
                if self.captures.env_cell_locals.contains(&local_id) {
                    Ok(LoweredStmt::Expr(
                        LoweredExpr::EnvCellSet {
                            cell: local_id,
                            expr: Box::new(lowered),

                            span: Span::generated("env_cell_set"),
                        },
                        Span::generated("expr_stmt"),
                    ))
                } else {
                    Ok(LoweredStmt::Assign(
                        local_id,
                        lowered,
                        Span::generated("assign"),
                    ))
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
                    && self.facts.array_locals.contains(&local_id)
                {
                    return Ok(LoweredStmt::Assign(
                        local_id,
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeIntrinsic::ArrayPushGrow,
                            args: vec![
                                LoweredExpr::Local(local_id, Span::generated("local")),
                                self.lower_expr(&args[0])?,
                            ],

                            span: Span::generated("runtime_call"),
                        },
                        Span::generated("assign_stmt"),
                    ));
                }
                Ok(LoweredStmt::Expr(
                    self.lower_expr(expr)?,
                    Span::generated("expr_stmt"),
                ))
            }
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let condition = self.lower_expr(condition)?;
                let incoming_bigint_locals = self.facts.bigint_locals.clone();
                let incoming_div_rem_locals = self.facts.control_flow_bigint_div_rem_locals.clone();
                let incoming_mixed_locals = self.facts.control_flow_mixed_bigint_locals.clone();

                let then_body = self.lower_nested_block(then_body)?;
                let then_add_sub_bigint_locals = self.facts.bigint_locals.clone();
                let then_div_rem_bigint_locals = self.bigint_div_rem_candidate_locals();
                let then_mixed_locals = self.facts.control_flow_mixed_bigint_locals.clone();

                self.facts.bigint_locals = incoming_bigint_locals.clone();
                self.facts.control_flow_bigint_div_rem_locals = incoming_div_rem_locals.clone();
                self.facts.control_flow_mixed_bigint_locals = incoming_mixed_locals.clone();

                let else_body = self.lower_nested_block(else_body)?;
                let else_add_sub_bigint_locals = self.facts.bigint_locals.clone();
                let else_div_rem_bigint_locals = self.bigint_div_rem_candidate_locals();
                let else_mixed_locals = self.facts.control_flow_mixed_bigint_locals.clone();

                self.facts.bigint_locals = then_add_sub_bigint_locals
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

                self.facts.control_flow_bigint_div_rem_locals = definite_div_rem_locals
                    .difference(&self.facts.bigint_locals)
                    .copied()
                    .collect();
                self.facts.control_flow_mixed_bigint_locals = branch_mixed_locals;
                Ok(LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,

                    span: Span::generated("if_stmt"),
                })
            }
            ResolvedStmt::While { condition, body } => Ok(LoweredStmt::While {
                condition: self.lower_expr(condition)?,
                body: self.lower_nested_block(body)?,
                span: Span::generated("while"),
            }),
            ResolvedStmt::Return(expr) => {
                if let ResolvedExpr::Ident(name) = expr
                    && let Some(closure) = self
                        .resolve_local(name)
                        .ok()
                        .and_then(|local| self.facts.arrow_locals.get(&local))
                {
                    return Ok(LoweredStmt::Return(
                        closure.to_expr(ClosureRepresentation::HeapObject),
                        Span::generated("return_stmt"),
                    ));
                }
                if let ResolvedExpr::ArrowFn {
                    params,
                    body,
                    body_stmts,
                    ..
                } = expr
                {
                    let lowered = self.lower_arrow_fn(params, body, body_stmts)?;
                    if let LoweredExpr::ArrowFn {
                        func_id, captures, ..
                    } = &lowered
                        && !captures.is_empty()
                    {
                        return Ok(LoweredStmt::Return(
                            LoweredExpr::ArrowFn {
                                func_id: *func_id,
                                captures: captures.clone(),
                                representation: ClosureRepresentation::HeapObject,
                                span: Span::generated("arrow_fn"),
                            },
                            Span::generated("return_stmt"),
                        ));
                    }
                    return Ok(LoweredStmt::Return(lowered, Span::generated("return_stmt")));
                }
                Ok(LoweredStmt::Return(
                    self.lower_expr(expr)?,
                    Span::generated("return_stmt"),
                ))
            }
            ResolvedStmt::Function {
                name, params, body, ..
            } => {
                let local_id = self.declare_local(name)?;
                if self.captures.env_cell_names.contains(name) {
                    self.captures.env_cell_locals.insert(local_id);
                }
                let closure = self.lower_nested_function(name, params, body)?;
                if let LoweredExpr::ArrowFn {
                    func_id,
                    captures,
                    representation,

                    span: _,
                } = &closure
                {
                    if matches!(representation, ClosureRepresentation::HeapObject) {
                        self.captures.heap_closure_locals.insert(local_id);
                    } else {
                        self.facts.arrow_locals.insert(
                            local_id,
                            ArrowClosure {
                                func_id: *func_id,
                                captures: captures.clone(),
                            },
                        );
                    }
                }
                self.facts.nullish_locals.remove(&local_id);
                if self.captures.env_cell_locals.contains(&local_id) {
                    Ok(LoweredStmt::Block(
                        vec![
                            LoweredStmt::Let(
                                local_id,
                                LoweredExpr::EnvCellNew(
                                    Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                                    Span::generated("env_cell_new"),
                                ),
                                Span::generated("let_stmt"),
                            ),
                            LoweredStmt::Expr(
                                LoweredExpr::EnvCellSet {
                                    cell: local_id,
                                    expr: Box::new(closure),
                                    span: Span::generated("env_cell_set"),
                                },
                                Span::generated("expr_stmt"),
                            ),
                        ],
                        Span::generated("block"),
                    ))
                } else {
                    Ok(LoweredStmt::Let(
                        local_id,
                        closure,
                        Span::generated("let_stmt"),
                    ))
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
                    span: Span::generated("try_catch"),
                })
            }
            ResolvedStmt::Throw(expr) => Ok(LoweredStmt::Throw(
                self.lower_expr(expr)?,
                Span::generated("throw_stmt"),
            )),
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
                    span: Span::generated("switch"),
                })
            }
            ResolvedStmt::DoWhile { body, condition } => Ok(LoweredStmt::DoWhile {
                body: self.lower_nested_block(body)?,
                condition: self.lower_expr(condition)?,
                span: Span::generated("do_while"),
            }),
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                self.locals.scopes.push(HashMap::new());
                let resolved_init = init
                    .as_ref()
                    .map(|s| self.lower_stmt(s))
                    .transpose()?
                    .map(Box::new);
                let resolved = LoweredStmt::For {
                    init: resolved_init,
                    condition: condition.as_ref().map(|c| self.lower_expr(c)).transpose()?,
                    update: update.as_ref().map(|u| self.lower_expr(u)).transpose()?,
                    body: self.lower_nested_block(body)?,
                    span: Span::generated("for"),
                };
                self.locals.scopes.pop();
                Ok(resolved)
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
                    span: Span::generated("for_in"),
                })
            }
            ResolvedStmt::ForOf { var, iter, body } => {
                let var_id = self.declare_local(var)?;
                // Route custom iterables through iterator protocol (Symbol.iterator)
                if self.resolved_expr_has_symbol_iterator_property(iter) {
                    return self.lower_for_of_via_iterator(var_id, iter, body);
                }
                let lowered_iter = if let ResolvedExpr::Ident(name) = iter
                    && let Ok(local_id) = self.resolve_local(name)
                {
                    let class_name = self.classes.local_classes.get(&local_id);
                    if class_name.is_some_and(|c| c == "Set") {
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeIntrinsic::SetValuesArray,
                            args: vec![LoweredExpr::Local(local_id, Span::generated("local"))],

                            span: Span::generated("runtime_call"),
                        }
                    } else if class_name.is_some_and(|c| c == "Map") {
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeIntrinsic::MapValuesArray,
                            args: vec![LoweredExpr::Local(local_id, Span::generated("local"))],
                            span: Span::generated("runtime_call"),
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
                    span: Span::generated("for_of"),
                })
            }
            ResolvedStmt::Labeled { label, body } => Ok(LoweredStmt::Labeled {
                label: label.clone(),
                body: Box::new(self.lower_stmt(body)?),
                span: Span::generated("labeled"),
            }),
            ResolvedStmt::Break { label } => Ok(LoweredStmt::Break {
                label: label.clone(),
                span: Span::generated("break"),
            }),
            ResolvedStmt::Continue { label } => Ok(LoweredStmt::Continue {
                label: label.clone(),
                span: Span::generated("continue"),
            }),
            ResolvedStmt::Export { name, expr } => Ok(LoweredStmt::Export {
                name: name.clone(),
                expr: self.lower_expr(expr)?,
                span: Span::generated("export"),
            }),
            ResolvedStmt::ModuleExportsAssign { expr } => Ok(LoweredStmt::ModuleExportsAssign {
                expr: self.lower_expr(expr)?,
                span: Span::generated("module_exports_assign"),
            }),
            ResolvedStmt::ClassDecl { .. } => Ok(LoweredStmt::Expr(
                LoweredExpr::Undefined(Span::generated("undef")),
                Span::generated("expr_stmt"),
            )),
            ResolvedStmt::Block { statements, .. } => Ok(LoweredStmt::Block(
                self.lower_block(statements)?,
                Span::generated("block"),
            )),
        }
    }

    pub(super) fn lower_class_static_private_field(
        &mut self,
        class_name: &str,
        field: &str,
        initializer: &ResolvedExpr,
    ) -> Result<LoweredStmt, Diagnostic> {
        let local_name =
            crate::builtin_resolver::static_private_field_local_name(class_name, field);
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
        let previous = self.classes.current_class.replace(class_name.to_owned());
        let result = f(self);
        self.classes.current_class = previous;
        result
    }
}

pub(crate) fn class_maps(
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

pub(crate) fn lowered_binding_default(default: &BindingDefault) -> LoweredExpr {
    match default {
        BindingDefault::Number(value) => LoweredExpr::Number(*value, Span::generated("num")),
        BindingDefault::String(value) => LoweredExpr::String(value.clone(), Span::generated("str")),
        BindingDefault::Bool(value) => LoweredExpr::Bool(*value, Span::generated("bool")),
        BindingDefault::Null => LoweredExpr::Null(Span::generated("null")),
        BindingDefault::Undefined => LoweredExpr::Undefined(Span::generated("undef")),
        BindingDefault::Object(props) => {
            let _ = props;
            LoweredExpr::Undefined(Span::generated("undef"))
        }
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
                // Push the clean name (without `...`) so capture exclusion matches
                names.push(inner.to_owned());
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

pub(crate) fn is_private_field_storage_key(key: &str) -> bool {
    key.starts_with(PRIVATE_FIELD_STORAGE_PREFIX)
}

pub(crate) fn private_storage_observable_access_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-255: private field backing storage is not accessible through ordinary property access in this private field runtime slice".to_owned(),
        span,


        phase: None,}
}

pub(crate) fn is_static_copy_safe_object_prop_value(expr: &ResolvedExpr) -> bool {
    matches!(
        expr,
        ResolvedExpr::Number(_)
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined
    )
}

pub(crate) fn is_set_prototype_property(
    object: &ResolvedExpr,
    key: &str,
    expected_key: &str,
) -> bool {
    key == expected_key && matches_set_prototype_object(object)
}

pub(crate) fn is_set_prototype_property_expr(expr: &ResolvedExpr, expected_key: &str) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    is_set_prototype_property(object, key, expected_key)
}

pub(crate) fn is_array_prototype_push_property(object: &ResolvedExpr, key: &str) -> bool {
    key == "push" && matches_array_prototype_object(object)
}

pub(crate) fn is_array_prototype_push_expr(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    is_array_prototype_push_property(object, key)
}

pub(crate) fn matches_array_prototype_object(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Array"
        )
}

pub(crate) fn matches_set_prototype_object(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Set"
        )
}

pub(crate) fn unsupported_array_map_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-270: Array.prototype.map requires callback dispatch and new array allocation semantics that are not supported in this runtime slice".to_owned(),
        span,


        phase: None,}
}

pub(crate) fn unsupported_array_sort_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-299: Array.prototype.sort is currently supported only for dense numeric arrays with comparator `(a, b) => a - b`".to_owned(),
        span,


        phase: None,}
}

pub(crate) fn is_static_date_constructor_expr(expr: &ResolvedExpr) -> bool {
    matches!(expr, ResolvedExpr::New { class_name, .. } if class_name == "Date")
}

pub(crate) fn is_invalid_date_constructor_expr(expr: &ResolvedExpr) -> bool {
    matches!(
        expr,
        ResolvedExpr::New {
            class_name,
            args,
            ..
        } if class_name == "Date"
            && (matches!(args.as_slice(), [ResolvedExpr::Object(_)])
                || matches!(args.as_slice(), [ResolvedExpr::Ident(name)] if name == "NaN"))
    )
}

pub(crate) fn is_array_prototype_map_call_receiver(object: &ResolvedExpr, method: &str) -> bool {
    method == "call" && matches_array_prototype_map_property(object)
}

pub(crate) fn is_array_from_call_receiver(object: &ResolvedExpr, method: &str) -> bool {
    method == "from" && matches!(object, ResolvedExpr::Ident(name) if name == "Array")
}

pub(crate) fn matches_array_prototype_map_property(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "map" && matches_array_prototype_property(object)
}

pub(crate) fn matches_array_prototype_property(expr: &ResolvedExpr) -> bool {
    let ResolvedExpr::PropertyAccess { object, key, .. } = expr else {
        return false;
    };
    key == "prototype"
        && matches!(
            object.as_ref(),
            ResolvedExpr::Ident(name) if name == "Array"
        )
}

pub(crate) fn is_string_split_result_expr(expr: &ResolvedExpr) -> bool {
    matches!(
        expr,
        ResolvedExpr::MethodCall { method, .. } if method == "split"
    )
}

pub(crate) fn is_identity_arrow_callback(args: &[ResolvedExpr]) -> bool {
    let [ResolvedExpr::ArrowFn { params, body, .. }] = args else {
        return false;
    };
    let [param] = params.as_slice() else {
        return false;
    };
    matches!(body.as_ref(), ResolvedExpr::Ident(name) if name == param)
}

pub(crate) fn is_number_double_arrow_callback(args: &[ResolvedExpr]) -> bool {
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

pub(crate) fn string_split_arrow_separator(args: &[ResolvedExpr]) -> Option<&ResolvedExpr> {
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

pub(crate) fn string_constructor_arrow_callback(args: &[ResolvedExpr]) -> bool {
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

pub(crate) fn unary_plus_arrow_callback(args: &[ResolvedExpr]) -> bool {
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

pub(crate) fn numeric_ascending_sort_arrow_callback(args: &[ResolvedExpr]) -> bool {
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

/// Wrapper that converts bigint_runtime_fn_name string output to RuntimeIntrinsic.
fn bigint_runtime_fn_intrinsic(name: &str) -> Option<RuntimeIntrinsic> {
    match crate::builtin_resolver::bigint_runtime_fn_name(name) {
        Some("BigIntFromValue") => Some(RuntimeIntrinsic::BigIntFromValue),
        Some("BigIntAsIntN") => Some(RuntimeIntrinsic::BigIntAsIntN),
        Some("BigIntAsUintN") => Some(RuntimeIntrinsic::BigIntAsUintN),
        _ => None,
    }
}
