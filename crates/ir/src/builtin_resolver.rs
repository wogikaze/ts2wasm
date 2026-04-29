use std::collections::{HashMap, HashSet};

use ts2wasm_frontend::{
    BinaryOp, ClassPrivateElement, ClassStaticBlock, DiagCode, Diagnostic, Expr, Span, Stmt,
    UnaryOp,
};

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
                let then_body = self.fork().fold_stmts(then_body);
                let else_body = self.fork().fold_stmts(else_body);
                self.invalidate_assigned_in_stmts(then_body.as_slice());
                self.invalidate_assigned_in_stmts(else_body.as_slice());
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
                span,
            } => Stmt::Function {
                name: name.clone(),
                params: params.clone(),
                body: BigIntStaticBuiltinFolder::default().fold_stmts(body),
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
                let body = body_folder.fold_stmts(body);
                self.locals.remove(var);
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
                let body = body_folder.fold_stmts(body);
                self.locals.remove(var);
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
            Expr::Unary { op, expr, span } => Expr::Unary {
                op: *op,
                expr: Box::new(self.fold_expr(expr)),
                span: *span,
            },
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => Expr::Binary {
                left: Box::new(self.fold_expr(left)),
                op: *op,
                right: Box::new(self.fold_expr(right)),
                span: *span,
            },
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
}

fn static_bigint_builtin_const_expr(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::Number { .. } | Expr::BigInt { .. } => Some(expr.clone()),
        _ => None,
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
                        reject_class_method_outer_local_references(
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

fn collect_top_level_bindings(program: &[Stmt]) -> Result<HashSet<String>, Diagnostic> {
    let mut bindings = HashSet::new();
    for stmt in program {
        collect_stmt_declared_bindings(stmt, &mut bindings)?;
    }
    Ok(bindings)
}

fn collect_stmt_declared_bindings(
    stmt: &Stmt,
    bindings: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    match stmt {
        Stmt::Let { name, span, .. } => {
            collect_binding_names(name, Some(*span), bindings)?;
        }
        Stmt::Function { name, .. } | Stmt::ClassDecl { name, .. } => {
            bindings.insert(name.clone());
        }
        Stmt::TryCatch {
            catch_param,
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            if let Some(param) = catch_param {
                bindings.insert(param.clone());
            }
            collect_stmt_declared_bindings_in_block(try_block, bindings)?;
            if let Some(block) = catch_block {
                collect_stmt_declared_bindings_in_block(block, bindings)?;
            }
            if let Some(block) = finally_block {
                collect_stmt_declared_bindings_in_block(block, bindings)?;
            }
        }
        Stmt::If {
            then_body,
            else_body,
            ..
        } => {
            collect_stmt_declared_bindings_in_block(then_body, bindings)?;
            collect_stmt_declared_bindings_in_block(else_body, bindings)?;
        }
        Stmt::While { body, .. }
        | Stmt::DoWhile { body, .. }
        | Stmt::For { body, .. }
        | Stmt::ForIn { body, .. }
        | Stmt::ForOf { body, .. } => {
            collect_stmt_declared_bindings_in_block(body, bindings)?;
            if let Stmt::ForIn { var, .. } | Stmt::ForOf { var, .. } = stmt {
                bindings.insert(var.clone());
            }
        }
        Stmt::Switch { cases, .. } => {
            for (_, body) in cases {
                collect_stmt_declared_bindings_in_block(body, bindings)?;
            }
        }
        Stmt::Labeled { body, .. } => collect_stmt_declared_bindings(body, bindings)?,
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
        | Stmt::Assign { .. }
        | Stmt::Expr { .. }
        | Stmt::Return { .. }
        | Stmt::Throw { .. }
        | Stmt::Break { .. }
        | Stmt::Continue { .. } => {}
    }
    Ok(())
}

fn collect_stmt_declared_bindings_in_block(
    block: &[Stmt],
    bindings: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    for stmt in block {
        collect_stmt_declared_bindings(stmt, bindings)?;
    }
    Ok(())
}

fn collect_binding_names(
    name: &str,
    span: Option<Span>,
    bindings: &mut HashSet<String>,
) -> Result<(), Diagnostic> {
    if let Some(pattern) = parse_binding_pattern(name, span)? {
        for binding_name in pattern.names() {
            bindings.insert(binding_name.to_owned());
        }
    } else {
        bindings.insert(name.to_owned());
    }
    Ok(())
}

fn reject_class_method_outer_local_references(
    class_name: &str,
    method_name: &str,
    params: &[(String, Option<Expr>, bool)],
    body: &[Stmt],
    outer_bindings: &HashSet<String>,
) -> Result<(), Diagnostic> {
    if outer_bindings.is_empty() {
        return Ok(());
    }

    let mut method_locals = HashSet::new();
    method_locals.insert(class_name.to_owned());
    for (param, default, _) in params {
        collect_binding_names(param, default.as_ref().map(Expr::span), &mut method_locals)?;
    }
    collect_stmt_declared_bindings_in_block(body, &mut method_locals)?;

    if let Some((name, span)) =
        first_outer_local_reference_in_stmts(body, outer_bindings, &method_locals)
    {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-289: class method `{method_name}` references outer local `{name}`; class-method lexical captures require environment support"
            ),
            span: Some(span),
        });
    }

    Ok(())
}

