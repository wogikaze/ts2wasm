//! MIR (Mid-level IR) — runtime ABI operations.
//!
//! This module defines the boundary between HIR (JavaScript semantic operations)
//! and MIR (runtime ABI operations). The current `LoweredExpr` in `types.rs`
//! serves both roles. These type stubs define the aspirational separation
//! WITHOUT forcing migration.
//!
//! MIR operations represent runtime ABI-level operations: runtime intrinsic
//! calls, WASM primitives, memory operations, etc. They are the output of the
//! HIR→MIR lowering pass (see `lower.rs`).
//!
//! Design note (docs/24 §7.1): The HIR/MIR separation is aspirational for this
//! milestone. When migration begins, a MIR expression should be the input to
//! WASM emission, replacing the current LoweredExpr for that role. Current
//! LoweredExpr handles both HIR and MIR responsibilities.

use crate::lowered::{FuncId, LocalId, ModuleInfo, RuntimeIntrinsic};

/// A mid-level IR expression representing a runtime ABI operation.
///
/// MirExpr variants correspond to concrete runtime operations: calling a
/// runtime intrinsic, accessing WASM locals, invoking closures, etc.
/// Unlike HirExpr, MirExpr has no "spec-level" semantics — every variant
/// maps directly to a WASM emission pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirExpr {
    /// WASM i32 constant
    I32Const(i32),

    /// WASM string constant (emitted as a data segment reference)
    StringConst(String),

    /// Load a WASM local variable
    Local(LocalId),

    /// Call a runtime intrinsic function
    CallRuntime {
        intrinsic: RuntimeIntrinsic,
        args: Vec<MirExpr>,
    },

    /// Direct function call (user-defined WASM function)
    CallFunction { func: FuncId, args: Vec<MirExpr> },

    /// Indirect function call via closure
    CallClosure {
        closure: Box<MirExpr>,
        args: Vec<MirExpr>,
    },

    /// Construct a new object with static properties
    NewObject { props: Vec<(String, MirExpr)> },

    /// Construct a new array with given elements
    NewArray { elements: Vec<MirExpr> },

    /// Load a module
    LoadModule { module_id: usize },

    /// Block expression (sequence of statements yielding a value)
    Block {
        stmts: Vec<MirStmt>,
        result: Box<MirExpr>,
    },
}

/// A sequence of MIR statements.
///
/// These are runtime-level operations that the WASM emitter consumes
/// directly. They are more constrained than HirStmt — e.g., no nested
/// function declarations, no class declarations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirStmt {
    /// Allocate a new WASM local
    Let { local: LocalId, init: MirExpr },

    /// Assign to an existing WASM local
    Assign { local: LocalId, init: MirExpr },

    /// Evaluate expression for side effects
    Expr(MirExpr),

    /// Conditional branch
    If {
        condition: MirExpr,
        then_body: Vec<MirStmt>,
        else_body: Vec<MirStmt>,
    },

    /// Loop
    While {
        condition: MirExpr,
        body: Vec<MirStmt>,
    },

    /// Return from function
    Return(MirExpr),

    /// Throw exception
    Throw(MirExpr),

    /// Try-catch-finally
    TryCatch {
        try_body: Vec<MirStmt>,
        catch_var: Option<LocalId>,
        catch_body: Option<Vec<MirStmt>>,
        finally_body: Option<Vec<MirStmt>>,
    },

    /// Switch statement
    Switch {
        expr: MirExpr,
        cases: Vec<(Option<MirExpr>, Vec<MirStmt>)>,
    },

    /// Labeled statement (for break/continue targeting)
    Labeled { label: String, body: Box<MirStmt> },

    /// Break to a label
    Break { label: Option<String> },

    /// Continue to a label
    Continue { label: Option<String> },

    /// Class declaration (emits prototype chain setup)
    ClassDecl {
        name: String,
        extends: Option<String>,
        constructor: Option<FuncId>,
        methods: Vec<(String, FuncId)>,
        static_methods: Vec<(String, FuncId)>,
        private_fields: Vec<String>,
    },

    /// Export statement
    Export { name: String, expr: MirExpr },

    /// Module exports assignment
    ModuleExportsAssign { expr: MirExpr },
}

/// A compiled function in MIR form — ready for WASM emission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirFunction {
    pub id: FuncId,
    pub params: Vec<LocalId>,
    pub uses_receiver: bool,
    pub min_required_params: usize,
    pub rest_param_index: Option<usize>,
    pub locals: Vec<LocalId>,
    pub body: Vec<MirStmt>,
    pub recursion_depth: usize,
    pub is_async: bool,
}

/// A complete program in MIR form — ready for WASM emission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirProgram {
    pub top_level_statements: Vec<MirStmt>,
    pub top_level_locals: Vec<LocalId>,
    pub functions: Vec<MirFunction>,
    pub modules: Vec<ModuleInfo>,
}
