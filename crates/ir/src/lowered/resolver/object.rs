use crate::builtin_resolved::{ResolvedExpr, ResolvedObjectProp};
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
                    span: None,

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
                        span: None,

                        phase: None,}
                })?;
                lowered.extend(self.lower_object_literal_props(&spread_props)?);
                continue;
            }
            if self.is_function_identifier(value) {
                continue;
            }
            lowered.push((key.to_owned(), self.lower_expr(value)?));
        }
        Ok(lowered)
    }

    pub(super) fn lower_object_literal_expr(
        &mut self,
        props: &[ResolvedObjectProp],
    ) -> Result<LoweredExpr, Diagnostic> {
        if props.iter().any(|prop| prop.computed_key().is_some()) {
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
            pending.push((key.to_owned(), self.lower_expr(value)?));
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
                        self.lower_expr(key)?,
                        self.lower_expr(value)?,
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
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message:
                                "issue-274: object literal spread is only supported for object literals and known static object-literal locals in this milestone"
                                    .to_owned(),
                            span: None,

                            phase: None,
                        });
                    }
                    if self.is_function_identifier(value) {
                        continue;
                    }
                    if initialized {
                        let set_expr = object_kernel::ordinary_set(
                            LoweredExpr::Local(object_local, Span::generated("local")),
                            key,
                            self.lower_expr(value)?,
                            Span::generated("property_set"),
                        );
                        stmts.push(LoweredStmt::Expr(set_expr, Span::generated("property_set")));
                    } else {
                        pending.push((key.to_owned(), self.lower_expr(value)?));
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
