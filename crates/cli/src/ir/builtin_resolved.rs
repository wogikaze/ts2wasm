use crate::{BinaryOp, UnaryOp};

use super::builtin::{BuiltinId, BuiltinPropertyId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedStmt {
    Let(String, ResolvedExpr),
    Assign(String, ResolvedExpr),
    Expr(ResolvedExpr),
    If {
        condition: ResolvedExpr,
        then_body: Vec<ResolvedStmt>,
        else_body: Vec<ResolvedStmt>,
    },
    While {
        condition: ResolvedExpr,
        body: Vec<ResolvedStmt>,
    },
    Return(ResolvedExpr),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<ResolvedStmt>,
    },
    TryCatch {
        try_block: Vec<ResolvedStmt>,
        catch_param: Option<String>,
        catch_block: Option<Vec<ResolvedStmt>>,
        finally_block: Option<Vec<ResolvedStmt>>,
    },
    Throw(ResolvedExpr),
    Switch {
        expr: ResolvedExpr,
        cases: Vec<(Option<ResolvedExpr>, Vec<ResolvedStmt>)>,
    },
    DoWhile {
        body: Vec<ResolvedStmt>,
        condition: ResolvedExpr,
    },
    For {
        init: Option<Box<ResolvedStmt>>,
        condition: Option<ResolvedExpr>,
        update: Option<ResolvedExpr>,
        body: Vec<ResolvedStmt>,
    },
    ForIn {
        var: String,
        iter: ResolvedExpr,
        body: Vec<ResolvedStmt>,
    },
    ForOf {
        var: String,
        iter: ResolvedExpr,
        body: Vec<ResolvedStmt>,
    },
    Break,
    Continue,
    ClassDecl {
        name: String,
        extends: Option<String>,
        constructor: Option<(Vec<String>, Vec<ResolvedStmt>)>,
        methods: Vec<ClassMethod>,
        statics: Vec<(String, ResolvedExpr)>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClassMethod {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Vec<ResolvedStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedExpr {
    Number(i32),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    Ident(String),
    Unary {
        op: UnaryOp,
        expr: Box<ResolvedExpr>,
    },
    Binary {
        left: Box<ResolvedExpr>,
        op: BinaryOp,
        right: Box<ResolvedExpr>,
    },
    Call {
        callee: Box<ResolvedExpr>,
        args: Vec<ResolvedExpr>,
    },
    Array(Vec<ResolvedExpr>),
    Object(Vec<(String, ResolvedExpr)>),
    ComputedIndex {
        object: Box<ResolvedExpr>,
        index: Box<ResolvedExpr>,
    },
    BuiltinCall {
        builtin: BuiltinId,
        args: Vec<ResolvedExpr>,
    },
    BuiltinProperty {
        builtin: BuiltinPropertyId,
        object: Box<ResolvedExpr>,
    },
    PropertyAccess {
        object: Box<ResolvedExpr>,
        key: String,
    },
    MethodCall {
        object: Box<ResolvedExpr>,
        method: String,
        args: Vec<ResolvedExpr>,
    },
    PropertyAssign {
        object: Box<ResolvedExpr>,
        key: String,
        value: Box<ResolvedExpr>,
    },
    New {
        class_name: String,
        args: Vec<ResolvedExpr>,
    },
}
