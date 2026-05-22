use super::super::{
    is_map_prototype_property, is_private_field_storage_key, is_set_prototype_property,
    is_set_prototype_property_expr, private_storage_observable_access_diagnostic,
};
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::classes::ObjectAccessorKey;
use crate::lowered::object_kernel;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::LogicalAssignOp;

impl super::super::Resolver {
    pub(super) fn lower_assign_expr(
        &mut self,
        name: &str,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if self
            .ctx
            .strict_mode_check(crate::lowered::ctx::StrictModeCheck::StrictEval)
            && matches!(name, "eval" | "arguments")
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-450: {:?} strict mode forbids assigning to `{name}`",
                    crate::lowered::ctx::StrictModeCheck::StrictEval
                ),
                span: None,
                phase: None,
            });
        }
        let local = self.resolve_local(name)?;
        crate::lowered::resolver::expr::facts::invalidate_static_object_literal_local(
            &mut self.ctx,
            local,
        );
        crate::lowered::resolver::expr::facts::invalidate_static_function_array_like_local(
            &mut self.ctx,
            local,
        );
        let lowered_expr = Box::new(self.lower_expr(expr)?);
        self.ctx.facts.nullish_locals.remove(&local);
        crate::lowered::resolver::expr::facts::update_host_function_handle_local(
            &mut self.ctx,
            local,
            expr,
        );
        crate::lowered::resolver::expr::facts::update_host_external_object_local(
            &mut self.ctx,
            local,
            expr,
        );
        if self.ctx.facts.env_cell_locals.contains(&local) {
            Ok(LoweredExpr::EnvCellSet {
                cell: local,
                expr: lowered_expr,
                span: Span::generated("env_cell_set"),
            })
        } else {
            Ok(LoweredExpr::Assign {
                local,
                expr: lowered_expr,
                span: Span::generated("assign"),
            })
        }
    }

    pub(super) fn lower_logical_assign_expr(
        &mut self,
        name: &str,
        op: &LogicalAssignOp,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        let local = self.resolve_local(name)?;
        crate::lowered::resolver::expr::facts::invalidate_static_object_literal_local(
            &mut self.ctx,
            local,
        );
        crate::lowered::resolver::expr::facts::invalidate_static_function_array_like_local(
            &mut self.ctx,
            local,
        );
        Ok(LoweredExpr::LogicalAssign {
            local,
            op: lower_logical_assign_op(*op),
            expr: Box::new(self.lower_expr(expr)?),
            span: Span::generated("logical_assign"),
        })
    }

    pub(super) fn lower_logical_property_assign_expr(
        &mut self,
        object: &str,
        key: &str,
        op: &LogicalAssignOp,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if is_private_field_storage_key(key) {
            return Err(private_storage_observable_access_diagnostic(None));
        }
        let object = self.resolve_local(object)?;
        crate::lowered::resolver::expr::facts::invalidate_static_object_literal_local(
            &mut self.ctx,
            object,
        );
        Ok(LoweredExpr::LogicalPropertyAssign {
            object,
            key: key.to_owned(),
            op: lower_logical_assign_op(*op),
            expr: Box::new(self.lower_expr(expr)?),
            span: Span::generated("logical_prop_assign"),
        })
    }

    pub(super) fn lower_logical_computed_property_assign_expr(
        &mut self,
        object: &str,
        key: &ResolvedExpr,
        op: &LogicalAssignOp,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        let object = self.resolve_local(object)?;
        crate::lowered::resolver::expr::facts::invalidate_static_object_literal_local(
            &mut self.ctx,
            object,
        );
        if self.local_has_private_progress_storage(object) {
            return Err(private_storage_observable_access_diagnostic(None));
        }
        Ok(LoweredExpr::LogicalComputedPropertyAssign {
            object,
            key: Box::new(self.lower_expr(key)?),
            op: lower_logical_assign_op(*op),
            expr: Box::new(self.lower_expr(expr)?),
            span: Span::generated("logical_comp_prop_assign"),
        })
    }

    pub(super) fn lower_logical_member_assign_expr(
        &mut self,
        object: &ResolvedExpr,
        key: &str,
        op: &LogicalAssignOp,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        Ok(LoweredExpr::LogicalMemberAssign {
            object: Box::new(self.lower_expr(object)?),
            key: key.to_owned(),
            op: lower_logical_assign_op(*op),
            expr: Box::new(self.lower_expr(expr)?),
            span: Span::generated("logical_member_assign"),
        })
    }

    pub(super) fn lower_logical_computed_member_assign_expr(
        &mut self,
        object: &ResolvedExpr,
        key: &ResolvedExpr,
        op: &LogicalAssignOp,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        Ok(LoweredExpr::LogicalComputedMemberAssign {
            object: {
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Box::new(self.lower_expr(object)?)
            },
            key: Box::new(self.lower_expr(key)?),
            op: lower_logical_assign_op(*op),
            expr: Box::new(self.lower_expr(expr)?),
            span: Span::generated("logical_comp_member_assign"),
        })
    }

    pub(super) fn lower_property_assign_expr(
        &mut self,
        object: &ResolvedExpr,
        key: &str,
        value: &ResolvedExpr,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let ResolvedExpr::Ident(name) = object
            && let Ok(local_id) = self.resolve_local(name)
        {
            crate::lowered::resolver::expr::facts::invalidate_static_object_literal_local(
                &mut self.ctx,
                local_id,
            );
        }
        if is_private_field_storage_key(key) {
            return Err(private_storage_observable_access_diagnostic(Some(span)));
        }
        if is_set_prototype_property(object, key, "add") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeAddSet,
                args: vec![self.lower_set_prototype_add_assignment_value(value)?],
                span: Span::generated("runtime_call"),
            });
        }
        if is_set_prototype_property(object, key, "originalAdd")
            && is_set_prototype_property_expr(value, "add")
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeAddGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        if is_set_prototype_property(object, key, "has") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeHasSet,
                args: vec![self.lower_expr(value)?],
                span: Span::generated("runtime_call"),
            });
        }
        if is_set_prototype_property(object, key, "delete") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeDeleteSet,
                args: vec![self.lower_expr(value)?],
                span: Span::generated("runtime_call"),
            });
        }
        if is_set_prototype_property(object, key, "forEach") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeForEachSet,
                args: vec![self.lower_expr(value)?],
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "get") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeGetSet,
                args: vec![self.lower_expr(value)?],
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "set") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeSetSet,
                args: vec![self.lower_expr(value)?],
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "has") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeHasSet,
                args: vec![self.lower_expr(value)?],
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "delete") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeDeleteSet,
                args: vec![self.lower_expr(value)?],
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "forEach") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeForEachSet,
                args: vec![self.lower_expr(value)?],
                span: Span::generated("runtime_call"),
            });
        }
        if key.starts_with('#') {
            return self.lower_private_field_assign(object, key, value, span);
        }
        if let Some(proxy) =
            crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(&self.ctx, object)
        {
            return self.lower_proxy_trap_call(
                proxy,
                crate::lowered::facts::ProxyTrapKind::ProxySet,
                vec![ResolvedExpr::String(key.to_owned()), value.clone()],
                span,
            );
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "super") {
            return self.lower_super_property_assign(object, key, value, span);
        }
        if key == "__proto__" {
            return Ok(object_kernel::ordinary_set_prototype_of(
                self.lower_expr(object)?,
                self.lower_expr(value)?,
                Span::generated("object_proto_set"),
            ));
        }
        if let ResolvedExpr::Ident(name) = object
            && let Ok(obj_local) = self.resolve_local(name)
            && let Some(setter_id) = self
                .ctx
                .classes
                .object_accessor_props
                .get(&obj_local)
                .and_then(|props| props.get(&ObjectAccessorKey::Property(key.to_owned())))
                .and_then(|prop| prop.set)
        {
            let lowered_args = self.lower_function_call_args(
                setter_id,
                LoweredExpr::Local(obj_local, Span::generated("local")),
                std::slice::from_ref(value),
            )?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(setter_id),
                args: lowered_args,
                span: Span::generated("call"),
            });
        }
        let lowered_value = self.lower_expr(value)?;
        let lowered_object = self.lower_property_assignment_object(object)?;
        // Track function/arrow assignments on known locals so method calls
        // on untyped receivers (e.g. assert.sameValue()) can be dispatched
        // via object_function_props in lower_mcall_dispatch_early.
        if let ResolvedExpr::Ident(name) = object
            && let Ok(local_id) = self.resolve_local(name)
        {
            let func_id = match &lowered_value {
                LoweredExpr::ArrowFn { func_id, .. } => Some(*func_id),
                _ => None,
            };
            if let Some(fid) = func_id {
                self.ctx
                    .classes
                    .object_function_props
                    .entry(local_id)
                    .or_default()
                    .insert(ObjectAccessorKey::Property(key.to_owned()), fid);
            }
            if self
                .ctx
                .classes
                .object_accessor_props
                .get(&local_id)
                .and_then(|props| props.get(&ObjectAccessorKey::Property(key.to_owned())))
                .is_none_or(|prop| prop.set.is_none())
                && let Some(props) = self.ctx.classes.object_accessor_props.get_mut(&local_id)
            {
                props.remove(&ObjectAccessorKey::Property(key.to_owned()));
                if props.is_empty() {
                    self.ctx.classes.object_accessor_props.remove(&local_id);
                }
            }
        } else if matches!(object, ResolvedExpr::Ident(name) if name == "globalThis") {
            if let LoweredExpr::ArrowFn { func_id, .. } = &lowered_value {
                self.ctx
                    .classes
                    .global_object_function_props
                    .insert(ObjectAccessorKey::Property(key.to_owned()), *func_id);
            }
        }
        Ok(self.lower_property_set_with_null_guard(
            lowered_object,
            key,
            lowered_value,
            span,
        ))
    }

    pub(super) fn lower_property_assign_dynamic_expr(
        &mut self,
        object: &ResolvedExpr,
        key: &ResolvedExpr,
        value: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let ResolvedExpr::Ident(name) = object
            && let Ok(local_id) = self.resolve_local(name)
        {
            crate::lowered::resolver::expr::facts::invalidate_static_object_literal_local(
                &mut self.ctx,
                local_id,
            );
            crate::lowered::resolver::expr::facts::update_static_function_array_like_index(
                &mut self.ctx,
                local_id,
                key,
                value,
            );
        }
        if self.expr_has_private_progress_storage(object) {
            return Err(private_storage_observable_access_diagnostic(None));
        }
        if let Some(proxy) =
            crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(&self.ctx, object)
        {
            return self.lower_proxy_trap_call(
                proxy,
                crate::lowered::facts::ProxyTrapKind::ProxySet,
                vec![key.clone(), value.clone()],
                Span::generated("proxy_set"),
            );
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "super") {
            return self.lower_super_property_assign_dynamic(object, key, value);
        }
        if let ResolvedExpr::Ident(name) = object
            && let Ok(obj_local) = self.resolve_local(name)
            && let Some(static_key) =
                super::super::string::resolved_expr_static_accessor_key(&self.ctx, key)
            && let Some(setter_id) = self
                .ctx
                .classes
                .object_accessor_props
                .get(&obj_local)
                .and_then(|props| props.get(&static_key))
                .and_then(|prop| prop.set)
        {
            let lowered_args = self.lower_function_call_args(
                setter_id,
                LoweredExpr::Local(obj_local, Span::generated("local")),
                std::slice::from_ref(value),
            )?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(setter_id),
                args: lowered_args,
                span: Span::generated("call"),
            });
        }
        if let ResolvedExpr::Ident(name) = object
            && let Ok(obj_local) = self.resolve_local(name)
            && self
                .ctx
                .classes
                .local_classes
                .get(&obj_local)
                .is_some_and(|cn| crate::lowered::program_builtins::is_typed_array_class(cn))
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::TypedArrayStore,
                args: vec![
                    self.lower_expr(object)?,
                    self.lower_expr(key)?,
                    self.lower_expr(value)?,
                ],
                span: Span::generated("typed_array_store"),
            });
        }
        let lowered_object = self.lower_property_assignment_object(object)?;
        let lowered_key = self.lower_expr(key)?;
        let lowered_value = self.lower_expr(value)?;
        Ok(self.lower_property_set_dynamic_with_null_guard(
            lowered_object,
            lowered_key,
            lowered_value,
            Span::generated("prop_set_dyn"),
        ))
    }

    fn lower_property_assignment_object(
        &mut self,
        object: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let ResolvedExpr::PropertyAccess {
            object: prototype_base,
            key,
            ..
        } = object
            && key == "prototype"
            && let ResolvedExpr::Ident(name) = prototype_base.as_ref()
            && self.resolve_func(name).is_ok()
        {
            return Ok(LoweredExpr::ObjectNew {
                props: Vec::new(),
                non_enumerable: 0,
                span: Span::generated("function_prototype_object"),
            });
        }

        self.lower_expr(object)
    }

    fn lower_private_field_assign(
        &mut self,
        object: &ResolvedExpr,
        key: &str,
        value: &ResolvedExpr,
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
                let expr = Box::new(self.lower_expr(value)?);
                return Ok(if self.ctx.facts.env_cell_locals.contains(&local) {
                    LoweredExpr::EnvCellSet {
                        cell: local,
                        expr,
                        span: Span::generated("env_cell_set"),
                    }
                } else {
                    LoweredExpr::Assign {
                        local,
                        expr,
                        span: Span::generated("assign"),
                    }
                });
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-255: static private field `{key}` assignment is currently supported only as `this.{key} = value` inside static methods or `Class.{key} = value` inside the declaring class"
                ),
                span: Some(span),
                phase: None,
            });
        }
        if let Some(setter_id) = self.current_static_private_setter_id(key) {
            if self.is_same_class_static_private_receiver(object) {
                return Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(setter_id),
                    args: vec![self.lower_expr(value)?],
                    span: Span::generated("call"),
                });
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-255: static private setter `{key}` assignment is currently supported only as `this.{key} = value` inside static methods or `Class.{key} = value` inside the declaring class"
                ),
                span: Some(span),
                phase: None,
            });
        }
        if let Some(setter_id) = self.current_private_setter_id(key) {
            let receiver = if matches!(object, ResolvedExpr::This { .. }) {
                LoweredExpr::Local(self.resolve_local("this")?, Span::generated("local"))
            } else {
                let class_name = self.ctx.classes.current_class.clone().ok_or_else(|| {
                    Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: private setter `{key}` assignment requires declaring class context"
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
                kind: FunctionCallKind::User(setter_id),
                args: vec![receiver, self.lower_expr(value)?],
                span: Span::generated("call"),
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
                span: Some(span),
                phase: None,
            });
        }
        let (brand, slot) = self.private_field_brand_and_slot(object, key, span)?;
        Ok(LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::PrivateFieldSet,
            args: vec![
                self.lower_expr(object)?,
                LoweredExpr::Number(brand as i32, Span::generated("num")),
                LoweredExpr::Number(slot as i32, Span::generated("num")),
                self.lower_expr(value)?,
            ],
            span: Span::generated("runtime_call"),
        })
    }

    fn lower_super_property_assign(
        &mut self,
        _object: &ResolvedExpr,
        key: &str,
        value: &ResolvedExpr,
        _span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let class_name = self
            .ctx
            .classes
            .current_class
            .as_ref()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super property assignment requires class context".to_owned(),
                span: Some(Span::generated("super-assign")),
                phase: None,
            })?;
        let _parent_name = self
            .ctx
            .classes
            .class_parents
            .get(class_name)
            .and_then(|p| p.clone())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super property assignment used in class without extends".to_owned(),
                span: Some(Span::generated("super-assign")),
                phase: None,
            })?;
        Ok(object_kernel::ordinary_set(
            LoweredExpr::Local(self.resolve_local("this")?, Span::generated("local")),
            key,
            self.lower_expr(value)?,
            Span::generated("super_prop_set"),
        ))
    }

    fn lower_super_property_assign_dynamic(
        &mut self,
        _object: &ResolvedExpr,
        key: &ResolvedExpr,
        value: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        let class_name = self
            .ctx
            .classes
            .current_class
            .as_ref()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super computed assignment requires class context".to_owned(),
                span: Some(Span::generated("super-computed")),
                phase: None,
            })?;
        let _parent_name = self
            .ctx
            .classes
            .class_parents
            .get(class_name)
            .and_then(|p| p.clone())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super computed assignment used in class without extends".to_owned(),
                span: Some(Span::generated("super-computed")),
                phase: None,
            })?;
        Ok(object_kernel::ordinary_set_dynamic(
            LoweredExpr::Local(self.resolve_local("this")?, Span::generated("local")),
            self.lower_expr(key)?,
            self.lower_expr(value)?,
            Span::generated("super_prop_set_dyn"),
        ))
    }
}
