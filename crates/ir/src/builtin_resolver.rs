#[path = "builtin_resolver_bigint.rs"]
mod builtin_resolver_bigint;
#[path = "builtin_resolver_bigint_ops.rs"]
mod builtin_resolver_bigint_ops;
#[path = "builtin_resolver_class_features.rs"]
mod builtin_resolver_class_features;
#[path = "builtin_resolver_host.rs"]
mod builtin_resolver_host;
#[path = "builtin_resolver_outer.rs"]
mod builtin_resolver_outer;
pub(crate) use builtin_resolver_bigint::bigint_runtime_fn_name;
use builtin_resolver_bigint::*;
use builtin_resolver_bigint_ops::*;
pub(crate) use builtin_resolver_class_features::static_private_field_local_name;
use builtin_resolver_class_features::*;
use builtin_resolver_host::*;
use builtin_resolver_outer::*;
use std::collections::{HashMap, HashSet};

use ts2wasm_frontend::{
    BinaryOp, ClassPrivateElement, ClassStaticBlock, DiagCode, Diagnostic, Expr, Span, Stmt,
    UnaryOp,
};
use ts2wasm_runtime_abi::ValueTag;

use super::binding_pattern::parse_binding_pattern;
use super::builtin::BuiltinId;
use super::builtin::BuiltinPropertyId;
use super::builtin_resolved::{ClassMethod, ResolvedExpr, ResolvedParam, ResolvedStmt};

const BIGINT_FROM_VALUE_RUNTIME_CALL: &str = "__ts2wasm_bigint_from_value";
const BIGINT_AS_INT_N_RUNTIME_CALL: &str = "__ts2wasm_bigint_as_int_n";
const BIGINT_AS_UINT_N_RUNTIME_CALL: &str = "__ts2wasm_bigint_as_uint_n";
const BIGINT_RUNTIME_OBJECT: &str = "__ts2wasm_bigint_runtime";

pub fn resolve_builtins(program: &[Stmt]) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    let program = BigIntStaticBuiltinFolder::default().fold_stmts(program);
    BigIntRuntimeGuard::default().visit_stmts(&program)?;
    let outer_bindings = collect_top_level_bindings(&program)?;
    program
        .iter()
        .map(|stmt| resolve_stmt_with_outer_bindings(stmt, &outer_bindings))
        .collect()
}

#[derive(Default)]
struct BigIntStaticBuiltinFolder {
    locals: HashMap<String, Expr>,
    object_toprimitive_locals: HashMap<String, Expr>,
}

impl BigIntStaticBuiltinFolder {
    fn fold_stmts(&mut self, stmts: &[Stmt]) -> Vec<Stmt> {
        stmts.iter().map(|stmt| self.fold_stmt(stmt)).collect()
    }

