use std::collections::HashMap;

use crate::runtime::value::ValueTag;
use crate::{BinaryOp, DiagCode, Diagnostic, Expr, Stmt, UnaryOp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct FuncId(pub(crate) usize);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum BuiltinId {
    ConsoleLog,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoweredProgram {
    pub(crate) top_level_statements: Vec<LoweredStmt>,
    pub(crate) top_level_locals: Vec<LocalId>,
    pub(crate) functions: Vec<LoweredFunction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoweredFunction {
    pub(crate) id: FuncId,
    pub(crate) params: Vec<LocalId>,
    pub(crate) locals: Vec<LocalId>,
    pub(crate) body: Vec<LoweredStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoweredStmt {
    Let(LocalId, LoweredExpr),
    Assign(LocalId, LoweredExpr),
    Expr(LoweredExpr),
    If {
        condition: LoweredExpr,
        then_body: Vec<LoweredStmt>,
        else_body: Vec<LoweredStmt>,
    },
    While {
        condition: LoweredExpr,
        body: Vec<LoweredStmt>,
    },
    Return(LoweredExpr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionCallKind {
    User(FuncId),
    #[allow(dead_code)]
    Builtin(BuiltinId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoweredExpr {
    Number(i32),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    Local(LocalId),
    Unary {
        op: LoweredUnaryOp,
        expr: Box<LoweredExpr>,
    },
    Binary {
        left: Box<LoweredExpr>,
        op: LoweredBinaryOp,
        right: Box<LoweredExpr>,
    },
    Call {
        kind: FunctionCallKind,
        args: Vec<LoweredExpr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoweredBinaryOp {
    Add,
    Subtract,
    Less,
    StrictEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoweredUnaryOp {
    Not,
}

pub(crate) fn lower_program(program: &[Stmt]) -> Result<LoweredProgram, Diagnostic> {
    let function_ids = collect_function_ids(program)?;
    let mut resolver = Resolver::new(&function_ids);
    let mut top_level_statements = Vec::new();
    let mut functions = Vec::new();

    for stmt in program {
        match stmt {
            Stmt::Function { name, params, body } => {
                let func_id = function_ids[name];
                functions.push(lower_function(func_id, params, body, &function_ids)?);
            }
            _ => top_level_statements.push(resolver.lower_stmt(stmt)?),
        }
    }

    Ok(LoweredProgram {
        top_level_statements,
        top_level_locals: resolver.locals,
        functions,
    })
}

fn collect_function_ids(program: &[Stmt]) -> Result<HashMap<String, FuncId>, Diagnostic> {
    let mut function_ids = HashMap::new();
    let mut next_func_id = 0;

    for stmt in program {
        if let Stmt::Function { name, .. } = stmt {
            if function_ids.contains_key(name.as_str()) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateFunction,
                    message: format!("duplicate function definition: `{name}`"),
                    span: None,
                });
            }
            function_ids.insert(name.clone(), FuncId(next_func_id));
            next_func_id += 1;
        }
    }

    Ok(function_ids)
}

fn lower_function(
    id: FuncId,
    params: &[String],
    body: &[Stmt],
    function_ids: &HashMap<String, FuncId>,
) -> Result<LoweredFunction, Diagnostic> {
    let (mut resolver, param_ids) = Resolver::with_params(function_ids, params)?;
    let body = resolver.lower_block(body)?;

    Ok(LoweredFunction {
        id,
        params: param_ids,
        locals: resolver.locals,
        body,
    })
}

fn lower_binary_op(op: BinaryOp) -> LoweredBinaryOp {
    match op {
        BinaryOp::Add => LoweredBinaryOp::Add,
        BinaryOp::Subtract => LoweredBinaryOp::Subtract,
        BinaryOp::Less => LoweredBinaryOp::Less,
        BinaryOp::StrictEqual => LoweredBinaryOp::StrictEqual,
    }
}

fn lower_unary_op(op: UnaryOp) -> LoweredUnaryOp {
    match op {
        UnaryOp::Not => LoweredUnaryOp::Not,
    }
}

struct Resolver<'a> {
    function_ids: &'a HashMap<String, FuncId>,
    scopes: Vec<HashMap<String, LocalId>>,
    next_local_id: usize,
    locals: Vec<LocalId>,
}

impl<'a> Resolver<'a> {
    fn new(function_ids: &'a HashMap<String, FuncId>) -> Self {
        Self {
            function_ids,
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            locals: Vec::new(),
        }
    }

    fn with_params(
        function_ids: &'a HashMap<String, FuncId>,
        params: &[String],
    ) -> Result<(Self, Vec<LocalId>), Diagnostic> {
        let mut resolver = Self {
            function_ids,
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            locals: Vec::new(),
        };
        let mut param_ids = Vec::new();
        let mut seen_params = HashMap::new();

        for param in params {
            if seen_params.contains_key(param) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateParameter,
                    message: format!("duplicate parameter name: `{param}`"),
                    span: None,
                });
            }
            seen_params.insert(param.clone(), ());
            let local_id = LocalId(resolver.next_local_id);
            resolver.next_local_id += 1;
            resolver
                .scopes
                .last_mut()
                .expect("function scope must exist")
                .insert(param.clone(), local_id);
            param_ids.push(local_id);
        }

        Ok((resolver, param_ids))
    }

    fn lower_block(&mut self, statements: &[Stmt]) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let mut lowered = Vec::with_capacity(statements.len());
        for statement in statements {
            lowered.push(self.lower_stmt(statement)?);
        }
        Ok(lowered)
    }

    fn lower_nested_block(&mut self, statements: &[Stmt]) -> Result<Vec<LoweredStmt>, Diagnostic> {
        self.scopes.push(HashMap::new());
        let lowered = self.lower_block(statements);
        self.scopes.pop();
        lowered
    }

    fn lower_stmt(&mut self, stmt: &Stmt) -> Result<LoweredStmt, Diagnostic> {
        match stmt {
            Stmt::Let(name, expr) => {
                let expr = self.lower_expr(expr)?;
                let local_id = self.declare_local(name)?;
                Ok(LoweredStmt::Let(local_id, expr))
            }
            Stmt::Assign(name, expr) => {
                let local_id = self.resolve_local(name)?;
                Ok(LoweredStmt::Assign(local_id, self.lower_expr(expr)?))
            }
            Stmt::Expr(expr) => {
                if let Some(args) = Self::as_console_log_call(expr) {
                    let lowered_args = args
                        .iter()
                        .map(|arg| self.lower_expr(arg))
                        .collect::<Result<Vec<_>, _>>()?;
                    if lowered_args.len() != 1 {
                        return Err(Diagnostic {
                            code: DiagCode::ArityMismatch,
                            message: format!(
                                "console.log expects 1 argument in this milestone, got {}",
                                lowered_args.len()
                            ),
                            span: None,
                        });
                    }
                    return Ok(LoweredStmt::Expr(LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: lowered_args,
                    }));
                }
                Ok(LoweredStmt::Expr(self.lower_expr(expr)?))
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => Ok(LoweredStmt::If {
                condition: self.lower_expr(condition)?,
                then_body: self.lower_nested_block(then_body)?,
                else_body: self.lower_nested_block(else_body)?,
            }),
            Stmt::While { condition, body } => Ok(LoweredStmt::While {
                condition: self.lower_expr(condition)?,
                body: self.lower_nested_block(body)?,
            }),
            Stmt::Return(expr) => Ok(LoweredStmt::Return(self.lower_expr(expr)?)),
            Stmt::Function { .. } => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "nested function declarations are not supported in this milestone"
                    .to_owned(),
                span: None,
            }),
        }
    }

    fn lower_expr(&self, expr: &Expr) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            Expr::Number(value) => Ok(LoweredExpr::Number(*value)),
            Expr::String(value) => Ok(LoweredExpr::String(value.clone())),
            Expr::Bool(value) => Ok(LoweredExpr::Bool(*value)),
            Expr::Null => Ok(LoweredExpr::Null),
            Expr::Undefined => Ok(LoweredExpr::Undefined),
            Expr::Ident(name) => Ok(LoweredExpr::Local(self.resolve_local(name)?)),
            Expr::Unary { op, expr } => Ok(LoweredExpr::Unary {
                op: lower_unary_op(*op),
                expr: Box::new(self.lower_expr(expr)?),
            }),
            Expr::Binary { left, op, right } => Ok(LoweredExpr::Binary {
                left: Box::new(self.lower_expr(left)?),
                op: lower_binary_op(*op),
                right: Box::new(self.lower_expr(right)?),
            }),
            Expr::Call { callee, args } => {
                let func_name = match callee.as_ref() {
                    Expr::Ident(name) => name,
                    _ => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "only identifier calls are supported in expression context"
                                .to_owned(),
                            span: None,
                        });
                    }
                };
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(self.resolve_func(func_name)?),
                    args: args
                        .iter()
                        .map(|arg| self.lower_expr(arg))
                        .collect::<Result<Vec<_>, _>>()?,
                })
            }
            Expr::Member { .. } => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "member expressions are only supported as console.log(...) callee"
                    .to_owned(),
                span: None,
            }),
        }
    }

    fn as_console_log_call(expr: &Expr) -> Option<&[Expr]> {
        let Expr::Call { callee, args } = expr else {
            return None;
        };
        let Expr::Member { object, property } = callee.as_ref() else {
            return None;
        };
        let Expr::Ident(object_name) = object.as_ref() else {
            return None;
        };
        if object_name == "console" && property == "log" {
            Some(args)
        } else {
            None
        }
    }

    fn declare_local(&mut self, name: &str) -> Result<LocalId, Diagnostic> {
        let scope = self.scopes.last_mut().expect("scope must exist");
        if scope.contains_key(name) {
            return Err(Diagnostic {
                code: DiagCode::DuplicateLocal,
                message: format!("duplicate local binding: `{name}`"),
                span: None,
            });
        }
        let local_id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(local_id);
        scope.insert(name.to_owned(), local_id);
        Ok(local_id)
    }

    fn resolve_local(&self, name: &str) -> Result<LocalId, Diagnostic> {
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

    fn resolve_func(&self, name: &str) -> Result<FuncId, Diagnostic> {
        self.function_ids
            .get(name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedFunction,
                message: format!("unresolved function: `{name}`"),
                span: None,
            })
    }
}

