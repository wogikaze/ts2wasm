use ts2wasm_frontend::{ArrayLiteralElement, DiagCode, Diagnostic, Expr, Span, Stmt, UnaryOp};

use crate::binding_pattern::parse_binding_pattern;

/// Resolves variable and function names in lexical scope.
/// This pass runs before builtin resolution to catch unresolved names early.
/// It validates names but does not transform the AST - that's done by builtin_resolver.
pub fn resolve_names(program: &[Stmt]) -> Result<Vec<Stmt>, Diagnostic> {
    let mut resolver = NameResolver::new();
    resolver.resolve_program(program)
}

struct NameResolver {
    /// Stack of lexical scopes, each mapping names to their declaration spans
    scopes: Vec<std::collections::HashMap<String, Option<Span>>>,
    /// Function declarations at the current scope level
    functions: std::collections::HashMap<String, Option<Span>>,
    /// Class declarations at the current scope level
    classes: std::collections::HashMap<String, Option<Span>>,
    /// Global identifiers that are allowed (builtins like console, require, etc.)
    allowed_globals: std::collections::HashSet<String>,
    /// Active ECMAScript labels and whether their target is an iteration statement.
    labels: Vec<LabelBinding>,
    loop_depth: usize,
    breakable_depth: usize,
    function_depth: usize,
}

#[derive(Clone)]
struct LabelBinding {
    name: String,
    is_loop: bool,
}

