use std::collections::BTreeMap;

use ts2wasm_ir::lowered::{LoweredExpr, LoweredFunction, LoweredProgram, LoweredStmt};
use ts2wasm_runtime_abi::{RuntimeString, ValueTag, layout::Layout};

use crate::emitter::function_symbol;
use crate::expr_emit::{
    CLOSURE_CAPTURE_COUNT_OFFSET, CLOSURE_CAPTURE_SLOT_SIZE, CLOSURE_CAPTURE_SLOTS_OFFSET,
    CLOSURE_CODE_ID_OFFSET, CLOSURE_SENTINEL, CLOSURE_SUBTYPE_OFFSET,
};
use crate::runtime::core::typed;
use crate::runtime_fn::RuntimeFn;
use crate::runtime_link_plan::build_runtime_link_plan;
use crate::wasm_ir::{WasmBlockType, WasmFunction, WasmInstr, WasmValType};

pub(crate) const NATIVE_MEMORY_MAX_PAGES: u32 = 4096;

#[derive(Clone, Debug, Default)]
pub(crate) struct NativeRuntimeData {
    pub(crate) strings: NativeRuntimeStringTable,
    pub(crate) property_get: PropertyGetData,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct NativeRuntimeStringTable {
    entries: BTreeMap<&'static str, RuntimeStringEntry>,
}

#[derive(Clone, Copy, Debug, Default)]
struct RuntimeStringEntry {
    value: i32,
    reference: RuntimeStringRef,
}

impl NativeRuntimeStringTable {
    pub(crate) fn insert(
        &mut self,
        value: &'static str,
        tagged_value: i32,
        reference: RuntimeStringRef,
    ) {
        self.entries.insert(
            value,
            RuntimeStringEntry {
                value: tagged_value,
                reference,
            },
        );
    }

    fn value(&self, value: &'static str) -> i32 {
        self.entries
            .get(value)
            .map(|entry| entry.value)
            .unwrap_or_default()
    }

    fn reference(&self, value: &'static str) -> RuntimeStringRef {
        self.entries
            .get(value)
            .map(|entry| entry.reference)
            .unwrap_or_default()
    }

    fn typeof_values(&self) -> TypeOfStringValues {
        TypeOfStringValues {
            undefined: self.value(RuntimeString::UNDEFINED),
            object: self.value("object"),
            boolean: self.value("boolean"),
            number: self.value("number"),
            string: self.value("string"),
            bigint: self.value("bigint"),
            function: self.value("function"),
            symbol: self.value("symbol"),
        }
    }

    fn value_to_string_refs(&self) -> ValueToStringRefs {
        ValueToStringRefs {
            undefined: self.reference(RuntimeString::UNDEFINED),
            null: self.reference(RuntimeString::NULL),
            false_: self.reference(RuntimeString::FALSE),
            true_: self.reference(RuntimeString::TRUE),
        }
    }

    fn log_newline(&self) -> RuntimeStringRef {
        self.reference(RuntimeString::NEWLINE)
    }

    fn boolean_to_string_values(&self) -> BooleanToStringValues {
        BooleanToStringValues {
            false_: self.value(RuntimeString::FALSE),
            true_: self.value(RuntimeString::TRUE),
        }
    }

    fn generator_return_data(&self) -> GeneratorReturnData {
        GeneratorReturnData {
            value_key: self.value("value"),
            done_key: self.value("done"),
        }
    }

    fn generator_yield_data(&self) -> GeneratorYieldData {
        GeneratorYieldData {
            values_key: self.value("values"),
            state_key: self.value("state"),
        }
    }

    fn array_iterator_state_data(&self) -> ArrayIteratorStateData {
        ArrayIteratorStateData {
            array_key: self.value("array"),
            index_key: self.value("index"),
            kind_key: self.value("kind"),
        }
    }

    fn array_iterator_next_data(&self) -> ArrayIteratorNextData {
        ArrayIteratorNextData {
            value_key: self.value("value"),
            done_key: self.value("done"),
        }
    }

    fn promise_with_resolvers_data(&self) -> PromiseWithResolversData {
        PromiseWithResolversData {
            promise_key: self.value("promise"),
            resolve_key: self.value("resolve"),
            reject_key: self.value("reject"),
        }
    }

    fn promise_all_settled_data(&self) -> PromiseAllSettledData {
        PromiseAllSettledData {
            status_key: self.value("status"),
            value_key: self.value("value"),
            reason_key: self.value("reason"),
            fulfilled_value: self.value("fulfilled"),
            rejected_value: self.value("rejected"),
        }
    }

    fn promise_any_data(&self) -> PromiseAnyData {
        PromiseAnyData {
            all_rejected_message: self.value("All promises were rejected"),
        }
    }

    fn aggregate_error_data(&self) -> AggregateErrorData {
        AggregateErrorData {
            errors_key: self.value("errors"),
            message_key: self.value("message"),
            name_key: self.value("name"),
            aggregate_error_name: self.value("AggregateError"),
        }
    }

    fn bigint_mixed_arithmetic_error_data(&self) -> BigIntMixedArithmeticErrorData {
        BigIntMixedArithmeticErrorData {
            diagnostic: self.reference(RuntimeString::BIGINT_MIXED_ARITHMETIC_TYPE_ERROR),
            message_key: self.value("message"),
            message_value: self
                .value("Cannot mix BigInt and other types, use explicit conversions"),
        }
    }

    fn bigint_division_by_zero_range_error_data(&self) -> RuntimeCatchableErrorData {
        RuntimeCatchableErrorData {
            diagnostic: self.reference(RuntimeString::BIGINT_DIVISION_BY_ZERO_RANGE_ERROR),
            message_key: self.value("message"),
            message_value: self.value("Division by zero"),
        }
    }

    fn bigint_string_comparison_boundary_error(&self) -> RuntimeStringRef {
        self.reference(RuntimeString::BIGINT_STRING_COMPARISON_BOUNDARY_ERROR)
    }

    fn object_to_string_values(&self) -> ObjectToStringValues {
        ObjectToStringValues {
            undefined: self.value("[object Undefined]"),
            null: self.value("[object Null]"),
            boolean: self.value("[object Boolean]"),
            number: self.value("[object Number]"),
            string: self.value("[object String]"),
            function: self.value("[object Function]"),
            array: self.value("[object Array]"),
            bigint: self.value("[object BigInt]"),
            symbol: self.value("[object Symbol]"),
            object: self.value("[object Object]"),
        }
    }

    fn error_to_string_data(&self) -> ErrorToStringData {
        ErrorToStringData {
            name_key: self.reference("name"),
            message_key: self.reference("message"),
            empty_string: self.value(""),
            error_string: self.value("Error"),
            colon_space: self.value(": "),
        }
    }

    fn string_match_all_data(&self) -> StringMatchAllData {
        StringMatchAllData {
            zero_key: self.reference("0"),
            index_key: self.reference("index"),
            input_key: self.reference("input"),
        }
    }

    fn private_brand_type_error_data(&self) -> RuntimeCatchableErrorData {
        RuntimeCatchableErrorData {
            diagnostic: self.reference(RuntimeString::PRIVATE_BRAND_TYPE_ERROR),
            message_key: self.value("message"),
            message_value: self
                .value("Cannot read private member from an object whose class did not declare it"),
        }
    }