    fn fold_stmt(&mut self, stmt: &Stmt) -> Stmt {
        match stmt {
            Stmt::Let { name, expr, span } => {
                let expr = self.fold_expr(expr);
                if let Some(value) = static_bigint_builtin_const_expr(&expr) {
                    self.locals.insert(name.clone(), value);
                } else {
                    self.locals.remove(name);
                }
                if let Some(value) = object_toprimitive_supported_primitive_expr(&expr) {
                    self.object_toprimitive_locals.insert(name.clone(), value);
                } else {
                    self.object_toprimitive_locals.remove(name);
                }
                Stmt::Let {
                    name: name.clone(),
                    expr,
                    span: *span,
                }
            }
            Stmt::Assign { name, expr, span } => {
                let expr = self.fold_expr(expr);
                if let Some(value) = static_bigint_builtin_const_expr(&expr) {
                    self.locals.insert(name.clone(), value);
                } else {
                    self.locals.remove(name);
                }
                if let Some(value) = object_toprimitive_supported_primitive_expr(&expr) {
                    self.object_toprimitive_locals.insert(name.clone(), value);
                } else {
                    self.object_toprimitive_locals.remove(name);
                }
                Stmt::Assign {
                    name: name.clone(),
                    expr,
                    span: *span,
                }
            }
            Stmt::Expr { expr, span } => Stmt::Expr {
                expr: self.fold_expr(expr),
                span: *span,
            },
            Stmt::If {
                condition,
                then_body,
                else_body,
                span,
            } => {
                let condition = self.fold_expr(condition);
                if matches!(condition, Expr::Bool { value: true, .. }) {
                    let then_body = self.fold_stmts(then_body);
                    let else_body = self.fork().fold_stmts(else_body);
                    return Stmt::If {
                        condition,
                        then_body,
                        else_body,
                        span: *span,
                    };
                }
                if matches!(condition, Expr::Bool { value: false, .. }) {
                    let then_body = self.fork().fold_stmts(then_body);
                    let else_body = self.fold_stmts(else_body);
                    return Stmt::If {
                        condition,
                        then_body,
                        else_body,
                        span: *span,
                    };
                }
                let mut then_folder = self.fork();
                let then_body = then_folder.fold_stmts(then_body);
                let mut else_folder = self.fork();
                let else_body = else_folder.fold_stmts(else_body);
                self.merge_if_branch_facts(
                    then_body.as_slice(),
                    &then_folder,
                    else_body.as_slice(),
                    &else_folder,
                );
                Stmt::If {
                    condition,
                    then_body,
                    else_body,
                    span: *span,
                }
            }
            Stmt::While {
                condition,
                body,
                span,
            } => {
                let condition = self.fold_expr(condition);
                let body = self.fork().fold_stmts(body);
                self.invalidate_assigned_in_stmts(body.as_slice());
                Stmt::While {
                    condition,
                    body,
                    span: *span,
                }
            }
            Stmt::Function {
                name,
                params,
                body,
                is_generator,
                span,
            } => Stmt::Function {
                name: name.clone(),
                params: params.clone(),
                body: BigIntStaticBuiltinFolder::default().fold_stmts(body),
                is_generator: *is_generator,
                span: *span,
            },
            Stmt::Return { expr, span } => Stmt::Return {
                expr: self.fold_expr(expr),
                span: *span,
            },
            Stmt::ClassDecl {
                name,
                extends,
                body,
                static_blocks,
                private_elements,
                span,
            } => Stmt::ClassDecl {
                name: name.clone(),
                extends: extends
                    .as_ref()
                    .map(|extends| Box::new(self.fold_expr(extends))),
                body: BigIntStaticBuiltinFolder::default().fold_stmts(body),
                static_blocks: static_blocks
                    .iter()
                    .map(|block| ClassStaticBlock {
                        body: BigIntStaticBuiltinFolder::default().fold_stmts(&block.body),
                        span: block.span,
                    })
                    .collect(),
                private_elements: private_elements.clone(),
                span: *span,
            },
            Stmt::TryCatch {
                try_block,
                catch_param,
                catch_block,
                finally_block,
                span,
            } => {
                let try_block = self.fork().fold_stmts(try_block);
                let catch_block = catch_block
                    .as_ref()
                    .map(|body| self.fork().fold_stmts(body));
                let finally_block = finally_block
                    .as_ref()
                    .map(|body| self.fork().fold_stmts(body));
                self.invalidate_assigned_in_stmts(try_block.as_slice());
                if let Some(catch_block) = &catch_block {
                    self.invalidate_assigned_in_stmts(catch_block.as_slice());
                }
                if let Some(finally_block) = &finally_block {
                    self.invalidate_assigned_in_stmts(finally_block.as_slice());
                }
                Stmt::TryCatch {
                    try_block,
                    catch_param: catch_param.clone(),
                    catch_block,
                    finally_block,
                    span: *span,
                }
            }
            Stmt::Throw { expr, span } => Stmt::Throw {
                expr: self.fold_expr(expr),
                span: *span,
            },
            Stmt::Switch { expr, cases, span } => {
                let expr = self.fold_expr(expr);
                let cases = cases
                    .iter()
                    .map(|(case_expr, body)| {
                        let case_expr = case_expr.as_ref().map(|expr| self.fold_expr(expr));
                        let body = self.fork().fold_stmts(body);
                        self.invalidate_assigned_in_stmts(body.as_slice());
                        (case_expr, body)
                    })
                    .collect();
                Stmt::Switch {
                    expr,
                    cases,
                    span: *span,
                }
            }
            Stmt::DoWhile {
                body,
                condition,
                span,
            } => {
                let body = self.fork().fold_stmts(body);
                let condition = self.fold_expr(condition);
                self.invalidate_assigned_in_stmts(body.as_slice());
                Stmt::DoWhile {
                    body,
                    condition,
                    span: *span,
                }
            }
            Stmt::For {
                init,
                condition,
                update,
                body,
                span,
            } => {
                let mut loop_folder = self.fork();
                let init = init
                    .as_ref()
                    .map(|init| Box::new(loop_folder.fold_stmt(init)));
                let condition = condition
                    .as_ref()
                    .map(|condition| loop_folder.fold_expr(condition));
                let update = update.as_ref().map(|update| loop_folder.fold_expr(update));
                let body = loop_folder.fold_stmts(body);
                if let Some(update) = &update {
                    self.invalidate_assigned_in_expr(update);
                }
                self.invalidate_assigned_in_stmts(body.as_slice());
                Stmt::For {
                    init,
                    condition,
                    update,
                    body,
                    span: *span,
                }
            }
            Stmt::ForIn {
                var,
                iter,
                body,
                span,
            } => {
                let iter = self.fold_expr(iter);
                let mut body_folder = self.fork();
                body_folder.locals.remove(var);
                body_folder.object_toprimitive_locals.remove(var);
                let body = body_folder.fold_stmts(body);
                self.locals.remove(var);
                self.object_toprimitive_locals.remove(var);
                self.invalidate_assigned_in_stmts(body.as_slice());
                Stmt::ForIn {
                    var: var.clone(),
                    iter,
                    body,
                    span: *span,
                }
            }
            Stmt::ForOf {
                var,
                iter,
                body,
                span,
            } => {
                let iter = self.fold_expr(iter);
                let mut body_folder = self.fork();
                body_folder.locals.remove(var);
                body_folder.object_toprimitive_locals.remove(var);
                let body = body_folder.fold_stmts(body);
                self.locals.remove(var);
                self.object_toprimitive_locals.remove(var);
                self.invalidate_assigned_in_stmts(body.as_slice());
                Stmt::ForOf {
                    var: var.clone(),
                    iter,
                    body,
                    span: *span,
                }
            }
            Stmt::Labeled { label, body, span } => Stmt::Labeled {
                label: label.clone(),
                body: Box::new(self.fold_stmt(body)),
                span: *span,
            },
            Stmt::ExportDefault {
                expr,
                default_span,
                span,
            } => Stmt::ExportDefault {
                expr: self.fold_expr(expr),
                default_span: *default_span,
                span: *span,
            },
            Stmt::ExportDecl {
                declaration,
                specifier,
                span,
            } => Stmt::ExportDecl {
                declaration: Box::new(self.fold_stmt(declaration)),
                specifier: specifier.clone(),
                span: *span,
            },
            Stmt::ImportSideEffect { .. }
            | Stmt::ImportNamed { .. }
            | Stmt::ImportDefault { .. }
            | Stmt::ImportDefaultNamed { .. }
            | Stmt::ImportNamespace { .. }
            | Stmt::ImportDefaultNamespace { .. }
            | Stmt::ExportNamed { .. }
            | Stmt::ExportNamedFrom { .. }
            | Stmt::ExportAllFrom { .. }
            | Stmt::ExportNamespaceFrom { .. }
            | Stmt::Break { .. }
            | Stmt::Continue { .. } => stmt.clone(),
        }
    }

