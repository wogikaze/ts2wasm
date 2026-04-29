use std::collections::{HashMap, HashSet};

use ts2wasm_frontend::{
    BinaryOp, ClassPrivateElement, ClassStaticBlock, DiagCode, Diagnostic, Expr, Span, Stmt,
    UnaryOp,
};

use super::binding_pattern::parse_binding_pattern;
use super::builtin::BuiltinId;
use super::builtin::BuiltinPropertyId;
use super::builtin_resolved::{ClassMethod, ResolvedExpr, ResolvedParam, ResolvedStmt};

pub fn resolve_builtins(program: &[Stmt]) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    BigIntRuntimeGuard::default().visit_stmts(program)?;
    program.iter().map(resolve_stmt).collect()
}

fn resolve_stmt(stmt: &Stmt) -> Result<ResolvedStmt, Diagnostic> {
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

            let (private_fields, private_field_initializers, private_methods) =
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
                    } if method_name == "constructor" => {
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
                            prepend_private_field_initializers(
                                &private_field_initializers,
                                resolved_body,
                            ),
                        ));
                    }
                    // Regular methods
                    Stmt::Function {
                        name: method_name,
                        params,
                        body: method_body,
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
                            });
                        } else {
                            methods.push(ClassMethod {
                                name: method_name.clone(),
                                params: resolved_params,
                                body: resolved_body,
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

            if constructor.is_none() && !private_field_initializers.is_empty() {
                constructor = Some((Vec::new(), private_field_initializers.clone()));
            }

            let static_blocks = static_blocks
                .iter()
                .map(|block| {
                    validate_static_block_supported(block)?;
                    block
                        .body
                        .iter()
                        .map(resolve_stmt)
                        .collect::<Result<Vec<_>, _>>()
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
                    let syntactic_number_mix = (left_contains_bigint
                        && matches!(right.as_ref(), Expr::Number { .. }))
                        || (right_contains_bigint && matches!(left.as_ref(), Expr::Number { .. }));
                    if syntactic_number_mix {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-260: mixed Number/BigInt arithmetic is not implemented in the dynamic BigInt runtime slice".to_owned(),
                            span: Some(*span),
                        });
                    }
                    if matches!(op, BinaryOp::Add | BinaryOp::Subtract) {
                        return Ok(ResolvedExpr::Binary {
                            left: Box::new(left_resolved),
                            op: *op,
                            right: Box::new(right_resolved),
                        });
                    }
                }
                let issue = match op {
                    BinaryOp::Add
                    | BinaryOp::Subtract
                    | BinaryOp::Multiply
                    | BinaryOp::Divide
                    | BinaryOp::Modulo
                    | BinaryOp::Power
                    | BinaryOp::BitwiseAnd
                    | BinaryOp::BitwiseOr
                    | BinaryOp::BitwiseXor
                    | BinaryOp::LeftShift
                    | BinaryOp::RightShift
                    | BinaryOp::UnsignedRightShift => {
                        "issue-260: BigInt arithmetic and bitwise operators are tracked separately from literal runtime values"
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
                    BinaryOp::And | BinaryOp::Or | BinaryOp::NullishCoalesce => "",
                    BinaryOp::InstanceOf | BinaryOp::In => {
                        "issue-261: BigInt object/coercion operator boundaries are tracked separately from literal runtime values"
                    }
                };
                if !issue.is_empty() {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: issue.to_owned(),
                        span: Some(*span),
                    });
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

fn resolve_private_elements(
    class_name: &str,
    extends_name: Option<&String>,
    private_elements: &[ClassPrivateElement],
) -> Result<(Vec<String>, Vec<ResolvedStmt>, Vec<ClassMethod>), Diagnostic> {
    let mut fields = Vec::new();
    let mut initializers = Vec::new();
    let mut methods = Vec::new();
    let mut seen = HashSet::new();

    for element in private_elements {
        match element {
            ClassPrivateElement::Field {
                name,
                name_span,
                value,
                is_static,
                span,
            } => {
                if *is_static {
                    return Err(unsupported_private_element(
                        "static private fields are not supported in this private field runtime slice",
                        *span,
                    ));
                }
                if extends_name.is_some() {
                    return Err(unsupported_private_element(
                        "private fields on derived classes require coordinated super() initialization support",
                        *span,
                    ));
                }
                if !seen.insert(name.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: duplicate private field `#{name}` in class `{class_name}`"
                        ),
                        span: Some(*span),
                    });
                }
                fields.push(name.clone());
                initializers.push(ResolvedStmt::Expr(ResolvedExpr::PropertyAssign {
                    object: Box::new(ResolvedExpr::This { span: *name_span }),
                    key: format!("#{name}"),
                    value: Box::new(
                        value
                            .as_ref()
                            .map(resolve_expr)
                            .transpose()?
                            .unwrap_or(ResolvedExpr::Undefined),
                    ),
                    span: *span,
                }));
            }
            ClassPrivateElement::Method {
                name,
                params,
                body,
                is_static,
                span,
                ..
            } => {
                if extends_name.is_some() {
                    return Err(unsupported_private_element(
                        "private methods on derived classes require full private brand semantics",
                        *span,
                    ));
                }
                if !seen.insert(name.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: duplicate private element `#{name}` in class `{class_name}`"
                        ),
                        span: Some(*span),
                    });
                }
                methods.push(ClassMethod {
                    name: if *is_static {
                        format!("static::#{name}")
                    } else {
                        format!("#{name}")
                    },
                    params: params
                        .iter()
                        .map(|(param_name, default, is_rest)| {
                            Ok(ResolvedParam {
                                name: param_name.clone(),
                                default: default.as_ref().map(resolve_expr).transpose()?,
                                is_rest: *is_rest,
                                span: Some(*span),
                            })
                        })
                        .collect::<Result<Vec<_>, Diagnostic>>()?,
                    body: body
                        .iter()
                        .map(resolve_stmt)
                        .collect::<Result<Vec<_>, _>>()?,
                });
            }
            ClassPrivateElement::Getter {
                name,
                body,
                is_static,
                span,
                ..
            } => {
                if *is_static {
                    return Err(unsupported_private_element(
                        "static private accessors are not supported in this private accessor runtime slice",
                        *span,
                    ));
                }
                if extends_name.is_some() {
                    return Err(unsupported_private_element(
                        "private accessors on derived classes require full private brand semantics",
                        *span,
                    ));
                }
                if !seen.insert(name.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: duplicate private element `#{name}` in class `{class_name}`"
                        ),
                        span: Some(*span),
                    });
                }
                methods.push(ClassMethod {
                    name: private_getter_method_name(name),
                    params: Vec::new(),
                    body: body
                        .iter()
                        .map(resolve_stmt)
                        .collect::<Result<Vec<_>, _>>()?,
                });
            }
            ClassPrivateElement::Setter {
                name,
                param,
                body,
                is_static,
                span,
                ..
            } => {
                if *is_static {
                    return Err(unsupported_private_element(
                        "static private accessors are not supported in this private accessor runtime slice",
                        *span,
                    ));
                }
                if extends_name.is_some() {
                    return Err(unsupported_private_element(
                        "private accessors on derived classes require full private brand semantics",
                        *span,
                    ));
                }
                if block_contains_return_stmt(body) {
                    return Err(unsupported_private_element(
                        "private setters with explicit return are not supported in this private setter runtime slice",
                        *span,
                    ));
                }
                if !seen.insert(name.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-255: duplicate private element `#{name}` in class `{class_name}`"
                        ),
                        span: Some(*span),
                    });
                }
                let mut resolved_body = body
                    .iter()
                    .map(resolve_stmt)
                    .collect::<Result<Vec<_>, _>>()?;
                resolved_body.push(ResolvedStmt::Return(ResolvedExpr::Ident(param.clone())));
                methods.push(ClassMethod {
                    name: private_setter_method_name(name),
                    params: vec![ResolvedParam {
                        name: param.clone(),
                        default: None,
                        is_rest: false,
                        span: Some(*span),
                    }],
                    body: resolved_body,
                });
            }
        }
    }

    Ok((fields, initializers, methods))
}

