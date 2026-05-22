use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_await_expr(
        &mut self,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        if matches!(
            expr,
            ResolvedExpr::Number(_)
                | ResolvedExpr::DecimalNumber(_)
                | ResolvedExpr::BigIntLiteral { .. }
                | ResolvedExpr::String(_)
                | ResolvedExpr::Bool(_)
                | ResolvedExpr::Null
                | ResolvedExpr::Undefined
        ) {
            return self.lower_expr(expr);
        }
        Ok(LoweredExpr::PromiseGetValue {
            promise: Box::new(self.lower_expr(expr)?),
            span: Span::generated("promise_get_value"),
        })
    }

    pub(super) fn lower_this_expr(&mut self) -> Result<LoweredExpr, Diagnostic> {
        match self.resolve_local("this") {
            Ok(local) if self.ctx.facts.env_cell_locals.contains(&local) => Ok(
                LoweredExpr::EnvCellGet(local, Span::generated("env_cell_get")),
            ),
            Ok(local) => Ok(LoweredExpr::Local(local, Span::generated("local"))),
            Err(_) if self.ctx.classes.static_block_this_class.is_some() => {
                let class_name = self
                    .ctx
                    .classes
                    .static_block_this_class
                    .clone()
                    .unwrap_or_default();
                Ok(LoweredExpr::ClassPrototype(
                    self.class_prototype_ref(&class_name)?,
                    Span::generated("class_static_this"),
                ))
            }
            Err(_) => Ok(LoweredExpr::Undefined(Span::generated("undef"))),
        }
    }

    pub(super) fn lower_new_target_expr(&mut self, span: Span) -> Result<LoweredExpr, Diagnostic> {
        if let Ok(local_id) =
            self.resolve_local(crate::lowered::program::SYNTHETIC_NEW_TARGET_PARAM)
        {
            return Ok(LoweredExpr::Local(local_id, span));
        }
        // NewTargetPropagate: constructor scopes expose new.target and arrows inherit it.
        let class_name = self.ctx.classes.new_target_class.clone().or_else(|| {
            (self.ctx.classes.in_constructor)
                .then(|| self.ctx.classes.current_class.clone())
                .flatten()
        });
        let Some(class_name) = class_name else {
            return Ok(LoweredExpr::Undefined(span));
        };
        Ok(LoweredExpr::ClassPrototype(
            self.class_prototype_ref(&class_name)?,
            span,
        ))
    }

    pub(super) fn lower_ident_expr(&mut self, name: &str) -> Result<LoweredExpr, Diagnostic> {
        use ts2wasm_runtime_abi::ValueTag;
        if name == "Infinity" {
            return Ok(LoweredExpr::Number(
                ValueTag::INFINITY_PAYLOAD << ValueTag::NUMBER_SHIFT | ValueTag::NUMBER,
                Span::generated("infinity"),
            ));
        }
        if name == "NaN" {
            return Ok(LoweredExpr::Number(
                ValueTag::NAN_PAYLOAD << ValueTag::NUMBER_SHIFT | ValueTag::NUMBER,
                Span::generated("nan"),
            ));
        }
        if name == "globalThis" {
            return Ok(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::GlobalThis,
                args: Vec::new(),
                span: Span::generated("globalThis"),
            });
        }
        if let Some(token) = crate::lowered::program_builtins::builtin_function_token_expr(
            name,
            Span::generated("builtin_function"),
        ) {
            return Ok(token);
        }
        if name == "Number"
            || name == "Boolean"
            || name == "Math"
            || name == "Object"
            || name == "String"
            || name == "Function"
            || name == "JSON"
            || name == "Array"
            || name == "BigInt"
            || name == "Date"
            || name == "RegExp"
            || name == "Map"
            || name == "Set"
            || name == "Symbol"
            || name == "Promise"
            || name == "console"
            || name == "process"
            || name == "Buffer"
            || name == "WeakMap"
            || name == "WeakSet"
            || name == "Atomics"
            || name == "Intl"
            || name == "ArrayBuffer"
            || name == "SharedArrayBuffer"
            || name == "DataView"
            || name == "Int8Array"
            || name == "Uint8Array"
            || name == "Uint8ClampedArray"
            || name == "Int16Array"
            || name == "Uint16Array"
            || name == "Int32Array"
            || name == "Uint32Array"
            || name == "Float32Array"
            || name == "Float64Array"
            || name == "BigInt64Array"
            || name == "BigUint64Array"
            || name == "Float16Array"
            || name == "escape"
            || name == "unescape"
            || name == "Reflect"
            || name == "Proxy"
            || name == "isNaN"
            || name == "isFinite"
            || name == "encodeURI"
            || name == "decodeURI"
            || name == "encodeURIComponent"
            || name == "decodeURIComponent"
            || name == "parseInt"
            || name == "parseFloat"
            || name == "Error"
            || name == "require"
            || name == "__ts2wasm_dynamic_import"
            || name == "exports"
            || name == "module"
            || name == "global"
            || name == "setTimeout"
            || name == "clearTimeout"
            || name == "setInterval"
            || name == "clearInterval"
            || name == "setImmediate"
            || name == "print"
            || name == "performance"
            || name == "queueMicrotask"
            || name == "structuredClone"
            || name == "Bun"
            || name == "ERROR"
            || name == "verifyProperty"
            || name == "verifyWritable"
            || name == "verifyNotWritable"
            || name == "verifyEnumerable"
            || name == "verifyNotEnumerable"
            || name == "verifyConfigurable"
            || name == "verifyNotConfigurable"
            || name == "isSameValue"
            || name == "isEqualTo"
            || name == "isConstructor"
            || name == "compareArray"
            || name == "isPrimitive"
            || name == "fnGlobalObject"
            || name == "$DONE"
            || name == "$262"
            || name == "Test262Error"
            || name == "$ERROR"
            || name == "$DONOTEVALUATE"
            || name == "assert"
            || name == "asyncTest"
            || name == "$MAX_ITERATIONS"
            || name == "__assert_throws"
            || name == "TypeError"
            || name == "SyntaxError"
            || name == "RangeError"
            || name == "ReferenceError"
            || name == "URIError"
            || name == "EvalError"
            || name == "AggregateError"
            || name == "Temporal"
            || name == "WebAssembly"
            || name == "FinalizationRegistry"
            || name == "WeakRef"
            || name == "Iterator"
            || name == "AsyncIterator"
            || name == "GeneratorFunction"
            || name == "AsyncFunction"
            || name == "AsyncGeneratorFunction"
            || name == "SuppressedError"
            || name == "DisposableStack"
            || name == "AsyncDisposableStack"
            || name == "ShadowRealm"
            || name == "createRealm"
            || name == "detachArrayBuffer"
            || name == "TypedArray"
            || name == "undefined"
            || name == "eval"
        {
            return Ok(LoweredExpr::Undefined(Span::generated("undef")));
        }
        if name == "super" {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-5255: super property access is not supported in this milestone"
                    .to_owned(),
                span: Some(Span::generated("issue-5255")),
                phase: None,
            });
        }
        match self.resolve_local(name) {
            Ok(local) if self.ctx.facts.env_cell_locals.contains(&local) => {
                Ok(LoweredExpr::EnvCellGet(local, Span::generated("env_cell_get")))
            }
            Ok(local) => Ok(LoweredExpr::Local(local, Span::generated("local"))),
            Err(_) if name == "arguments" => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-062d: `arguments` is only supported inside non-arrow functions in this milestone"
                        .to_owned(),
                span: Some(Span::generated("issue-062d")),
                phase: None,
            }),
            Err(_) if self.ctx.classes.class_constructor_ids.contains_key(name) => {
                Ok(LoweredExpr::ClassPrototype(
                    self.class_prototype_ref(name)?,
                    Span::generated("class_proto"),
                ))
            }
            Err(err) => {
                let Ok(func_id) = self.resolve_func(name) else {
                    return Err(err);
                };
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
                    span: Span::generated("arrow_fn"),
                })
            }
        }
    }

    pub(super) fn lower_spread_expr(&self) -> Result<LoweredExpr, Diagnostic> {
        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-274: spread expressions are only supported in call arguments over literal arrays in this milestone"
                    .to_owned(),
            span: Some(Span::generated("issue-274")),
            phase: None,
        })
    }
}
