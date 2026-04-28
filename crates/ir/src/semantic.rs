use std::collections::HashMap;

use ts2wasm_frontend::{BinaryOp, DiagCode, Diagnostic, UnaryOp};

use crate::builtin::{BuiltinId, BuiltinPropertyId};
use crate::builtin_resolved::{ResolvedExpr, ResolvedStmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirFunctionId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirProgram {
    pub body: Vec<HirStmt>,
    pub locals: Vec<HirLocalId>,
    pub functions: Vec<HirFunction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirFunction {
    pub id: HirFunctionId,
    pub params: Vec<HirLocalId>,
    pub locals: Vec<HirLocalId>,
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HirStmt {
    Let {
        local: HirLocalId,
        init: HirExpr,
    },
    StoreLocal {
        local: HirLocalId,
        value: HirExpr,
    },
    Expr(HirExpr),
    BranchIfTruthy {
        condition: HirExpr,
        then_body: Vec<HirStmt>,
        else_body: Vec<HirStmt>,
    },
    LoopWhile {
        condition: HirExpr,
        body: Vec<HirStmt>,
    },
    Return(HirExpr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HirExpr {
    ConstUndefined,
    ConstNull,
    ConstBool(bool),
    ConstNumber(i32),
    ConstString(String),
    LoadLocal(HirLocalId),
    LoadBuiltin(String),
    ToBoolean(Box<HirExpr>),
    JsUnaryNot(Box<HirExpr>),
    JsAdd {
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    JsStrictEqual {
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    JsAbstractEqual {
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    JsRelational {
        op: HirRelationalOp,
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    GetProp {
        object: Box<HirExpr>,
        key: String,
    },
    GetIndex {
        object: Box<HirExpr>,
        index: Box<HirExpr>,
    },
    ArrayLength(Box<HirExpr>),
    CallBuiltin {
        builtin: BuiltinId,
        args: Vec<HirExpr>,
    },
    CallFunction {
        function: HirFunctionId,
        args: Vec<HirExpr>,
    },
    CallMethod {
        receiver: Box<HirExpr>,
        method: String,
        args: Vec<HirExpr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HirRelationalOp {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub fn lower_to_hir(program: &[ResolvedStmt]) -> Result<HirProgram, Diagnostic> {
    let function_ids = collect_function_ids(program)?;
    let mut lowerer = HirLowerer::new(&function_ids);
    let mut body = Vec::new();
    let mut functions = Vec::new();

    for stmt in program {
        match stmt {
            ResolvedStmt::Function {
                name, params, body, ..
            } => {
                let id = function_ids[name.as_str()];
                functions.push(lower_function(id, params, body, &function_ids)?);
            }
            _ => body.push(lowerer.lower_stmt(stmt)?),
        }
    }

    Ok(HirProgram {
        body,
        locals: lowerer.locals,
        functions,
    })
}

pub fn validate_hir(program: &HirProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    validate_function_ids(program, &mut errors);
    let top_context = ValidationContext {
        locals: &program.locals,
        functions_len: program.functions.len(),
        allow_return: false,
    };
    validate_stmts(&program.body, top_context, &mut errors);

    for (index, function) in program.functions.iter().enumerate() {
        if function.id.0 != index {
            errors.push(invariant(format!(
                "function id {:?} does not match function table index {index}",
                function.id
            )));
        }
        for param in &function.params {
            if !function.locals.contains(param) {
                errors.push(invariant(format!(
                    "function {:?} parameter {:?} is not present in locals",
                    function.id, param
                )));
            }
        }
        let context = ValidationContext {
            locals: &function.locals,
            functions_len: program.functions.len(),
            allow_return: true,
        };
        validate_stmts(&function.body, context, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[derive(Clone, Copy)]
struct ValidationContext<'a> {
    locals: &'a [HirLocalId],
    functions_len: usize,
    allow_return: bool,
}

fn validate_function_ids(program: &HirProgram, errors: &mut Vec<Diagnostic>) {
    for (index, local) in program.locals.iter().enumerate() {
        if local.0 != index {
            errors.push(invariant(format!(
                "top-level local id {:?} does not match locals index {index}",
                local
            )));
        }
    }
}

fn validate_stmts(stmts: &[HirStmt], context: ValidationContext<'_>, errors: &mut Vec<Diagnostic>) {
    for stmt in stmts {
        match stmt {
            HirStmt::Let { local, init } | HirStmt::StoreLocal { local, value: init } => {
                validate_local(*local, context, errors);
                validate_expr(init, context, errors);
            }
            HirStmt::Expr(expr) => validate_expr(expr, context, errors),
            HirStmt::BranchIfTruthy {
                condition,
                then_body,
                else_body,
            } => {
                if !matches!(condition, HirExpr::ToBoolean(_)) {
                    errors.push(invariant(
                        "BranchIfTruthy condition must be explicit ToBoolean".to_owned(),
                    ));
                }
                validate_expr(condition, context, errors);
                validate_stmts(then_body, context, errors);
                validate_stmts(else_body, context, errors);
            }
            HirStmt::LoopWhile { condition, body } => {
                if !matches!(condition, HirExpr::ToBoolean(_)) {
                    errors.push(invariant(
                        "LoopWhile condition must be explicit ToBoolean".to_owned(),
                    ));
                }
                validate_expr(condition, context, errors);
                validate_stmts(body, context, errors);
            }
            HirStmt::Return(expr) => {
                if !context.allow_return {
                    errors.push(invariant("top-level Return is not valid HIR".to_owned()));
                }
                validate_expr(expr, context, errors);
            }
        }
    }
}

fn validate_expr(expr: &HirExpr, context: ValidationContext<'_>, errors: &mut Vec<Diagnostic>) {
    match expr {
        HirExpr::LoadLocal(local) => validate_local(*local, context, errors),
        HirExpr::LoadBuiltin(_) => {}
        HirExpr::ToBoolean(expr) | HirExpr::JsUnaryNot(expr) | HirExpr::ArrayLength(expr) => {
            validate_expr(expr, context, errors);
        }
        HirExpr::JsAdd { left, right }
        | HirExpr::JsStrictEqual { left, right }
        | HirExpr::JsAbstractEqual { left, right }
        | HirExpr::JsRelational { left, right, .. }
        | HirExpr::GetIndex {
            object: left,
            index: right,
        } => {
            validate_expr(left, context, errors);
            validate_expr(right, context, errors);
        }
        HirExpr::GetProp { object, .. } => validate_expr(object, context, errors),
        HirExpr::CallBuiltin { args, .. } => {
            for arg in args {
                validate_expr(arg, context, errors);
            }
        }
        HirExpr::CallFunction { function, args } => {
            if function.0 >= context.functions_len {
                errors.push(invariant(format!("invalid function id {:?}", function)));
            }
            for arg in args {
                validate_expr(arg, context, errors);
            }
        }
        HirExpr::CallMethod { receiver, args, .. } => {
            validate_expr(receiver, context, errors);
            for arg in args {
                validate_expr(arg, context, errors);
            }
        }
        HirExpr::ConstUndefined
        | HirExpr::ConstNull
        | HirExpr::ConstBool(_)
        | HirExpr::ConstNumber(_)
        | HirExpr::ConstString(_) => {}
    }
}

fn validate_local(local: HirLocalId, context: ValidationContext<'_>, errors: &mut Vec<Diagnostic>) {
    if !context.locals.contains(&local) {
        errors.push(invariant(format!("invalid local id {:?}", local)));
    }
}

fn invariant(message: String) -> Diagnostic {
    Diagnostic {
        code: DiagCode::InvariantViolation,
        message,
        span: None,
    }
}

fn collect_function_ids(
    program: &[ResolvedStmt],
) -> Result<HashMap<String, HirFunctionId>, Diagnostic> {
    let mut ids = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::Function { name, .. } = stmt {
            if ids.contains_key(name.as_str()) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateFunction,
                    message: format!("duplicate function definition: `{name}`"),
                    span: None,
                });
            }
            ids.insert(name.clone(), HirFunctionId(ids.len()));
        }
    }
    Ok(ids)
}

fn lower_function(
    id: HirFunctionId,
    params: &[(String, Option<ResolvedExpr>, bool)],
    body: &[ResolvedStmt],
    function_ids: &HashMap<String, HirFunctionId>,
) -> Result<HirFunction, Diagnostic> {
    let mut lowerer = HirLowerer::new(function_ids);
    let mut param_ids = Vec::new();
    for (name, _, _) in params {
        param_ids.push(lowerer.declare_local(name)?);
    }
    let body = lowerer.lower_block(body)?;
    Ok(HirFunction {
        id,
        params: param_ids,
        locals: lowerer.locals,
        body,
    })
}

struct HirLowerer<'a> {
    function_ids: &'a HashMap<String, HirFunctionId>,
    scopes: Vec<HashMap<String, HirLocalId>>,
    locals: Vec<HirLocalId>,
}

impl<'a> HirLowerer<'a> {
    fn new(function_ids: &'a HashMap<String, HirFunctionId>) -> Self {
        Self {
            function_ids,
            scopes: vec![HashMap::new()],
            locals: Vec::new(),
        }
    }

    fn lower_block(&mut self, statements: &[ResolvedStmt]) -> Result<Vec<HirStmt>, Diagnostic> {
        let mut lowered = Vec::with_capacity(statements.len());
        for statement in statements {
            lowered.push(self.lower_stmt(statement)?);
        }
        Ok(lowered)
    }

    fn lower_nested_block(
        &mut self,
        statements: &[ResolvedStmt],
    ) -> Result<Vec<HirStmt>, Diagnostic> {
        self.scopes.push(HashMap::new());
        let lowered = self.lower_block(statements);
        self.scopes.pop();
        lowered
    }

    fn lower_stmt(&mut self, stmt: &ResolvedStmt) -> Result<HirStmt, Diagnostic> {
        match stmt {
            ResolvedStmt::Let(name, expr) => {
                let local = self.declare_local(name)?;
                Ok(HirStmt::Let {
                    local,
                    init: self.lower_expr(expr)?,
                })
            }
            ResolvedStmt::Assign(name, expr) => {
                let local = self.resolve_local(name)?;
                Ok(HirStmt::StoreLocal {
                    local,
                    value: self.lower_expr(expr)?,
                })
            }
            ResolvedStmt::Expr(expr) => Ok(HirStmt::Expr(self.lower_expr(expr)?)),
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => Ok(HirStmt::BranchIfTruthy {
                condition: HirExpr::ToBoolean(Box::new(self.lower_expr(condition)?)),
                then_body: self.lower_nested_block(then_body)?,
                else_body: self.lower_nested_block(else_body)?,
            }),
            ResolvedStmt::While { condition, body } => Ok(HirStmt::LoopWhile {
                condition: HirExpr::ToBoolean(Box::new(self.lower_expr(condition)?)),
                body: self.lower_nested_block(body)?,
            }),
            ResolvedStmt::Return(expr) => Ok(HirStmt::Return(self.lower_expr(expr)?)),
            ResolvedStmt::Function { .. } => Err(unsupported("nested function declarations")),
            _ => Err(unsupported(
                "statement kind is not part of the initial HIR slice",
            )),
        }
    }

    fn lower_expr(&mut self, expr: &ResolvedExpr) -> Result<HirExpr, Diagnostic> {
        match expr {
            ResolvedExpr::Number(value) => Ok(HirExpr::ConstNumber(*value)),
            ResolvedExpr::String(value) => Ok(HirExpr::ConstString(value.clone())),
            ResolvedExpr::Bool(value) => Ok(HirExpr::ConstBool(*value)),
            ResolvedExpr::Null => Ok(HirExpr::ConstNull),
            ResolvedExpr::Undefined => Ok(HirExpr::ConstUndefined),
            ResolvedExpr::Ident(name) => match self.resolve_local(name) {
                Ok(local) => Ok(HirExpr::LoadLocal(local)),
                Err(_) => Ok(HirExpr::LoadBuiltin(name.clone())),
            },
            ResolvedExpr::Unary {
                op: UnaryOp::Not,
                expr,
            } => Ok(HirExpr::JsUnaryNot(Box::new(self.lower_expr(expr)?))),
            ResolvedExpr::Binary { left, op, right } => self.lower_binary(left, *op, right),
            ResolvedExpr::BuiltinCall { builtin, args } => Ok(HirExpr::CallBuiltin {
                builtin: *builtin,
                args: self.lower_args(args)?,
            }),
            ResolvedExpr::BuiltinProperty { builtin, object } => match builtin {
                BuiltinPropertyId::Length => {
                    Ok(HirExpr::ArrayLength(Box::new(self.lower_expr(object)?)))
                }
            },
            ResolvedExpr::PropertyAccess { object, key } => Ok(HirExpr::GetProp {
                object: Box::new(self.lower_expr(object)?),
                key: key.clone(),
            }),
            ResolvedExpr::ComputedIndex { object, index } => Ok(HirExpr::GetIndex {
                object: Box::new(self.lower_expr(object)?),
                index: Box::new(self.lower_expr(index)?),
            }),
            ResolvedExpr::Call { callee, args, .. } => match callee.as_ref() {
                ResolvedExpr::Ident(name) => {
                    let function =
                        self.function_ids
                            .get(name.as_str())
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnresolvedFunction,
                                message: format!("unresolved function: `{name}`"),
                                span: None,
                            })?;
                    Ok(HirExpr::CallFunction {
                        function: *function,
                        args: self.lower_args(args)?,
                    })
                }
                _ => Err(unsupported("dynamic function calls in initial HIR slice")),
            },
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => Ok(HirExpr::CallMethod {
                receiver: Box::new(self.lower_expr(object)?),
                method: method.clone(),
                args: self.lower_args(args)?,
            }),
            ResolvedExpr::Assign { .. }
            | ResolvedExpr::LogicalAssign { .. }
            | ResolvedExpr::LogicalPropertyAssign { .. }
            | ResolvedExpr::LogicalMemberAssign { .. }
            | ResolvedExpr::LogicalComputedPropertyAssign { .. } => Err(unsupported(
                "assignment expressions are not part of the initial HIR slice",
            )),
            _ => Err(unsupported(
                "expression kind is not part of the initial HIR slice",
            )),
        }
    }

    fn lower_binary(
        &mut self,
        left: &ResolvedExpr,
        op: BinaryOp,
        right: &ResolvedExpr,
    ) -> Result<HirExpr, Diagnostic> {
        let left = Box::new(self.lower_expr(left)?);
        let right = Box::new(self.lower_expr(right)?);
        match op {
            BinaryOp::Add => Ok(HirExpr::JsAdd { left, right }),
            BinaryOp::StrictEqual => Ok(HirExpr::JsStrictEqual { left, right }),
            BinaryOp::EqualEqual => Ok(HirExpr::JsAbstractEqual { left, right }),
            BinaryOp::Less => Ok(HirExpr::JsRelational {
                op: HirRelationalOp::Less,
                left,
                right,
            }),
            BinaryOp::LessEqual => Ok(HirExpr::JsRelational {
                op: HirRelationalOp::LessEqual,
                left,
                right,
            }),
            BinaryOp::Greater => Ok(HirExpr::JsRelational {
                op: HirRelationalOp::Greater,
                left,
                right,
            }),
            BinaryOp::GreaterEqual => Ok(HirExpr::JsRelational {
                op: HirRelationalOp::GreaterEqual,
                left,
                right,
            }),
            _ => Err(unsupported(
                "binary operator is not part of the initial HIR slice",
            )),
        }
    }

    fn lower_args(&mut self, args: &[ResolvedExpr]) -> Result<Vec<HirExpr>, Diagnostic> {
        args.iter().map(|arg| self.lower_expr(arg)).collect()
    }

    fn declare_local(&mut self, name: &str) -> Result<HirLocalId, Diagnostic> {
        if self
            .scopes
            .last()
            .expect("scope must exist")
            .contains_key(name)
        {
            return Err(Diagnostic {
                code: DiagCode::DuplicateLocal,
                message: format!("duplicate local binding: `{name}`"),
                span: None,
            });
        }
        let local = HirLocalId(self.locals.len());
        self.locals.push(local);
        self.scopes
            .last_mut()
            .expect("scope must exist")
            .insert(name.to_owned(), local);
        Ok(local)
    }

    fn resolve_local(&self, name: &str) -> Result<HirLocalId, Diagnostic> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedName,
                message: format!("unresolved name: `{name}`"),
                span: None,
            })
    }
}

fn unsupported(message: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: message.to_owned(),
        span: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_to_hir(source: &str) -> HirProgram {
        let tokens = ts2wasm_frontend::Lexer::new(source).tokenize().unwrap();
        let ast = ts2wasm_frontend::Parser::new(tokens)
            .parse_program()
            .unwrap();
        let named = crate::name_resolver::resolve_names(&ast).unwrap();
        let resolved = crate::builtin_resolver::resolve_builtins(&named).unwrap();
        lower_to_hir(&resolved).unwrap()
    }

    #[test]
    fn lowers_addition_to_js_add() {
        let hir = parse_to_hir("let a = 1; let b = 2; let c = a + b;");
        let HirStmt::Let { init, .. } = &hir.body[2] else {
            panic!("expected third statement to be let");
        };
        assert!(matches!(init, HirExpr::JsAdd { .. }));
    }

    #[test]
    fn lowers_if_condition_to_truthy_branch() {
        let hir = parse_to_hir("let a = 1; if (a) { console.log(\"yes\"); }");
        assert!(matches!(
            &hir.body[1],
            HirStmt::BranchIfTruthy {
                condition: HirExpr::ToBoolean(_),
                ..
            }
        ));
    }

    #[test]
    fn lowers_console_log_to_builtin_call() {
        let hir = parse_to_hir("console.log(\"ok\");");
        assert!(matches!(
            &hir.body[0],
            HirStmt::Expr(HirExpr::CallBuiltin {
                builtin: BuiltinId::ConsoleLog,
                ..
            })
        ));
    }

    #[test]
    fn validates_valid_hir() {
        let hir = parse_to_hir("let a = 1; if (a) { console.log(\"yes\"); }");
        validate_hir(&hir).unwrap();
    }

    #[test]
    fn validation_rejects_invalid_local_id() {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::LoadLocal(HirLocalId(99)))],
            locals: vec![],
            functions: vec![],
        };
        let errors = validate_hir(&hir).unwrap_err();
        assert!(errors.iter().any(|error| {
            error.code == DiagCode::InvariantViolation && error.message.contains("local id")
        }));
    }

    #[test]
    fn validation_rejects_branch_without_to_boolean() {
        let hir = HirProgram {
            body: vec![HirStmt::BranchIfTruthy {
                condition: HirExpr::ConstBool(true),
                then_body: vec![],
                else_body: vec![],
            }],
            locals: vec![],
            functions: vec![],
        };
        let errors = validate_hir(&hir).unwrap_err();
        assert!(errors.iter().any(|error| {
            error.code == DiagCode::InvariantViolation && error.message.contains("ToBoolean")
        }));
    }

    #[test]
    fn validation_rejects_invalid_function_id() {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::CallFunction {
                function: HirFunctionId(7),
                args: vec![],
            })],
            locals: vec![],
            functions: vec![],
        };
        let errors = validate_hir(&hir).unwrap_err();
        assert!(errors.iter().any(|error| {
            error.code == DiagCode::InvariantViolation && error.message.contains("function id")
        }));
    }
}