fn prepend_private_field_initializers(
    initializers: &[ResolvedStmt],
    mut body: Vec<ResolvedStmt>,
) -> Vec<ResolvedStmt> {
    if initializers.is_empty() {
        return body;
    }
    let mut merged = initializers.to_vec();
    merged.append(&mut body);
    merged
}

fn unsupported_private_element(detail: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("issue-255: {detail}"),
        span: Some(span),
    }
}

fn is_private_member_key(key: &str) -> bool {
    key.starts_with('#')
}

fn private_getter_method_name(name: &str) -> String {
    format!("#get::{name}")
}

fn private_setter_method_name(name: &str) -> String {
    format!("#set::{name}")
}

fn block_contains_return_stmt(statements: &[Stmt]) -> bool {
    statements.iter().any(stmt_contains_return_stmt)
}

fn stmt_contains_return_stmt(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Return { .. } => true,
        Stmt::If {
            then_body,
            else_body,
            ..
        } => block_contains_return_stmt(then_body) || block_contains_return_stmt(else_body),
        Stmt::While { body, .. }
        | Stmt::DoWhile { body, .. }
        | Stmt::ForIn { body, .. }
        | Stmt::ForOf { body, .. } => block_contains_return_stmt(body),
        Stmt::For { init, body, .. } => {
            init.as_deref().is_some_and(stmt_contains_return_stmt)
                || block_contains_return_stmt(body)
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_contains_return_stmt(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|body| block_contains_return_stmt(body))
                || finally_block
                    .as_ref()
                    .is_some_and(|body| block_contains_return_stmt(body))
        }
        Stmt::Switch { cases, .. } => cases
            .iter()
            .any(|(_, body)| block_contains_return_stmt(body)),
        Stmt::Labeled { body, .. } => stmt_contains_return_stmt(body),
        Stmt::Function { .. } | Stmt::ClassDecl { .. } => false,
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
        | Stmt::ExportDecl { .. }
        | Stmt::ExportDefault { .. }
        | Stmt::Let { .. }
        | Stmt::Assign { .. }
        | Stmt::Expr { .. }
        | Stmt::Throw { .. }
        | Stmt::Break { .. }
        | Stmt::Continue { .. } => false,
    }
}

fn parse_bigint_literal(raw: &str, span: Span) -> Result<ResolvedExpr, Diagnostic> {
    let Some(body) = raw.strip_suffix('n') else {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("issue-259: invalid BigInt literal `{raw}` reached runtime lowering"),
            span: Some(span),
        });
    };
    let (radix, digits) =
        if let Some(digits) = body.strip_prefix("0b").or_else(|| body.strip_prefix("0B")) {
            (2_u32, digits)
        } else if let Some(digits) = body.strip_prefix("0o").or_else(|| body.strip_prefix("0O")) {
            (8_u32, digits)
        } else if let Some(digits) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
            (16_u32, digits)
        } else {
            (10_u32, body)
        };

    let mut decimal_digits = vec![0_u8];
    let mut magnitude: u64 = 0;
    let mut magnitude_overflowed = false;
    for ch in digits.chars() {
        let Some(digit) = ch.to_digit(radix) else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("issue-259: invalid BigInt literal digit in `{raw}`"),
                span: Some(span),
            });
        };
        decimal_mul_add(&mut decimal_digits, radix as u8, digit as u8);
        if !magnitude_overflowed {
            if let Some(next) = magnitude
                .checked_mul(radix as u64)
                .and_then(|value| value.checked_add(digit as u64))
            {
                magnitude = next;
            } else {
                magnitude_overflowed = true;
            }
        }
    }

    trim_decimal_zeroes(&mut decimal_digits);
    let decimal = decimal_digits
        .iter()
        .map(|digit| char::from(b'0' + *digit))
        .collect::<String>();
    let sign = if decimal == "0" { 0 } else { 1 };
    let (limb_low, limb_high) = if magnitude_overflowed {
        (0, 0)
    } else {
        (magnitude as u32, (magnitude >> 32) as u32)
    };

    Ok(ResolvedExpr::BigIntLiteral {
        decimal,
        sign,
        limb_low,
        limb_high,
    })
}

fn decimal_mul_add(digits: &mut Vec<u8>, radix: u8, add: u8) {
    let mut carry = add as u16;
    for digit in digits.iter_mut().rev() {
        let value = (*digit as u16) * (radix as u16) + carry;
        *digit = (value % 10) as u8;
        carry = value / 10;
    }
    while carry > 0 {
        digits.insert(0, (carry % 10) as u8);
        carry /= 10;
    }
}

fn trim_decimal_zeroes(digits: &mut Vec<u8>) {
    while digits.len() > 1 && digits.first() == Some(&0) {
        digits.remove(0);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BigIntConst {
    sign: i32,
    digits: Vec<u8>,
}

impl BigIntConst {
    fn zero() -> Self {
        Self {
            sign: 0,
            digits: vec![0],
        }
    }

    fn from_decimal(sign: i32, decimal: &str) -> Self {
        let body = decimal.strip_prefix('-').unwrap_or(decimal);
        let mut digits = body
            .bytes()
            .filter(|byte| byte.is_ascii_digit())
            .map(|byte| byte - b'0')
            .collect::<Vec<_>>();
        if digits.is_empty() {
            digits.push(0);
        }
        trim_decimal_zeroes(&mut digits);
        let sign = if digits == [0] { 0 } else { sign.signum() };
        Self { sign, digits }
    }

    fn negated(mut self) -> Self {
        self.sign = -self.sign;
        self
    }

    fn decimal_string(&self) -> String {
        let mut out = String::new();
        if self.sign < 0 {
            out.push('-');
        }
        out.extend(self.digits.iter().map(|digit| char::from(b'0' + *digit)));
        out
    }

    fn fits_runtime_signed_i64(&self) -> bool {
        decimal_digits_to_u64(&self.digits).is_some_and(|magnitude| magnitude <= i64::MAX as u64)
    }
}

fn bigint_from_resolved(expr: &ResolvedExpr) -> Option<BigIntConst> {
    match expr {
        ResolvedExpr::BigIntLiteral { decimal, sign, .. } => {
            Some(BigIntConst::from_decimal(*sign, decimal))
        }
        _ => None,
    }
}

fn bigint_to_resolved(value: BigIntConst) -> ResolvedExpr {
    let magnitude = decimal_digits_to_u64(&value.digits);
    let (limb_low, limb_high) = magnitude
        .map(|magnitude| (magnitude as u32, (magnitude >> 32) as u32))
        .unwrap_or((0, 0));
    ResolvedExpr::BigIntLiteral {
        decimal: value.decimal_string(),
        sign: value.sign,
        limb_low,
        limb_high,
    }
}

fn resolve_bigint_function_call(
    args: &[ResolvedExpr],
    span: Span,
) -> Result<ResolvedExpr, Diagnostic> {
    let [arg] = args else {
        return Err(bigint_builtin_unsupported_diagnostic(span));
    };
    let value = match arg {
        ResolvedExpr::BigIntLiteral { .. } => return Ok(arg.clone()),
        ResolvedExpr::String(value) => bigint_from_string_builtin(value, span)?,
        ResolvedExpr::Bool(true) => BigIntConst::from_decimal(1, "1"),
        ResolvedExpr::Bool(false) => BigIntConst::zero(),
        ResolvedExpr::Number(value) => bigint_from_i32(*value),
        ResolvedExpr::Unary { op, expr }
            if *op == UnaryOp::Negate && matches!(expr.as_ref(), ResolvedExpr::Number(_)) =>
        {
            let ResolvedExpr::Number(value) = expr.as_ref() else {
                unreachable!("guarded by matches")
            };
            bigint_from_i64(-i64::from(*value))
        }
        _ => return Err(bigint_builtin_unsupported_diagnostic(span)),
    };
    Ok(bigint_to_resolved(value))
}

fn bigint_from_i32(value: i32) -> BigIntConst {
    bigint_from_i64(i64::from(value))
}

fn bigint_from_i64(value: i64) -> BigIntConst {
    if value == 0 {
        return BigIntConst::zero();
    }
    let sign = value.signum() as i32;
    BigIntConst::from_decimal(sign, value.unsigned_abs().to_string().as_str())
}

fn bigint_from_string_builtin(value: &str, span: Span) -> Result<BigIntConst, Diagnostic> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(BigIntConst::zero());
    }
    let (sign, explicit_sign, digits) = if let Some(digits) = trimmed.strip_prefix('-') {
        (-1, true, digits)
    } else if let Some(digits) = trimmed.strip_prefix('+') {
        (1, true, digits)
    } else {
        (1, false, trimmed)
    };

    let (radix, digits) = if let Some(digits) = digits
        .strip_prefix("0b")
        .or_else(|| digits.strip_prefix("0B"))
    {
        (2_u32, digits)
    } else if let Some(digits) = digits
        .strip_prefix("0o")
        .or_else(|| digits.strip_prefix("0O"))
    {
        (8_u32, digits)
    } else if let Some(digits) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        (16_u32, digits)
    } else {
        (10_u32, digits)
    };

    if (explicit_sign && radix != 10) || digits.is_empty() {
        return Err(bigint_string_diagnostic(span));
    }
    let mut decimal_digits = vec![0_u8];
    for ch in digits.chars() {
        let Some(digit) = ch.to_digit(radix) else {
            return Err(bigint_string_diagnostic(span));
        };
        decimal_mul_add(&mut decimal_digits, radix as u8, digit as u8);
    }
    trim_decimal_zeroes(&mut decimal_digits);
    let decimal = decimal_digits
        .iter()
        .map(|digit| char::from(b'0' + *digit))
        .collect::<String>();
    Ok(BigIntConst::from_decimal(sign, &decimal))
}

