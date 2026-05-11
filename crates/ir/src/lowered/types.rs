use std::collections::{HashMap, HashSet};

use crate::binding_pattern::{
    ArrayBinding, BindingDefault, BindingPattern, ObjectBinding, parse_binding_pattern,
};
use crate::builtin::{BuiltinId, BuiltinPropertyId, BuiltinResult};
use crate::builtin_resolved::{ClassMethodKind, ResolvedExpr, ResolvedParam, ResolvedStmt};
use crate::lowered::RuntimeFn;
use crate::lowered::validate::validate_lowered;
use ts2wasm_shared::{
    BinaryOp, DiagCode, Diagnostic, LogicalAssignOp, OBJECT_SPREAD_SENTINEL,
    SYMBOL_ITERATOR_OBJECT_KEY, Span, UnaryOp,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncId(pub usize);

pub(crate) type ClassConstructorMap = HashMap<String, FuncId>;
pub(crate) type ClassMethodMap = HashMap<(String, String), FuncId>;
pub(crate) type ClassPrivateFieldSlots = HashMap<String, HashMap<String, usize>>;
pub(crate) type ClassStaticPrivateFields = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassPrototypeRef {
    pub constructor: FuncId,
    pub parent_constructors: Vec<FuncId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltinErrorConstructor {
    Error,
    RangeError,
    TypeError,
    ReferenceError,
    SyntaxError,
}

impl BuiltinErrorConstructor {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Error" => Some(Self::Error),
            "RangeError" => Some(Self::RangeError),
            "TypeError" => Some(Self::TypeError),
            "ReferenceError" => Some(Self::ReferenceError),
            "SyntaxError" => Some(Self::SyntaxError),
            _ => None,
        }
    }

    pub fn parent(self) -> Option<Self> {
        match self {
            Self::Error => None,
            Self::RangeError | Self::TypeError | Self::ReferenceError | Self::SyntaxError => {
                Some(Self::Error)
            }
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
    /// Static recursion depth: 0 = not recursive, 1+ = part of a recursive cycle.
    /// Used by ABC451 runtime tracking to distinguish top-level array growth
    /// (depth 0) from nested/recursive array growth (depth 1+).
    pub recursion_depth: usize,
    pub is_async: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredStmt {
    Block(Vec<LoweredStmt>, Span),
    Let(LocalId, LoweredExpr, Span),
    Assign(LocalId, LoweredExpr, Span),
    Expr(LoweredExpr, Span),
    If {
        condition: LoweredExpr,
        then_body: Vec<LoweredStmt>,
        else_body: Vec<LoweredStmt>,
        span: Span,
    },
    While {
        condition: LoweredExpr,
        body: Vec<LoweredStmt>,
        span: Span,
    },
    Return(LoweredExpr, Span),
    Throw(LoweredExpr, Span),
    TryCatch {
        try_body: Vec<LoweredStmt>,
        catch_var: Option<LocalId>,
        catch_body: Option<Vec<LoweredStmt>>,
        finally_body: Option<Vec<LoweredStmt>>,
        span: Span,
    },
    Switch {
        expr: LoweredExpr,
        cases: Vec<(Option<LoweredExpr>, Vec<LoweredStmt>)>,
        span: Span,
    },
    DoWhile {
        body: Vec<LoweredStmt>,
        condition: LoweredExpr,
        span: Span,
    },
    For {
        init: Option<Box<LoweredStmt>>,
        condition: Option<LoweredExpr>,
        update: Option<LoweredExpr>,
        body: Vec<LoweredStmt>,
        span: Span,
    },
    ForIn {
        var: LocalId,
        iter: LoweredExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<LoweredStmt>,
        span: Span,
    },
    ForOf {
        var: LocalId,
        iter: LoweredExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<LoweredStmt>,
        span: Span,
    },
    Labeled {
        label: String,
        body: Box<LoweredStmt>,
        span: Span,
    },
    Break {
        label: Option<String>,
        span: Span,
    },
    Continue {
        label: Option<String>,
        span: Span,
    },
    Export {
        name: String,
        expr: LoweredExpr,
        span: Span,
    },
    ModuleExportsAssign {
        expr: LoweredExpr,
        span: Span,
    },
    ClassDecl {
        name: String,
        extends: Option<String>,
        constructor: Option<FuncId>,
        methods: Vec<(String, FuncId)>,
        static_methods: Vec<(String, FuncId)>,
        private_fields: Vec<String>,
        span: Span,
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
pub enum LoweredArraySlot {
    Present(LoweredExpr),
    Hole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredExpr {
    Number(i32, Span),
    BigIntLiteral {
        decimal: String,
        sign: i32,
        limb_low: u32,
        limb_high: u32,
        span: Span,
    },
    String(String, Span),
    Bool(bool, Span),
    Null(Span),
    Undefined(Span),
    Local(LocalId, Span),
    EnvCellNew(Box<LoweredExpr>, Span),
    EnvCellGet(LocalId, Span),
    EnvCellSet {
        cell: LocalId,
        expr: Box<LoweredExpr>,
        span: Span,
    },
    Unary {
        op: LoweredUnaryOp,
        expr: Box<LoweredExpr>,
        span: Span,
    },
    Binary {
        left: Box<LoweredExpr>,
        op: LoweredBinaryOp,
        right: Box<LoweredExpr>,
        span: Span,
    },
    PropertyIn {
        obj: Box<LoweredExpr>,
        key: String,
        span: Span,
    },
    PropertyInDynamic {
        obj: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
        span: Span,
    },
    Call {
        kind: FunctionCallKind,
        args: Vec<LoweredExpr>,
        span: Span,
    },
    Assign {
        local: LocalId,
        expr: Box<LoweredExpr>,
        span: Span,
    },
    LogicalAssign {
        local: LocalId,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
        span: Span,
    },
    LogicalPropertyAssign {
        object: LocalId,
        key: String,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
        span: Span,
    },
    LogicalComputedPropertyAssign {
        object: LocalId,
        key: Box<LoweredExpr>,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
        span: Span,
    },
    LogicalComputedMemberAssign {
        object: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
        span: Span,
    },
    LogicalMemberAssign {
        object: Box<LoweredExpr>,
        key: String,
        op: LoweredLogicalAssignOp,
        expr: Box<LoweredExpr>,
        span: Span,
    },
    ArrayNew {
        elements: Vec<LoweredExpr>,
        span: Span,
    },
    ArrayNewSparse {
        slots: Vec<LoweredArraySlot>,
        span: Span,
    },
    ArrayGet {
        arr: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
        span: Span,
    },
    Index {
        object: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
        span: Span,
    },
    GetLength(Box<LoweredExpr>, Span),
    ObjectNew {
        props: Vec<(String, LoweredExpr)>,
        non_enumerable: u32, // bitmask: bit i = property i is non-enumerable
        span: Span,
    },
    ErrorNew {
        constructor: BuiltinErrorConstructor,
        message: Box<LoweredExpr>,
        span: Span,
    },
    PropertyGet {
        obj: Box<LoweredExpr>,
        key: String,
        span: Span,
    },
    OptionalPropertyGet {
        obj: Box<LoweredExpr>,
        key: String,
        span: Span,
    },
    PropertyGetDynamic {
        obj: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
        span: Span,
    },
    OptionalIndex {
        object: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
        span: Span,
    },
    OptionalCall {
        callee: Box<LoweredExpr>,
        call: Box<LoweredExpr>,
        span: Span,
    },
    MethodCall {
        object: Box<LoweredExpr>,
        method: String,
        span: Span,
    },
    /// Extract the resolved value from a fulfilled Promise.
    /// Reads promise slot[1] (result value) at offset ARRAY_HEADER_SIZE + 4.
    /// Returns undefined if the promise is not fulfilled.
    PromiseGetValue {
        promise: Box<LoweredExpr>,
        span: Span,
    },
    RuntimeCall {
        intrinsic: RuntimeFn,
        args: Vec<LoweredExpr>,
        span: Span,
    },
    PropertySet {
        object: Box<LoweredExpr>,
        key: String,
        value: Box<LoweredExpr>,
        span: Span,
    },
    PropertyDelete {
        object: Box<LoweredExpr>,
        key: String,
        span: Span,
    },
    PropertyDeleteDynamic {
        object: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
        span: Span,
    },
    PropertySetDynamic {
        object: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
        value: Box<LoweredExpr>,
        span: Span,
    },
    New {
        constructor: FuncId,
        prototype: ClassPrototypeRef,
        args: Vec<LoweredExpr>,
        base_local: LocalId,
        private_brand: Option<u32>,
        private_slot_count: usize,
        span: Span,
    },
    ClassPrototype(ClassPrototypeRef, Span),
    BuiltinErrorPrototype(BuiltinErrorConstructor, Span),
    ModuleLoad {
        module_id: usize,
        span: Span,
    },
    Block {
        stmts: Vec<LoweredStmt>,
        result: Box<LoweredExpr>,
        span: Span,
    },
    This(Span),
    ArrowFn {
        func_id: FuncId,
        captures: Vec<LocalId>,
        representation: ClosureRepresentation,
        span: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoweredBinaryOp {
    Add,
    Subtract,
    Multiply,
    Power,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
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
    Plus,
    Negate,
    TypeOf,
    Delete,
    Void,
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
            Self::Number(_, _) => InferredType::Number,
            Self::BigIntLiteral { .. } => InferredType::Unknown,
            Self::String(_, _) => InferredType::String,
            Self::Bool(_, _) => InferredType::Boolean,
            Self::Unary { op, expr, .. } => match op {
                LoweredUnaryOp::Plus if expr.inferred_type() == InferredType::Number => {
                    InferredType::Number
                }
                LoweredUnaryOp::Negate if expr.inferred_type() == InferredType::Number => {
                    InferredType::Number
                }
                LoweredUnaryOp::Not => InferredType::Boolean,
                _ => InferredType::Unknown,
            },
            Self::Binary {
                left, op, right, ..
            } => match op {
                LoweredBinaryOp::Add => match (left.inferred_type(), right.inferred_type()) {
                    (InferredType::Number, InferredType::Number) => InferredType::Number,
                    (InferredType::String, InferredType::String) => InferredType::String,
                    _ => InferredType::Unknown,
                },
                LoweredBinaryOp::Subtract
                | LoweredBinaryOp::Multiply
                | LoweredBinaryOp::Power
                | LoweredBinaryOp::Divide
                | LoweredBinaryOp::Modulo
                | LoweredBinaryOp::BitwiseAnd
                | LoweredBinaryOp::BitwiseXor
                | LoweredBinaryOp::BitwiseOr => {
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
                LoweredBinaryOp::And | LoweredBinaryOp::Or | LoweredBinaryOp::NullishCoalesce => {
                    InferredType::Unknown
                }
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

/// Newtype wrapper around a successfully-validated `LoweredProgram`.
///
/// The validation step checks for invariant violations (fatal errors that
/// would crash the backend) and separates non-fatal diagnostics into the
/// wrapper so callers can report them without blocking emission.
///
/// Backend APIs should accept `&Validated<LoweredProgram>` instead of
/// `&LoweredProgram` to enforce the contract at the type level.
#[derive(Debug)]
pub struct Validated<T> {
    inner: T,
    non_fatal: Vec<Diagnostic>,
}

impl Validated<LoweredProgram> {
    /// Validate `program` and return a `Validated` wrapper.
    ///
    /// Returns `Err` only for fatal errors (InvariantViolation).
    /// Non-fatal diagnostics are returned in the tuple alongside the wrapper.
    pub fn new(program: LoweredProgram) -> Result<(Self, Vec<Diagnostic>), Diagnostic> {
        let mut non_fatal = Vec::new();
        if let Err(errors) = validate_lowered(&program) {
            for e in errors {
                if e.code == DiagCode::InvariantViolation {
                    return Err(Diagnostic {
                        code: e.code,
                        message: e.message,
                        span: e.span,
                        phase: None,
                    });
                }
                non_fatal.push(e);
            }
        }
        Ok((
            Self {
                inner: program,
                non_fatal: non_fatal.clone(),
            },
            non_fatal,
        ))
    }

    pub fn program(&self) -> &LoweredProgram {
        &self.inner
    }

    pub fn into_inner(self) -> LoweredProgram {
        self.inner
    }

    pub fn warnings(&self) -> &[Diagnostic] {
        &self.non_fatal
    }
    pub fn take_warnings(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.non_fatal)
    }
}

impl<T> AsRef<T> for Validated<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}
