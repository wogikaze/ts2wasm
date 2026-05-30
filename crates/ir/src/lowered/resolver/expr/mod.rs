use super::super::*;
mod assignment;
mod binary;
mod binding;
mod control;
mod dynamic_code;
mod eval;
mod eval_function;
pub(crate) mod facts;
mod literal;
mod property;
mod ternary;
mod unary;

use crate::builtin_resolved::ResolvedExpr;
use std::collections::HashMap;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::Resolver {
    pub(crate) fn lower_expr(&mut self, expr: &ResolvedExpr) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            // Literals (trivial constructors)
            ResolvedExpr::Number(value) => Ok(LoweredExpr::Number(*value, Span::generated("num"))),
            ResolvedExpr::DecimalNumber(value) => Ok(LoweredExpr::DecimalNumber(
                value.clone(),
                Span::generated("num"),
            )),
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
            ResolvedExpr::Yield { expr, delegate } => {
                if *delegate {
                    return Err(Diagnostic::unsupported_at(Span::generated("yield_star"), "yield* delegation is not supported in expression context (only as statement or let initializer)".to_owned()));
                }
                expr.as_ref()
                    .map(|expr| self.lower_expr(expr))
                    .transpose()
                    .map(|expr| {
                        expr.unwrap_or_else(|| LoweredExpr::Undefined(Span::generated("yield")))
                    })
            }
            ResolvedExpr::This { .. } => self.lower_this_expr(),
            ResolvedExpr::NewTarget { span } => self.lower_new_target_expr(*span),
            ResolvedExpr::ImportMeta { span } => self.lower_import_meta_expr(*span),
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
            ResolvedExpr::Call { callee, args, span } => self.lower_call_expr(callee, args, *span),
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
            ResolvedExpr::FunctionConstructor { .. } | ResolvedExpr::Eval { .. } => {
                self.lower_dynamic_code_expr(expr)
            }
            ResolvedExpr::ModuleLoad {
                specifier,
                is_dynamic_import,
            } => Ok(LoweredExpr::ModuleLoad {
                module_id: self.module_id_for_specifier(specifier),
                // ModuleLoadDynamic: dynamic import() keeps async module-load identity in lowered IR.
                kind: if *is_dynamic_import {
                    ModuleLoadKind::DynamicImport
                } else {
                    ModuleLoadKind::StaticRequire
                },
                span: Span::generated("module_load"),
            }),
            ResolvedExpr::ArrowFn {
                params,
                body,
                body_stmts,
                source_text,
            } => self.lower_arrow_fn_with_source_text(params, body, body_stmts, source_text),
            ResolvedExpr::FunctionExpr {
                name,
                params,
                body,
                is_generator,
                origin,
                constructor_metadata,
                source_text,
                ..
            } => self.lower_named_function_expr(
                name,
                params,
                body,
                *is_generator,
                *origin,
                constructor_metadata.as_ref(),
                source_text,
            ),
            ResolvedExpr::ClassExpr { .. } => Ok(LoweredExpr::Undefined(Span::generated("undef"))),
            ResolvedExpr::Sequence(exprs) => self.lower_sequence_expr(exprs),
            ResolvedExpr::EvalCompletion(plan) => self.lower_eval_completion_expr(plan),
        }
    }

    pub(super) fn lower_import_meta_expr(&mut self, span: Span) -> Result<LoweredExpr, Diagnostic> {
        Ok(LoweredExpr::ObjectNew {
            props: vec![("url".to_owned(), self.lower_module_meta_url(span))],
            non_enumerable: 0,
            span,
        })
    }

    pub(super) fn lower_module_meta_url(&self, span: Span) -> LoweredExpr {
        // ModuleMetaUrl: import.meta.url resolves to the active module URL/specifier.
        LoweredExpr::String(self.ctx.current_module_url.clone(), span)
    }

    fn lower_sequence_expr(&mut self, exprs: &[ResolvedExpr]) -> Result<LoweredExpr, Diagnostic> {
        let mut stmts = Vec::new();
        let len = exprs.len();
        for (i, expr) in exprs.iter().enumerate() {
            let lowered = self.lower_expr(expr)?;
            if i < len - 1 {
                stmts.push(LoweredStmt::Expr(lowered, Span::generated("seq")));
            } else {
                return Ok(LoweredExpr::Block {
                    stmts,
                    result: Box::new(lowered),
                    span: Span::generated("seq"),
                });
            }
        }
        // Single-element sequence (shouldn't happen but handle gracefully)
        unreachable!("sequence with zero elements")
    }

    fn lower_eval_completion_expr(
        &mut self,
        plan: &crate::builtin_resolved::EvalCompletionPlan,
    ) -> Result<LoweredExpr, Diagnostic> {
        if !plan.landing_state_is_consistent() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedEval,
                message: "eval completion landing steps do not match scope mode".to_owned(),
                span: Some(Span::generated("eval_completion_scope")),
                phase: None,
            });
        }
        let completion = self.alloc_temp();
        self.ctx.symbols.scopes.push(HashMap::new());
        let saved_class_constructor_ids = self.ctx.classes.class_constructor_ids.clone();
        let saved_class_method_ids = self.ctx.classes.class_method_ids.clone();
        let saved_class_static_method_ids = self.ctx.classes.class_static_method_ids.clone();
        let saved_class_parents = self.ctx.classes.class_parents.clone();
        let saved_class_private_fields = self.ctx.classes.class_private_fields.clone();
        let saved_class_static_private_fields =
            self.ctx.classes.class_static_private_fields.clone();
        let caller_scope_index = self.ctx.symbols.scopes.len().saturating_sub(2);
        let lowered = (|| {
            let mut stmts = vec![LoweredStmt::Let(
                completion,
                LoweredExpr::Undefined(Span::generated("eval_completion_init")),
                Span::generated("eval_completion_let"),
            )];
            self.lower_eval_declaration_plan_into(
                &plan.declarations,
                caller_scope_index,
                &mut stmts,
            )?;
            self.lower_eval_completion_steps_into(
                plan.steps(),
                completion,
                caller_scope_index,
                &mut stmts,
            )?;
            Ok(LoweredExpr::Block {
                stmts,
                result: Box::new(LoweredExpr::Local(
                    completion,
                    Span::generated("eval_completion_result"),
                )),
                span: Span::generated("eval_completion"),
            })
        })();
        self.ctx.classes.class_constructor_ids = saved_class_constructor_ids;
        self.ctx.classes.class_method_ids = saved_class_method_ids;
        self.ctx.classes.class_static_method_ids = saved_class_static_method_ids;
        self.ctx.classes.class_parents = saved_class_parents;
        self.ctx.classes.class_private_fields = saved_class_private_fields;
        self.ctx.classes.class_static_private_fields = saved_class_static_private_fields;
        self.ctx.symbols.scopes.pop();
        lowered
    }

    fn lower_eval_declaration_plan_into(
        &mut self,
        declarations: &crate::builtin_resolved::EvalDeclarationPlan,
        caller_scope_index: usize,
        stmts: &mut Vec<LoweredStmt>,
    ) -> Result<(), Diagnostic> {
        self.lower_eval_hoisted_vars_into(&declarations.var_names, caller_scope_index, stmts)?;
        self.lower_eval_hoisted_functions_into(
            &declarations.function_hoists,
            caller_scope_index,
            stmts,
        )
    }

    fn lower_eval_hoisted_vars_into(
        &mut self,
        names: &[String],
        caller_scope_index: usize,
        stmts: &mut Vec<LoweredStmt>,
    ) -> Result<(), Diagnostic> {
        for name in names {
            let (local, existed) =
                self.declare_eval_var_in_caller_scope(name, caller_scope_index)?;
            if self.ctx.facts.env_cell_names.contains(name) {
                self.ctx.facts.env_cell_locals.insert(local);
                self.ctx.facts.initialized_env_cell_locals.insert(local);
            }
            if !existed {
                let init = if self.ctx.facts.env_cell_locals.contains(&local) {
                    LoweredExpr::EnvCellNew(
                        Box::new(LoweredExpr::Undefined(Span::generated("eval_var_hoist"))),
                        Span::generated("eval_var_hoist_cell"),
                    )
                } else {
                    LoweredExpr::Undefined(Span::generated("eval_var_hoist"))
                };
                stmts.push(LoweredStmt::Let(
                    local,
                    init,
                    Span::generated("eval_var_hoist_let"),
                ));
            }
        }
        Ok(())
    }

    fn lower_eval_hoisted_functions_into(
        &mut self,
        functions: &[crate::builtin_resolved::EvalFunctionHoist],
        caller_scope_index: usize,
        stmts: &mut Vec<LoweredStmt>,
    ) -> Result<(), Diagnostic> {
        for function in functions {
            stmts.push(self.lower_eval_function_decl_in_caller_scope(
                &function.name,
                &function.params,
                &function.body,
                function.is_async,
                caller_scope_index,
            )?);
        }
        Ok(())
    }
}

