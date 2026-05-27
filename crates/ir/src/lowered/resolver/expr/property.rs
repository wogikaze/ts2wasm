use super::super::{
    is_array_prototype_push_property, is_map_prototype_property, is_private_field_storage_key,
    is_set_prototype_property, private_storage_observable_access_diagnostic,
};
use super::{is_global_builtin_function_name, lower_global_builtin_function_metadata_property};
use crate::builtin::BuiltinPropertyId;
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::classes::ObjectAccessorKey;
use crate::lowered::object_kernel;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

const STATIC_TYPED_ARRAY_BUFFER_MARKER_KEY: &str = "@@ts2wasm_typed_array_buffer";
const STATIC_TYPED_ARRAY_BUFFER_SOURCE_KEY: &str = "@@ts2wasm_typed_array_buffer_source";
const STATIC_TYPED_ARRAY_BUFFER_ELEMENT_SIZE_KEY: &str =
    "@@ts2wasm_typed_array_buffer_element_size";

impl super::super::Resolver {
    pub(super) fn lower_builtin_property_expr(
        &mut self,
        builtin: BuiltinPropertyId,
        object: &ResolvedExpr,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        match builtin {
            BuiltinPropertyId::Length => match object {
                ResolvedExpr::NewTarget { .. } => Ok(object_kernel::ordinary_get(
                    self.lower_expr(object)?,
                    "length",
                    Span::generated("new_target_length"),
                )),
                ResolvedExpr::Ident(name) if self.resolve_func(name.as_str()).is_ok() => {
                    self.lower_function_metadata_property(name.as_str(), "length", span)
                }
                ResolvedExpr::Ident(name) if is_global_builtin_function_name(name) => {
                    lower_global_builtin_function_metadata_property(name, "length")
                }
                ResolvedExpr::Ident(name) => {
                    if let Some(length) = self.local_arrow_function_length(name) {
                        Ok(LoweredExpr::Number(length as i32, Span::generated("num")))
                    } else {
                        Ok(LoweredExpr::GetLength(
                            Box::new(self.lower_expr(object)?),
                            Span::generated("get_length"),
                        ))
                    }
                }
                ResolvedExpr::PropertyAccess {
                    object: inner_object,
                    key: builtin_name,
                    ..
                } if matches!(inner_object.as_ref(), ResolvedExpr::Ident(name) if name == "Number")
                    && crate::lowered::program_builtins::builtin_function_token_value(
                        builtin_name,
                    )
                    .is_some() =>
                {
                    lower_global_builtin_function_metadata_property(builtin_name, "length")
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
        if matches!(object, ResolvedExpr::ImportMeta { .. }) && key == "url" {
            return Ok(self.lower_module_meta_url(span));
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "arguments")
            && key == "callee"
            && !self.ctx.is_strict_context()
            && let Some(func_id) = self.current_func_id
        {
            return Ok(LoweredExpr::ArrowFn {
                func_id,
                captures: Vec::new(),
                representation: ClosureRepresentation::DirectLocalToken,
                span: Span::generated("arguments_callee"),
            });
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "Object") && key == "prototype" {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ObjectPrototype,
                args: Vec::new(),
                span: Span::generated("object_prototype"),
            });
        }
        if key == "prototype"
            && let ResolvedExpr::Ident(name) = object
            && let Some(prototype) = static_builtin_prototype_object(name, span)
        {
            return Ok(prototype);
        }
        if let ResolvedExpr::Ident(name) = object
            && name == "Number"
            && let Some(token) =
                crate::lowered::program_builtins::builtin_function_token_expr(key, span)
        {
            return Ok(token);
        }
        if let ResolvedExpr::Ident(name) = object
            && let Some(value) =
                crate::lowered::program_builtins::known_global_property_value_expr(name, key, span)
        {
            return Ok(value);
        }
        if let ResolvedExpr::PropertyAccess {
            object: inner_object,
            key: builtin_name,
            ..
        } = object
            && matches!(inner_object.as_ref(), ResolvedExpr::Ident(name) if name == "Number")
            && crate::lowered::program_builtins::builtin_function_token_value(builtin_name)
                .is_some()
            && matches!(key, "name" | "length")
        {
            return lower_global_builtin_function_metadata_property(builtin_name, key);
        }
        if key == "__proto__" {
            return Ok(object_kernel::ordinary_get_prototype_of(
                self.lower_expr(object)?,
                Span::generated("object_proto_get"),
            ));
        }
        if matches!(object, ResolvedExpr::Ident(name) if name == "Number")
            && matches!(key, "parseInt" | "parseFloat")
        {
            return Ok(LoweredExpr::Undefined(Span::generated("undef")));
        }
        if key == "prototype"
            && let ResolvedExpr::MethodCall {
                object: callee_object,
                method,
                args,
                ..
            } = object
            && matches!(callee_object.as_ref(), ResolvedExpr::Ident(name) if name == "Object")
            && method == "getPrototypeOf"
            && matches!(
                args.as_slice(),
                [ResolvedExpr::FunctionExpr {
                    is_generator: true,
                    ..
                }]
            )
        {
            return Ok(LoweredExpr::ObjectNew {
                props: Vec::new(),
                non_enumerable: 0,
                span: Span::generated("generator_prototype"),
            });
        }
        if key == "constructor"
            && let ResolvedExpr::PropertyAccess {
                object: prototype_object,
                key: prototype_key,
                ..
            } = object
            && prototype_key == "prototype"
            && let ResolvedExpr::Ident(name) = prototype_object.as_ref()
            && self.constructable_function_prototype_ref(name).is_some()
            && let Ok(local) = self.resolve_local(name)
        {
            return Ok(LoweredExpr::Local(local, Span::generated("local")));
        }
        if key == "description" {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SymbolDescription,
                args: vec![self.lower_expr(object)?],
                span: Span::generated("runtime_call"),
            });
        }
        // Well-known symbols: Symbol.iterator, Symbol.species, etc.
        if let ResolvedExpr::Ident(name) = object
            && name == "Symbol"
            && let Some(wk_index) = well_known_symbol_index(key)
        {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SymbolWellKnown,
                args: vec![
                    LoweredExpr::Number(wk_index as i32, Span::generated("wk_idx")),
                    LoweredExpr::String(
                        well_known_symbol_description(key),
                        Span::generated("wk_desc"),
                    ),
                ],
                span: Span::generated("runtime_call"),
            });
        }
        if is_array_prototype_push_property(object, key) {
            return Ok(LoweredExpr::Number(0, Span::generated("num")));
        }
        if key.starts_with('#') {
            return self.lower_private_field_get(object, key, span);
        }
        if let ResolvedExpr::Ident(name) = object
            && let Ok(obj_local) = self.resolve_local(name)
            && let Some(getter_id) = self
                .ctx
                .classes
                .object_accessor_props
                .get(&obj_local)
                .and_then(|props| props.get(&ObjectAccessorKey::Property(key.to_owned())))
                .and_then(|prop| prop.get)
        {
            let lowered_args = self.lower_function_call_args(
                getter_id,
                LoweredExpr::Local(obj_local, Span::generated("local")),
                &[],
            )?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(getter_id),
                args: lowered_args,
                span: Span::generated("call"),
            });
        }
        if let ResolvedExpr::Ident(name) = object
            && let Ok(obj_local) = self.resolve_local(name)
            && let Some(class_name) = self.ctx.classes.local_classes.get(&obj_local).cloned()
            && let Some(getter_id) = self.resolve_class_getter(&class_name, key)
        {
            let lowered_args = self.lower_function_call_args(
                getter_id,
                LoweredExpr::Local(obj_local, Span::generated("local")),
                &[],
            )?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(getter_id),
                args: lowered_args,
                span: Span::generated("call"),
            });
        }
        if let ResolvedExpr::Ident(name) = object
            && self.ctx.classes.class_constructor_ids.contains_key(name)
            && let Some(getter_id) = self.resolve_static_class_method(name, &format!("get {key}"))
        {
            let receiver = LoweredExpr::ClassPrototype(
                self.class_prototype_ref(name)?,
                Span::generated("class_static_getter"),
            );
            let lowered_args = self.lower_function_call_args(getter_id, receiver, &[])?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(getter_id),
                args: lowered_args,
                span: Span::generated("call"),
            });
        }
        if let Some(function) = self.object_function_property(object, key, span) {
            return Ok(function);
        }
        if let ResolvedExpr::Ident(name) = object
            && self.resolve_func(name.as_str()).is_ok()
        {
            // Function metadata properties (name, length, prototype) go directly
            // to the metadata resolver.  Non-metadata properties (e.g. user-defined
            // properties like assert._isSameValue) fall through to OrdinaryGet so
            // they work when assigned at runtime via assert.foo = function(){}.
            if matches!(key, "name" | "length" | "prototype") {
                return self.lower_function_metadata_property(name.as_str(), key, span);
            }
        }
        if matches!(key, "name" | "length" | "prototype")
            && let ResolvedExpr::Ident(name) = object
            && let Some(metadata) = self.local_arrow_function_metadata_property(name, key)
        {
            return Ok(metadata);
        }
        if let ResolvedExpr::Ident(name) = object
            && is_global_builtin_function_name(name)
            && matches!(key, "name" | "length")
        {
            return lower_global_builtin_function_metadata_property(name, key);
        }
        if key == "size"
            && let Some(result) = self.lower_collection_size(object)?
        {
            return Ok(result);
        }
        if let Some(result) = self.lower_typed_array_property(object, key, span)? {
            return Ok(result);
        }
        if let Some(result) = self.lower_typed_array_constructor_property(object, key)? {
            return Ok(result);
        }
        if is_set_prototype_property(object, key, "add") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeAddGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        if is_set_prototype_property(object, key, "has") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeHasGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        if is_set_prototype_property(object, key, "delete") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeDeleteGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        if is_set_prototype_property(object, key, "forEach") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SetPrototypeForEachGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "get") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeGetGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "set") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeSetGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "has") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeHasGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "delete") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeDeleteGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        if is_map_prototype_property(object, key, "forEach") {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MapPrototypeForEachGet,
                args: Vec::new(),
                span: Span::generated("runtime_call"),
            });
        }
        // Interface-typed property access: use interface definition to resolve
        // property type before falling through to proxy checks.
        // Also fall back to local_classes when local_type_aliases hasn't been
        // populated (e.g. for built-in class names that match interface names).
        if let ResolvedExpr::Ident(name) = object
            && let Ok(obj_local) = self.resolve_local(name)
            && let Some(iface_name) = self
                .ctx
                .classes
                .local_type_aliases
                .get(&obj_local)
                .or_else(|| self.ctx.classes.local_classes.get(&obj_local))
            && let Some(props) = self.ctx.lookup_interface_properties(iface_name)
            && props.iter().any(|(pn, _)| pn == key)
        {
            let lowered_object = self.lower_expr(object)?;
            return Ok(object_kernel::ordinary_get(lowered_object, key, span));
        }
        if let Some(proxy) =
            crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(&self.ctx, object)
        {
            return self.lower_proxy_trap_call(
                proxy,
                crate::lowered::facts::ProxyTrapKind::ProxyGet,
                vec![ResolvedExpr::String(key.to_owned())],
                span,
            );
        }
        let lowered_object = self.lower_expr(object)?;
        if let Some(function) = self.lowered_object_arrow_fn_property(&lowered_object, key) {
            return Ok(function);
        }
        Ok(self.lower_property_get_with_null_guard(lowered_object, key, span))
    }

    fn lowered_object_arrow_fn_property(
        &self,
        object: &LoweredExpr,
        key: &str,
    ) -> Option<LoweredExpr> {
        let LoweredExpr::ObjectNew { props, .. } = object else {
            return None;
        };
        props
            .iter()
            .rev()
            .find(|(prop_key, _)| prop_key == key)
            .and_then(|(_, value)| {
                matches!(value, LoweredExpr::ArrowFn { .. }).then(|| value.clone())
            })
    }

    fn lower_property_get_with_null_guard(
        &mut self,
        object: LoweredExpr,
        key: &str,
        span: Span,
    ) -> LoweredExpr {
        let temp = self.alloc_temp();
        let error_msg = format!("Cannot read properties of undefined (reading '{}')", key);
        LoweredExpr::Block {
            stmts: vec![
                LoweredStmt::Let(temp, object, Span::generated("let_stmt")),
                LoweredStmt::If {
                    condition: LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Local(temp, Span::generated("local"))),
                        op: LoweredBinaryOp::EqualEqual,
                        right: Box::new(LoweredExpr::Null(Span::generated("null"))),
                        span: Span::generated("null_check"),
                    },
                    then_body: vec![LoweredStmt::Throw(
                        LoweredExpr::ErrorNew {
                            constructor: BuiltinErrorConstructor::TypeError,
                            message: Box::new(LoweredExpr::String(
                                error_msg,
                                Span::generated("str"),
                            )),
                            cause: None,
                            errors: None,
                            span: Span::generated("error_new"),
                        },
                        Span::generated("throw"),
                    )],
                    else_body: vec![],
                    span: Span::generated("if"),
                },
            ],
            result: Box::new(object_kernel::ordinary_get(
                LoweredExpr::Local(temp, Span::generated("local")),
                key,
                span,
            )),
            span,
        }
    }

    pub(super) fn lower_property_set_with_null_guard(
        &mut self,
        object: LoweredExpr,
        key: &str,
        value: LoweredExpr,
        span: Span,
    ) -> LoweredExpr {
        let temp = self.alloc_temp();
        let error_msg = format!("Cannot set properties of undefined (setting '{}')", key);
        LoweredExpr::Block {
            stmts: vec![
                LoweredStmt::Let(temp, object, Span::generated("let_stmt")),
                LoweredStmt::If {
                    condition: LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Local(temp, Span::generated("local"))),
                        op: LoweredBinaryOp::EqualEqual,
                        right: Box::new(LoweredExpr::Null(Span::generated("null"))),
                        span: Span::generated("null_check"),
                    },
                    then_body: vec![LoweredStmt::Throw(
                        LoweredExpr::ErrorNew {
                            constructor: BuiltinErrorConstructor::TypeError,
                            message: Box::new(LoweredExpr::String(
                                error_msg,
                                Span::generated("str"),
                            )),
                            cause: None,
                            errors: None,
                            span: Span::generated("error_new"),
                        },
                        Span::generated("throw"),
                    )],
                    else_body: vec![],
                    span: Span::generated("if"),
                },
            ],
            result: Box::new(object_kernel::ordinary_set(
                LoweredExpr::Local(temp, Span::generated("local")),
                key,
                value,
                span,
            )),
            span,
        }
    }

    pub(super) fn lower_property_get_dynamic_with_null_guard(
        &mut self,
        object: LoweredExpr,
        index: LoweredExpr,
        span: Span,
    ) -> LoweredExpr {
        let temp = self.alloc_temp();
        let error_msg = "Cannot read properties of undefined (reading '<computed>')".to_owned();
        LoweredExpr::Block {
            stmts: vec![
                LoweredStmt::Let(temp, object, Span::generated("let_stmt")),
                LoweredStmt::If {
                    condition: LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Local(temp, Span::generated("local"))),
                        op: LoweredBinaryOp::EqualEqual,
                        right: Box::new(LoweredExpr::Null(Span::generated("null"))),
                        span: Span::generated("null_check"),
                    },
                    then_body: vec![LoweredStmt::Throw(
                        LoweredExpr::ErrorNew {
                            constructor: BuiltinErrorConstructor::TypeError,
                            message: Box::new(LoweredExpr::String(
                                error_msg,
                                Span::generated("str"),
                            )),
                            cause: None,
                            errors: None,
                            span: Span::generated("error_new"),
                        },
                        Span::generated("throw"),
                    )],
                    else_body: vec![],
                    span: Span::generated("if"),
                },
            ],
            result: Box::new(object_kernel::ordinary_get_dynamic(
                LoweredExpr::Local(temp, Span::generated("local")),
                index,
                span,
            )),
            span,
        }
    }

    pub(super) fn lower_property_set_dynamic_with_null_guard(
        &mut self,
        object: LoweredExpr,
        index: LoweredExpr,
        value: LoweredExpr,
        span: Span,
    ) -> LoweredExpr {
        let temp = self.alloc_temp();
        let error_msg = "Cannot set properties of undefined (setting '<computed>')".to_owned();
        LoweredExpr::Block {
            stmts: vec![
                LoweredStmt::Let(temp, object, Span::generated("let_stmt")),
                LoweredStmt::If {
                    condition: LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Local(temp, Span::generated("local"))),
                        op: LoweredBinaryOp::EqualEqual,
                        right: Box::new(LoweredExpr::Null(Span::generated("null"))),
                        span: Span::generated("null_check"),
                    },
                    then_body: vec![LoweredStmt::Throw(
                        LoweredExpr::ErrorNew {
                            constructor: BuiltinErrorConstructor::TypeError,
                            message: Box::new(LoweredExpr::String(
                                error_msg,
                                Span::generated("str"),
                            )),
                            cause: None,
                            errors: None,
                            span: Span::generated("error_new"),
                        },
                        Span::generated("throw"),
                    )],
                    else_body: vec![],
                    span: Span::generated("if"),
                },
            ],
            result: Box::new(object_kernel::ordinary_set_dynamic(
                LoweredExpr::Local(temp, Span::generated("local")),
                index,
                value,
                span,
            )),
            span,
        }
    }

    fn object_function_property(
        &self,
        object: &ResolvedExpr,
        key: &str,
        span: Span,
    ) -> Option<LoweredExpr> {
        let ResolvedExpr::Ident(name) = object else {
            return None;
        };
        let obj_local = self.resolve_local(name).ok()?;
        let func_id = self
            .ctx
            .classes
            .object_function_props
            .get(&obj_local)
            .and_then(|props| props.get(&ObjectAccessorKey::Property(key.to_owned())))
            .copied()?;
        self.function_token_for_object_method(func_id, span)
    }

    fn computed_object_function_property(
        &self,
        object: &ResolvedExpr,
        index: &ResolvedExpr,
        span: Span,
    ) -> Option<LoweredExpr> {
        let ResolvedExpr::Ident(name) = object else {
            return None;
        };
        let obj_local = self.resolve_local(name).ok()?;
        let key = super::super::string::resolved_expr_static_accessor_key(&self.ctx, index)?;
        let func_id = self
            .ctx
            .classes
            .object_function_props
            .get(&obj_local)
            .and_then(|props| props.get(&key))
            .copied()?;
        self.function_token_for_object_method(func_id, span)
    }

    fn lowered_object_function_dynamic_property(
        &self,
        object: &LoweredExpr,
        index: &LoweredExpr,
        span: Span,
    ) -> Option<LoweredExpr> {
        let key = self.property_lowered_static_accessor_key(index)?;
        let props = self.function_props_for_lowered_object_expr(object)?;
        let func_id = props.get(&key).copied()?;
        self.function_token_for_object_method(func_id, span)
    }

    fn property_lowered_static_accessor_key(
        &self,
        expr: &LoweredExpr,
    ) -> Option<ObjectAccessorKey> {
        match expr {
            LoweredExpr::String(value, _) => Some(ObjectAccessorKey::Property(value.clone())),
            LoweredExpr::Number(value, _) => Some(ObjectAccessorKey::Property(value.to_string())),
            LoweredExpr::Local(local, _) => self
                .ctx
                .facts
                .string_value(*local)
                .cloned()
                .map(ObjectAccessorKey::Property)
                .or_else(|| {
                    self.ctx
                        .facts
                        .symbol_value_locals
                        .contains(local)
                        .then_some(ObjectAccessorKey::SymbolLocal(*local))
                }),
            LoweredExpr::Call {
                kind: FunctionCallKind::User(func_id),
                args,
                ..
            } => {
                if let Some(value) = self.ctx.functions.static_string_returns.get(func_id) {
                    return Some(ObjectAccessorKey::Property(value.clone()));
                }
                let signature = self.ctx.symbols.function_signatures.get(func_id)?;
                if signature.returns_first_param_identity && args.len() == 1 {
                    self.property_lowered_static_accessor_key(&args[0])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub(crate) fn function_token_for_object_method(
        &self,
        func_id: FuncId,
        span: Span,
    ) -> Option<LoweredExpr> {
        let captures = self
            .ctx
            .functions
            .function_captures
            .get(&func_id)
            .map(|captures| {
                captures
                    .iter()
                    .map(|capture| self.resolve_local(capture))
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .unwrap_or_else(|| Some(Vec::new()))?;
        Some(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: ClosureRepresentation::DirectLocalToken,
            span,
        })
    }

    fn local_arrow_function_length(&self, name: &str) -> Option<usize> {
        let local = self.resolve_local(name).ok()?;
        let closure = self.ctx.facts.arrow_locals.get(&local)?;
        self.ctx
            .symbols
            .function_signatures
            .get(&closure.func_id)
            .and_then(|signature| signature.metadata_length)
    }

    fn local_arrow_function_metadata_property(&self, name: &str, key: &str) -> Option<LoweredExpr> {
        let local = self.resolve_local(name).ok()?;
        let closure = self.ctx.facts.arrow_locals.get(&local)?;
        match key {
            "name" => Some(LoweredExpr::String(
                self.ctx
                    .facts
                    .function_metadata_name_locals
                    .get(&local)
                    .cloned()
                    .unwrap_or_else(|| name.to_owned()),
                Span::generated("str"),
            )),
            "length" => {
                let length = self
                    .ctx
                    .symbols
                    .function_signatures
                    .get(&closure.func_id)
                    .and_then(|signature| signature.metadata_length)?;
                Some(LoweredExpr::Number(length as i32, Span::generated("num")))
            }
            "prototype" => {
                if let Some(prototype) = self.constructable_function_prototype_ref(name) {
                    Some(LoweredExpr::ClassPrototype(
                        prototype,
                        Span::generated("function_prototype_object"),
                    ))
                } else {
                    Some(LoweredExpr::ObjectNew {
                        props: Vec::new(),
                        non_enumerable: 0,
                        span: Span::generated("function_prototype_object"),
                    })
                }
            }
            _ => None,
        }
    }

    pub(super) fn lower_optional_property_access_expr(
        &mut self,
        object: &ResolvedExpr,
        key: &str,
    ) -> Result<LoweredExpr, Diagnostic> {
        if is_private_field_storage_key(key) {
            return Err(private_storage_observable_access_diagnostic(None));
        }
        // Interface-typed optional property access: use interface definition to
        // determine whether the property is known at compile time, falling through
        // to the generic optional get path for unknown properties.
        if let ResolvedExpr::Ident(name) = object
            && let Ok(obj_local) = self.resolve_local(name)
            && let Some(iface_name) = self
                .ctx
                .classes
                .local_type_aliases
                .get(&obj_local)
                .or_else(|| self.ctx.classes.local_classes.get(&obj_local))
            && let Some(props) = self.ctx.lookup_interface_properties(iface_name)
            && props.iter().any(|(pn, _)| pn == key)
        {
            return Ok(object_kernel::ordinary_get(
                self.lower_expr(object)?,
                key,
                Span::generated("opt_prop_get"),
            ));
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
        if let ResolvedExpr::Ident(name) = object
            && let Ok(obj_local) = self.resolve_local(name)
            && let Some(static_key) =
                super::super::string::resolved_expr_static_accessor_key(&self.ctx, index)
            && let Some(getter_id) = self
                .ctx
                .classes
                .object_accessor_props
                .get(&obj_local)
                .and_then(|props| props.get(&static_key))
                .and_then(|prop| prop.get)
        {
            let lowered_args = self.lower_function_call_args(
                getter_id,
                LoweredExpr::Local(obj_local, Span::generated("local")),
                &[],
            )?;
            return Ok(LoweredExpr::Call {
                kind: FunctionCallKind::User(getter_id),
                args: lowered_args,
                span: Span::generated("call"),
            });
        }
        if let Some(function) =
            self.computed_object_function_property(object, index, Span::generated("index"))
        {
            return Ok(function);
        }
        if self.should_lower_static_index_as_property(object, index)
            && let Some(static_key) =
                super::super::string::resolved_expr_static_property_key_value(&self.ctx, index)
        {
            return Ok(object_kernel::ordinary_get(
                self.lower_expr(object)?,
                &static_key,
                Span::generated("index"),
            ));
        }
        // Interface-typed computed access: if the index is a static string that
        // matches an interface property, bypass proxy checks and go directly to
        // ordinary_get to avoid misrouting proxy traps.
        if let ResolvedExpr::Ident(name) = object
            && let Ok(obj_local) = self.resolve_local(name)
            && let Some(iface_name) = self
                .ctx
                .classes
                .local_type_aliases
                .get(&obj_local)
                .or_else(|| self.ctx.classes.local_classes.get(&obj_local))
            && let Some(props) = self.ctx.lookup_interface_properties(iface_name)
            && let Some(static_key) =
                super::super::string::resolved_expr_static_property_key_value(&self.ctx, index)
            && props.iter().any(|(pn, _)| pn == &static_key)
        {
            return Ok(object_kernel::ordinary_get(
                self.lower_expr(object)?,
                &static_key,
                Span::generated("index"),
            ));
        }
        if let Some(proxy) =
            crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(&self.ctx, object)
        {
            return self.lower_proxy_trap_call(
                proxy,
                crate::lowered::facts::ProxyTrapKind::ProxyGet,
                vec![index.clone()],
                Span::generated("proxy_get"),
            );
        }
        let lowered_object = self.lower_expr(object)?;
        let lowered_index = self.lower_expr(index)?;
        if let Some(function) = self.lowered_object_function_dynamic_property(
            &lowered_object,
            &lowered_index,
            Span::generated("index"),
        ) {
            return Ok(function);
        }

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
        } else if crate::lowered::resolver::expr::facts::is_known_typed_array_expr(
            &self.ctx, object,
        ) {
            Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::TypedArrayLoad,
                args: vec![lowered_object, lowered_index],
                span: Span::generated("typed_array_load"),
            })
        } else {
            Ok(self.lower_property_get_dynamic_with_null_guard(
                lowered_object,
                lowered_index,
                Span::generated("index"),
            ))
        }
    }

    fn should_lower_static_index_as_property(
        &self,
        object: &ResolvedExpr,
        index: &ResolvedExpr,
    ) -> bool {
        if super::super::string::resolved_expr_static_property_key_value(&self.ctx, index).is_none()
        {
            return false;
        }
        if crate::lowered::resolver::expr::facts::resolved_expr_proxy_binding(&self.ctx, object)
            .is_some()
        {
            return false;
        }
        if matches!(object, ResolvedExpr::Array(_)) {
            return false;
        }
        if let ResolvedExpr::Ident(name) = object
            && let Ok(local) = self.resolve_local(name)
        {
            return !self.ctx.facts.array_locals.contains(&local);
        }
        true
    }

    fn lower_super_property_get(
        &mut self,
        _object: &ResolvedExpr,
        key: &str,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let Some(class_name) = self.ctx.classes.current_class.as_ref() else {
            let this_local = self.resolve_local("this").map_err(|_| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super property access requires class context or object method receiver"
                    .to_owned(),
                span: Some(span),
                phase: None,
            })?;
            // SuperPropertyObject: object literal methods resolve super.prop via
            // Object.getPrototypeOf(this).prop when statically dispatched.
            return Ok(object_kernel::ordinary_get(
                object_kernel::ordinary_get_prototype_of(
                    LoweredExpr::Local(this_local, Span::generated("local")),
                    Span::generated("object_home_proto"),
                ),
                key,
                span,
            ));
        };
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
        if let Some(method_id) = self.resolve_class_method(&parent_name, key) {
            return self.lower_direct_function_token(method_id);
        }
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
        let Some(class_name) = self.ctx.classes.current_class.as_ref() else {
            let this_local = self.resolve_local("this").map_err(|_| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super computed access requires class context or object method receiver"
                    .to_owned(),
                span: Some(Span::generated("super-computed")),
                phase: None,
            })?;
            return Ok(object_kernel::ordinary_get_dynamic(
                object_kernel::ordinary_get_prototype_of(
                    LoweredExpr::Local(this_local, Span::generated("local")),
                    Span::generated("object_home_proto"),
                ),
                self.lower_expr(index)?,
                Span::generated("super_index_get"),
            ));
        };
        let parent_name = self
            .ctx
            .classes
            .class_parents
            .get(class_name)
            .and_then(|p| p.clone())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "super computed access used in class without extends".to_owned(),
                span: Some(Span::generated("super-computed")),
                phase: None,
            })?;
        if let Some(ObjectAccessorKey::Property(key)) =
            super::super::string::resolved_expr_static_accessor_key(&self.ctx, index)
            && let Some(method_id) = self.resolve_class_method(&parent_name, &key)
        {
            return self.lower_direct_function_token(method_id);
        }
        let parent_ref = self.class_prototype_ref(&parent_name)?;
        Ok(object_kernel::ordinary_get_dynamic(
            LoweredExpr::ClassPrototype(parent_ref, Span::generated("class_proto")),
            self.lower_expr(index)?,
            Span::generated("super_index_get"),
        ))
    }

    fn lower_direct_function_token(&mut self, func_id: FuncId) -> Result<LoweredExpr, Diagnostic> {
        let captures = self
            .ctx
            .functions
            .function_captures
            .get(&func_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .iter()
            .map(|capture| self.resolve_local(capture))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation: ClosureRepresentation::DirectLocalToken,
            span: Span::generated("class_method_token"),
        })
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

    /// Handle `.byteLength`, `.buffer`, `.byteOffset` on TypedArray, ArrayBuffer, DataView instances.
    fn lower_typed_array_property(
        &mut self,
        object: &ResolvedExpr,
        key: &str,
        span: Span,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        let ResolvedExpr::Ident(name) = object else {
            return Ok(None);
        };
        let Ok(obj_local) = self.resolve_local(name) else {
            return Ok(None);
        };
        let Some(class_name) = self.ctx.classes.local_classes.get(&obj_local) else {
            return Ok(None);
        };
        match (class_name.as_str(), key) {
            ("ArrayBuffer" | "SharedArrayBuffer", "byteLength") => {
                // ArrayBuffer layout has byte_length as the first field, same as GetLength reads.
                Ok(Some(LoweredExpr::GetLength(
                    Box::new(self.lower_expr(object)?),
                    span,
                )))
            }
            ("ArrayBuffer" | "SharedArrayBuffer", "detached") => {
                // detach not tracked at runtime — always false for non-transferred buffers
                Ok(Some(LoweredExpr::Bool(false, span)))
            }
            ("DataView", "byteLength") => {
                // DataView byteLength = underlying buffer byte length, same as ArrayBuffer
                Ok(Some(LoweredExpr::GetLength(
                    Box::new(self.lower_expr(object)?),
                    span,
                )))
            }
            ("ArrayBuffer" | "SharedArrayBuffer", "maxByteLength") => {
                // Non-resizable buffers: maxByteLength == byteLength
                Ok(Some(LoweredExpr::GetLength(
                    Box::new(self.lower_expr(object)?),
                    span,
                )))
            }
            ("ArrayBuffer" | "SharedArrayBuffer", "resizable" | "growable") => {
                // Resizable buffers not implemented
                Ok(Some(LoweredExpr::Bool(false, span)))
            }
            ("ArrayBuffer", "immutable") => {
                // Non-resizable buffers are immutable in practice
                Ok(Some(LoweredExpr::Bool(true, span)))
            }
            ("DataView", "buffer") => Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DataViewGetBuffer,
                args: vec![self.lower_expr(object)?],
                span,
            })),
            ("DataView", "byteOffset") => Ok(Some(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DataViewGetByteOffset,
                args: vec![self.lower_expr(object)?],
                span,
            })),
            (cn, "byteLength") if crate::lowered::program_builtins::is_typed_array_class(cn) => {
                // byteLength = element_count * BYTES_PER_ELEMENT
                if let Some(elem_size) = typed_array_element_size(cn) {
                    Ok(Some(LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::GetLength(
                            Box::new(self.lower_expr(object)?),
                            Span::generated("get_length"),
                        )),
                        op: crate::lowered::LoweredBinaryOp::Multiply,
                        right: Box::new(LoweredExpr::Number(elem_size, Span::generated("num"))),
                        span,
                    }))
                } else {
                    Ok(None)
                }
            }
            (cn, "byteOffset") if is_typed_array_or_dataview(cn) => {
                if cn == "DataView" {
                    Ok(Some(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::DataViewGetByteOffset,
                        args: vec![self.lower_expr(object)?],
                        span,
                    }))
                } else {
                    // TypedArray: byteOffset = 0 (non-buffer-backed)
                    Ok(Some(LoweredExpr::Number(0, Span::generated("num"))))
                }
            }
            (cn, "buffer") if is_typed_array_or_dataview(cn) => {
                if cn == "DataView" {
                    Ok(Some(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::DataViewGetBuffer,
                        args: vec![self.lower_expr(object)?],
                        span,
                    }))
                } else if let Some(elem_size) = typed_array_element_size(cn) {
                    let source = self.lower_expr(object)?;
                    let byte_length = LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::GetLength(
                            Box::new(source.clone()),
                            Span::generated("get_length"),
                        )),
                        op: crate::lowered::LoweredBinaryOp::Multiply,
                        right: Box::new(LoweredExpr::Number(elem_size, Span::generated("num"))),
                        span,
                    };
                    Ok(Some(LoweredExpr::ObjectNew {
                        props: vec![
                            (
                                STATIC_TYPED_ARRAY_BUFFER_MARKER_KEY.to_owned(),
                                LoweredExpr::Bool(true, Span::generated("typed_array_buffer")),
                            ),
                            (STATIC_TYPED_ARRAY_BUFFER_SOURCE_KEY.to_owned(), source),
                            (
                                STATIC_TYPED_ARRAY_BUFFER_ELEMENT_SIZE_KEY.to_owned(),
                                LoweredExpr::Number(elem_size, Span::generated("num")),
                            ),
                            ("byteLength".to_owned(), byte_length),
                        ],
                        non_enumerable: 0b1111,
                        span,
                    }))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    /// Handle `BYTES_PER_ELEMENT` on TypedArray constructors (e.g. `Int8Array.BYTES_PER_ELEMENT`).
    fn lower_typed_array_constructor_property(
        &mut self,
        object: &ResolvedExpr,
        key: &str,
    ) -> Result<Option<LoweredExpr>, Diagnostic> {
        let ResolvedExpr::Ident(name) = object else {
            return Ok(None);
        };
        if key == "BYTES_PER_ELEMENT"
            && crate::lowered::program_builtins::is_typed_array_class(name)
        {
            return Ok(typed_array_element_size(name)
                .map(|size| LoweredExpr::Number(size, Span::generated("num"))));
        }
        Ok(None)
    }

    /// Returns true if the resolved expression refers to a typed array local.
    fn expr_is_typed_array(&self, object: &ResolvedExpr) -> bool {
        let ResolvedExpr::Ident(name) = object else {
            return false;
        };
        let Ok(obj_local) = self.resolve_local(name) else {
            return false;
        };
        self.ctx
            .classes
            .local_classes
            .get(&obj_local)
            .is_some_and(|cn| crate::lowered::program_builtins::is_typed_array_class(cn))
    }

    pub(crate) fn lower_proxy_trap_call(
        &mut self,
        proxy: crate::lowered::facts::ProxyBinding,
        trap: crate::lowered::facts::ProxyTrapKind,
        mut args: Vec<ResolvedExpr>,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        // ProxyDispatch: statically known Proxy receivers route through handler traps.
        let mut trap_args = Vec::with_capacity(args.len() + 1);
        trap_args.push(proxy.target);
        trap_args.append(&mut args);
        self.lower_method_call_expr(&proxy.handler, trap.method_name(), &trap_args, span)
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

fn static_builtin_prototype_object(name: &str, span: Span) -> Option<LoweredExpr> {
    match name {
        "Date" => Some(LoweredExpr::ObjectNew {
            props: vec![
                (
                    "getYear".to_owned(),
                    LoweredExpr::Undefined(Span::generated("date_get_year")),
                ),
                (
                    "setYear".to_owned(),
                    LoweredExpr::Undefined(Span::generated("date_set_year")),
                ),
                (
                    "toGMTString".to_owned(),
                    LoweredExpr::Undefined(Span::generated("date_to_gmt_string")),
                ),
            ],
            non_enumerable: 0b111,
            span,
        }),
        "Error" | "EvalError" | "RangeError" | "ReferenceError" | "SyntaxError" | "TypeError"
        | "URIError" | "AggregateError" => Some(LoweredExpr::ObjectNew {
            props: vec![
                (
                    "constructor".to_owned(),
                    LoweredExpr::Undefined(Span::generated("error_prototype_constructor")),
                ),
                (
                    "name".to_owned(),
                    LoweredExpr::String(name.to_owned(), Span::generated("error_prototype_name")),
                ),
                (
                    "message".to_owned(),
                    LoweredExpr::String(String::new(), Span::generated("error_prototype_message")),
                ),
            ],
            non_enumerable: 0b111,
            span,
        }),
        _ => None,
    }
}

/// Returns the well-known symbol cache index for a given property key.
fn well_known_symbol_index(key: &str) -> Option<u32> {
    match key {
        "iterator" => Some(0),
        "species" => Some(1),
        "toPrimitive" => Some(2),
        "toStringTag" => Some(3),
        "hasInstance" => Some(4),
        "isConcatSpreadable" => Some(5),
        "match" => Some(6),
        "replace" => Some(7),
        "search" => Some(8),
        "split" => Some(9),
        "unscopables" => Some(10),
        "asyncIterator" => Some(11),
        "asyncDispose" => Some(12),
        "dispose" => Some(13),
        "matchAll" => Some(14),
        "metadata" => Some(15),
        _ => None,
    }
}

/// Returns the ECMAScript description string for a well-known symbol.
fn well_known_symbol_description(key: &str) -> String {
    match key {
        "iterator" => "Symbol.iterator",
        "species" => "Symbol.species",
        "toPrimitive" => "Symbol.toPrimitive",
        "toStringTag" => "Symbol.toStringTag",
        "hasInstance" => "Symbol.hasInstance",
        "isConcatSpreadable" => "Symbol.isConcatSpreadable",
        "match" => "Symbol.match",
        "replace" => "Symbol.replace",
        "search" => "Symbol.search",
        "split" => "Symbol.split",
        "unscopables" => "Symbol.unscopables",
        "asyncIterator" => "Symbol.asyncIterator",
        "asyncDispose" => "Symbol.asyncDispose",
        "dispose" => "Symbol.dispose",
        "matchAll" => "Symbol.matchAll",
        "metadata" => "Symbol.metadata",
        _ => key,
    }
    .to_owned()
}

/// Returns BYTES_PER_ELEMENT for a typed array class name.
fn typed_array_element_size(class_name: &str) -> Option<i32> {
    match class_name {
        "Int8Array" | "Uint8Array" | "Uint8ClampedArray" => Some(1),
        "Int16Array" | "Uint16Array" | "Float16Array" => Some(2),
        "Int32Array" | "Uint32Array" | "Float32Array" => Some(4),
        "Float64Array" | "BigInt64Array" | "BigUint64Array" => Some(8),
        _ => None,
    }
}

/// Returns true for TypedArray, DataView, or ArrayBuffer class names.
fn is_typed_array_or_dataview(class_name: &str) -> bool {
    crate::lowered::program_builtins::is_typed_array_class(class_name) || class_name == "DataView"
}