fn fold_bigint_static_abstract_equality(
    left: &ResolvedExpr,
    op: BinaryOp,
    right: &ResolvedExpr,
    span: Span,
) -> Result<Option<ResolvedExpr>, Diagnostic> {
    if !matches!(op, BinaryOp::EqualEqual | BinaryOp::BangEqual) {
        return Ok(None);
    }

    let compare = if let (Some(bigint), ResolvedExpr::String(value)) =
        (bigint_from_resolved(left), right)
    {
        Some((bigint, bigint_from_string_builtin(value, span).ok()))
    } else if let (ResolvedExpr::String(value), Some(bigint)) = (left, bigint_from_resolved(right))
    {
        Some((bigint, bigint_from_string_builtin(value, span).ok()))
    } else if let (Some(bigint), ResolvedExpr::Bool(value)) = (bigint_from_resolved(left), right) {
        Some((bigint, Some(bigint_from_bool(*value))))
    } else if let (ResolvedExpr::Bool(value), Some(bigint)) = (left, bigint_from_resolved(right)) {
        Some((bigint, Some(bigint_from_bool(*value))))
    } else if let (Some(bigint), ResolvedExpr::Number(value)) = (bigint_from_resolved(left), right)
    {
        Some((bigint, Some(bigint_from_i32(*value))))
    } else if let (ResolvedExpr::Number(value), Some(bigint)) = (left, bigint_from_resolved(right))
    {
        Some((bigint, Some(bigint_from_i32(*value))))
    } else {
        None
    };

    let Some((bigint, parsed_string)) = compare else {
        return Ok(None);
    };
    let equal = parsed_string.is_some_and(|string_bigint| string_bigint == bigint);
    Ok(Some(ResolvedExpr::Bool(if op == BinaryOp::BangEqual {
        !equal
    } else {
        equal
    })))
}

fn bigint_from_bool(value: bool) -> BigIntConst {
    if value {
        BigIntConst::from_decimal(1, "1")
    } else {
        BigIntConst::zero()
    }
}

fn bigint_string_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-262: BigInt(string) currently supports decimal, binary, octal, or hexadecimal integer string literals"
            .to_owned(),
        span: Some(span),
    }
}

fn bigint_builtin_unsupported_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-262: BigInt(...) currently supports string, boolean, integer number, or BigInt literal inputs in this builtin slice".to_owned(),
        span: Some(span),
    }
}

fn resolve_bigint_static_function_call(
    callee: &Expr,
    args: &[ResolvedExpr],
    span: Span,
) -> Result<Option<ResolvedExpr>, Diagnostic> {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return Ok(None);
    };
    let Expr::Ident { name, .. } = object.as_ref() else {
        return Ok(None);
    };
    if name != "BigInt" || !matches!(property.as_str(), "asIntN" | "asUintN") {
        return Ok(None);
    }

    let [bits_arg, value_arg] = args else {
        return Err(bigint_static_width_diagnostic(span));
    };
    let bits = bigint_static_width(bits_arg, span)?;
    let value = bigint_from_resolved(value_arg).ok_or_else(|| Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-262: BigInt.asIntN/asUintN currently require a BigInt literal value input"
            .to_owned(),
        span: Some(span),
    })?;
    let value = if property == "asIntN" {
        bigint_as_int_n(bits, value)
    } else {
        bigint_as_uint_n(bits, value)
    };
    Ok(Some(bigint_to_resolved(value)))
}

fn bigint_static_width(arg: &ResolvedExpr, span: Span) -> Result<u32, Diagnostic> {
    let ResolvedExpr::Number(bits) = arg else {
        return Err(bigint_static_width_diagnostic(span));
    };
    if !(0..=64).contains(bits) {
        return Err(bigint_static_width_diagnostic(span));
    }
    Ok(*bits as u32)
}

fn bigint_static_width_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message:
            "issue-262: BigInt.asIntN/asUintN currently support integer literal bit widths 0..64"
                .to_owned(),
        span: Some(span),
    }
}

fn bigint_as_uint_n(bits: u32, value: BigIntConst) -> BigIntConst {
    if bits == 0 || value.sign == 0 {
        return BigIntConst::zero();
    }
    let modulo = decimal_power_of_two(bits);
    let (_, remainder) = div_rem_abs(&value.digits, &modulo);
    if value.sign > 0 || remainder == [0] {
        return BigIntConst {
            sign: if remainder == [0] { 0 } else { 1 },
            digits: remainder,
        };
    }
    BigIntConst {
        sign: 1,
        digits: sub_abs(&modulo, &remainder),
    }
}

fn bigint_as_int_n(bits: u32, value: BigIntConst) -> BigIntConst {
    if bits == 0 {
        return BigIntConst::zero();
    }
    let unsigned = bigint_as_uint_n(bits, value);
    let threshold = decimal_power_of_two(bits - 1);
    if unsigned.sign == 0 || cmp_abs(&unsigned.digits, &threshold) == std::cmp::Ordering::Less {
        return unsigned;
    }
    BigIntConst {
        sign: -1,
        digits: sub_abs(&decimal_power_of_two(bits), &unsigned.digits),
    }
}

fn decimal_power_of_two(bits: u32) -> Vec<u8> {
    let mut digits = vec![1_u8];
    for _ in 0..bits {
        digits = mul_abs(&digits, &[2]);
    }
    digits
}

fn decimal_digits_to_u64(digits: &[u8]) -> Option<u64> {
    let mut magnitude = 0_u64;
    for digit in digits {
        magnitude = magnitude.checked_mul(10)?.checked_add(u64::from(*digit))?;
    }
    Some(magnitude)
}

fn bigint_arithmetic_op(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Modulo
    )
}

#[derive(Debug, Clone)]
struct BigIntStaticInfo {
    value: Option<BigIntConst>,
    helper_safe: bool,
    runtime_needed: bool,
}

