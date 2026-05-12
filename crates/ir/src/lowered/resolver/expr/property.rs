use super::super::{
    is_array_prototype_push_property, is_private_field_storage_key, is_set_prototype_property,
    private_storage_observable_access_diagnostic,
};
use super::{
    is_global_builtin_function_name, lower_global_builtin_function_metadata_property,
};
use crate::builtin::BuiltinPropertyId;
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::object_kernel;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_builtin_property_expr(
        &mut self,
        builtin: BuiltinPropertyId,
        object: &ResolvedExpr,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        match builtin {
            BuiltinPropertyId::Length => match object {
                ResolvedExpr::Ident(name) if self.resolve_func(name.as_str()).is_ok() => {
                    self.lower_function_metadata_property(name.as_str(), "length", span)
                }
                ResolvedExpr::Ident(name) if is_global_builtin_function_name(name) => {
                    lower_global_builtin_function_metadata_property(name, "length")
                }
                _ => Ok(LoweredExpr::GetLength(
                    Box::new(self.lower_expr(object)?),
                    Span::generated("get_length"),
                )),
            },
        }
    }

    pub(super) fn lower_property_access_expr(
        &mut self,
        object: &ResolvedExpr,
        key: &str,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if is_private_field_storage_key(key) {
            return Err(private_storage_observable_access_diagnostic(Some(span)));
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "super") {
            return self.lower_super_property_get(object, key, span);
        }
        if is_array_prototype_push_property(object, key) {
            return Ok(LoweredExpr::Number(0, Span::generated("num")));
        }
        if key.starts_with('#') {
            return self.lower_private_field_get(object, key, span);
        }
        if let ResolvedExpr::Ident(name) = object
            && self.resolve_func(name.as_str()).is_ok()
        {
            return self.lower_function_metadata_property(name.as_str(), key, span);
        }
        if let ResolvedExpr::Ident(name) = object
            && is_global_builtin_function_name(name)
            && matches!(key, "name" | "length")
        {
            return lower_global_builtin_function_metadata_property(name, key);
        }
        if key == "size" {
            if let Some(result) = self.lower_collection_size(object)? {
                return Ok(result);
            }
        }
        if is_set_prototype_property(object, key, "add") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeAddGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        Ok(object_kernel::ordinary_get(
            self.lower_expr(object)?,
            key,
            span,
        ))
    }

    pub(super) fn lower_optional_property_access_expr(
        &mut self,
        object: &ResolvedExpr,
        key: &str,
    ) -> Result<LoweredExpr, Diagnostic> {
        if is_private_field_storage_key(key) {
            return Err(private_storage_observable_access_diagnostic(None));
        }
        Ok(object_kernel::ordinary_get_optional(
            self.lower_expr(object)?,
            key,
            Span::generated("opt_prop_get"),
        ))
    }

    pub(super) fn lower_optional_computed_index_expr(
        &mut self,
        object: &ResolvedExpr,
        index: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if self.expr_has_private_progress_storage(object) {
            return Err(private_storage_observable_access_diagnostic(None));
        }
        Ok(LoweredExpr::OptionalIndex {
            object: Box::new(self.lower_expr(object)?),
            index: Box::new(self.lower_expr(index)?),
            span: Span::generated("opt_index"),
        })
    }

    pub(super) fn lower_computed_index_expr(
        &mut self,
        object: &ResolvedExpr,
        index: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if self.expr_has_private_progress_storage(object) {
            return Err(private_storage_observable_access_diagnostic(None));
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "super") {
            return self.lower_super_computed_index(object, index);
        }
        let lowered_object = self.lower_expr(object)?;
        let lowered_index = self.lower_expr(index)?;

        if matches!(object, ResolvedExpr::String(_)) {
            Ok(object_kernel::ordinary_get_dynamic(
                lowered_object,
                lowered_index,
                Span::generated("index"),
            ))
        } else if matches!(object, ResolvedExpr::Array(_))
            || matches!(
                lowered_object,
                LoweredExpr::ArrayNew { .. } | LoweredExpr::ArrayNewSparse { .. }
            )
        {
            Ok(LoweredExpr::ArrayGet {
                arr: Box::new(lowered_object),
                index: Box::new(lowered_index),
                span: Span::generated("array_get"),
            })
        } else {
            Ok(object_kernel::ordinary_get_dynamic(
                lowered_object,
                lowered_index,
                Span::generated("index"),
            ))
        }
    }

    fn lower_super_property_get(
        &mut self,
        _object: &ResolvedExpr,
        key: &str,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let class_name = self.ctx.classes.current_class.as_ref().ok_or_else(|| {
            Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super property access requires class context".to_owned(),
                span: Some(span),
                phase: None,
            }
        })?;
        let parent_name = self
            .ctx
            .classes
            .class_parents
            .get(class_name)
            .and_then(|p| p.clone())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super property access used in class without extends".to_owned(),
                span: Some(span),
                phase: None,
            })?;
        let parent_ref = self.class_prototype_ref(&parent_name)?;
        Ok(object_kernel::ordinary_get(
            LoweredExpr::ClassPrototype(parent_ref, Span::generated("class_proto")),
            key,
            span,
        ))
    }

    fn lower_super_computed_index(
        &mut self,
        _object: &ResolvedExpr,
        index: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        let class_name = self.ctx.classes.current_class.as_ref().ok_or_else(|| {
            Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super computed access requires class context".to_owned(),
                span: None,
                phase: None,
            }
        })?;
        let parent_name = self
            .ctx
            .classes
            .class_parents
            .get(class_name)
            .and_then(|p| p.clone())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super computed access used in class without extends".to_owned(),
                span: None,
                phase: None,
            })?;
        let parent_ref = self.class_prototype_ref(&parent_name)?;
        Ok(object_kernel::ordinary_get_dynamic(
            LoweredExpr::ClassPrototype(parent_ref, Span::generated("class_proto")),
            self.lower_expr(index)?,
            Span::generated("super_index_get"),
        ))
    }

    fn lower_collection_size(
        &mut self,
        object: &ResolvedExpr,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        let ResolvedExpr::Ident(receiver_name) = object else {
            return Ok(None);
        };
        let obj_local = self.resolve_local(receiver_name.as_str())?;
        let class_name = self.ctx.classes.local_classes.get(&obj_local);
        match class_name.map(String::as_str) {
            Some("Set") => Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetSize,
                args: vec![LoweredExpr::Local(obj_local, Span::generated("local"))],
                span: Span::generated("runtime_call"),
            })),
            Some("Map") => Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapSize,
                args: vec![LoweredExpr::Local(obj_local, Span::generated("local"))],
                span: Span::generated("runtime_call"),
            })),
            _ => Ok(None),
        }
    }

    fn lower_private_field_get(
        &mut self,
        object: &ResolvedExpr,
        key: &str,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let Some(local_name) = self.current_static_private_field_local_name(key) {
            if self.is_same_class_static_private_receiver(object) {
                let local = self.resolve_local(&local_name).map_err(|_| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-352: static private field `{key}` cannot be accessed before its declaration in class static initialization order"
                    ),
                    span: Some(span),
                    phase: None,
                })?;
                return Ok(if self.ctx.facts.env_cell_locals.contains(&local) {
                    LoweredExpr::EnvCellGet(local, Span::generated("env_cell_get"))
                } else {
                    LoweredExpr::Local(local, Span::generated("local"))
                });
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-255: static private field `{key}` access is currently supported only as `this.{key}` inside static methods or `Class.{key}` inside the declaring class"
                ),
                span: Some(span),
                phase: None,
            });
        }
        if let Some(getter_id) = self.current_static_private_getter_id(key) {
            if self.is_same_class_static_private_receiver(object) {
                return Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(getter_id),
                    args: Vec::new(),
                    span: Span::generated("call"),
                });
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-255: static private getter `{key}` access is currently supported only as `this.{key}` inside static methods or `Class.{key}` inside the declaring class"
                ),
                span: Some(span),
                phase: None,
            });
        }
        if self.current_private_method_id(key).is_some() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-255: private method `{key}` extraction is not supported in this private method runtime slice; call it directly as `this.{key}(...)`"
                ),
                span: Some(span),
                phase: None,
            });
        }
        if let Some(getter_id) = self.current_private_getter_id(key) {
            let receiver = if matches!(object, ResolvedExpr::This { .. }) {
                LoweredExpr::Local(self.resolve_local("this")?, Span::generated("local"))
            } else {
                let class_name = self.ctx.classes.current_class.clone().ok_or_else(|| {
                    Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: private getter `{key}` access requires declaring class context"
                        ),
                        span: Some(span),
                        phase: None,
                    }
                })?;
                let brand = self.private_brand_for_class(&class_name, Some(span))?;
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::PrivateBrandCheck,
                    args: vec![
                        self.lower_expr(object)?,
                        LoweredExpr::Number(brand as i32, Span::generated("num")),
                    ],
                    span: Span::generated("runtime_call"),
                }
            };
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(getter_id),
                args: vec![receiver],
                span: Span::generated("call"),
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
                span: Some(span),
                phase: None,
            });
        }
        let (brand, slot) = self.private_field_brand_and_slot(object, key, span)?;
        Ok(LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::PrivateFieldGet,
            args: vec![
                self.lower_expr(object)?,
                LoweredExpr::Number(brand as i32, Span::generated("num")),
                LoweredExpr::Number(slot as i32, Span::generated("num")),
            ],
            span: Span::generated("runtime_call"),
        })
    }
}
