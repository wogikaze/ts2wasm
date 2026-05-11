mod assignment;
mod binary;
mod binding;
mod control;
mod facts;
mod literal;
mod property;
mod ternary;
mod unary;

use super::{
    is_array_prototype_push_property, is_private_field_storage_key, is_set_prototype_property,
    is_set_prototype_property_expr, private_storage_observable_access_diagnostic,
};
use crate::builtin::{BuiltinId, BuiltinPropertyId};
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_syntax::{BinaryOp, UnaryOp};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::Resolver {
    pub(crate) fn lower_expr(&mut self, expr: &ResolvedExpr) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            // Literals (trivial constructors)
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
                span: Span::generated("bigint"),
            }),
            ResolvedExpr::String(value) => {
                Ok(LoweredExpr::String(value.clone(), Span::generated("str")))
            }
            ResolvedExpr::Bool(value) => Ok(LoweredExpr::Bool(*value, Span::generated("bool"))),
            ResolvedExpr::Null => Ok(LoweredExpr::Null(Span::generated("null"))),
            ResolvedExpr::Undefined => Ok(LoweredExpr::Undefined(Span::generated("undef"))),

            // Control flow
            ResolvedExpr::Await { expr } => self.lower_await_expr(expr),
            ResolvedExpr::This { .. } => self.lower_this_expr(),
            ResolvedExpr::NewTarget { span } => self.lower_new_target_expr(*span),
            ResolvedExpr::Ident(name) => self.lower_ident_expr(name),
            ResolvedExpr::Spread(_) => self.lower_spread_expr(),

            // Unary / Binary / Ternary
            ResolvedExpr::Unary { op, expr } => self.lower_unary_expr(op, expr),
            ResolvedExpr::Binary { left, op, right } => self.lower_binary_expr(left, op, right),
            ResolvedExpr::Ternary {
                condition,
                then_expr,
                else_expr,
                ..
            } => self.lower_ternary_expr(condition, then_expr, else_expr),

            // Assignment expressions
            ResolvedExpr::Assign { name, expr } => self.lower_assign_expr(name, expr),
            ResolvedExpr::LogicalAssign { name, op, expr } => {
                self.lower_logical_assign_expr(name, op, expr)
            }
            ResolvedExpr::LogicalPropertyAssign {
                object,
                key,
                op,
                expr,
            } => self.lower_logical_property_assign_expr(object, key, op, expr),
            ResolvedExpr::LogicalComputedPropertyAssign {
                object,
                key,
                op,
                expr,
            } => self.lower_logical_computed_property_assign_expr(object, key, op, expr),
            ResolvedExpr::LogicalComputedMemberAssign {
                object,
                key,
                op,
                expr,
            } => self.lower_logical_computed_member_assign_expr(object, key, op, expr),
            ResolvedExpr::LogicalMemberAssign {
                object,
                key,
                op,
                expr,
            } => self.lower_logical_member_assign_expr(object, key, op, expr),
            ResolvedExpr::PropertyAssign {
                object,
                key,
                value,
                span,
            } => self.lower_property_assign_expr(object, key, value, *span),
            ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
                self.lower_property_assign_dynamic_expr(object, key, value)
            }

            // Property access
            ResolvedExpr::PropertyAccess { object, key, span } => {
                self.lower_property_access_expr(object, key, *span)
            }
            ResolvedExpr::OptionalPropertyAccess { object, key, .. } => {
                self.lower_optional_property_access_expr(object, key)
            }
            ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
                self.lower_optional_computed_index_expr(object, index)
            }
            ResolvedExpr::ComputedIndex { object, index } => {
                self.lower_computed_index_expr(object, index)
            }

            // Delegations to other modules
            ResolvedExpr::OptionalCall { callee, args, span } => {
                self.lower_optional_call(callee, args, *span)
            }
            ResolvedExpr::Call { callee, args, span } => {
                self.lower_call_expr(callee, args, *span)
            }
            ResolvedExpr::BuiltinCall { builtin, args } => {
                self.lower_builtin_call_expr(*builtin, args)
            }
            ResolvedExpr::BuiltinProperty {
                builtin,
                object,
                span,
            } => self.lower_builtin_property_expr(*builtin, object, *span),
            ResolvedExpr::Array(elements) => self.lower_array_literal(elements),
            ResolvedExpr::Object(props) => self.lower_object_literal_expr(props),
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                span,
            } => self.lower_method_call_expr(object, method, args, *span),
            ResolvedExpr::New {
                class_name,
                args,
                span,
            } => self.lower_new_expr(class_name, args, *span),
            ResolvedExpr::ModuleLoad { specifier } => Ok(LoweredExpr::ModuleLoad {
                module_id: self.module_id_for_specifier(specifier),
                span: Span::generated("module_load"),
            }),
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
                Ok(LoweredExpr::Undefined(Span::generated("undef")))
            }
        }
    }
}

/// Check if a name is a known global builtin function (for metadata queries).
pub(super) fn is_global_builtin_function_name(name: &str) -> bool {
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

pub(super) fn lower_global_builtin_function_metadata_property(
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
        "escape" | "unescape" | "isNaN" | "parseFloat" | "isFinite" | "encodeURI"
        | "decodeURI" => 1,
        _ => 0,
    }
}
