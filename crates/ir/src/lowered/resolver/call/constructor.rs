use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr};
use crate::lowered::facts::ProxyTrapKind;
use crate::lowered::*;
use crate::name_resolver::INTRINSIC_FUNCTION_CONSTRUCTOR_NEW;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(crate) fn lower_new_expr(
        &mut self,
        class_name: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if let Ok(local_id) = self.resolve_local(class_name)
            && let Some(bound) = self
                .ctx
                .facts
                .bound_constructor_locals
                .get(&local_id)
                .cloned()
        {
            let combined_args = bound
                .bound_args
                .iter()
                .chain(args.iter())
                .cloned()
                .collect::<Vec<_>>();
            return self.lower_new_with_prototype(&bound.class_name, &combined_args, span);
        }
        // Proxy construct trap: new proxy(args) → handler.construct(target, args, proxy)
        if let Ok(local_id) = self.resolve_local(class_name)
            && let Some(binding) = self.ctx.facts.proxy_locals.get(&local_id)
        {
            let args_array = ResolvedExpr::Array(
                args.iter()
                    .map(|a| ResolvedArrayElement::Present(a.clone()))
                    .collect(),
            );
            let construct_args = vec![args_array, ResolvedExpr::Ident(class_name.to_owned())];
            let pb = binding.clone();
            return self.lower_proxy_trap_call(
                pb,
                ProxyTrapKind::ProxyConstruct,
                construct_args,
                span,
            );
        }
        if let Ok(local_id) = self.resolve_local(class_name)
            && self
                .ctx
                .facts
                .host_function_handle_locals
                .contains(&local_id)
        {
            let args_array = ResolvedExpr::Array(
                args.iter()
                    .cloned()
                    .map(ResolvedArrayElement::Present)
                    .collect(),
            );
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::FunctionConstructHost,
                args: vec![
                    LoweredExpr::Local(local_id, Span::generated("local")),
                    self.lower_expr(&args_array)?,
                ],
                span: Span::generated("runtime_call"),
            });
        }
        if let Some(prototype) = self.constructable_function_prototype_ref(class_name) {
            let lowered_args = self.lower_construct_args(prototype.constructor, args)?;
            return Ok(LoweredExpr::New {
                constructor: prototype.constructor,
                prototype,
                args: lowered_args,
                base_local: self.alloc_temp(),
                private_brand: None,
                private_slot_count: 0,
                span: Span::generated("new_function_constructor"),
            });
        }
        if class_name == INTRINSIC_FUNCTION_CONSTRUCTOR_NEW {
            return self.lower_dynamic_function_constructor_host_compile(args, span);
        }
        if class_name == "eval" && self.resolve_local(class_name).is_err() {
            return Ok(LoweredExpr::Block {
                stmts: vec![LoweredStmt::Throw(
                    LoweredExpr::ErrorNew {
                        constructor: BuiltinErrorConstructor::TypeError,
                        message: Box::new(LoweredExpr::String(
                            "eval is not a constructor".to_owned(),
                            Span::generated("str"),
                        )),
                        cause: None,
                        span: Span::generated("error_new"),
                    },
                    Span::generated("throw"),
                )],
                result: Box::new(LoweredExpr::Undefined(Span::generated("undef"))),
                span: Span::generated("new_eval"),
            });
        }
        if class_name == "RegExp" {
            return self.lower_regexp_constructor(args);
        }
        if class_name == "Proxy" {
            let [target, _handler] = args else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-106: Proxy constructor requires target and handler arguments"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            };
            return self.lower_expr(target);
        }
        if class_name == "Reflect" {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-106: Reflect API is not implemented yet".to_owned(),
                span: Some(span),

                phase: None,
            });
        }
        if matches!(class_name, "Intl.NumberFormat" | "NumberFormat") {
            return self.lower_intl_number_format_constructor(args);
        }
        if matches!(class_name, "Intl.DateTimeFormat" | "DateTimeFormat") {
            return self.lower_intl_date_time_format_constructor(args);
        }
        if matches!(class_name, "Intl.DurationFormat" | "DurationFormat") {
            return self.lower_intl_duration_format_constructor(args);
        }
        if matches!(class_name, "Intl.ListFormat" | "ListFormat") {
            return self.lower_intl_list_format_constructor(args);
        }
        if class_name == "Date" {
            if args.is_empty() {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::DateNewLive,
                    args: vec![],

                    span: Span::generated("runtime_call"),
                });
            }
            let is_invalid_date = class_name == "Date"
                && (matches!(args, [ResolvedExpr::Object(_)])
                    || matches!(args, [ResolvedExpr::Ident(name)] if name == "NaN"));
            if is_invalid_date {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::DateNew,
                    args: vec![LoweredExpr::Number(0, Span::generated("num"))],

                    span: Span::generated("runtime_call"),
                });
            }
            if args.len() > 1 {
                // new Date(year, month, date, hours, minutes, seconds, ms)
                let mut date_args: Vec<LoweredExpr> = Vec::with_capacity(7);
                for arg in args.iter().take(7) {
                    date_args.push(LoweredExpr::Unary {
                        op: LoweredUnaryOp::Plus,
                        expr: Box::new(self.lower_expr(arg)?),
                        span: Span::generated("date_ctor_coerce"),
                    });
                }
                // Pad missing args: month=0, date=1, hours=0, minutes=0, seconds=0, ms=0
                while date_args.len() < 2 {
                    date_args.push(LoweredExpr::Number(0, Span::generated("date_ctor_default")));
                }
                while date_args.len() < 3 {
                    date_args.push(LoweredExpr::Number(1, Span::generated("date_ctor_default")));
                }
                while date_args.len() < 7 {
                    date_args.push(LoweredExpr::Number(0, Span::generated("date_ctor_default")));
                }
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::DateNew,
                    args: vec![LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::DateUTC,
                        args: date_args,
                        span: Span::generated("runtime_call"),
                    }],
                    span: Span::generated("runtime_call"),
                });
            }
            let epoch_ms = &args[0];
            if is_date_now_expr(epoch_ms) {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::DateNew,
                    args: vec![self.lower_expr(epoch_ms)?],

                    span: Span::generated("runtime_call"),
                });
            }
            if !is_date_constructor_epoch_arg(epoch_ms)
                && !self.is_static_number_literal_epoch_arg(epoch_ms)
                && matches!(epoch_ms, ResolvedExpr::String(_))
            {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-5243: string-based Date parsing like new Date(\"2024-01-01\") is not supported in this slice".to_owned(),
                    span: None,

                    phase: None,
                });
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DateNew,
                args: vec![self.lower_expr(epoch_ms)?],

                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "Array" {
            if args.is_empty() {
                return Ok(LoweredExpr::ArrayNew {
                    elements: Vec::new(),

                    span: Span::generated("array_new"),
                });
            }
            let [length] = args else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-405: new Array(length) currently supports exactly one small non-negative integer length".to_owned(),
                    span: None,

                    phase: None,
                });
            };
            let ResolvedExpr::Number(length) = length else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-405: new Array(length) currently requires a small non-negative integer length literal"
                            .to_owned(),
                    span: None,

                    phase: None,
                });
            };
            if *length < 0 || *length > 32 {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-405: new Array(length) currently supports lengths from 0 through 32"
                            .to_owned(),
                    span: None,

                    phase: None,
                });
            }
            return Ok(LoweredExpr::ArrayNewSparse {
                slots: vec![LoweredArraySlot::Hole; *length as usize],
                span: Span::generated("array_new_sparse"),
            });
        }
        if class_name == "Map"
            || class_name == "Set"
            || class_name == "WeakMap"
            || class_name == "WeakSet"
        {
            if args.is_empty() || class_name == "WeakMap" || class_name == "WeakSet" {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: match class_name {
                        "Map" => RuntimeFn::MapNew,
                        "Set" => RuntimeFn::SetNew,
                        "WeakMap" => RuntimeFn::WeakMapNew,
                        "WeakSet" => RuntimeFn::WeakSetNew,
                        _ => unreachable!(),
                    },
                    args: Vec::new(),

                    span: Span::generated("runtime_call"),
                });
            }
            if class_name == "Set"
                && args.len() == 1
                && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, &args[0])
            {
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::SetFromArray,
                    args: vec![self.lower_expr(&args[0])?],

                    span: Span::generated("runtime_call"),
                });
            }
            if class_name == "Set" {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-276: new Set(iterable) currently supports only known dense array inputs"
                            .to_owned(),
                    span: None,

                    phase: None,
                });
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("issue-049: new {class_name}(iterable) is not supported yet"),
                span: None,

                phase: None,
            });
        }
        if class_name == "WeakRef" {
            if args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: "WeakRef constructor requires a target argument".to_owned(),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::WeakRefNew,
                args: vec![self.lower_expr(&args[0])?],
                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "FinalizationRegistry" {
            if args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: "FinalizationRegistry constructor requires a callback argument"
                        .to_owned(),
                    span: Some(span),

                    phase: None,
                });
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::FinalizationRegistryNew,
                args: vec![self.lower_expr(&args[0])?],
                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "Promise" {
            if args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-5422: new Promise() without executor is not supported"
                        .to_owned(),
                    span: None,

                    phase: None,
                });
            }
            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(self.lower_expr(arg)?);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::PromiseConstructor,
                args: lowered_args,

                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "AggregateError" {
            let errors = args
                .first()
                .map(|arg| self.lower_expr(arg))
                .transpose()?
                .unwrap_or_else(|| LoweredExpr::ArrayNew {
                    elements: Vec::new(),
                    span: Span::generated("array_new"),
                });
            let message = match args.get(1) {
                Some(ResolvedExpr::Undefined) => {
                    LoweredExpr::String(String::new(), Span::generated("str"))
                }
                Some(message) => LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ErrorMessage,
                    args: vec![self.lower_expr(message)?],
                    span: Span::generated("runtime_call"),
                },
                None => LoweredExpr::String(String::new(), Span::generated("str")),
            };
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::AggregateError,
                args: vec![errors, message],
                span: Span::generated("runtime_call"),
            });
        }
        if is_typed_array_constructor(class_name) {
            if args.is_empty() {
                return Ok(LoweredExpr::ArrayNew {
                    elements: Vec::new(),
                    span: Span::generated("array_new"),
                });
            }
            let [arg] = args else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-419: new {class_name} currently supports zero arguments, one small length literal, or one array/TypedArray source"
                    ),
                    span: Some(span),

                    phase: None,
                });
            };
            if let ResolvedExpr::Number(length) = arg {
                if *length < 0 || *length > 32 {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-419: new {class_name}(length) currently supports lengths from 0 through 32"
                        ),
                        span: Some(span),

                        phase: None,
                    });
                }
                let dense = LoweredExpr::ArrayNew {
                    elements: (0..*length)
                        .map(|_| LoweredExpr::Number(0, Span::generated("num")))
                        .collect(),
                    span: Span::generated("array_new"),
                };
                return Ok(LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::TypedArrayFromArray,
                    args: vec![dense],
                    span: Span::generated("runtime_call"),
                });
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::TypedArrayFromArray,
                args: vec![self.lower_expr(arg)?],
                span: Span::generated("runtime_call"),
            });
        }
        if matches!(class_name, "ArrayBuffer" | "SharedArrayBuffer") {
            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(self.lower_expr(arg)?);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayBufferNew,
                args: lowered_args,
                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "SharedArrayBuffer" {
            if args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::ArityMismatch,
                    message: "SharedArrayBuffer constructor requires a byteLength argument"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::SharedArrayBufferNew,
                args: vec![self.lower_expr(&args[0])?],
                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "DataView" {
            if args.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-206: DataView constructor requires a buffer argument"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            }
            if args.len() > 2 {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-424: DataView constructor byteLength is not supported yet"
                        .to_owned(),
                    span: Some(span),
                    phase: None,
                });
            }
            let buffer = self.lower_expr(&args[0])?;
            let byte_offset = match args.get(1) {
                Some(offset) => self.lower_expr(offset)?,
                None => LoweredExpr::Number(0, Span::generated("dataview_byte_offset")),
            };
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DataViewNew,
                args: vec![buffer, byte_offset],
                span: Span::generated("runtime_call"),
            });
        }
        if let Some(constructor) = BuiltinErrorConstructor::from_name(class_name) {
            let message = match args.first() {
                Some(ResolvedExpr::Undefined) => {
                    LoweredExpr::String(String::new(), Span::generated("str"))
                }
                Some(message) => LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ErrorMessage,
                    args: vec![self.lower_expr(message)?],

                    span: Span::generated("runtime_call"),
                },
                None => LoweredExpr::String(String::new(), Span::generated("str")),
            };
            let cause = args.get(1).and_then(|options| match options {
                ResolvedExpr::Object(props) => props.iter().find_map(|prop| {
                    if prop.static_key() == Some("cause") {
                        Some(prop.value())
                    } else {
                        None
                    }
                }),
                _ => None,
            });
            let cause = match cause {
                Some(cause_expr) => {
                    match self.lower_expr(cause_expr) {
                        Ok(lowered) => Some(Box::new(lowered)),
                        Err(_) => None, // skip cause if we can't lower it
                    }
                }
                None => None,
            };
            return Ok(LoweredExpr::ErrorNew {
                constructor,
                message: Box::new(message),
                cause,
                span: Span::generated("error_new"),
            });
        }

        self.lower_new_with_prototype(class_name, args, span)
    }

    fn lower_regexp_constructor(
        &mut self,
        args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        if let Ok(raw) = regexp_constructor_literal(&self.ctx, args) {
            return Ok(LoweredExpr::String(raw, Span::generated("str")));
        }
        let flags = regexp_constructor_static_flags(&self.ctx, args)?;
        let pattern = args.first().expect("regexp arity was validated");
        Ok(LoweredExpr::Binary {
            left: Box::new(LoweredExpr::Binary {
                left: Box::new(LoweredExpr::String(
                    "/".to_owned(),
                    Span::generated("regexp_prefix"),
                )),
                op: LoweredBinaryOp::Add,
                right: Box::new(self.lower_expr(pattern)?),
                span: Span::generated("regexp_pattern_concat"),
            }),
            op: LoweredBinaryOp::Add,
            right: Box::new(LoweredExpr::String(
                format!("/{flags}"),
                Span::generated("regexp_suffix"),
            )),
            span: Span::generated("regexp_constructor"),
        })
    }

    fn is_static_number_literal_epoch_arg(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local_id| {
                self.ctx.facts.number_literal_locals.contains_key(&local_id)
            }),
            ResolvedExpr::Unary { op, expr } if *op == ts2wasm_syntax::UnaryOp::Negate => {
                self.is_static_number_literal_epoch_arg(expr)
            }
            _ => false,
        }
    }

    /// Helper for lower_new_expr: construct the New expression with
    /// prototype, private_brand, and private_slot_count setup.
    fn lower_new_with_prototype(
        &mut self,
        class_name: &str,
        args: &[ResolvedExpr],
        _span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let prototype = match self.class_prototype_ref(class_name) {
            Ok(proto) => proto,
            Err(_diag) => {
                return Ok(LoweredExpr::Null(Span::generated("null")));
            }
        };

        let lowered_args = self.lower_construct_args(prototype.constructor, args)?;
        let private_slot_count = self.private_slot_count(class_name);
        let private_brand = if self.class_has_instance_private_brand(class_name) {
            Some(self.private_brand_for_class(class_name, None)?)
        } else {
            None
        };

        Ok(LoweredExpr::New {
            constructor: prototype.constructor,
            prototype,
            args: lowered_args,
            base_local: self.alloc_temp(),
            private_brand,
            private_slot_count,
            span: Span::generated("new"),
        })
    }
}

fn is_typed_array_constructor(class_name: &str) -> bool {
    matches!(
        class_name,
        "Int8Array"
            | "Uint8Array"
            | "Uint8ClampedArray"
            | "Int16Array"
            | "Uint16Array"
            | "Int32Array"
            | "Uint32Array"
            | "Float32Array"
            | "Float64Array"
            | "Float16Array"
            | "BigInt64Array"
            | "BigUint64Array"
    )
}