fn focused_direct_eval_tdz_candidates(source: &str) -> Vec<String> {
    let trimmed = source.trim();
    if is_direct_eval_tdz_candidate_name(trimmed) {
        return vec![trimmed.to_owned()];
    }
    if let Some(inner) = trimmed
        .strip_prefix('(')
        .and_then(|rest| rest.strip_suffix(')'))
    {
        let inner = inner.trim();
        if is_direct_eval_tdz_candidate_name(inner) {
            return vec![inner.to_owned()];
        }
    }
    if let Some(inner) = trimmed.strip_prefix("typeof ") {
        let inner = inner.trim();
        if is_direct_eval_tdz_candidate_name(inner) {
            return vec![inner.to_owned()];
        }
    }
    if let Some((base, property)) = trimmed.split_once('.') {
        let base = base.trim();
        let property = property.trim();
        if is_direct_eval_tdz_candidate_name(base) && is_ascii_js_identifier(property) {
            return vec![base.to_owned()];
        }
    }
    if let Some((base, property)) = trimmed.split_once("?.") {
        let base = base.trim();
        let property = property.trim();
        if is_direct_eval_tdz_candidate_name(base) && is_ascii_js_identifier(property) {
            return vec![base.to_owned()];
        }
    }
    if let Some((base, rest)) = trimmed.split_once("?.[") {
        let base = base.trim();
        if is_direct_eval_tdz_candidate_name(base) && rest.trim_end().ends_with(']') {
            return vec![base.to_owned()];
        }
    }
    if let Some((base, rest)) = trimmed.split_once('[') {
        let base = base.trim();
        if is_direct_eval_tdz_candidate_name(base) && rest.trim_end().ends_with(']') {
            return vec![base.to_owned()];
        }
    }
    if let Some(inner) = trimmed
        .strip_prefix("`${")
        .and_then(|rest| rest.strip_suffix("}`"))
    {
        let inner = inner.trim();
        if is_direct_eval_tdz_candidate_name(inner) {
            return vec![inner.to_owned()];
        }
    }
    Vec::new()
}