    pub(crate) fn get(&self, value: &'static str) -> RuntimeStringRef {
        self.reference(value)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct TypeOfStringValues {
    pub(crate) undefined: i32,
    pub(crate) object: i32,
    pub(crate) boolean: i32,
    pub(crate) number: i32,
    pub(crate) string: i32,
    pub(crate) bigint: i32,
    pub(crate) function: i32,
    pub(crate) symbol: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RuntimeStringRef {
    pub(crate) ptr: i32,
    pub(crate) len: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct StringMatchAllData {
    pub(crate) zero_key: RuntimeStringRef,
    pub(crate) index_key: RuntimeStringRef,
    pub(crate) input_key: RuntimeStringRef,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ValueToStringRefs {
    pub(crate) undefined: RuntimeStringRef,
    pub(crate) null: RuntimeStringRef,
    pub(crate) false_: RuntimeStringRef,
    pub(crate) true_: RuntimeStringRef,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct BooleanToStringValues {
    pub(crate) false_: i32,
    pub(crate) true_: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ObjectToStringValues {
    pub(crate) undefined: i32,
    pub(crate) null: i32,
    pub(crate) boolean: i32,
    pub(crate) number: i32,
    pub(crate) string: i32,
    pub(crate) function: i32,
    pub(crate) array: i32,
    pub(crate) bigint: i32,
    pub(crate) symbol: i32,
    pub(crate) object: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ErrorToStringData {
    pub(crate) name_key: RuntimeStringRef,
    pub(crate) message_key: RuntimeStringRef,
    pub(crate) empty_string: i32,
    pub(crate) error_string: i32,
    pub(crate) colon_space: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RuntimeCatchableErrorData {
    pub(crate) diagnostic: RuntimeStringRef,
    pub(crate) message_key: i32,
    pub(crate) message_value: i32,
}

pub(crate) type BigIntMixedArithmeticErrorData = RuntimeCatchableErrorData;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct GeneratorReturnData {
    pub(crate) value_key: i32,
    pub(crate) done_key: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct GeneratorYieldData {
    pub(crate) values_key: i32,
    pub(crate) state_key: i32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ArrayIteratorStateData {
    pub(crate) array_key: i32,
    pub(crate) index_key: i32,
    pub(crate) kind_key: i32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ArrayIteratorNextData {
    pub(crate) value_key: i32,
    pub(crate) done_key: i32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PromiseWithResolversData {
    pub(crate) promise_key: i32,
    pub(crate) resolve_key: i32,
    pub(crate) reject_key: i32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PromiseAllSettledData {
    pub(crate) status_key: i32,
    pub(crate) value_key: i32,
    pub(crate) reason_key: i32,
    pub(crate) fulfilled_value: i32,
    pub(crate) rejected_value: i32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PromiseAnyData {
    pub(crate) all_rejected_message: i32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct AggregateErrorData {
    pub(crate) errors_key: i32,
    pub(crate) message_key: i32,
    pub(crate) name_key: i32,
    pub(crate) aggregate_error_name: i32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PropertyGetData {
    pub(crate) name_key: RuntimeStringRef,
    pub(crate) length_key: RuntimeStringRef,
    pub(crate) direct_functions: Vec<DirectFunctionProperty>,
    /// NativeError constructor property data (Error, AggregateError, etc.).
    /// When the object is a NUMBER-tagged sentinel in the NATIVE_ERROR_PAYLOAD range,
    /// getOwnPropertyDescriptor returns descriptors for "name", "length", and "prototype".
    pub(crate) native_errors: Vec<NativeErrorPropertyData>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct NativeErrorPropertyData {
    /// The NATIVE_ERROR_PAYLOAD sentinel value for this error constructor.
    pub(crate) payload: i32,
    /// Tagged value for the error constructor's identity sentinel (NUMBER-tagged).
    pub(crate) sentinel_value: i32,
    /// Tagged value for the error constructor's `.name` string (e.g., "AggregateError").
    pub(crate) name_value: i32,
    /// Encoded number for the error constructor's `.length` (e.g., 2 for AggregateError).
    pub(crate) length_value: i32,
    /// Global name for the error constructor's prototype object (e.g., "$error_proto_aggregate_error").
    pub(crate) prototype_global: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DirectFunctionProperty {
    pub(crate) payload: i32,
    pub(crate) name_value: Option<i32>,
    pub(crate) length_value: i32,
}

const PSEUDO_RUNTIME_FUNCTIONS: &[RuntimeFn] = &[
    RuntimeFn::ArrayPushMany,
    RuntimeFn::HeapClosureCall,
    RuntimeFn::PrivateBrandCheck,
    RuntimeFn::PrivateFieldGet,
    RuntimeFn::PrivateFieldSet,
    RuntimeFn::RegExpSourceOf,
    RuntimeFn::RegExpFlagsOf,
    RuntimeFn::RegExpCompile,
];

#[cfg(test)]
pub(crate) fn embed_native_runtime_functions(program: &LoweredProgram) -> Vec<WasmFunction> {
    embed_native_runtime_functions_with_data(program, &NativeRuntimeData::default())
}

#[cfg(test)]
pub(crate) fn embed_native_runtime_functions_with_data(
    program: &LoweredProgram,
    data: &NativeRuntimeData,
) -> Vec<WasmFunction> {
    embed_native_runtime_functions_with_data_and_extra(program, data, &[])
}

pub(crate) fn embed_native_runtime_functions_with_data_and_extra(
    program: &LoweredProgram,
    data: &NativeRuntimeData,
    extra_required: &[RuntimeFn],
) -> Vec<WasmFunction> {
    let required = ordered_required_native_runtime_functions_with_extra(program, extra_required);
    let mut functions = Vec::new();
    let requires_string_char_at = required.contains(&RuntimeFn::StringCharAt);
    let requires_string_at = required.contains(&RuntimeFn::StringAt);
    let requires_string_char_code_at = required.contains(&RuntimeFn::StringCharCodeAt);
    let requires_string_code_point_at = required.contains(&RuntimeFn::StringCodePointAt);
    let requires_string_range = required.contains(&RuntimeFn::StringSubstring)
        || required.contains(&RuntimeFn::StringSubstr)
        || required.contains(&RuntimeFn::StringSlice);
    let requires_string_search = required.contains(&RuntimeFn::StringIndexOf)
        || required.contains(&RuntimeFn::StringLastIndexOf)
        || required.contains(&RuntimeFn::StringIncludes);

    if required.contains(&RuntimeFn::GetLength)
        || requires_string_at
        || requires_string_char_code_at
        || requires_string_code_point_at
        || requires_string_range
        || requires_string_search
    {
        functions.push(typed::build_utf8_cp_count());
    }
    if requires_string_search {
        functions.push(typed::build_utf8_byte_to_cp_index());
    }
    if required.contains(&RuntimeFn::Index)
        || requires_string_char_at
        || requires_string_at
        || requires_string_char_code_at
        || requires_string_code_point_at
        || requires_string_range
    {
        functions.push(typed::build_utf8_cp_to_byte_index());
    }
    if required.contains(&RuntimeFn::Index) || requires_string_char_at || requires_string_at {
        functions.push(typed::build_utf8_cp_byte_length());
    }
    if requires_string_code_point_at && !requires_string_char_code_at {
        functions.push(typed::build_string_char_code_at());
    }
    if required
        .iter()
        .any(|runtime_fn| is_relational_runtime(*runtime_fn))
    {
        functions.push(typed::build_bigint_compare_small_int());
        functions.push(typed::build_bigint_compare_decimal_string_for_relational());
        functions.push(typed::build_bigint_string_to_small_int_for_comparison());
    }

    functions.extend(required.into_iter().flat_map(|runtime_fn| {
        build_native_runtime_functions_for_program(runtime_fn, data, program)
    }));
    functions
}

#[cfg(test)]
pub(crate) fn ordered_required_native_runtime_functions(
    program: &LoweredProgram,
) -> Vec<RuntimeFn> {
    ordered_required_native_runtime_functions_with_extra(program, &[])
}

pub(crate) fn ordered_required_native_runtime_functions_with_extra(
    program: &LoweredProgram,
    extra_required: &[RuntimeFn],
) -> Vec<RuntimeFn> {
    let mut plan = build_runtime_link_plan(program);
    for runtime_fn in extra_required {
        plan.add_required_runtime(*runtime_fn);
    }
    if !extra_required.is_empty() {
        plan.populate_derived_sets();
    }
    let result: Vec<RuntimeFn> = RuntimeFn::emission_order()
        .iter()
        .copied()
        .filter(|runtime_fn| plan.required_runtime_functions().contains(runtime_fn))
        .filter(|runtime_fn| native_runtime_function_survives_lowering(program, *runtime_fn))
        .filter(|runtime_fn| !is_pseudo_runtime_function(*runtime_fn))
        .filter(|runtime_fn| native_runtime_function_available(*runtime_fn))
        .collect();
    result
}

fn native_runtime_function_survives_lowering(
    _program: &LoweredProgram,
    _runtime_fn: RuntimeFn,
) -> bool {
    true
}

fn stmt_contains_runtime_call_matching(
    stmt: &LoweredStmt,
    runtime_fn: RuntimeFn,
    predicate: fn(&[LoweredExpr]) -> bool,
) -> bool {
    match stmt {
        LoweredStmt::Block(stmts, _) => stmts
            .iter()
            .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate)),
        LoweredStmt::TryFinally {
            try_body,
            finally_body,
            ..
        } => {
            try_body
                .iter()
                .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
                || finally_body
                    .iter()
                    .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
        }
        LoweredStmt::Let(_, expr, _)
        | LoweredStmt::Assign(_, expr, _)
        | LoweredStmt::Expr(expr, _)
        | LoweredStmt::Yield(expr, _)
        | LoweredStmt::Return(expr, _)
        | LoweredStmt::Throw(expr, _)
        | LoweredStmt::Export { expr, .. }
        | LoweredStmt::ModuleExportsAssign { expr, .. } => {
            expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
        }
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            expr_contains_runtime_call_matching(condition, runtime_fn, predicate)
                || then_body
                    .iter()
                    .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
                || else_body
                    .iter()
                    .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
        }
        LoweredStmt::While {
            condition, body, ..
        }
        | LoweredStmt::DoWhile {
            condition, body, ..
        } => {
            expr_contains_runtime_call_matching(condition, runtime_fn, predicate)
                || body
                    .iter()
                    .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
        }
        LoweredStmt::TryCatch {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            try_body
                .iter()
                .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
                || catch_body.as_deref().is_some_and(|body| {
                    body.iter().any(|stmt| {
                        stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate)
                    })
                })
                || finally_body.as_deref().is_some_and(|body| {
                    body.iter().any(|stmt| {
                        stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate)
                    })
                })
        }
        LoweredStmt::Switch { expr, cases, .. } => {
            expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
                || cases.iter().any(|(case_expr, body)| {
                    case_expr.as_ref().is_some_and(|expr| {
                        expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
                    }) || body.iter().any(|stmt| {
                        stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate)
                    })
                })
        }
        LoweredStmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            init.as_deref().is_some_and(|stmt| {
                stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate)
            }) || condition.as_ref().is_some_and(|expr| {
                expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
            }) || update.as_ref().is_some_and(|expr| {
                expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
            }) || body
                .iter()
                .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
        }
        LoweredStmt::ForIn { iter, body, .. } | LoweredStmt::ForOf { iter, body, .. } => {
            expr_contains_runtime_call_matching(iter, runtime_fn, predicate)
                || body
                    .iter()
                    .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
        }
        LoweredStmt::ForAwaitOfLower { iter, body, .. } => {
            expr_contains_runtime_call_matching(iter, runtime_fn, predicate)
                || body
                    .iter()
                    .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
        }
        LoweredStmt::Labeled { body, .. } => {
            stmt_contains_runtime_call_matching(body, runtime_fn, predicate)
        }
        LoweredStmt::Break { .. }
        | LoweredStmt::Continue { .. }
        | LoweredStmt::ModuleExportsUpdate { .. }
        | LoweredStmt::ClassDecl { .. } => false,
    }
}

fn expr_contains_runtime_call_matching(
    expr: &LoweredExpr,
    runtime_fn: RuntimeFn,
    predicate: fn(&[LoweredExpr]) -> bool,
) -> bool {
    match expr {
        LoweredExpr::RuntimeCall {
            intrinsic, args, ..
        } => {
            (*intrinsic == runtime_fn && predicate(args))
                || args
                    .iter()
                    .any(|arg| expr_contains_runtime_call_matching(arg, runtime_fn, predicate))
        }
        LoweredExpr::EnvCellNew(expr, _)
        | LoweredExpr::EnvCellSet { expr, .. }
        | LoweredExpr::Unary { expr, .. }
        | LoweredExpr::Assign { expr, .. }
        | LoweredExpr::LogicalAssign { expr, .. }
        | LoweredExpr::LogicalPropertyAssign { expr, .. }
        | LoweredExpr::GetLength(expr, _)
        | LoweredExpr::PromiseGetValue { promise: expr, .. } => {
            expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
        }
        LoweredExpr::Binary { left, right, .. } => {
            expr_contains_runtime_call_matching(left, runtime_fn, predicate)
                || expr_contains_runtime_call_matching(right, runtime_fn, predicate)
        }
        LoweredExpr::PropertyIn { obj, .. }
        | LoweredExpr::PropertyGet { obj, .. }
        | LoweredExpr::OptionalPropertyGet { obj, .. }
        | LoweredExpr::MethodCall { object: obj, .. }
        | LoweredExpr::PropertyDelete { object: obj, .. } => {
            expr_contains_runtime_call_matching(obj, runtime_fn, predicate)
        }
        LoweredExpr::PropertyInDynamic { obj, key, .. }
        | LoweredExpr::PropertyGetDynamic { obj, key, .. }
        | LoweredExpr::OptionalIndex {
            object: obj,
            index: key,
            ..
        }
        | LoweredExpr::ArrayGet {
            arr: obj,
            index: key,
            ..
        }
        | LoweredExpr::Index {
            object: obj,
            index: key,
            ..
        }
        | LoweredExpr::LogicalComputedPropertyAssign { key, expr: obj, .. } => {
            expr_contains_runtime_call_matching(obj, runtime_fn, predicate)
                || expr_contains_runtime_call_matching(key, runtime_fn, predicate)
        }
        LoweredExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            expr_contains_runtime_call_matching(object, runtime_fn, predicate)
                || expr_contains_runtime_call_matching(key, runtime_fn, predicate)
                || expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
        }
        LoweredExpr::LogicalMemberAssign { object, expr, .. }
        | LoweredExpr::PropertySet {
            object,
            value: expr,
            ..
        } => {
            expr_contains_runtime_call_matching(object, runtime_fn, predicate)
                || expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => {
            expr_contains_runtime_call_matching(object, runtime_fn, predicate)
                || expr_contains_runtime_call_matching(index, runtime_fn, predicate)
                || expr_contains_runtime_call_matching(value, runtime_fn, predicate)
        }
        LoweredExpr::PropertyDeleteDynamic { object, key, .. } => {
            expr_contains_runtime_call_matching(object, runtime_fn, predicate)
                || expr_contains_runtime_call_matching(key, runtime_fn, predicate)
        }
        LoweredExpr::OptionalCall { callee, call, .. } => {
            expr_contains_runtime_call_matching(callee, runtime_fn, predicate)
                || expr_contains_runtime_call_matching(call, runtime_fn, predicate)
        }
        LoweredExpr::Call { args, .. }
        | LoweredExpr::ArrayNew { elements: args, .. }
        | LoweredExpr::New { args, .. } => args
            .iter()
            .any(|arg| expr_contains_runtime_call_matching(arg, runtime_fn, predicate)),
        LoweredExpr::ArrayNewSparse { slots, .. } => slots.iter().any(|slot| {
            matches!(
                slot,
                ts2wasm_ir::lowered::LoweredArraySlot::Present(expr)
                    if expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
            )
        }),
        LoweredExpr::ObjectNew { props, .. } => props
            .iter()
            .any(|(_, expr)| expr_contains_runtime_call_matching(expr, runtime_fn, predicate)),
        LoweredExpr::ErrorNew {
            message,
            cause,
            errors,
            ..
        } => {
            expr_contains_runtime_call_matching(message, runtime_fn, predicate)
                || cause.as_deref().is_some_and(|expr| {
                    expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
                })
                || errors.as_deref().is_some_and(|expr| {
                    expr_contains_runtime_call_matching(expr, runtime_fn, predicate)
                })
        }
        LoweredExpr::Block { stmts, result, .. } => {
            stmts
                .iter()
                .any(|stmt| stmt_contains_runtime_call_matching(stmt, runtime_fn, predicate))
                || expr_contains_runtime_call_matching(result, runtime_fn, predicate)
        }
        LoweredExpr::Number(..)
        | LoweredExpr::DecimalNumber(..)
        | LoweredExpr::BigIntLiteral { .. }
        | LoweredExpr::String(..)
        | LoweredExpr::Bool(..)
        | LoweredExpr::Null(..)
        | LoweredExpr::Undefined(..)
        | LoweredExpr::Local(..)
        | LoweredExpr::EnvCellGet(..)
        | LoweredExpr::ClassPrototype(..)
        | LoweredExpr::BuiltinErrorPrototype(..)
        | LoweredExpr::ModuleLoad { .. }
        | LoweredExpr::This(..)
        | LoweredExpr::ArrowFn { .. } => false,
    }
}

#[cfg(test)]
pub(crate) fn native_runtime_builder_missing() -> Vec<RuntimeFn> {
    RuntimeFn::emission_order()
        .iter()
        .copied()
        .filter(|runtime_fn| !is_pseudo_runtime_function(*runtime_fn))
        .filter(|runtime_fn| !native_runtime_function_available(*runtime_fn))
        .collect()
}

pub(crate) fn is_pseudo_runtime_function(runtime_fn: RuntimeFn) -> bool {
    PSEUDO_RUNTIME_FUNCTIONS.contains(&runtime_fn)
}

fn is_relational_runtime(runtime_fn: RuntimeFn) -> bool {
    matches!(
        runtime_fn,
        RuntimeFn::Less
            | RuntimeFn::LessFast
            | RuntimeFn::LessEqual
            | RuntimeFn::LessEqualFast
            | RuntimeFn::Greater
            | RuntimeFn::GreaterFast
            | RuntimeFn::GreaterEqual
            | RuntimeFn::GreaterEqualFast
    )
}

pub(crate) fn native_runtime_function_available(runtime_fn: RuntimeFn) -> bool {
    matches!(
        runtime_fn,
        RuntimeFn::And
            | RuntimeFn::Add
            | RuntimeFn::AddFast
            | RuntimeFn::AllocHeap
            | RuntimeFn::AtomicsAdd
            | RuntimeFn::AtomicsAnd
            | RuntimeFn::AtomicsCompareExchange
            | RuntimeFn::AtomicsElementPtr
            | RuntimeFn::AtomicsExchange
            | RuntimeFn::AtomicsIsLockFree
            | RuntimeFn::AtomicsLoad
            | RuntimeFn::AtomicsNotify
            | RuntimeFn::AtomicsOr
            | RuntimeFn::AtomicsStore
            | RuntimeFn::AtomicsSub
            | RuntimeFn::AtomicsWait
            | RuntimeFn::AtomicsWaitAsync
            | RuntimeFn::AtomicsXor
            | RuntimeFn::ArrayBufferIsView
            | RuntimeFn::ArrayBufferNew
            | RuntimeFn::ArrayBufferSlice
            | RuntimeFn::ArrayBufferTransfer
            | RuntimeFn::ArrayConcat
            | RuntimeFn::ArrayCtorWithLength
            | RuntimeFn::ArrayAt
            | RuntimeFn::ArrayCopyWithin
            | RuntimeFn::ArrayEvery
            | RuntimeFn::ArrayFill
            | RuntimeFn::ArrayFilter
            | RuntimeFn::ArrayFind
            | RuntimeFn::ArrayFindIndex
            | RuntimeFn::ArrayFindLast
            | RuntimeFn::ArrayFindLastIndex
            | RuntimeFn::ArrayFlat
            | RuntimeFn::ArrayForEach
            | RuntimeFn::ArrayGet
            | RuntimeFn::ArrayIncludes
            | RuntimeFn::ArrayIndexOf
            | RuntimeFn::ArrayIndexPresent
            | RuntimeFn::ArrayIsArray
            | RuntimeFn::ArrayJoin
            | RuntimeFn::ArrayLastIndexOf
            | RuntimeFn::ArrayMap
            | RuntimeFn::ArrayMapArrayLikeDouble
            | RuntimeFn::ArrayMapArrayLikeIdentity
            | RuntimeFn::ArrayMapStringSplit
            | RuntimeFn::ArrayMapUnaryPlus
            | RuntimeFn::ArrayMapValueToString
            | RuntimeFn::ArrayPop
            | RuntimeFn::ArrayPush
            | RuntimeFn::ArrayPushGrow
            | RuntimeFn::ArrayGrowTo
            | RuntimeFn::ArrayPushOrSpread
            | RuntimeFn::ArrayReduce
            | RuntimeFn::ArrayReduceRight
            | RuntimeFn::ArrayReverse
            | RuntimeFn::ArrayShift
            | RuntimeFn::ArraySlice
            | RuntimeFn::ArraySome
            | RuntimeFn::ArraySortLexicographic
            | RuntimeFn::ArraySortNumeric
            | RuntimeFn::ArraySplice
            | RuntimeFn::ArrayToReversed
            | RuntimeFn::ArrayToSorted
            | RuntimeFn::ArrayToSpliced
            | RuntimeFn::ArrayValues
            | RuntimeFn::ArrayKeys
            | RuntimeFn::ArrayEntries
            | RuntimeFn::ArrayIteratorNext
            | RuntimeFn::ArrayUnshift
            | RuntimeFn::ArrayWith
            | RuntimeFn::BigIntAdd
            | RuntimeFn::BigIntAsIntN
            | RuntimeFn::BigIntAsUintN
            | RuntimeFn::BigIntBitwiseAnd
            | RuntimeFn::BigIntBitwiseNot
            | RuntimeFn::BigIntBitwiseOr
            | RuntimeFn::BigIntBitwiseXor
            | RuntimeFn::BigIntCompare
            | RuntimeFn::BigIntDiv
            | RuntimeFn::BigIntDivisionByZeroRangeError
            | RuntimeFn::BigIntFromValue
            | RuntimeFn::BigIntLeftShift
            | RuntimeFn::BigIntMixedArithmeticTypeError
            | RuntimeFn::BigIntMul
            | RuntimeFn::BigIntPow
            | RuntimeFn::BigIntRem
            | RuntimeFn::BigIntRightShift
            | RuntimeFn::BigIntStringComparisonBoundaryError
            | RuntimeFn::BigIntSub
            | RuntimeFn::BigIntToBoolean
            | RuntimeFn::BigIntToString
            | RuntimeFn::BigIntUnaryMinus
            | RuntimeFn::BooleanCoerce
            | RuntimeFn::BooleanToString
            | RuntimeFn::BitwiseAnd
            | RuntimeFn::BitwiseOr
            | RuntimeFn::BitwiseToI32
            | RuntimeFn::BitwiseXor
            | RuntimeFn::BangEqual
            | RuntimeFn::ConsoleCountImpl
            | RuntimeFn::ConsoleCountResetImpl
            | RuntimeFn::ConsoleGroupEndFn
            | RuntimeFn::ConsoleGroupStart
            | RuntimeFn::ConsoleTimeEndFn
            | RuntimeFn::ConsoleTimeStart
            | RuntimeFn::Concat
            | RuntimeFn::Copy
            | RuntimeFn::CryptoRandomBytes
            | RuntimeFn::DataViewGetBuffer
            | RuntimeFn::DataViewGetByteOffset
            | RuntimeFn::DataViewGetBigInt64
            | RuntimeFn::DataViewGetBigUint64
            | RuntimeFn::DataViewGetFloat16
            | RuntimeFn::DataViewGetFloat32
            | RuntimeFn::DataViewGetFloat64
            | RuntimeFn::DataViewGetInt8
            | RuntimeFn::DataViewGetInt16
            | RuntimeFn::DataViewGetInt32
            | RuntimeFn::DataViewGetUint8
            | RuntimeFn::DataViewGetUint16
            | RuntimeFn::DataViewGetUint32
            | RuntimeFn::DataViewNew
            | RuntimeFn::DataViewSetBigInt64
            | RuntimeFn::DataViewSetBigUint64
            | RuntimeFn::DataViewSetFloat16
            | RuntimeFn::DataViewSetFloat32
            | RuntimeFn::DataViewSetFloat64
            | RuntimeFn::DataViewSetInt8
            | RuntimeFn::DataViewSetInt16
            | RuntimeFn::DataViewSetInt32
            | RuntimeFn::DataViewSetUint8
            | RuntimeFn::DataViewSetUint16
            | RuntimeFn::DataViewSetUint32
            | RuntimeFn::DateEpochMsNowNumber
            | RuntimeFn::DateGetLocalTimeField
            | RuntimeFn::DateGetTimezoneOffset
            | RuntimeFn::DateGetTime
            | RuntimeFn::DateGetUtcDate
            | RuntimeFn::DateGetUtcDay
            | RuntimeFn::DateGetUtcFullYear
            | RuntimeFn::DateGetUtcHours
            | RuntimeFn::DateGetUtcMilliseconds
            | RuntimeFn::DateGetUtcMinutes
            | RuntimeFn::DateGetUtcMonth
            | RuntimeFn::DateGetUtcSeconds
            | RuntimeFn::DateGetYear
            | RuntimeFn::DateNew
            | RuntimeFn::DateNewLive
            | RuntimeFn::DateNow
            | RuntimeFn::DateParse
            | RuntimeFn::DateSetDate
            | RuntimeFn::DateSetFullYear
            | RuntimeFn::DateSetHours
            | RuntimeFn::DateSetMilliseconds
            | RuntimeFn::DateSetMinutes
            | RuntimeFn::DateSetMonth
            | RuntimeFn::DateSetSeconds
            | RuntimeFn::DateSetTime
            | RuntimeFn::DateSetUTCDate
            | RuntimeFn::DateSetUTCFullYear
            | RuntimeFn::DateSetUTCHours
            | RuntimeFn::DateSetUTCMilliseconds
            | RuntimeFn::DateSetUTCMinutes
            | RuntimeFn::DateSetUTCMonth
            | RuntimeFn::DateSetUTCSeconds
            | RuntimeFn::DateSetYear
            | RuntimeFn::DateUTC
            | RuntimeFn::DateToDateString
            | RuntimeFn::DateToGMTString
            | RuntimeFn::DateToISOString
            | RuntimeFn::DateToString
            | RuntimeFn::DateToTimeString
            | RuntimeFn::Dollar262Eval
            | RuntimeFn::EncodeURI
            | RuntimeFn::EncodeURIComponent
            | RuntimeFn::DecodeURI
            | RuntimeFn::DecodeURIComponent
            | RuntimeFn::Escape
            | RuntimeFn::Unescape
            | RuntimeFn::ErrorMessage
            | RuntimeFn::EvalDirectHost
            | RuntimeFn::EvalIndirectHost
            | RuntimeFn::EqualEqual
            | RuntimeFn::FinalizationRegistryNew
            | RuntimeFn::FinalizationRegistryRegister
            | RuntimeFn::FinalizationRegistryUnregister
            | RuntimeFn::FunctionCallHost
            | RuntimeFn::FunctionCallMethodHost
            | RuntimeFn::FunctionCompileHost
            | RuntimeFn::FunctionConstructHost
            | RuntimeFn::FsReadFileSync
            | RuntimeFn::FsWriteFileSync
            | RuntimeFn::FsAppendFileSync
            | RuntimeFn::GeneratorYield
            | RuntimeFn::GeneratorReturn
            | RuntimeFn::GeneratorNext
            | RuntimeFn::GlobalParseFloat
            | RuntimeFn::GlobalParseInt
            | RuntimeFn::Greater
            | RuntimeFn::GetIterator
            | RuntimeFn::GetLength
            | RuntimeFn::GreaterEqual
            | RuntimeFn::GreaterEqualFast
            | RuntimeFn::GreaterFast
            | RuntimeFn::InstanceOf
            | RuntimeFn::Index
            | RuntimeFn::IntlDateTimeFormatFormat
            | RuntimeFn::IntlNumberFormatFormat
            | RuntimeFn::JsonParse
            | RuntimeFn::JsonStringify
            | RuntimeFn::IteratorMap
            | RuntimeFn::IteratorFilter
            | RuntimeFn::IteratorTake
            | RuntimeFn::IteratorDrop
            | RuntimeFn::IteratorToArray
            | RuntimeFn::IteratorReduce
            | RuntimeFn::IteratorForEach
            | RuntimeFn::IteratorSome
            | RuntimeFn::IteratorEvery
            | RuntimeFn::IteratorFind
            | RuntimeFn::IteratorFrom
            | RuntimeFn::IteratorNext
            | RuntimeFn::IsFinite
            | RuntimeFn::IsNaN
            | RuntimeFn::IsString
            | RuntimeFn::Less
            | RuntimeFn::LessEqual
            | RuntimeFn::LessEqualFast
            | RuntimeFn::LessFast
            | RuntimeFn::Log
            | RuntimeFn::LogError
            | RuntimeFn::LogWarn
            | RuntimeFn::MakeBigIntLiteral
            | RuntimeFn::MapClear
            | RuntimeFn::MapDelete
            | RuntimeFn::MapEntriesArray
            | RuntimeFn::MapEntryPairsArray
            | RuntimeFn::MapForEach
            | RuntimeFn::MapGet
            | RuntimeFn::MapHas
            | RuntimeFn::MapKeysArray
            | RuntimeFn::MapKeysIterator
            | RuntimeFn::MapNew
            | RuntimeFn::MapPrototypeDeleteGet
            | RuntimeFn::MapPrototypeDeleteSet
            | RuntimeFn::MapPrototypeForEachGet
            | RuntimeFn::MapPrototypeForEachSet
            | RuntimeFn::MapPrototypeGetGet
            | RuntimeFn::MapPrototypeGetSet
            | RuntimeFn::MapPrototypeHasGet
            | RuntimeFn::MapPrototypeHasSet
            | RuntimeFn::MapPrototypeSetGet
            | RuntimeFn::MapPrototypeSetSet
            | RuntimeFn::MapSet
            | RuntimeFn::MapSize
            | RuntimeFn::MapValuesArray
            | RuntimeFn::MapValuesIterator
            | RuntimeFn::MemEqual
            | RuntimeFn::ModuleExportsAssign
            | RuntimeFn::ModuleExportsSet
            | RuntimeFn::ModuleRequire
            | RuntimeFn::Div
            | RuntimeFn::DivFast
            | RuntimeFn::Mod
            | RuntimeFn::ModFast
            | RuntimeFn::Mul
            | RuntimeFn::MulFast
            | RuntimeFn::Negate
            | RuntimeFn::Not
            | RuntimeFn::NumberCoerce
            | RuntimeFn::NumberFromI32
            | RuntimeFn::NumberIsFinite
            | RuntimeFn::NumberIsInteger
            | RuntimeFn::NumberIsNaN
            | RuntimeFn::NumberIsSafeInteger
            | RuntimeFn::NumberToI32
            | RuntimeFn::NumberToExponential
            | RuntimeFn::NumberToFixed
            | RuntimeFn::NumberToPrecision
            | RuntimeFn::NumberToString
            | RuntimeFn::NumberToStringRadix
            | RuntimeFn::MathFloor
            | RuntimeFn::MathCeil
            | RuntimeFn::MathRound
            | RuntimeFn::MathAbs
            | RuntimeFn::MathMax
            | RuntimeFn::MathMin
            | RuntimeFn::MathPow
            | RuntimeFn::MathTrunc
            | RuntimeFn::MathSign
            | RuntimeFn::MathCbrt
            | RuntimeFn::MathImul
            | RuntimeFn::MathClz32
            | RuntimeFn::MathSqrt
            | RuntimeFn::MathFround
            | RuntimeFn::MathF16round
            | RuntimeFn::MathRandom
            | RuntimeFn::MathAcos
            | RuntimeFn::MathAcosh
            | RuntimeFn::MathAsin
            | RuntimeFn::MathAsinh
            | RuntimeFn::MathAtan
            | RuntimeFn::MathAtan2
            | RuntimeFn::MathAtanh
            | RuntimeFn::MathCos
            | RuntimeFn::MathCosh
            | RuntimeFn::MathExp
            | RuntimeFn::MathExpm1
            | RuntimeFn::MathHypot
            | RuntimeFn::MathLog
            | RuntimeFn::MathLog10
            | RuntimeFn::MathLog1p
            | RuntimeFn::MathLog2
            | RuntimeFn::MathSin
            | RuntimeFn::MathSinh
            | RuntimeFn::MathTan
            | RuntimeFn::MathTanh
            | RuntimeFn::PromiseConstructor
            | RuntimeFn::PromiseResolve
            | RuntimeFn::PromiseReject
            | RuntimeFn::PromiseThen
            | RuntimeFn::PromiseCatch
            | RuntimeFn::PromiseWithResolvers
            | RuntimeFn::PromiseFinally
            | RuntimeFn::PromiseAll
            | RuntimeFn::PromiseAllSettled
            | RuntimeFn::PromiseAny
            | RuntimeFn::PromiseRace
            | RuntimeFn::AggregateError
            | RuntimeFn::ObjectHasOwn
            | RuntimeFn::ObjectHasOwnProperty
            | RuntimeFn::ObjectGetPrototypeOf
            | RuntimeFn::ObjectSetPrototypeOf
            | RuntimeFn::ObjectFreeze
            | RuntimeFn::ObjectSeal
            | RuntimeFn::ObjectPreventExtensions
            | RuntimeFn::ObjectIsExtensible
            | RuntimeFn::ObjectIsSealed
            | RuntimeFn::ObjectIsFrozen
            | RuntimeFn::ObjectPrototype
            | RuntimeFn::GlobalThis
            | RuntimeFn::ObjectCreate
            | RuntimeFn::ObjectToObject
            | RuntimeFn::ObjectDefineProperties
            | RuntimeFn::ObjectDefineProperty
            | RuntimeFn::ObjectIs
            | RuntimeFn::IsPrototypeOf
            | RuntimeFn::PropertyIsEnumerable
            | RuntimeFn::ObjectToString
            | RuntimeFn::ErrorToString
            | RuntimeFn::ObjectToLocaleString
            | RuntimeFn::ObjectAssign
            | RuntimeFn::ObjectGetOwnPropertyDescriptor
            | RuntimeFn::ObjectGetOwnPropertyDescriptors
            | RuntimeFn::ObjectGetOwnPropertyNames
            | RuntimeFn::ObjectGetOwnPropertySymbols
            | RuntimeFn::ObjectFromEntries
            | RuntimeFn::ObjectKeys
            | RuntimeFn::ObjectSpread
            | RuntimeFn::RestObject
            | RuntimeFn::SpreadViaIterator
            | RuntimeFn::ObjectValues
            | RuntimeFn::ObjectEntries
            | RuntimeFn::PathBasename
            | RuntimeFn::PathDirname
            | RuntimeFn::PathJoin
            | RuntimeFn::PathResolve
            | RuntimeFn::ReflectDefineProperty
            | RuntimeFn::ReflectPreventExtensions
            | RuntimeFn::ReflectSet
            | RuntimeFn::ReflectSetPrototypeOf
            | RuntimeFn::PropertyDelete
            | RuntimeFn::PropertyGet
            | RuntimeFn::PropertyHas
            | RuntimeFn::PropertySet
            | RuntimeFn::PrivateBrandTypeError
            | RuntimeFn::ReadStdinBytes
            | RuntimeFn::RegexpMatchInner
            | RuntimeFn::RegexpParseFlags
            | RuntimeFn::RegExpTest
            | RuntimeFn::RegExpMatch
            | RuntimeFn::RegExpSearch
            | RuntimeFn::ReflectDeleteProperty
            | RuntimeFn::ReflectGet
            | RuntimeFn::ReflectHas
            | RuntimeFn::ReflectOwnKeys
            | RuntimeFn::ReflectApply
            | RuntimeFn::ReflectConstruct
            | RuntimeFn::ProcessArgv
            | RuntimeFn::ProcessEnv
            | RuntimeFn::ProcessExit
            | RuntimeFn::Or
            | RuntimeFn::SameValueZero
            | RuntimeFn::SetAdd
            | RuntimeFn::SetClear
            | RuntimeFn::SetDelete
            | RuntimeFn::SetDifference
            | RuntimeFn::SetEntriesArray
            | RuntimeFn::SetForEach
            | RuntimeFn::SetFromArray
            | RuntimeFn::SetHas
            | RuntimeFn::SetIntersection
            | RuntimeFn::SetIsDisjointFrom
            | RuntimeFn::SetIsSubsetOf
            | RuntimeFn::SetIsSupersetOf
            | RuntimeFn::SetNew
            | RuntimeFn::SetPrototypeAddGet
            | RuntimeFn::SetPrototypeAddSet
            | RuntimeFn::SetPrototypeDeleteGet
            | RuntimeFn::SetPrototypeDeleteSet
            | RuntimeFn::SetPrototypeForEachGet
            | RuntimeFn::SetPrototypeForEachSet
            | RuntimeFn::SetPrototypeHasGet
            | RuntimeFn::SetPrototypeHasSet
            | RuntimeFn::SetSize
            | RuntimeFn::SetSymmetricDifference
            | RuntimeFn::SetUnion
            | RuntimeFn::SetValuesArray
            | RuntimeFn::SetValuesIterator
            | RuntimeFn::SharedArrayBufferNew
            | RuntimeFn::StringAt
            | RuntimeFn::StringCharAt
            | RuntimeFn::StringCharCodeAt
            | RuntimeFn::StringCodePointAt
            | RuntimeFn::StringFromCharCode
            | RuntimeFn::StringFromCodePoint
            | RuntimeFn::StringEndsWith
            | RuntimeFn::StringIncludes
            | RuntimeFn::StringIndexOf
            | RuntimeFn::StringEqual
            | RuntimeFn::StringIsWellFormed
            | RuntimeFn::StringLastIndexOf
            | RuntimeFn::StringLocaleCompare
            | RuntimeFn::StringMatch
            | RuntimeFn::StringMatchAll
            | RuntimeFn::StringNormalize
            | RuntimeFn::StringPadEnd
            | RuntimeFn::StringPadStart
            | RuntimeFn::StringReplace
            | RuntimeFn::StringReplaceAll
            | RuntimeFn::StringRepeat
            | RuntimeFn::StringSearch
            | RuntimeFn::StringSlice
            | RuntimeFn::StringSplit
            | RuntimeFn::StringStartsWith
            | RuntimeFn::StringSubstr
            | RuntimeFn::StringSubstring
            | RuntimeFn::StringRaw
            | RuntimeFn::StringToLowerCase
            | RuntimeFn::StringToLocaleString
            | RuntimeFn::StringTrim
            | RuntimeFn::StringTrimEnd
            | RuntimeFn::StringTrimStart
            | RuntimeFn::StringToUpperCase
            | RuntimeFn::StringToWellFormed
            | RuntimeFn::StrictEqual
            | RuntimeFn::StrictNotEqual
            | RuntimeFn::Sub
            | RuntimeFn::SubFast
            | RuntimeFn::SuperCallExternal
            | RuntimeFn::SymbolDescription
            | RuntimeFn::SymbolFor
            | RuntimeFn::SymbolHasInstance
            | RuntimeFn::SymbolKeyFor
            | RuntimeFn::SymbolNew
            | RuntimeFn::SymbolToPrimitive
            | RuntimeFn::SymbolToString
            | RuntimeFn::SymbolToStringTag
            | RuntimeFn::SymbolWellKnown
            | RuntimeFn::TaskPoll
            | RuntimeFn::TaskDrop
            | RuntimeFn::TaskResult
            | RuntimeFn::TypedArrayCtorFromBuffer
            | RuntimeFn::TypedArrayCtorWithLength
            | RuntimeFn::TypedArrayFromArray
            | RuntimeFn::TypedArrayLoad
            | RuntimeFn::TypedArraySet
            | RuntimeFn::TypedArrayStore
            | RuntimeFn::TypeOf
            | RuntimeFn::TruthyBool
            | RuntimeFn::ValueOf
            | RuntimeFn::ValueToStringInto
            | RuntimeFn::WeakMapDelete
            | RuntimeFn::WeakMapGet
            | RuntimeFn::WeakMapHas
            | RuntimeFn::WeakRefDeref
            | RuntimeFn::WeakRefNew
            | RuntimeFn::WeakMapNew
            | RuntimeFn::WeakMapSet
            | RuntimeFn::WeakSetAdd
            | RuntimeFn::WeakSetDelete
            | RuntimeFn::WeakSetHas
            | RuntimeFn::WeakSetNew
            | RuntimeFn::Dollar262Global
            | RuntimeFn::Write
    )
}

fn build_native_runtime_function(
    runtime_fn: RuntimeFn,
    data: &NativeRuntimeData,
) -> Option<WasmFunction> {
    match runtime_fn {
        RuntimeFn::And => Some(typed::build_and()),
        RuntimeFn::Add => Some(typed::build_add()),
        RuntimeFn::AddFast => Some(typed::build_add_fast()),
        RuntimeFn::AllocHeap => Some(typed::build_alloc_heap_with_memory_max_pages(
            NATIVE_MEMORY_MAX_PAGES,
        )),
        RuntimeFn::AtomicsAdd => Some(typed::build_atomics_add()),
        RuntimeFn::AtomicsAnd => Some(typed::build_atomics_and()),
        RuntimeFn::AtomicsCompareExchange => Some(typed::build_atomics_compare_exchange()),
        RuntimeFn::AtomicsElementPtr => Some(typed::build_atomics_element_ptr()),
        RuntimeFn::AtomicsExchange => Some(typed::build_atomics_exchange()),
        RuntimeFn::AtomicsIsLockFree => Some(typed::build_atomics_is_lock_free()),
        RuntimeFn::AtomicsLoad => Some(typed::build_atomics_load()),
        RuntimeFn::AtomicsNotify => Some(typed::build_atomics_notify()),
        RuntimeFn::AtomicsOr => Some(typed::build_atomics_or()),
        RuntimeFn::AtomicsStore => Some(typed::build_atomics_store()),
        RuntimeFn::AtomicsSub => Some(typed::build_atomics_sub()),
        RuntimeFn::AtomicsWait => Some(typed::build_atomics_wait()),
        RuntimeFn::AtomicsWaitAsync => Some(typed::build_atomics_wait_async()),
        RuntimeFn::AtomicsXor => Some(typed::build_atomics_xor()),
        RuntimeFn::ArrayBufferIsView => Some(typed::build_arraybuffer_is_view()),
        RuntimeFn::ArrayBufferNew => Some(typed::build_arraybuffer_new()),
        RuntimeFn::ArrayBufferSlice => Some(typed::build_arraybuffer_slice()),
        RuntimeFn::ArrayBufferTransfer => Some(typed::build_arraybuffer_transfer()),
        RuntimeFn::ArrayConcat => Some(typed::build_array_concat()),
        RuntimeFn::ArrayCtorWithLength => Some(typed::build_array_ctor_with_length()),
        RuntimeFn::ArrayAt => Some(typed::build_array_at()),
        RuntimeFn::ArrayCopyWithin => Some(typed::build_array_copy_within()),
        RuntimeFn::ArrayEvery => Some(typed::build_array_every()),
        RuntimeFn::ArrayFill => Some(typed::build_array_fill()),
        RuntimeFn::ArrayFilter => Some(typed::build_array_filter()),
        RuntimeFn::ArrayFind => Some(typed::build_array_find()),
        RuntimeFn::ArrayFindIndex => Some(typed::build_array_find_index()),
        RuntimeFn::ArrayFindLast => Some(typed::build_array_find_last()),
        RuntimeFn::ArrayFindLastIndex => Some(typed::build_array_find_last_index()),
        RuntimeFn::ArrayFlat => Some(typed::build_array_flat()),
        RuntimeFn::ArrayForEach => Some(typed::build_array_for_each()),
        RuntimeFn::ArrayGet => Some(typed::build_array_get()),
        RuntimeFn::ArrayIncludes => Some(typed::build_array_includes()),
        RuntimeFn::ArrayIndexOf => Some(typed::build_array_index_of()),
        RuntimeFn::ArrayIndexPresent => Some(typed::build_array_index_present()),
        RuntimeFn::ArrayIsArray => Some(typed::build_array_is_array()),
        RuntimeFn::ArrayJoin => Some(typed::build_array_join()),
        RuntimeFn::ArrayLastIndexOf => Some(typed::build_array_last_index_of()),
        RuntimeFn::ArrayMap => Some(typed::build_array_map()),
        RuntimeFn::ArrayMapArrayLikeDouble => Some(typed::build_array_map_array_like_double()),
        RuntimeFn::ArrayMapArrayLikeIdentity => Some(typed::build_array_map_array_like_identity()),
        RuntimeFn::ArrayMapStringSplit => Some(typed::build_array_map_string_split()),
        RuntimeFn::ArrayMapUnaryPlus => Some(typed::build_array_map_unary_plus()),
        RuntimeFn::ArrayMapValueToString => Some(typed::build_array_map_value_to_string()),
        RuntimeFn::ArrayPop => Some(typed::build_array_pop()),
        RuntimeFn::ArrayPush => Some(typed::build_array_push()),
        RuntimeFn::ArrayPushGrow => Some(typed::build_array_push_grow()),
        RuntimeFn::ArrayGrowTo => Some(typed::build_array_grow_to()),
        RuntimeFn::ArrayPushOrSpread => Some(typed::build_array_push_or_spread()),
        RuntimeFn::ArrayReduce => Some(typed::build_array_reduce()),
        RuntimeFn::ArrayReduceRight => Some(typed::build_array_reduce_right()),
        RuntimeFn::ArrayReverse => Some(typed::build_array_reverse()),
        RuntimeFn::ArrayShift => Some(typed::build_array_shift()),
        RuntimeFn::ArraySlice => Some(typed::build_array_slice()),
        RuntimeFn::ArraySome => Some(typed::build_array_some()),
        RuntimeFn::ArraySortLexicographic => Some(typed::build_array_sort_lexicographic()),
        RuntimeFn::ArraySortNumeric => Some(typed::build_array_sort_numeric()),
        RuntimeFn::ArrayToReversed => Some(typed::build_array_to_reversed()),
        RuntimeFn::ArrayToSorted => Some(typed::build_array_to_sorted()),
        RuntimeFn::ArrayToSpliced => Some(typed::build_array_to_spliced()),
        RuntimeFn::ArraySplice => Some(typed::build_array_splice()),
        RuntimeFn::ArrayValues => Some(typed::build_array_values(
            data.strings.array_iterator_state_data(),
        )),
        RuntimeFn::ArrayKeys => Some(typed::build_array_keys(
            data.strings.array_iterator_state_data(),
        )),
        RuntimeFn::ArrayEntries => Some(typed::build_array_entries(
            data.strings.array_iterator_state_data(),
        )),
        RuntimeFn::ArrayIteratorNext => Some(typed::build_array_iterator_next(
            data.strings.array_iterator_next_data(),
        )),
        RuntimeFn::ArrayUnshift => Some(typed::build_array_unshift()),
        RuntimeFn::ArrayWith => Some(typed::build_array_with()),
        RuntimeFn::BigIntAdd => Some(typed::build_bigint_add()),
        RuntimeFn::BigIntAsIntN => Some(typed::build_bigint_as_int_n()),
        RuntimeFn::BigIntAsUintN => Some(typed::build_bigint_as_uint_n()),
        RuntimeFn::BigIntBitwiseAnd => Some(typed::build_bigint_bitwise_and()),
        RuntimeFn::BigIntBitwiseNot => Some(typed::build_bigint_bitwise_not()),
        RuntimeFn::BigIntBitwiseOr => Some(typed::build_bigint_bitwise_or()),
        RuntimeFn::BigIntBitwiseXor => Some(typed::build_bigint_bitwise_xor()),
        RuntimeFn::BigIntCompare => Some(typed::build_bigint_compare()),
        RuntimeFn::BigIntDiv => Some(typed::build_bigint_div()),
        RuntimeFn::BigIntFromValue => Some(typed::build_bigint_from_value()),
        RuntimeFn::BigIntLeftShift => Some(typed::build_bigint_left_shift()),
        RuntimeFn::BigIntMul => Some(typed::build_bigint_mul()),
        RuntimeFn::BigIntPow => Some(typed::build_bigint_pow()),
        RuntimeFn::BigIntRem => Some(typed::build_bigint_rem()),
        RuntimeFn::BigIntRightShift => Some(typed::build_bigint_right_shift()),
        RuntimeFn::BigIntSub => Some(typed::build_bigint_sub()),
        RuntimeFn::BigIntToBoolean => Some(typed::build_bigint_to_boolean()),
        RuntimeFn::BigIntToString => Some(typed::build_bigint_to_string()),
        RuntimeFn::BigIntUnaryMinus => Some(typed::build_bigint_unary_minus()),
        RuntimeFn::BigIntDivisionByZeroRangeError => {
            Some(typed::build_bigint_division_by_zero_range_error(
                data.strings.bigint_division_by_zero_range_error_data(),
            ))
        }
        RuntimeFn::BigIntMixedArithmeticTypeError => {
            Some(typed::build_bigint_mixed_arithmetic_type_error(
                data.strings.bigint_mixed_arithmetic_error_data(),
            ))
        }
        RuntimeFn::BigIntStringComparisonBoundaryError => {
            Some(typed::build_bigint_string_comparison_boundary_error(
                data.strings.bigint_string_comparison_boundary_error(),
            ))
        }
        RuntimeFn::BooleanCoerce => Some(typed::build_boolean_coerce()),
        RuntimeFn::BooleanToString => Some(typed::build_boolean_to_string(
            data.strings.boolean_to_string_values(),
        )),
        RuntimeFn::BitwiseAnd => Some(typed::build_bitwise_and()),
        RuntimeFn::BitwiseOr => Some(typed::build_bitwise_or()),
        RuntimeFn::BitwiseToI32 => Some(typed::build_bitwise_to_i32()),
        RuntimeFn::BitwiseXor => Some(typed::build_bitwise_xor()),
        RuntimeFn::BangEqual => Some(typed::build_bang_equal()),
        RuntimeFn::ConsoleCountImpl => {
            Some(typed::build_console_count(data.strings.log_newline().ptr))
        }
        RuntimeFn::ConsoleCountResetImpl => Some(typed::build_console_count_reset()),
        RuntimeFn::ConsoleGroupEndFn => Some(typed::build_console_group_end()),
        RuntimeFn::ConsoleGroupStart => Some(typed::build_console_group_start(
            data.strings.log_newline().ptr,
        )),
        RuntimeFn::ConsoleTimeEndFn => Some(typed::build_console_time_end(
            data.strings.log_newline().ptr,
        )),
        RuntimeFn::ConsoleTimeStart => Some(typed::build_console_time_start()),
        RuntimeFn::Concat => Some(typed::build_concat()),
        RuntimeFn::Copy => Some(typed::build_copy()),
        RuntimeFn::CryptoRandomBytes => Some(typed::build_crypto_random_bytes()),
        RuntimeFn::DataViewGetBuffer => Some(typed::build_dataview_get_buffer()),
        RuntimeFn::DataViewGetByteOffset => Some(typed::build_dataview_get_byte_offset()),
        RuntimeFn::DataViewGetBigInt64 => Some(typed::build_dataview_get_bigint64()),
        RuntimeFn::DataViewGetBigUint64 => Some(typed::build_dataview_get_biguint64()),
        RuntimeFn::DataViewGetFloat16 => Some(typed::build_dataview_get_float16()),
        RuntimeFn::DataViewGetFloat32 => Some(typed::build_dataview_get_float32()),
        RuntimeFn::DataViewGetFloat64 => Some(typed::build_dataview_get_float64()),
        RuntimeFn::DataViewGetInt8 => Some(typed::build_dataview_get_int8()),
        RuntimeFn::DataViewGetInt16 => Some(typed::build_dataview_get_int16()),
        RuntimeFn::DataViewGetInt32 => Some(typed::build_dataview_get_int32()),
        RuntimeFn::DataViewGetUint8 => Some(typed::build_dataview_get_uint8()),
        RuntimeFn::DataViewGetUint16 => Some(typed::build_dataview_get_uint16()),
        RuntimeFn::DataViewGetUint32 => Some(typed::build_dataview_get_uint32()),
        RuntimeFn::DataViewNew => Some(typed::build_dataview_new()),
        RuntimeFn::DataViewSetBigInt64 => Some(typed::build_dataview_set_bigint64()),
        RuntimeFn::DataViewSetBigUint64 => Some(typed::build_dataview_set_biguint64()),
        RuntimeFn::DataViewSetFloat16 => Some(typed::build_dataview_set_float16()),
        RuntimeFn::DataViewSetFloat32 => Some(typed::build_dataview_set_float32()),
        RuntimeFn::DataViewSetFloat64 => Some(typed::build_dataview_set_float64()),
        RuntimeFn::DataViewSetInt8 => Some(typed::build_dataview_set_int8()),
        RuntimeFn::DataViewSetInt16 => Some(typed::build_dataview_set_int16()),
        RuntimeFn::DataViewSetInt32 => Some(typed::build_dataview_set_int32()),
        RuntimeFn::DataViewSetUint8 => Some(typed::build_dataview_set_uint8()),
        RuntimeFn::DataViewSetUint16 => Some(typed::build_dataview_set_uint16()),
        RuntimeFn::DataViewSetUint32 => Some(typed::build_dataview_set_uint32()),
        RuntimeFn::DateEpochMsNowNumber => Some(typed::build_date_epoch_ms_now_number()),
        RuntimeFn::DateGetLocalTimeField => Some(typed::build_date_get_local_time_field()),
        RuntimeFn::DateGetTimezoneOffset => Some(typed::build_date_get_timezone_offset()),
        RuntimeFn::DateGetTime => Some(typed::build_date_get_time()),
        RuntimeFn::DateGetUtcDate => Some(typed::build_date_get_utc_date()),
        RuntimeFn::DateGetUtcDay => Some(typed::build_date_get_utc_day()),
        RuntimeFn::DateGetUtcFullYear => Some(typed::build_date_get_utc_full_year()),
        RuntimeFn::DateGetUtcHours => Some(typed::build_date_get_utc_hours()),
        RuntimeFn::DateGetUtcMilliseconds => Some(typed::build_date_get_utc_milliseconds()),
        RuntimeFn::DateGetUtcMinutes => Some(typed::build_date_get_utc_minutes()),
        RuntimeFn::DateGetUtcMonth => Some(typed::build_date_get_utc_month()),
        RuntimeFn::DateGetUtcSeconds => Some(typed::build_date_get_utc_seconds()),
        RuntimeFn::DateGetYear => Some(typed::build_date_get_year()),
        RuntimeFn::DateNew => Some(typed::build_date_new()),
        RuntimeFn::DateNewLive => Some(typed::build_date_new_live()),
        RuntimeFn::DateNow => Some(typed::build_date_now()),
        RuntimeFn::DateParse => Some(typed::build_date_parse()),
        RuntimeFn::DateSetDate => Some(typed::build_date_set_date()),
        RuntimeFn::DateSetFullYear => Some(typed::build_date_set_full_year()),
        RuntimeFn::DateSetHours => Some(typed::build_date_set_hours()),
        RuntimeFn::DateSetMilliseconds => Some(typed::build_date_set_milliseconds()),
        RuntimeFn::DateSetMinutes => Some(typed::build_date_set_minutes()),
        RuntimeFn::DateSetMonth => Some(typed::build_date_set_month()),
        RuntimeFn::DateSetSeconds => Some(typed::build_date_set_seconds()),
        RuntimeFn::DateSetTime => Some(typed::build_date_set_time()),
        RuntimeFn::DateSetUTCDate => Some(typed::build_date_set_utc_date()),
        RuntimeFn::DateSetUTCFullYear => Some(typed::build_date_set_utc_full_year()),
        RuntimeFn::DateSetUTCHours => Some(typed::build_date_set_utc_hours()),
        RuntimeFn::DateSetUTCMilliseconds => Some(typed::build_date_set_utc_milliseconds()),
        RuntimeFn::DateSetUTCMinutes => Some(typed::build_date_set_utc_minutes()),
        RuntimeFn::DateSetUTCMonth => Some(typed::build_date_set_utc_month()),
        RuntimeFn::DateSetUTCSeconds => Some(typed::build_date_set_utc_seconds()),
        RuntimeFn::DateSetYear => Some(typed::build_date_set_year()),
        RuntimeFn::DateUTC => Some(typed::build_date_utc()),
        RuntimeFn::DateToDateString => Some(typed::build_date_to_date_string()),
        RuntimeFn::DateToGMTString => Some(typed::build_date_to_gmt_string()),
        RuntimeFn::DateToISOString => Some(typed::build_date_to_iso_string()),
        RuntimeFn::DateToString => Some(typed::build_date_to_string()),
        RuntimeFn::DateToTimeString => Some(typed::build_date_to_time_string()),
        RuntimeFn::Dollar262Eval => Some(typed::build_dollar_262_eval()),
        RuntimeFn::EncodeURI => Some(typed::build_encode_uri()),
        RuntimeFn::EncodeURIComponent => Some(typed::build_encode_uri_component()),
        RuntimeFn::DecodeURI => Some(typed::build_decode_uri()),
        RuntimeFn::DecodeURIComponent => Some(typed::build_decode_uri_component()),
        RuntimeFn::Escape => Some(typed::build_escape()),
        RuntimeFn::Unescape => Some(typed::build_unescape()),
        RuntimeFn::ErrorMessage => Some(typed::build_error_message()),
        RuntimeFn::EvalDirectHost => Some(typed::build_eval_direct_host()),
        RuntimeFn::EvalIndirectHost => Some(typed::build_eval_indirect_host()),
        RuntimeFn::EqualEqual => Some(typed::build_equal_equal()),
        RuntimeFn::FinalizationRegistryNew => Some(typed::build_finalization_registry_new()),
        RuntimeFn::FinalizationRegistryRegister => {
            Some(typed::build_finalization_registry_register())
        }
        RuntimeFn::FinalizationRegistryUnregister => {
            Some(typed::build_finalization_registry_unregister())
        }
        RuntimeFn::FunctionCallHost => Some(typed::build_function_call_host()),
        RuntimeFn::FunctionCallMethodHost => Some(typed::build_function_call_method_host()),
        RuntimeFn::FunctionCompileHost => Some(typed::build_function_compile_host()),
        RuntimeFn::FunctionConstructHost => Some(typed::build_function_construct_host()),
        RuntimeFn::FsReadFileSync => Some(typed::build_fs_read_file_sync()),
        RuntimeFn::FsWriteFileSync => Some(typed::build_fs_write_file_sync()),
        RuntimeFn::FsAppendFileSync => Some(typed::build_fs_append_file_sync()),
        RuntimeFn::GeneratorYield => Some(typed::build_generator_yield(
            data.strings.generator_yield_data(),
        )),
        RuntimeFn::GeneratorReturn => Some(typed::build_generator_return(
            data.strings.generator_return_data(),
        )),
        RuntimeFn::GeneratorNext => Some(typed::build_generator_next(
            data.strings.generator_return_data(),
        )),
        RuntimeFn::GlobalParseFloat => Some(typed::build_global_parse_float()),
        RuntimeFn::GlobalParseInt => Some(typed::build_global_parse_int()),
        RuntimeFn::Greater => Some(typed::build_greater()),
        RuntimeFn::GetIterator => Some(typed::build_get_iterator()),
        RuntimeFn::GetLength => Some(typed::build_get_length()),
        RuntimeFn::GreaterEqual => Some(typed::build_greater_equal()),
        RuntimeFn::GreaterEqualFast => Some(typed::build_greater_equal_fast()),
        RuntimeFn::GreaterFast => Some(typed::build_greater_fast()),
        RuntimeFn::InstanceOf => Some(typed::build_instanceof()),
        RuntimeFn::Index => Some(typed::build_index()),
        RuntimeFn::IntlDateTimeFormatFormat => Some(typed::build_intl_date_time_format_format()),
        RuntimeFn::IntlNumberFormatFormat => Some(typed::build_intl_number_format_format()),
        RuntimeFn::JsonParse => Some(typed::build_json_parse()),
        RuntimeFn::JsonStringify => Some(typed::build_json_stringify()),
        RuntimeFn::IteratorMap => Some(typed::build_iterator_map()),
        RuntimeFn::IteratorFilter => Some(typed::build_iterator_filter()),
        RuntimeFn::IteratorTake => Some(typed::build_iterator_take()),
        RuntimeFn::IteratorDrop => Some(typed::build_iterator_drop()),
        RuntimeFn::IteratorToArray => Some(typed::build_iterator_to_array()),
        RuntimeFn::IteratorReduce => Some(typed::build_iterator_reduce()),
        RuntimeFn::IteratorForEach => Some(typed::build_iterator_for_each()),
        RuntimeFn::IteratorSome => Some(typed::build_iterator_some()),
        RuntimeFn::IteratorEvery => Some(typed::build_iterator_every()),
        RuntimeFn::IteratorFind => Some(typed::build_iterator_find()),
        RuntimeFn::IteratorFrom => Some(typed::build_iterator_from()),
        RuntimeFn::IteratorNext => Some(typed::build_iterator_next()),
        RuntimeFn::IsFinite => Some(typed::build_is_finite()),
        RuntimeFn::IsNaN => Some(typed::build_is_nan()),
        RuntimeFn::IsString => Some(typed::build_is_string()),
        RuntimeFn::Less => Some(typed::build_less()),
        RuntimeFn::LessEqual => Some(typed::build_less_equal()),
        RuntimeFn::LessEqualFast => Some(typed::build_less_equal_fast()),
        RuntimeFn::LessFast => Some(typed::build_less_fast()),
        RuntimeFn::Log => Some(typed::build_log(data.strings.log_newline().ptr)),
        RuntimeFn::LogError => Some(typed::build_log_error(data.strings.log_newline().ptr)),
        RuntimeFn::LogWarn => Some(typed::build_log_warn(data.strings.log_newline().ptr)),
        RuntimeFn::MakeBigIntLiteral => Some(typed::build_make_bigint_literal()),
        RuntimeFn::MapClear => Some(typed::build_map_clear()),
        RuntimeFn::MapDelete => Some(typed::build_map_delete()),
        RuntimeFn::MapEntriesArray => Some(typed::build_map_entries_array()),
        RuntimeFn::MapEntryPairsArray => Some(typed::build_map_entry_pairs_array()),
        RuntimeFn::MapForEach => Some(typed::build_map_for_each()),
        RuntimeFn::MapGet => Some(typed::build_map_get()),
        RuntimeFn::MapHas => Some(typed::build_map_has()),
        RuntimeFn::MapKeysArray => Some(typed::build_map_keys_array()),
        RuntimeFn::MapKeysIterator => Some(typed::build_map_keys_iterator()),
        RuntimeFn::MapNew => Some(typed::build_map_new()),
        RuntimeFn::MapPrototypeDeleteGet => Some(typed::build_map_prototype_delete_get()),
        RuntimeFn::MapPrototypeDeleteSet => Some(typed::build_map_prototype_delete_set()),
        RuntimeFn::MapPrototypeForEachGet => Some(typed::build_map_prototype_for_each_get()),
        RuntimeFn::MapPrototypeForEachSet => Some(typed::build_map_prototype_for_each_set()),
        RuntimeFn::MapPrototypeGetGet => Some(typed::build_map_prototype_get_get()),
        RuntimeFn::MapPrototypeGetSet => Some(typed::build_map_prototype_get_set()),
        RuntimeFn::MapPrototypeHasGet => Some(typed::build_map_prototype_has_get()),
        RuntimeFn::MapPrototypeHasSet => Some(typed::build_map_prototype_has_set()),
        RuntimeFn::MapPrototypeSetGet => Some(typed::build_map_prototype_set_get()),
        RuntimeFn::MapPrototypeSetSet => Some(typed::build_map_prototype_set_set()),
        RuntimeFn::MapSet => Some(typed::build_map_set()),
        RuntimeFn::MapSize => Some(typed::build_map_size()),
        RuntimeFn::MapValuesArray => Some(typed::build_map_values_array()),
        RuntimeFn::MapValuesIterator => Some(typed::build_map_values_iterator()),
        RuntimeFn::MemEqual => Some(typed::build_mem_equal()),
        RuntimeFn::ModuleExportsAssign => Some(typed::build_module_exports_assign()),
        RuntimeFn::ModuleExportsSet => Some(typed::build_module_exports_set()),
        RuntimeFn::ModuleRequire => Some(typed::build_module_require()),
        RuntimeFn::Div => Some(typed::build_div()),
        RuntimeFn::DivFast => Some(typed::build_div_fast()),
        RuntimeFn::Mod => Some(typed::build_mod()),
        RuntimeFn::ModFast => Some(typed::build_mod_fast()),
        RuntimeFn::Mul => Some(typed::build_mul()),
        RuntimeFn::MulFast => Some(typed::build_mul_fast()),
        RuntimeFn::Negate => Some(typed::build_negate()),
        RuntimeFn::Not => Some(typed::build_not()),
        RuntimeFn::NumberCoerce => Some(typed::build_number_coerce()),
        RuntimeFn::NumberFromI32 => Some(typed::build_number_from_i32()),
        RuntimeFn::NumberIsFinite => Some(typed::build_number_is_finite()),
        RuntimeFn::NumberIsInteger => Some(typed::build_number_is_integer()),
        RuntimeFn::NumberIsNaN => Some(typed::build_number_is_nan()),
        RuntimeFn::NumberIsSafeInteger => Some(typed::build_number_is_safe_integer()),
        RuntimeFn::NumberToI32 => Some(typed::build_number_to_i32()),
        RuntimeFn::NumberToExponential => Some(typed::build_number_to_exponential()),
        RuntimeFn::NumberToFixed => Some(typed::build_number_to_fixed()),
        RuntimeFn::NumberToPrecision => Some(typed::build_number_to_precision()),
        RuntimeFn::NumberToString => Some(typed::build_number_to_string()),
        RuntimeFn::NumberToStringRadix => Some(typed::build_number_to_string_radix()),
        RuntimeFn::MathFloor => Some(typed::build_math_floor()),
        RuntimeFn::MathCeil => Some(typed::build_math_ceil()),
        RuntimeFn::MathRound => Some(typed::build_math_round()),
        RuntimeFn::MathAbs => Some(typed::build_math_abs()),
        RuntimeFn::MathMax => Some(typed::build_math_max()),
        RuntimeFn::MathMin => Some(typed::build_math_min()),
        RuntimeFn::MathPow => Some(typed::build_math_pow()),
        RuntimeFn::MathTrunc => Some(typed::build_math_trunc()),
        RuntimeFn::MathSign => Some(typed::build_math_sign()),
        RuntimeFn::MathCbrt => Some(typed::build_math_cbrt()),
        RuntimeFn::MathImul => Some(typed::build_math_imul()),
        RuntimeFn::MathClz32 => Some(typed::build_math_clz32()),
        RuntimeFn::MathSqrt => Some(typed::build_math_sqrt()),
        RuntimeFn::MathFround => Some(typed::build_math_fround()),
        RuntimeFn::MathF16round => Some(typed::build_math_f16round()),
        RuntimeFn::MathRandom => Some(typed::build_math_random()),
        RuntimeFn::MathAcos => Some(typed::build_math_acos()),
        RuntimeFn::MathAcosh => Some(typed::build_math_acosh()),
        RuntimeFn::MathAsin => Some(typed::build_math_asin()),
        RuntimeFn::MathAsinh => Some(typed::build_math_asinh()),
        RuntimeFn::MathAtan => Some(typed::build_math_atan()),
        RuntimeFn::MathAtan2 => Some(typed::build_math_atan2()),
        RuntimeFn::MathAtanh => Some(typed::build_math_atanh()),
        RuntimeFn::MathCos => Some(typed::build_math_cos()),
        RuntimeFn::MathCosh => Some(typed::build_math_cosh()),
        RuntimeFn::MathExp => Some(typed::build_math_exp()),
        RuntimeFn::MathExpm1 => Some(typed::build_math_expm1()),
        RuntimeFn::MathHypot => Some(typed::build_math_hypot()),
        RuntimeFn::MathLog => Some(typed::build_math_log()),
        RuntimeFn::MathLog10 => Some(typed::build_math_log10()),
        RuntimeFn::MathLog1p => Some(typed::build_math_log1p()),
        RuntimeFn::MathLog2 => Some(typed::build_math_log2()),
        RuntimeFn::MathSin => Some(typed::build_math_sin()),
        RuntimeFn::MathSinh => Some(typed::build_math_sinh()),
        RuntimeFn::MathTan => Some(typed::build_math_tan()),
        RuntimeFn::MathTanh => Some(typed::build_math_tanh()),
        RuntimeFn::PromiseConstructor => Some(typed::build_promise_constructor()),
        RuntimeFn::PromiseResolve => Some(typed::build_promise_resolve()),
        RuntimeFn::PromiseReject => Some(typed::build_promise_reject()),
        RuntimeFn::PromiseThen => Some(build_promise_then_native(&[])),
        RuntimeFn::PromiseCatch => Some(build_promise_catch_native(&[])),
        RuntimeFn::PromiseWithResolvers => Some(typed::build_promise_with_resolvers(
            data.strings.promise_with_resolvers_data(),
        )),
        RuntimeFn::PromiseFinally => Some(typed::build_promise_finally()),
        RuntimeFn::PromiseAll => Some(typed::build_promise_all()),
        RuntimeFn::PromiseAllSettled => Some(typed::build_promise_all_settled(
            data.strings.promise_all_settled_data(),
        )),
        RuntimeFn::PromiseAny => Some(typed::build_promise_any(data.strings.promise_any_data())),
        RuntimeFn::PromiseRace => Some(typed::build_promise_race()),
        RuntimeFn::AggregateError => Some(typed::build_aggregate_error(
            data.strings.aggregate_error_data(),
        )),
        RuntimeFn::ObjectHasOwn => Some(typed::build_object_has_own()),
        RuntimeFn::ObjectHasOwnProperty => Some(typed::build_object_has_own_property()),
        RuntimeFn::ObjectGetPrototypeOf => Some(typed::build_object_get_prototype_of()),
        RuntimeFn::ObjectSetPrototypeOf => Some(typed::build_object_set_prototype_of()),
        RuntimeFn::ObjectFreeze => Some(typed::build_object_freeze()),
        RuntimeFn::ObjectSeal => Some(typed::build_object_seal()),
        RuntimeFn::ObjectPreventExtensions => Some(typed::build_object_prevent_extensions()),
        RuntimeFn::ObjectIsExtensible => Some(typed::build_object_is_extensible()),
        RuntimeFn::ObjectIsSealed => Some(typed::build_object_is_sealed()),
        RuntimeFn::ObjectIsFrozen => Some(typed::build_object_is_frozen()),
        RuntimeFn::ObjectPrototype => Some(typed::build_object_prototype()),
        RuntimeFn::GlobalThis => Some(typed::build_global_this(
            data.property_get.native_errors.as_slice(),
        )),
        RuntimeFn::ObjectCreate => Some(typed::build_object_create()),
        RuntimeFn::ObjectToObject => Some(typed::build_object_to_object()),
        RuntimeFn::ObjectIs => Some(typed::build_object_is()),
        RuntimeFn::IsPrototypeOf => Some(typed::build_is_prototype_of()),
        RuntimeFn::PropertyIsEnumerable => Some(typed::build_property_is_enumerable()),
        RuntimeFn::ObjectToString => Some(typed::build_object_to_string(
            data.strings.object_to_string_values(),
        )),
        RuntimeFn::ErrorToString => Some(typed::build_error_to_string(
            data.strings.error_to_string_data(),
        )),
        RuntimeFn::ObjectToLocaleString => Some(typed::build_object_to_locale_string()),
        RuntimeFn::ObjectAssign => Some(typed::build_object_assign()),
        RuntimeFn::ObjectGetOwnPropertyDescriptor => Some(
            typed::build_object_get_own_property_descriptor(data.property_get.clone()),
        ),
        RuntimeFn::ObjectGetOwnPropertyDescriptors => {
            Some(typed::build_object_get_own_property_descriptors())
        }
        RuntimeFn::ObjectGetOwnPropertyNames => Some(typed::build_object_get_own_property_names()),
        RuntimeFn::ObjectGetOwnPropertySymbols => {
            Some(typed::build_object_get_own_property_symbols())
        }
        RuntimeFn::ObjectFromEntries => Some(typed::build_object_from_entries()),
        RuntimeFn::ObjectKeys => Some(typed::build_object_keys()),
        RuntimeFn::ObjectSpread => Some(typed::build_object_spread()),
        RuntimeFn::RestObject => Some(typed::build_rest_object()),
        RuntimeFn::SpreadViaIterator => Some(typed::build_spread_via_iterator()),
        RuntimeFn::ObjectValues => Some(typed::build_object_values()),
        RuntimeFn::ObjectEntries => Some(typed::build_object_entries()),
        RuntimeFn::ObjectDefineProperties => Some(typed::build_object_define_properties()),
        RuntimeFn::ObjectDefineProperty => Some(typed::build_object_define_property()),
        RuntimeFn::PathBasename => Some(typed::build_path_basename()),
        RuntimeFn::PathDirname => Some(typed::build_path_dirname()),
        RuntimeFn::PathJoin => Some(typed::build_path_join()),
        RuntimeFn::PathResolve => Some(typed::build_path_resolve()),
        RuntimeFn::ReflectDefineProperty => Some(typed::build_reflect_define_property()),
        RuntimeFn::ReflectPreventExtensions => Some(typed::build_reflect_prevent_extensions()),
        RuntimeFn::ReflectSet => Some(typed::build_reflect_set()),
        RuntimeFn::ReflectSetPrototypeOf => Some(typed::build_reflect_set_prototype_of()),
        RuntimeFn::PropertyDelete => Some(typed::build_property_delete()),
        RuntimeFn::PropertyGet => Some(typed::build_property_get(data.property_get.clone())),
        RuntimeFn::PropertyHas => Some(typed::build_property_has()),
        RuntimeFn::PropertySet => Some(typed::build_property_set()),
        RuntimeFn::PrivateBrandTypeError => Some(typed::build_private_brand_type_error(
            data.strings.private_brand_type_error_data(),
        )),
        RuntimeFn::ReadStdinBytes => Some(typed::build_read_stdin_bytes()),
        RuntimeFn::RegexpMatchInner => Some(typed::build_regexp_match_inner()),
        RuntimeFn::RegexpParseFlags => Some(typed::build_regexp_parse_flags()),
        RuntimeFn::RegExpTest => Some(typed::build_regexp_test()),
        RuntimeFn::RegExpMatch => Some(typed::build_regexp_match()),
        RuntimeFn::RegExpSearch => Some(typed::build_regexp_search()),
        RuntimeFn::ReflectDeleteProperty => Some(typed::build_reflect_delete_property()),
        RuntimeFn::ReflectGet => Some(typed::build_reflect_get()),
        RuntimeFn::ReflectHas => Some(typed::build_reflect_has()),
        RuntimeFn::ReflectOwnKeys => Some(typed::build_reflect_own_keys()),
        RuntimeFn::ReflectApply => Some(typed::build_reflect_apply()),
        RuntimeFn::ReflectConstruct => Some(typed::build_reflect_construct()),
        RuntimeFn::ProcessArgv => Some(typed::build_process_argv()),
        RuntimeFn::ProcessEnv => Some(typed::build_process_env()),
        RuntimeFn::ProcessExit => Some(typed::build_process_exit()),
        RuntimeFn::Or => Some(typed::build_or()),
        RuntimeFn::SameValueZero => Some(typed::build_same_value_zero()),
        RuntimeFn::SetAdd => Some(typed::build_set_add()),
        RuntimeFn::SetClear => Some(typed::build_set_clear()),
        RuntimeFn::SetDelete => Some(typed::build_set_delete()),
        RuntimeFn::SetDifference => Some(typed::build_set_difference()),
        RuntimeFn::SetEntriesArray => Some(typed::build_set_entries_array()),
        RuntimeFn::SetForEach => Some(typed::build_set_for_each()),
        RuntimeFn::SetFromArray => Some(typed::build_set_from_array()),
        RuntimeFn::SetHas => Some(typed::build_set_has()),
        RuntimeFn::SetIntersection => Some(typed::build_set_intersection()),
        RuntimeFn::SetIsDisjointFrom => Some(typed::build_set_is_disjoint_from()),
        RuntimeFn::SetIsSubsetOf => Some(typed::build_set_is_subset_of()),
        RuntimeFn::SetIsSupersetOf => Some(typed::build_set_is_superset_of()),
        RuntimeFn::SetNew => Some(typed::build_set_new()),
        RuntimeFn::SetPrototypeAddGet => Some(typed::build_set_prototype_add_get()),
        RuntimeFn::SetPrototypeAddSet => Some(typed::build_set_prototype_add_set()),
        RuntimeFn::SetPrototypeDeleteGet => Some(typed::build_set_prototype_delete_get()),
        RuntimeFn::SetPrototypeDeleteSet => Some(typed::build_set_prototype_delete_set()),
        RuntimeFn::SetPrototypeForEachGet => Some(typed::build_set_prototype_for_each_get()),
        RuntimeFn::SetPrototypeForEachSet => Some(typed::build_set_prototype_for_each_set()),
        RuntimeFn::SetPrototypeHasGet => Some(typed::build_set_prototype_has_get()),
        RuntimeFn::SetPrototypeHasSet => Some(typed::build_set_prototype_has_set()),
        RuntimeFn::SetSize => Some(typed::build_set_size()),
        RuntimeFn::SetSymmetricDifference => Some(typed::build_set_symmetric_difference()),
        RuntimeFn::SetUnion => Some(typed::build_set_union()),
        RuntimeFn::SetValuesArray => Some(typed::build_set_values_array()),
        RuntimeFn::SetValuesIterator => Some(typed::build_set_values_iterator()),
        RuntimeFn::SharedArrayBufferNew => Some(typed::build_shared_array_buffer_new()),
        RuntimeFn::StringAt => Some(typed::build_string_at()),
        RuntimeFn::StringCharAt => Some(typed::build_string_char_at()),
        RuntimeFn::StringCharCodeAt => Some(typed::build_string_char_code_at()),
        RuntimeFn::StringCodePointAt => Some(typed::build_string_code_point_at()),
        RuntimeFn::StringEndsWith => Some(typed::build_string_ends_with()),
        RuntimeFn::StringFromCharCode => Some(typed::build_string_from_char_code()),
        RuntimeFn::StringFromCodePoint => Some(typed::build_string_from_code_point()),
        RuntimeFn::StringIncludes => Some(typed::build_string_includes()),
        RuntimeFn::StringIndexOf => Some(typed::build_string_index_of()),
        RuntimeFn::StringEqual => Some(typed::build_string_equal()),
        RuntimeFn::StringIsWellFormed => Some(typed::build_string_is_well_formed()),
        RuntimeFn::StringLastIndexOf => Some(typed::build_string_last_index_of()),
        RuntimeFn::StringLocaleCompare => Some(typed::build_string_locale_compare()),
        RuntimeFn::StringMatch => Some(typed::build_string_match()),
        RuntimeFn::StringMatchAll => Some(typed::build_string_match_all(
            data.strings.string_match_all_data(),
        )),
        RuntimeFn::StringNormalize => Some(typed::build_string_normalize()),
        RuntimeFn::StringPadEnd => Some(typed::build_string_pad_end()),
        RuntimeFn::StringPadStart => Some(typed::build_string_pad_start()),
        RuntimeFn::StringReplace => Some(typed::build_string_replace()),
        RuntimeFn::StringReplaceAll => Some(typed::build_string_replace_all()),
        RuntimeFn::StringRepeat => Some(typed::build_string_repeat()),
        RuntimeFn::StringSearch => Some(typed::build_string_search()),
        RuntimeFn::StringSlice => Some(typed::build_string_slice()),
        RuntimeFn::StringSplit => Some(typed::build_string_split()),
        RuntimeFn::StringStartsWith => Some(typed::build_string_starts_with()),
        RuntimeFn::StringSubstr => Some(typed::build_string_substr()),
        RuntimeFn::StringSubstring => Some(typed::build_string_substring()),
        RuntimeFn::StringRaw => Some(typed::build_string_raw(
            data.strings.value(""),
            data.strings.get("raw"),
        )),
        RuntimeFn::StringToLowerCase => Some(typed::build_string_to_lower_case()),
        RuntimeFn::StringToLocaleString => Some(typed::build_string_to_locale_string()),
        RuntimeFn::StringTrim => Some(typed::build_string_trim()),
        RuntimeFn::StringTrimEnd => Some(typed::build_string_trim_end()),
        RuntimeFn::StringTrimStart => Some(typed::build_string_trim_start()),
        RuntimeFn::StringToUpperCase => Some(typed::build_string_to_upper_case()),
        RuntimeFn::StringToWellFormed => Some(typed::build_string_to_well_formed()),
        RuntimeFn::StrictEqual => Some(typed::build_strict_equal()),
        RuntimeFn::StrictNotEqual => Some(typed::build_strict_not_equal()),
        RuntimeFn::Sub => Some(typed::build_sub()),
        RuntimeFn::SubFast => Some(typed::build_sub_fast()),
        RuntimeFn::SuperCallExternal => Some(typed::build_super_call_external()),
        RuntimeFn::SymbolDescription => Some(typed::build_symbol_description()),
        RuntimeFn::SymbolFor => Some(typed::build_symbol_for(0, 0)),
        RuntimeFn::SymbolHasInstance => Some(typed::build_symbol_has_instance(
            data.strings.get("prototype"),
        )),
        RuntimeFn::SymbolKeyFor => Some(typed::build_symbol_key_for()),
        RuntimeFn::SymbolNew => Some(typed::build_symbol_new(0, 0, 0, ValueTag::UNDEFINED)),
        RuntimeFn::SymbolToPrimitive => Some(typed::build_symbol_to_primitive()),
        RuntimeFn::SymbolToString => Some(typed::build_symbol_to_string()),
        RuntimeFn::SymbolToStringTag => Some(typed::build_symbol_to_string_tag(
            data.strings.value("Symbol"),
        )),
        RuntimeFn::SymbolWellKnown => Some(typed::build_symbol_well_known()),
        RuntimeFn::TaskPoll => Some(typed::build_task_poll()),
        RuntimeFn::TaskDrop => Some(typed::build_task_drop()),
        RuntimeFn::TaskResult => Some(typed::build_task_result()),
        RuntimeFn::TypedArrayCtorFromBuffer => Some(typed::build_typed_array_ctor_from_buffer()),
        RuntimeFn::TypedArrayCtorWithLength => Some(typed::build_typed_array_ctor_with_length()),
        RuntimeFn::TypedArrayFromArray => Some(typed::build_typed_array_from_array()),
        RuntimeFn::TypedArrayLoad => Some(typed::build_typed_array_load()),
        RuntimeFn::TypedArraySet => Some(typed::build_typed_array_set()),
        RuntimeFn::TypedArrayStore => Some(typed::build_typed_array_store()),
        RuntimeFn::TypeOf => Some(typed::build_typeof(data.strings.typeof_values())),
        RuntimeFn::TruthyBool => Some(typed::build_truthy_bool()),
        RuntimeFn::ValueOf => Some(typed::build_value_of()),
        RuntimeFn::ValueToStringInto => Some(typed::build_value_to_string_into(
            data.strings.value_to_string_refs(),
        )),
        RuntimeFn::WeakMapDelete => Some(typed::build_weak_map_delete()),
        RuntimeFn::WeakMapGet => Some(typed::build_weak_map_get()),
        RuntimeFn::WeakMapHas => Some(typed::build_weak_map_has()),
        RuntimeFn::WeakRefDeref => Some(typed::build_weak_ref_deref()),
        RuntimeFn::WeakRefNew => Some(typed::build_weak_ref_new()),
        RuntimeFn::WeakMapNew => Some(typed::build_weak_map_new()),
        RuntimeFn::WeakMapSet => Some(typed::build_weak_map_set()),
        RuntimeFn::WeakSetAdd => Some(typed::build_weak_set_add()),
        RuntimeFn::WeakSetDelete => Some(typed::build_weak_set_delete()),
        RuntimeFn::WeakSetHas => Some(typed::build_weak_set_has()),
        RuntimeFn::WeakSetNew => Some(typed::build_weak_set_new()),
        RuntimeFn::Dollar262Global => Some(typed::build_dollar_262_global()),
        RuntimeFn::Write => Some(typed::build_write()),
        _ => None,
    }
}

fn push_promise_state_load(
    body: &mut Vec<WasmInstr>,
    promise: usize,
    base: usize,
    state: usize,
    value: usize,
) {
    body.extend([
        WasmInstr::LocalGet(promise),
        WasmInstr::I32Const(ValueTag::HEAP_MASK),
        WasmInstr::I32And,
        WasmInstr::LocalSet(base),
        WasmInstr::LocalGet(base),
        WasmInstr::I32Load {
            align: 2,
            offset: Layout::ARRAY_HEADER_SIZE,
        },
        WasmInstr::LocalSet(state),
        WasmInstr::LocalGet(base),
        WasmInstr::I32Load {
            align: 2,
            offset: Layout::ARRAY_HEADER_SIZE + 4,
        },
        WasmInstr::LocalSet(value),
    ]);
}

fn push_callback_call_args(
    body: &mut Vec<WasmInstr>,
    function: &LoweredFunction,
    user_param_count: usize,
    capture_count: usize,
    value: usize,
    payload: usize,
) {
    for param_index in 0..user_param_count {
        if param_index == 0 {
            body.push(WasmInstr::LocalGet(value));
        } else {
            body.push(WasmInstr::I32Const(ValueTag::UNDEFINED));
        }
    }
    for capture_index in 0..capture_count {
        body.push(WasmInstr::LocalGet(payload));
        body.push(WasmInstr::I32Load {
            align: 2,
            offset: CLOSURE_CAPTURE_SLOTS_OFFSET + capture_index as u32 * CLOSURE_CAPTURE_SLOT_SIZE,
        });
    }
    body.push(WasmInstr::Call(format!(
        "${}",
        function_symbol(function.id)
    )));
}

fn push_direct_callback_dispatch(
    body: &mut Vec<WasmInstr>,
    functions: &[LoweredFunction],
    callback: usize,
    value: usize,
    payload: usize,
    call_result: usize,
) {
    body.extend([
        WasmInstr::LocalGet(callback),
        WasmInstr::I32Const(ValueTag::NUMBER_SHIFT),
        WasmInstr::I32ShrU,
        WasmInstr::LocalSet(payload),
        WasmInstr::Block("$dispatch_done".to_owned()),
    ]);

    for function in functions {
        body.extend([
            WasmInstr::LocalGet(payload),
            WasmInstr::I32Const(ValueTag::DIRECT_LOCAL_TOKEN_PAYLOAD_BASE + function.id.0 as i32),
            WasmInstr::I32Eq,
            WasmInstr::If {
                result_ty: WasmBlockType::Empty,
            },
            WasmInstr::Then,
        ]);
        push_callback_call_args(body, function, function.params.len(), 0, value, payload);
        body.extend([
            WasmInstr::LocalSet(call_result),
            WasmInstr::Br("$dispatch_done".to_owned()),
            WasmInstr::End,
        ]);
    }

    body.extend([
        WasmInstr::I32Const(ValueTag::UNDEFINED),
        WasmInstr::LocalSet(call_result),
        WasmInstr::End,
    ]);
}

fn push_object_callback_dispatch(
    body: &mut Vec<WasmInstr>,
    functions: &[LoweredFunction],
    value: usize,
    payload: usize,
    call_result: usize,
) {
    body.push(WasmInstr::Block("$dispatch_done".to_owned()));
    for function in functions {
        for capture_count in 0..=function.params.len() {
            let user_param_count = function.params.len() - capture_count;
            body.extend([
                WasmInstr::LocalGet(payload),
                WasmInstr::I32Load {
                    align: 2,
                    offset: CLOSURE_CODE_ID_OFFSET,
                },
                WasmInstr::I32Const(function.id.0 as i32),
                WasmInstr::I32Eq,
                WasmInstr::LocalGet(payload),
                WasmInstr::I32Load {
                    align: 2,
                    offset: CLOSURE_CAPTURE_COUNT_OFFSET,
                },
                WasmInstr::I32Const(capture_count as i32),
                WasmInstr::I32Eq,
                WasmInstr::I32And,
                WasmInstr::If {
                    result_ty: WasmBlockType::Empty,
                },
                WasmInstr::Then,
            ]);
            push_callback_call_args(
                body,
                function,
                user_param_count,
                capture_count,
                value,
                payload,
            );
            body.extend([
                WasmInstr::LocalSet(call_result),
                WasmInstr::Br("$dispatch_done".to_owned()),
                WasmInstr::End,
            ]);
        }
    }
    body.extend([
        WasmInstr::I32Const(ValueTag::UNDEFINED),
        WasmInstr::LocalSet(call_result),
        WasmInstr::End,
    ]);
}

fn push_callable_callback_dispatch(
    body: &mut Vec<WasmInstr>,
    functions: &[LoweredFunction],
    callback: usize,
    value: usize,
    payload: usize,
    call_result: usize,
    non_callable_target: &str,
) {
    body.extend([
        WasmInstr::LocalGet(callback),
        WasmInstr::I32Const(ValueTag::TAG_MASK),
        WasmInstr::I32And,
        WasmInstr::I32Const(ValueTag::NUMBER),
        WasmInstr::I32Eq,
        WasmInstr::If {
            result_ty: WasmBlockType::Empty,
        },
        WasmInstr::Then,
    ]);
    push_direct_callback_dispatch(body, functions, callback, value, payload, call_result);
    body.extend([
        WasmInstr::LocalGet(call_result),
        WasmInstr::Call("$promise_resolve".to_owned()),
        WasmInstr::Return,
        WasmInstr::End,
        WasmInstr::LocalGet(callback),
        WasmInstr::I32Const(ValueTag::TAG_MASK),
        WasmInstr::I32And,
        WasmInstr::I32Const(ValueTag::OBJECT),
        WasmInstr::I32Ne,
        WasmInstr::If {
            result_ty: WasmBlockType::Empty,
        },
        WasmInstr::Then,
        WasmInstr::LocalGet(value),
        WasmInstr::Call(non_callable_target.to_owned()),
        WasmInstr::Return,
        WasmInstr::End,
        WasmInstr::LocalGet(callback),
        WasmInstr::I32Const(ValueTag::HEAP_MASK),
        WasmInstr::I32And,
        WasmInstr::LocalSet(payload),
        WasmInstr::LocalGet(payload),
        WasmInstr::I32Load {
            align: 2,
            offset: CLOSURE_SUBTYPE_OFFSET,
        },
        WasmInstr::I32Const(CLOSURE_SENTINEL),
        WasmInstr::I32Ne,
        WasmInstr::If {
            result_ty: WasmBlockType::Empty,
        },
        WasmInstr::Then,
        WasmInstr::LocalGet(value),
        WasmInstr::Call(non_callable_target.to_owned()),
        WasmInstr::Return,
        WasmInstr::End,
    ]);
    push_object_callback_dispatch(body, functions, value, payload, call_result);
    body.extend([
        WasmInstr::LocalGet(call_result),
        WasmInstr::Call("$promise_resolve".to_owned()),
        WasmInstr::Return,
    ]);
}

fn build_promise_then_native(functions: &[LoweredFunction]) -> WasmFunction {
    let base = 3;
    let state = 4;
    let value = 5;
    let call_result = 6;
    let payload = 7;
    let mut body = Vec::new();

    push_promise_state_load(&mut body, 0, base, state, value);
    body.extend([
        WasmInstr::LocalGet(state),
        WasmInstr::I32Const(1),
        WasmInstr::I32Eq,
        WasmInstr::If {
            result_ty: WasmBlockType::Empty,
        },
        WasmInstr::Then,
    ]);
    push_callable_callback_dispatch(
        &mut body,
        functions,
        1,
        value,
        payload,
        call_result,
        "$promise_resolve",
    );
    body.push(WasmInstr::End);

    body.extend([
        WasmInstr::LocalGet(state),
        WasmInstr::I32Const(2),
        WasmInstr::I32Eq,
        WasmInstr::If {
            result_ty: WasmBlockType::Empty,
        },
        WasmInstr::Then,
    ]);
    push_callable_callback_dispatch(
        &mut body,
        functions,
        2,
        value,
        payload,
        call_result,
        "$promise_reject",
    );
    body.push(WasmInstr::End);

    body.extend([
        WasmInstr::LocalGet(base),
        WasmInstr::LocalGet(1),
        WasmInstr::I32Store {
            align: 2,
            offset: Layout::ARRAY_HEADER_SIZE + 8,
        },
        WasmInstr::LocalGet(base),
        WasmInstr::LocalGet(2),
        WasmInstr::I32Store {
            align: 2,
            offset: Layout::ARRAY_HEADER_SIZE + 12,
        },
        WasmInstr::LocalGet(0),
    ]);

    WasmFunction::new("$promise_then")
        .param(WasmValType::I32)
        .param(WasmValType::I32)
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .local(WasmValType::I32)
        .local(WasmValType::I32)
        .local(WasmValType::I32)
        .local(WasmValType::I32)
        .local(WasmValType::I32)
        .body(body)
}

fn build_promise_catch_native(functions: &[LoweredFunction]) -> WasmFunction {
    let base = 2;
    let state = 3;
    let value = 4;
    let call_result = 5;
    let payload = 6;
    let mut body = Vec::new();

    push_promise_state_load(&mut body, 0, base, state, value);
    body.extend([
        WasmInstr::LocalGet(state),
        WasmInstr::I32Const(2),
        WasmInstr::I32Eq,
        WasmInstr::If {
            result_ty: WasmBlockType::Empty,
        },
        WasmInstr::Then,
    ]);
    push_callable_callback_dispatch(
        &mut body,
        functions,
        1,
        value,
        payload,
        call_result,
        "$promise_reject",
    );
    body.push(WasmInstr::End);

    body.extend([
        WasmInstr::LocalGet(state),
        WasmInstr::I32Const(1),
        WasmInstr::I32Eq,
        WasmInstr::If {
            result_ty: WasmBlockType::Empty,
        },
        WasmInstr::Then,
        WasmInstr::LocalGet(value),
        WasmInstr::Call("$promise_resolve".to_owned()),
        WasmInstr::Return,
        WasmInstr::End,
        WasmInstr::LocalGet(base),
        WasmInstr::LocalGet(1),
        WasmInstr::I32Store {
            align: 2,
            offset: Layout::ARRAY_HEADER_SIZE + 12,
        },
        WasmInstr::LocalGet(0),
    ]);

    WasmFunction::new("$promise_catch")
        .param(WasmValType::I32)
        .param(WasmValType::I32)
        .result(WasmValType::I32)
        .local(WasmValType::I32)
        .local(WasmValType::I32)
        .local(WasmValType::I32)
        .local(WasmValType::I32)
        .local(WasmValType::I32)
        .body(body)
}

fn build_native_runtime_functions_for_program(
    runtime_fn: RuntimeFn,
    data: &NativeRuntimeData,
    program: &LoweredProgram,
) -> Vec<WasmFunction> {
    match runtime_fn {
        RuntimeFn::PromiseThen => vec![build_promise_then_native(&program.functions)],
        RuntimeFn::PromiseCatch => vec![build_promise_catch_native(&program.functions)],
        _ => build_native_runtime_functions_for(runtime_fn, data),
    }
}

fn build_native_runtime_functions_for(
    runtime_fn: RuntimeFn,
    data: &NativeRuntimeData,
) -> Vec<WasmFunction> {
    match runtime_fn {
        RuntimeFn::BigIntAdd => vec![
            typed::build_bigint_abs_data(),
            typed::build_bigint_abs_len(),
            typed::build_bigint_from_decimal_slice(),
            typed::build_bigint_copy_with_sign(),
            typed::build_bigint_decimal_abs_cmp(),
            typed::build_bigint_add_abs_decimal(),
            typed::build_bigint_sub_abs_decimal(),
            typed::build_bigint_add_core(),
            typed::build_bigint_add(),
        ],
        RuntimeFn::BigIntAsIntN => vec![
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_signed_i64(),
            typed::build_bigint_index_0_64(),
            typed::build_bigint_as_int_n(),
        ],
        RuntimeFn::BigIntAsUintN => vec![
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_from_unsigned_i64(),
            typed::build_bigint_signed_i64(),
            typed::build_bigint_index_0_64(),
            typed::build_bigint_as_uint_n(),
        ],
        RuntimeFn::BigIntBitwiseAnd => vec![
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_signed_i64(),
            typed::build_bigint_bitwise_and(),
        ],
        RuntimeFn::BigIntBitwiseNot => vec![
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_signed_i64(),
            typed::build_bigint_bitwise_not(),
        ],
        RuntimeFn::BigIntBitwiseOr => vec![
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_signed_i64(),
            typed::build_bigint_bitwise_or(),
        ],
        RuntimeFn::BigIntBitwiseXor => vec![
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_signed_i64(),
            typed::build_bigint_bitwise_xor(),
        ],
        RuntimeFn::BigIntCompare => vec![typed::build_is_bigint(), typed::build_bigint_compare()],
        RuntimeFn::ObjectIs => vec![typed::build_is_bigint(), typed::build_object_is()],
        RuntimeFn::BigIntDiv => vec![
            typed::build_bigint_abs_data(),
            typed::build_bigint_abs_len(),
            typed::build_bigint_from_decimal_slice(),
            typed::build_bigint_decimal_trim(),
            typed::build_bigint_decimal_cmp(),
            typed::build_bigint_decimal_sub_in_place(),
            typed::build_bigint_div_rem_decimal(),
            typed::build_bigint_div(),
        ],
        RuntimeFn::BigIntFromValue => vec![
            typed::build_is_bigint(),
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_from_unsigned_i64(),
            typed::build_bigint_from_string(),
            typed::build_bigint_from_value(),
        ],
        RuntimeFn::BigIntLeftShift => vec![
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_signed_i64(),
            typed::build_bigint_left_shift(),
        ],
        RuntimeFn::BigIntMul => vec![
            typed::build_bigint_abs_data(),
            typed::build_bigint_abs_len(),
            typed::build_bigint_from_decimal_slice(),
            typed::build_bigint_mul(),
        ],
        RuntimeFn::BigIntPow => vec![
            typed::build_bigint_abs_data(),
            typed::build_bigint_abs_len(),
            typed::build_bigint_from_decimal_slice(),
            typed::build_bigint_decimal_trim(),
            typed::build_bigint_decimal_decrement_in_place(),
            typed::build_bigint_pow(),
        ],
        RuntimeFn::BigIntRem => vec![
            typed::build_bigint_abs_data(),
            typed::build_bigint_abs_len(),
            typed::build_bigint_from_decimal_slice(),
            typed::build_bigint_decimal_trim(),
            typed::build_bigint_decimal_cmp(),
            typed::build_bigint_decimal_sub_in_place(),
            typed::build_bigint_div_rem_decimal(),
            typed::build_bigint_rem(),
        ],
        RuntimeFn::BigIntRightShift => vec![
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_signed_i64(),
            typed::build_bigint_right_shift(),
        ],
        RuntimeFn::BigIntSub => vec![
            typed::build_bigint_abs_data(),
            typed::build_bigint_abs_len(),
            typed::build_bigint_from_decimal_slice(),
            typed::build_bigint_copy_with_sign(),
            typed::build_bigint_decimal_abs_cmp(),
            typed::build_bigint_add_abs_decimal(),
            typed::build_bigint_sub_abs_decimal(),
            typed::build_bigint_add_core(),
            typed::build_bigint_sub(),
        ],
        RuntimeFn::BigIntUnaryMinus => vec![
            typed::build_bigint_abs_data(),
            typed::build_bigint_abs_len(),
            typed::build_bigint_from_decimal_slice(),
            typed::build_bigint_copy_with_sign(),
            typed::build_bigint_unary_minus(),
        ],
        RuntimeFn::DataViewGetBigInt64 => vec![
            typed::build_bigint_from_signed_i64(),
            typed::build_dataview_get_bigint64(),
        ],
        RuntimeFn::DataViewGetBigUint64 => vec![
            typed::build_bigint_from_unsigned_i64(),
            typed::build_dataview_get_biguint64(),
        ],
        RuntimeFn::DataViewSetBigInt64 | RuntimeFn::DataViewSetBigUint64 => vec![
            typed::build_bigint_abs_data(),
            typed::build_bigint_abs_len(),
            typed::build_alloc_heap_with_memory_max_pages(NATIVE_MEMORY_MAX_PAGES),
            typed::build_copy(),
            typed::build_is_bigint(),
            typed::build_is_string(),
            typed::build_make_bigint_literal(),
            typed::build_bigint_from_signed_i64(),
            typed::build_bigint_from_unsigned_i64(),
            typed::build_bigint_from_string(),
            typed::build_bigint_from_value(),
            typed::build_bigint_signed_i64(),
            build_native_runtime_function(runtime_fn, data)
                .expect("DataView BigInt setter has native builder"),
        ],
        RuntimeFn::EqualEqual => vec![
            typed::build_is_bigint(),
            typed::build_string_to_number_for_equality(),
            typed::build_primitive_to_number_for_equality(),
            typed::build_bigint_string_to_small_int_for_comparison(),
            typed::build_bigint_equal_small_int(),
            typed::build_equal_equal(),
        ],
        RuntimeFn::GlobalParseFloat => vec![
            typed::build_parse_float_string(),
            typed::build_global_parse_float(),
        ],
        RuntimeFn::GlobalParseInt => vec![
            typed::build_parse_int_string(),
            typed::build_global_parse_int(),
        ],
        RuntimeFn::IsFinite => vec![typed::build_is_finite_string(), typed::build_is_finite()],
        RuntimeFn::IsNaN => vec![typed::build_is_nan_string(), typed::build_is_nan()],
        _ => build_native_runtime_function(runtime_fn, data)
            .into_iter()
            .collect(),
    }
}

#[cfg(test)]
fn native_helper_symbols_for(runtime_fn: RuntimeFn) -> &'static [&'static str] {
    match runtime_fn {
        RuntimeFn::GetLength => &["$utf8_cp_count"],
        RuntimeFn::Index => &["$utf8_cp_to_byte_index", "$utf8_cp_byte_length"],
        RuntimeFn::StringCharAt => &["$utf8_cp_to_byte_index", "$utf8_cp_byte_length"],
        RuntimeFn::StringAt => &[
            "$utf8_cp_count",
            "$utf8_cp_to_byte_index",
            "$utf8_cp_byte_length",
        ],
        RuntimeFn::StringSubstring | RuntimeFn::StringSlice => {
            &["$utf8_cp_count", "$utf8_cp_to_byte_index"]
        }
        RuntimeFn::StringSubstr => &["$utf8_cp_count"],
        RuntimeFn::StringIndexOf | RuntimeFn::StringLastIndexOf => &["$utf8_byte_to_cp_index"],
        RuntimeFn::StringCharCodeAt => &["$utf8_cp_count", "$utf8_cp_to_byte_index"],
        RuntimeFn::StringCodePointAt => &[
            "$utf8_cp_count",
            "$utf8_cp_to_byte_index",
            "$string_char_code_at",
        ],
        RuntimeFn::NumberCoerce => &["$parse_int_string"],
        RuntimeFn::DataViewGetBigInt64 => &["$bigint_from_signed_i64"],
        RuntimeFn::DataViewGetBigUint64 => &["$bigint_from_unsigned_i64"],
        RuntimeFn::DataViewSetBigInt64 | RuntimeFn::DataViewSetBigUint64 => &["$bigint_signed_i64"],
        RuntimeFn::ArrayMapUnaryPlus => &["$primitive_to_number_for_equality"],
        RuntimeFn::StrictEqual | RuntimeFn::SameValueZero => &["$is_bigint"],
        runtime_fn if is_relational_runtime(runtime_fn) => &[
            "$bigint_compare_small_int",
            "$bigint_compare_decimal_string_for_relational",
            "$bigint_string_to_small_int_for_comparison",
            "$is_bigint",
        ],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm_encoder_backend::emit_wasm_module_binary;
    use crate::wasm_ir::{
        WasmGlobal, WasmInstr, WasmMemory, WasmModule, WasmValType, wasm_import_from_host_spec,
    };
    use std::collections::{BTreeMap, BTreeSet};
    use ts2wasm_ir::builtin::BuiltinId;
    use ts2wasm_ir::lowered::{
        BuiltinErrorConstructor, FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredFunction,
        LoweredProgram, LoweredStmt, LoweredUnaryOp, RuntimeFn, Validated,
    };
    use ts2wasm_runtime_abi::ValueTag;
    use ts2wasm_source::Span;

    #[test]
    fn embeds_only_available_native_runtime_functions_from_link_plan() {
        let span = Span::generated("native-runtime-embed");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Unary {
                    op: LoweredUnaryOp::Not,
                    expr: Box::new(LoweredExpr::Local(LocalId(0), span)),
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let symbols = embed_native_runtime_functions(&program)
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();

        // Native emitter inlines Not as i32.eqz and TruthyBool is not needed
        // for simple locals, so $truthy_bool and $not may not appear.
        assert!(!symbols.contains(&"$property_get".to_owned()));
        assert!(
            symbols.iter().all(
                |symbol| RuntimeFn::emission_order().iter().any(|runtime_fn| {
                    runtime_fn.symbol() == symbol && native_runtime_function_available(*runtime_fn)
                })
            )
        );
    }

    #[test]
    fn native_runtime_embedding_uses_runtime_fn_emission_order() {
        let span = Span::generated("native-runtime-emission-order");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ArrayPush,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Number(1, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::TruthyBool,
                        args: vec![LoweredExpr::Local(LocalId(1), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![],
            modules: vec![],
        };

        let ordered = ordered_required_native_runtime_functions(&program);
        let expected = RuntimeFn::emission_order()
            .iter()
            .copied()
            .filter(|runtime_fn| ordered.contains(runtime_fn))
            .collect::<Vec<_>>();

        assert_eq!(ordered, expected);
        assert_eq!(
            embed_native_runtime_functions(&program)
                .into_iter()
                .map(|function| function.symbol)
                .collect::<Vec<_>>(),
            ordered
                .into_iter()
                .map(|runtime_fn| runtime_fn.symbol().to_owned())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn native_runtime_embedding_elides_pseudo_intrinsics() {
        let span = Span::generated("native-runtime-pseudo");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ArrayPushMany,
                    args: vec![
                        LoweredExpr::Local(LocalId(0), span),
                        LoweredExpr::Number(1, span),
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        assert!(
            ordered_required_native_runtime_functions(&program)
                .into_iter()
                .all(native_runtime_function_available)
        );
        assert!(
            embed_native_runtime_functions(&program)
                .into_iter()
                .all(|function| !function.symbol.starts_with("$pseudo_"))
        );
    }

    #[test]
    fn native_runtime_builder_coverage_reports_missing_non_pseudo_builders() {
        let missing = native_runtime_builder_missing();

        assert!(
            !missing.contains(&RuntimeFn::ErrorMessage),
            "coverage should not report available Core ABI seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Log),
            "coverage should not report available Core ABI seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::LogWarn),
            "coverage should not report available Core ABI seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::LogError),
            "coverage should not report available Core ABI seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ReadStdinBytes),
            "coverage should not report available stdin seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ConsoleGroupStart),
            "coverage should not report available console seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ConsoleGroupEndFn),
            "coverage should not report available console seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ConsoleTimeStart),
            "coverage should not report available console seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ConsoleTimeEndFn),
            "coverage should not report available console seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ConsoleCountImpl),
            "coverage should not report available console seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ConsoleCountResetImpl),
            "coverage should not report available console seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ValueToStringInto),
            "coverage should not report available Core ABI seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AllocHeap),
            "coverage should not report available Core ABI seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ArrayGet),
            "coverage should not report available array seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ArrayIndexPresent),
            "coverage should not report available array seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BitwiseToI32),
            "coverage should not report available bitwise seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BitwiseAnd),
            "coverage should not report available bitwise seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BitwiseXor),
            "coverage should not report available bitwise seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BitwiseOr),
            "coverage should not report available bitwise seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Negate),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Less),
            "coverage should not report available comparison seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::LessFast),
            "coverage should not report available comparison seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::LessEqual),
            "coverage should not report available comparison seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::LessEqualFast),
            "coverage should not report available comparison seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Greater),
            "coverage should not report available comparison seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::GreaterFast),
            "coverage should not report available comparison seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::GreaterEqual),
            "coverage should not report available comparison seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::GreaterEqualFast),
            "coverage should not report available comparison seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Add),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AddFast),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Sub),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::SubFast),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Mul),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::MulFast),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Div),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::DivFast),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Mod),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ModFast),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::StringEqual),
            "coverage should not report available string seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::StringCharCodeAt),
            "coverage should not report available string seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::StringCodePointAt),
            "coverage should not report available string seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::StringFromCharCode),
            "coverage should not report available string seed builders as missing"
        );
        for runtime_fn in [
            RuntimeFn::StringIsWellFormed,
            RuntimeFn::StringToWellFormed,
            RuntimeFn::StringNormalize,
            RuntimeFn::StringToLocaleString,
        ] {
            assert!(
                !missing.contains(&runtime_fn),
                "coverage should not report available string seed builders as missing"
            );
        }
        assert!(
            !missing.contains(&RuntimeFn::StringFromCodePoint),
            "coverage should not report available string seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Concat),
            "coverage should not report available string seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntCompare),
            "coverage should not report available BigInt seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntAdd),
            "coverage should not report available BigInt arithmetic builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntAsIntN),
            "coverage should not report available BigInt i64 builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntAsUintN),
            "coverage should not report available BigInt i64 builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntBitwiseNot),
            "coverage should not report available BigInt i64 builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntBitwiseAnd),
            "coverage should not report available BigInt i64 builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntBitwiseOr),
            "coverage should not report available BigInt i64 builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntBitwiseXor),
            "coverage should not report available BigInt i64 builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntLeftShift),
            "coverage should not report available BigInt i64 builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntRightShift),
            "coverage should not report available BigInt i64 builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntSub),
            "coverage should not report available BigInt arithmetic builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntMul),
            "coverage should not report available BigInt arithmetic builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntPow),
            "coverage should not report available BigInt arithmetic builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntDiv),
            "coverage should not report available BigInt arithmetic builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntFromValue),
            "coverage should not report available BigInt conversion builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntRem),
            "coverage should not report available BigInt arithmetic builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::MakeBigIntLiteral),
            "coverage should not report available BigInt seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntDivisionByZeroRangeError),
            "coverage should not report available BigInt seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntMixedArithmeticTypeError),
            "coverage should not report available BigInt seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntStringComparisonBoundaryError),
            "coverage should not report available BigInt seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntToBoolean),
            "coverage should not report available BigInt seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntToString),
            "coverage should not report available BigInt seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BigIntUnaryMinus),
            "coverage should not report available BigInt unary builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::PrivateBrandTypeError),
            "coverage should not report available private brand error builder as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::MapNew),
            "coverage should not report available collection seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::SetNew),
            "coverage should not report available collection seed builders as missing"
        );
        for runtime_fn in [
            RuntimeFn::MapGet,
            RuntimeFn::MapSet,
            RuntimeFn::MapHas,
            RuntimeFn::MapDelete,
            RuntimeFn::MapForEach,
            RuntimeFn::MapClear,
            RuntimeFn::MapSize,
            RuntimeFn::MapValuesArray,
            RuntimeFn::MapKeysArray,
            RuntimeFn::MapEntriesArray,
            RuntimeFn::MapEntryPairsArray,
            RuntimeFn::SetAdd,
            RuntimeFn::SetHas,
            RuntimeFn::SetDelete,
            RuntimeFn::SetForEach,
            RuntimeFn::SetFromArray,
            RuntimeFn::SetIsDisjointFrom,
            RuntimeFn::SetIsSubsetOf,
            RuntimeFn::SetIsSupersetOf,
            RuntimeFn::SetUnion,
            RuntimeFn::SetIntersection,
            RuntimeFn::SetDifference,
            RuntimeFn::SetSymmetricDifference,
            RuntimeFn::SetSize,
            RuntimeFn::SetClear,
            RuntimeFn::SetValuesArray,
            RuntimeFn::SetEntriesArray,
            RuntimeFn::SetPrototypeAddGet,
            RuntimeFn::SetPrototypeAddSet,
            RuntimeFn::SetPrototypeHasGet,
            RuntimeFn::SetPrototypeHasSet,
            RuntimeFn::SetPrototypeDeleteGet,
            RuntimeFn::SetPrototypeDeleteSet,
            RuntimeFn::SetPrototypeForEachGet,
            RuntimeFn::SetPrototypeForEachSet,
            RuntimeFn::MapPrototypeGetGet,
            RuntimeFn::MapPrototypeGetSet,
            RuntimeFn::MapPrototypeSetGet,
            RuntimeFn::MapPrototypeSetSet,
            RuntimeFn::MapPrototypeHasGet,
            RuntimeFn::MapPrototypeHasSet,
            RuntimeFn::MapPrototypeDeleteGet,
            RuntimeFn::MapPrototypeDeleteSet,
            RuntimeFn::MapPrototypeForEachGet,
            RuntimeFn::MapPrototypeForEachSet,
        ] {
            assert!(
                !missing.contains(&runtime_fn),
                "coverage should not report available collection method builders as missing"
            );
        }
        assert!(
            !missing.contains(&RuntimeFn::WeakRefNew),
            "coverage should not report available weak ref seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::WeakRefDeref),
            "coverage should not report available weak ref seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::WeakMapNew),
            "coverage should not report available weak collection seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::WeakSetNew),
            "coverage should not report available weak collection seed builders as missing"
        );
        for runtime_fn in [
            RuntimeFn::WeakMapSet,
            RuntimeFn::WeakMapGet,
            RuntimeFn::WeakMapHas,
            RuntimeFn::WeakMapDelete,
            RuntimeFn::WeakSetAdd,
            RuntimeFn::WeakSetHas,
            RuntimeFn::WeakSetDelete,
        ] {
            assert!(
                !missing.contains(&runtime_fn),
                "coverage should not report available weak collection method builders as missing"
            );
        }
        assert!(
            !missing.contains(&RuntimeFn::FinalizationRegistryNew),
            "coverage should not report available finalization registry seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::FinalizationRegistryRegister),
            "coverage should not report available finalization registry seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::FinalizationRegistryUnregister),
            "coverage should not report available finalization registry seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ArrayBufferNew),
            "coverage should not report available ArrayBuffer seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ArrayBufferIsView),
            "coverage should not report available ArrayBuffer seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ArrayBufferTransfer),
            "coverage should not report available ArrayBuffer seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ArrayBufferSlice),
            "coverage should not report available ArrayBuffer seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsIsLockFree),
            "coverage should not report available Atomics sentinel builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsElementPtr),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsLoad),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsStore),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsAdd),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsSub),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsAnd),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsOr),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsXor),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsExchange),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsCompareExchange),
            "coverage should not report available Atomics value builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsWait),
            "coverage should not report available Atomics sentinel builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsWaitAsync),
            "coverage should not report available Atomics sentinel builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::AtomicsNotify),
            "coverage should not report available Atomics sentinel builders as missing"
        );
        for runtime_fn in [
            RuntimeFn::TypedArrayFromArray,
            RuntimeFn::TypedArrayCtorFromBuffer,
            RuntimeFn::TypedArrayCtorWithLength,
            RuntimeFn::TypedArraySet,
            RuntimeFn::TypedArrayLoad,
            RuntimeFn::TypedArrayStore,
        ] {
            assert!(
                !missing.contains(&runtime_fn),
                "coverage should not report available TypedArray builders as missing"
            );
        }
        assert!(
            !missing.contains(&RuntimeFn::DataViewNew),
            "coverage should not report available DataView seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::DataViewGetBuffer),
            "coverage should not report available DataView seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::DataViewGetByteOffset),
            "coverage should not report available DataView seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::DataViewGetInt8),
            "coverage should not report available DataView byte builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::DataViewGetUint8),
            "coverage should not report available DataView byte builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::DataViewSetInt8),
            "coverage should not report available DataView byte builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::DataViewSetUint8),
            "coverage should not report available DataView byte builders as missing"
        );
        for runtime_fn in [
            RuntimeFn::DataViewGetInt16,
            RuntimeFn::DataViewSetInt16,
            RuntimeFn::DataViewGetUint16,
            RuntimeFn::DataViewSetUint16,
            RuntimeFn::DataViewGetInt32,
            RuntimeFn::DataViewSetInt32,
            RuntimeFn::DataViewGetUint32,
            RuntimeFn::DataViewSetUint32,
            RuntimeFn::DataViewGetFloat16,
            RuntimeFn::DataViewSetFloat16,
            RuntimeFn::DataViewGetFloat32,
            RuntimeFn::DataViewSetFloat32,
            RuntimeFn::DataViewGetFloat64,
            RuntimeFn::DataViewSetFloat64,
            RuntimeFn::DataViewGetBigInt64,
            RuntimeFn::DataViewSetBigInt64,
            RuntimeFn::DataViewGetBigUint64,
            RuntimeFn::DataViewSetBigUint64,
        ] {
            assert!(
                !missing.contains(&runtime_fn),
                "coverage should not report available DataView wide integer builders as missing"
            );
        }
        for runtime_fn in [
            RuntimeFn::DateNew,
            RuntimeFn::DateEpochMsNowNumber,
            RuntimeFn::DateNewLive,
            RuntimeFn::DateNow,
            RuntimeFn::DateGetTime,
            RuntimeFn::DateSetTime,
            RuntimeFn::DateGetUtcMilliseconds,
            RuntimeFn::DateGetUtcSeconds,
            RuntimeFn::DateGetUtcMinutes,
            RuntimeFn::DateGetUtcHours,
            RuntimeFn::DateGetUtcDay,
            RuntimeFn::DateGetUtcDate,
            RuntimeFn::DateGetUtcMonth,
            RuntimeFn::DateGetUtcFullYear,
            RuntimeFn::DateSetUTCFullYear,
            RuntimeFn::DateSetUTCMonth,
            RuntimeFn::DateSetUTCDate,
            RuntimeFn::DateSetUTCHours,
            RuntimeFn::DateSetUTCMinutes,
            RuntimeFn::DateSetUTCSeconds,
            RuntimeFn::DateSetUTCMilliseconds,
            RuntimeFn::DateSetFullYear,
            RuntimeFn::DateSetMonth,
            RuntimeFn::DateSetDate,
            RuntimeFn::DateSetHours,
            RuntimeFn::DateSetMinutes,
            RuntimeFn::DateSetSeconds,
            RuntimeFn::DateSetMilliseconds,
            RuntimeFn::DateSetYear,
            RuntimeFn::DateParse,
            RuntimeFn::DateUTC,
            RuntimeFn::DateToString,
            RuntimeFn::DateGetLocalTimeField,
            RuntimeFn::DateToISOString,
            RuntimeFn::DateGetTimezoneOffset,
            RuntimeFn::DateToDateString,
            RuntimeFn::DateToTimeString,
            RuntimeFn::DateGetYear,
            RuntimeFn::DateToGMTString,
            RuntimeFn::IntlDateTimeFormatFormat,
            RuntimeFn::IntlNumberFormatFormat,
        ] {
            assert!(
                !missing.contains(&runtime_fn),
                "coverage should not report available Date/Intl seed builders as missing"
            );
        }
        assert!(
            !missing.contains(&RuntimeFn::SharedArrayBufferNew),
            "coverage should not report available ArrayBuffer seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::StrictEqual),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::SameValueZero),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::EqualEqual),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BangEqual),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::StrictNotEqual),
            "coverage should not report available operator seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::NumberIsNaN),
            "coverage should not report available number seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::NumberIsFinite),
            "coverage should not report available number seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::NumberIsInteger),
            "coverage should not report available number seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::NumberIsSafeInteger),
            "coverage should not report available number seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::GlobalParseInt),
            "coverage should not report available type-coercion seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::GlobalParseFloat),
            "coverage should not report available type-coercion seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::NumberCoerce),
            "coverage should not report available type-coercion seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::IsNaN),
            "coverage should not report available type-coercion seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::IsFinite),
            "coverage should not report available type-coercion seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BooleanCoerce),
            "coverage should not report available type-coercion seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::BooleanToString),
            "coverage should not report available type-coercion seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::TruthyBool),
            "coverage should not report available seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ValueOf),
            "coverage should not report available type-coercion seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ObjectHasOwnProperty),
            "coverage should not report available object seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ObjectHasOwn),
            "coverage should not report available object seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::PropertyHas),
            "coverage should not report available object seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::PropertyGet),
            "coverage should not report available object seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::PropertySet),
            "coverage should not report available object seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::PropertyDelete),
            "coverage should not report available object seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ReflectDeleteProperty),
            "coverage should not report available object seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ReflectGet),
            "coverage should not report available object seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ReflectHas),
            "coverage should not report available object seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::Index),
            "coverage should not report available collection seed builders as missing"
        );
        assert!(
            !missing.contains(&RuntimeFn::ArrayPushMany),
            "coverage should not report pseudo-intrinsics as missing builders"
        );
        assert!(
            missing
                .iter()
                .all(|runtime_fn| !is_pseudo_runtime_function(*runtime_fn)
                    && !native_runtime_function_available(*runtime_fn)),
            "missing builder coverage must contain only non-pseudo unavailable RuntimeFn variants"
        );
        assert!(
            missing.is_empty(),
            "native runtime builder coverage must remain complete; missing: {missing:?}"
        );
    }

    #[test]
    fn available_native_runtime_builders_match_catalog_stack_effects() {
        for runtime_fn in RuntimeFn::emission_order()
            .iter()
            .copied()
            .filter(|runtime_fn| native_runtime_function_available(*runtime_fn))
        {
            let data = NativeRuntimeData::default();
            let function = build_native_runtime_function(runtime_fn, &data)
                .expect("available builder should exist");
            let sig = runtime_fn.stack_effect();

            assert_eq!(
                function.params.len(),
                sig.params,
                "{runtime_fn:?} params must match RuntimeFn::stack_effect()"
            );
            assert_eq!(
                function.results.len(),
                sig.results,
                "{runtime_fn:?} results must match RuntimeFn::stack_effect()"
            );
            assert!(
                function
                    .body
                    .iter()
                    .all(|instr| !matches!(instr, crate::wasm_ir::WasmInstr::Raw(_))),
                "{runtime_fn:?} builder must not use WasmInstr::Raw"
            );
        }
    }

    #[test]
    fn available_native_runtime_builder_calls_are_declared() {
        for runtime_fn in RuntimeFn::emission_order()
            .iter()
            .copied()
            .filter(|runtime_fn| native_runtime_function_available(*runtime_fn))
        {
            let data = NativeRuntimeData::default();
            let functions = build_native_runtime_functions_for(runtime_fn, &data);
            let bundle_symbols = functions
                .iter()
                .map(|function| function.symbol.as_str())
                .collect::<BTreeSet<_>>();
            let declared_runtime_deps = runtime_fn
                .spec()
                .deps
                .iter()
                .map(|dep| dep.symbol())
                .collect::<BTreeSet<_>>();
            let declared_imports = runtime_fn
                .spec()
                .imports
                .iter()
                .map(|import| import.spec().wat_symbol)
                .collect::<BTreeSet<_>>();
            let native_helpers = native_helper_symbols_for(runtime_fn)
                .iter()
                .copied()
                .collect::<BTreeSet<_>>();

            for function in &functions {
                for call in function.body.iter().filter_map(|instr| match instr {
                    crate::wasm_ir::WasmInstr::Call(symbol) => Some(symbol.as_str()),
                    _ => None,
                }) {
                    assert!(
                        bundle_symbols.contains(call)
                            || declared_runtime_deps.contains(call)
                            || declared_imports.contains(call)
                            || native_helpers.contains(call),
                        "{runtime_fn:?} builder emits undeclared call {call} from {}",
                        function.symbol
                    );
                }
            }
        }
    }

    fn available_native_runtime_functions_with_helpers() -> Vec<WasmFunction> {
        let data = NativeRuntimeData::default();
        let mut functions = vec![
            typed::build_utf8_cp_count(),
            typed::build_utf8_cp_to_byte_index(),
            typed::build_utf8_cp_byte_length(),
            typed::build_utf8_byte_to_cp_index(),
            typed::build_bigint_compare_small_int(),
            typed::build_bigint_compare_decimal_string_for_relational(),
            typed::build_bigint_string_to_small_int_for_comparison(),
        ];
        functions.extend(
            RuntimeFn::emission_order()
                .iter()
                .copied()
                .filter(|runtime_fn| native_runtime_function_available(*runtime_fn))
                .flat_map(|runtime_fn| build_native_runtime_functions_for(runtime_fn, &data)),
        );

        let mut deduped = Vec::new();
        let mut seen = BTreeSet::new();
        for function in functions {
            if seen.insert(function.symbol.clone()) {
                deduped.push(function);
            }
        }
        deduped
    }

    fn collect_function_calls(functions: &[WasmFunction]) -> BTreeSet<String> {
        functions
            .iter()
            .flat_map(|function| function.body.iter())
            .filter_map(|instr| match instr {
                WasmInstr::Call(symbol) => Some(symbol.clone()),
                _ => None,
            })
            .collect()
    }

    fn collect_function_globals(functions: &[WasmFunction]) -> BTreeSet<String> {
        functions
            .iter()
            .flat_map(|function| function.body.iter())
            .filter_map(|instr| match instr {
                WasmInstr::GlobalGet(symbol) | WasmInstr::GlobalSet(symbol) => Some(symbol.clone()),
                _ => None,
            })
            .collect()
    }

    fn runtime_function_symbols() -> BTreeMap<&'static str, RuntimeFn> {
        RuntimeFn::emission_order()
            .iter()
            .copied()
            .map(|runtime_fn| (runtime_fn.symbol(), runtime_fn))
            .collect()
    }

    fn runtime_stub_function(runtime_fn: RuntimeFn) -> WasmFunction {
        let sig = runtime_fn.stack_effect();
        let mut function = WasmFunction::new(runtime_fn.symbol());
        function.params = vec![WasmValType::I32; sig.params];
        function.results = vec![WasmValType::I32; sig.results];
        function.body = vec![WasmInstr::I32Const(0); sig.results];
        function
    }

    #[test]
    fn available_native_runtime_module_validates_with_wasmparser() {
        let mut functions = available_native_runtime_functions_with_helpers();
        let runtime_imports = RuntimeFn::emission_order()
            .iter()
            .copied()
            .filter(|runtime_fn| native_runtime_function_available(*runtime_fn))
            .flat_map(|runtime_fn| runtime_fn.spec().imports.iter().copied())
            .collect::<BTreeSet<_>>();
        let imported_symbols = runtime_imports
            .iter()
            .map(|import| import.spec().wat_symbol.to_owned())
            .collect::<BTreeSet<_>>();
        let mut declared_symbols = functions
            .iter()
            .map(|function| function.symbol.clone())
            .collect::<BTreeSet<_>>();
        declared_symbols.extend(imported_symbols);
        let runtime_symbols = runtime_function_symbols();

        for call in collect_function_calls(&functions) {
            if declared_symbols.contains(&call) {
                continue;
            }
            let runtime_fn = *runtime_symbols
                .get(call.as_str())
                .unwrap_or_else(|| panic!("native runtime test module has unresolved call {call}"));
            let stub = runtime_stub_function(runtime_fn);
            declared_symbols.insert(stub.symbol.clone());
            functions.push(stub);
        }

        let mut module = WasmModule::new().memory(WasmMemory::new(1, 256));
        for import in runtime_imports {
            let spec = import.spec();
            module = module.import(wasm_import_from_host_spec(&spec));
        }

        for global in collect_function_globals(&functions) {
            module = module.global(WasmGlobal::i32_mut(global, 0));
        }

        for function in functions {
            module = module.function(function);
        }

        let wasm = emit_wasm_module_binary(&module)
            .expect("available native runtime module should encode");
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("available native runtime module should validate");
    }

    #[test]
    fn native_runtime_call_to_embedded_helper_encodes_without_unresolved_symbols() {
        let span = Span::generated("native-runtime-call");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::TruthyBool,
                    args: vec![LoweredExpr::Undefined(span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("native runtime helper call should emit");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn array_is_array_runtime_call_to_embedded_helper_encodes_without_unresolved_symbols() {
        let span = Span::generated("native-array-is-array-runtime-call");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ArrayIsArray,
                    args: vec![LoweredExpr::Local(LocalId(0), span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("native Array.isArray helper call should emit");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native Array.isArray helper call should validate");
    }

    #[test]
    fn array_push_runtime_call_to_embedded_helper_encodes_without_unresolved_symbols() {
        let span = Span::generated("native-array-push-runtime-call");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ArrayPush,
                    args: vec![
                        LoweredExpr::Local(LocalId(0), span),
                        LoweredExpr::Number(1, span),
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("native ArrayPush helper call should emit");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native ArrayPush helper call should validate");
    }

    #[test]
    fn array_push_grow_runtime_call_to_embedded_helper_encodes_without_unresolved_symbols() {
        let span = Span::generated("native-array-push-grow-runtime-call");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ArrayPushGrow,
                    args: vec![
                        LoweredExpr::Local(LocalId(0), span),
                        LoweredExpr::Number(1, span),
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("native ArrayPushGrow helper call should emit");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native ArrayPushGrow helper call should validate");
    }

    #[test]
    fn array_iterator_runtime_calls_to_embedded_helpers_encode_without_unresolved_symbols() {
        let span = Span::generated("native-array-iterator-runtime-call");
        let mut top_level_statements = Vec::new();
        for intrinsic in [
            RuntimeFn::ArrayValues,
            RuntimeFn::ArrayKeys,
            RuntimeFn::ArrayEntries,
            RuntimeFn::ArrayIteratorNext,
        ] {
            top_level_statements.push(LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: vec![LoweredExpr::Local(LocalId(0), span)],
                    span,
                },
                span,
            ));
        }
        let program = LoweredProgram {
            top_level_statements,
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("native array iterator helper calls should emit");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native array iterator helper calls should validate");
    }

    #[test]
    fn strict_equal_runtime_call_embeds_transitive_native_helpers() {
        let span = Span::generated("native-strict-equal-runtime-call");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::StrictEqual,
                    args: vec![LoweredExpr::Number(1, span), LoweredExpr::Number(1, span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("native StrictEqual helper call should emit");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native StrictEqual helper call should validate");
    }

    #[test]
    fn core_abi_runtime_calls_to_embedded_helpers_validate() {
        let span = Span::generated("native-core-abi-runtime-call");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::AllocHeap,
                        args: vec![LoweredExpr::Number(8, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Copy,
                        args: vec![
                            LoweredExpr::Number(0, span),
                            LoweredExpr::Number(16, span),
                            LoweredExpr::Number(0, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Write,
                        args: vec![LoweredExpr::Number(0, span), LoweredExpr::Number(0, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::NumberToI32,
                        args: vec![LoweredExpr::Number(ValueTag::NUMBER, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::NumberFromI32,
                        args: vec![LoweredExpr::Number(ValueTag::NUMBER_PAYLOAD_MAX + 1, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::BitwiseToI32,
                        args: vec![LoweredExpr::Number(ValueTag::encode_number(1), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::BitwiseAnd,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(1), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::BitwiseXor,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(1), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::BitwiseOr,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(1), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Negate,
                        args: vec![LoweredExpr::Number(ValueTag::encode_number(7), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Sub,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(7), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::SubFast,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(7), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Mul,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(7), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::MulFast,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(7), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Div,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(7), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::DivFast,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(7), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Mod,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(7), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ModFast,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(7), span),
                            LoweredExpr::Number(ValueTag::encode_number(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::NumberIsNaN,
                        args: vec![LoweredExpr::Number(ValueTag::encode_number(7), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::NumberIsFinite,
                        args: vec![LoweredExpr::Number(ValueTag::encode_number(7), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::NumberIsInteger,
                        args: vec![LoweredExpr::Number(ValueTag::encode_number(7), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::NumberIsSafeInteger,
                        args: vec![LoweredExpr::Number(ValueTag::encode_number(7), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IsNaN,
                        args: vec![LoweredExpr::Undefined(span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IsFinite,
                        args: vec![LoweredExpr::Undefined(span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::GlobalParseInt,
                        args: vec![
                            LoweredExpr::String("123".into(), span),
                            LoweredExpr::Number(ValueTag::encode_number(10), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::GlobalParseFloat,
                        args: vec![LoweredExpr::String("123".into(), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::NumberCoerce,
                        args: vec![LoweredExpr::String("123".into(), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::BooleanCoerce,
                        args: vec![LoweredExpr::Undefined(span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::BooleanToString,
                        args: vec![LoweredExpr::Bool(true, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::TypeOf,
                        args: vec![LoweredExpr::Undefined(span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ValueOf,
                        args: vec![LoweredExpr::Number(ValueTag::encode_number(7), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ValueToStringInto,
                        args: vec![
                            LoweredExpr::Number(ValueTag::encode_number(123), span),
                            LoweredExpr::Number(64, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ErrorMessage,
                        args: vec![LoweredExpr::Undefined(span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Log,
                        args: vec![LoweredExpr::Undefined(span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::LogWarn,
                        args: vec![LoweredExpr::Undefined(span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::LogError,
                        args: vec![LoweredExpr::Undefined(span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::MemEqual,
                        args: vec![
                            LoweredExpr::Number(0, span),
                            LoweredExpr::Number(16, span),
                            LoweredExpr::Number(0, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ArrayGet,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::Number(ValueTag::NUMBER, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ArrayIndexPresent,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::Number(ValueTag::NUMBER, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ObjectHasOwnProperty,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::String("x".into(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ObjectHasOwn,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::String("x".into(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PropertyHas,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::Number(0, span),
                            LoweredExpr::Number(0, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PropertyGet,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::Number(0, span),
                            LoweredExpr::Number(0, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Index,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::Number(ValueTag::NUMBER, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PropertySet,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::Number(0, span),
                            LoweredExpr::Number(0, span),
                            LoweredExpr::Undefined(span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PropertyDelete,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::Number(0, span),
                            LoweredExpr::Number(0, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ReflectDeleteProperty,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::String("x".into(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ReflectGet,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::String("x".into(), span),
                            LoweredExpr::Undefined(span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ReflectHas,
                        args: vec![
                            LoweredExpr::Undefined(span),
                            LoweredExpr::String("x".into(), span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let symbols = embed_native_runtime_functions(&program)
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();

        assert!(symbols.contains(&RuntimeFn::AllocHeap.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Copy.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Write.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::NumberFromI32.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::NumberToI32.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::BitwiseToI32.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::BitwiseAnd.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::BitwiseXor.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::BitwiseOr.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Negate.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Sub.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::SubFast.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Mul.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::MulFast.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Div.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::DivFast.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Mod.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ModFast.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::NumberIsNaN.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::NumberIsFinite.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::NumberIsInteger.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::NumberIsSafeInteger.symbol().to_owned()));
        assert!(symbols.contains(&"$number_to_string".to_owned()));
        assert!(symbols.contains(&"$parse_int_string".to_owned()));
        assert!(symbols.contains(&RuntimeFn::GlobalParseInt.symbol().to_owned()));
        assert!(symbols.contains(&"$parse_float_string".to_owned()));
        assert!(symbols.contains(&RuntimeFn::GlobalParseFloat.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::NumberCoerce.symbol().to_owned()));
        assert!(symbols.contains(&"$is_nan_string".to_owned()));
        assert!(symbols.contains(&RuntimeFn::IsNaN.symbol().to_owned()));
        assert!(symbols.contains(&"$is_finite_string".to_owned()));
        assert!(symbols.contains(&RuntimeFn::IsFinite.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::BooleanCoerce.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::BooleanToString.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::TypeOf.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ValueOf.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ValueToStringInto.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ErrorMessage.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Log.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::LogWarn.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::LogError.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::MemEqual.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ArrayGet.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ArrayIndexPresent.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ObjectHasOwnProperty.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ObjectHasOwn.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PropertyHas.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PropertyGet.symbol().to_owned()));
        assert!(symbols.contains(&"$utf8_cp_to_byte_index".to_owned()));
        assert!(symbols.contains(&"$utf8_cp_byte_length".to_owned()));
        assert!(symbols.contains(&RuntimeFn::Index.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PropertySet.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PropertyDelete.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ReflectDeleteProperty.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ReflectGet.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ReflectHas.symbol().to_owned()));

        let (validated, _) = Validated::new(program).expect("program should validate");
        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("native Core ABI helper calls should emit");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native Core ABI helper calls should validate");
    }

    #[test]
    fn unary_typeof_fallback_consumes_inner_value() {
        let span = Span::generated("native-typeof-stack");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Unary {
                    op: LoweredUnaryOp::TypeOf,
                    expr: Box::new(LoweredExpr::Local(LocalId(0), span)),
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("typeof fallback should emit valid native wasm");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("typeof fallback must not leave its operand on the stack");
    }

    #[test]
    fn typeof_direct_closure_compares_as_function_string() {
        let span = Span::generated("native-typeof-closure");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrowFn {
                        func_id: FuncId(0),
                        captures: vec![],
                        representation:
                            ts2wasm_ir::lowered::ClosureRepresentation::DirectLocalToken,
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Unary {
                            op: LoweredUnaryOp::TypeOf,
                            expr: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            span,
                        }),
                        op: ts2wasm_ir::lowered::LoweredBinaryOp::StrictEqual,
                        right: Box::new(LoweredExpr::String("function".to_owned(), span)),
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("typeof direct closure should emit valid native wasm");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("typeof direct closure comparison should validate");
    }

    #[test]
    fn return_of_void_user_call_supplies_undefined_value() {
        let span = Span::generated("native-return-void-call");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::User(FuncId(0)),
                    args: vec![],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![
                LoweredFunction {
                    id: FuncId(0),
                    params: vec![],
                    uses_receiver: false,
                    min_required_params: 0,
                    rest_param_index: None,
                    metadata_length: None,
                    metadata_name: None,
                    locals: vec![],
                    body: vec![LoweredStmt::Return(
                        LoweredExpr::Call {
                            kind: FunctionCallKind::User(FuncId(1)),
                            args: vec![],
                            span,
                        },
                        span,
                    )],
                    recursion_depth: 0,
                    is_async: false,
                    is_generator: false,
                    generator_state: None,
                },
                LoweredFunction {
                    id: FuncId(1),
                    params: vec![],
                    uses_receiver: false,
                    min_required_params: 0,
                    rest_param_index: None,
                    metadata_length: None,
                    metadata_name: None,
                    locals: vec![],
                    body: vec![LoweredStmt::Expr(LoweredExpr::Undefined(span), span)],
                    recursion_depth: 0,
                    is_async: false,
                    is_generator: false,
                    generator_state: None,
                },
            ],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("returning a void user call should emit valid native wasm");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("returning a void user call must supply an i32 result");
    }

    #[test]
    fn non_throwing_try_finally_is_emitted_by_native_emitter() {
        let span = Span::generated("native-non-throwing-try-finally");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::TryFinally {
                try_body: vec![LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::Number(42, span),
                    span,
                )],
                finally_body: vec![LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(0), span)],
                        span,
                    },
                    span,
                )],
                span,
            }],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("non-throwing try-finally should emit in native emitter");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn non_throwing_try_catch_skips_catch_in_native_emitter() {
        let span = Span::generated("native-non-throwing-try-catch");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::TryCatch {
                try_body: vec![LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::String("try".to_owned(), span)],
                        span,
                    },
                    span,
                )],
                catch_var: Some(LocalId(0)),
                catch_body: Some(vec![LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::HeapClosureCall,
                        args: vec![LoweredExpr::Local(LocalId(0), span)],
                        span,
                    },
                    span,
                )]),
                finally_body: None,
                span,
            }],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("non-throwing try-catch should skip catch in native emitter");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn direct_throw_is_caught_by_native_try_catch() {
        let span = Span::generated("native-catchable-try-catch");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::TryCatch {
                try_body: vec![LoweredStmt::Throw(LoweredExpr::Null(span), span)],
                catch_var: Some(LocalId(0)),
                catch_body: Some(vec![LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::String("catch".to_owned(), span)],
                        span,
                    },
                    span,
                )]),
                finally_body: None,
                span,
            }],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("direct throw should be caught by native try-catch");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn heap_closure_throw_is_caught_by_native_try_catch() {
        let span = Span::generated("native-heap-closure-try-catch");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrowFn {
                        func_id: FuncId(0),
                        captures: vec![],
                        representation:
                            ts2wasm_ir::lowered::ClosureRepresentation::DirectLocalToken,
                        span,
                    },
                    span,
                ),
                LoweredStmt::TryCatch {
                    try_body: vec![LoweredStmt::Expr(
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::HeapClosureCall,
                            args: vec![LoweredExpr::Local(LocalId(0), span)],
                            span,
                        },
                        span,
                    )],
                    catch_var: Some(LocalId(1)),
                    catch_body: Some(vec![LoweredStmt::Assign(
                        LocalId(2),
                        LoweredExpr::Bool(true, span),
                        span,
                    )]),
                    finally_body: None,
                    span,
                },
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(2), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1), LocalId(2)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Throw(
                    LoweredExpr::String("boom".to_owned(), span),
                    span,
                )],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("heap closure throw should be caught by native try-catch");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn catch_body_heap_closure_dispatch_includes_target_functions() {
        let span = Span::generated("native-catch-body-heap-closure");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrowFn {
                        func_id: FuncId(0),
                        captures: vec![],
                        representation:
                            ts2wasm_ir::lowered::ClosureRepresentation::DirectLocalToken,
                        span,
                    },
                    span,
                ),
                LoweredStmt::TryCatch {
                    try_body: vec![LoweredStmt::Throw(
                        LoweredExpr::String("boom".to_owned(), span),
                        span,
                    )],
                    catch_var: Some(LocalId(1)),
                    catch_body: Some(vec![LoweredStmt::Expr(
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::HeapClosureCall,
                            args: vec![LoweredExpr::Local(LocalId(0), span)],
                            span,
                        },
                        span,
                    )]),
                    finally_body: None,
                    span,
                },
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Return(LoweredExpr::Number(7, span), span)],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("catch-body heap closure dispatch should include target functions");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("catch-body heap closure dispatch should validate");
    }

    #[test]
    fn block_stmt_heap_closure_dispatch_includes_target_functions() {
        let span = Span::generated("native-block-heap-closure");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrowFn {
                        func_id: FuncId(0),
                        captures: vec![],
                        representation:
                            ts2wasm_ir::lowered::ClosureRepresentation::DirectLocalToken,
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Block {
                        stmts: vec![LoweredStmt::Expr(
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::HeapClosureCall,
                                args: vec![LoweredExpr::Local(LocalId(0), span)],
                                span,
                            },
                            span,
                        )],
                        result: Box::new(LoweredExpr::Undefined(span)),
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Return(LoweredExpr::Number(7, span), span)],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("block heap closure dispatch should include target functions");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("block heap closure dispatch should validate");
    }

    #[test]
    fn switch_expr_user_call_includes_target_function() {
        let span = Span::generated("native-switch-user-call");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Switch {
                expr: LoweredExpr::Call {
                    kind: FunctionCallKind::User(FuncId(0)),
                    args: vec![],
                    span,
                },
                cases: vec![(
                    Some(LoweredExpr::Number(1, span)),
                    vec![LoweredStmt::Expr(LoweredExpr::Undefined(span), span)],
                )],
                span,
            }],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Return(LoweredExpr::Number(1, span), span)],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("switch expr user call should include target function");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("switch expr user call should validate");
    }

    #[test]
    fn non_static_user_call_stmt_includes_target_function() {
        let span = Span::generated("native-non-static-user-call");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrayNew {
                        elements: vec![LoweredExpr::Number(1, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::User(FuncId(0)),
                        args: vec![LoweredExpr::GetLength(
                            Box::new(LoweredExpr::Local(LocalId(0), span)),
                            span,
                        )],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0)],
                uses_receiver: false,
                min_required_params: 1,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Expr(LoweredExpr::Undefined(span), span)],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("non-static user call statement should include target function");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("non-static user call statement should validate");
    }

    #[test]
    fn symbol_well_known_runtime_call_embeds_native_helper() {
        let span = Span::generated("native-symbol-well-known");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::SymbolWellKnown,
                    args: vec![
                        LoweredExpr::Number(0, span),
                        LoweredExpr::String("Symbol.iterator".to_owned(), span),
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::SymbolWellKnown.symbol().to_owned()));

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("well-known symbol should emit native helper");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("well-known symbol native helper should validate");
    }

    #[test]
    fn math_host_runtime_call_embeds_native_helper_and_import() {
        let span = Span::generated("native-math-host");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::MathSin,
                    args: vec![LoweredExpr::Number(0, span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::MathSin.symbol().to_owned()));

        let module =
            crate::emit_wasm_module_native(&validated).expect("math host helper should emit");
        assert!(
            module
                .imports
                .iter()
                .any(|import| import.func_symbol == "$host_math_sin")
        );

        let wasm = emit_wasm_module_binary(&module).expect("math host helper should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("math host native helper should validate");
    }

    #[test]
    fn promise_and_dollar_runtime_calls_embed_native_helpers() {
        let span = Span::generated("native-promise-dollar");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseResolve,
                        args: vec![LoweredExpr::Number(1, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseWithResolvers,
                        args: vec![],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::AggregateError,
                        args: vec![
                            LoweredExpr::Number(1, span),
                            LoweredExpr::String("boom".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseAny,
                        args: vec![LoweredExpr::ArrayNew {
                            elements: vec![],
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseAll,
                        args: vec![LoweredExpr::ArrayNew {
                            elements: vec![],
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseFinally,
                        args: vec![LoweredExpr::Number(1, span), LoweredExpr::Number(2, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseThen,
                        args: vec![
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::PromiseResolve,
                                args: vec![LoweredExpr::Number(1, span)],
                                span,
                            },
                            LoweredExpr::Number(2, span),
                            LoweredExpr::Undefined(span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseCatch,
                        args: vec![
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::PromiseReject,
                                args: vec![LoweredExpr::Number(1, span)],
                                span,
                            },
                            LoweredExpr::Number(2, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseRace,
                        args: vec![LoweredExpr::ArrayNew {
                            elements: vec![],
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseAllSettled,
                        args: vec![LoweredExpr::ArrayNew {
                            elements: vec![],
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Dollar262Global,
                        args: vec![],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::PromiseResolve.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseWithResolvers.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseConstructor.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::AggregateError.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseAny.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseAll.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseAllSettled.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseFinally.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseThen.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseCatch.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseRace.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::PromiseReject.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ObjectPrototype.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Dollar262Global.symbol().to_owned()));

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("promise/dollar runtime helpers should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("promise/dollar native helpers should validate");
    }

    #[test]
    fn promise_then_catch_native_helpers_embed_user_function_dispatch() {
        let span = Span::generated("native-promise-callback-dispatch");
        let callback_local = LocalId(0);
        let callback_arg = LocalId(0);
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    callback_local,
                    LoweredExpr::ArrowFn {
                        func_id: FuncId(0),
                        captures: vec![],
                        representation:
                            ts2wasm_ir::lowered::ClosureRepresentation::DirectLocalToken,
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseThen,
                        args: vec![
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::PromiseResolve,
                                args: vec![LoweredExpr::Number(1, span)],
                                span,
                            },
                            LoweredExpr::Local(callback_local, span),
                            LoweredExpr::Undefined(span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PromiseCatch,
                        args: vec![
                            LoweredExpr::RuntimeCall {
                                intrinsic: RuntimeFn::PromiseReject,
                                args: vec![LoweredExpr::Number(2, span)],
                                span,
                            },
                            LoweredExpr::Local(callback_local, span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![callback_local],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![callback_arg],
                uses_receiver: false,
                min_required_params: 1,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Return(
                    LoweredExpr::Local(callback_arg, span),
                    span,
                )],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let runtime_functions = embed_native_runtime_functions(validated.program());
        let user_function_symbol = format!("${}", function_symbol(FuncId(0)));
        for runtime_fn in [RuntimeFn::PromiseThen, RuntimeFn::PromiseCatch] {
            let helper = runtime_functions
                .iter()
                .find(|function| function.symbol == runtime_fn.symbol())
                .unwrap_or_else(|| panic!("{runtime_fn:?} helper should be embedded"));
            assert!(
                helper.body.iter().any(
                    |instr| matches!(instr, WasmInstr::Call(symbol) if symbol == &user_function_symbol)
                ),
                "{runtime_fn:?} should dispatch direct callback tokens to user functions"
            );
        }

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("promise callback dispatch native helpers should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("promise callback dispatch native helpers should validate");
    }

    #[test]
    fn spread_via_iterator_runtime_call_embeds_native_passthrough() {
        let span = Span::generated("native-spread-via-iterator");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::SpreadViaIterator,
                    args: vec![LoweredExpr::Number(1, span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::SpreadViaIterator.symbol().to_owned()));

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("spread-via-iterator native helper should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("spread-via-iterator native helper should validate");
    }

    #[test]
    fn rest_object_runtime_call_embeds_native_helper() {
        let span = Span::generated("native-rest-object");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::RestObject,
                    args: vec![
                        LoweredExpr::ObjectNew {
                            props: vec![
                                ("keep".to_owned(), LoweredExpr::Number(1, span)),
                                ("drop".to_owned(), LoweredExpr::Number(2, span)),
                            ],
                            non_enumerable: 0,
                            span,
                        },
                        LoweredExpr::ArrayNew {
                            elements: vec![LoweredExpr::String("drop".to_owned(), span)],
                            span,
                        },
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::RestObject.symbol().to_owned()));

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("rest_object native helper should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("rest_object native helper should validate");
    }

    #[test]
    fn regexp_runtime_calls_embed_native_helpers() {
        let span = Span::generated("native-regexp");
        let pattern = LoweredExpr::String("/lit/".to_owned(), span);
        let input = LoweredExpr::String("literal".to_owned(), span);
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::RegExpTest,
                        args: vec![pattern.clone(), input.clone()],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::RegExpMatch,
                        args: vec![pattern.clone(), input.clone()],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::RegExpSearch,
                        args: vec![pattern.clone(), input.clone()],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringMatch,
                        args: vec![input.clone(), pattern.clone()],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringSearch,
                        args: vec![input, pattern],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        for runtime_fn in [
            RuntimeFn::RegExpTest,
            RuntimeFn::RegExpMatch,
            RuntimeFn::RegExpSearch,
            RuntimeFn::StringMatch,
            RuntimeFn::StringSearch,
        ] {
            assert!(symbols.contains(&runtime_fn.symbol().to_owned()));
        }

        let wasm =
            crate::emit_wasm_binary_native(&validated).expect("regexp native helpers should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("regexp native helpers should validate");
    }

    #[test]
    fn string_replace_runtime_call_embeds_native_helper() {
        let span = Span::generated("native-string-replace");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringReplace,
                        args: vec![
                            LoweredExpr::String("hello world".to_owned(), span),
                            LoweredExpr::String("world".to_owned(), span),
                            LoweredExpr::String("native".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringReplaceAll,
                        args: vec![
                            LoweredExpr::String("a-b-c".to_owned(), span),
                            LoweredExpr::String("-".to_owned(), span),
                            LoweredExpr::String("_".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::StringReplace.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::StringReplaceAll.symbol().to_owned()));

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("string replace native helpers should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("string replace native helpers should validate");
    }

    #[test]
    fn string_match_all_runtime_call_embeds_native_helper() {
        let span = Span::generated("native-string-match-all");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::StringMatchAll,
                    args: vec![
                        LoweredExpr::String("bananas".to_owned(), span),
                        LoweredExpr::String("a".to_owned(), span),
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::StringMatchAll.symbol().to_owned()));

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("string_match_all native helper should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("string_match_all native helper should validate");
    }

    #[test]
    fn task_drop_runtime_call_embeds_native_effect_only_helper() {
        let span = Span::generated("native-task-drop");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::TaskDrop,
                    args: vec![LoweredExpr::Number(64, span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::TaskDrop.symbol().to_owned()));

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("task_drop native helper should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("task_drop native helper should validate");
    }

    #[test]
    fn generator_return_runtime_call_embeds_native_helper() {
        let span = Span::generated("native-generator-return");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::GeneratorReturn,
                    args: vec![LoweredExpr::Number(7, span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::GeneratorReturn.symbol().to_owned()));

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("generator return runtime helper should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("generator return native helper should validate");
    }

    #[test]
    fn generator_yield_next_runtime_calls_embed_native_helpers() {
        let span = Span::generated("native-generator-yield-next");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::GeneratorYield,
                        args: vec![LoweredExpr::ArrayNew {
                            elements: vec![LoweredExpr::Number(1, span)],
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::GeneratorNext,
                        args: vec![LoweredExpr::Local(LocalId(0), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::GeneratorYield.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::GeneratorNext.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ArrayGet.symbol().to_owned()));

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("generator yield/next runtime helpers should emit");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("generator yield/next native helpers should validate");
    }

    #[test]
    fn reflect_host_runtime_calls_embed_native_helpers_and_imports() {
        let span = Span::generated("native-reflect-host");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ReflectApply,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(1), span),
                            LoweredExpr::Local(LocalId(2), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ReflectConstruct,
                        args: vec![
                            LoweredExpr::Local(LocalId(3), span),
                            LoweredExpr::Local(LocalId(4), span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1), LocalId(2), LocalId(3), LocalId(4)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::ReflectApply.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ReflectConstruct.symbol().to_owned()));

        let module =
            crate::emit_wasm_module_native(&validated).expect("reflect host helpers should emit");
        assert!(
            module
                .imports
                .iter()
                .any(|import| import.func_symbol == "$host_reflect_apply")
        );
        assert!(
            module
                .imports
                .iter()
                .any(|import| import.func_symbol == "$host_reflect_construct")
        );

        let wasm = emit_wasm_module_binary(&module).expect("reflect host helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("reflect host native helpers should validate");
    }

    #[test]
    fn iterator_host_runtime_calls_embed_native_helpers_and_imports() {
        let span = Span::generated("native-iterator-host");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::GetIterator,
                        args: vec![LoweredExpr::Local(LocalId(0), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorNext,
                        args: vec![LoweredExpr::Local(LocalId(1), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorFrom,
                        args: vec![LoweredExpr::Local(LocalId(2), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorMap,
                        args: vec![
                            LoweredExpr::Local(LocalId(3), span),
                            LoweredExpr::Local(LocalId(4), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorFilter,
                        args: vec![
                            LoweredExpr::Local(LocalId(5), span),
                            LoweredExpr::Local(LocalId(6), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorTake,
                        args: vec![
                            LoweredExpr::Local(LocalId(7), span),
                            LoweredExpr::Local(LocalId(8), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorDrop,
                        args: vec![
                            LoweredExpr::Local(LocalId(9), span),
                            LoweredExpr::Local(LocalId(10), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorToArray,
                        args: vec![LoweredExpr::Local(LocalId(11), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorReduce,
                        args: vec![
                            LoweredExpr::Local(LocalId(12), span),
                            LoweredExpr::Local(LocalId(13), span),
                            LoweredExpr::Local(LocalId(14), span),
                            LoweredExpr::Local(LocalId(15), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorForEach,
                        args: vec![
                            LoweredExpr::Local(LocalId(16), span),
                            LoweredExpr::Local(LocalId(17), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorSome,
                        args: vec![
                            LoweredExpr::Local(LocalId(18), span),
                            LoweredExpr::Local(LocalId(19), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorEvery,
                        args: vec![
                            LoweredExpr::Local(LocalId(20), span),
                            LoweredExpr::Local(LocalId(21), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::IteratorFind,
                        args: vec![
                            LoweredExpr::Local(LocalId(22), span),
                            LoweredExpr::Local(LocalId(23), span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: (0..24).map(LocalId).collect(),
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::GetIterator.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::IteratorNext.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::IteratorFrom.symbol().to_owned()));
        for runtime_fn in [
            RuntimeFn::IteratorMap,
            RuntimeFn::IteratorFilter,
            RuntimeFn::IteratorTake,
            RuntimeFn::IteratorDrop,
            RuntimeFn::IteratorToArray,
            RuntimeFn::IteratorReduce,
            RuntimeFn::IteratorForEach,
            RuntimeFn::IteratorSome,
            RuntimeFn::IteratorEvery,
            RuntimeFn::IteratorFind,
        ] {
            assert!(symbols.contains(&runtime_fn.symbol().to_owned()));
        }

        let module =
            crate::emit_wasm_module_native(&validated).expect("iterator host helpers should emit");
        assert!(
            module
                .imports
                .iter()
                .any(|import| import.func_symbol == "$host_get_iterator")
        );
        assert!(
            module
                .imports
                .iter()
                .any(|import| import.func_symbol == "$host_iterator_next")
        );
        for import_symbol in [
            "$host_iterator_map",
            "$host_iterator_filter",
            "$host_iterator_take",
            "$host_iterator_drop",
            "$host_iterator_to_array",
            "$host_iterator_reduce",
            "$host_iterator_for_each",
            "$host_iterator_some",
            "$host_iterator_every",
            "$host_iterator_find",
        ] {
            assert!(
                module
                    .imports
                    .iter()
                    .any(|import| import.func_symbol == import_symbol),
                "expected import {import_symbol}"
            );
        }

        let wasm = emit_wasm_module_binary(&module).expect("iterator host helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("iterator host native helpers should validate");
    }

    #[test]
    fn dynamic_host_runtime_calls_embed_native_helpers_and_imports() {
        let span = Span::generated("native-dynamic-host");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::EvalDirectHost,
                        args: vec![
                            LoweredExpr::String("1 + 1".to_owned(), span),
                            LoweredExpr::Local(LocalId(0), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::FunctionCompileHost,
                        args: vec![LoweredExpr::Local(LocalId(1), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::FunctionCallHost,
                        args: vec![
                            LoweredExpr::Local(LocalId(2), span),
                            LoweredExpr::Local(LocalId(3), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::FunctionCallMethodHost,
                        args: vec![
                            LoweredExpr::Local(LocalId(4), span),
                            LoweredExpr::Local(LocalId(5), span),
                            LoweredExpr::Local(LocalId(6), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::FunctionConstructHost,
                        args: vec![
                            LoweredExpr::Local(LocalId(7), span),
                            LoweredExpr::Local(LocalId(8), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Dollar262Eval,
                        args: vec![LoweredExpr::String("0".to_owned(), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: (0..=8).map(LocalId).collect(),
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        for runtime_fn in [
            RuntimeFn::EvalDirectHost,
            RuntimeFn::FunctionCompileHost,
            RuntimeFn::FunctionCallHost,
            RuntimeFn::FunctionCallMethodHost,
            RuntimeFn::FunctionConstructHost,
            RuntimeFn::Dollar262Eval,
            RuntimeFn::EvalIndirectHost,
        ] {
            assert!(symbols.contains(&runtime_fn.symbol().to_owned()));
        }

        let module =
            crate::emit_wasm_module_native(&validated).expect("dynamic host helpers should emit");
        for import_symbol in [
            "$host_eval_direct",
            "$host_eval_indirect",
            "$host_function_compile",
            "$host_function_call",
            "$host_function_call_method",
            "$host_function_construct",
        ] {
            assert!(
                module
                    .imports
                    .iter()
                    .any(|import| import.func_symbol == import_symbol),
                "expected import {import_symbol}"
            );
        }

        let wasm = emit_wasm_module_binary(&module).expect("dynamic host helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("dynamic host native helpers should validate");
    }

    #[test]
    fn json_runtime_calls_embed_native_helpers_and_imports() {
        let span = Span::generated("native-json-host");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::JsonStringify,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Undefined(span),
                            LoweredExpr::Undefined(span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::JsonParse,
                        args: vec![
                            LoweredExpr::Local(LocalId(1), span),
                            LoweredExpr::Undefined(span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::JsonStringify.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::JsonParse.symbol().to_owned()));

        let module = crate::emit_wasm_module_native(&validated).expect("json helpers should emit");
        for import_symbol in ["$host_json_stringify", "$host_json_parse"] {
            assert!(
                module
                    .imports
                    .iter()
                    .any(|import| import.func_symbol == import_symbol),
                "expected import {import_symbol}"
            );
        }

        let wasm = emit_wasm_module_binary(&module).expect("json helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("json native helpers should validate");
    }

    #[test]
    fn node_path_process_crypto_runtime_calls_embed_native_helpers_and_imports() {
        let span = Span::generated("native-node-path-process-crypto");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PathJoin,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(1), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PathResolve,
                        args: vec![LoweredExpr::Local(LocalId(2), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PathBasename,
                        args: vec![LoweredExpr::Local(LocalId(3), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PathDirname,
                        args: vec![LoweredExpr::Local(LocalId(4), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::CryptoRandomBytes,
                        args: vec![LoweredExpr::Local(LocalId(5), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ProcessExit,
                        args: vec![LoweredExpr::Local(LocalId(6), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::FsAppendFileSync,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(1), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::FsReadFileSync,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(1), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::FsWriteFileSync,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(1), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ProcessArgv,
                        args: vec![],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ProcessEnv,
                        args: vec![],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: (0..=6).map(LocalId).collect(),
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        for runtime_fn in [
            RuntimeFn::PathJoin,
            RuntimeFn::PathResolve,
            RuntimeFn::PathBasename,
            RuntimeFn::PathDirname,
            RuntimeFn::CryptoRandomBytes,
            RuntimeFn::ProcessExit,
            RuntimeFn::FsReadFileSync,
            RuntimeFn::FsWriteFileSync,
            RuntimeFn::FsAppendFileSync,
            RuntimeFn::ProcessArgv,
            RuntimeFn::ProcessEnv,
        ] {
            assert!(symbols.contains(&runtime_fn.symbol().to_owned()));
        }

        let module =
            crate::emit_wasm_module_native(&validated).expect("node host helpers should emit");
        for import_symbol in [
            "$host_path_join",
            "$host_path_resolve",
            "$host_path_basename",
            "$host_path_dirname",
            "$host_crypto_random_bytes",
            "$host_process_exit",
            "$path_open",
            "$fd_read",
            "$fd_write",
            "$fd_close",
            "$host_fs_append_file_sync",
            "$args_sizes_get",
            "$args_get",
            "$environ_sizes_get",
            "$environ_get",
        ] {
            assert!(
                module
                    .imports
                    .iter()
                    .any(|import| import.func_symbol == import_symbol),
                "expected import {import_symbol}"
            );
        }

        let wasm = emit_wasm_module_binary(&module).expect("node host helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("node host native helpers should validate");
    }

    #[test]
    fn uri_encode_runtime_calls_embed_native_helpers() {
        let span = Span::generated("native-uri-encode");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::EncodeURI,
                        args: vec![LoweredExpr::String(
                            "https://example.com/a b".to_owned(),
                            span,
                        )],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::EncodeURIComponent,
                        args: vec![LoweredExpr::String("a b?x=1".to_owned(), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::EncodeURI.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::EncodeURIComponent.symbol().to_owned()));

        let module = crate::emit_wasm_module_native(&validated)
            .expect("URI encode native helpers should emit");
        assert!(
            module
                .imports
                .iter()
                .all(|import| !import.func_symbol.starts_with("$host_"))
        );

        let wasm = emit_wasm_module_binary(&module).expect("URI encode helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("URI encode native helpers should validate");
    }

    #[test]
    fn encoding_decode_escape_runtime_calls_embed_native_helpers() {
        let span = Span::generated("native-encoding-decode-escape");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::DecodeURI,
                        args: vec![LoweredExpr::String(
                            "https://example.com/a%20b?x=1".to_owned(),
                            span,
                        )],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::DecodeURIComponent,
                        args: vec![LoweredExpr::String("a%20b%3Fx%3D1".to_owned(), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Escape,
                        args: vec![LoweredExpr::String("snow ☃".to_owned(), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Unescape,
                        args: vec![LoweredExpr::String("%u2603%20x".to_owned(), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        for runtime_fn in [
            RuntimeFn::DecodeURI,
            RuntimeFn::DecodeURIComponent,
            RuntimeFn::Escape,
            RuntimeFn::Unescape,
        ] {
            assert!(symbols.contains(&runtime_fn.symbol().to_owned()));
        }

        let module = crate::emit_wasm_module_native(&validated)
            .expect("encoding decode/escape native helpers should emit");
        assert!(
            module
                .imports
                .iter()
                .all(|import| !import.func_symbol.starts_with("$host_"))
        );

        let wasm =
            emit_wasm_module_binary(&module).expect("encoding decode/escape helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("encoding decode/escape native helpers should validate");
    }

    #[test]
    fn descriptor_define_runtime_calls_embed_native_helpers() {
        let span = Span::generated("native-descriptor-define");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ObjectDefineProperty,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::String("x".to_owned(), span),
                            LoweredExpr::Local(LocalId(1), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ObjectDefineProperties,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(2), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ReflectDefineProperty,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::String("y".to_owned(), span),
                            LoweredExpr::Local(LocalId(3), span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: (0..=3).map(LocalId).collect(),
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        for runtime_fn in [
            RuntimeFn::ObjectDefineProperty,
            RuntimeFn::ObjectDefineProperties,
            RuntimeFn::ReflectDefineProperty,
        ] {
            assert!(symbols.contains(&runtime_fn.symbol().to_owned()));
        }

        let module =
            crate::emit_wasm_module_native(&validated).expect("descriptor helpers should emit");
        assert!(
            module
                .imports
                .iter()
                .all(|import| !import.func_symbol.starts_with("$host_"))
        );

        let wasm = emit_wasm_module_binary(&module).expect("descriptor helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("descriptor native helpers should validate");
    }

    #[test]
    fn descriptor_introspection_runtime_calls_embed_native_helpers() {
        let span = Span::generated("native-descriptor-introspection");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ObjectGetOwnPropertyDescriptor,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::String("x".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ObjectGetOwnPropertyDescriptors,
                        args: vec![LoweredExpr::Local(LocalId(0), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        for runtime_fn in [
            RuntimeFn::ObjectGetOwnPropertyDescriptor,
            RuntimeFn::ObjectGetOwnPropertyDescriptors,
        ] {
            assert!(symbols.contains(&runtime_fn.symbol().to_owned()));
        }

        let module =
            crate::emit_wasm_module_native(&validated).expect("descriptor helpers should emit");
        assert!(
            module
                .imports
                .iter()
                .all(|import| !import.func_symbol.starts_with("$host_"))
        );

        let wasm = emit_wasm_module_binary(&module).expect("descriptor helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("descriptor introspection native helpers should validate");
    }

    #[test]
    fn three_arg_reflect_construct_embeds_native_helpers_and_imports() {
        let span = Span::generated("native-reflect-construct-inline");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ReflectConstruct,
                    args: vec![
                        LoweredExpr::Local(LocalId(0), span),
                        LoweredExpr::ArrayNew {
                            elements: vec![],
                            span,
                        },
                        LoweredExpr::Local(LocalId(1), span),
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        assert!(
            ordered_required_native_runtime_functions(validated.program())
                .contains(&RuntimeFn::ReflectConstruct)
        );
        let module = crate::emit_wasm_module_native(&validated)
            .expect("three-arg Reflect.construct should emit");
        assert!(
            module
                .imports
                .iter()
                .any(|import| import.func_symbol == "$host_reflect_construct")
        );
        assert!(
            module
                .functions
                .iter()
                .any(|function| function.symbol == RuntimeFn::ReflectConstruct.symbol())
        );
    }

    #[test]
    fn symbol_description_runtime_call_embeds_native_helper() {
        let span = Span::generated("native-symbol-description");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::SymbolDescription,
                    args: vec![LoweredExpr::Local(LocalId(0), span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let symbols = embed_native_runtime_functions(&program)
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::SymbolDescription.symbol().to_owned()));

        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("symbol description should emit through native helper");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native SymbolDescription helper call should validate");
    }

    #[test]
    fn symbol_to_string_runtime_call_embeds_native_helper_chain() {
        let span = Span::generated("native-symbol-to-string");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::SymbolToString,
                    args: vec![LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::SymbolNew,
                        args: vec![LoweredExpr::String("desc".to_owned(), span)],
                        span,
                    }],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let symbols = embed_native_runtime_functions(&program)
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::SymbolToString.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ValueToStringInto.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::AllocHeap.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Copy.symbol().to_owned()));

        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("Symbol.toString should emit natively");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native SymbolToString helper chain should validate");
    }

    #[test]
    fn symbol_for_runtime_call_embeds_native_helper_chain() {
        let span = Span::generated("native-symbol-for");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::SymbolFor,
                    args: vec![LoweredExpr::String("shared".to_owned(), span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let symbols = embed_native_runtime_functions(&program)
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::SymbolFor.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::AllocHeap.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::StringEqual.symbol().to_owned()));

        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm =
            crate::emit_wasm_binary_native(&validated).expect("Symbol.for should emit natively");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native SymbolFor helper chain should validate");
    }

    #[test]
    fn instanceof_runtime_call_has_native_fallback() {
        let span = Span::generated("native-instanceof-fallback");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::InstanceOf,
                    args: vec![
                        LoweredExpr::Undefined(span),
                        LoweredExpr::BuiltinErrorPrototype(
                            BuiltinErrorConstructor::TypeError,
                            span,
                        ),
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("instanceof should emit as native fallback");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn non_static_array_push_many_has_native_fallback() {
        let span = Span::generated("native-array-push-many-fallback");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(LocalId(0), LoweredExpr::Undefined(span), span),
                LoweredStmt::Let(
                    LocalId(1),
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ArrayPushMany,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Number(1, span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("non-static ArrayPushMany fallback should emit");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn heap_object_closure_expression_is_opaque_in_native_emitter() {
        let span = Span::generated("native-heap-object-closure");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::ArrowFn {
                    func_id: FuncId(0),
                    captures: vec![],
                    representation: ts2wasm_ir::lowered::ClosureRepresentation::HeapObject,
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Return(LoweredExpr::Undefined(span), span)],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("heap-object closure expression should emit as opaque token");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn error_message_runtime_call_passes_through_native_emitter() {
        let span = Span::generated("native-error-message");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ErrorMessage,
                    args: vec![LoweredExpr::String("boom".to_owned(), span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("ErrorMessage should pass through in native emitter");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn concat_runtime_call_embeds_native_helper() {
        let span = Span::generated("native-concat-helper");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::Concat,
                    args: vec![
                        LoweredExpr::String("a".to_owned(), span),
                        LoweredExpr::Local(LocalId(0), span),
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("Concat helper should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native Concat helper call should validate");
    }

    #[test]
    fn string_single_code_point_runtime_calls_embed_native_helper_chain() {
        let span = Span::generated("native-string-single-code-point-helper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringCharAt,
                        args: vec![
                            LoweredExpr::String("abc".to_owned(), span),
                            LoweredExpr::Number(1, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringAt,
                        args: vec![
                            LoweredExpr::String("abc".to_owned(), span),
                            LoweredExpr::Number(-1, span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("String charAt/at helpers should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native String charAt/at helper chain should validate");
    }

    #[test]
    fn string_range_runtime_calls_embed_native_helper_chain() {
        let span = Span::generated("native-string-range-helper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringSubstring,
                        args: vec![
                            LoweredExpr::String("abcdef".to_owned(), span),
                            LoweredExpr::Number(1, span),
                            LoweredExpr::Number(4, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringSubstr,
                        args: vec![
                            LoweredExpr::String("abcdef".to_owned(), span),
                            LoweredExpr::Number(2, span),
                            LoweredExpr::Number(3, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringSlice,
                        args: vec![
                            LoweredExpr::String("abcdef".to_owned(), span),
                            LoweredExpr::Number(-4, span),
                            LoweredExpr::Number(-1, span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("String range helpers should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native String range helper chain should validate");
    }

    #[test]
    fn string_search_runtime_calls_embed_native_helper_chain() {
        let span = Span::generated("native-string-search-helper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringIndexOf,
                        args: vec![
                            LoweredExpr::String("abcabc".to_owned(), span),
                            LoweredExpr::String("bc".to_owned(), span),
                            LoweredExpr::Number(0, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringLastIndexOf,
                        args: vec![
                            LoweredExpr::String("abcabc".to_owned(), span),
                            LoweredExpr::String("bc".to_owned(), span),
                            LoweredExpr::Number(6, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringLocaleCompare,
                        args: vec![
                            LoweredExpr::String("abc".to_owned(), span),
                            LoweredExpr::String("abd".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringIncludes,
                        args: vec![
                            LoweredExpr::String("abcabc".to_owned(), span),
                            LoweredExpr::String("ca".to_owned(), span),
                            LoweredExpr::Number(0, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringStartsWith,
                        args: vec![
                            LoweredExpr::String("abcabc".to_owned(), span),
                            LoweredExpr::String("ab".to_owned(), span),
                            LoweredExpr::Number(0, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringEndsWith,
                        args: vec![
                            LoweredExpr::String("abcabc".to_owned(), span),
                            LoweredExpr::String("bc".to_owned(), span),
                            LoweredExpr::Number(6, span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("String search helpers should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native String search helper chain should validate");
    }

    #[test]
    fn string_construct_runtime_calls_embed_native_helper_chain() {
        let span = Span::generated("native-string-construct-helper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringPadStart,
                        args: vec![
                            LoweredExpr::String("x".to_owned(), span),
                            LoweredExpr::Number(3, span),
                            LoweredExpr::String("0".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringPadEnd,
                        args: vec![
                            LoweredExpr::String("x".to_owned(), span),
                            LoweredExpr::Number(3, span),
                            LoweredExpr::String("0".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringRepeat,
                        args: vec![
                            LoweredExpr::String("ab".to_owned(), span),
                            LoweredExpr::Number(2, span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("String construct helpers should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native String construct helper chain should validate");
    }

    #[test]
    fn string_split_runtime_calls_embed_native_helper_chain() {
        let span = Span::generated("native-string-split-helper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringSplit,
                        args: vec![
                            LoweredExpr::String("a,b,c".to_owned(), span),
                            LoweredExpr::String(",".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ArrayMapStringSplit,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::String(" ".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(symbols.contains(&RuntimeFn::StringSplit.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::ArrayMapStringSplit.symbol().to_owned()));

        let module = crate::emit_wasm_module_native(&validated)
            .expect("String split helpers should emit through native runtime embedding");
        assert!(
            module
                .imports
                .iter()
                .all(|import| !import.func_symbol.starts_with("$host_"))
        );

        let wasm = emit_wasm_module_binary(&module).expect("String split helpers should encode");
        assert!(wasm.starts_with(b"\0asm"));
        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native String split helper chain should validate");
    }

    #[test]
    fn string_case_runtime_calls_embed_native_helper_chain() {
        let span = Span::generated("native-string-case-helper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringToUpperCase,
                        args: vec![LoweredExpr::String("abc".to_owned(), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringToLowerCase,
                        args: vec![LoweredExpr::String("XYZ".to_owned(), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("String case helpers should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native String case helper chain should validate");
    }

    #[test]
    fn string_trim_runtime_calls_embed_native_helper_chain() {
        let span = Span::generated("native-string-trim-helper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringTrim,
                        args: vec![LoweredExpr::String(" abc ".to_owned(), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringTrimStart,
                        args: vec![LoweredExpr::String(" abc ".to_owned(), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::StringTrimEnd,
                        args: vec![LoweredExpr::String(" abc ".to_owned(), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("String trim helpers should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native String trim helper chain should validate");
    }

    #[test]
    fn string_raw_runtime_call_embeds_native_helper_chain() {
        let span = Span::generated("native-string-raw-helper");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::StringRaw,
                    args: vec![
                        LoweredExpr::String("template".to_owned(), span),
                        LoweredExpr::String("sub0".to_owned(), span),
                        LoweredExpr::String("sub1".to_owned(), span),
                    ],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("String.raw helper should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native String.raw helper chain should validate");
    }

    #[test]
    fn add_runtime_call_embeds_native_helper_chain() {
        let span = Span::generated("native-add-helper");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::Add,
                    args: vec![LoweredExpr::Number(1, span), LoweredExpr::Number(2, span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("Add helper should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native Add helper call should validate");
    }

    #[test]
    fn loose_equality_runtime_calls_embed_native_helper_chain() {
        let span = Span::generated("native-loose-equality-helper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::EqualEqual,
                        args: vec![
                            LoweredExpr::String("1".to_owned(), span),
                            LoweredExpr::Bool(true, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::BangEqual,
                        args: vec![LoweredExpr::Undefined(span), LoweredExpr::Null(span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");
        let symbols = embed_native_runtime_functions(validated.program())
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();
        assert!(
            symbols.contains(
                &RuntimeFn::BigIntStringComparisonBoundaryError
                    .symbol()
                    .to_owned()
            )
        );

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("loose equality helpers should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native loose equality helper calls should validate");
    }

    #[test]
    fn relational_runtime_calls_embed_native_helper_chain() {
        let span = Span::generated("native-relational-helper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Less,
                        args: vec![LoweredExpr::Number(1, span), LoweredExpr::Number(2, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::LessEqualFast,
                        args: vec![LoweredExpr::Number(2, span), LoweredExpr::Number(2, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::Greater,
                        args: vec![LoweredExpr::Number(3, span), LoweredExpr::Number(2, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::GreaterEqualFast,
                        args: vec![LoweredExpr::Number(3, span), LoweredExpr::Number(3, span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let symbols = embed_native_runtime_functions(&program)
            .into_iter()
            .map(|function| function.symbol)
            .collect::<Vec<_>>();

        assert!(symbols.contains(&"$bigint_compare_small_int".to_owned()));
        assert!(symbols.contains(&RuntimeFn::Less.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::LessEqualFast.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::Greater.symbol().to_owned()));
        assert!(symbols.contains(&RuntimeFn::GreaterEqualFast.symbol().to_owned()));

        let (validated, _) = Validated::new(program).expect("program should validate");
        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("relational helpers should emit through native runtime embedding");

        wasmparser::Validator::new()
            .validate_all(&wasm)
            .expect("native relational helper calls should validate");
    }

    #[test]
    fn static_array_push_many_is_folded_by_native_emitter() {
        let span = Span::generated("native-array-push-many");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrayNew {
                        elements: vec![LoweredExpr::Number(1, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ArrayPushMany,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Number(2, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Index {
                            object: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            index: Box::new(LoweredExpr::Number(1, span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("static ArrayPushMany should fold in native emitter");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn static_array_push_many_user_wrapper_is_folded_by_native_emitter() {
        let span = Span::generated("native-array-push-many-wrapper");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrayNew {
                        elements: vec![LoweredExpr::Number(1, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::User(FuncId(0)),
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Number(2, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Index {
                            object: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            index: Box::new(LoweredExpr::Number(1, span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0), LocalId(1)],
                uses_receiver: false,
                min_required_params: 2,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::ArrayPushMany,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(1), span),
                        ],
                        span,
                    },
                    span,
                )],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("static ArrayPushMany wrapper should fold in native emitter");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn static_array_push_many_bound_heap_closure_is_folded_by_native_emitter() {
        let span = Span::generated("native-array-push-many-bound-closure");
        let bind_push = LoweredExpr::Block {
            stmts: vec![LoweredStmt::Let(
                LocalId(2),
                LoweredExpr::PropertyGet {
                    obj: Box::new(LoweredExpr::Undefined(span)),
                    key: "call".to_owned(),
                    span,
                },
                span,
            )],
            result: Box::new(LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::HeapClosureCall,
                args: vec![
                    LoweredExpr::PropertyGet {
                        obj: Box::new(LoweredExpr::Local(LocalId(2), span)),
                        key: "bind".to_owned(),
                        span,
                    },
                    LoweredExpr::Local(LocalId(2), span),
                    LoweredExpr::Number(0, span),
                ],
                span,
            }),
            span,
        };
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(LocalId(0), bind_push, span),
                LoweredStmt::Let(
                    LocalId(1),
                    LoweredExpr::ArrayNew {
                        elements: vec![],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::HeapClosureCall,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(1), span),
                            LoweredExpr::String("x".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Index {
                            object: Box::new(LoweredExpr::Local(LocalId(1), span)),
                            index: Box::new(LoweredExpr::Number(0, span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1), LocalId(2)],
            functions: vec![],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("static bound ArrayPushMany closure should fold in native emitter");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn static_object_has_own_property_wrapper_is_folded_by_native_emitter() {
        let span = Span::generated("native-object-has-own-wrapper");
        let has_own_method = LoweredExpr::Block {
            stmts: vec![LoweredStmt::Let(
                LocalId(2),
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::ObjectPrototype,
                    args: vec![],
                    span,
                },
                span,
            )],
            result: Box::new(LoweredExpr::PropertyGet {
                obj: Box::new(LoweredExpr::Local(LocalId(2), span)),
                key: "hasOwnProperty".to_owned(),
                span,
            }),
            span,
        };
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ObjectNew {
                        props: vec![("a".to_owned(), LoweredExpr::Number(1, span))],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                LoweredStmt::If {
                    condition: LoweredExpr::Call {
                        kind: FunctionCallKind::User(FuncId(0)),
                        args: vec![
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::String("a".to_owned(), span),
                        ],
                        span,
                    },
                    then_body: vec![LoweredStmt::Expr(
                        LoweredExpr::Call {
                            kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                            args: vec![LoweredExpr::String("ok".to_owned(), span)],
                            span,
                        },
                        span,
                    )],
                    else_body: vec![],
                    span,
                },
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0), LocalId(1)],
                uses_receiver: false,
                min_required_params: 2,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![LocalId(2)],
                body: vec![LoweredStmt::Return(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::HeapClosureCall,
                        args: vec![
                            has_own_method,
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(1), span),
                        ],
                        span,
                    },
                    span,
                )],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("static hasOwnProperty wrapper should fold in native emitter");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn static_array_join_wrapper_is_folded_by_native_emitter() {
        let span = Span::generated("native-array-join-wrapper");
        let join_method = LoweredExpr::Block {
            stmts: vec![LoweredStmt::Let(
                LocalId(2),
                LoweredExpr::Block {
                    stmts: vec![],
                    result: Box::new(LoweredExpr::PropertyGet {
                        obj: Box::new(LoweredExpr::Undefined(span)),
                        key: "prototype".to_owned(),
                        span,
                    }),
                    span,
                },
                span,
            )],
            result: Box::new(LoweredExpr::PropertyGet {
                obj: Box::new(LoweredExpr::Local(LocalId(2), span)),
                key: "join".to_owned(),
                span,
            }),
            span,
        };
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrayNew {
                        elements: vec![
                            LoweredExpr::String("a".to_owned(), span),
                            LoweredExpr::String("b".to_owned(), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Call {
                            kind: FunctionCallKind::User(FuncId(0)),
                            args: vec![
                                LoweredExpr::Local(LocalId(0), span),
                                LoweredExpr::String("-".to_owned(), span),
                            ],
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0), LocalId(1)],
                uses_receiver: false,
                min_required_params: 2,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![LocalId(2)],
                body: vec![LoweredStmt::Return(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::HeapClosureCall,
                        args: vec![
                            join_method,
                            LoweredExpr::Local(LocalId(0), span),
                            LoweredExpr::Local(LocalId(1), span),
                        ],
                        span,
                    },
                    span,
                )],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("static Array.prototype.join.call wrapper should fold in native emitter");

        assert!(wasm.starts_with(b"\0asm"));
    }

    #[test]
    fn static_array_index_equal_callback_is_inlined_by_native_emitter() {
        let span = Span::generated("native-array-index-callback");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrayNew {
                        elements: vec![LoweredExpr::Number(7, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Let(
                    LocalId(1),
                    LoweredExpr::Index {
                        object: Box::new(LoweredExpr::Local(LocalId(0), span)),
                        index: Box::new(LoweredExpr::Number(0, span)),
                        span,
                    },
                    span,
                ),
                LoweredStmt::Let(
                    LocalId(2),
                    LoweredExpr::Call {
                        kind: FunctionCallKind::User(FuncId(0)),
                        args: vec![
                            LoweredExpr::Local(LocalId(1), span),
                            LoweredExpr::Number(0, span),
                            LoweredExpr::Local(LocalId(0), span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::If {
                    condition: LoweredExpr::Local(LocalId(2), span),
                    then_body: vec![LoweredStmt::Expr(
                        LoweredExpr::Call {
                            kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                            args: vec![LoweredExpr::String("ok".to_owned(), span)],
                            span,
                        },
                        span,
                    )],
                    else_body: vec![],
                    span,
                },
            ],
            top_level_locals: vec![LocalId(0), LocalId(1), LocalId(2)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0), LocalId(1), LocalId(2)],
                uses_receiver: false,
                min_required_params: 3,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![LocalId(3)],
                body: vec![LoweredStmt::If {
                    condition: LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Block {
                            stmts: vec![LoweredStmt::Let(
                                LocalId(3),
                                LoweredExpr::Local(LocalId(2), span),
                                span,
                            )],
                            result: Box::new(LoweredExpr::PropertyGetDynamic {
                                obj: Box::new(LoweredExpr::Local(LocalId(3), span)),
                                key: Box::new(LoweredExpr::Local(LocalId(1), span)),
                                span,
                            }),
                            span,
                        }),
                        op: ts2wasm_ir::lowered::LoweredBinaryOp::StrictEqual,
                        right: Box::new(LoweredExpr::Local(LocalId(0), span)),
                        span,
                    },
                    then_body: vec![LoweredStmt::Return(LoweredExpr::Bool(true, span), span)],
                    else_body: vec![],
                    span,
                }],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };
        let (validated, _) = Validated::new(program).expect("program should validate");

        let wasm = crate::emit_wasm_binary_native(&validated)
            .expect("static array index equality callback should inline in native emitter");

        assert!(wasm.starts_with(b"\0asm"));
    }
}
