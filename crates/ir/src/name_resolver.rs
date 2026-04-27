use ts2wasm_frontend::{DiagCode, Diagnostic, Expr, Span, Stmt};

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
            "Math",
            "Date",
            "RegExp",
            "JSON",
            "Error",
            "Map",
            "Set",
            "Promise",
            "Symbol",
            "TypeError",
            "ReferenceError",
            "SyntaxError",
            "RangeError",
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
        }
    }

    fn resolve_program(&mut self, program: &[Stmt]) -> Result<Vec<Stmt>, Diagnostic> {
        // First pass: collect all function declarations (hoisting)
        for stmt in program {
            if let Stmt::Function { name, span, .. } = stmt {
                self.functions.insert(name.clone(), Some(*span));
            }
        }
        // First pass: collect all class declarations (hoisting)
        for stmt in program {
            if let Stmt::ClassDecl { name, span, .. } = stmt {
                self.classes.insert(name.clone(), Some(*span));
            }
        }

        // Second pass: resolve all statements
        program.iter().map(|stmt| self.resolve_stmt(stmt)).collect()
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<Stmt, Diagnostic> {
        match stmt {
            Stmt::Let { name, expr, span } => {
                self.declare_variable(name, Some(*span))?;
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
                span,
            } => {
                // Function declarations are already collected in first pass
                // Now resolve the function body with its own scope
                self.enter_scope();
                for (param_name, default, is_rest) in params {
                    self.declare_variable(param_name, None)?;
                    if let Some(default_expr) = default {
                        self.resolve_expr(default_expr)?;
                    }
                    // Rest parameters don't need special handling in name resolution
                    let _ = is_rest;
                }
                let resolved_body = self.resolve_block(body)?;
                self.exit_scope();
                Ok(Stmt::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: resolved_body,
                    span: *span,
                })
            }
            Stmt::ClassDecl { .. } => {
                // Class declarations not yet supported in name resolution
                // Pass through for now
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
            Expr::This { span } => Ok(Expr::This { span: *span }),
            Expr::Ident { name, span } => {
                // Check if it's a function name
                if self.functions.contains_key(name) {
                    return Ok(Expr::Ident {
                        name: name.clone(),
                        span: *span,
                    });
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
            } => Ok(Expr::Binary {
                left: Box::new(self.resolve_expr(left)?),
                op: *op,
                right: Box::new(self.resolve_expr(right)?),
                span: *span,
            }),
            Expr::Call { callee, args, span } => {
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
            Expr::Member {
                object,
                property,
                span,
            } => {
                let resolved_object = self.resolve_expr(object)?;
                Ok(Expr::Member {
                    object: Box::new(resolved_object),
                    property: property.clone(),
                    span: *span,
                })
            }
            Expr::Array { elements, span } => Ok(Expr::Array {
                elements: elements
                    .iter()
                    .map(|e| self.resolve_expr(e))
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
            Expr::New { expr, args, span } => Ok(Expr::New {
                expr: Box::new(self.resolve_expr(expr)?),
                args: args
                    .iter()
                    .map(|a| self.resolve_expr(a))
                    .collect::<Result<Vec<_>, _>>()?,
                span: *span,
            }),
            Expr::PropertyAssign {
                object,
                property,
                value,
                span,
            } => Ok(Expr::PropertyAssign {
                object: Box::new(self.resolve_expr(object)?),
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
                object: Box::new(self.resolve_expr(object)?),
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
            Expr::ArrowFn { params, body, span } => Ok(Expr::ArrowFn {
                params: params.clone(),
                body: Box::new(self.resolve_expr(body)?),
                span: *span,
            }),
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

    fn resolve_block(&mut self, block: &[Stmt]) -> Result<Vec<Stmt>, Diagnostic> {
        self.enter_scope();
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

    fn declare_variable(&mut self, name: &str, span: Option<Span>) -> Result<(), Diagnostic> {
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
        // Check all scopes from innermost to outermost
        self.scopes
            .iter()
            .rev()
            .any(|scope| scope.contains_key(name))
    }

    fn resolve_identifier(&mut self, name: &str, span: Span) -> Result<(), Diagnostic> {
        if !self.is_declared(name) {
            Err(Diagnostic {
                code: DiagCode::UnresolvedName,
                message: format!("unresolved name: `{name}`"),
                span: Some(span),
            })
        } else {
            Ok(())
        }
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
