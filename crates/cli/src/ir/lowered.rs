use std::collections::HashMap;

use crate::{BinaryOp, Expr, Stmt, UnaryOp};

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
    ConsoleLog(LoweredExpr),
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

pub(crate) fn lower_program(program: &[Stmt]) -> LoweredProgram {
    let function_ids = collect_function_ids(program);
    let mut resolver = Resolver::new(&function_ids);
    let mut top_level_statements = Vec::new();
    let mut functions = Vec::new();

    for stmt in program {
        match stmt {
            Stmt::Function { name, params, body } => {
                let func_id = function_ids[name];
                functions.push(lower_function(func_id, params, body, &function_ids));
            }
            _ => top_level_statements.push(resolver.lower_stmt(stmt)),
        }
    }

    LoweredProgram {
        top_level_statements,
        top_level_locals: resolver.locals,
        functions,
    }
}

fn collect_function_ids(program: &[Stmt]) -> HashMap<String, FuncId> {
    let mut function_ids = HashMap::new();
    let mut next_func_id = 0;

    for stmt in program {
        if let Stmt::Function { name, .. } = stmt {
            function_ids.insert(name.clone(), FuncId(next_func_id));
            next_func_id += 1;
        }
    }

    function_ids
}

fn lower_function(
    id: FuncId,
    params: &[String],
    body: &[Stmt],
    function_ids: &HashMap<String, FuncId>,
) -> LoweredFunction {
    let (mut resolver, param_ids) = Resolver::with_params(function_ids, params);
    let body = resolver.lower_block(body);

    LoweredFunction {
        id,
        params: param_ids,
        locals: resolver.locals,
        body,
    }
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
    ) -> (Self, Vec<LocalId>) {
        let mut resolver = Self {
            function_ids,
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            locals: Vec::new(),
        };
        let mut param_ids = Vec::new();

        for param in params {
            let local_id = LocalId(resolver.next_local_id);
            resolver.next_local_id += 1;
            resolver
                .scopes
                .last_mut()
                .expect("function scope must exist")
                .insert(param.clone(), local_id);
            param_ids.push(local_id);
        }

        (resolver, param_ids)
    }

    fn lower_block(&mut self, statements: &[Stmt]) -> Vec<LoweredStmt> {
        let mut lowered = Vec::with_capacity(statements.len());
        for statement in statements {
            lowered.push(self.lower_stmt(statement));
        }
        lowered
    }

    fn lower_nested_block(&mut self, statements: &[Stmt]) -> Vec<LoweredStmt> {
        self.scopes.push(HashMap::new());
        let lowered = self.lower_block(statements);
        self.scopes.pop();
        lowered
    }

    fn lower_stmt(&mut self, stmt: &Stmt) -> LoweredStmt {
        match stmt {
            Stmt::Let(name, expr) => {
                let expr = self.lower_expr(expr);
                let local_id = self.declare_local(name);
                LoweredStmt::Let(local_id, expr)
            }
            Stmt::Assign(name, expr) => {
                let local_id = self.resolve_local(name);
                LoweredStmt::Assign(local_id, self.lower_expr(expr))
            }
            Stmt::ConsoleLog(expr) => LoweredStmt::ConsoleLog(self.lower_expr(expr)),
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => LoweredStmt::If {
                condition: self.lower_expr(condition),
                then_body: self.lower_nested_block(then_body),
                else_body: self.lower_nested_block(else_body),
            },
            Stmt::While { condition, body } => LoweredStmt::While {
                condition: self.lower_expr(condition),
                body: self.lower_nested_block(body),
            },
            Stmt::Return(expr) => LoweredStmt::Return(self.lower_expr(expr)),
            Stmt::Function { .. } => {
                panic!("function declarations must be split before lowering statements")
            }
        }
    }

    fn lower_expr(&self, expr: &Expr) -> LoweredExpr {
        match expr {
            Expr::Number(value) => LoweredExpr::Number(*value),
            Expr::String(value) => LoweredExpr::String(value.clone()),
            Expr::Bool(value) => LoweredExpr::Bool(*value),
            Expr::Null => LoweredExpr::Null,
            Expr::Undefined => LoweredExpr::Undefined,
            Expr::Ident(name) => LoweredExpr::Local(self.resolve_local(name)),
            Expr::Unary { op, expr } => LoweredExpr::Unary {
                op: lower_unary_op(*op),
                expr: Box::new(self.lower_expr(expr)),
            },
            Expr::Binary { left, op, right } => LoweredExpr::Binary {
                left: Box::new(self.lower_expr(left)),
                op: lower_binary_op(*op),
                right: Box::new(self.lower_expr(right)),
            },
            Expr::Call { name, args } => LoweredExpr::Call {
                kind: FunctionCallKind::User(self.resolve_func(name)),
                args: args.iter().map(|arg| self.lower_expr(arg)).collect(),
            },
        }
    }

    fn declare_local(&mut self, name: &str) -> LocalId {
        let local_id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(local_id);
        self.scopes
            .last_mut()
            .expect("scope must exist")
            .insert(name.to_owned(), local_id);
        local_id
    }

    fn resolve_local(&self, name: &str) -> LocalId {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
            .unwrap_or_else(|| panic!("unresolved local during lowering: {name}"))
    }

    fn resolve_func(&self, name: &str) -> FuncId {
        self.function_ids
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("unresolved function during lowering: {name}"))
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionCallKind, LoweredExpr, LoweredStmt, lower_program};

    #[test]
    fn lowering_splits_functions_and_resolves_ids() {
        let program = crate::parse_program(
            "function add(a, b) { return a + b; } let x = 1; console.log(add(x, 2));",
        )
        .unwrap();

        let lowered = lower_program(&program);

        assert_eq!(lowered.functions.len(), 1);
        assert_eq!(lowered.top_level_statements.len(), 2);
        assert_eq!(lowered.top_level_locals.len(), 1);

        match &lowered.top_level_statements[1] {
            LoweredStmt::ConsoleLog(LoweredExpr::Call { kind, args }) => {
                assert!(matches!(kind, FunctionCallKind::User(_)));
                assert!(matches!(args[0], LoweredExpr::Local(_)));
            }
            other => panic!("unexpected lowered statement: {other:?}"),
        }
    }
}