impl BigIntStaticInfo {
    fn from_const(value: BigIntConst) -> Self {
        let helper_safe = value.fits_runtime_signed_i64();
        Self {
            value: Some(value),
            helper_safe,
            runtime_needed: false,
        }
    }
}

#[derive(Default)]
struct BigIntRuntimeGuard {
    locals: HashMap<String, BigIntStaticInfo>,
}

impl BigIntRuntimeGuard {
    fn visit_stmts(&mut self, stmts: &[Stmt]) -> Result<(), Diagnostic> {
        for stmt in stmts {
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), Diagnostic> {
        match stmt {
            Stmt::Let { name, expr, .. } | Stmt::Assign { name, expr, .. } => {
                let info = self.expr_bigint_info(expr)?;
                if let Some(info) = info {
                    self.locals.insert(name.clone(), info);
                } else {
                    self.locals.remove(name);
                }
                Ok(())
            }
            Stmt::Expr { expr, .. } | Stmt::Return { expr, .. } | Stmt::Throw { expr, .. } => {
                self.expr_bigint_info(expr).map(|_| ())
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.expr_bigint_info(condition)?;
                self.fork().visit_stmts(then_body)?;
                self.fork().visit_stmts(else_body)?;
                self.invalidate_assigned_in_stmts(then_body);
                self.invalidate_assigned_in_stmts(else_body);
                Ok(())
            }
            Stmt::While {
                condition, body, ..
            }
            | Stmt::DoWhile {
                condition, body, ..
            } => {
                self.expr_bigint_info(condition)?;
                self.fork().visit_stmts(body)?;
                self.invalidate_assigned_in_stmts(body);
                Ok(())
            }
            Stmt::Function { body, .. } => BigIntRuntimeGuard::default().visit_stmts(body),
            Stmt::ClassDecl { body, .. } => {
                for item in body {
                    if let Stmt::Function { body, .. } = item {
                        BigIntRuntimeGuard::default().visit_stmts(body)?;
                    }
                }
                Ok(())
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                self.fork().visit_stmts(try_block)?;
                if let Some(catch_block) = catch_block {
                    self.fork().visit_stmts(catch_block)?;
                }
                if let Some(finally_block) = finally_block {
                    self.fork().visit_stmts(finally_block)?;
                }
                self.invalidate_assigned_in_stmts(try_block);
                if let Some(catch_block) = catch_block {
                    self.invalidate_assigned_in_stmts(catch_block);
                }
                if let Some(finally_block) = finally_block {
                    self.invalidate_assigned_in_stmts(finally_block);
                }
                Ok(())
            }
            Stmt::Switch { expr, cases, .. } => {
                self.expr_bigint_info(expr)?;
                for (case_expr, body) in cases {
                    if let Some(case_expr) = case_expr {
                        self.expr_bigint_info(case_expr)?;
                    }
                    self.fork().visit_stmts(body)?;
                    self.invalidate_assigned_in_stmts(body);
                }
                Ok(())
            }
            Stmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                let mut loop_guard = self.fork();
                if let Some(init) = init {
                    loop_guard.visit_stmt(init)?;
                }
                if let Some(condition) = condition {
                    loop_guard.expr_bigint_info(condition)?;
                }
                if let Some(update) = update {
                    loop_guard.expr_bigint_info(update)?;
                }
                loop_guard.visit_stmts(body)?;
                if let Some(update) = update {
                    self.invalidate_assigned_in_expr(update);
                }
                self.invalidate_assigned_in_stmts(body);
                Ok(())
            }
            Stmt::ForIn {
                var, iter, body, ..
            }
            | Stmt::ForOf {
                var, iter, body, ..
            } => {
                self.expr_bigint_info(iter)?;
                let mut body_guard = self.fork();
                body_guard.locals.remove(var);
                body_guard.visit_stmts(body)?;
                self.locals.remove(var);
                self.invalidate_assigned_in_stmts(body);
                Ok(())
            }
            Stmt::Labeled { body, .. } => self.visit_stmt(body),
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
            | Stmt::ExportDecl { .. }
            | Stmt::ExportDefault { .. }
            | Stmt::Break { .. }
            | Stmt::Continue { .. } => Ok(()),
        }
    }

    fn fork(&self) -> Self {
        Self {
            locals: self.locals.clone(),
        }
    }

    fn invalidate_assigned_in_stmts(&mut self, stmts: &[Stmt]) {
        for name in assigned_names_in_stmts(stmts) {
            self.locals.remove(&name);
        }
    }

    fn invalidate_assigned_in_expr(&mut self, expr: &Expr) {
        for name in assigned_names_in_expr(expr) {
            self.locals.remove(&name);
        }
    }

