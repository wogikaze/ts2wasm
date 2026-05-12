use super::super::{
    is_array_from_call_receiver, is_array_prototype_map_call_receiver,
    is_array_prototype_push_expr, is_identity_arrow_callback, is_set_prototype_property_expr,
    is_static_date_constructor_expr, is_string_split_result_expr,
    numeric_ascending_sort_arrow_callback, private_storage_observable_access_diagnostic,
    string_constructor_arrow_callback, string_split_arrow_separator, unary_plus_arrow_callback,
    unsupported_array_map_diagnostic, unsupported_array_sort_diagnostic,
};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedParam, ResolvedStmt};
use crate::lowered::*;
use std::collections::HashMap;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl<'a> super::super::Resolver {
    pub(crate) fn lower_new_expr(
        &mut self,
        class_name: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if class_name == "RegExp" {
            return Ok(LoweredExpr::String(
                regexp_constructor_literal(args)?,
                Span::generated("str"),
            ));
        }
        if class_name == "Proxy" {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-106: Proxy constructor — Proxy is not implemented yet; use plain objects instead"
                        .to_owned(),
                span: Some(span),

                phase: None,
            });
        }
        if class_name == "Reflect" {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-106: Reflect API is not implemented yet".to_owned(),
                span: Some(span),

                phase: None,
            });
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
            if args.len() != 1 {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: if args.len() > 1 {
                        "issue-5243: multi-argument new Date(year, month, ...) is not supported in this slice".to_string()
                    } else {
                        "issue-5243: Date constructor requires an epoch-millisecond number, not a string or expression".to_string()
                    },
                    span: None,

                    phase: None,
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
            if !is_date_constructor_epoch_arg(epoch_ms) {
                let msg = if matches!(epoch_ms, ResolvedExpr::String(_)) {
                    "issue-5243: string-based Date parsing like new Date(\"2024-01-01\") is not supported in this slice"
                } else {
                    "issue-5243: Date constructor requires an epoch-millisecond number argument"
                };
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: msg.to_owned(),
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
                return Ok(LoweredExpr::ArrayNewSparse {
                    slots: Vec::new(),

                    span: Span::generated("array_new_sparse"),
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
            if class_name == "Set" && args.len() == 1 && crate::lowered::resolver::expr::facts::is_known_array_expr(&self.ctx, &args[0]) {
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
        if matches!(
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
                | "BigInt64Array"
        ) {
            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(self.lower_expr(arg)?);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::TypedArrayFromArray,
                args: lowered_args,
                span: Span::generated("runtime_call"),
            });
        }
        if class_name == "ArrayBuffer" {
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
            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(self.lower_expr(arg)?);
            }
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::DataViewNew,
                args: lowered_args,
                span: Span::generated("runtime_call"),
            });
        }
        if let Some(constructor) = BuiltinErrorConstructor::from_name(class_name) {
            let message = match args.first() {
                Some(message) => LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ErrorMessage,
                    args: vec![self.lower_expr(message)?],

                    span: Span::generated("runtime_call"),
                },
                None => LoweredExpr::String(String::new(), Span::generated("str")),
            };
            return Ok(LoweredExpr::ErrorNew {
                constructor,
                message: Box::new(message),
                span: Span::generated("error_new"),
            });
        }

        self.lower_new_with_prototype(class_name, args, span)
    }

    /// Helper for lower_new_expr: construct the New expression with
    /// prototype, private_brand, and private_slot_count setup.
    fn lower_new_with_prototype(
        &mut self,
        class_name: &str,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let prototype = match self.class_prototype_ref(class_name) {
            Ok(proto) => proto,
            Err(_diag) => {
                return Ok(LoweredExpr::Null(Span::generated("null")));
            }
        };

        let lowered_args = args
            .iter()
            .map(|arg| self.lower_expr(arg))
            .collect::<Result<Vec<_>, _>>()?;
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