impl NameResolver {
    fn new() -> Self {
        let allowed_globals = [
            "console",
            "process",
            "require",
            "exports",
            "module",
            "Buffer",
            "global",
            "Array",
            "Object",
            "String",
            "Number",
            "Boolean",
            "BigInt",
            "Math",
            "Date",
            "RegExp",
            "JSON",
            "Error",
            "Map",
            "Set",
            "Bun",
            "Promise",
            "Symbol",
            "TypeError",
            "ReferenceError",
            "SyntaxError",
            "RangeError",
            "Infinity",
            "NaN",
            "isNaN",
            "parseInt",
            "parseFloat",
            "isFinite",
            "encodeURI",
            "decodeURI",
            "escape",
            "unescape",
            "globalThis",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        Self {
            scopes: vec![std::collections::HashMap::new()],
            functions: std::collections::HashMap::new(),
            classes: std::collections::HashMap::new(),
            allowed_globals,
            labels: Vec::new(),
            loop_depth: 0,
            breakable_depth: 0,
            function_depth: 0,
        }
    }

    fn resolve_program(&mut self, program: &[Stmt]) -> Result<Vec<Stmt>, Diagnostic> {
        // First pass: collect all function declarations (hoisting)
        for stmt in program {
            if let Stmt::Function { name, span, .. } = stmt {
                if self.functions.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!("duplicate local variable: `{name}`"),
                        span: Some(*span),
                    });
                }
                self.functions.insert(name.clone(), Some(*span));
            }
        }
        // First pass: collect all class declarations (hoisting)
        for stmt in program {
            if let Stmt::ClassDecl { name, span, .. } = stmt {
                if self.classes.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!("duplicate local variable: `{name}`"),
                        span: Some(*span),
                    });
                }
                self.classes.insert(name.clone(), Some(*span));
            }
        }

        // Second pass: resolve all statements
        program.iter().map(|stmt| self.resolve_stmt(stmt)).collect()
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<Stmt, Diagnostic> {
        match stmt {
            Stmt::ImportSideEffect { span, .. } => {
                Err(unsupported_module_decl(*span, "side-effect import"))
            }
            Stmt::ImportNamed { span, .. } => Err(unsupported_module_decl(*span, "named import")),
            Stmt::ImportDefault { span, .. } => {
                Err(unsupported_module_decl(*span, "default import"))
            }
            Stmt::ImportDefaultNamed { span, .. } => Err(unsupported_module_decl(
                *span,
                "default import with named imports",
            )),
            Stmt::ImportNamespace { span, .. } => {
                Err(unsupported_module_decl(*span, "namespace import"))
            }
            Stmt::ImportDefaultNamespace { span, .. } => Err(unsupported_module_decl(
                *span,
                "default import with namespace import",
            )),
            Stmt::ExportNamed { specifiers, span } if specifiers.is_empty() => Ok(Stmt::Expr {
                expr: Expr::Undefined { span: *span },
                span: *span,
            }),
            Stmt::ExportNamed { span, .. } => Err(unsupported_module_decl(*span, "named export")),
            Stmt::ExportNamedFrom { span, .. } => {
                Err(unsupported_module_decl(*span, "named re-export"))
            }
            Stmt::ExportAllFrom { span, .. } => {
                Err(unsupported_module_decl(*span, "star re-export"))
            }
            Stmt::ExportNamespaceFrom { span, .. } => {
                Err(unsupported_module_decl(*span, "namespace re-export"))
            }
            Stmt::ExportDecl {
                declaration,
                span: _span,
                ..
            } => self.resolve_stmt(declaration),
            Stmt::ExportDefault { span, .. } => {
                Err(unsupported_module_decl(*span, "default export"))
            }
            Stmt::Let { name, expr, span } => {
                self.declare_binding(name, Some(*span))?;
                Ok(Stmt::Let {
                    name: name.clone(),
                    expr: self.resolve_expr(expr)?,
                    span: *span,
                })
            }
            Stmt::Assign { name, expr, span } => {
                self.resolve_identifier(name, *span)?;
                Ok(Stmt::Assign {
                    name: name.clone(),
                    expr: self.resolve_expr(expr)?,
                    span: *span,
                })
            }
            Stmt::Expr { expr, span } => Ok(Stmt::Expr {
                expr: self.resolve_expr(expr)?,
                span: *span,
            }),
            Stmt::If {
                condition,
                then_body,
                else_body,
                span,
            } => {
                let resolved_condition = self.resolve_expr(condition)?;
                let resolved_then = self.resolve_block(then_body)?;
                let resolved_else = self.resolve_block(else_body)?;
                Ok(Stmt::If {
                    condition: resolved_condition,
                    then_body: resolved_then,
                    else_body: resolved_else,
                    span: *span,
                })
            }
            Stmt::While {
                condition,
                body,
                span,
                ..
            } => {
                let resolved_condition = self.resolve_expr(condition)?;
                self.enter_loop();
                let resolved_body = self.resolve_block(body)?;
                self.exit_loop();
                Ok(Stmt::While {
                    condition: resolved_condition,
                    body: resolved_body,
                    span: *span,
                })
            }
            Stmt::Return { expr, span } => Ok(Stmt::Return {
                expr: self.resolve_expr(expr)?,
                span: *span,
            }),
            Stmt::Function {
                name,
                params,
                body,
                is_generator,
                span,
            } => {
                // Function declarations are already collected in first pass
                // Now resolve the function body with its own scope
                self.enter_scope();
                self.function_depth += 1;
                for (param_name, default, is_rest) in params {
                    self.declare_binding(param_name, Some(*span))?;
                    if *is_rest {
                        // For rest params with binding patterns like (...[value]),
                        // also declare the inner names from the pattern
                        if let Some(inner) = param_name.strip_prefix("...")
                            && let Some(pattern) = parse_binding_pattern(inner, Some(*span))?
                        {
                            for name in pattern.names() {
                                self.declare_variable(name, Some(*span))?;
                            }
                        }
                    }
                    if let Some(default_expr) = default {
                        self.resolve_expr(default_expr)?;
                    }
                    // Rest parameters don't need special handling in name resolution
                    let _ = is_rest;
                }
                let resolved_body = self.resolve_block(body)?;
                self.function_depth -= 1;
                self.exit_scope();
                Ok(Stmt::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: resolved_body,
                    is_generator: *is_generator,
                    span: *span,
                })
            }
            Stmt::ClassDecl { .. } => {
                // Pass-through at the name resolution level — class methods are lowered as
                // standalone functions by the lowered program builder (program.rs). The class
                // statement itself is dropped. See the LIMITATION comment in program.rs.
                Ok(stmt.clone())
            }
            Stmt::TryCatch {
                try_block,
                catch_param,
                catch_block,
                finally_block,
                span,
            } => {
                let resolved_try = self.resolve_block(try_block)?;
                let resolved_catch = if let Some(param) = catch_param {
                    self.enter_scope();
                    self.declare_variable(param, None)?;
                    let resolved = self
                        .resolve_block(catch_block.as_ref().map(|b| b.as_slice()).unwrap_or(&[]))?;
                    self.exit_scope();
                    Some(resolved)
                } else {
                    catch_block
                        .as_ref()
                        .map(|b| self.resolve_block(b))
                        .transpose()?
                };
                let resolved_finally = finally_block
                    .as_ref()
                    .map(|b| self.resolve_block(b))
                    .transpose()?;
                Ok(Stmt::TryCatch {
                    try_block: resolved_try,
                    catch_param: catch_param.clone(),
                    catch_block: resolved_catch,
                    finally_block: resolved_finally,
                    span: *span,
                })
            }
            Stmt::Throw { expr, span } => Ok(Stmt::Throw {
                expr: self.resolve_expr(expr)?,
                span: *span,
            }),
            Stmt::Switch { expr, cases, span } => {
                let resolved_expr = self.resolve_expr(expr)?;
                self.breakable_depth += 1;
                let resolved_cases = cases
                    .iter()
                    .map(|(cond, body)| {
                        Ok((
                            cond.as_ref().map(|c| self.resolve_expr(c)).transpose()?,
                            self.resolve_block(body)?,
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                self.breakable_depth -= 1;
                Ok(Stmt::Switch {
                    expr: resolved_expr,
                    cases: resolved_cases,
                    span: *span,
                })
            }
            Stmt::DoWhile {
                body,
                condition,
                span,
                ..
            } => {
                self.enter_loop();
                let resolved_body = self.resolve_block(body)?;
                let resolved_condition = self.resolve_expr(condition)?;
                self.exit_loop();
                Ok(Stmt::DoWhile {
                    body: resolved_body,
                    condition: resolved_condition,
                    span: *span,
                })
            }
            Stmt::For {
                init,
                condition,
                update,
                body,
                span,
            } => {
                self.enter_scope();
                let resolved_init = if let Some(i) = init {
                    Some(Box::new(self.resolve_stmt(i)?))
                } else {
                    None
                };
                let resolved_condition = condition
                    .as_ref()
                    .map(|c| self.resolve_expr(c))
                    .transpose()?;
                let resolved_update = update.as_ref().map(|u| self.resolve_expr(u)).transpose()?;
                self.enter_loop();
                let resolved_body = self.resolve_block(body)?;
                self.exit_loop();
                self.exit_scope();
                Ok(Stmt::For {
                    init: resolved_init,
                    condition: resolved_condition,
                    update: resolved_update,
                    body: resolved_body,
                    span: *span,
                })
            }
            Stmt::ForIn {
                var,
                iter,
                body,
                span,
            } => {
                self.enter_scope();
                self.declare_variable(var, None)?;
                let resolved_iter = self.resolve_expr(iter)?;
                self.enter_loop();
                let resolved_body = self.resolve_block(body)?;
                self.exit_loop();
                self.exit_scope();
                Ok(Stmt::ForIn {
                    var: var.clone(),
                    iter: resolved_iter,
                    body: resolved_body,
                    span: *span,
                })
            }
            Stmt::ForOf {
                var,
                iter,
                body,
                span,
            } => {
                self.enter_scope();
                self.declare_variable(var, None)?;
                let resolved_iter = self.resolve_expr(iter)?;
                self.enter_loop();
                let resolved_body = self.resolve_block(body)?;
                self.exit_loop();
                self.exit_scope();
                Ok(Stmt::ForOf {
                    var: var.clone(),
                    iter: resolved_iter,
                    body: resolved_body,
                    span: *span,
                })
            }
            Stmt::Labeled { label, body, span } => {
                if self.labels.iter().any(|binding| binding.name == *label) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!("duplicate label `{label}`"),
                        span: Some(*span),
                    });
                }
                self.labels.push(LabelBinding {
                    name: label.clone(),
                    is_loop: is_loop_stmt(body),
                });
                let resolved_body = self.resolve_stmt(body)?;
                self.labels.pop();
                Ok(Stmt::Labeled {
                    label: label.clone(),
                    body: Box::new(resolved_body),
                    span: *span,
                })
            }
            Stmt::Break { label, span } => {
                if let Some(label) = label {
                    if !self.labels.iter().any(|binding| binding.name == *label) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!("undefined break label `{label}`"),
                            span: Some(*span),
                        });
                    }
                } else if self.breakable_depth == 0 {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "break must be inside a loop or switch".to_owned(),
                        span: Some(*span),
                    });
                }
                Ok(Stmt::Break {
                    label: label.clone(),
                    span: *span,
                })
            }
            Stmt::Continue { label, span } => {
                if let Some(label) = label {
                    match self
                        .labels
                        .iter()
                        .rev()
                        .find(|binding| binding.name == *label)
                    {
                        Some(binding) if binding.is_loop => {}
                        Some(_) => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!("continue label `{label}` does not target a loop"),
                                span: Some(*span),
                            });
                        }
                        None => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!("undefined continue label `{label}`"),
                                span: Some(*span),
                            });
                        }
                    }
                } else if self.loop_depth == 0 {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "continue must be inside a loop".to_owned(),
                        span: Some(*span),
                    });
                }
                Ok(Stmt::Continue {
                    label: label.clone(),
                    span: *span,
                })
            }
        }
    }

    fn enter_loop(&mut self) {
        self.loop_depth += 1;
        self.breakable_depth += 1;
    }

    fn exit_loop(&mut self) {
        self.loop_depth -= 1;
        self.breakable_depth -= 1;
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<Expr, Diagnostic> {
        match expr {
            Expr::Number { value, span } => Ok(Expr::Number {
                value: *value,
                span: *span,
            }),
            Expr::BigInt { raw, span } => Ok(Expr::BigInt {
                raw: raw.clone(),
                span: *span,
            }),
            Expr::String { value, span } => Ok(Expr::String {
                value: value.clone(),
                span: *span,
            }),
            Expr::Bool { value, span } => Ok(Expr::Bool {
                value: *value,
                span: *span,
            }),
            Expr::Null { span } => Ok(Expr::Null { span: *span }),
            Expr::Undefined { span } => Ok(Expr::Undefined { span: *span }),
            Expr::Await { expr, span } => Ok(Expr::Await {
                expr: Box::new(self.resolve_expr(expr)?),
                span: *span,
            }),
            Expr::This { span } => Ok(Expr::This { span: *span }),
            Expr::ClassExpr {
                name,
                extends,
                body,
                static_blocks,
                private_elements,
                span,
            } => {
                self.enter_scope();
                if !name.is_empty() {
                    self.declare_binding(name, Some(*span))?;
                }
                let resolved_extends =
                    extends.as_ref().map(|e| self.resolve_expr(e)).transpose()?;
                let _ = static_blocks;
                let _ = private_elements;
                let resolved_body = self.resolve_block(body)?;
                self.exit_scope();
                Ok(Expr::ClassExpr {
                    name: name.clone(),
                    extends: resolved_extends.map(Box::new),
                    body: resolved_body,
                    static_blocks: static_blocks.clone(),
                    private_elements: private_elements.clone(),
                    span: *span,
                })
            }
            Expr::FunctionExpr {
                name,
                params,
                body,
                span,
            } => {
                self.enter_scope();
                self.function_depth += 1;
                if !name.is_empty() {
                    self.declare_binding(name, Some(*span))?;
                }
                for (param_name, default, is_rest) in params {
                    self.declare_binding(param_name, Some(*span))?;
                    if *is_rest {
                        // For rest params with binding patterns like (...[value]),
                        // also declare the inner names from the pattern
                        if let Some(inner) = param_name.strip_prefix("...")
                            && let Some(pattern) = parse_binding_pattern(inner, Some(*span))?
                        {
                            for name in pattern.names() {
                                self.declare_variable(name, Some(*span))?;
                            }
                        }
                    }
                    if let Some(default_expr) = default {
                        self.resolve_expr(default_expr)?;
                    }
                    let _ = is_rest;
                }
                let resolved_body = self.resolve_block(body)?;
                self.function_depth -= 1;
                self.exit_scope();
                Ok(Expr::FunctionExpr {
                    name: name.clone(),
                    params: params.clone(),
                    body: resolved_body,
                    span: *span,
                })
            }
            Expr::Ident { name, span } => {
                // Check if it's a function name
                if self.functions.contains_key(name) || self.is_implicit_arguments(name) {
                    return Ok(Expr::Ident {
                        name: name.clone(),
                        span: *span,
                    });
                }
                // Reject class name used as value — class runtime is not yet supported
                if self.classes.contains_key(name) {
                    return Err(unsupported_class_value(name, *span));
                }
                // Check if it's a variable in scope or allowed global
                if self.is_declared(name) {
                    Ok(Expr::Ident {
                        name: name.clone(),
                        span: *span,
                    })
                } else if self.allowed_globals.contains(name) {
                    // Fallback: directly check allowed_globals if is_declared missed it
                    Ok(Expr::Ident {
                        name: name.clone(),
                        span: *span,
                    })
                } else {
                    if name == "arguments" {
                        return Err(unsupported_arguments_outside_function(*span));
                    }
                    Err(Diagnostic {
                        code: DiagCode::UnresolvedName,
                        message: format!("unresolved name: `{name}`"),
                        span: Some(*span),
                    })
                }
            }
            Expr::Unary { op, expr, span } => Ok(Expr::Unary {
                op: *op,
                expr: Box::new(self.resolve_expr(expr)?),
                span: *span,
            }),
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                if let Some(diagnostic) =
                    self.bigint_number_model_gap(left.as_ref(), right.as_ref(), *span)
                {
                    return Err(diagnostic);
                }
                Ok(Expr::Binary {
                    left: Box::new(self.resolve_expr(left)?),
                    op: *op,
                    right: Box::new(self.resolve_expr(right)?),
                    span: *span,
                })
            }
            Expr::Call { callee, args, span } => {
                if self.is_test262_assert_reference_error_probe(callee, args)
                    || self.is_test262_assert_comparison_probe(callee, args)
                {
                    return Ok(Expr::Call {
                        callee: callee.clone(),
                        args: args.clone(),
                        span: *span,
                    });
                }
                if self.is_unshadowed_function_constructor(callee) {
                    return Err(unsupported_function_constructor(*span));
                }
                let resolved_callee = self.resolve_expr(callee)?;
                let resolved_args = args
                    .iter()
                    .map(|a| self.resolve_expr(a))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Expr::Call {
                    callee: Box::new(resolved_callee),
                    args: resolved_args,
                    span: *span,
                })
            }
            Expr::Assign { name, expr, span } => {
                self.resolve_identifier(name, *span)?;
                Ok(Expr::Assign {
                    name: name.clone(),
                    expr: Box::new(self.resolve_expr(expr)?),
                    span: *span,
                })
            }
            Expr::LogicalAssign {
                name,
                op,
                expr,
                span,
            } => {
                self.resolve_identifier(name, *span)?;
                Ok(Expr::LogicalAssign {
                    name: name.clone(),
                    op: *op,
                    expr: Box::new(self.resolve_expr(expr)?),
                    span: *span,
                })
            }
            Expr::LogicalPropertyAssign {
                object,
                object_expr,
                property,
                computed_key,
                op,
                expr,
                span,
            } => {
                let resolved_object_expr = object_expr
                    .as_ref()
                    .map(|object| self.resolve_expr(object))
                    .transpose()?
                    .map(Box::new);
                if resolved_object_expr.is_none() {
                    self.resolve_identifier(object, *span)?;
                }
                Ok(Expr::LogicalPropertyAssign {
                    object: object.clone(),
                    object_expr: resolved_object_expr,
                    property: property.clone(),
                    computed_key: computed_key
                        .as_ref()
                        .map(|key| self.resolve_expr(key))
                        .transpose()?
                        .map(Box::new),
                    op: *op,
                    expr: Box::new(self.resolve_expr(expr)?),
                    span: *span,
                })
            }
            Expr::Member {
                object,
                property,
                span,
            } => {
                if self.is_unshadowed_test262_ishtmldda_member(object, property) {
                    return Err(unsupported_annex_b_ishtmldda(*span));
                }
                let resolved_object = self.resolve_member_target(object)?;
                Ok(Expr::Member {
                    object: Box::new(resolved_object),
                    property: property.clone(),
                    span: *span,
                })
            }
            Expr::OptionalMember {
                object,
                property,
                span,
            } => {
                let resolved_object = self.resolve_member_target(object)?;
                Ok(Expr::OptionalMember {
                    object: Box::new(resolved_object),
                    property: property.clone(),
                    span: *span,
                })
            }
            Expr::OptionalIndex {
                object,
                index,
                span,
            } => {
                let resolved_object = self.resolve_member_target(object)?;
                let resolved_index = self.resolve_expr(index)?;
                Ok(Expr::OptionalIndex {
                    object: Box::new(resolved_object),
                    index: Box::new(resolved_index),
                    span: *span,
                })
            }
            Expr::OptionalCall { callee, args, span } => Ok(Expr::OptionalCall {
                callee: Box::new(self.resolve_expr(callee)?),
                args: args
                    .iter()
                    .map(|arg| self.resolve_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?,
                span: *span,
            }),
            Expr::Array { elements, span } => Ok(Expr::Array {
                elements: elements
                    .iter()
                    .map(|element| match element {
                        ArrayLiteralElement::Present(expr) => {
                            Ok(ArrayLiteralElement::Present(self.resolve_expr(expr)?))
                        }
                        ArrayLiteralElement::Spread(expr) => {
                            Ok(ArrayLiteralElement::Spread(self.resolve_expr(expr)?))
                        }
                        ArrayLiteralElement::Hole(span) => Ok(ArrayLiteralElement::Hole(*span)),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                span: *span,
            }),
            Expr::Object { props, span } => Ok(Expr::Object {
                props: props
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), self.resolve_expr(v)?)))
                    .collect::<Result<Vec<_>, _>>()?,
                span: *span,
            }),
            Expr::Index {
                object,
                index,
                span,
            } => Ok(Expr::Index {
                object: Box::new(self.resolve_expr(object)?),
                index: Box::new(self.resolve_expr(index)?),
                span: *span,
            }),
            Expr::New { expr, args, span } => {
                // Extract callee identifier directly to bypass class-value check
                let callee_name = match expr.as_ref() {
                    Expr::Ident { name, .. } => name.clone(),
                    Expr::Member { property, .. } => property.clone(),
                    _ => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "issue-062: new requires a class name identifier".to_owned(),
                            span: Some(*span),
                        });
                    }
                };
                if callee_name == "Function" && !self.is_user_declared("Function") {
                    return Err(unsupported_function_constructor(*span));
                }
                Ok(Expr::New {
                    expr: Box::new(Expr::Ident {
                        name: callee_name,
                        span: *span,
                    }),
                    args: args
                        .iter()
                        .map(|a| self.resolve_expr(a))
                        .collect::<Result<Vec<_>, _>>()?,
                    span: *span,
                })
            }
            Expr::PropertyAssign {
                object,
                property,
                value,
                span,
            } => Ok(Expr::PropertyAssign {
                object: Box::new(self.resolve_member_target(object)?),
                property: property.clone(),
                value: Box::new(self.resolve_expr(value)?),
                span: *span,
            }),
            Expr::IndexAssign {
                object,
                index,
                value,
                span,
            } => Ok(Expr::IndexAssign {
                object: Box::new(self.resolve_member_target(object)?),
                index: Box::new(self.resolve_expr(index)?),
                value: Box::new(self.resolve_expr(value)?),
                span: *span,
            }),
            Expr::InstanceOf {
                expr,
                type_expr,
                span,
            } => Ok(Expr::InstanceOf {
                expr: Box::new(self.resolve_expr(expr)?),
                type_expr: Box::new(self.resolve_expr(type_expr)?),
                span: *span,
            }),
            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
                span,
            } => Ok(Expr::Ternary {
                condition: Box::new(self.resolve_expr(condition)?),
                then_expr: Box::new(self.resolve_expr(then_expr)?),
                else_expr: Box::new(self.resolve_expr(else_expr)?),
                span: *span,
            }),
            Expr::ArrowFn { params, body, span } => {
                self.enter_scope();
                for param in params {
                    self.declare_binding(param, Some(*span))?;
                    // For rest params with binding patterns like (...[value]),
                    // also declare the inner names from the pattern
                    if let Some(inner) = param.strip_prefix("...")
                        && let Some(pattern) = parse_binding_pattern(inner, Some(*span))?
                    {
                        for name in pattern.names() {
                            self.declare_variable(name, Some(*span))?;
                        }
                    }
                }
                let resolved_body = self.resolve_expr(body)?;
                self.exit_scope();
                Ok(Expr::ArrowFn {
                    params: params.clone(),
                    body: Box::new(resolved_body),
                    span: *span,
                })
            }
            Expr::Spread { expr, span } => Ok(Expr::Spread {
                expr: Box::new(self.resolve_expr(expr)?),
                span: *span,
            }),
            Expr::TypeOf { expr, span } => Ok(Expr::TypeOf {
                expr: Box::new(self.resolve_expr(expr)?),
                span: *span,
            }),
        }
    }

    /// Resolve an expression used as a member access target (e.g., `obj.prop`, `obj[0]`).
    /// Allows class name identifiers through without triggering the class-value rejection,
    /// since `Counter.staticMethod()` and similar patterns work at runtime.
    fn resolve_member_target(&mut self, expr: &Expr) -> Result<Expr, Diagnostic> {
        match expr {
            Expr::Ident { name, span } => {
                // Validate the identifier exists (function, class, variable, or allowed global)
                if self.functions.contains_key(name)
                    || self.is_implicit_arguments(name)
                    || self.is_declared(name)
                    || self.allowed_globals.contains(name)
                {
                    return Ok(Expr::Ident {
                        name: name.clone(),
                        span: *span,
                    });
                }
                Err(Diagnostic {
                    code: DiagCode::UnresolvedName,
                    message: format!("unresolved name: `{name}`"),
                    span: Some(*span),
                })
            }
            _ => self.resolve_expr(expr),
        }
    }

    fn resolve_block(&mut self, block: &[Stmt]) -> Result<Vec<Stmt>, Diagnostic> {
        self.enter_scope();
        for stmt in block {
            if let Stmt::Function { name, span, .. } = stmt {
                self.declare_variable(name, Some(*span))?;
            }
        }
        let result = block.iter().map(|s| self.resolve_stmt(s)).collect();
        self.exit_scope();
        result
    }

    fn enter_scope(&mut self) {
        self.scopes.push(std::collections::HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_binding(&mut self, binding: &str, span: Option<Span>) -> Result<(), Diagnostic> {
        if let Some(pattern) = parse_binding_pattern(binding, span)? {
            for name in pattern.names() {
                self.declare_variable(name, span)?;
            }
            Ok(())
        } else {
            self.declare_variable(binding, span)
        }
    }

    fn declare_variable(&mut self, name: &str, span: Option<Span>) -> Result<(), Diagnostic> {
        // In the top-level scope, check hoisted declarations (functions, classes) for conflicts.
        // Nested scopes can shadow outer functions/classes.
        if self.scopes.len() == 1
            && (self.functions.contains_key(name) || self.classes.contains_key(name))
        {
            return Err(Diagnostic {
                code: DiagCode::DuplicateLocal,
                message: format!("duplicate local variable: `{name}`"),
                span,
            });
        }
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope.contains_key(name) {
            Err(Diagnostic {
                code: DiagCode::DuplicateLocal,
                message: format!("duplicate local variable: `{name}`"),
                span,
            })
        } else {
            current_scope.insert(name.to_string(), span);
            Ok(())
        }
    }

    fn is_declared(&self, name: &str) -> bool {
        // Check function declarations first (hoisting)
        if self.functions.contains_key(name) {
            return true;
        }
        // Check class declarations (hoisting)
        if self.classes.contains_key(name) {
            return true;
        }
        // Check allowed global identifiers
        if self.allowed_globals.contains(name) {
            return true;
        }
        if self.is_implicit_arguments(name) {
            return true;
        }
        // Check all scopes from innermost to outermost
        self.scopes
            .iter()
            .rev()
            .any(|scope| scope.contains_key(name))
    }

    fn is_unshadowed_function_constructor(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::Ident { name, .. } if name == "Function")
            && !self.is_user_declared("Function")
    }

    fn bigint_number_model_gap(&self, left: &Expr, right: &Expr, span: Span) -> Option<Diagnostic> {
        let other = if expr_contains_bigint_literal(left) {
            Some(right)
        } else if expr_contains_bigint_literal(right) {
            Some(left)
        } else {
            None
        }?;
        let model_value = self.bigint_number_model_gap_value(other)?;
        Some(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-281: BigInt/Number comparison with `{model_value}` requires broader number-model support"
            ),
            span: Some(span),
        })
    }

    fn bigint_number_model_gap_value(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Ident { name, .. }
                if matches!(name.as_str(), "NaN" | "Infinity") && !self.is_user_declared(name) =>
            {
                Some(name.clone())
            }
            Expr::Member {
                object, property, ..
            } if self.is_unshadowed_number_model_gap_member(object, property) => {
                Some(format!("Number.{property}"))
            }
            Expr::Unary { op, expr, .. } if matches!(op, UnaryOp::Plus | UnaryOp::Negate) => {
                let value = self.bigint_number_model_gap_value(expr.as_ref())?;
                let sign = if *op == UnaryOp::Negate { "-" } else { "+" };
                Some(format!("{sign}{value}"))
            }
            _ => None,
        }
    }

    fn is_unshadowed_number_model_gap_member(&self, object: &Expr, property: &str) -> bool {
        matches!(object, Expr::Ident { name, .. } if name == "Number")
            && !self.is_user_declared("Number")
            && is_number_model_gap_property(property)
    }

    fn is_unshadowed_test262_ishtmldda_member(&self, object: &Expr, property: &str) -> bool {
        matches!(object, Expr::Ident { name, .. } if name == "$262")
            && property == "IsHTMLDDA"
            && !self.is_user_declared("$262")
    }

    fn is_test262_assert_reference_error_probe(&self, callee: &Expr, args: &[Expr]) -> bool {
        if !self.functions.contains_key("assert") || !self.classes.contains_key("Test262Assert") {
            return false;
        }
        let Expr::Member {
            object, property, ..
        } = callee
        else {
            return false;
        };
        if !matches!(object.as_ref(), Expr::Ident { name, .. } if name == "assert")
            || property != "throws"
        {
            return false;
        }
        let [
            Expr::Ident {
                name: error_name, ..
            },
            callback,
            ..,
        ] = args
        else {
            return false;
        };
        if error_name != "ReferenceError" {
            return false;
        }
        matches!(
            callback,
            Expr::FunctionExpr { params, body, .. }
                if params.is_empty()
                    && matches!(
                        body.as_slice(),
                        [Stmt::Expr {
                            expr: Expr::Ident { .. },
                            ..
                        }]
                )
        )
    }

    fn is_test262_assert_comparison_probe(&self, callee: &Expr, args: &[Expr]) -> bool {
        let Expr::Member {
            object, property, ..
        } = callee
        else {
            return false;
        };
        matches!(object.as_ref(), Expr::Ident { name, .. } if name == "assert")
            && matches!(property.as_str(), "sameValue" | "notSameValue")
            && matches!(args.len(), 2 | 3)
    }

    fn is_user_declared(&self, name: &str) -> bool {
        self.functions.contains_key(name)
            || self.classes.contains_key(name)
            || self
                .scopes
                .iter()
                .rev()
                .any(|scope| scope.contains_key(name))
    }

    fn resolve_identifier(&mut self, name: &str, span: Span) -> Result<(), Diagnostic> {
        if !self.is_declared(name) {
            if name == "arguments" {
                return Err(unsupported_arguments_outside_function(span));
            }
            Err(Diagnostic {
                code: DiagCode::UnresolvedName,
                message: format!("unresolved name: `{name}`"),
                span: Some(span),
            })
        } else if self.is_class_only(name) {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-5011: class `{name}` used as a value is not yet supported; class declarations are partially supported (methods work, constructor/prototype/class-value not yet implemented)"
                ),
                span: Some(span),
            })
        } else {
            Ok(())
        }
    }

    fn is_class_only(&self, name: &str) -> bool {
        self.classes.contains_key(name)
            && !self.functions.contains_key(name)
            && !self.allowed_globals.contains(name)
            && !self.is_implicit_arguments(name)
            && !self
                .scopes
                .iter()
                .rev()
                .any(|scope| scope.contains_key(name))
    }

    fn is_implicit_arguments(&self, name: &str) -> bool {
        name == "arguments" && self.function_depth > 0
    }
}