    fn fold_expr(&mut self, expr: &Expr) -> Expr {
        match expr {
            Expr::Call { callee, args, span } if is_bigint_static_builtin_callee(callee) => {
                let callee = Box::new(self.fold_expr(callee));
                let args = args
                    .iter()
                    .map(|arg| {
                        if let Expr::Ident { name, .. } = arg {
                            self.locals
                                .get(name)
                                .cloned()
                                .unwrap_or_else(|| self.fold_expr(arg))
                        } else {
                            self.fold_expr(arg)
                        }
                    })
                    .collect();
                Expr::Call {
                    callee,
                    args,
                    span: *span,
                }
            }
            Expr::Call { callee, args, span } => Expr::Call {
                callee: Box::new(self.fold_expr(callee)),
                args: args.iter().map(|arg| self.fold_expr(arg)).collect(),
                span: *span,
            },
            Expr::OptionalCall { callee, args, span } => Expr::OptionalCall {
                callee: Box::new(self.fold_expr(callee)),
                args: args.iter().map(|arg| self.fold_expr(arg)).collect(),
                span: *span,
            },
            Expr::Unary { op, expr, span } => {
                let expr = self.fold_expr(expr);
                if *op == UnaryOp::Negate
                    && let Some(value) = self.bigint_const_value(&expr)
                {
                    return bigint_const_to_expr(value.negated(), *span);
                }
                if *op == UnaryOp::BitwiseNot
                    && let Some(value) = self.bigint_const_value(&expr)
                    && let Ok(result) = fold_bigint_unary_bitwise_not(value, *span)
                {
                    return bigint_const_to_expr(result, *span);
                }
                Expr::Unary {
                    op: *op,
                    expr: Box::new(expr),
                    span: *span,
                }
            }
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                let left = self.fold_expr(left);
                let right = self.fold_expr(right);
                if matches!(
                    op,
                    BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor
                ) && let (Some(left_value), Some(right_value)) = (
                    self.bigint_const_value(&left),
                    self.bigint_const_value(&right),
                ) && let Ok(result) = fold_bigint_binary(left_value, *op, right_value, *span)
                {
                    return bigint_const_to_expr(result, *span);
                }
                let left_toprimitive = self.object_toprimitive_primitive_expr(&left);
                let right_toprimitive = self.object_toprimitive_primitive_expr(&right);
                let is_relational = matches!(
                    op,
                    BinaryOp::Less
                        | BinaryOp::LessEqual
                        | BinaryOp::Greater
                        | BinaryOp::GreaterEqual
                );
                let should_fold = matches!(
                    op,
                    BinaryOp::EqualEqual
                        | BinaryOp::BangEqual
                        | BinaryOp::Less
                        | BinaryOp::LessEqual
                        | BinaryOp::Greater
                        | BinaryOp::GreaterEqual
                ) && (expr_contains_bigint(&left)
                    || expr_contains_bigint(&right)
                    || left_toprimitive.as_ref().is_some_and(expr_contains_bigint)
                    || right_toprimitive.as_ref().is_some_and(expr_contains_bigint));
                let left_can_fold_toprimitive = left_toprimitive
                    .as_ref()
                    .is_some_and(|expr| object_toprimitive_can_fold_for_bigint_op(expr, *op));
                let right_can_fold_toprimitive = right_toprimitive
                    .as_ref()
                    .is_some_and(|expr| object_toprimitive_can_fold_for_bigint_op(expr, *op));
                Expr::Binary {
                    left: Box::new(if should_fold && left_can_fold_toprimitive {
                        let expr = left_toprimitive.unwrap_or(left);
                        if is_relational {
                            object_toprimitive_relational_expr(expr)
                        } else {
                            expr
                        }
                    } else {
                        left
                    }),
                    op: *op,
                    right: Box::new(if should_fold && right_can_fold_toprimitive {
                        let expr = right_toprimitive.unwrap_or(right);
                        if is_relational {
                            object_toprimitive_relational_expr(expr)
                        } else {
                            expr
                        }
                    } else {
                        right
                    }),
                    span: *span,
                }
            }
            Expr::Member {
                object,
                property,
                span,
            } => Expr::Member {
                object: Box::new(self.fold_expr(object)),
                property: property.clone(),
                span: *span,
            },
            Expr::OptionalMember {
                object,
                property,
                span,
            } => Expr::OptionalMember {
                object: Box::new(self.fold_expr(object)),
                property: property.clone(),
                span: *span,
            },
            Expr::Assign { name, expr, span } => {
                let folded = self.fold_expr(expr);
                if let Some(value) = static_bigint_builtin_const_expr(&folded) {
                    self.locals.insert(name.clone(), value);
                } else {
                    self.locals.remove(name);
                }
                if let Some(value) = object_toprimitive_supported_primitive_expr(&folded) {
                    self.object_toprimitive_locals.insert(name.clone(), value);
                } else {
                    self.object_toprimitive_locals.remove(name);
                }
                Expr::Assign {
                    name: name.clone(),
                    expr: Box::new(folded),
                    span: *span,
                }
            }
            Expr::LogicalAssign {
                name,
                op,
                expr,
                span,
            } => {
                self.locals.remove(name);
                self.object_toprimitive_locals.remove(name);
                Expr::LogicalAssign {
                    name: name.clone(),
                    op: *op,
                    expr: Box::new(self.fold_expr(expr)),
                    span: *span,
                }
            }
            Expr::LogicalPropertyAssign {
                object,
                object_expr,
                property,
                computed_key,
                op,
                expr,
                span,
            } => Expr::LogicalPropertyAssign {
                object: object.clone(),
                object_expr: object_expr
                    .as_ref()
                    .map(|object_expr| Box::new(self.fold_expr(object_expr))),
                property: property.clone(),
                computed_key: computed_key
                    .as_ref()
                    .map(|computed_key| Box::new(self.fold_expr(computed_key))),
                op: *op,
                expr: Box::new(self.fold_expr(expr)),
                span: *span,
            },
            Expr::Array { elements, span } => Expr::Array {
                elements: elements
                    .iter()
                    .map(|element| self.fold_expr(element))
                    .collect(),
                span: *span,
            },
            Expr::Object { props, span } => Expr::Object {
                props: props
                    .iter()
                    .map(|(key, value)| (key.clone(), self.fold_expr(value)))
                    .collect(),
                span: *span,
            },
            Expr::Index {
                object,
                index,
                span,
            } => Expr::Index {
                object: Box::new(self.fold_expr(object)),
                index: Box::new(self.fold_expr(index)),
                span: *span,
            },
            Expr::OptionalIndex {
                object,
                index,
                span,
            } => Expr::OptionalIndex {
                object: Box::new(self.fold_expr(object)),
                index: Box::new(self.fold_expr(index)),
                span: *span,
            },
            Expr::New { expr, args, span } => Expr::New {
                expr: Box::new(self.fold_expr(expr)),
                args: args.iter().map(|arg| self.fold_expr(arg)).collect(),
                span: *span,
            },
            Expr::TypeOf { expr, span } => Expr::TypeOf {
                expr: Box::new(self.fold_expr(expr)),
                span: *span,
            },
            Expr::Await { expr, span } => Expr::Await {
                expr: Box::new(self.fold_expr(expr)),
                span: *span,
            },
            Expr::InstanceOf {
                expr,
                type_expr,
                span,
            } => Expr::InstanceOf {
                expr: Box::new(self.fold_expr(expr)),
                type_expr: Box::new(self.fold_expr(type_expr)),
                span: *span,
            },
            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
                span,
            } => Expr::Ternary {
                condition: Box::new(self.fold_expr(condition)),
                then_expr: Box::new(self.fork().fold_expr(then_expr)),
                else_expr: Box::new(self.fork().fold_expr(else_expr)),
                span: *span,
            },
            Expr::ArrowFn { params, body, span } => Expr::ArrowFn {
                params: params.clone(),
                body: Box::new(BigIntStaticBuiltinFolder::default().fold_expr(body)),
                span: *span,
            },
            Expr::FunctionExpr {
                name,
                params,
                body,
                span,
            } => Expr::FunctionExpr {
                name: name.clone(),
                params: params.clone(),
                body: BigIntStaticBuiltinFolder::default().fold_stmts(body),
                span: *span,
            },
            Expr::Spread { expr, span } => Expr::Spread {
                expr: Box::new(self.fold_expr(expr)),
                span: *span,
            },
            Expr::PropertyAssign {
                object,
                property,
                value,
                span,
            } => Expr::PropertyAssign {
                object: Box::new(self.fold_expr(object)),
                property: property.clone(),
                value: Box::new(self.fold_expr(value)),
                span: *span,
            },
            Expr::IndexAssign {
                object,
                index,
                value,
                span,
            } => Expr::IndexAssign {
                object: Box::new(self.fold_expr(object)),
                index: Box::new(self.fold_expr(index)),
                value: Box::new(self.fold_expr(value)),
                span: *span,
            },
            Expr::Number { .. }
            | Expr::BigInt { .. }
            | Expr::String { .. }
            | Expr::Bool { .. }
            | Expr::Null { .. }
            | Expr::Undefined { .. }
            | Expr::This { .. }
            | Expr::Ident { .. } => expr.clone(),
        }
    }

    fn fork(&self) -> Self {
        Self {
            locals: self.locals.clone(),
            object_toprimitive_locals: self.object_toprimitive_locals.clone(),
        }
    }

    fn invalidate_assigned_in_stmts(&mut self, stmts: &[Stmt]) {
        for name in assigned_names_in_stmts(stmts) {
            self.locals.remove(&name);
            self.object_toprimitive_locals.remove(&name);
        }
    }

    fn invalidate_assigned_in_expr(&mut self, expr: &Expr) {
        for name in assigned_names_in_expr(expr) {
            self.locals.remove(&name);
            self.object_toprimitive_locals.remove(&name);
        }
    }

    fn merge_if_branch_facts(
        &mut self,
        then_body: &[Stmt],
        then_folder: &Self,
        else_body: &[Stmt],
        else_folder: &Self,
    ) {
        let mut assigned = assigned_names_in_stmts(then_body);
        assigned.extend(assigned_names_in_stmts(else_body));
        for name in assigned {
            self.locals.remove(&name);
            self.object_toprimitive_locals.remove(&name);
            if let (Some(then_expr), Some(else_expr)) =
                (then_folder.locals.get(&name), else_folder.locals.get(&name))
                && bigint_expr_const_value(then_expr) == bigint_expr_const_value(else_expr)
            {
                self.locals.insert(name.clone(), then_expr.clone());
            }
            if let (Some(then_expr), Some(else_expr)) = (
                then_folder.object_toprimitive_locals.get(&name),
                else_folder.object_toprimitive_locals.get(&name),
            ) && then_expr == else_expr
            {
                self.object_toprimitive_locals
                    .insert(name.clone(), then_expr.clone());
            }
        }
    }

    fn object_toprimitive_primitive_expr(&self, expr: &Expr) -> Option<Expr> {
        match expr {
            Expr::Ident { name, .. } => self.object_toprimitive_locals.get(name).cloned(),
            _ => object_toprimitive_supported_primitive_expr(expr),
        }
    }

    fn bigint_const_value(&self, expr: &Expr) -> Option<BigIntConst> {
        match expr {
            Expr::Ident { name, .. } => self
                .locals
                .get(name)
                .and_then(|expr| bigint_expr_const_value(expr)),
            _ => bigint_expr_const_value(expr),
        }
    }
}

