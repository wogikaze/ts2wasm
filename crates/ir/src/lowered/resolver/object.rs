use crate::builtin_resolved::{ResolvedExpr, ResolvedObjectProp};
use crate::lowered::classes::ObjectAccessorKey;
use crate::lowered::object_kernel;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::OBJECT_SPREAD_SENTINEL;

impl super::Resolver {
    pub(super) fn lower_object_literal_props(
        &mut self,
        props: &[ResolvedObjectProp],
    ) -> Result<Vec<(String, LoweredExpr)>, Diagnostic> {
        let mut lowered = Vec::new();
        for prop in props {
            let Some(key) = prop.static_key() else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-410: computed keys in statically spread object literals are not supported"
                            .to_owned(),
                    span: Some(Span::generated("issue-410")),

                    phase: None,
                });
            };
            let value = prop.value();
            if key == OBJECT_SPREAD_SENTINEL {
                let spread_props = self.static_object_literal_spread_props(value).ok_or_else(|| {
                    Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "issue-274: object literal spread is only supported for object literals and known static object-literal locals in this milestone"
                                .to_owned(),
                        span: Some(Span::generated("issue-274")),

                        phase: None,}
                })?;
                lowered.extend(self.lower_object_literal_props(&spread_props)?);
                continue;
            }
            if self.is_function_identifier(value) {
                continue;
            }
            let lowered_value = self.lower_object_method_shorthand_value(prop, value)?;
            lowered.push((key.to_owned(), lowered_value));
        }
        Ok(lowered)
    }

    fn lower_object_method_shorthand_value(
        &mut self,
        prop: &ResolvedObjectProp,
        value: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let ResolvedObjectProp::MethodShorthand { value, .. } = prop
            && let ResolvedExpr::FunctionExpr {
                name,
                params,
                body,
                is_generator,
                ..
            } = value
        {
            if is_object_literal_accessor_function_name(name) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-67ZV8S: object literal getter/setter accessors are not supported"
                            .to_owned(),
                    span: Some(Span::generated("issue-67ZV8S")),
                    phase: None,
                });
            }
            return self.lower_object_method_function_expr(name, params, body, *is_generator);
        }
        self.lower_expr(value)
    }

    fn lower_object_computed_value(
        &mut self,
        value: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let ResolvedExpr::FunctionExpr {
            name,
            params,
            body,
            is_generator,
            ..
        } = value
        {
            return self.lower_object_method_function_expr(name, params, body, *is_generator);
        }
        self.lower_expr(value)
    }

    pub(super) fn lower_object_literal_expr(
        &mut self,
        props: &[ResolvedObjectProp],
    ) -> Result<LoweredExpr, Diagnostic> {
        if props.iter().any(|prop| {
            prop.computed_key().is_some()
                || prop.static_key() == Some("__proto__")
                || object_literal_accessor_kind(prop).is_some()
        }) {
            return self.lower_object_literal_expr_with_computed_keys(props);
        }

        let mut result: Option<LoweredExpr> = None;
        let mut pending = Vec::new();

        for prop in props {
            let Some(key) = prop.static_key() else {
                unreachable!(
                    "computed keys are handled by lower_object_literal_expr_with_computed_keys"
                );
            };
            let value = prop.value();
            if key == OBJECT_SPREAD_SENTINEL {
                if let Some(spread_props) = self.static_object_literal_spread_props(value) {
                    pending.extend(self.lower_object_literal_props(&spread_props)?);
                    continue;
                }

                let target = result.take().unwrap_or_else(|| LoweredExpr::ObjectNew {
                    props: Vec::new(),
                    non_enumerable: 0,
                    span: Span::generated("object_new"),
                });
                let target = if pending.is_empty() {
                    target
                } else {
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ObjectSpread,
                        args: vec![
                            target,
                            LoweredExpr::ObjectNew {
                                props: std::mem::take(&mut pending),
                                non_enumerable: 0,

                                span: Span::generated("object_new"),
                            },
                        ],
                        span: Span::generated("RuntimeCall"),
                    }
                };
                result = Some(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ObjectSpread,
                    args: vec![target, self.lower_expr(value)?],

                    span: Span::generated("runtime_call"),
                });
                continue;
            }

            if self.is_function_identifier(value) {
                continue;
            }
            let lowered_value = self.lower_object_method_shorthand_value(prop, value)?;
            pending.push((key.to_owned(), lowered_value));
        }

        let target = result.unwrap_or_else(|| LoweredExpr::ObjectNew {
            props: Vec::new(),
            non_enumerable: 0,
            span: Span::generated("object_new"),
        });
        if pending.is_empty() {
            Ok(target)
        } else if matches!(target, LoweredExpr::ObjectNew { ref props, .. } if props.is_empty()) {
            Ok(LoweredExpr::ObjectNew {
                props: pending,
                non_enumerable: 0,
                span: Span::generated("object_new"),
            })
        } else {
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ObjectSpread,
                args: vec![
                    target,
                    LoweredExpr::ObjectNew {
                        props: pending,
                        non_enumerable: 0,
                        span: Span::generated("object_new"),
                    },
                ],
                span: Span::generated("RuntimeCall"),
            })
        }
    }

    fn lower_object_literal_expr_with_computed_keys(
        &mut self,
        props: &[ResolvedObjectProp],
    ) -> Result<LoweredExpr, Diagnostic> {
        let object_local = self.alloc_temp();
        let mut stmts = Vec::new();
        let mut pending = Vec::new();
        let mut initialized = false;

        for prop in props {
            match prop {
                ResolvedObjectProp::ComputedKey { key, value } => {
                    if let Some(kind) = object_literal_accessor_kind(prop) {
                        if !initialized {
                            stmts.push(LoweredStmt::Let(
                                object_local,
                                LoweredExpr::ObjectNew {
                                    props: std::mem::take(&mut pending),
                                    non_enumerable: 0,
                                    span: Span::generated("object_new"),
                                },
                                Span::generated("object_literal"),
                            ));
                            initialized = true;
                        }
                        let descriptor =
                            self.lower_object_literal_accessor_descriptor(prop, kind)?;
                        let define_expr = object_kernel::ordinary_define_own_property(
                            LoweredExpr::Local(object_local, Span::generated("local")),
                            self.lower_computed_property_key_expr(key)?,
                            descriptor,
                            Span::generated("object_computed_accessor_define"),
                        );
                        stmts.push(LoweredStmt::Expr(
                            define_expr,
                            Span::generated("object_computed_accessor_define"),
                        ));
                        continue;
                    }
                    if let Some(static_key) =
                        super::string::resolved_expr_static_property_key_value(&self.ctx, key)
                    {
                        let lowered_value = self.lower_object_computed_value(value)?;
                        if initialized {
                            let set_expr = object_kernel::ordinary_set(
                                LoweredExpr::Local(object_local, Span::generated("local")),
                                &static_key,
                                lowered_value,
                                Span::generated("property_set"),
                            );
                            stmts
                                .push(LoweredStmt::Expr(set_expr, Span::generated("property_set")));
                        } else {
                            pending.push((static_key, lowered_value));
                        }
                        continue;
                    }
                    if !initialized {
                        stmts.push(LoweredStmt::Let(
                            object_local,
                            LoweredExpr::ObjectNew {
                                props: std::mem::take(&mut pending),
                                non_enumerable: 0,
                                span: Span::generated("object_new"),
                            },
                            Span::generated("object_literal"),
                        ));
                        initialized = true;
                    }
                    let set_expr = object_kernel::ordinary_set_dynamic(
                        LoweredExpr::Local(object_local, Span::generated("local")),
                        self.lower_computed_property_key_expr(key)?,
                        self.lower_object_computed_value(value)?,
                        Span::generated("property_set_dynamic"),
                    );
                    stmts.push(LoweredStmt::Expr(
                        set_expr,
                        Span::generated("property_set_dynamic"),
                    ));
                }
                _ => {
                    let Some(key) = prop.static_key() else {
                        continue;
                    };
                    let value = prop.value();
                    if let Some(kind) = object_literal_accessor_kind(prop) {
                        if !initialized {
                            stmts.push(LoweredStmt::Let(
                                object_local,
                                LoweredExpr::ObjectNew {
                                    props: std::mem::take(&mut pending),
                                    non_enumerable: 0,
                                    span: Span::generated("object_new"),
                                },
                                Span::generated("object_literal"),
                            ));
                            initialized = true;
                        }
                        let descriptor =
                            self.lower_object_literal_accessor_descriptor(prop, kind)?;
                        let define_expr = object_kernel::ordinary_define_own_property(
                            LoweredExpr::Local(object_local, Span::generated("local")),
                            LoweredExpr::String(key.to_owned(), Span::generated("str")),
                            descriptor,
                            Span::generated("object_accessor_define"),
                        );
                        stmts.push(LoweredStmt::Expr(
                            define_expr,
                            Span::generated("object_accessor_define"),
                        ));
                        continue;
                    }
                    if key == "__proto__" {
                        if !initialized {
                            stmts.push(LoweredStmt::Let(
                                object_local,
                                LoweredExpr::ObjectNew {
                                    props: std::mem::take(&mut pending),
                                    non_enumerable: 0,
                                    span: Span::generated("object_new"),
                                },
                                Span::generated("object_literal"),
                            ));
                            initialized = true;
                        }
                        let set_expr = object_kernel::ordinary_set_prototype_of(
                            LoweredExpr::Local(object_local, Span::generated("local")),
                            self.lower_expr(value)?,
                            Span::generated("object_proto_set"),
                        );
                        stmts.push(LoweredStmt::Expr(
                            set_expr,
                            Span::generated("object_proto_set"),
                        ));
                        continue;
                    }
                    if key == OBJECT_SPREAD_SENTINEL {
                        if let Some(spread_props) = self.static_object_literal_spread_props(value) {
                            for spread_prop in spread_props {
                                if let Some(spread_key) = spread_prop.static_key()
                                    && spread_key != OBJECT_SPREAD_SENTINEL
                                    && !self.is_function_identifier(spread_prop.value())
                                {
                                    if initialized {
                                        let set_expr = object_kernel::ordinary_set(
                                            LoweredExpr::Local(
                                                object_local,
                                                Span::generated("local"),
                                            ),
                                            spread_key,
                                            self.lower_expr(spread_prop.value())?,
                                            Span::generated("property_set"),
                                        );
                                        stmts.push(LoweredStmt::Expr(
                                            set_expr,
                                            Span::generated("property_set"),
                                        ));
                                    } else {
                                        pending.push((
                                            spread_key.to_owned(),
                                            self.lower_expr(spread_prop.value())?,
                                        ));
                                    }
                                }
                            }
                            continue;
                        }
                        if !initialized {
                            stmts.push(LoweredStmt::Let(
                                object_local,
                                LoweredExpr::ObjectNew {
                                    props: std::mem::take(&mut pending),
                                    non_enumerable: 0,
                                    span: Span::generated("object_new"),
                                },
                                Span::generated("object_literal"),
                            ));
                            initialized = true;
                        }
                        let spread_expr = LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::ObjectSpread,
                            args: vec![
                                LoweredExpr::Local(object_local, Span::generated("local")),
                                self.lower_expr(value)?,
                            ],
                            span: Span::generated("object_spread"),
                        };
                        stmts.push(LoweredStmt::Assign(
                            object_local,
                            spread_expr,
                            Span::generated("object_spread"),
                        ));
                        continue;
                    }
                    if self.is_function_identifier(value) {
                        continue;
                    }
                    let lowered_value = self.lower_object_method_shorthand_value(prop, value)?;
                    if initialized {
                        let set_expr = object_kernel::ordinary_set(
                            LoweredExpr::Local(object_local, Span::generated("local")),
                            key,
                            lowered_value,
                            Span::generated("property_set"),
                        );
                        stmts.push(LoweredStmt::Expr(set_expr, Span::generated("property_set")));
                    } else {
                        pending.push((key.to_owned(), lowered_value));
                    }
                }
            }
        }

        if !initialized {
            return Ok(LoweredExpr::ObjectNew {
                props: pending,
                non_enumerable: 0,
                span: Span::generated("object_new"),
            });
        }

        Ok(LoweredExpr::Block {
            stmts,
            result: Box::new(LoweredExpr::Local(object_local, Span::generated("local"))),
            span: Span::generated("object_literal"),
        })
    }

    fn lower_computed_property_key_expr(
        &mut self,
        key: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let Some(expr) = self.lower_known_to_property_key_expr(key)? {
            return Ok(expr);
        }
        self.lower_expr(key)
    }

    fn lower_known_to_property_key_expr(
        &mut self,
        key: &ResolvedExpr,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        let ResolvedExpr::Ident(name) = key else {
            return Ok(None);
        };
        let Ok(obj_local) = self.resolve_local(name) else {
            return Ok(None);
        };
        let to_string_key = ObjectAccessorKey::Property("toString".to_owned());
        let Some(method_id) = self
            .ctx
            .classes
            .object_function_props
            .get(&obj_local)
            .and_then(|props| props.get(&to_string_key))
            .copied()
        else {
            return Ok(None);
        };
        let receiver = LoweredExpr::Local(obj_local, Span::generated("local"));
        let args = self.lower_function_call_args(method_id, receiver, &[])?;
        Ok(Some(LoweredExpr::Call {
            kind: FunctionCallKind::User(method_id),
            args,
            span: Span::generated("to_property_key"),
        }))
    }

    fn lower_object_literal_accessor_descriptor(
        &mut self,
        prop: &ResolvedObjectProp,
        kind: ObjectLiteralAccessorKind,
    ) -> Result<LoweredExpr, Diagnostic> {
        let value = prop.value();
        let ResolvedExpr::FunctionExpr {
            name,
            params,
            body,
            is_generator,
            ..
        } = value
        else {
            unreachable!("object literal accessor kind only matches function values");
        };
        let function = self.lower_object_method_function_expr(name, params, body, *is_generator)?;
        Ok(LoweredExpr::ObjectNew {
            props: vec![
                (kind.descriptor_key().to_owned(), function),
                (
                    "enumerable".to_owned(),
                    LoweredExpr::Bool(true, Span::generated("bool")),
                ),
                (
                    "configurable".to_owned(),
                    LoweredExpr::Bool(true, Span::generated("bool")),
                ),
            ],
            non_enumerable: 0,
            span: Span::generated("object_accessor_descriptor"),
        })
    }

    pub(super) fn static_object_literal_spread_props(
        &self,
        value: &ResolvedExpr,
    ) -> Option<Vec<ResolvedObjectProp>> {
        match value {
            ResolvedExpr::Object(spread_props) => Some(spread_props.clone()),
            ResolvedExpr::Ident(name) => {
                let local_id = self.resolve_local(name).ok()?;
                if self.ctx.facts.env_cell_locals.contains(&local_id) {
                    return None;
                }
                self.ctx
                    .facts
                    .static_object_literal_locals
                    .get(&local_id)
                    .cloned()
            }
            _ => None,
        }
    }

    pub(super) fn lower_set_prototype_add_assignment_value(
        &mut self,
        value: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let ResolvedExpr::Ident(name) = value
            && let Ok(func_id) = self.resolve_func(name)
        {
            return Ok(LoweredExpr::Number(
                func_id.0 as i32,
                Span::generated("num"),
            ));
        }
        self.lower_expr(value)
    }
}

fn is_object_literal_accessor_function_name(name: &str) -> bool {
    name.starts_with("get ") || name.starts_with("set ")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObjectLiteralAccessorKind {
    Get,
    Set,
}

impl ObjectLiteralAccessorKind {
    fn descriptor_key(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Set => "set",
        }
    }
}

fn object_literal_accessor_kind(prop: &ResolvedObjectProp) -> Option<ObjectLiteralAccessorKind> {
    let value = match prop {
        ResolvedObjectProp::MethodShorthand { value, .. }
        | ResolvedObjectProp::ComputedKey { value, .. } => value,
        _ => return None,
    };
    let ResolvedExpr::FunctionExpr { name, .. } = value else {
        return None;
    };
    if name.starts_with("get ") {
        Some(ObjectLiteralAccessorKind::Get)
    } else if name.starts_with("set ") {
        Some(ObjectLiteralAccessorKind::Set)
    } else {
        None
    }
}
