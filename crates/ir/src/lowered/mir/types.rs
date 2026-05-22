use crate::lowered::{
    BuiltinErrorConstructor, ClassPrototypeRef, ClosureRepresentation, FuncId, FunctionCallKind,
    GeneratorState, LocalId, LoweredBinaryOp, LoweredLogicalAssignOp, LoweredUnaryOp, ModuleInfo,
    ModuleLoadKind, RuntimeFn,
};
use ts2wasm_source::Span;

use super::induction_var::InductionVarInfo;

// ---------------------------------------------------------------------------
// Escape analysis types
// ---------------------------------------------------------------------------

/// Result of escape analysis for a single local variable.
///
/// Determines whether the value held by a local (e.g., an object or array)
/// can be referenced from outside the current function scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeStatus {
    /// Object/array escapes — must be heap-allocated.
    Escaped,
    /// Object/array does not escape current function.
    NotEscaped,
    /// Not yet analyzed.
    Unknown,
}

pub type MirBinaryOp = LoweredBinaryOp;
pub type MirBuiltinErrorConstructor = BuiltinErrorConstructor;
pub type MirClassPrototypeRef = ClassPrototypeRef;
pub type MirClosureRepresentation = ClosureRepresentation;
pub type MirFunctionCallKind = FunctionCallKind;
pub type MirLogicalAssignOp = LoweredLogicalAssignOp;
pub type MirModuleInfo = ModuleInfo;
pub type MirUnaryOp = LoweredUnaryOp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirArraySlot {
    Present(MirExpr),
    Hole,
}

// ---------------------------------------------------------------------------
// MirExpr — native expression type
// ---------------------------------------------------------------------------