fn static_bigint_builtin_const_expr(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::Number { .. } | Expr::BigInt { .. } => Some(expr.clone()),
        Expr::Unary {
            op, expr: inner, ..
        } if *op == UnaryOp::Negate
            && matches!(inner.as_ref(), Expr::Number { .. } | Expr::BigInt { .. }) =>
        {
            Some(expr.clone())
        }
        _ => None,
    }
}

fn bigint_expr_const_value(expr: &Expr) -> Option<BigIntConst> {
    match expr {
        Expr::BigInt { raw, span } => parse_bigint_literal(raw, *span)
            .ok()
            .and_then(|resolved| bigint_from_resolved(&resolved)),
        Expr::Unary { op, expr, .. } if *op == UnaryOp::Negate => {
            bigint_expr_const_value(expr).map(BigIntConst::negated)
        }
        _ => None,
    }
}

fn object_toprimitive_supported_primitive_expr(expr: &Expr) -> Option<Expr> {
    let Expr::Object { props, .. } = expr else {
        return None;
    };

    let value_of = props.iter().find(|(key, _)| key == "valueOf");
    if let Some((_, value)) = value_of {
        return match value {
            Expr::ArrowFn { params, body, .. } if params.is_empty() => match body.as_ref() {
                body if object_toprimitive_supported_return_expr(body) => Some(body.clone()),
                _ => None,
            },
            _ => None,
        };
    }

    props
        .iter()
        .find(|(key, _)| key == "toString")
        .and_then(|(_, value)| match value {
            Expr::ArrowFn { params, body, .. } if params.is_empty() => match body.as_ref() {
                body if object_toprimitive_supported_return_expr(body) => Some(body.clone()),
                _ => None,
            },
            _ => None,
        })
}

fn object_toprimitive_supported_return_expr(expr: &Expr) -> bool {
    match expr {
        Expr::BigInt { .. } | Expr::Bool { .. } | Expr::Null { .. } | Expr::Undefined { .. } => {
            true
        }
        Expr::Number { value, .. } => ValueTag::can_encode_number(*value),
        Expr::Unary { op, expr, .. } if *op == UnaryOp::Negate => {
            let Expr::Number { value, .. } = expr.as_ref() else {
                return false;
            };
            value.checked_neg().is_some_and(ValueTag::can_encode_number)
        }
        Expr::String { value, span } => bigint_from_string_builtin(value, *span)
            .ok()
            .is_some_and(|parsed| bigint_fits_runtime_mixed_string(&parsed)),
        _ => false,
    }
}

fn object_toprimitive_can_fold_for_bigint_op(expr: &Expr, op: BinaryOp) -> bool {
    if !object_toprimitive_supported_return_expr(expr) {
        return false;
    }
    match op {
        BinaryOp::EqualEqual | BinaryOp::BangEqual => true,
        BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
            !matches!(expr, Expr::Null { .. } | Expr::Undefined { .. })
        }
        _ => false,
    }
}

fn object_toprimitive_relational_expr(expr: Expr) -> Expr {
    let Expr::String { value, span } = expr else {
        return expr;
    };
    let Ok(parsed) = bigint_from_string_builtin(&value, span) else {
        return Expr::String { value, span };
    };
    if !bigint_fits_runtime_mixed_string(&parsed) {
        return Expr::String { value, span };
    }
    bigint_const_to_expr(parsed, span)
}

fn bigint_const_to_expr(value: BigIntConst, span: Span) -> Expr {
    let decimal = value.decimal_string();
    let magnitude = decimal.strip_prefix('-').unwrap_or(decimal.as_str());
    let literal = Expr::BigInt {
        raw: format!("{magnitude}n"),
        span,
    };
    if value.sign < 0 {
        Expr::Unary {
            op: UnaryOp::Negate,
            expr: Box::new(literal),
            span,
        }
    } else {
        literal
    }
}

fn is_bigint_static_builtin_callee(callee: &Expr) -> bool {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return false;
    };
    matches!(object.as_ref(), Expr::Ident { name, .. } if name == "BigInt")
        && matches!(property.as_str(), "asIntN" | "asUintN")
}

fn resolve_stmt(stmt: &Stmt) -> Result<ResolvedStmt, Diagnostic> {
    resolve_stmt_with_outer_bindings(stmt, &HashSet::new())
}

