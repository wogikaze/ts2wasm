use ts2wasm_frontend::{BinaryOp, UnaryOp};

use super::builtin::{BuiltinId, BuiltinPropertyId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedStmt {
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
        params: Vec<(String, Option<ResolvedExpr>, bool)>,
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
    Export {
        name: String,
        expr: Box<ResolvedExpr>,
    },
    ModuleExportsAssign {
        expr: Box<ResolvedExpr>,
    },
    ClassDecl {
        name: String,
        extends: Option<String>,
        constructor: Option<(Vec<(String, Option<ResolvedExpr>, bool)>, Vec<ResolvedStmt>)>,
        methods: Vec<ClassMethod>,
        statics: Vec<(String, ResolvedExpr)>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<(String, Option<ResolvedExpr>, bool)>,
    pub body: Vec<ResolvedStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedExpr {
    Number(i32),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    This,
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
    Spread(Box<ResolvedExpr>),
    PropertyAssignDynamic {
        object: Box<ResolvedExpr>,
        key: Box<ResolvedExpr>,
        value: Box<ResolvedExpr>,
    },
    New {
        class_name: String,
        args: Vec<ResolvedExpr>,
    },
    ModuleLoad {
        specifier: String,
    },
    ArrowFn {
        params: Vec<String>,
        body: Box<ResolvedExpr>,
    },
}