fn first_outer_local_reference_in_stmts(
    stmts: &[Stmt],
    outer_bindings: &HashSet<String>,
    method_locals: &HashSet<String>,
) -> Option<(String, Span)> {
    stmts
        .iter()
        .find_map(|stmt| first_outer_local_reference_in_stmt(stmt, outer_bindings, method_locals))
}

fn first_outer_local_reference_in_stmt(
    stmt: &Stmt,
    outer_bindings: &HashSet<String>,
    method_locals: &HashSet<String>,
) -> Option<(String, Span)> {
    match stmt {
        Stmt::Let { expr, .. } | Stmt::Return { expr, .. } | Stmt::Throw { expr, .. } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals)
        }
        Stmt::Assign { name, expr, span } => {
            reference_if_outer(name, *span, outer_bindings, method_locals).or_else(|| {
                first_outer_local_reference_in_expr(expr, outer_bindings, method_locals)
            })
        }
        Stmt::Expr { expr, .. } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals)
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => first_outer_local_reference_in_expr(condition, outer_bindings, method_locals)
            .or_else(|| {
                first_outer_local_reference_in_stmts(then_body, outer_bindings, method_locals)
            })
            .or_else(|| {
                first_outer_local_reference_in_stmts(else_body, outer_bindings, method_locals)
            }),
        Stmt::While {
            condition, body, ..
        }
        | Stmt::DoWhile {
            body, condition, ..
        } => first_outer_local_reference_in_expr(condition, outer_bindings, method_locals)
            .or_else(|| first_outer_local_reference_in_stmts(body, outer_bindings, method_locals)),
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => init
            .as_deref()
            .and_then(|stmt| {
                first_outer_local_reference_in_stmt(stmt, outer_bindings, method_locals)
            })
            .or_else(|| {
                condition.as_ref().and_then(|expr| {
                    first_outer_local_reference_in_expr(expr, outer_bindings, method_locals)
                })
            })
            .or_else(|| {
                update.as_ref().and_then(|expr| {
                    first_outer_local_reference_in_expr(expr, outer_bindings, method_locals)
                })
            })
            .or_else(|| first_outer_local_reference_in_stmts(body, outer_bindings, method_locals)),
        Stmt::ForIn { iter, body, .. } | Stmt::ForOf { iter, body, .. } => {
            first_outer_local_reference_in_expr(iter, outer_bindings, method_locals).or_else(|| {
                first_outer_local_reference_in_stmts(body, outer_bindings, method_locals)
            })
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => first_outer_local_reference_in_stmts(try_block, outer_bindings, method_locals)
            .or_else(|| {
                catch_block.as_ref().and_then(|block| {
                    first_outer_local_reference_in_stmts(block, outer_bindings, method_locals)
                })
            })
            .or_else(|| {
                finally_block.as_ref().and_then(|block| {
                    first_outer_local_reference_in_stmts(block, outer_bindings, method_locals)
                })
            }),
        Stmt::Switch { expr, cases, .. } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals).or_else(|| {
                cases.iter().find_map(|(case_expr, body)| {
                    case_expr
                        .as_ref()
                        .and_then(|expr| {
                            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals)
                        })
                        .or_else(|| {
                            first_outer_local_reference_in_stmts(
                                body,
                                outer_bindings,
                                method_locals,
                            )
                        })
                })
            })
        }
        Stmt::Labeled { body, .. } => {
            first_outer_local_reference_in_stmt(body, outer_bindings, method_locals)
        }
        Stmt::Function { .. } | Stmt::ClassDecl { .. } => None,
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
        | Stmt::Continue { .. } => None,
    }
}