    fn expr_bigint_info(&mut self, expr: &Expr) -> Result<Option<BigIntStaticInfo>, Diagnostic> {
        match expr {
            Expr::BigInt { raw, span } => {
                let resolved = parse_bigint_literal(raw, *span)?;
                Ok(bigint_from_resolved(&resolved).map(BigIntStaticInfo::from_const))
            }
            Expr::Ident { name, .. } => Ok(self.locals.get(name).cloned().map(|mut info| {
                info.runtime_needed = true;
                info
            })),
            Expr::Unary { op, expr, span } => {
                let info = self.expr_bigint_info(expr)?;
                if let Some(info) = info
                    && *op == UnaryOp::Negate
                {
                    let value = info.value.map(BigIntConst::negated);
                    let helper_safe = value
                        .as_ref()
                        .is_some_and(BigIntConst::fits_runtime_signed_i64);
                    if info.runtime_needed && !helper_safe {
                        return Err(bigint_dynamic_runtime_diagnostic(*span));
                    }
                    return Ok(Some(BigIntStaticInfo {
                        value,
                        helper_safe,
                        runtime_needed: info.runtime_needed,
                    }));
                }
                Ok(None)
            }
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                let left_info = self.expr_bigint_info(left)?;
                let right_info = self.expr_bigint_info(right)?;
                if left_info.is_none() && right_info.is_none() {
                    return Ok(None);
                }
                if !bigint_arithmetic_or_bitwise_op(*op) {
                    if bigint_equality_or_comparison_op(*op) {
                        let both_bigint = left_info.is_some() && right_info.is_some();
                        let strict_equality =
                            matches!(op, BinaryOp::StrictEqual | BinaryOp::StrictNotEqual);
                        let static_bigint_string_equality =
                            is_static_bigint_string_abstract_equality(
                                left,
                                left_info.as_ref(),
                                *op,
                                right,
                                right_info.as_ref(),
                            );
                        let static_bigint_boolean_equality =
                            is_static_bigint_boolean_abstract_equality(
                                left,
                                left_info.as_ref(),
                                *op,
                                right,
                                right_info.as_ref(),
                            );
                        let static_bigint_number_equality =
                            is_static_bigint_number_abstract_equality(
                                left,
                                left_info.as_ref(),
                                *op,
                                right,
                                right_info.as_ref(),
                            );
                        if both_bigint || strict_equality {
                            return Ok(None);
                        }
                        if static_bigint_string_equality
                            || static_bigint_boolean_equality
                            || static_bigint_number_equality
                        {
                            return Ok(None);
                        }
                        return Err(bigint_comparison_runtime_diagnostic(*span));
                    }
                    return Ok(None);
                }
                let (Some(left_info), Some(right_info)) = (left_info, right_info) else {
                    return Err(bigint_mixed_runtime_diagnostic(*span));
                };
                if !matches!(
                    op,
                    BinaryOp::Add
                        | BinaryOp::Subtract
                        | BinaryOp::Multiply
                        | BinaryOp::Divide
                        | BinaryOp::Modulo
                ) {
                    return Ok(None);
                }
                let runtime_needed = left_info.runtime_needed || right_info.runtime_needed;
                let value = match (left_info.value, right_info.value) {
                    (Some(left), Some(right)) => {
                        if runtime_needed
                            && matches!(op, BinaryOp::Divide | BinaryOp::Modulo)
                            && right.sign == 0
                        {
                            return Ok(Some(BigIntStaticInfo {
                                value: None,
                                helper_safe: left_info.helper_safe && right_info.helper_safe,
                                runtime_needed,
                            }));
                        }
                        let result = fold_bigint_binary(left, *op, right, *span)?;
                        if runtime_needed && !result.fits_runtime_signed_i64() {
                            return Err(bigint_dynamic_runtime_diagnostic(*span));
                        }
                        Some(result)
                    }
                    _ if runtime_needed => return Err(bigint_dynamic_runtime_diagnostic(*span)),
                    _ => None,
                };
                Ok(Some(BigIntStaticInfo {
                    value,
                    helper_safe: left_info.helper_safe && right_info.helper_safe,
                    runtime_needed,
                }))
            }
            Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
                self.expr_bigint_info(callee)?;
                for arg in args {
                    self.expr_bigint_info(arg)?;
                }
                Ok(None)
            }
            Expr::Member { object, .. }
            | Expr::OptionalMember { object, .. }
            | Expr::TypeOf { expr: object, .. }
            | Expr::Spread { expr: object, .. } => {
                self.expr_bigint_info(object)?;
                Ok(None)
            }
            Expr::Assign { name, expr, .. } | Expr::LogicalAssign { name, expr, .. } => {
                let info = self.expr_bigint_info(expr)?;
                if let Some(info) = &info {
                    self.locals.insert(name.clone(), info.clone());
                } else {
                    self.locals.remove(name);
                }
                Ok(info)
            }
            Expr::LogicalPropertyAssign {
                object_expr,
                computed_key,
                expr,
                ..
            } => {
                if let Some(object_expr) = object_expr {
                    self.expr_bigint_info(object_expr)?;
                }
                if let Some(computed_key) = computed_key {
                    self.expr_bigint_info(computed_key)?;
                }
                self.expr_bigint_info(expr)?;
                Ok(None)
            }
            Expr::Array { elements, .. } => {
                for element in elements {
                    self.expr_bigint_info(element)?;
                }
                Ok(None)
            }
            Expr::Object { props, .. } => {
                for (_, value) in props {
                    self.expr_bigint_info(value)?;
                }
                Ok(None)
            }
            Expr::Index { object, index, .. } | Expr::OptionalIndex { object, index, .. } => {
                self.expr_bigint_info(object)?;
                self.expr_bigint_info(index)?;
                Ok(None)
            }
            Expr::New { expr, args, .. } => {
                self.expr_bigint_info(expr)?;
                for arg in args {
                    self.expr_bigint_info(arg)?;
                }
                Ok(None)
            }
            Expr::InstanceOf {
                expr, type_expr, ..
            } => {
                self.expr_bigint_info(expr)?;
                self.expr_bigint_info(type_expr)?;
                Ok(None)
            }
            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.expr_bigint_info(condition)?;
                self.fork().expr_bigint_info(then_expr)?;
                self.fork().expr_bigint_info(else_expr)?;
                Ok(None)
            }
            Expr::PropertyAssign { object, value, .. } => {
                self.expr_bigint_info(object)?;
                self.expr_bigint_info(value)?;
                Ok(None)
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                ..
            } => {
                self.expr_bigint_info(object)?;
                self.expr_bigint_info(index)?;
                self.expr_bigint_info(value)?;
                Ok(None)
            }
            Expr::ArrowFn { body, .. } => BigIntRuntimeGuard::default().expr_bigint_info(body),
            Expr::Number { .. }
            | Expr::String { .. }
            | Expr::Bool { .. }
            | Expr::Null { .. }
            | Expr::Undefined { .. }
            | Expr::This { .. } => Ok(None),
        }
    }
}

fn bigint_arithmetic_or_bitwise_op(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Modulo
            | BinaryOp::Power
            | BinaryOp::BitwiseAnd
            | BinaryOp::BitwiseOr
            | BinaryOp::BitwiseXor
            | BinaryOp::LeftShift
            | BinaryOp::RightShift
            | BinaryOp::UnsignedRightShift
    )
}

fn bigint_dynamic_runtime_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-260: dynamic BigInt runtime arithmetic is limited to signed-i64-backed first-limb values in this slice".to_owned(),
        span: Some(span),
    }
}

fn bigint_mixed_runtime_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-260: mixed Number/BigInt arithmetic is not implemented in the dynamic BigInt runtime slice".to_owned(),
        span: Some(span),
    }
}

fn bigint_comparison_runtime_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-261: mixed BigInt abstract equality and relational comparison coercion is not implemented in this slice".to_owned(),
        span: Some(span),
    }
}

fn bigint_equality_or_comparison_op(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
            | BinaryOp::StrictEqual
            | BinaryOp::EqualEqual
            | BinaryOp::BangEqual
            | BinaryOp::StrictNotEqual
    )
}

fn is_static_bigint_string_abstract_equality(
    left: &Expr,
    left_info: Option<&BigIntStaticInfo>,
    op: BinaryOp,
    right: &Expr,
    right_info: Option<&BigIntStaticInfo>,
) -> bool {
    if !matches!(op, BinaryOp::EqualEqual | BinaryOp::BangEqual) {
        return false;
    }
    let left_static_bigint = left_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(right, Expr::String { .. })
    });
    let right_static_bigint = right_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(left, Expr::String { .. })
    });
    left_static_bigint || right_static_bigint
}

fn is_static_bigint_boolean_abstract_equality(
    left: &Expr,
    left_info: Option<&BigIntStaticInfo>,
    op: BinaryOp,
    right: &Expr,
    right_info: Option<&BigIntStaticInfo>,
) -> bool {
    if !matches!(op, BinaryOp::EqualEqual | BinaryOp::BangEqual) {
        return false;
    }
    let left_static_bigint = left_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(right, Expr::Bool { .. })
    });
    let right_static_bigint = right_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(left, Expr::Bool { .. })
    });
    left_static_bigint || right_static_bigint
}

fn is_static_bigint_number_abstract_equality(
    left: &Expr,
    left_info: Option<&BigIntStaticInfo>,
    op: BinaryOp,
    right: &Expr,
    right_info: Option<&BigIntStaticInfo>,
) -> bool {
    if !matches!(op, BinaryOp::EqualEqual | BinaryOp::BangEqual) {
        return false;
    }
    let left_static_bigint = left_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(right, Expr::Number { .. })
    });
    let right_static_bigint = right_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(left, Expr::Number { .. })
    });
    left_static_bigint || right_static_bigint
}

fn assigned_names_in_stmts(stmts: &[Stmt]) -> HashSet<String> {
    let mut names = HashSet::new();
    for stmt in stmts {
        collect_assigned_names_in_stmt(stmt, &mut names);
    }
    names
}

fn assigned_names_in_expr(expr: &Expr) -> HashSet<String> {
    let mut names = HashSet::new();
    collect_assigned_names_in_expr(expr, &mut names);
    names
}