fn resolve_stmt_with_outer_bindings(
    stmt: &Stmt,
    outer_bindings: &HashSet<String>,
) -> Result<ResolvedStmt, Diagnostic> {
    match stmt {
        Stmt::ImportSideEffect { span, .. }
        | Stmt::ImportNamed { span, .. }
        | Stmt::ImportDefault { span, .. }
        | Stmt::ImportDefaultNamed { span, .. }
        | Stmt::ImportNamespace { span, .. }
        | Stmt::ImportDefaultNamespace { span, .. }
        | Stmt::ExportNamed { span, .. }
        | Stmt::ExportNamedFrom { span, .. }
        | Stmt::ExportAllFrom { span, .. }
        | Stmt::ExportNamespaceFrom { span, .. }
        | Stmt::ExportDecl { span, .. }
        | Stmt::ExportDefault { span, .. } => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-055: static module declarations parse in the frontend but module resolution and loading are not implemented".to_owned(),
            span: Some(*span),
        }),
        Stmt::Let { name, expr, span } => {
            if let Some(pattern) = parse_binding_pattern(name, Some(*span))? {
                Ok(ResolvedStmt::DestructureLet {
                    pattern,
                    expr: resolve_expr(expr)?,
                })
            } else {
                Ok(ResolvedStmt::Let(name.clone(), resolve_expr(expr)?))
            }
        }
        Stmt::Assign { name, expr, .. } => {
            Ok(ResolvedStmt::Assign(name.clone(), resolve_expr(expr)?))
        }
        Stmt::Expr { expr, .. } => {
            if let Some(assertion) = resolve_test262_assert_stmt(expr)? {
                return Ok(assertion);
            }
            // Detect exports.X = ... and module.exports = ... patterns
            if let Expr::PropertyAssign {
                object,
                property,
                value,
                ..
            } = expr
                && let Expr::Ident { name, .. } = object.as_ref()
            {
                if name == "exports" {
                    return Ok(ResolvedStmt::Export {
                        name: property.clone(),
                        expr: Box::new(resolve_expr(value)?),
                    });
                }
                if name == "module" && property == "exports" {
                    return Ok(ResolvedStmt::ModuleExportsAssign {
                        expr: Box::new(resolve_expr(value)?),
                    });
                }
            }
            Ok(ResolvedStmt::Expr(resolve_expr(expr)?))
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => Ok(ResolvedStmt::If {
            condition: resolve_expr(condition)?,
            then_body: then_body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            else_body: else_body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::While {
            condition, body, ..
        } => Ok(ResolvedStmt::While {
            condition: resolve_expr(condition)?,
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::Return { expr, .. } => Ok(ResolvedStmt::Return(resolve_expr(expr)?)),
        Stmt::Function {
            name,
            params,
            body,
            is_generator,
            span,
        } => {
            let resolved_params = params
                .iter()
                .map(|(param_name, default, is_rest)| {
                    Ok(ResolvedParam {
                        name: param_name.clone(),
                        default: default.as_ref().map(resolve_expr).transpose()?,
                        is_rest: *is_rest,
                        span: Some(*span),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ResolvedStmt::Function {
                name: name.clone(),
                params: resolved_params,
                body: body
                    .iter()
                    .map(resolve_stmt)
                    .collect::<Result<Vec<_>, _>>()?,
                is_generator: *is_generator,
            })
        }
        Stmt::ClassDecl {
            name,
            extends,
            body,
            static_blocks,
            private_elements,
            ..
        } => {
            // Parse extends (must be an identifier for now)
            let extends_name = match extends {
                Some(ext_expr) => match ext_expr.as_ref() {
                    Expr::Ident { name: parent, .. } => Some(parent.clone()),
                    _ => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "only simple inheritance (extends ClassName) is supported"
                                .to_owned(),
                            span: None,
                        });
                    }
                },
                None => None,
            };

            let (private_fields, static_private_fields, private_field_initializers, private_methods) =
                resolve_private_elements(name, extends_name.as_ref(), private_elements)?;

            // Parse class body to extract constructor and methods
            let mut constructor = None;
            let mut methods = Vec::new();
            let mut statics = Vec::new();

            for stmt in body {
                match stmt {
                    // Constructor method (identified by being a Function named "constructor")
                    Stmt::Function {
                        name: method_name,
                        params,
                        body: method_body,
                        span,
                        ..
                    } if method_name == "constructor" => {
                        reject_class_method_outer_local_references(
                            name,
                            method_name,
                            params,
                            method_body,
                            outer_bindings,
                        )?;
                        if constructor.is_some() {
                            return Err(Diagnostic {
                                code: DiagCode::DuplicateFunction,
                                message: "duplicate constructor definition".to_owned(),
                                span: None,
                            });
                        }
                        let resolved_params = params
                            .iter()
                            .map(|(param_name, default, is_rest)| {
                                Ok(ResolvedParam {
                                    name: param_name.clone(),
                                    default: default.as_ref().map(resolve_expr).transpose()?,
                                    is_rest: *is_rest,
                                    span: Some(*span),
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        let resolved_body = method_body.iter().map(resolve_stmt).collect::<Result<
                            Vec<_>,
                            _,
                        >>(
                        )?;
                        constructor = Some((
                            resolved_params,
                            place_private_field_initializers(
                                &private_field_initializers,
                                resolved_body,
                                extends_name.is_some(),
                            )?,
                        ));
                    }
                    // Regular methods
                    Stmt::Function {
                        name: method_name,
                        params,
                        body: method_body,
                        span,
                        ..
                    } => {
                        let captures = class_method_outer_local_captures(
                            name,
                            method_name,
                            params,
                            method_body,
                            outer_bindings,
                        )?;
                        let resolved_params = params
                            .iter()
                            .map(|(param_name, default, is_rest)| {
                                Ok(ResolvedParam {
                                    name: param_name.clone(),
                                    default: default.as_ref().map(resolve_expr).transpose()?,
                                    is_rest: *is_rest,
                                    span: Some(*span),
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        let resolved_body = method_body.iter().map(resolve_stmt).collect::<Result<
                            Vec<_>,
                            _,
                        >>(
                        )?;
                        if let Some(stripped) = method_name.strip_prefix("static::") {
                            statics.push((stripped.to_owned(), ResolvedExpr::Undefined));
                            methods.push(ClassMethod {
                                name: method_name.clone(),
                                params: resolved_params,
                                body: resolved_body,
                                captures,
                            });
                        } else {
                            methods.push(ClassMethod {
                                name: method_name.clone(),
                                params: resolved_params,
                                body: resolved_body,
                                captures,
                            });
                        }
                    }
                    // Static members (for now, we'll just skip them - not yet supported)
                    _ => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "class body may only contain methods and constructors"
                                .to_owned(),
                            span: None,
                        });
                    }
                }
            }
            methods.extend(private_methods);
            if !static_private_fields.is_empty() {
                let static_private_capture_names = static_private_fields
                    .iter()
                    .map(|(field, _, _)| static_private_field_local_name(name, field))
                    .collect::<Vec<_>>();
                for method in &mut methods {
                    if method.name.starts_with("static::") {
                        method.captures.extend(static_private_capture_names.iter().cloned());
                    }
                }
            }

            if constructor.is_none() && !private_field_initializers.is_empty() {
                let body = if extends_name.is_some() {
                    implicit_derived_private_field_constructor_body(&private_field_initializers)
                } else {
                    private_field_initializers.clone()
                };
                constructor = Some((Vec::new(), body));
            }

            let static_blocks = static_blocks
                .iter()
                .map(|block| {
                    validate_static_block_supported(block)?;
                    Ok((
                        block.span,
                        block
                            .body
                            .iter()
                            .map(resolve_stmt)
                            .collect::<Result<Vec<_>, _>>()?,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(ResolvedStmt::ClassDecl {
                name: name.clone(),
                extends: extends_name,
                constructor,
                methods,
                statics,
                static_blocks,
                private_fields,
                static_private_fields,
            })
        }
        Stmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
            ..
        } => Ok(ResolvedStmt::TryCatch {
            try_block: try_block
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            catch_param: catch_param.clone(),
            catch_block: catch_block
                .as_ref()
                .map(|b| b.iter().map(resolve_stmt).collect::<Result<Vec<_>, _>>())
                .transpose()?,
            finally_block: finally_block
                .as_ref()
                .map(|b| b.iter().map(resolve_stmt).collect::<Result<Vec<_>, _>>())
                .transpose()?,
        }),
        Stmt::Throw { expr, .. } => Ok(ResolvedStmt::Throw(resolve_expr(expr)?)),
        Stmt::Switch { expr, cases, .. } => {
            let resolved_cases = cases
                .iter()
                .map(|(cond, body)| {
                    Ok((
                        cond.as_ref().map(resolve_expr).transpose()?,
                        body.iter()
                            .map(resolve_stmt)
                            .collect::<Result<Vec<_>, _>>()?,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ResolvedStmt::Switch {
                expr: resolve_expr(expr)?,
                cases: resolved_cases,
            })
        }
        Stmt::DoWhile {
            body, condition, ..
        } => Ok(ResolvedStmt::DoWhile {
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            condition: resolve_expr(condition)?,
        }),
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            let resolved_init = if let Some(i) = init {
                Some(Box::new(resolve_stmt(i)?))
            } else {
                None
            };
            let resolved_condition = if let Some(cond) = condition {
                Some(resolve_expr(cond)?)
            } else {
                None
            };
            let resolved_update = if let Some(upd) = update {
                Some(resolve_for_update_expr(upd)?)
            } else {
                None
            };
            Ok(ResolvedStmt::For {
                init: resolved_init,
                condition: resolved_condition,
                update: resolved_update,
                body: body
                    .iter()
                    .map(resolve_stmt)
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
        Stmt::ForIn {
            var, iter, body, ..
        } => Ok(ResolvedStmt::ForIn {
            var: var.clone(),
            iter: resolve_expr(iter)?,
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::ForOf {
            var, iter, body, ..
        } => Ok(ResolvedStmt::ForOf {
            var: var.clone(),
            iter: resolve_expr(iter)?,
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::Labeled { label, body, .. } => Ok(ResolvedStmt::Labeled {
            label: label.clone(),
            body: Box::new(resolve_stmt(body)?),
        }),
        Stmt::Break { label, .. } => Ok(ResolvedStmt::Break {
            label: label.clone(),
        }),
        Stmt::Continue { label, .. } => Ok(ResolvedStmt::Continue {
            label: label.clone(),
        }),
    }
}

fn resolve_expr(expr: &Expr) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
        Expr::Number { value, .. } => Ok(ResolvedExpr::Number(*value)),
        Expr::BigInt { raw, span } => parse_bigint_literal(raw, *span),
        Expr::String { value, .. } => Ok(ResolvedExpr::String(value.clone())),
        Expr::Bool { value, .. } => Ok(ResolvedExpr::Bool(*value)),
        Expr::Null { .. } => Ok(ResolvedExpr::Null),
        Expr::Undefined { .. } => Ok(ResolvedExpr::Undefined),
        Expr::This { span } => Ok(ResolvedExpr::This { span: *span }),
        Expr::Await { expr, span } => {
            let resolved = resolve_expr(expr)?;
            if matches!(
                resolved,
                ResolvedExpr::BuiltinCall {
                    builtin: BuiltinId::ReadStdinUtf8,
                    ..
                }
            ) {
                Ok(resolved)
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-294: await is only supported for Bun.file(\"/dev/stdin\").text() stdin lowering in this slice".to_owned(),
                    span: Some(*span),
                })
            }
        }
        Expr::Ident { name, .. } => Ok(ResolvedExpr::Ident(name.clone())),
        Expr::InstanceOf {
            expr, type_expr, ..
        } => Ok(ResolvedExpr::Binary {
            left: Box::new(resolve_expr(expr)?),
            op: BinaryOp::InstanceOf,
            right: Box::new(resolve_expr(type_expr)?),
        }),
        Expr::Ternary { span, .. } => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "ternary operator not yet supported".to_owned(),
            span: Some(*span),
        }),
        Expr::Unary { op, expr, span } => {
            if matches!(
                op,
                UnaryOp::Increment
                    | UnaryOp::Decrement
                    | UnaryOp::PreIncrement
                    | UnaryOp::PreDecrement
            ) {
                return Err(increment_update_diagnostic(*span));
            }
            if expr_contains_bigint(expr) {
                let resolved = resolve_expr(expr)?;
                if *op == UnaryOp::Negate {
                    if let Some(value) = bigint_from_resolved(&resolved) {
                        return Ok(bigint_to_resolved(value.negated()));
                    }
                    return Ok(ResolvedExpr::Unary {
                        op: *op,
                        expr: Box::new(resolved),
                    });
                }
                if *op == UnaryOp::BitwiseNot {
                    if let Some(value) = bigint_from_resolved(&resolved) {
                        return Ok(bigint_to_resolved(fold_bigint_unary_bitwise_not(
                            value, *span,
                        )?));
                    }
                    return Ok(ResolvedExpr::Unary {
                        op: *op,
                        expr: Box::new(resolved),
                    });
                }
                if let Some(message) = bigint_unary_op_issue(*op) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: message.to_owned(),
                        span: Some(*span),
                    });
                }
                return Ok(ResolvedExpr::Unary {
                    op: *op,
                    expr: Box::new(resolved),
                });
            }
            Ok(ResolvedExpr::Unary {
                op: *op,
                expr: Box::new(resolve_expr(expr)?),
            })
        }
        Expr::Binary {
            left,
            op,
            right,
            span,
        } => {
            let left_contains_bigint = expr_contains_bigint(left);
            let right_contains_bigint = expr_contains_bigint(right);
            if left_contains_bigint || right_contains_bigint {
                let left_resolved = resolve_expr(left)?;
                let right_resolved = resolve_expr(right)?;
                if bigint_arithmetic_op(*op) {
                    if let (Some(left_value), Some(right_value)) = (
                        bigint_from_resolved(&left_resolved),
                        bigint_from_resolved(&right_resolved),
                    ) {
                        let result = fold_bigint_binary(left_value, *op, right_value, *span)?;
                        return Ok(bigint_to_resolved(result));
                    }
                    if matches!(
                        op,
                        BinaryOp::Add
                            | BinaryOp::Subtract
                            | BinaryOp::Multiply
                            | BinaryOp::Divide
                            | BinaryOp::Modulo
                            | BinaryOp::Power
                    ) {
                        return Ok(ResolvedExpr::Binary {
                            left: Box::new(left_resolved),
                            op: *op,
                            right: Box::new(right_resolved),
                        });
                    }
                }
                if matches!(
                    op,
                    BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor
                ) {
                    if let (Some(left_value), Some(right_value)) = (
                        bigint_from_resolved(&left_resolved),
                        bigint_from_resolved(&right_resolved),
                    ) {
                        let result = fold_bigint_binary(left_value, *op, right_value, *span)?;
                        return Ok(bigint_to_resolved(result));
                    }
                    return Ok(ResolvedExpr::Binary {
                        left: Box::new(left_resolved),
                        op: *op,
                        right: Box::new(right_resolved),
                    });
                }
                if matches!(
                    op,
                    BinaryOp::LeftShift | BinaryOp::RightShift | BinaryOp::UnsignedRightShift
                ) {
                    if *op == BinaryOp::UnsignedRightShift {
                        return Err(bigint_shift_diagnostic(*span));
                    }
                    if let (Some(left_value), Some(right_value)) = (
                        bigint_from_resolved(&left_resolved),
                        bigint_from_resolved(&right_resolved),
                    ) {
                        let result = fold_bigint_binary(left_value, *op, right_value, *span)?;
                        return Ok(bigint_to_resolved(result));
                    }
                    return Err(bigint_shift_diagnostic(*span));
                }
                let diagnostic = match op {
                    BinaryOp::Add
                    | BinaryOp::Subtract
                    | BinaryOp::Multiply
                    | BinaryOp::Divide
                    | BinaryOp::Modulo => Some(bigint_dynamic_runtime_diagnostic(*span)),
                    BinaryOp::Power => Some(bigint_exponentiation_diagnostic(*span)),
                    BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor => {
                        Some(bigint_bitwise_diagnostic(*span))
                    }
                    BinaryOp::LeftShift | BinaryOp::RightShift | BinaryOp::UnsignedRightShift => {
                        Some(bigint_shift_diagnostic(*span))
                    }
                    BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual
                    | BinaryOp::StrictEqual
                    | BinaryOp::EqualEqual
                    | BinaryOp::BangEqual
                    | BinaryOp::StrictNotEqual => {
                        if let Some(folded) = fold_bigint_static_abstract_equality(
                            &left_resolved,
                            *op,
                            &right_resolved,
                            *span,
                        )? {
                            return Ok(folded);
                        }
                        return Ok(ResolvedExpr::Binary {
                            left: Box::new(left_resolved),
                            op: *op,
                            right: Box::new(right_resolved),
                        });
                    }
                    BinaryOp::And | BinaryOp::Or | BinaryOp::NullishCoalesce => None,
                    BinaryOp::InstanceOf | BinaryOp::In => Some(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-261: BigInt object/coercion operator boundaries are tracked separately from literal runtime values".to_owned(),
                        span: Some(*span),
                    }),
                };
                if let Some(diagnostic) = diagnostic {
                    return Err(diagnostic);
                }
            }
            Ok(ResolvedExpr::Binary {
                left: Box::new(resolve_expr(left)?),
                op: *op,
                right: Box::new(resolve_expr(right)?),
            })
        }
        Expr::Call { callee, args, .. } if is_require_call(callee, args) => {
            if let [
                Expr::String {
                    value: specifier, ..
                },
            ] = args.as_slice()
            {
                Ok(ResolvedExpr::ModuleLoad {
                    specifier: specifier.clone(),
                })
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "require() expects a string literal argument".to_owned(),
                    span: None,
                })
            }
        }
        Expr::Call { callee, args, span } => {
            if is_test262_assert_reference_error_probe(callee, args) {
                return Ok(ResolvedExpr::Undefined);
            }
            let resolved_args = args
                .iter()
                .map(resolve_expr)
                .collect::<Result<Vec<_>, _>>()?;
            if let Expr::Ident { name, .. } = callee.as_ref()
                && name == "BigInt"
            {
                return resolve_bigint_function_call(&resolved_args, *span);
            }
            if let Some(resolved) =
                resolve_bigint_static_function_call(callee.as_ref(), &resolved_args, *span)?
            {
                return Ok(resolved);
            }
            if let Some(builtin) = resolve_builtin_call(callee.as_ref(), args)? {
                let builtin_args = if matches!(builtin, BuiltinId::ReadStdinUtf8) {
                    Vec::new()
                } else {
                    resolved_args
                };
                Ok(ResolvedExpr::BuiltinCall {
                    builtin,
                    args: builtin_args,
                })
            } else if let Expr::Member {
                object, property, ..
            } = callee.as_ref()
            {
                Ok(ResolvedExpr::MethodCall {
                    object: Box::new(resolve_expr(object)?),
                    method: property.clone(),
                    args: resolved_args,
                    span: *span,
                })
            } else {
                Ok(ResolvedExpr::Call {
                    callee: Box::new(resolve_expr(callee)?),
                    args: resolved_args,
                    span: *span,
                })
            }
        }
        Expr::Assign { name, expr, .. } => Ok(ResolvedExpr::Assign {
            name: name.clone(),
            expr: Box::new(resolve_expr(expr)?),
        }),
        Expr::LogicalAssign { name, op, expr, .. } => Ok(ResolvedExpr::LogicalAssign {
            name: name.clone(),
            op: *op,
            expr: Box::new(resolve_expr(expr)?),
        }),
        Expr::LogicalPropertyAssign {
            object,
            object_expr,
            property,
            computed_key,
            op,
            expr,
            ..
        } => {
            if is_private_member_key(property) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-255: private field logical assignment is not supported in this private field runtime slice".to_owned(),
                    span: span_of_expr(expr),
                });
            }
            match (object_expr.as_ref(), computed_key.as_ref()) {
                (Some(object_expr), Some(key)) => Ok(ResolvedExpr::LogicalComputedMemberAssign {
                    object: Box::new(resolve_expr(object_expr)?),
                    key: Box::new(resolve_expr(key)?),
                    op: *op,
                    expr: Box::new(resolve_expr(expr)?),
                }),
                (Some(object_expr), None) => Ok(ResolvedExpr::LogicalMemberAssign {
                    object: Box::new(resolve_expr(object_expr)?),
                    key: property.clone(),
                    op: *op,
                    expr: Box::new(resolve_expr(expr)?),
                }),
                (None, Some(key)) => Ok(ResolvedExpr::LogicalComputedPropertyAssign {
                    object: object.clone(),
                    key: Box::new(resolve_expr(key)?),
                    op: *op,
                    expr: Box::new(resolve_expr(expr)?),
                }),
                (None, None) => Ok(ResolvedExpr::LogicalPropertyAssign {
                    object: object.clone(),
                    key: property.clone(),
                    op: *op,
                    expr: Box::new(resolve_expr(expr)?),
                }),
            }
        }
        Expr::Member {
            object,
            property,
            span,
        } => {
            if let Expr::Ident { name, .. } = object.as_ref()
                && name == "process"
            {
                return match property.as_str() {
                    "argv" => Ok(ResolvedExpr::BuiltinCall {
                        builtin: BuiltinId::ProcessArgv,
                        args: Vec::new(),
                    }),
                    "env" => Ok(ResolvedExpr::BuiltinCall {
                        builtin: BuiltinId::ProcessEnv,
                        args: Vec::new(),
                    }),
                    _ => Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!("process.{} is not supported in this milestone", property),
                        span: span_of_expr(expr),
                    }),
                };
            }

            let resolved_object = Box::new(resolve_expr(object)?);
            if property == "length" {
                Ok(ResolvedExpr::BuiltinProperty {
                    builtin: BuiltinPropertyId::Length,
                    object: resolved_object,
                    span: *span,
                })
            } else {
                Ok(ResolvedExpr::PropertyAccess {
                    object: resolved_object,
                    key: property.clone(),
                    span: *span,
                })
            }
        }
        Expr::OptionalMember {
            object,
            property,
            span,
        } => {
            if is_private_member_key(property) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-253: optional chaining of private fields is not supported"
                        .to_owned(),
                    span: Some(*span),
                });
            }
            Ok(ResolvedExpr::OptionalPropertyAccess {
                object: Box::new(resolve_expr(object)?),
                key: property.clone(),
                span: *span,
            })
        }
        Expr::OptionalIndex {
            object,
            index,
            span,
        } => Ok(ResolvedExpr::OptionalComputedIndex {
            object: Box::new(resolve_expr(object)?),
            index: Box::new(resolve_expr(index)?),
            span: *span,
        }),
        Expr::OptionalCall { callee, args, span } => Ok(ResolvedExpr::OptionalCall {
            callee: Box::new(resolve_expr(callee)?),
            args: args
                .iter()
                .map(resolve_expr)
                .collect::<Result<Vec<_>, _>>()?,
            span: *span,
        }),
        Expr::Array { elements, .. } => Ok(ResolvedExpr::Array(
            elements
                .iter()
                .map(resolve_expr)
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Expr::Object { props, .. } => Ok(ResolvedExpr::Object(
            props
                .iter()
                .map(|(k, v)| Ok((k.clone(), resolve_expr(v)?)))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Expr::Index { object, index, .. } => {
            // For string literal keys, use PropertyAccess (object property semantics)
            // For other expressions, use ComputedIndex (array indexing semantics)
            if let Expr::String { value, .. } = index.as_ref() {
                Ok(ResolvedExpr::PropertyAccess {
                    object: Box::new(resolve_expr(object)?),
                    key: value.clone(),
                    span: span_of_expr(expr).unwrap_or(Span { start: 0, end: 0 }),
                })
            } else {
                Ok(ResolvedExpr::ComputedIndex {
                    object: Box::new(resolve_expr(object)?),
                    index: Box::new(resolve_expr(index)?),
                })
            }
        }
        Expr::New {
            expr: new_expr,
            args,
            span,
        } => {
            // Extract class name from identifier
            if let Expr::Ident {
                name: class_name, ..
            } = new_expr.as_ref()
            {
                if class_name == "BigInt" {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "issue-262: BigInt is not a constructor; use BigInt(...) without new"
                                .to_owned(),
                        span: Some(*span),
                    });
                }
                let resolved_args = args
                    .iter()
                    .map(resolve_expr)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ResolvedExpr::New {
                    class_name: class_name.clone(),
                    args: resolved_args,
                    span: *span,
                })
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "only new ClassName(...) is supported".to_owned(),
                    span: None,
                })
            }
        }
        Expr::PropertyAssign {
            object,
            property,
            value,
            span,
        } => Ok(ResolvedExpr::PropertyAssign {
            object: Box::new(resolve_expr(object)?),
            key: property.clone(),
            value: Box::new(resolve_expr(value)?),
            span: *span,
        }),
        Expr::IndexAssign {
            object,
            index,
            value,
            span,
        } => {
            if let Expr::String { value: key, .. } = index.as_ref() {
                return Ok(ResolvedExpr::PropertyAssign {
                    object: Box::new(resolve_expr(object)?),
                    key: key.clone(),
                    value: Box::new(resolve_expr(value)?),
                    span: *span,
                });
            }
            Ok(ResolvedExpr::PropertyAssignDynamic {
                object: Box::new(resolve_expr(object)?),
                key: Box::new(resolve_expr(index)?),
                value: Box::new(resolve_expr(value)?),
            })
        }
        Expr::ArrowFn { params, body, .. } => {
            let resolved_body = resolve_expr(body)?;
            Ok(ResolvedExpr::ArrowFn {
                params: params.clone(),
                body: Box::new(resolved_body),
            })
        }
        Expr::FunctionExpr {
            name, params, body, ..
        } => Ok(ResolvedExpr::FunctionExpr {
            name: name.clone(),
            params: params
                .iter()
                .map(|(param_name, default, is_rest)| {
                    Ok(ResolvedParam {
                        name: param_name.clone(),
                        default: default.as_ref().map(resolve_expr).transpose()?,
                        is_rest: *is_rest,
                        span: None,
                    })
                })
                .collect::<Result<Vec<_>, Diagnostic>>()?,
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Expr::Spread { expr, .. } => Ok(ResolvedExpr::Spread(Box::new(resolve_expr(expr)?))),
        Expr::TypeOf { expr, .. } => Ok(ResolvedExpr::Unary {
            op: UnaryOp::TypeOf,
            expr: Box::new(resolve_expr(expr)?),
        }),
    }
}

fn resolve_for_update_expr(expr: &Expr) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
        Expr::Unary {
            op:
                op @ (UnaryOp::Increment
                | UnaryOp::Decrement
                | UnaryOp::PreIncrement
                | UnaryOp::PreDecrement),
            expr,
            span,
        } => {
            let Expr::Ident { name, .. } = expr.as_ref() else {
                return Err(increment_update_diagnostic(*span));
            };
            let binary_op = match op {
                UnaryOp::Increment | UnaryOp::PreIncrement => BinaryOp::Add,
                UnaryOp::Decrement | UnaryOp::PreDecrement => BinaryOp::Subtract,
                _ => unreachable!("matched for-loop update increment/decrement operator"),
            };
            Ok(ResolvedExpr::Assign {
                name: name.clone(),
                expr: Box::new(ResolvedExpr::Binary {
                    left: Box::new(ResolvedExpr::Ident(name.clone())),
                    op: binary_op,
                    right: Box::new(ResolvedExpr::Number(1)),
                }),
            })
        }
        _ => resolve_expr(expr),
    }
}

fn increment_update_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message:
            "issue-268: for-loop increment/decrement updates currently require an identifier target"
                .to_owned(),
        span: Some(span),
    }
}