/// A MIR expression node. Structurally equivalent to LoweredExpr.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirExpr {
    Number(i32, Span),
    DecimalNumber(String, Span),
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
    EnvCellNew(Box<MirExpr>, Span),
    EnvCellGet(LocalId, Span),
    EnvCellSet {
        cell: LocalId,
        expr: Box<MirExpr>,
        span: Span,
    },
    Unary {
        op: LoweredUnaryOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    Binary {
        left: Box<MirExpr>,
        op: LoweredBinaryOp,
        right: Box<MirExpr>,
        span: Span,
    },
    PropertyIn {
        obj: Box<MirExpr>,
        key: String,
        span: Span,
    },
    PropertyInDynamic {
        obj: Box<MirExpr>,
        key: Box<MirExpr>,
        span: Span,
    },
    Call {
        kind: FunctionCallKind,
        args: Vec<MirExpr>,
        span: Span,
    },
    Assign {
        local: LocalId,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalAssign {
        local: LocalId,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalPropertyAssign {
        object: LocalId,
        key: String,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalComputedPropertyAssign {
        object: LocalId,
        key: Box<MirExpr>,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalComputedMemberAssign {
        object: Box<MirExpr>,
        key: Box<MirExpr>,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalMemberAssign {
        object: Box<MirExpr>,
        key: String,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    ArrayNew {
        elements: Vec<MirExpr>,
        span: Span,
    },
    ArrayNewSparse {
        slots: Vec<MirArraySlot>,
        span: Span,
    },
    ArrayGet {
        arr: Box<MirExpr>,
        index: Box<MirExpr>,
        span: Span,
    },
    Index {
        object: Box<MirExpr>,
        index: Box<MirExpr>,
        span: Span,
    },
    GetLength(Box<MirExpr>, Span),
    ObjectNew {
        props: Vec<(String, MirExpr)>,
        non_enumerable: u32,
        span: Span,
    },
    ErrorNew {
        constructor: BuiltinErrorConstructor,
        message: Box<MirExpr>,
        cause: Option<Box<MirExpr>>,
        span: Span,
    },
    PropertyGet {
        obj: Box<MirExpr>,
        key: String,
        span: Span,
    },
    OptionalPropertyGet {
        obj: Box<MirExpr>,
        key: String,
        span: Span,
    },
    PropertyGetDynamic {
        obj: Box<MirExpr>,
        key: Box<MirExpr>,
        span: Span,
    },
    OptionalIndex {
        object: Box<MirExpr>,
        index: Box<MirExpr>,
        span: Span,
    },
    OptionalCall {
        callee: Box<MirExpr>,
        call: Box<MirExpr>,
        span: Span,
    },
    MethodCall {
        object: Box<MirExpr>,
        method: String,
        span: Span,
    },
    /// Extract the resolved value from a fulfilled Promise.
    PromiseGetValue {
        promise: Box<MirExpr>,
        span: Span,
    },
    RuntimeCall {
        intrinsic: RuntimeFn,
        args: Vec<MirExpr>,
        span: Span,
    },
    PropertySet {
        object: Box<MirExpr>,
        key: String,
        value: Box<MirExpr>,
        span: Span,
    },
    PropertyDelete {
        object: Box<MirExpr>,
        key: String,
        span: Span,
    },
    PropertyDeleteDynamic {
        object: Box<MirExpr>,
        key: Box<MirExpr>,
        span: Span,
    },
    PropertySetDynamic {
        object: Box<MirExpr>,
        index: Box<MirExpr>,
        value: Box<MirExpr>,
        span: Span,
    },
    New {
        constructor: FuncId,
        prototype: ClassPrototypeRef,
        args: Vec<MirExpr>,
        base_local: LocalId,
        private_brand: Option<u32>,
        private_slot_count: usize,
        span: Span,
    },
    ClassPrototype(ClassPrototypeRef, Span),
    BuiltinErrorPrototype(BuiltinErrorConstructor, Span),
    ModuleLoad {
        module_id: usize,
        kind: ModuleLoadKind,
        span: Span,
    },
    Block {
        stmts: Vec<MirStmt>,
        result: Box<MirExpr>,
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

// ---------------------------------------------------------------------------
// MirStmt — native statement type
// ---------------------------------------------------------------------------

/// A MIR statement node. Structurally equivalent to LoweredStmt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirStmt {
    Block(Vec<MirStmt>, Span),
    Let(LocalId, MirExpr, Span),
    Assign(LocalId, MirExpr, Span),
    Expr(MirExpr, Span),
    Yield(MirExpr, Span),
    If {
        condition: MirExpr,
        then_body: Vec<MirStmt>,
        else_body: Vec<MirStmt>,
        span: Span,
    },
    While {
        condition: MirExpr,
        body: Vec<MirStmt>,
        span: Span,
    },
    Return(MirExpr, Span),
    Throw(MirExpr, Span),
    TryFinally {
        try_body: Vec<MirStmt>,
        finally_body: Vec<MirStmt>,
        span: Span,
    },
    TryCatch {
        try_body: Vec<MirStmt>,
        catch_var: Option<LocalId>,
        catch_body: Option<Vec<MirStmt>>,
        finally_body: Option<Vec<MirStmt>>,
        span: Span,
    },
    Switch {
        expr: MirExpr,
        cases: Vec<(Option<MirExpr>, Vec<MirStmt>)>,
        span: Span,
    },
    DoWhile {
        body: Vec<MirStmt>,
        condition: MirExpr,
        span: Span,
    },
    For {
        init: Option<Box<MirStmt>>,
        condition: Option<MirExpr>,
        update: Option<MirExpr>,
        body: Vec<MirStmt>,
        span: Span,
    },
    ForIn {
        var: LocalId,
        iter: MirExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<MirStmt>,
        span: Span,
    },
    ForOf {
        var: LocalId,
        iter: MirExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<MirStmt>,
        span: Span,
    },
    ForAwaitOfLower {
        var: LocalId,
        iter: MirExpr,
        async_iter_local: LocalId,
        next_result_local: LocalId,
        done_local: LocalId,
        value_local: LocalId,
        body: Vec<MirStmt>,
        span: Span,
    },
    Labeled {
        label: String,
        body: Box<MirStmt>,
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
        expr: MirExpr,
        span: Span,
    },
    ModuleExportsUpdate {
        name: String,
        local: LocalId,
        span: Span,
    },
    ModuleExportsAssign {
        expr: MirExpr,
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

// ---------------------------------------------------------------------------
// MirFunction — native function type
// ---------------------------------------------------------------------------

/// A MIR function definition. Structurally equivalent to LoweredFunction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirFunction {
    pub id: FuncId,
    pub params: Vec<LocalId>,
    pub uses_receiver: bool,
    pub min_required_params: usize,
    pub rest_param_index: Option<usize>,
    pub locals: Vec<LocalId>,
    pub body: Vec<MirStmt>,
    /// Static recursion depth: 0 = not recursive, 1+ = part of a recursive cycle.
    pub recursion_depth: usize,
    pub is_async: bool,
    pub is_generator: bool,
    pub generator_state: Option<GeneratorState>,
    /// Induction variables detected by `induction_var::analyze_function`.
    pub induction_vars: Vec<InductionVarInfo>,
    /// Per-local escape analysis result. Indexed by `LocalId.0`.
    /// `None` means not yet analyzed (Unknown).
    pub escape_status: Vec<Option<EscapeStatus>>,
}

// ---------------------------------------------------------------------------
// MirProgram — native program type
// ---------------------------------------------------------------------------

/// A MIR (mid-level IR) program. Structurally equivalent to LoweredProgram.
///
/// The `modules` field uses `ModuleInfo` (the original type) because ModuleInfo
/// carries its own statement type (Vec<LoweredStmt>). A future native MIR
/// pass may replace this with a fully native ModuleInfo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirProgram {
    /// Top-level execution statements (excluding function definitions).
    pub top_level_statements: Vec<MirStmt>,
    /// Local variable IDs used at the top level.
    pub top_level_locals: Vec<LocalId>,
    /// User-defined function bodies.
    pub functions: Vec<MirFunction>,
    /// Module graph lowering information.
    pub modules: Vec<ModuleInfo>,
    /// Per-local escape analysis result for top-level locals.
    /// Indexed by `top_level_locals` order.
    pub escape_status: Vec<Option<EscapeStatus>>,
}
