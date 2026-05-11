use super::super::{
    is_static_copy_safe_object_prop_value, lowered_binding_default,
};
use crate::binding_pattern::{ArrayBinding, BindingDefault, BindingPattern, ObjectBinding};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedStmt};
use crate::lowered::*;
use std::collections::HashSet;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

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
                    LoweredExpr::GetLength(
                        Box::new(value.clone()),
                        Span::generated("get_length"),
                    ),
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
                return self.lower_binding_pattern_declarations(pattern, element_value, None);
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
            let key_name = if let Some(start) = key_raw.find("name: \"") {
                let after_start = &key_raw[start + 7..];
                if let Some(end) = after_start.find('\"') {
                    &after_start[..end]
                } else {
                    key_raw
                }
            } else {
                key_raw
            };
            let key_local = self.resolve_local(key_name)?;
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(value.clone()),
                key: Box::new(LoweredExpr::Local(key_local, Span::generated("local"))),
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
        self.lower_binding_declaration_with_default(local_id, property_value, binding.default.as_ref())
    }

    pub(crate) fn lower_object_rest_binding_declaration(
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
                message:
                    "issue-251: object rest binding currently requires a static object literal source in this runtime slice"
                        .to_owned(),
                span,
                phase: None,
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
                        span: Span::generated("prop_get"),
                    },
                )
            })
            .collect();
        Ok(vec![LoweredStmt::Let(
            local_id,
            LoweredExpr::ObjectNew {
                props: rest_props,
                non_enumerable: 0,
                span: Span::generated("object_new"),
            },
            Span::generated("let_stmt"),
        )])
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
                    lowered_binding_default(default),
                    Span::generated("assign"),
                )],
                else_body: vec![],
                span: Span::generated("If"),
            },
        ])
    }
}