fn first_outer_local_reference_in_expr(
    expr: &Expr,
    outer_bindings: &HashSet<String>,
    method_locals: &HashSet<String>,
) -> Option<(String, Span)> {
    match expr {
        Expr::Ident { name, span } => {
            reference_if_outer(name, *span, outer_bindings, method_locals)
        }
        Expr::Unary { expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Await { expr, .. }
        | Expr::Spread { expr, .. } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals)
        }
        Expr::Binary { left, right, .. } => {
            first_outer_local_reference_in_expr(left, outer_bindings, method_locals).or_else(|| {
                first_outer_local_reference_in_expr(right, outer_bindings, method_locals)
            })
        }
        Expr::Member { object, .. } | Expr::OptionalMember { object, .. } => {
            first_outer_local_reference_in_expr(object, outer_bindings, method_locals)
        }
        Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
            first_outer_local_reference_in_expr(callee, outer_bindings, method_locals).or_else(
                || {
                    args.iter().find_map(|arg| {
                        first_outer_local_reference_in_expr(arg, outer_bindings, method_locals)
                    })
                },
            )
        }
        Expr::Assign { name, expr, span }
        | Expr::LogicalAssign {
            name, expr, span, ..
        } => reference_if_outer(name, *span, outer_bindings, method_locals)
            .or_else(|| first_outer_local_reference_in_expr(expr, outer_bindings, method_locals)),
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => object_expr
            .as_deref()
            .and_then(|object| {
                first_outer_local_reference_in_expr(object, outer_bindings, method_locals)
            })
            .or_else(|| {
                computed_key.as_deref().and_then(|key| {
                    first_outer_local_reference_in_expr(key, outer_bindings, method_locals)
                })
            })
            .or_else(|| first_outer_local_reference_in_expr(expr, outer_bindings, method_locals)),
        Expr::Array { elements, .. } => elements.iter().find_map(|element| {
            first_outer_local_reference_in_expr(element, outer_bindings, method_locals)
        }),
        Expr::Object { props, .. } => props.iter().find_map(|(_, value)| {
            first_outer_local_reference_in_expr(value, outer_bindings, method_locals)
        }),
        Expr::Index { object, index, .. } | Expr::OptionalIndex { object, index, .. } => {
            first_outer_local_reference_in_expr(object, outer_bindings, method_locals).or_else(
                || first_outer_local_reference_in_expr(index, outer_bindings, method_locals),
            )
        }
        Expr::New { expr, args, .. } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals).or_else(|| {
                args.iter().find_map(|arg| {
                    first_outer_local_reference_in_expr(arg, outer_bindings, method_locals)
                })
            })
        }
        Expr::InstanceOf {
            expr, type_expr, ..
        } => {
            first_outer_local_reference_in_expr(expr, outer_bindings, method_locals).or_else(|| {
                first_outer_local_reference_in_expr(type_expr, outer_bindings, method_locals)
            })
        }
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => first_outer_local_reference_in_expr(condition, outer_bindings, method_locals)
            .or_else(|| {
                first_outer_local_reference_in_expr(then_expr, outer_bindings, method_locals)
            })
            .or_else(|| {
                first_outer_local_reference_in_expr(else_expr, outer_bindings, method_locals)
            }),
        Expr::PropertyAssign { object, value, .. } => {
            first_outer_local_reference_in_expr(object, outer_bindings, method_locals).or_else(
                || first_outer_local_reference_in_expr(value, outer_bindings, method_locals),
            )
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => first_outer_local_reference_in_expr(object, outer_bindings, method_locals)
            .or_else(|| first_outer_local_reference_in_expr(index, outer_bindings, method_locals))
            .or_else(|| first_outer_local_reference_in_expr(value, outer_bindings, method_locals)),
        Expr::ArrowFn { .. } | Expr::FunctionExpr { .. } => None,
        Expr::Number { .. }
        | Expr::BigInt { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::Undefined { .. }
        | Expr::This { .. } => None,
    }
}

