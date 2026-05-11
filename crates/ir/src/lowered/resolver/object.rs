use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_shared::{DiagCode, Diagnostic, OBJECT_SPREAD_SENTINEL, Span};

impl<'a> super::Resolver<'a> {
    pub(super) fn lower_object_literal_props(
        &mut self,
        props: &[(String, ResolvedExpr)],
    ) -> Result<Vec<(String, LoweredExpr)>, Diagnostic> {
        let mut lowered = Vec::new();
        for (key, value) in props {
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
            lowered.push((key.clone(), self.lower_expr(value)?));
        }
        Ok(lowered)
    }

    pub(super) fn lower_object_literal_expr(
        &mut self,
        props: &[(String, ResolvedExpr)],
    ) -> Result<LoweredExpr, Diagnostic> {
        let mut result: Option<LoweredExpr> = None;
        let mut pending = Vec::new();

        for (key, value) in props {
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
                        intrinsic: RuntimeIntrinsic::ObjectSpread,
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
                    intrinsic: RuntimeIntrinsic::ObjectSpread,
                    args: vec![target, self.lower_expr(value)?],

                    span: Span::generated("runtime_call"),
                });
                continue;
            }

            if self.is_function_identifier(value) {
                continue;
            }
            pending.push((key.clone(), self.lower_expr(value)?));
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
                intrinsic: RuntimeIntrinsic::ObjectSpread,
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

    pub(super) fn static_object_literal_spread_props(
        &self,
        value: &ResolvedExpr,
    ) -> Option<Vec<(String, ResolvedExpr)>> {
        match value {
            ResolvedExpr::Object(spread_props) => Some(spread_props.clone()),
            ResolvedExpr::Ident(name) => {
                let local_id = self.resolve_local(name).ok()?;
                if self.captures.env_cell_locals.contains(&local_id) {
                    return None;
                }
                self.facts
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
