use crate::{BinaryOp, Expr, Stmt, UnaryOp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoweredProgram {
    pub(crate) statements: Vec<LoweredStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoweredStmt {
    Let(String, LoweredExpr),
    Assign(String, LoweredExpr),
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
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<LoweredStmt>,
    },
    Return(LoweredExpr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoweredExpr {
    Number(i32),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    Ident(String),
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
        name: String,
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
    LoweredProgram {
        statements: program.iter().map(lower_stmt).collect(),
    }
}

fn lower_stmt(stmt: &Stmt) -> LoweredStmt {
    match stmt {
        Stmt::Let(name, expr) => LoweredStmt::Let(name.clone(), lower_expr(expr)),
        Stmt::Assign(name, expr) => LoweredStmt::Assign(name.clone(), lower_expr(expr)),
        Stmt::ConsoleLog(expr) => LoweredStmt::ConsoleLog(lower_expr(expr)),
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => LoweredStmt::If {
            condition: lower_expr(condition),
            then_body: then_body.iter().map(lower_stmt).collect(),
            else_body: else_body.iter().map(lower_stmt).collect(),
        },
        Stmt::While { condition, body } => LoweredStmt::While {
            condition: lower_expr(condition),
            body: body.iter().map(lower_stmt).collect(),
        },
        Stmt::Function { name, params, body } => LoweredStmt::Function {
            name: name.clone(),
            params: params.clone(),
            body: body.iter().map(lower_stmt).collect(),
        },
        Stmt::Return(expr) => LoweredStmt::Return(lower_expr(expr)),
    }
}

fn lower_expr(expr: &Expr) -> LoweredExpr {
    match expr {
        Expr::Number(value) => LoweredExpr::Number(*value),
        Expr::String(value) => LoweredExpr::String(value.clone()),
        Expr::Bool(value) => LoweredExpr::Bool(*value),
        Expr::Null => LoweredExpr::Null,
        Expr::Undefined => LoweredExpr::Undefined,
        Expr::Ident(name) => LoweredExpr::Ident(name.clone()),
        Expr::Unary { op, expr } => LoweredExpr::Unary {
            op: lower_unary_op(*op),
            expr: Box::new(lower_expr(expr)),
        },
        Expr::Binary { left, op, right } => LoweredExpr::Binary {
            left: Box::new(lower_expr(left)),
            op: lower_binary_op(*op),
            right: Box::new(lower_expr(right)),
        },
        Expr::Call { name, args } => LoweredExpr::Call {
            name: name.clone(),
            args: args.iter().map(lower_expr).collect(),
        },
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