/// Validate the structural invariants of a `LoweredProgram`.
///
/// This gate must pass before the program is handed to the backend.
/// It catches:
/// - `FuncId` values that are out of range
/// - `LocalId` values that are out of range for their enclosing scope
/// - Call arity mismatches
///
/// See `docs/14-ir-contracts.md` § validate_lowered.
pub(crate) fn validate_lowered(program: &LoweredProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let num_funcs = program.functions.len();

    validate_stmts(
        &program.top_level_statements,
        program.top_level_locals.len(),
        num_funcs,
        program,
        &mut errors,
    );

    for func in &program.functions {
        let local_count = func.params.len() + func.locals.len();
        validate_stmts(&func.body, local_count, num_funcs, program, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_stmts(
    stmts: &[LoweredStmt],
    local_count: usize,
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
) {
    for stmt in stmts {
        validate_stmt(stmt, local_count, num_funcs, program, errors);
    }
}

fn validate_stmt(
    stmt: &LoweredStmt,
    local_count: usize,
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
) {
    match stmt {
        LoweredStmt::Let(id, expr) | LoweredStmt::Assign(id, expr) => {
            check_local_id(*id, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors);
        }
        LoweredStmt::Expr(expr) | LoweredStmt::Return(expr) => {
            validate_expr(expr, local_count, num_funcs, program, errors);
        }
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            validate_expr(condition, local_count, num_funcs, program, errors);
            validate_stmts(then_body, local_count, num_funcs, program, errors);
            validate_stmts(else_body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::While { condition, body } => {
            validate_expr(condition, local_count, num_funcs, program, errors);
            validate_stmts(body, local_count, num_funcs, program, errors);
        }
    }
}

fn validate_expr(
    expr: &LoweredExpr,
    local_count: usize,
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
) {
    match expr {
        LoweredExpr::Number(n) => {
            if !ValueTag::can_encode_number(*n) {
                errors.push(Diagnostic {
                    code: DiagCode::NumberOutOfRange,
                    message: format!(
                        "number literal {n} is out of M0 tagged-int range ({MIN}..={MAX})",
                        MIN = ValueTag::NUMBER_PAYLOAD_MIN,
                        MAX = ValueTag::NUMBER_PAYLOAD_MAX,
                    ),
                    span: None,
                });
            }
        }
        LoweredExpr::Local(id) => check_local_id(*id, local_count, errors),
        LoweredExpr::Unary { expr, .. } => {
            validate_expr(expr, local_count, num_funcs, program, errors);
        }
        LoweredExpr::Binary { left, right, .. } => {
            validate_expr(left, local_count, num_funcs, program, errors);
            validate_expr(right, local_count, num_funcs, program, errors);
        }
        LoweredExpr::Call { kind, args } => {
            for arg in args {
                validate_expr(arg, local_count, num_funcs, program, errors);
            }
            if let FunctionCallKind::User(func_id) = kind {
                if func_id.0 >= num_funcs {
                    errors.push(Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "FuncId {} is out of range (program has {} function(s))",
                            func_id.0, num_funcs
                        ),
                        span: None,
                    });
                } else {
                    let expected = program.functions[func_id.0].params.len();
                    if args.len() != expected {
                        errors.push(Diagnostic {
                            code: DiagCode::ArityMismatch,
                            message: format!(
                                "function {} expects {} argument(s), got {}",
                                func_id.0,
                                expected,
                                args.len()
                            ),
                            span: None,
                        });
                    }
                }
            }
        }
        _ => {}
    }
}

fn check_local_id(id: LocalId, local_count: usize, errors: &mut Vec<Diagnostic>) {
    if id.0 >= local_count {
        errors.push(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "LocalId {} is out of range (scope has {} local(s))",
                id.0, local_count
            ),
            span: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BuiltinId, FunctionCallKind, LoweredExpr, LoweredStmt, lower_program, validate_lowered,
    };

    #[test]
    fn lowering_splits_functions_and_resolves_ids() {
        let program = crate::parse_program(
            "function add(a, b) { return a + b; } let x = 1; console.log(add(x, 2));",
        )
        .unwrap();

        let lowered = lower_program(&program).unwrap();

        assert_eq!(lowered.functions.len(), 1);
        assert_eq!(lowered.top_level_statements.len(), 2);
        assert_eq!(lowered.top_level_locals.len(), 1);

        match &lowered.top_level_statements[1] {
            LoweredStmt::Expr(LoweredExpr::Call { kind, args }) => {
                assert!(matches!(
                    kind,
                    FunctionCallKind::Builtin(BuiltinId::ConsoleLog)
                ));
                assert!(matches!(args[0], LoweredExpr::Call { .. }));
            }
            other => panic!("unexpected lowered statement: {other:?}"),
        }
    }

    #[test]
    fn lowering_rejects_unresolved_name() {
        let program = crate::parse_program("let x = y;").unwrap();
        let err = lower_program(&program).unwrap_err();
        assert_eq!(err.code, super::DiagCode::UnresolvedName);
        assert!(err.message.contains('`'));
    }

    #[test]
    fn lowering_rejects_duplicate_function() {
        let program =
            crate::parse_program("function f() { return 1; } function f() { return 2; }").unwrap();
        let err = lower_program(&program).unwrap_err();
        assert_eq!(err.code, super::DiagCode::DuplicateFunction);
    }

    #[test]
    fn lowering_rejects_duplicate_parameter() {
        let program = crate::parse_program("function f(a, a) { return a; }").unwrap();
        let err = lower_program(&program).unwrap_err();
        assert_eq!(err.code, super::DiagCode::DuplicateParameter);
    }

    #[test]
    fn validate_rejects_arity_mismatch() {
        // Build a program where add(a,b) is called with 3 args by manually
        // constructing a valid-but-wrong-arity LoweredProgram via parse then patch.
        use super::{DiagCode, FuncId, LoweredBinaryOp, LoweredFunction, LoweredProgram};
        use crate::ir::lowered::{LocalId, LoweredExpr};

        let func = LoweredFunction {
            id: FuncId(0),
            params: vec![LocalId(0), LocalId(1)],
            locals: vec![],
            body: vec![],
        };
        let call = LoweredStmt::Expr(LoweredExpr::Call {
            kind: FunctionCallKind::User(FuncId(0)),
            // 3 args instead of the expected 2
            args: vec![
                LoweredExpr::Number(1),
                LoweredExpr::Number(2),
                LoweredExpr::Number(3),
            ],
        });
        let program = LoweredProgram {
            top_level_statements: vec![call],
            top_level_locals: vec![],
            functions: vec![func],
        };

        let errs = validate_lowered(&program).unwrap_err();
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].code, DiagCode::ArityMismatch);
        let _ = LoweredBinaryOp::Add; // suppress dead_code lint in test
    }
}
