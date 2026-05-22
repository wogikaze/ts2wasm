use super::super::lowered_binding_default;
use crate::binding_pattern::{ArrayBinding, BindingDefault, BindingPattern, ObjectBinding};
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use std::collections::HashSet;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::BinaryOp;

impl super::super::Resolver {
    pub(crate) fn lower_binding_pattern_declarations(
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
                    statements.extend(
                        self.lower_object_binding_declaration(binding, bindings, &value, source)?,
                    );
                }
                Ok(statements)
            }
        }
    }

    pub(crate) fn lower_array_binding_declaration(
        &mut self,
        binding: &ArrayBinding,
        value: &LoweredExpr,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let element_value = if binding.is_rest {
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArraySlice,
                args: vec![
                    value.clone(),
                    LoweredExpr::Number(binding.index as i32, Span::generated("num")),
                    LoweredExpr::GetLength(Box::new(value.clone()), Span::generated("get_length")),
                ],
                span: Span::generated("runtime_call"),
            }
        } else {
            LoweredExpr::Index {
                object: Box::new(value.clone()),
                index: Box::new(LoweredExpr::Number(
                    binding.index as i32,
                    Span::generated("num"),
                )),
                span: Span::generated("index"),
            }
        };
        let Some(name) = binding.target.identifier() else {
            if let Some(pattern) = binding.target.pattern() {
                return self.lower_nested_binding_pattern_declaration(
                    pattern,
                    element_value,
                    binding.default.as_ref(),
                );
            }
            unreachable!("binding target must be identifier or pattern");
        };
        let local_id = self.declare_local(name)?;
        if binding.is_rest {
            return Ok(vec![LoweredStmt::Let(
                local_id,
                element_value,
                Span::generated("let_stmt"),
            )]);
        }
        self.lower_binding_declaration_with_default(
            local_id,
            element_value,
            binding.default.as_ref(),
        )
    }

    pub(crate) fn lower_object_binding_declaration(
        &mut self,
        binding: &ObjectBinding,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        source: Option<&ResolvedExpr>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let property_value = if binding.computed {
            let key_raw = binding.key.trim_start_matches('[').trim_end_matches(']');
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(value.clone()),
                key: Box::new(self.lower_computed_object_binding_key_expr(key_raw)?),
                span: Span::generated("prop_get_dynamic"),
            }
        } else {
            LoweredExpr::PropertyGet {
                obj: Box::new(value.clone()),
                key: binding.key.clone(),
                span: Span::generated("prop_get"),
            }
        };
        let Some(name) = binding.target.identifier() else {
            if let Some(pattern) = binding.target.pattern() {
                return self.lower_nested_binding_pattern_declaration(
                    pattern,
                    property_value,
                    binding.default.as_ref(),
                );
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

    pub(crate) fn lower_computed_object_binding_key_expr(
        &mut self,
        key_raw: &str,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let Some(value) = computed_key_quoted_string_value(key_raw) {
            return Ok(LoweredExpr::String(
                value.to_owned(),
                Span::generated("string"),
            ));
        }
        if let Some(value) = computed_key_string_value(key_raw) {
            return Ok(LoweredExpr::String(
                value.to_owned(),
                Span::generated("string"),
            ));
        }
        if let Some(callee) = computed_key_call_name(key_raw) {
            return self.lower_expr(&ResolvedExpr::Call {
                callee: Box::new(ResolvedExpr::Ident(callee.to_owned())),
                args: Vec::new(),
                span: Span::generated("call"),
            });
        }
        let key_name = computed_key_identifier_name(key_raw).unwrap_or(key_raw);
        self.lower_expr(&ResolvedExpr::Ident(key_name.to_owned()))
    }

    fn lower_nested_binding_pattern_declaration(
        &mut self,
        pattern: &BindingPattern,
        value: LoweredExpr,
        default: Option<&BindingDefault>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let Some(default) = default else {
            return self.lower_binding_pattern_declarations(pattern, value, None);
        };
        let temp_id = self.alloc_temp();
        let mut statements = vec![
            LoweredStmt::Let(temp_id, value, Span::generated("let_stmt")),
            LoweredStmt::If {
                condition: LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(temp_id, Span::generated("local"))),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                    span: Span::generated("binary"),
                },
                then_body: vec![LoweredStmt::Assign(
                    temp_id,
                    self.lower_binding_default_expr(default)?,
                    Span::generated("assign"),
                )],
                else_body: vec![],
                span: Span::generated("If"),
            },
        ];
        statements.extend(self.lower_binding_pattern_declarations(
            pattern,
            LoweredExpr::Local(temp_id, Span::generated("local")),
            None,
        )?);
        Ok(statements)
    }

    pub(crate) fn lower_object_rest_binding_declaration(
        &mut self,
        local_id: LocalId,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        source: Option<&ResolvedExpr>,
        _span: Option<Span>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        // Fast path: when the source is a static object literal, enumerate its
        // properties at compile time and build the rest object directly.
        if let Some(ResolvedExpr::Object(props)) = source {
            let excluded_keys = siblings
                .iter()
                .filter(|binding| !binding.is_rest)
                .map(|binding| binding.key.as_str())
                .collect::<HashSet<_>>();
            let rest_props = props
                .iter()
                .filter_map(|prop| prop.static_key())
                .filter(|key| !excluded_keys.contains(key))
                .map(|key| {
                    (
                        key.to_owned(),
                        LoweredExpr::PropertyGet {
                            obj: Box::new(value.clone()),
                            key: key.to_owned(),
                            span: Span::generated("prop_get"),
                        },
                    )
                })
                .collect();
            return Ok(vec![LoweredStmt::Let(
                local_id,
                LoweredExpr::ObjectNew {
                    props: rest_props,
                    non_enumerable: 0,
                    span: Span::generated("object_new"),
                },
                Span::generated("let_stmt"),
            )]);
        }

        // Dynamic path: copy via ObjectAssign({}, source), then delete excluded keys.
        let rest_temp = self.alloc_temp();
        let mut statements = vec![LoweredStmt::Let(
            rest_temp,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ObjectAssign,
                args: vec![
                    LoweredExpr::ObjectNew {
                        props: Vec::new(),
                        non_enumerable: 0,
                        span: Span::generated("object_rest_empty"),
                    },
                    value.clone(),
                ],
                span: Span::generated("object_rest_assign"),
            },
            Span::generated("let_stmt"),
        )];
        for sibling in siblings.iter().filter(|sibling| !sibling.is_rest) {
            let rest_object = LoweredExpr::Local(rest_temp, Span::generated("object_rest_local"));
            let delete_expr = if sibling.computed {
                let key_raw = sibling.key.trim_start_matches('[').trim_end_matches(']');
                LoweredExpr::PropertyDeleteDynamic {
                    object: Box::new(rest_object),
                    key: Box::new(self.lower_computed_object_binding_key_expr(key_raw)?),
                    span: Span::generated("object_rest_delete_dynamic"),
                }
            } else {
                LoweredExpr::PropertyDelete {
                    object: Box::new(rest_object),
                    key: sibling.key.clone(),
                    span: Span::generated("object_rest_delete"),
                }
            };
            statements.push(LoweredStmt::Expr(delete_expr, Span::generated("expr_stmt")));
        }
        statements.push(LoweredStmt::Let(
            local_id,
            LoweredExpr::Local(rest_temp, Span::generated("object_rest_local")),
            Span::generated("let_stmt"),
        ));
        Ok(statements)
    }

    pub(crate) fn lower_binding_declaration_with_default(
        &mut self,
        local_id: LocalId,
        value: LoweredExpr,
        default: Option<&BindingDefault>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let Some(default) = default else {
            return Ok(vec![LoweredStmt::Let(
                local_id,
                value,
                Span::generated("let_stmt"),
            )]);
        };
        let temp_id = self.alloc_temp();
        Ok(vec![
            LoweredStmt::Let(temp_id, value, Span::generated("let_stmt")),
            LoweredStmt::Let(
                local_id,
                LoweredExpr::Local(temp_id, Span::generated("local")),
                Span::generated("let"),
            ),
            LoweredStmt::If {
                condition: LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(temp_id, Span::generated("local"))),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                    span: Span::generated("binary"),
                },
                then_body: vec![LoweredStmt::Assign(
                    local_id,
                    self.lower_binding_default_expr(default)?,
                    Span::generated("assign"),
                )],
                else_body: vec![],
                span: Span::generated("If"),
            },
        ])
    }

    pub(crate) fn lower_binding_default_expr(
        &mut self,
        default: &BindingDefault,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let Some(expr) = lowered_binding_default(default) {
            return Ok(expr);
        }
        match default {
            BindingDefault::Ident(name) => self.lower_identifier_binding_default_expr(name),
            BindingDefault::FunctionExpr { name, is_generator } => {
                self.lower_expr(&ResolvedExpr::FunctionExpr {
                    name: name.clone(),
                    params: Vec::new(),
                    body: Vec::new(),
                    is_generator: *is_generator,
                    origin: ts2wasm_syntax::FunctionExprOrigin::User,
                    constructor_metadata: None,
                    source_text: String::new(),
                })
            }
            BindingDefault::ArrowFn => self.lower_expr(&ResolvedExpr::ArrowFn {
                params: Vec::new(),
                body: Box::new(ResolvedExpr::Undefined),
                body_stmts: Vec::new(),
                source_text: String::new(),
            }),
            BindingDefault::ClassExpr { name } => self.lower_expr(&ResolvedExpr::ClassExpr {
                name: name.clone(),
                body: Vec::new(),
            }),
            BindingDefault::Call(callee) => self.lower_expr(&ResolvedExpr::Call {
                callee: Box::new(ResolvedExpr::Ident(callee.clone())),
                args: Vec::new(),
                span: Span::generated("call"),
            }),
            BindingDefault::PreIncrement(name) => self.lower_expr(&ResolvedExpr::Assign {
                name: name.clone(),
                expr: Box::new(ResolvedExpr::Binary {
                    left: Box::new(ResolvedExpr::Ident(name.clone())),
                    op: BinaryOp::Add,
                    right: Box::new(ResolvedExpr::Number(1)),
                }),
            }),
            BindingDefault::FunctionIife {
                increment,
                return_ident,
                throw_error,
            } => self.lower_function_iife_binding_default_expr(
                increment.as_deref(),
                return_ident.as_deref(),
                throw_error.as_deref(),
            ),
            BindingDefault::Array(elements) => Ok(LoweredExpr::ArrayNew {
                elements: elements
                    .iter()
                    .map(|element| {
                        if let Some(element) = element.as_ref() {
                            self.lower_binding_default_expr(element)
                        } else {
                            Ok(LoweredExpr::Undefined(Span::generated("undef")))
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                span: Span::generated("array_new"),
            }),
            BindingDefault::Object(props) => Ok(LoweredExpr::ObjectNew {
                props: props
                    .iter()
                    .map(|(key, value)| {
                        self.lower_binding_default_expr(value)
                            .map(|value| (key.clone(), value))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                non_enumerable: 0,
                span: Span::generated("object_new"),
            }),
            BindingDefault::Number(_)
            | BindingDefault::String(_)
            | BindingDefault::Bool(_)
            | BindingDefault::Null
            | BindingDefault::Undefined => {
                unreachable!("lowered_binding_default covers literal defaults")
            }
        }
    }

    fn lower_function_iife_binding_default_expr(
        &mut self,
        increment: Option<&str>,
        return_ident: Option<&str>,
        throw_error: Option<&str>,
    ) -> Result<LoweredExpr, Diagnostic> {
        let mut stmts = Vec::new();
        if let Some(name) = increment {
            stmts.push(LoweredStmt::Expr(
                self.lower_expr(&ResolvedExpr::Assign {
                    name: name.to_owned(),
                    expr: Box::new(ResolvedExpr::Binary {
                        left: Box::new(ResolvedExpr::Ident(name.to_owned())),
                        op: BinaryOp::Add,
                        right: Box::new(ResolvedExpr::Number(1)),
                    }),
                })?,
                Span::generated("expr_stmt"),
            ));
        }
        if let Some(name) = throw_error {
            let constructor =
                BuiltinErrorConstructor::from_name(name).unwrap_or(BuiltinErrorConstructor::Error);
            stmts.push(LoweredStmt::Throw(
                LoweredExpr::ErrorNew {
                    constructor,
                    message: Box::new(LoweredExpr::String(name.to_owned(), Span::generated("str"))),
                    cause: None,
                    errors: None,
                    span: Span::generated("error_new"),
                },
                Span::generated("throw"),
            ));
            return Ok(LoweredExpr::Block {
                stmts,
                result: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                span: Span::generated("block"),
            });
        }
        let result = if let Some(name) = return_ident {
            self.lower_expr(&ResolvedExpr::Ident(name.to_owned()))?
        } else {
            LoweredExpr::Undefined(Span::generated("undef"))
        };
        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(result),
            span: Span::generated("block"),
        })
    }

    pub(crate) fn lower_binding_pattern_assignments(
        &mut self,
        pattern: &BindingPattern,
        value: LoweredExpr,
        source: Option<&ResolvedExpr>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        match pattern {
            BindingPattern::Array(bindings) => {
                let mut statements = Vec::new();
                for binding in bindings {
                    statements.extend(self.lower_array_binding_assignment(binding, &value)?);
                }
                Ok(statements)
            }
            BindingPattern::Object(bindings) => {
                let mut statements = Vec::new();
                for binding in bindings {
                    statements.extend(self.lower_object_binding_assignment(
                        binding,
                        bindings,
                        &value,
                        source,
                    )?);
                }
                Ok(statements)
            }
        }
    }

    pub(crate) fn lower_array_binding_assignment(
        &mut self,
        binding: &ArrayBinding,
        value: &LoweredExpr,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let element_value = if binding.is_rest {
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArraySlice,
                args: vec![
                    value.clone(),
                    LoweredExpr::Number(binding.index as i32, Span::generated("num")),
                    LoweredExpr::GetLength(Box::new(value.clone()), Span::generated("get_length")),
                ],
                span: Span::generated("runtime_call"),
            }
        } else {
            LoweredExpr::Index {
                object: Box::new(value.clone()),
                index: Box::new(LoweredExpr::Number(
                    binding.index as i32,
                    Span::generated("num"),
                )),
                span: Span::generated("index"),
            }
        };
        let Some(name) = binding.target.identifier() else {
            if let Some(pattern) = binding.target.pattern() {
                return self.lower_nested_binding_pattern_assignment(
                    pattern,
                    element_value,
                    binding.default.as_ref(),
                );
            }
            unreachable!("binding target must be identifier or pattern");
        };
        let local_id = self.resolve_local(name)?;
        if binding.is_rest {
            return Ok(vec![LoweredStmt::Assign(
                local_id,
                element_value,
                Span::generated("assign"),
            )]);
        }
        self.lower_binding_assignment_with_default(
            local_id,
            element_value,
            binding.default.as_ref(),
        )
    }

    pub(crate) fn lower_object_binding_assignment(
        &mut self,
        binding: &ObjectBinding,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        source: Option<&ResolvedExpr>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let property_value = if binding.computed {
            let key_raw = binding.key.trim_start_matches('[').trim_end_matches(']');
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(value.clone()),
                key: Box::new(self.lower_computed_object_binding_key_expr(key_raw)?),
                span: Span::generated("prop_get_dynamic"),
            }
        } else {
            LoweredExpr::PropertyGet {
                obj: Box::new(value.clone()),
                key: binding.key.clone(),
                span: Span::generated("prop_get"),
            }
        };
        let Some(name) = binding.target.identifier() else {
            if let Some(pattern) = binding.target.pattern() {
                return self.lower_nested_binding_pattern_assignment(
                    pattern,
                    property_value,
                    binding.default.as_ref(),
                );
            }
            unreachable!("binding target must be identifier or pattern");
        };
        let local_id = self.resolve_local(name)?;
        if binding.is_rest {
            return self.lower_object_rest_binding_assignment_with_local(
                local_id,
                siblings,
                value,
                source,
                binding.span,
            );
        }
        self.lower_binding_assignment_with_default(
            local_id,
            property_value,
            binding.default.as_ref(),
        )
    }

    fn lower_nested_binding_pattern_assignment(
        &mut self,
        pattern: &BindingPattern,
        value: LoweredExpr,
        default: Option<&BindingDefault>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let Some(default) = default else {
            return self.lower_binding_pattern_assignments(pattern, value, None);
        };
        let temp_id = self.alloc_temp();
        let mut statements = vec![
            LoweredStmt::Let(temp_id, value, Span::generated("let_stmt")),
            LoweredStmt::If {
                condition: LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(temp_id, Span::generated("local"))),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                    span: Span::generated("binary"),
                },
                then_body: vec![LoweredStmt::Assign(
                    temp_id,
                    self.lower_binding_default_expr(default)?,
                    Span::generated("assign"),
                )],
                else_body: vec![],
                span: Span::generated("If"),
            },
        ];
        statements.extend(self.lower_binding_pattern_assignments(
            pattern,
            LoweredExpr::Local(temp_id, Span::generated("local")),
            None,
        )?);
        Ok(statements)
    }

    pub(crate) fn lower_binding_assignment_with_default(
        &mut self,
        local_id: LocalId,
        value: LoweredExpr,
        default: Option<&BindingDefault>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let Some(default) = default else {
            return Ok(vec![LoweredStmt::Assign(
                local_id,
                value,
                Span::generated("assign"),
            )]);
        };
        let temp_id = self.alloc_temp();
        Ok(vec![
            LoweredStmt::Let(temp_id, value, Span::generated("let_stmt")),
            LoweredStmt::Assign(
                local_id,
                LoweredExpr::Local(temp_id, Span::generated("local")),
                Span::generated("assign"),
            ),
            LoweredStmt::If {
                condition: LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(temp_id, Span::generated("local"))),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                    span: Span::generated("binary"),
                },
                then_body: vec![LoweredStmt::Assign(
                    local_id,
                    self.lower_binding_default_expr(default)?,
                    Span::generated("assign"),
                )],
                else_body: vec![],
                span: Span::generated("If"),
            },
        ])
    }

    pub(crate) fn lower_object_rest_binding_assignment_with_local(
        &mut self,
        local_id: LocalId,
        siblings: &[ObjectBinding],
        value: &LoweredExpr,
        source: Option<&ResolvedExpr>,
        _span: Option<Span>,
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        // Fast path: when the source is a static object literal, enumerate its
        // properties at compile time and build the rest object directly.
        if let Some(ResolvedExpr::Object(props)) = source {
            let excluded_keys = siblings
                .iter()
                .filter(|binding| !binding.is_rest)
                .map(|binding| binding.key.as_str())
                .collect::<HashSet<_>>();
            let rest_props = props
                .iter()
                .filter_map(|prop| prop.static_key())
                .filter(|key| !excluded_keys.contains(key))
                .map(|key| {
                    (
                        key.to_owned(),
                        LoweredExpr::PropertyGet {
                            obj: Box::new(value.clone()),
                            key: key.to_owned(),
                            span: Span::generated("prop_get"),
                        },
                    )
                })
                .collect();
            return Ok(vec![LoweredStmt::Assign(
                local_id,
                LoweredExpr::ObjectNew {
                    props: rest_props,
                    non_enumerable: 0,
                    span: Span::generated("object_new"),
                },
                Span::generated("assign"),
            )]);
        }

        // Dynamic path: copy via ObjectAssign({}, source), then delete excluded keys.
        let rest_temp = self.alloc_temp();
        let mut statements = vec![LoweredStmt::Let(
            rest_temp,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ObjectAssign,
                args: vec![
                    LoweredExpr::ObjectNew {
                        props: Vec::new(),
                        non_enumerable: 0,
                        span: Span::generated("object_rest_empty"),
                    },
                    value.clone(),
                ],
                span: Span::generated("object_rest_assign"),
            },
            Span::generated("let_stmt"),
        )];
        for sibling in siblings.iter().filter(|sibling| !sibling.is_rest) {
            let rest_object = LoweredExpr::Local(rest_temp, Span::generated("object_rest_local"));
            let delete_expr = if sibling.computed {
                let key_raw = sibling.key.trim_start_matches('[').trim_end_matches(']');
                LoweredExpr::PropertyDeleteDynamic {
                    object: Box::new(rest_object),
                    key: Box::new(self.lower_computed_object_binding_key_expr(key_raw)?),
                    span: Span::generated("object_rest_delete_dynamic"),
                }
            } else {
                LoweredExpr::PropertyDelete {
                    object: Box::new(rest_object),
                    key: sibling.key.clone(),
                    span: Span::generated("object_rest_delete"),
                }
            };
            statements.push(LoweredStmt::Expr(
                delete_expr,
                Span::generated("expr_stmt"),
            ));
        }
        statements.push(LoweredStmt::Assign(
            local_id,
            LoweredExpr::Local(rest_temp, Span::generated("object_rest_local")),
            Span::generated("assign"),
        ));
        Ok(statements)
    }

    fn lower_identifier_binding_default_expr(
        &mut self,
        name: &str,
    ) -> Result<LoweredExpr, Diagnostic> {
        match self.lower_expr(&ResolvedExpr::Ident(name.to_owned())) {
            Ok(expr) => Ok(expr),
            Err(err) if err.code == DiagCode::UnresolvedName => Ok(LoweredExpr::Block {
                stmts: vec![LoweredStmt::Throw(
                    LoweredExpr::ErrorNew {
                        constructor: BuiltinErrorConstructor::ReferenceError,
                        message: Box::new(LoweredExpr::String(
                            format!("{name} is not defined"),
                            Span::generated("str"),
                        )),
                        cause: None,
                        errors: None,
                        span: Span::generated("error_new"),
                    },
                    Span::generated("throw"),
                )],
                result: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                span: Span::generated("block"),
            }),
            Err(err) => Err(err),
        }
    }
}

fn computed_key_call_name(key_raw: &str) -> Option<&str> {
    if let Some(callee) = key_raw.strip_suffix("()")
        && is_binding_key_identifier(callee)
    {
        return Some(callee);
    }
    if key_raw.contains("Call") {
        return computed_key_identifier_name(key_raw);
    }
    None
}

fn computed_key_identifier_name(key_raw: &str) -> Option<&str> {
    let start = key_raw.find("name: \"")?;
    let after_start = &key_raw[start + 7..];
    let end = after_start.find('"')?;
    Some(&after_start[..end])
}

fn computed_key_string_value(key_raw: &str) -> Option<&str> {
    let start = key_raw.find("value: \"")?;
    let after_start = &key_raw[start + 8..];
    let end = after_start.find('"')?;
    Some(&after_start[..end])
}

fn computed_key_quoted_string_value(key_raw: &str) -> Option<&str> {
    key_raw
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
}

fn is_binding_key_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first == '_' || first == '$' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch == '$' || ch.is_ascii_alphanumeric())
}