fn collect_assigned_names_in_stmt(stmt: &Stmt, names: &mut HashSet<String>) {
    match stmt {
        Stmt::Let { name, .. } | Stmt::Assign { name, .. } => {
            names.insert(name.clone());
        }
        Stmt::Expr { expr, .. } | Stmt::Return { expr, .. } | Stmt::Throw { expr, .. } => {
            collect_assigned_names_in_expr(expr, names);
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            collect_assigned_names_in_expr(condition, names);
            collect_assigned_names_in_stmts(then_body, names);
            collect_assigned_names_in_stmts(else_body, names);
        }
        Stmt::While {
            condition, body, ..
        }
        | Stmt::DoWhile {
            condition, body, ..
        } => {
            collect_assigned_names_in_expr(condition, names);
            collect_assigned_names_in_stmts(body, names);
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            collect_assigned_names_in_stmts(try_block, names);
            if let Some(catch_block) = catch_block {
                collect_assigned_names_in_stmts(catch_block, names);
            }
            if let Some(finally_block) = finally_block {
                collect_assigned_names_in_stmts(finally_block, names);
            }
        }
        Stmt::Switch { expr, cases, .. } => {
            collect_assigned_names_in_expr(expr, names);
            for (case_expr, body) in cases {
                if let Some(case_expr) = case_expr {
                    collect_assigned_names_in_expr(case_expr, names);
                }
                collect_assigned_names_in_stmts(body, names);
            }
        }
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            if let Some(init) = init {
                collect_assigned_names_in_stmt(init, names);
            }
            if let Some(condition) = condition {
                collect_assigned_names_in_expr(condition, names);
            }
            if let Some(update) = update {
                collect_assigned_names_in_expr(update, names);
            }
            collect_assigned_names_in_stmts(body, names);
        }
        Stmt::ForIn {
            var, iter, body, ..
        }
        | Stmt::ForOf {
            var, iter, body, ..
        } => {
            names.insert(var.clone());
            collect_assigned_names_in_expr(iter, names);
            collect_assigned_names_in_stmts(body, names);
        }
        Stmt::Labeled { body, .. } => collect_assigned_names_in_stmt(body, names),
        Stmt::Function { .. }
        | Stmt::ClassDecl { .. }
        | Stmt::ImportSideEffect { .. }
        | Stmt::ImportNamed { .. }
        | Stmt::ImportDefault { .. }
        | Stmt::ImportDefaultNamed { .. }
        | Stmt::ImportNamespace { .. }
        | Stmt::ImportDefaultNamespace { .. }
        | Stmt::ExportNamed { .. }
        | Stmt::ExportNamedFrom { .. }
        | Stmt::ExportAllFrom { .. }
        | Stmt::ExportNamespaceFrom { .. }
        | Stmt::ExportDecl { .. }
        | Stmt::ExportDefault { .. }
        | Stmt::Break { .. }
        | Stmt::Continue { .. } => {}
    }
}

fn collect_assigned_names_in_stmts(stmts: &[Stmt], names: &mut HashSet<String>) {
    for stmt in stmts {
        collect_assigned_names_in_stmt(stmt, names);
    }
}

fn collect_assigned_names_in_expr(expr: &Expr, names: &mut HashSet<String>) {
    match expr {
        Expr::Assign { name, expr, .. } | Expr::LogicalAssign { name, expr, .. } => {
            names.insert(name.clone());
            collect_assigned_names_in_expr(expr, names);
        }
        Expr::Binary { left, right, .. }
        | Expr::Index {
            object: left,
            index: right,
            ..
        }
        | Expr::OptionalIndex {
            object: left,
            index: right,
            ..
        }
        | Expr::InstanceOf {
            expr: left,
            type_expr: right,
            ..
        } => {
            collect_assigned_names_in_expr(left, names);
            collect_assigned_names_in_expr(right, names);
        }
        Expr::Unary { expr, .. }
        | Expr::Member { object: expr, .. }
        | Expr::OptionalMember { object: expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Spread { expr, .. } => collect_assigned_names_in_expr(expr, names),
        Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
            collect_assigned_names_in_expr(callee, names);
            for arg in args {
                collect_assigned_names_in_expr(arg, names);
            }
        }
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => {
            if let Some(object_expr) = object_expr {
                collect_assigned_names_in_expr(object_expr, names);
            }
            if let Some(computed_key) = computed_key {
                collect_assigned_names_in_expr(computed_key, names);
            }
            collect_assigned_names_in_expr(expr, names);
        }
        Expr::Array { elements, .. } => {
            for element in elements {
                collect_assigned_names_in_expr(element, names);
            }
        }
        Expr::Object { props, .. } => {
            for (_, value) in props {
                collect_assigned_names_in_expr(value, names);
            }
        }
        Expr::New { expr, args, .. } => {
            collect_assigned_names_in_expr(expr, names);
            for arg in args {
                collect_assigned_names_in_expr(arg, names);
            }
        }
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_assigned_names_in_expr(condition, names);
            collect_assigned_names_in_expr(then_expr, names);
            collect_assigned_names_in_expr(else_expr, names);
        }
        Expr::PropertyAssign { object, value, .. } => {
            collect_assigned_names_in_expr(object, names);
            collect_assigned_names_in_expr(value, names);
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            collect_assigned_names_in_expr(object, names);
            collect_assigned_names_in_expr(index, names);
            collect_assigned_names_in_expr(value, names);
        }
        Expr::ArrowFn { .. }
        | Expr::Number { .. }
        | Expr::BigInt { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::Undefined { .. }
        | Expr::This { .. }
        | Expr::Ident { .. } => {}
    }
}

fn fold_bigint_binary(
    left: BigIntConst,
    op: BinaryOp,
    right: BigIntConst,
    span: Span,
) -> Result<BigIntConst, Diagnostic> {
    match op {
        BinaryOp::Add => Ok(bigint_add(left, right)),
        BinaryOp::Subtract => Ok(bigint_add(left, right.negated())),
        BinaryOp::Multiply => Ok(bigint_mul(left, right)),
        BinaryOp::Divide | BinaryOp::Modulo if right.sign == 0 => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-260: BigInt division by zero runtime throw is not implemented in this literal-folding slice"
                    .to_owned(),
            span: Some(span),
        }),
        BinaryOp::Divide => {
            let (quotient, _) = div_rem_abs(&left.digits, &right.digits);
            let sign = if quotient == [0] {
                0
            } else {
                left.sign * right.sign
            };
            Ok(BigIntConst {
                sign,
                digits: quotient,
            })
        }
        BinaryOp::Modulo => {
            let (_, remainder) = div_rem_abs(&left.digits, &right.digits);
            let sign = if remainder == [0] { 0 } else { left.sign };
            Ok(BigIntConst {
                sign,
                digits: remainder,
            })
        }
        _ => unreachable!("non-arithmetic BigInt operator reached literal fold"),
    }
}

fn bigint_add(left: BigIntConst, right: BigIntConst) -> BigIntConst {
    if left.sign == 0 {
        return right;
    }
    if right.sign == 0 {
        return left;
    }
    if left.sign == right.sign {
        return BigIntConst {
            sign: left.sign,
            digits: add_abs(&left.digits, &right.digits),
        };
    }
    match cmp_abs(&left.digits, &right.digits) {
        std::cmp::Ordering::Greater => BigIntConst {
            sign: left.sign,
            digits: sub_abs(&left.digits, &right.digits),
        },
        std::cmp::Ordering::Less => BigIntConst {
            sign: right.sign,
            digits: sub_abs(&right.digits, &left.digits),
        },
        std::cmp::Ordering::Equal => BigIntConst::zero(),
    }
}

fn bigint_mul(left: BigIntConst, right: BigIntConst) -> BigIntConst {
    if left.sign == 0 || right.sign == 0 {
        return BigIntConst::zero();
    }
    BigIntConst {
        sign: left.sign * right.sign,
        digits: mul_abs(&left.digits, &right.digits),
    }
}

