use super::{
    Resolver, is_array_prototype_push_property, is_private_field_storage_key,
    is_set_prototype_property, is_set_prototype_property_expr,
    private_storage_observable_access_diagnostic,
};
use crate::builtin::{BuiltinId, BuiltinPropertyId};
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_shared::{BinaryOp, UnaryOp};
use ts2wasm_shared::{DiagCode, Diagnostic, Span};

impl<'a> Resolver<'a> {
    pub(crate) fn lower_expr(&mut self, expr: &ResolvedExpr) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            ResolvedExpr::Number(value) => Ok(LoweredExpr::Number(*value, Span::generated("num"))),
            ResolvedExpr::BigIntLiteral {
                decimal,
                sign,
                limb_low,
                limb_high,
            } => Ok(LoweredExpr::BigIntLiteral {
                decimal: decimal.clone(),
                sign: *sign,
                limb_low: *limb_low,
                limb_high: *limb_high,
                span: Span::generated("bigint"),}),
            ResolvedExpr::String(value) => Ok(LoweredExpr::String(value.clone(), Span::generated("str"))),
            ResolvedExpr::Bool(value) => Ok(LoweredExpr::Bool(*value, Span::generated("bool"))),
            ResolvedExpr::Null => Ok(LoweredExpr::Null(Span::generated("null"))),
            ResolvedExpr::Undefined => Ok(LoweredExpr::Undefined(Span::generated("undef"))),
            ResolvedExpr::Await { expr } => {
                // await expr: extract the resolved value from the promise
                Ok(LoweredExpr::PromiseGetValue {
                    promise: Box::new(self.lower_expr(expr)?),
                    span: Span::generated("promise_get_value"),
                })
            }
            ResolvedExpr::This { span: _ } => match self.resolve_local("this") {
                Ok(local) => Ok(LoweredExpr::Local(local, Span::generated("local"))),
                Err(_) => {
                    // Top-level `this` in modules resolves to `undefined`
                    Ok(LoweredExpr::Undefined(Span::generated("undef")))
                }
            },
            ResolvedExpr::NewTarget { span } => {
                if !self.classes.in_constructor {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-236: new.target is only supported in class constructors"
                            .to_owned(),
                        span: Some(*span),
                        phase: None,
                    });
                }
                let class_name = self.classes.current_class.clone().ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-236: new.target requires a class constructor context".to_owned(),
                    span: Some(*span),
                    phase: None,
                })?;
                Ok(LoweredExpr::ClassPrototype(
                    self.class_prototype_ref(&class_name)?,
                    *span,
                ))
            }
            ResolvedExpr::Ident(name) => {
                // Handle special global constants Infinity and NaN
                // Note: These are approximated as max/min representable numbers due to small-int number model
                // Proper Infinity/NaN support requires broader number-model support (issue-281)
                use ts2wasm_runtime_abi::ValueTag;
                if name == "Infinity" {
                    return Ok(LoweredExpr::Number(ValueTag::INFINITY_PAYLOAD << ValueTag::NUMBER_SHIFT | ValueTag::NUMBER, Span::generated("infinity")));
                }
                if name == "NaN" {
                    return Ok(LoweredExpr::Number(ValueTag::NAN_PAYLOAD << ValueTag::NUMBER_SHIFT | ValueTag::NUMBER, Span::generated("nan")));
                }
                if name == "globalThis" {
                    // globalThis resolves to undefined in the current WASM model
                    // Full global object semantics require broader runtime support
                    return Ok(LoweredExpr::Undefined(Span::generated("undef")));
                }
                // Builtin global constructor/namespace identifiers used as values
                // These resolve to Undefined since their value semantics aren't needed
                // for standalone references (they're meaningful only as call targets or member containers)
                if name == "Number" || name == "Boolean" || name == "Math"
                    || name == "Object" || name == "String" || name == "Function" || name == "JSON"
                    || name == "Array" || name == "BigInt" || name == "Date"
                    || name == "RegExp" || name == "Error" || name == "Map"
                    || name == "Set" || name == "Symbol" || name == "Promise"
                    || name == "console" || name == "process" || name == "Buffer"
                    || name == "TypeError" || name == "ReferenceError" || name == "SyntaxError"
                    || name == "RangeError" || name == "URIError" || name == "EvalError"
                    || name == "AggregateError"
                    || name == "WeakMap" || name == "WeakSet"
                    || name == "Atomics" || name == "Intl" || name == "globalThis"
                    // TypedArray/ArrayBuffer/DataView constructors
                    || name == "ArrayBuffer" || name == "DataView"
                    || name == "Int8Array" || name == "Uint8Array"
                    || name == "Uint8ClampedArray"
                    || name == "Int16Array" || name == "Uint16Array"
                    || name == "Int32Array" || name == "Uint32Array"
                    || name == "Float32Array" || name == "Float64Array"
                    || name == "BigInt64Array" || name == "BigUint64Array"
                    // Global functions registered as BuiltinId (callable via builtin_resolver)
                    // but treated as Undefined when referenced as bare identifiers or property targets
                    || name == "escape" || name == "unescape"
                    || name == "Reflect" || name == "Proxy"
                    || name == "isNaN" || name == "parseInt" || name == "parseFloat" || name == "isFinite"
                    || name == "encodeURI" || name == "decodeURI"
                {
                    return Ok(LoweredExpr::Undefined(Span::generated("undef")));
                }
                if name == "super" {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-5255: super property access is not supported in this milestone".to_owned(),
                        span: None,

                        phase: None,});
                }
                match self.resolve_local(name) {
                    Ok(local) if self.captures.env_cell_locals.contains(&local) => Ok(LoweredExpr::EnvCellGet(local, Span::generated("env_cell_get"))),
                    Ok(local) => Ok(LoweredExpr::Local(local, Span::generated("local"))),
                    Err(_) if name == "arguments" => Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-062d: `arguments` is only supported inside non-arrow functions in this milestone".to_owned(),
                        span: None,

                        phase: None,}),
                    Err(_) if self.classes.class_constructor_ids.contains_key(name.as_str()) => {
                        Ok(LoweredExpr::ClassPrototype(self.class_prototype_ref(name)?, Span::generated("class_proto")))
                    }
                    Err(err) => Err(err),
                }
            }
            ResolvedExpr::Spread(_) => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-274: spread expressions are only supported in call arguments over literal arrays in this milestone".to_owned(),
                span: None,

                phase: None,}),
            ResolvedExpr::Unary { op, expr } => {
                // Handle -Infinity specially to ensure it returns the minimum representable number
                if *op == UnaryOp::Negate {
                    if let ResolvedExpr::Ident(name) = expr.as_ref() && name == "Infinity" {
                        use ts2wasm_runtime_abi::ValueTag;
                        return Ok(LoweredExpr::Number(ValueTag::NUMBER_PAYLOAD_MIN, Span::generated("num")));
                    }
                    if self.resolved_expr_is_bigint(expr) {
                        return Ok(LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeIntrinsic::BigIntUnaryMinus,
                            args: vec![self.lower_expr(expr)?],

                            span: Span::generated("runtime_call"),});
                    }
                }
                if *op == UnaryOp::BitwiseNot && self.resolved_expr_is_bigint(expr) {
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntBitwiseNot,
                        args: vec![self.lower_expr(expr)?],

                        span: Span::generated("runtime_call"),});
                }
                if *op == UnaryOp::Delete {
                    // Lower delete to PropertyDelete or PropertyDeleteDynamic
                    match expr.as_ref() {
                        ResolvedExpr::PropertyAccess {
                            object,
                            key,
                            span,
                        } => {
                            if is_private_field_storage_key(key) {
                                return Err(private_storage_observable_access_diagnostic(Some(
                                    *span,
                                )));
                            }
                            if key.starts_with('#') {
                                return Err(Diagnostic {
                                    code: DiagCode::UnsupportedSyntax,
                                    message: format!(
                                        "issue-255: private member `{key}` cannot be deleted in this private class runtime slice"
                                    ),
                                    span: Some(*span),

                                    phase: None,});
                            }
                            Ok(LoweredExpr::PropertyDelete {
                                object: Box::new(self.lower_expr(object)?),
                                key: key.clone(),
                                span: Span::generated("prop_delete"),})
                        }
                        ResolvedExpr::ComputedIndex { object, index } => {
                            if self.expr_has_private_progress_storage(object) {
                                return Err(private_storage_observable_access_diagnostic(None));
                            }
                            if let ResolvedExpr::String(key) = index.as_ref()
                                && is_private_field_storage_key(key)
                            {
                                return Err(private_storage_observable_access_diagnostic(None));
                            }
                            Ok(LoweredExpr::PropertyDeleteDynamic {
                                object: Box::new(self.lower_expr(object)?),
                                key: Box::new(self.lower_expr(index)?),
                                span: Span::generated("prop_delete_dyn"),})
                        }
                        _ => Ok(LoweredExpr::Unary {
                            op: lower_unary_op(*op)?,
                            expr: Box::new(self.lower_expr(expr)?),

                            span: Span::generated("unary"),}),
                    }
                } else {
                    Ok(LoweredExpr::Unary {
                        op: lower_unary_op(*op)?,
                        expr: Box::new(self.lower_expr(expr)?),
                        span: Span::generated("unary"),})
                }
            }
            ResolvedExpr::Binary { left, op, right } => {
                if *op == BinaryOp::InstanceOf {
                    let prototype = match right.as_ref() {
                        ResolvedExpr::Ident(name) => {
                            if let Some(constructor) = BuiltinErrorConstructor::from_name(name) {
                                LoweredExpr::BuiltinErrorPrototype(constructor, Span::generated("builtin_error_proto"))
                            } else {
                                self.class_prototype_ref(name)
                                    .map(|p| LoweredExpr::ClassPrototype(p, Span::generated("class_proto")))?
                            }
                        }
                        _ => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "issue-207: instanceof right-hand side must be a supported class constructor".to_owned(),
                                span: None,

                                phase: None,});
                        }
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::InstanceOf,
                        args: vec![self.lower_expr(left)?, prototype],

                        span: Span::generated("runtime_call"),})
                } else if *op == BinaryOp::In {
                    // Lower in to PropertyIn or PropertyInDynamic
                    // key in object -> check if key exists in object
                    match left.as_ref() {
                        ResolvedExpr::Number(index) => Ok(LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeIntrinsic::ArrayIndexPresent,
                            args: vec![self.lower_expr(right)?, LoweredExpr::Number(*index, Span::generated("num"))],

                            span: Span::generated("runtime_call"),}),
                        ResolvedExpr::String(key) => Ok(LoweredExpr::PropertyIn {
                            obj: Box::new(self.lower_expr(right)?),
                            key: key.clone(),
                            span: Span::generated("prop_in"),}),
                        _ => Ok(LoweredExpr::PropertyInDynamic {
                            obj: Box::new(self.lower_expr(right)?),
                            key: Box::new(self.lower_expr(left)?),
                            span: Span::generated("prop_in_dyn"),}),
                    }
                } else if matches!(op, BinaryOp::Divide | BinaryOp::Modulo)
                    && self.resolved_expr_is_bigint_div_rem_operand(left)
                    && self.resolved_expr_is_bigint_div_rem_operand(right)
                {
                    let intrinsic = match op {
                        BinaryOp::Divide => RuntimeIntrinsic::BigIntDiv,
                        BinaryOp::Modulo => RuntimeIntrinsic::BigIntRem,
                        _ => unreachable!("checked above"),
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        intrinsic: intrinsic,
                        args: vec![self.lower_expr(left)?, self.lower_expr(right)?],

                        span: Span::generated("runtime_call"),})
                } else if matches!(op, BinaryOp::Divide | BinaryOp::Modulo)
                    && ((self.resolved_expr_is_control_flow_mixed_bigint(left)
                        && self.resolved_expr_is_bigint_div_rem_operand(right))
                        || (self.resolved_expr_is_bigint_div_rem_operand(left)
                            && self.resolved_expr_is_control_flow_mixed_bigint(right)))
                {
                    Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "issue-370: mixed Number/BigInt arithmetic TypeError parity is not implemented in the control-flow BigInt div/rem slice"
                                .to_owned(),
                        span: None,

                        phase: None,})
                } else if matches!(
                    op,
                    BinaryOp::Add
                        | BinaryOp::Subtract
                        | BinaryOp::Multiply
                        | BinaryOp::Divide
                        | BinaryOp::Modulo
                )
                    && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
                {
                    let intrinsic = match op {
                        BinaryOp::Add => RuntimeIntrinsic::BigIntAdd,
                        BinaryOp::Subtract => RuntimeIntrinsic::BigIntSub,
                        BinaryOp::Multiply => RuntimeIntrinsic::BigIntMul,
                        BinaryOp::Divide => RuntimeIntrinsic::BigIntDiv,
                        BinaryOp::Modulo => RuntimeIntrinsic::BigIntRem,
                        _ => unreachable!("checked above"),
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        intrinsic: intrinsic,
                        args: vec![self.lower_expr(left)?, self.lower_expr(right)?],

                        span: Span::generated("runtime_call"),})
                } else if *op == BinaryOp::Power
                    && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
                {
                    Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntPow,
                        args: vec![self.lower_expr(left)?, self.lower_expr(right)?],

                        span: Span::generated("runtime_call"),})
                } else if matches!(
                    op,
                    BinaryOp::Add
                        | BinaryOp::Subtract
                        | BinaryOp::Multiply
                        | BinaryOp::Divide
                        | BinaryOp::Modulo
                        | BinaryOp::Power
                ) && (self.resolved_expr_is_bigint(left) || self.resolved_expr_is_bigint(right))
                {
                    // For Add, emit regular $add which handles string concat
                    // with BigInt at runtime via $value_to_string_into,
                    // and BigInt+Number via TypeError check in $add.
                    // For other mixed BigInt/non-BigInt arithmetic, emit TypeError.
                    if *op == BinaryOp::Add {
                        Ok(LoweredExpr::Binary {
                            left: Box::new(self.lower_expr(left)?),
                            op: LoweredBinaryOp::Add,
                            right: Box::new(self.lower_expr(right)?),

                            span: Span::generated("binary"),})
                    } else {
                        Ok(LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeIntrinsic::BigIntMixedArithmeticTypeError,
                            args: vec![self.lower_expr(left)?, self.lower_expr(right)?],

                            span: Span::generated("runtime_call"),})
                    }
                } else if matches!(
                    op,
                    BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor
                ) && self.resolved_expr_is_bigint(left)
                    && self.resolved_expr_is_bigint(right)
                {
                    let intrinsic = match op {
                        BinaryOp::BitwiseAnd => RuntimeIntrinsic::BigIntBitwiseAnd,
                        BinaryOp::BitwiseOr => RuntimeIntrinsic::BigIntBitwiseOr,
                        BinaryOp::BitwiseXor => RuntimeIntrinsic::BigIntBitwiseXor,
                        _ => unreachable!("checked above"),
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        intrinsic: intrinsic,
                        args: vec![self.lower_expr(left)?, self.lower_expr(right)?],

                        span: Span::generated("runtime_call"),})
                } else if matches!(
                    op,
                    BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor
                ) && (self.resolved_expr_is_bigint(left) || self.resolved_expr_is_bigint(right))
                    && !(self.resolved_expr_is_bigint(left)
                        && self.resolved_expr_is_bigint(right))
                {
                    // Mixed BigInt/non-BigInt bitwise → TypeError
                    Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::BigIntMixedArithmeticTypeError,
                        args: vec![self.lower_expr(left)?, self.lower_expr(right)?],

                        span: Span::generated("runtime_call"),})
                } else if matches!(
                    op,
                    BinaryOp::LeftShift | BinaryOp::RightShift | BinaryOp::UnsignedRightShift
                ) && self.resolved_expr_is_bigint(left)
                {
                    // BigInt unsigned right shift (>>>) always throws
                    // TypeError; use the mixed-arithmetic type error.
                    if *op == BinaryOp::UnsignedRightShift {
                        Ok(LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeIntrinsic::BigIntMixedArithmeticTypeError,
                            args: vec![self.lower_expr(left)?, self.lower_expr(right)?],

                            span: Span::generated("runtime_call"),})
                    } else if self.resolved_expr_is_bigint(right) {
                        let intrinsic = match op {
                            BinaryOp::LeftShift => RuntimeIntrinsic::BigIntLeftShift,
                            BinaryOp::RightShift => RuntimeIntrinsic::BigIntRightShift,
                            _ => unreachable!("checked above"),
                        };
                        Ok(LoweredExpr::RuntimeCall {
                            intrinsic: intrinsic,
                            args: vec![self.lower_expr(left)?, self.lower_expr(right)?],

                            span: Span::generated("runtime_call"),})
                    } else {
                        // Mixed BigInt/non-BigInt shift → TypeError
                        Ok(LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeIntrinsic::BigIntMixedArithmeticTypeError,
                            args: vec![self.lower_expr(left)?, self.lower_expr(right)?],

                            span: Span::generated("runtime_call"),})
                    }
                } else {
                    Ok(LoweredExpr::Binary {
                        left: Box::new(self.lower_expr(left)?),
                        op: lower_binary_op(*op)?,
                        right: Box::new(self.lower_expr(right)?),

                        span: Span::generated("binary"),})
                }
            }
            ResolvedExpr::Assign { name, expr } => {
                let local = self.resolve_local(name)?;
                self.invalidate_static_object_literal_local(local);
                self.invalidate_static_function_array_like_local(local);
                let expr = Box::new(self.lower_expr(expr)?);
                if self.captures.env_cell_locals.contains(&local) {
                    Ok(LoweredExpr::EnvCellSet { cell: local, expr ,
                    span: Span::generated("env_cell_set"),})
                } else {
                    Ok(LoweredExpr::Assign { local, expr , span: Span::generated("assign")})
                }
            }
            ResolvedExpr::LogicalAssign { name, op, expr } => {
                let local = self.resolve_local(name)?;
                self.invalidate_static_object_literal_local(local);
                self.invalidate_static_function_array_like_local(local);
                Ok(LoweredExpr::LogicalAssign {
                    local,
                    op: lower_logical_assign_op(*op),
                    expr: Box::new(self.lower_expr(expr)?),
                    span: Span::generated("logical_assign"),})
            }
            ResolvedExpr::LogicalPropertyAssign {
                object,
                key,
                op,
                expr,
            } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                let object = self.resolve_local(object)?;
                self.invalidate_static_object_literal_local(object);
                Ok(LoweredExpr::LogicalPropertyAssign {
                    object,
                    key: key.clone(),
                    op: lower_logical_assign_op(*op),
                    expr: Box::new(self.lower_expr(expr)?),
                    span: Span::generated("logical_prop_assign"),})
            }
            ResolvedExpr::LogicalComputedPropertyAssign {
                object,
                key,
                op,
                expr,
            } => {
                let object = self.resolve_local(object)?;
                self.invalidate_static_object_literal_local(object);
                if self.local_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::LogicalComputedPropertyAssign {
                    object,
                    key: Box::new(self.lower_expr(key)?),
                    op: lower_logical_assign_op(*op),
                    expr: Box::new(self.lower_expr(expr)?),
                    span: Span::generated("logical_comp_prop_assign"),})
            }
            ResolvedExpr::LogicalComputedMemberAssign {
                object,
                key,
                op,
                expr,
            } => Ok(LoweredExpr::LogicalComputedMemberAssign {
                object: {
                    if self.expr_has_private_progress_storage(object) {
                        return Err(private_storage_observable_access_diagnostic(None));
                    }
                    Box::new(self.lower_expr(object)?)
                },
                key: Box::new(self.lower_expr(key)?),
                op: lower_logical_assign_op(*op),
                expr: Box::new(self.lower_expr(expr)?),
                span: Span::generated("logical_comp_member_assign"),}),
            ResolvedExpr::LogicalMemberAssign {
                object,
                key,
                op,
                expr,
            } => Ok(LoweredExpr::LogicalMemberAssign {
                object: Box::new(self.lower_expr(object)?),
                key: key.clone(),
                op: lower_logical_assign_op(*op),
                expr: Box::new(self.lower_expr(expr)?),
                span: Span::generated("logical_member_assign"),}),
            ResolvedExpr::Ternary {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                let result = self.alloc_temp();
                Ok(LoweredExpr::Block {
                    stmts: vec![
                        LoweredStmt::Let(result, LoweredExpr::Undefined(Span::generated("undef")), Span::generated("let_stmt")),
                        LoweredStmt::If {
                            condition: self.lower_expr(condition)?,
                            then_body: vec![LoweredStmt::Assign(
                                result,
                                self.lower_expr(then_expr)?,
                            Span::generated("assign"))],
                            else_body: vec![LoweredStmt::Assign(
                                result,
                                self.lower_expr(else_expr)?,
                            Span::generated("assign"))],
                            span: Span::generated("if_stmt"),},
                    ],
                    result: Box::new(LoweredExpr::Local(result, Span::generated("local"))),
                    span: Span::generated("block"),})
            }
            ResolvedExpr::Call { callee, args, span } => {
                self.lower_call_expr(callee, args, *span)
            }
            ResolvedExpr::BuiltinCall { builtin, args } => {
                let mut lowered_args = args
                    .iter()
                    .map(|arg| self.lower_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                // ParseInt accepts an optional second argument (radix).
                // When omitted, default to 0 (auto-detect radix) per JS semantics.
                if *builtin == BuiltinId::ParseInt && lowered_args.len() == 1 {
                    lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
                }
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(*builtin),
                    args: lowered_args,

                    span: Span::generated("call"),})
            }
            ResolvedExpr::BuiltinProperty {
                builtin,
                object,
                span,
            } => match builtin {
                BuiltinPropertyId::Length => match object.as_ref() {
                    ResolvedExpr::Ident(name) if self.resolve_func(name).is_ok() => {
                        self.lower_function_metadata_property(name, "length", *span)
                    }
                    ResolvedExpr::Ident(name) if is_global_builtin_function_name(name) => {
                        lower_global_builtin_function_metadata_property(name, "length")
                    }
                    _ => Ok(LoweredExpr::GetLength(Box::new(self.lower_expr(object)?), Span::generated("get_length"))),
                },
            },
            ResolvedExpr::PropertyAccess { object, key, span } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(Some(*span)));
                }
                if matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "super") {
                    let class_name = self.classes.current_class.as_ref().ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "super property access requires class context".to_owned(),
                        span: Some(*span),

                        phase: None,})?;
                    let parent_name = self
                        .classes.class_parents
                        .get(class_name)
                        .and_then(|p| p.clone())
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super property access used in class without extends"
                                .to_owned(),
                            span: Some(*span),

                            phase: None,})?;
                    let parent_ref = self.class_prototype_ref(&parent_name)?;
                    return Ok(LoweredExpr::PropertyGet {
                        obj: Box::new(LoweredExpr::ClassPrototype(
                            parent_ref,
                            Span::generated("class_proto"),
                        )),
                        key: key.clone(),
                        span: Span::generated("super_prop_get"),
                    });
                }
                if is_array_prototype_push_property(object, key) {
                    return Ok(LoweredExpr::Number(0, Span::generated("num")));
                }
                if key.starts_with('#') {
                    if let Some(local_name) = self.current_static_private_field_local_name(key) {
                        if self.is_same_class_static_private_receiver(object) {
                            let local = self.resolve_local(&local_name).map_err(|_| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-352: static private field `{key}` cannot be accessed before its declaration in class static initialization order"
                                ),
                                span: Some(*span),

                                phase: None,})?;
                            return Ok(if self.captures.env_cell_locals.contains(&local) {
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
                            span: Some(*span),

                            phase: None,});
                    }
                    if let Some(getter_id) = self.current_static_private_getter_id(key) {
                        if self.is_same_class_static_private_receiver(object) {
                            return Ok(LoweredExpr::Call {
                                kind: FunctionCallKind::User(getter_id),
                                args: Vec::new(),

                                span: Span::generated("call"),});
                        }
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: static private getter `{key}` access is currently supported only as `this.{key}` inside static methods or `Class.{key}` inside the declaring class"
                            ),
                            span: Some(*span),

                            phase: None,});
                    }
                    if self.current_private_method_id(key).is_some() {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private method `{key}` extraction is not supported in this private method runtime slice; call it directly as `this.{key}(...)`"
                            ),
                            span: Some(*span),

                            phase: None,});
                    }
                    if let Some(getter_id) = self.current_private_getter_id(key) {
                        let receiver = if matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                            LoweredExpr::Local(self.resolve_local("this")?, Span::generated("local"))
                        } else {
                            let class_name = self.classes.current_class.clone().ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-255: private getter `{key}` access requires declaring class context"
                                ),
                                span: Some(*span),

                                phase: None,})?;
                            let brand = self.private_brand_for_class(&class_name, Some(*span))?;
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeIntrinsic::PrivateBrandCheck,
                                args: vec![
                                    self.lower_expr(object)?,
                                    LoweredExpr::Number(brand as i32, Span::generated("num")),
                                ],

                                span: Span::generated("runtime_call"),}
                        };
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(getter_id),
                            args: vec![receiver],

                            span: Span::generated("call"),});
                    }
                    if let Some(class_name) = self.infer_class_for_expr(object)
                        && self.private_getter_id_for_class(&class_name, key).is_some()
                    {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private getter `{key}` external access is not supported in this private accessor runtime slice"
                            ),
                            span: Some(*span),

                            phase: None,});
                    }
                    let (brand, slot) = self.private_field_brand_and_slot(object, key, *span)?;
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::PrivateFieldGet,
                        args: vec![
                            self.lower_expr(object)?,
                            LoweredExpr::Number(brand as i32, Span::generated("num")),
                            LoweredExpr::Number(slot as i32, Span::generated("num")),
                        ],

                        span: Span::generated("runtime_call"),});
                }
                if let ResolvedExpr::Ident(name) = object.as_ref()
                    && self.resolve_func(name).is_ok()
                {
                    return self.lower_function_metadata_property(name, key, *span);
                }
                if let ResolvedExpr::Ident(name) = object.as_ref()
                    && is_global_builtin_function_name(name)
                    && matches!(key.as_str(), "name" | "length")
                {
                    return lower_global_builtin_function_metadata_property(name, key);
                }
                if key == "size"
                    && let ResolvedExpr::Ident(receiver_name) = object.as_ref()
                    && let Ok(obj_local) = self.resolve_local(receiver_name)
                    && self
                        .classes.local_classes
                        .get(&obj_local)
                        .is_some_and(|class_name| class_name == "Set")
                {
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::SetSize,
                        args: vec![LoweredExpr::Local(obj_local, Span::generated("local"))],

                        span: Span::generated("runtime_call"),});
                }
                if key == "size"
                    && let ResolvedExpr::Ident(receiver_name) = object.as_ref()
                    && let Ok(obj_local) = self.resolve_local(receiver_name)
                    && self
                        .classes.local_classes
                        .get(&obj_local)
                        .is_some_and(|class_name| class_name == "Map")
                {
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::MapSize,
                        args: vec![LoweredExpr::Local(obj_local, Span::generated("local"))],
                        span: Span::generated("runtime_call"),});
                }
                if is_set_prototype_property(object, key, "add") {
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::SetPrototypeAddGet,
                        args: Vec::new(),

                        span: Span::generated("runtime_call"),});
                }
                Ok(LoweredExpr::PropertyGet {
                    obj: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                    span: Span::generated("prop_get"),})
            },
            ResolvedExpr::OptionalPropertyAccess { object, key, .. } => {
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::OptionalPropertyGet {
                    obj: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                    span: Span::generated("opt_prop_get"),})
            }
            ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                Ok(LoweredExpr::OptionalIndex {
                    object: Box::new(self.lower_expr(object)?),
                    index: Box::new(self.lower_expr(index)?),
                    span: Span::generated("opt_index"),})
            }
            ResolvedExpr::OptionalCall { callee, args, span } => {
                self.lower_optional_call(callee, args, *span)
            }
            ResolvedExpr::ComputedIndex { object, index } => {
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                // super['prop'] — look up on parent prototype using dynamic key
                if matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "super") {
                    let class_name = self.classes.current_class.as_ref().ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "super computed access requires class context".to_owned(),
                        span: None,

                        phase: None,})?;
                    let parent_name = self
                        .classes.class_parents
                        .get(class_name)
                        .and_then(|p| p.clone())
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super computed access used in class without extends"
                                .to_owned(),
                            span: None,

                            phase: None,})?;
                    let parent_ref = self.class_prototype_ref(&parent_name)?;
                    return Ok(LoweredExpr::PropertyGetDynamic {
                        obj: Box::new(LoweredExpr::ClassPrototype(
                            parent_ref,
                            Span::generated("class_proto"),
                        )),
                        key: Box::new(self.lower_expr(index)?),
                        span: Span::generated("super_index_get"),
                    });
                }
                // Lower the object first to determine its type
                let lowered_object = self.lower_expr(object)?;
                let lowered_index = self.lower_expr(index)?;

                // Keep obvious array literals on the compact array helper. Unknown
                // receivers must use the generic index helper so object[stringKey]
                // and array[numberIndex] both preserve JavaScript semantics.
                if matches!(object.as_ref(), ResolvedExpr::String(_)) {
                    Ok(LoweredExpr::Index {
                        object: Box::new(lowered_object),
                        index: Box::new(lowered_index),

                        span: Span::generated("index"),})
                } else if matches!(object.as_ref(), ResolvedExpr::Array(_))
                    || matches!(
                        lowered_object,
                        LoweredExpr::ArrayNew { .. } | LoweredExpr::ArrayNewSparse { .. }
                    )
                {
                    Ok(LoweredExpr::ArrayGet {
                        arr: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                        span: Span::generated("array_get"),})
                } else {
                    Ok(LoweredExpr::Index {
                        object: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                        span: Span::generated("index"),})
                }
            }
            ResolvedExpr::Array(elements) => {
                self.lower_array_literal(elements)
            }
            ResolvedExpr::Object(props) => {
                self.lower_object_literal_expr(props)
            }
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                span,
            } => {
                self.lower_method_call_expr(object, method, args, *span)
            }
            ResolvedExpr::PropertyAssign {
                object,
                key,
                value,
                span,
            } => {
                if let ResolvedExpr::Ident(name) = object.as_ref()
                    && let Ok(local_id) = self.resolve_local(name)
                {
                    self.invalidate_static_object_literal_local(local_id);
                }
                if is_private_field_storage_key(key) {
                    return Err(private_storage_observable_access_diagnostic(Some(*span)));
                }
                if is_set_prototype_property(object, key, "add") {
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::SetPrototypeAddSet,
                        args: vec![self.lower_set_prototype_add_assignment_value(value)?],

                        span: Span::generated("runtime_call"),});
                }
                if is_set_prototype_property(object, key, "originalAdd")
                    && is_set_prototype_property_expr(value, "add")
                {
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::SetPrototypeAddGet,
                        args: Vec::new(),

                        span: Span::generated("runtime_call"),});
                }
                if key.starts_with('#') {
                    if let Some(local_name) = self.current_static_private_field_local_name(key) {
                        if self.is_same_class_static_private_receiver(object) {
                            let local = self.resolve_local(&local_name).map_err(|_| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-352: static private field `{key}` cannot be accessed before its declaration in class static initialization order"
                                ),
                                span: Some(*span),

                                phase: None,})?;
                            let expr = Box::new(self.lower_expr(value)?);
                            return Ok(if self.captures.env_cell_locals.contains(&local) {
                                LoweredExpr::EnvCellSet { cell: local, expr ,
                                span: Span::generated("env_cell_set"),}
                            } else {
                                LoweredExpr::Assign { local, expr , span: Span::generated("assign")}
                            });
                        }
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: static private field `{key}` assignment is currently supported only as `this.{key} = value` inside static methods or `Class.{key} = value` inside the declaring class"
                            ),
                            span: Some(*span),

                            phase: None,});
                    }
                    if let Some(setter_id) = self.current_static_private_setter_id(key) {
                        if self.is_same_class_static_private_receiver(object) {
                            return Ok(LoweredExpr::Call {
                                kind: FunctionCallKind::User(setter_id),
                                args: vec![self.lower_expr(value)?],

                                span: Span::generated("call"),});
                        }
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: static private setter `{key}` assignment is currently supported only as `this.{key} = value` inside static methods or `Class.{key} = value` inside the declaring class"
                            ),
                            span: Some(*span),

                            phase: None,});
                    }
                    if let Some(setter_id) = self.current_private_setter_id(key) {
                        let receiver = if matches!(object.as_ref(), ResolvedExpr::This { .. }) {
                            LoweredExpr::Local(self.resolve_local("this")?, Span::generated("local"))
                        } else {
                            let class_name = self.classes.current_class.clone().ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-255: private setter `{key}` assignment requires declaring class context"
                                ),
                                span: Some(*span),

                                phase: None,})?;
                            let brand = self.private_brand_for_class(&class_name, Some(*span))?;
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeIntrinsic::PrivateBrandCheck,
                                args: vec![
                                    self.lower_expr(object)?,
                                    LoweredExpr::Number(brand as i32, Span::generated("num")),
                                ],

                                span: Span::generated("runtime_call"),}
                        };
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(setter_id),
                            args: vec![receiver, self.lower_expr(value)?],
                            span: Span::generated("call"),});
                    }
                    if let Some(class_name) = self.infer_class_for_expr(object)
                        && self.private_setter_id_for_class(&class_name, key).is_some()
                    {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-255: private setter `{key}` external assignment is not supported in this private setter runtime slice"
                            ),
                            span: Some(*span),

                            phase: None,});
                    }
                    let (brand, slot) = self.private_field_brand_and_slot(object, key, *span)?;
                    return Ok(LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeIntrinsic::PrivateFieldSet,
                        args: vec![
                            self.lower_expr(object)?,
                            LoweredExpr::Number(brand as i32, Span::generated("num")),
                            LoweredExpr::Number(slot as i32, Span::generated("num")),
                            self.lower_expr(value)?,
                        ],

                        span: Span::generated("runtime_call"),});
                }
                // super.prop = value — writes to `this`, not the parent prototype
                if matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "super") {
                    let class_name = self.classes.current_class.as_ref().ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "super property assignment requires class context".to_owned(),
                        span: Some(*span),

                        phase: None,})?;
                    let _parent_name = self
                        .classes.class_parents
                        .get(class_name)
                        .and_then(|p| p.clone())
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super property assignment used in class without extends"
                                .to_owned(),
                            span: Some(*span),

                            phase: None,})?;
                    return Ok(LoweredExpr::PropertySet {
                        object: Box::new(LoweredExpr::Local(
                            self.resolve_local("this")?,
                            Span::generated("local"),
                        )),
                        key: key.clone(),
                        value: Box::new(self.lower_expr(value)?),
                        span: Span::generated("super_prop_set"),
                    });
                }
                Ok(LoweredExpr::PropertySet {
                    object: Box::new(self.lower_expr(object)?),
                    key: key.clone(),
                    value: Box::new(self.lower_expr(value)?),
                    span: Span::generated("prop_set"),})
            }
            ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
                if let ResolvedExpr::Ident(name) = object.as_ref()
                    && let Ok(local_id) = self.resolve_local(name)
                {
                    self.invalidate_static_object_literal_local(local_id);
                    self.update_static_function_array_like_index(local_id, key, value);
                }
                if self.expr_has_private_progress_storage(object) {
                    return Err(private_storage_observable_access_diagnostic(None));
                }
                // super['prop'] = value — writes to `this`, not the parent prototype
                if matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "super") {
                    let class_name = self.classes.current_class.as_ref().ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "super computed assignment requires class context".to_owned(),
                        span: None,

                        phase: None,})?;
                    let _parent_name = self
                        .classes.class_parents
                        .get(class_name)
                        .and_then(|p| p.clone())
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super computed assignment used in class without extends"
                                .to_owned(),
                            span: None,

                            phase: None,})?;
                    return Ok(LoweredExpr::PropertySetDynamic {
                        object: Box::new(LoweredExpr::Local(
                            self.resolve_local("this")?,
                            Span::generated("local"),
                        )),
                        index: Box::new(self.lower_expr(key)?),
                        value: Box::new(self.lower_expr(value)?),
                        span: Span::generated("super_prop_set_dyn"),
                    });
                }
                Ok(LoweredExpr::PropertySetDynamic {
                    object: Box::new(self.lower_expr(object)?),
                    index: Box::new(self.lower_expr(key)?),
                    value: Box::new(self.lower_expr(value)?),
                    span: Span::generated("prop_set_dyn"),})
            }
            ResolvedExpr::New {
                class_name,
                args,
                span,
            } => {
                self.lower_new_expr(class_name, args, *span)
            }
            ResolvedExpr::ModuleLoad { specifier } => Ok(LoweredExpr::ModuleLoad {
                module_id: self.module_id_for_specifier(specifier),
                span: Span::generated("module_load"),}),
            ResolvedExpr::ArrowFn {
                params,
                body,
                body_stmts,
                ..
            } => self.lower_arrow_fn(params, body, body_stmts),
            ResolvedExpr::FunctionExpr { name, params, body } => {
                self.lower_named_function_expr(name, params, body)
            }
            ResolvedExpr::ClassExpr { .. } => {
                // issue-5248: placeholder for class expression lowering.
                Ok(LoweredExpr::Undefined(Span::generated("undef")))
            }
        }
    }
}

fn is_global_builtin_function_name(name: &str) -> bool {
    matches!(
        name,
        "escape"
            | "unescape"
            | "isNaN"
            | "parseInt"
            | "parseFloat"
            | "isFinite"
            | "encodeURI"
            | "decodeURI"
    )
}

fn lower_global_builtin_function_metadata_property(
    name: &str,
    key: &str,
) -> Result<LoweredExpr, Diagnostic> {
    match key {
        "name" => Ok(LoweredExpr::String(name.to_owned(), Span::generated("str"))),
        "length" => Ok(LoweredExpr::Number(
            global_builtin_function_length(name),
            Span::generated("num"),
        )),
        _ => unreachable!("caller filters global builtin function metadata property"),
    }
}

fn global_builtin_function_length(name: &str) -> i32 {
    match name {
        "parseInt" => 2,
        "escape" | "unescape" | "isNaN" | "parseFloat" | "isFinite" | "encodeURI" | "decodeURI" => {
            1
        }
        _ => 0,
    }
}