fn is_direct_eval_tdz_candidate_name(value: &str) -> bool {
    is_ascii_js_identifier(value) && !is_known_eval_global_name(value)
}

fn is_known_eval_global_name(value: &str) -> bool {
    matches!(
        value,
        "globalThis"
            | "console"
            | "Array"
            | "BigInt"
            | "Boolean"
            | "Date"
            | "Error"
            | "Infinity"
            | "JSON"
            | "Map"
            | "Math"
            | "NaN"
            | "Number"
            | "Object"
            | "Promise"
            | "RangeError"
            | "ReferenceError"
            | "RegExp"
            | "Set"
            | "String"
            | "Symbol"
            | "SyntaxError"
            | "TypeError"
            | "undefined"
    )
}

fn is_ascii_js_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first == '$' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch == '$' || ch.is_ascii_alphanumeric())
}

fn is_eval_for_head_identifier_key(value: &str) -> bool {
    is_ascii_js_identifier(value)
}

struct DirectEvalEnvDescriptor {
    caller_is_strict: bool,
    bindings: Vec<DirectEvalEnvBinding>,
}

struct DirectEvalEnvBinding {
    name: String,
    kind: DirectEvalEnvBindingKind,
}

enum DirectEvalEnvBindingKind {
    ReadWrite { local: LocalId },
    Tdz,
}