fn cmp_abs(left: &[u8], right: &[u8]) -> std::cmp::Ordering {
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

fn add_abs(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut carry = 0_u8;
    let mut li = left.len();
    let mut ri = right.len();
    while li > 0 || ri > 0 || carry > 0 {
        let ld = if li > 0 {
            li -= 1;
            left[li]
        } else {
            0
        };
        let rd = if ri > 0 {
            ri -= 1;
            right[ri]
        } else {
            0
        };
        let sum = ld + rd + carry;
        out.push(sum % 10);
        carry = sum / 10;
    }
    out.reverse();
    out
}

fn sub_abs(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut borrow = 0_i8;
    let mut li = left.len();
    let mut ri = right.len();
    while li > 0 {
        li -= 1;
        let mut ld = left[li] as i8 - borrow;
        let rd = if ri > 0 {
            ri -= 1;
            right[ri] as i8
        } else {
            0
        };
        if ld < rd {
            ld += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push((ld - rd) as u8);
    }
    out.reverse();
    trim_decimal_zeroes(&mut out);
    out
}

fn mul_abs(left: &[u8], right: &[u8]) -> Vec<u8> {
    if left == [0] || right == [0] {
        return vec![0];
    }
    let mut out = vec![0_u16; left.len() + right.len()];
    for (li, ld) in left.iter().rev().enumerate() {
        for (ri, rd) in right.iter().rev().enumerate() {
            let idx = out.len() - 1 - li - ri;
            out[idx] += u16::from(*ld) * u16::from(*rd);
        }
    }
    for idx in (1..out.len()).rev() {
        let carry = out[idx] / 10;
        out[idx] %= 10;
        out[idx - 1] += carry;
    }
    let mut digits = out.into_iter().map(|digit| digit as u8).collect::<Vec<_>>();
    trim_decimal_zeroes(&mut digits);
    digits
}

fn div_rem_abs(left: &[u8], right: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut quotient = Vec::with_capacity(left.len());
    let mut remainder = vec![0_u8];
    for digit in left {
        if remainder == [0] {
            remainder[0] = *digit;
        } else {
            remainder.push(*digit);
        }
        trim_decimal_zeroes(&mut remainder);
        let mut q = 0_u8;
        while cmp_abs(&remainder, right) != std::cmp::Ordering::Less {
            remainder = sub_abs(&remainder, right);
            q += 1;
        }
        quotient.push(q);
    }
    trim_decimal_zeroes(&mut quotient);
    trim_decimal_zeroes(&mut remainder);
    (quotient, remainder)
}

fn bigint_unary_op_issue(op: UnaryOp) -> Option<&'static str> {
    match op {
        UnaryOp::Negate
        | UnaryOp::BitwiseNot
        | UnaryOp::Increment
        | UnaryOp::Decrement
        | UnaryOp::PreIncrement
        | UnaryOp::PreDecrement => Some(
            "issue-260: BigInt unary arithmetic and bitwise operators are tracked separately from literal runtime values",
        ),
        UnaryOp::Not | UnaryOp::TypeOf | UnaryOp::Delete | UnaryOp::Void => None,
    }
}

fn expr_contains_bigint(expr: &Expr) -> bool {
    match expr {
        Expr::BigInt { .. } => true,
        Expr::Unary { expr, .. } | Expr::TypeOf { expr, .. } | Expr::Spread { expr, .. } => {
            expr_contains_bigint(expr)
        }
        Expr::Binary { left, right, .. }
        | Expr::InstanceOf {
            expr: left,
            type_expr: right,
            ..
        }
        | Expr::Index {
            object: left,
            index: right,
            ..
        } => expr_contains_bigint(left) || expr_contains_bigint(right),
        Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
            expr_contains_bigint(callee) || args.iter().any(expr_contains_bigint)
        }
        Expr::Member { object, .. } | Expr::OptionalMember { object, .. } => {
            expr_contains_bigint(object)
        }
        Expr::OptionalIndex { object, index, .. } => {
            expr_contains_bigint(object) || expr_contains_bigint(index)
        }
        Expr::Assign { expr, .. } | Expr::LogicalAssign { expr, .. } => expr_contains_bigint(expr),
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => {
            object_expr.as_deref().is_some_and(expr_contains_bigint)
                || computed_key.as_deref().is_some_and(expr_contains_bigint)
                || expr_contains_bigint(expr)
        }
        Expr::Array { elements, .. } => elements.iter().any(expr_contains_bigint),
        Expr::Object { props, .. } => props.iter().any(|(_, value)| expr_contains_bigint(value)),
        Expr::New { args, .. } => args.iter().any(expr_contains_bigint),
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_bigint(condition)
                || expr_contains_bigint(then_expr)
                || expr_contains_bigint(else_expr)
        }
        Expr::ArrowFn { body, .. } => expr_contains_bigint(body),
        Expr::PropertyAssign { object, value, .. } => {
            expr_contains_bigint(object) || expr_contains_bigint(value)
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            expr_contains_bigint(object)
                || expr_contains_bigint(index)
                || expr_contains_bigint(value)
        }
        Expr::Number { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::This { .. }
        | Expr::Undefined { .. }
        | Expr::Ident { .. } => false,
    }
}

/// Check if an expression is a `require("...")` call.
fn is_require_call(callee: &Expr, args: &[Expr]) -> bool {
    let Expr::Ident { name, .. } = callee else {
        return false;
    };
    name == "require" && args.len() == 1 && matches!(&args[0], Expr::String { .. })
}

fn resolve_builtin_call(
    callee: &Expr,
    call_args: &[Expr],
) -> Result<Option<BuiltinId>, Diagnostic> {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return Ok(None);
    };

    if let Expr::Ident {
        name: object_name, ..
    } = object.as_ref()
    {
        if object_name == "BigInt" && matches!(property.as_str(), "asIntN" | "asUintN") {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-262: BigInt.asIntN/asUintN require literal bit width and BigInt value inputs in this builtin slice"
                        .to_owned(),
                span: span_of_expr(callee),
            });
        }
        if object_name == "console" {
            return if property == "log" {
                Ok(Some(BuiltinId::ConsoleLog))
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("console.{} is not supported in this milestone", property),
                    span: span_of_expr(callee),
                })
            };
        }
        if object_name == "Math" && property == "pow" {
            return Ok(Some(BuiltinId::MathPow));
        }
        if object_name == "process" {
            return if property == "exit" {
                Ok(Some(BuiltinId::ProcessExit))
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("process.{} is not supported in this milestone", property),
                    span: span_of_expr(callee),
                })
            };
        }
    }

    if let Some(builtin) = resolve_require_module_builtin(object.as_ref(), property, call_args)? {
        return Ok(Some(builtin));
    }

    Ok(None)
}

fn resolve_require_module_builtin(
    object: &Expr,
    property: &str,
    call_args: &[Expr],
) -> Result<Option<BuiltinId>, Diagnostic> {
    let Expr::Call {
        callee: require_callee,
        args: require_args,
        ..
    } = object
    else {
        return Ok(None);
    };
    let Expr::Ident {
        name: require_name, ..
    } = require_callee.as_ref()
    else {
        return Ok(None);
    };
    if require_name != "require" {
        return Ok(None);
    }
    let module_name = match require_args.as_slice() {
        [Expr::String { value, .. }] => value.as_str(),
        _ => return Ok(None),
    };

    let builtin = match (module_name, property) {
        ("fs", "readFileSync") => {
            if matches!(call_args.first(), Some(Expr::Number { .. })) {
                validate_read_stdin_utf8_args(call_args, object)?;
                BuiltinId::ReadStdinUtf8
            } else {
                BuiltinId::FsReadFileSync
            }
        }
        ("fs", "writeFileSync") => BuiltinId::FsWriteFileSync,
        ("fs", "appendFileSync") => BuiltinId::FsAppendFileSync,
        ("path", "join") => BuiltinId::PathJoin,
        ("path", "resolve") => BuiltinId::PathResolve,
        ("path", "basename") => BuiltinId::PathBasename,
        ("path", "dirname") => BuiltinId::PathDirname,
        ("crypto", "randomBytes") => BuiltinId::CryptoRandomBytes,
        ("fs", unsupported)
        | ("path", unsupported)
        | ("crypto", unsupported)
        | ("util", unsupported) => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "require(\"{}\").{} is not supported in this milestone",
                    module_name, unsupported
                ),
                span: span_of_expr(object),
            });
        }
        _ => return Ok(None),
    };

    if !matches!(builtin, BuiltinId::ReadStdinUtf8) && call_args.len() != builtin.expected_arity() {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "builtin call expects {} arguments, got {}",
                builtin.expected_arity(),
                call_args.len()
            ),
            span: span_of_expr(object),
        });
    }

    Ok(Some(builtin))
}

