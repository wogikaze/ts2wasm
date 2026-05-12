use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_await_expr(
        &mut self,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        Ok(LoweredExpr::PromiseGetValue {
            promise: Box::new(self.lower_expr(expr)?),
            span: Span::generated("promise_get_value"),
        })
    }

    pub(super) fn lower_this_expr(&mut self) -> Result<LoweredExpr, Diagnostic> {
        match self.resolve_local("this") {
            Ok(local) => Ok(LoweredExpr::Local(local, Span::generated("local"))),
            Err(_) => Ok(LoweredExpr::Undefined(Span::generated("undef"))),
        }
    }

    pub(super) fn lower_new_target_expr(
        &mut self,
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        if !self.ctx.classes.in_constructor {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-236: new.target is only supported in class constructors"
                    .to_owned(),
                span: Some(span),
                phase: None,
            });
        }
        let class_name = self.ctx.classes.current_class.clone().ok_or_else(|| {
            Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-236: new.target requires a class constructor context".to_owned(),
                span: Some(span),
                phase: None,
            }
        })?;
        Ok(LoweredExpr::ClassPrototype(
            self.class_prototype_ref(&class_name)?,
            span,
        ))
    }

    pub(super) fn lower_ident_expr(
        &mut self,
        name: &str,
    ) -> Result<LoweredExpr, Diagnostic> {
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
            return Ok(LoweredExpr::Undefined(Span::generated("undef")));
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
            || name == "Error"
            || name == "Map"
            || name == "Set"
            || name == "Symbol"
            || name == "Promise"
            || name == "console"
            || name == "process"
            || name == "Buffer"
            || name == "TypeError"
            || name == "ReferenceError"
            || name == "SyntaxError"
            || name == "RangeError"
            || name == "URIError"
            || name == "EvalError"
            || name == "AggregateError"
            || name == "WeakMap"
            || name == "WeakSet"
            || name == "Atomics"
            || name == "Intl"
            || name == "globalThis"
            || name == "ArrayBuffer"
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
            || name == "escape"
            || name == "unescape"
            || name == "Reflect"
            || name == "Proxy"
            || name == "isNaN"
            || name == "parseInt"
            || name == "parseFloat"
            || name == "isFinite"
            || name == "encodeURI"
            || name == "decodeURI"
        {
            return Ok(LoweredExpr::Undefined(Span::generated("undef")));
        }
        if name == "super" {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-5255: super property access is not supported in this milestone"
                    .to_owned(),
                span: None,
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
                span: None,
                phase: None,
            }),
            Err(_) if self.ctx.classes.class_constructor_ids.contains_key(name) => {
                Ok(LoweredExpr::ClassPrototype(
                    self.class_prototype_ref(name)?,
                    Span::generated("class_proto"),
                ))
            }
            Err(err) => Err(err),
        }
    }

    pub(super) fn lower_spread_expr(&self) -> Result<LoweredExpr, Diagnostic> {
        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-274: spread expressions are only supported in call arguments over literal arrays in this milestone"
                    .to_owned(),
            span: None,
            phase: None,
        })
    }
}