fn reference_if_outer(
    name: &str,
    span: Span,
    outer_bindings: &HashSet<String>,
    method_locals: &HashSet<String>,
) -> Option<(String, Span)> {
    if outer_bindings.contains(name) && !method_locals.contains(name) {
        Some((name.to_owned(), span))
    } else {
        None
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
                    if matches!(
                        op,
                        BinaryOp::Add
                            | BinaryOp::Subtract
                            | BinaryOp::Multiply
                            | BinaryOp::Divide
                            | BinaryOp::Modulo
                    ) {
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

fn static_number_bigint_const(expr: &Expr) -> Option<BigIntConst> {
    match expr {
        Expr::Number { value, .. } => Some(bigint_from_i32(*value)),
        Expr::Unary { op, expr, .. } if *op == UnaryOp::Negate => {
            let Expr::Number { value, .. } = expr.as_ref() else {
                return None;
            };
            Some(bigint_from_i64(-i64::from(*value)))
        }
        _ => None,
    }
}

fn resolved_number_bigint_const(expr: &ResolvedExpr) -> Option<BigIntConst> {
    match expr {
        ResolvedExpr::Number(value) => Some(bigint_from_i32(*value)),
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Negate => {
            let ResolvedExpr::Number(value) = expr.as_ref() else {
                return None;
            };
            Some(bigint_from_i64(-i64::from(*value)))
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
        ResolvedExpr::Null | ResolvedExpr::Undefined => {
            return Err(bigint_builtin_unsupported_diagnostic(span));
        }
        _ => {
            return Ok(ResolvedExpr::MethodCall {
                object: Box::new(ResolvedExpr::Ident(BIGINT_RUNTIME_OBJECT.to_owned())),
                method: BIGINT_FROM_VALUE_RUNTIME_CALL.to_owned(),
                args: args.to_vec(),
                span,
            });
        }
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
    if !matches!(
        op,
        BinaryOp::EqualEqual
            | BinaryOp::BangEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
    ) {
        return Ok(None);
    }

    if let Some(ordering) = fold_static_bigint_number_ordering(left, right) {
        let result = match op {
            BinaryOp::EqualEqual => ordering == std::cmp::Ordering::Equal,
            BinaryOp::BangEqual => ordering != std::cmp::Ordering::Equal,
            BinaryOp::Less => ordering == std::cmp::Ordering::Less,
            BinaryOp::LessEqual => {
                matches!(
                    ordering,
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal
                )
            }
            BinaryOp::Greater => ordering == std::cmp::Ordering::Greater,
            BinaryOp::GreaterEqual => {
                matches!(
                    ordering,
                    std::cmp::Ordering::Greater | std::cmp::Ordering::Equal
                )
            }
            _ => unreachable!("guarded BigInt/Number comparison op"),
        };
        return Ok(Some(ResolvedExpr::Bool(result)));
    }

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
    } else if bigint_from_resolved(left).is_some()
        && matches!(right, ResolvedExpr::Null | ResolvedExpr::Undefined)
    {
        Some((BigIntConst::zero(), None))
    } else if matches!(left, ResolvedExpr::Null | ResolvedExpr::Undefined)
        && bigint_from_resolved(right).is_some()
    {
        Some((BigIntConst::zero(), None))
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

fn fold_static_bigint_number_ordering(
    left: &ResolvedExpr,
    right: &ResolvedExpr,
) -> Option<std::cmp::Ordering> {
    if let (Some(bigint), Some(number)) = (
        bigint_from_resolved(left),
        resolved_number_bigint_const(right),
    ) {
        Some(bigint_cmp(&bigint, &number))
    } else if let (Some(number), Some(bigint)) = (
        resolved_number_bigint_const(left),
        bigint_from_resolved(right),
    ) {
        Some(bigint_cmp(&number, &bigint))
    } else {
        None
    }
}

fn bigint_cmp(left: &BigIntConst, right: &BigIntConst) -> std::cmp::Ordering {
    left.sign
        .cmp(&right.sign)
        .then_with(|| match left.sign.cmp(&0) {
            std::cmp::Ordering::Less => cmp_abs(&right.digits, &left.digits),
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater => cmp_abs(&left.digits, &right.digits),
        })
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
        message: "issue-280: BigInt(string) currently supports decimal, binary, octal, or hexadecimal integer string literals"
            .to_owned(),
        span: Some(span),
    }
}

fn bigint_builtin_unsupported_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-280: BigInt(...) currently supports static string/boolean/integer number inputs and dynamic boolean/integer number/BigInt inputs in this builtin slice".to_owned(),
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
    let static_bits = match bits_arg {
        ResolvedExpr::Number(_) => Some(bigint_static_width(bits_arg, span)?),
        ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::BigIntLiteral { .. } => return Err(bigint_static_width_diagnostic(span)),
        _ => None,
    };
    let static_value = bigint_from_resolved(value_arg);
    if let (Some(bits), Some(value)) = (static_bits, static_value) {
        let value = if property == "asIntN" {
            bigint_as_int_n(bits, value)
        } else {
            bigint_as_uint_n(bits, value)
        };
        return Ok(Some(bigint_to_resolved(value)));
    }
    if matches!(
        value_arg,
        ResolvedExpr::Number(_)
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined
    ) {
        return Err(bigint_as_value_diagnostic(span));
    }
    Ok(Some(ResolvedExpr::MethodCall {
        object: Box::new(ResolvedExpr::Ident(BIGINT_RUNTIME_OBJECT.to_owned())),
        method: if property == "asIntN" {
            BIGINT_AS_INT_N_RUNTIME_CALL
        } else {
            BIGINT_AS_UINT_N_RUNTIME_CALL
        }
        .to_owned(),
        args: args.to_vec(),
        span,
    }))
}

fn bigint_runtime_call_name(name: &str) -> Option<&'static str> {
    match name {
        BIGINT_FROM_VALUE_RUNTIME_CALL => Some("BigIntFromValue"),
        BIGINT_AS_INT_N_RUNTIME_CALL => Some("BigIntAsIntN"),
        BIGINT_AS_UINT_N_RUNTIME_CALL => Some("BigIntAsUintN"),
        _ => None,
    }
}

pub(crate) fn bigint_runtime_fn_name(name: &str) -> Option<&'static str> {
    bigint_runtime_call_name(name)
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
            "issue-280: BigInt.asIntN/asUintN currently support integer literal bit widths 0..64"
                .to_owned(),
        span: Some(span),
    }
}

fn bigint_as_value_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message:
            "issue-280: BigInt.asIntN/asUintN currently require a supported BigInt value input"
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
    string_locals: HashSet<String>,
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
                if self.expr_is_definitely_string(expr) {
                    self.string_locals.insert(name.clone());
                } else {
                    self.string_locals.remove(name);
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
            string_locals: self.string_locals.clone(),
        }
    }

    fn invalidate_assigned_in_stmts(&mut self, stmts: &[Stmt]) {
        for name in assigned_names_in_stmts(stmts) {
            self.locals.remove(&name);
            self.string_locals.remove(&name);
        }
    }

    fn invalidate_assigned_in_expr(&mut self, expr: &Expr) {
        for name in assigned_names_in_expr(expr) {
            self.locals.remove(&name);
            self.string_locals.remove(&name);
        }
    }

    fn expr_is_definitely_string(&self, expr: &Expr) -> bool {
        match expr {
            Expr::String { .. } => true,
            Expr::Ident { name, .. } => self.string_locals.contains(name),
            Expr::Binary {
                left,
                op: BinaryOp::Add,
                right,
                ..
            } => self.expr_is_definitely_string(left) || self.expr_is_definitely_string(right),
            _ => false,
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
                        let static_bigint_number_comparison = is_static_bigint_number_comparison(
                            left,
                            left_info.as_ref(),
                            *op,
                            right,
                            right_info.as_ref(),
                        );
                        let static_bigint_nullish_equality =
                            is_static_bigint_nullish_abstract_equality(
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
                            || static_bigint_number_comparison
                            || static_bigint_nullish_equality
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
            Expr::Call { callee, args, span } if matches!(callee.as_ref(), Expr::Ident { name, .. } if name == "BigInt") =>
            {
                let [arg] = args.as_slice() else {
                    return Err(bigint_builtin_unsupported_diagnostic(*span));
                };
                self.expr_bigint_info(arg)?;
                let static_supported_arg = match arg {
                    Expr::String { .. }
                    | Expr::Bool { .. }
                    | Expr::Number { .. }
                    | Expr::BigInt { .. } => true,
                    Expr::Unary {
                        op: UnaryOp::Negate,
                        expr,
                        ..
                    } => matches!(expr.as_ref(), Expr::Number { .. }),
                    _ => false,
                };
                if static_supported_arg {
                    return Ok(None);
                }
                Ok(Some(BigIntStaticInfo {
                    value: None,
                    helper_safe: true,
                    runtime_needed: true,
                }))
            }
            Expr::Call { callee, args, span }
                if is_bigint_static_builtin_callee(callee.as_ref()) =>
            {
                let [bits, value] = args.as_slice() else {
                    return Err(bigint_static_width_diagnostic(*span));
                };
                let static_bits = match bits {
                    Expr::Number { value, .. } if (0..=64).contains(value) => true,
                    Expr::Number { .. }
                    | Expr::String { .. }
                    | Expr::Bool { .. }
                    | Expr::Null { .. }
                    | Expr::Undefined { .. }
                    | Expr::BigInt { .. } => return Err(bigint_static_width_diagnostic(*span)),
                    _ => {
                        self.expr_bigint_info(bits)?;
                        false
                    }
                };
                let Some(value_info) = self.expr_bigint_info(value)? else {
                    return Err(bigint_as_value_diagnostic(*span));
                };
                let runtime_needed = !static_bits || value_info.runtime_needed;
                if runtime_needed && !value_info.helper_safe {
                    return Err(bigint_as_value_diagnostic(*span));
                }
                Ok(Some(BigIntStaticInfo {
                    value: None,
                    helper_safe: true,
                    runtime_needed: true,
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
            | Expr::Await { expr: object, .. }
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
                if self.expr_is_definitely_string(expr) {
                    self.string_locals.insert(name.clone());
                } else {
                    self.string_locals.remove(name);
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
            Expr::FunctionExpr { body, .. } => {
                BigIntRuntimeGuard::default().visit_stmts(body)?;
                Ok(None)
            }
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
        message: "issue-282: mixed BigInt abstract equality and relational comparison coercion is not implemented in this runtime coercion slice".to_owned(),
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

fn is_static_bigint_number_comparison(
    left: &Expr,
    left_info: Option<&BigIntStaticInfo>,
    op: BinaryOp,
    right: &Expr,
    right_info: Option<&BigIntStaticInfo>,
) -> bool {
    if !matches!(
        op,
        BinaryOp::EqualEqual
            | BinaryOp::BangEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
    ) {
        return false;
    }
    let left_static_bigint = left_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && static_number_bigint_const(right).is_some()
    });
    let right_static_bigint = right_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && static_number_bigint_const(left).is_some()
    });
    left_static_bigint || right_static_bigint
}

fn is_static_bigint_nullish_abstract_equality(
    left: &Expr,
    left_info: Option<&BigIntStaticInfo>,
    op: BinaryOp,
    right: &Expr,
    right_info: Option<&BigIntStaticInfo>,
) -> bool {
    if !matches!(op, BinaryOp::EqualEqual | BinaryOp::BangEqual) {
        return false;
    }
    let right_nullish = matches!(right, Expr::Null { .. } | Expr::Undefined { .. });
    let left_nullish = matches!(left, Expr::Null { .. } | Expr::Undefined { .. });
    let left_static_bigint =
        left_info.is_some_and(|info| !info.runtime_needed && info.value.is_some() && right_nullish);
    let right_static_bigint =
        right_info.is_some_and(|info| !info.runtime_needed && info.value.is_some() && left_nullish);
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
        | Expr::Await { expr, .. }
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
        | Expr::FunctionExpr { .. }
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
        | UnaryOp::Plus
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
        Expr::Unary { expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Await { expr, .. }
        | Expr::Spread { expr, .. } => expr_contains_bigint(expr),
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
        Expr::FunctionExpr { .. } => false,
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
                    "issue-280: BigInt.asIntN/asUintN require literal bit width and BigInt value inputs in this builtin slice"
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
    if let Some(builtin) = resolve_bun_file_text_builtin(object.as_ref(), property, call_args)? {
        return Ok(Some(builtin));
    }

    Ok(None)
}

fn resolve_bun_file_text_builtin(
    object: &Expr,
    property: &str,
    call_args: &[Expr],
) -> Result<Option<BuiltinId>, Diagnostic> {
    if property != "text" {
        return Ok(None);
    }
    if !call_args.is_empty() {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "Bun.file(\"/dev/stdin\").text expects 0 arguments in this milestone, got {}",
                call_args.len()
            ),
            span: span_of_expr(object),
        });
    }
    let Expr::Call {
        callee: file_callee,
        args: file_args,
        ..
    } = object
    else {
        return Ok(None);
    };
    let Expr::Member {
        object: bun_object,
        property: file_property,
        ..
    } = file_callee.as_ref()
    else {
        return Ok(None);
    };
    let Expr::Ident {
        name: object_name, ..
    } = bun_object.as_ref()
    else {
        return Ok(None);
    };
    if object_name != "Bun" || file_property != "file" {
        return Ok(None);
    }
    match file_args.as_slice() {
        [Expr::String { value, .. }] if value == "/dev/stdin" => Ok(Some(BuiltinId::ReadStdinUtf8)),
        [arg] => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "Bun.file(...).text() currently supports only \"/dev/stdin\" stdin lowering"
                .to_owned(),
            span: span_of_expr(arg),
        }),
        _ => Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "Bun.file expects 1 argument in this milestone, got {}",
                file_args.len()
            ),
            span: span_of_expr(object),
        }),
    }
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
        | Expr::Await { span, .. }
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
        | Expr::FunctionExpr { span, .. }
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
        | Expr::Await { expr, .. }
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
        Expr::FunctionExpr { body, .. } => validate_static_block_stmts(body),
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
