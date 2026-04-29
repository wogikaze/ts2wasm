use std::collections::{HashMap, HashSet};

use super::builtin::{BuiltinId, BuiltinPropertyId, BuiltinResult};
use super::builtin_resolved::{ResolvedExpr, ResolvedParam, ResolvedStmt};
use ts2wasm_frontend::{BinaryOp, DiagCode, Diagnostic, LogicalAssignOp, Span, UnaryOp};
use ts2wasm_runtime_abi::ValueTag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncId(pub usize);

type ClassConstructorMap = HashMap<String, FuncId>;
type ClassMethodMap = HashMap<(String, String), FuncId>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassPrototypeRef {
    pub constructor: FuncId,
    pub parent_constructors: Vec<FuncId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltinErrorConstructor {
    Error,
    TypeError,
    ReferenceError,
    SyntaxError,
}

impl BuiltinErrorConstructor {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Error" => Some(Self::Error),
            "TypeError" => Some(Self::TypeError),
            "ReferenceError" => Some(Self::ReferenceError),
            "SyntaxError" => Some(Self::SyntaxError),
            _ => None,
        }
    }

    pub fn parent(self) -> Option<Self> {
        match self {
            Self::Error => None,
            Self::TypeError | Self::ReferenceError | Self::SyntaxError => Some(Self::Error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleInfo {
    pub id: usize,
    pub specifier: String,
    pub statements: Vec<LoweredStmt>,
    pub locals_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredProgram {
    pub top_level_statements: Vec<LoweredStmt>,
    pub top_level_locals: Vec<LocalId>,
    pub functions: Vec<LoweredFunction>,
    pub modules: Vec<ModuleInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredFunction {
    pub id: FuncId,
    pub params: Vec<LocalId>,
    pub uses_receiver: bool,
    pub min_required_params: usize,
    pub rest_param_index: Option<usize>,
    pub locals: Vec<LocalId>,
    pub body: Vec<LoweredStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredStmt {
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
    Throw(LoweredExpr),
    TryCatch {
        try_body: Vec<LoweredStmt>,
        catch_var: Option<LocalId>,
        catch_body: Option<Vec<LoweredStmt>>,
        finally_body: Option<Vec<LoweredStmt>>,
    },
    Switch {
        expr: LoweredExpr,
        cases: Vec<(Option<LoweredExpr>, Vec<LoweredStmt>)>,
    },
    DoWhile {
        body: Vec<LoweredStmt>,
        condition: LoweredExpr,
    },
    For {
        init: Option<Box<LoweredStmt>>,
        condition: Option<LoweredExpr>,
        update: Option<LoweredExpr>,
        body: Vec<LoweredStmt>,
    },
    ForIn {
        var: LocalId,
        iter: LoweredExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<LoweredStmt>,
    },
    ForOf {
        var: LocalId,
        iter: LoweredExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<LoweredStmt>,
    },
    Labeled {
        label: String,
        body: Box<LoweredStmt>,
    },
    Break {
        label: Option<String>,
    },
    Continue {
        label: Option<String>,
    },
    Export {
        name: String,
        expr: LoweredExpr,
    },
    ModuleExportsAssign {
        expr: LoweredExpr,
    },
    ClassDecl {
        name: String,
        extends: Option<String>,
        constructor: Option<(Vec<LocalId>, Vec<LoweredStmt>)>,
        methods: Vec<(String, Vec<LocalId>, Vec<LoweredStmt>)>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionCallKind {
    User(FuncId),
    Builtin(BuiltinId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureRepresentation {
    DirectLocalToken,
    HeapObject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredExpr {
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
    PropertyIn {
        obj: Box<LoweredExpr>,
        key: String,
    },
    PropertyInDynamic {
        obj: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
    },
    Call {
        kind: FunctionCallKind,
        args: Vec<LoweredExpr>,
    },
    Assign {
        local: LocalId,
        expr: Box<LoweredExpr>,
    },
    LogicalAssign {
        local: LocalId,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
    },
    LogicalPropertyAssign {
        object: LocalId,
        key: String,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
    },
    LogicalComputedPropertyAssign {
        object: LocalId,
        key: Box<LoweredExpr>,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
    },
    LogicalComputedMemberAssign {
        object: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
    },
    LogicalMemberAssign {
        object: Box<LoweredExpr>,
        key: String,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
    },
    ArrayNew {
        elements: Vec<LoweredExpr>,
    },
    ArrayGet {
        arr: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
    },
    Index {
        object: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
    },
    GetLength(Box<LoweredExpr>),
    ObjectNew {
        props: Vec<(String, LoweredExpr)>,
    },
    ErrorNew {
        constructor: BuiltinErrorConstructor,
        message: Box<LoweredExpr>,
    },
    PropertyGet {
        obj: Box<LoweredExpr>,
        key: String,
    },
    OptionalPropertyGet {
        obj: Box<LoweredExpr>,
        key: String,
    },
    PropertyGetDynamic {
        obj: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
    },
    OptionalIndex {
        object: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
    },
    OptionalCall {
        callee: Box<LoweredExpr>,
        call: Box<LoweredExpr>,
    },
    MethodCall {
        object: Box<LoweredExpr>,
        method: String,
    },
    RuntimeCall {
        runtime_fn: String,
        args: Vec<LoweredExpr>,
    },
    PropertySet {
        object: Box<LoweredExpr>,
        key: String,
        value: Box<LoweredExpr>,
    },
    PropertyDelete {
        object: Box<LoweredExpr>,
        key: String,
    },
    PropertyDeleteDynamic {
        object: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
    },
    PropertySetDynamic {
        object: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
        value: Box<LoweredExpr>,
    },
    New {
        constructor: FuncId,
        prototype: ClassPrototypeRef,
        args: Vec<LoweredExpr>,
        base_local: LocalId,
    },
    ClassPrototype(ClassPrototypeRef),
    BuiltinErrorPrototype(BuiltinErrorConstructor),
    ModuleLoad {
        module_id: usize,
    },
    This,
    ArrowFn {
        func_id: FuncId,
        captures: Vec<LocalId>,
        representation: ClosureRepresentation,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoweredBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    StrictEqual,
    EqualEqual,
    BangEqual,
    StrictNotEqual,
    And,
    Or,
    NullishCoalesce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoweredUnaryOp {
    Not,
    Negate,
    TypeOf,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoweredLogicalAssignOp {
    And,
    Or,
    Nullish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferredType {
    Number,
    String,
    Boolean,
    Unknown,
}

impl LoweredExpr {
    pub fn inferred_type(&self) -> InferredType {
        match self {
            Self::Number(_) => InferredType::Number,
            Self::BigIntLiteral { .. } => InferredType::Unknown,
            Self::String(_) => InferredType::String,
            Self::Bool(_) => InferredType::Boolean,
            Self::Unary { op, expr } => match op {
                LoweredUnaryOp::Negate if expr.inferred_type() == InferredType::Number => {
                    InferredType::Number
                }
                LoweredUnaryOp::Not => InferredType::Boolean,
                _ => InferredType::Unknown,
            },
            Self::Binary { left, op, right } => match op {
                LoweredBinaryOp::Add => match (left.inferred_type(), right.inferred_type()) {
                    (InferredType::Number, InferredType::Number) => InferredType::Number,
                    (InferredType::String, InferredType::String) => InferredType::String,
                    _ => InferredType::Unknown,
                },
                LoweredBinaryOp::Subtract
                | LoweredBinaryOp::Multiply
                | LoweredBinaryOp::Divide
                | LoweredBinaryOp::Modulo => {
                    if left.inferred_type() == InferredType::Number
                        && right.inferred_type() == InferredType::Number
                    {
                        InferredType::Number
                    } else {
                        InferredType::Unknown
                    }
                }
                LoweredBinaryOp::Less
                | LoweredBinaryOp::LessEqual
                | LoweredBinaryOp::Greater
                | LoweredBinaryOp::GreaterEqual
                | LoweredBinaryOp::StrictEqual
                | LoweredBinaryOp::EqualEqual
                | LoweredBinaryOp::BangEqual
                | LoweredBinaryOp::StrictNotEqual => InferredType::Boolean,
                LoweredBinaryOp::And
                | LoweredBinaryOp::Or
                | LoweredBinaryOp::NullishCoalesce => InferredType::Unknown,
            },
            Self::Assign { expr, .. } => expr.inferred_type(),
            Self::LogicalAssign { .. }
            | Self::LogicalPropertyAssign { .. }
            | Self::LogicalMemberAssign { .. }
            | Self::LogicalComputedMemberAssign { .. }
            | Self::LogicalComputedPropertyAssign { .. } => InferredType::Unknown,
            _ => InferredType::Unknown,
        }
    }
}
