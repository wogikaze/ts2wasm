use ts2wasm_shared::{BinaryOp, LogicalAssignOp, UnaryOp};
use ts2wasm_source::Span;

use super::binding_pattern::BindingPattern;
use super::builtin::{BuiltinId, BuiltinPropertyId};

pub type ResolvedConstructor = (Vec<ResolvedParam>, Vec<ResolvedStmt>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedParam {
    pub name: String,
    pub default: Option<ResolvedExpr>,
    pub is_rest: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedStmt {
    AmbientValue(String),
    Let(String, ResolvedExpr),
    DestructureLet {
        pattern: BindingPattern,
        expr: ResolvedExpr,
    },
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
        params: Vec<ResolvedParam>,
        body: Vec<ResolvedStmt>,
        is_generator: bool,
        is_async: bool,
        is_ambient: bool,
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
    Labeled {
        label: String,
        body: Box<ResolvedStmt>,
    },
    Break {
        label: Option<String>,
    },
    Continue {
        label: Option<String>,
    },
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
        constructor: Option<ResolvedConstructor>,
        methods: Vec<ClassMethod>,
        statics: Vec<(String, ResolvedExpr)>,
        static_blocks: Vec<(Span, Vec<ResolvedStmt>)>,
        private_fields: Vec<String>,
        static_private_fields: Vec<(String, ResolvedExpr, Span)>,
    },
    Block {
        statements: Vec<ResolvedStmt>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassMethod {
    pub name: String,
    pub kind: ClassMethodKind,
    pub params: Vec<ResolvedParam>,
    pub body: Vec<ResolvedStmt>,
    pub captures: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassMethodKind {
    Method,
    Getter,
    Setter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedExpr {
    Number(i32),
    BigIntLiteral {
        decimal: String,
        sign: i32,
        limb_low: u32,
        limb_high: u32,
    },
    String(String),
    Bool(bool),
    Null,
    Undefined,
    This {
        span: Span,
    },
    NewTarget {
        span: Span,
    },
    Await {
        expr: Box<ResolvedExpr>,
    },
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
    Ternary {
        condition: Box<ResolvedExpr>,
        then_expr: Box<ResolvedExpr>,
        else_expr: Box<ResolvedExpr>,
        span: Span,
    },
    Call {
        callee: Box<ResolvedExpr>,
        args: Vec<ResolvedExpr>,
        span: Span,
    },
    Assign {
        name: String,
        expr: Box<ResolvedExpr>,
    },
    LogicalAssign {
        name: String,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    LogicalPropertyAssign {
        object: String,
        key: String,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    LogicalComputedPropertyAssign {
        object: String,
        key: Box<ResolvedExpr>,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    LogicalComputedMemberAssign {
        object: Box<ResolvedExpr>,
        key: Box<ResolvedExpr>,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    LogicalMemberAssign {
        object: Box<ResolvedExpr>,
        key: String,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    Array(Vec<ResolvedArrayElement>),
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
        span: Span,
    },
    PropertyAccess {
        object: Box<ResolvedExpr>,
        key: String,
        span: Span,
    },
    OptionalPropertyAccess {
        object: Box<ResolvedExpr>,
        key: String,
        span: Span,
    },
    OptionalComputedIndex {
        object: Box<ResolvedExpr>,
        index: Box<ResolvedExpr>,
        span: Span,
    },
    OptionalCall {
        callee: Box<ResolvedExpr>,
        args: Vec<ResolvedExpr>,
        span: Span,
    },
    MethodCall {
        object: Box<ResolvedExpr>,
        method: String,
        args: Vec<ResolvedExpr>,
        span: Span,
    },
    PropertyAssign {
        object: Box<ResolvedExpr>,
        key: String,
        value: Box<ResolvedExpr>,
        span: Span,
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
        span: Span,
    },
    ModuleLoad {
        specifier: String,
    },
    ArrowFn {
        params: Vec<String>,
        body: Box<ResolvedExpr>,
        body_stmts: Vec<ResolvedStmt>,
    },
    FunctionExpr {
        name: String,
        params: Vec<ResolvedParam>,
        body: Vec<ResolvedStmt>,
    },
    ClassExpr {
        name: String,
        body: Vec<ResolvedStmt>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedArrayElement {
    Present(ResolvedExpr),
    Hole,
}