fn validate_read_stdin_utf8_args(args: &[Expr], callee: &Expr) -> Result<(), Diagnostic> {
    if args.len() != 2 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "require(\"fs\").readFileSync expects 2 arguments in this milestone, got {}",
                args.len()
            ),
            span: span_of_expr(callee),
        });
    }
    let fd_expr = &args[0];
    let encoding_expr = &args[1];

    match fd_expr {
        Expr::Number { value: 0, .. } => {}
        _ => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "require(\"fs\").readFileSync currently supports only fd 0 as first argument"
                        .to_owned(),
                span: span_of_expr(fd_expr),
            });
        }
    }

    match encoding_expr {
        Expr::String { value, .. } if value == "utf8" => Ok(()),
        _ => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "require(\"fs\").readFileSync currently supports only \"utf8\" encoding"
                .to_owned(),
            span: span_of_expr(encoding_expr),
        }),
    }
}

fn span_of_expr(expr: &Expr) -> Option<Span> {
    match expr {
        Expr::Number { span, .. }
        | Expr::BigInt { span, .. }
        | Expr::String { span, .. }
        | Expr::Bool { span, .. }
        | Expr::Null { span }
        | Expr::This { span }
        | Expr::Undefined { span }
        | Expr::Ident { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Member { span, .. }
        | Expr::OptionalMember { span, .. }
        | Expr::Call { span, .. }
        | Expr::OptionalCall { span, .. }
        | Expr::Assign { span, .. }
        | Expr::LogicalAssign { span, .. }
        | Expr::LogicalPropertyAssign { span, .. }
        | Expr::Array { span, .. }
        | Expr::Object { span, .. }
        | Expr::Index { span, .. }
        | Expr::OptionalIndex { span, .. }
        | Expr::New { span, .. }
        | Expr::TypeOf { span, .. }
        | Expr::InstanceOf { span, .. }
        | Expr::Ternary { span, .. }
        | Expr::ArrowFn { span, .. }
        | Expr::Spread { span, .. }
        | Expr::PropertyAssign { span, .. }
        | Expr::IndexAssign { span, .. } => Some(*span),
    }
}

fn validate_static_block_supported(block: &ClassStaticBlock) -> Result<(), Diagnostic> {
    for stmt in &block.body {
        validate_static_block_stmt(stmt)?;
    }
    Ok(())
}

fn validate_static_block_stmt(stmt: &Stmt) -> Result<(), Diagnostic> {
    match stmt {
        Stmt::Return { span, .. } => Err(static_block_unsupported(
            "return statements are not valid in class static blocks",
            *span,
        )),
        Stmt::Let { expr, .. }
        | Stmt::Assign { expr, .. }
        | Stmt::Expr { expr, .. }
        | Stmt::Throw { expr, .. } => validate_static_block_expr(expr),
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            validate_static_block_expr(condition)?;
            validate_static_block_stmts(then_body)?;
            validate_static_block_stmts(else_body)
        }
        Stmt::While {
            condition, body, ..
        }
        | Stmt::DoWhile {
            condition, body, ..
        } => {
            validate_static_block_expr(condition)?;
            validate_static_block_stmts(body)
        }
        Stmt::Function { params, body, .. } => {
            for (_, default, _) in params {
                if let Some(default) = default {
                    validate_static_block_expr(default)?;
                }
            }
            validate_static_block_stmts(body)
        }
        Stmt::ClassDecl {
            extends,
            body,
            static_blocks,
            ..
        } => {
            if let Some(extends) = extends {
                validate_static_block_expr(extends)?;
            }
            validate_static_block_stmts(body)?;
            for block in static_blocks {
                validate_static_block_supported(block)?;
            }
            Ok(())
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            validate_static_block_stmts(try_block)?;
            if let Some(block) = catch_block {
                validate_static_block_stmts(block)?;
            }
            if let Some(block) = finally_block {
                validate_static_block_stmts(block)?;
            }
            Ok(())
        }
        Stmt::Switch { expr, cases, .. } => {
            validate_static_block_expr(expr)?;
            for (case, body) in cases {
                if let Some(case) = case {
                    validate_static_block_expr(case)?;
                }
                validate_static_block_stmts(body)?;
            }
            Ok(())
        }
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            if let Some(init) = init {
                validate_static_block_stmt(init)?;
            }
            if let Some(condition) = condition {
                validate_static_block_expr(condition)?;
            }
            if let Some(update) = update {
                validate_static_block_expr(update)?;
            }
            validate_static_block_stmts(body)
        }
        Stmt::ForIn { iter, body, .. } | Stmt::ForOf { iter, body, .. } => {
            validate_static_block_expr(iter)?;
            validate_static_block_stmts(body)
        }
        Stmt::Labeled { body, .. } => validate_static_block_stmt(body),
        Stmt::Break { .. } | Stmt::Continue { .. } => Ok(()),
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
        | Stmt::ExportDefault { span, .. } => Err(static_block_unsupported(
            "module declarations inside class static blocks are not supported",
            *span,
        )),
    }
}

fn validate_static_block_stmts(stmts: &[Stmt]) -> Result<(), Diagnostic> {
    for stmt in stmts {
        validate_static_block_stmt(stmt)?;
    }
    Ok(())
}

fn validate_static_block_expr(expr: &Expr) -> Result<(), Diagnostic> {
    match expr {
        Expr::This { span } => Err(static_block_unsupported(
            "`this` in class static blocks needs constructor-object binding support",
            *span,
        )),
        Expr::Ident { name, span } if name == "super" => Err(static_block_unsupported(
            "`super` in class static blocks is not supported",
            *span,
        )),
        Expr::Unary { expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Assign { expr, .. }
        | Expr::Spread { expr, .. } => validate_static_block_expr(expr),
        Expr::Binary {
            left: expr_left,
            right: expr_right,
            ..
        }
        | Expr::Index {
            object: expr_left,
            index: expr_right,
            ..
        }
        | Expr::PropertyAssign {
            object: expr_left,
            value: expr_right,
            ..
        } => {
            validate_static_block_expr(expr_left)?;
            validate_static_block_expr(expr_right)
        }
        Expr::InstanceOf {
            expr, type_expr, ..
        } => {
            validate_static_block_expr(expr)?;
            validate_static_block_expr(type_expr)
        }
        Expr::LogicalAssign { name: _, expr, .. } => validate_static_block_expr(expr),
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => {
            if let Some(object) = object_expr {
                validate_static_block_expr(object)?;
            }
            if let Some(key) = computed_key {
                validate_static_block_expr(key)?;
            }
            validate_static_block_expr(expr)
        }
        Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
            validate_static_block_expr(callee)?;
            validate_static_block_exprs(args)
        }
        Expr::Member { object, .. } | Expr::OptionalMember { object, .. } => {
            validate_static_block_expr(object)
        }
        Expr::OptionalIndex { object, index, .. } => {
            validate_static_block_expr(object)?;
            validate_static_block_expr(index)
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            validate_static_block_expr(object)?;
            validate_static_block_expr(index)?;
            validate_static_block_expr(value)
        }
        Expr::Array { elements, .. } => validate_static_block_exprs(elements),
        Expr::Object { props, .. } => {
            for (_, value) in props {
                validate_static_block_expr(value)?;
            }
            Ok(())
        }
        Expr::New { expr, args, .. } => {
            validate_static_block_expr(expr)?;
            validate_static_block_exprs(args)
        }
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            validate_static_block_expr(condition)?;
            validate_static_block_expr(then_expr)?;
            validate_static_block_expr(else_expr)
        }
        Expr::ArrowFn { body, .. } => validate_static_block_expr(body),
        Expr::Number { .. }
        | Expr::BigInt { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::Undefined { .. }
        | Expr::Ident { .. } => Ok(()),
    }
}

fn validate_static_block_exprs(exprs: &[Expr]) -> Result<(), Diagnostic> {
    for expr in exprs {
        validate_static_block_expr(expr)?;
    }
    Ok(())
}

fn static_block_unsupported(detail: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("issue-254: {detail}"),
        span: Some(span),
    }
}