impl DirectEvalEnvBindingKind {
    fn as_descriptor_tag(&self) -> &'static str {
        match self {
            Self::ReadWrite { .. } => "readwrite",
            Self::Tdz => "tdz",
        }
    }

    fn target_expr(&self) -> LoweredExpr {
        match self {
            Self::ReadWrite { local } => {
                LoweredExpr::Local(*local, Span::generated("eval_env_cell"))
            }
            Self::Tdz => LoweredExpr::Undefined(Span::generated("eval_env_tdz")),
        }
    }
}

const DIRECT_EVAL_DESCRIPTOR_VERSION_KEY: &str = "__ts2wasm_eval_descriptor_v2";
const DIRECT_EVAL_DESCRIPTOR_CALLER_STRICT_KEY: &str = "__ts2wasm_eval_caller_strict";
const DIRECT_EVAL_DESCRIPTOR_BINDINGS_KEY: &str = "__ts2wasm_eval_bindings";

impl DirectEvalEnvDescriptor {
    fn into_lowered_expr(self) -> LoweredExpr {
        if self.bindings.is_empty() && !self.caller_is_strict {
            return LoweredExpr::Undefined(Span::generated("eval_env"));
        }

        let mut elements = Vec::with_capacity(6);
        elements.push(LoweredExpr::String(
            DIRECT_EVAL_DESCRIPTOR_VERSION_KEY.to_owned(),
            Span::generated("eval_env_version"),
        ));
        elements.push(LoweredExpr::Bool(true, Span::generated("eval_env_version")));
        elements.push(LoweredExpr::String(
            DIRECT_EVAL_DESCRIPTOR_CALLER_STRICT_KEY.to_owned(),
            Span::generated("eval_env_strict"),
        ));
        elements.push(LoweredExpr::Bool(
            self.caller_is_strict,
            Span::generated("eval_env_strict"),
        ));
        elements.push(LoweredExpr::String(
            DIRECT_EVAL_DESCRIPTOR_BINDINGS_KEY.to_owned(),
            Span::generated("eval_env_bindings"),
        ));
        elements.push(LoweredExpr::ArrayNew {
            elements: self
                .bindings
                .into_iter()
                .map(DirectEvalEnvBinding::into_lowered_expr)
                .collect(),
            span: Span::generated("eval_env_bindings"),
        });
        LoweredExpr::ArrayNew {
            elements,
            span: Span::generated("eval_env"),
        }
    }
}

impl DirectEvalEnvBinding {
    fn into_lowered_expr(self) -> LoweredExpr {
        LoweredExpr::ArrayNew {
            elements: vec![
                LoweredExpr::String(self.name, Span::generated("eval_env_name")),
                self.kind.target_expr(),
                LoweredExpr::String(
                    self.kind.as_descriptor_tag().to_owned(),
                    Span::generated("eval_env_binding_kind"),
                ),
            ],
            span: Span::generated("eval_env_binding"),
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
            | "encodeURIComponent"
            | "decodeURIComponent"
            | "structuredClone"
            | "queueMicrotask"
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
        "escape" | "unescape" | "isNaN" | "parseFloat" | "isFinite" | "encodeURI" | "decodeURI"
        | "encodeURIComponent" | "decodeURIComponent" => 1,
        "structuredClone" => 1,
        "queueMicrotask" => 1,
        _ => 0,
    }
}