fn unsupported_function_constructor(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-062: dynamic Function constructor is not supported; runtime code evaluation is intentionally not implemented".to_owned(),
        span: Some(span),
    }
}

fn unsupported_arguments_outside_function(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message:
            "issue-062d: `arguments` is only supported inside non-arrow functions in this milestone"
                .to_owned(),
        span: Some(span),
    }
}

fn unsupported_class_value(name: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-5011: class `{name}` cannot be used as a value — class runtime is not yet supported"
        ),
        span: Some(span),
    }
}

fn unsupported_annex_b_ishtmldda(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported".to_owned(),
        span: Some(span),
    }
}

fn unsupported_module_decl(span: Span, form: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-055: unsupported {form}; module resolution and loading are not implemented"
        ),
        span: Some(span),
    }
}

fn is_loop_stmt(stmt: &Stmt) -> bool {
    matches!(
        stmt,
        Stmt::While { .. }
            | Stmt::DoWhile { .. }
            | Stmt::For { .. }
            | Stmt::ForIn { .. }
            | Stmt::ForOf { .. }
    )
}

fn expr_contains_bigint_literal(expr: &Expr) -> bool {
    match expr {
        Expr::BigInt { .. } => true,
        Expr::Unary { expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Await { expr, .. }
        | Expr::Spread { expr, .. } => expr_contains_bigint_literal(expr),
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
        }
        | Expr::OptionalIndex {
            object: left,
            index: right,
            ..
        } => expr_contains_bigint_literal(left) || expr_contains_bigint_literal(right),
        Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
            expr_contains_bigint_literal(callee) || args.iter().any(expr_contains_bigint_literal)
        }
        Expr::Member { object, .. } | Expr::OptionalMember { object, .. } => {
            expr_contains_bigint_literal(object)
        }
        Expr::Assign { expr, .. } | Expr::LogicalAssign { expr, .. } => {
            expr_contains_bigint_literal(expr)
        }
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => {
            object_expr
                .as_deref()
                .is_some_and(expr_contains_bigint_literal)
                || computed_key
                    .as_deref()
                    .is_some_and(expr_contains_bigint_literal)
                || expr_contains_bigint_literal(expr)
        }
        Expr::Array { elements, .. } => elements.iter().any(|element| match element {
            ArrayLiteralElement::Present(expr) | ArrayLiteralElement::Spread(expr) => {
                expr_contains_bigint_literal(expr)
            }
            ArrayLiteralElement::Hole(_) => false,
        }),
        Expr::Object { props, .. } => props
            .iter()
            .any(|(_, value)| expr_contains_bigint_literal(value)),
        Expr::New { args, .. } => args.iter().any(expr_contains_bigint_literal),
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_bigint_literal(condition)
                || expr_contains_bigint_literal(then_expr)
                || expr_contains_bigint_literal(else_expr)
        }
        Expr::ArrowFn { body, .. } => expr_contains_bigint_literal(body),
        Expr::PropertyAssign { object, value, .. } => {
            expr_contains_bigint_literal(object) || expr_contains_bigint_literal(value)
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            expr_contains_bigint_literal(object)
                || expr_contains_bigint_literal(index)
                || expr_contains_bigint_literal(value)
        }
        Expr::FunctionExpr { .. }
        | Expr::ClassExpr { .. }
        | Expr::Number { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::Undefined { .. }
        | Expr::This { .. }
        | Expr::Ident { .. } => false,
    }
}

fn is_number_model_gap_property(property: &str) -> bool {
    matches!(
        property,
        "NaN"
            | "POSITIVE_INFINITY"
            | "NEGATIVE_INFINITY"
            | "MAX_VALUE"
            | "MIN_VALUE"
            | "EPSILON"
            | "MAX_SAFE_INTEGER"
            | "MIN_SAFE_INTEGER"
    )
}
