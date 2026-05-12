//! HIR (High-level IR) — JS-semantic operations.
//!
//! This module defines the boundary between HIR (JavaScript semantic operations)
//! and MIR (runtime ABI operations). The current `LoweredExpr` in `types.rs`
//! serves both roles. These type stubs define the aspirational separation
//! WITHOUT forcing migration.
//!
//! HIR operations represent ECMAScript-level semantics: property access,
//! arithmetic, comparison, object construction, etc. They do NOT reference
//! runtime intrinsics or WASM primitives directly.
//!
//! Design note (docs/24 §7.1): The HIR/MIR separation is aspirational for this
//! milestone. Current lowering goes directly from ResolvedExpr to LoweredExpr
//! (which conflates HIR and MIR). When migration begins, HIR lowering should
//! produce HirExpr values, and a subsequent pass (see `lower.rs`) should
//! lower HirExpr to MirExpr.
//!
//! Until migration, new expression kinds should be added to LoweredExpr.
//! These stubs exist to define the conceptual boundary.

use crate::lowered::{FuncId, LocalId, LoweredUnaryOp};

/// A high-level IR expression representing a JavaScript semantic operation.
///
/// HirExpr variants correspond to ECMAScript specification-level algorithms
/// (e.g., OrdinaryGet, ToNumber). They are NOT lowered to runtime ABI calls
/// at this level — that happens during the HIR→MIR pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HirExpr {
    /// Literal values
    Number(i32),
    String(String),
    Bool(bool),
    Null,
    Undefined,

    /// Variable references
    Local(LocalId),

    /// Unary JS-semantic operation (e.g., !, +, -, typeof, void)
    Unary {
        op: LoweredUnaryOp,
        expr: Box<Self>,
    },

    /// Binary JS-semantic operation (e.g., +, ===, <, &&)
    Binary {
        left: Box<Self>,
        op: HirBinaryOp,
        right: Box<Self>,
    },

    /// Property access by static string key
    GetProp {
        object: Box<Self>,
        key: String,
    },

    /// Property access by dynamic key
    GetIndex {
        object: Box<Self>,
        index: Box<Self>,
    },

    /// Property assignment by static key
    SetProp {
        object: Box<Self>,
        key: String,
        value: Box<Self>,
    },

    /// Property assignment by dynamic key
    SetIndex {
        object: Box<Self>,
        index: Box<Self>,
        value: Box<Self>,
    },

    /// `key in obj` operation
    HasProperty {
        object: Box<Self>,
        key: Box<Self>,
    },

    /// `delete obj[key]`
    DeleteProperty {
        object: Box<Self>,
        key: Box<Self>,
    },

    /// Object literal construction
    ObjectLiteral {
        props: Vec<(String, Self)>,
    },

    /// Array literal construction
    ArrayLiteral {
        elements: Vec<Self>,
    },

    /// Function call (user-defined or builtin)
    Call {
        callee: Box<Self>,
        args: Vec<Self>,
    },

    /// Method call (property lookup + call)
    MethodCall {
        receiver: Box<Self>,
        method: String,
        args: Vec<Self>,
    },

    /// Constructor call with `new`
    New {
        constructor: FuncId,
        args: Vec<Self>,
    },

    /// Conditional expression (ternary)
    If {
        condition: Box<Self>,
        then_expr: Box<Self>,
        else_expr: Box<Self>,
    },
}

/// Binary operators in HIR (JS-semantic).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HirBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    StrictEqual,
    EqualEqual,
    StrictNotEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    NullishCoalesce,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    In,
    InstanceOf,
    Exponentiate,
}

/// A sequence of HIR statements defining a function body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HirStmt {
    /// Local variable declaration with initializer
    Let { local: LocalId, init: HirExpr },
    /// Local variable assignment
    Assign { local: LocalId, expr: HirExpr },
    /// Expression statement (evaluated for side effects)
    Expr(HirExpr),
    /// Conditional branch
    If {
        condition: HirExpr,
        then_body: Vec<HirStmt>,
        else_body: Vec<HirStmt>,
    },
    /// Loop
    While {
        condition: HirExpr,
        body: Vec<HirStmt>,
    },
    /// Return with value
    Return(HirExpr),
    /// Throw
    Throw(HirExpr),
}

/// A lowered function in HIR form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirFunction {
    pub id: FuncId,
    pub params: Vec<LocalId>,
    pub locals: Vec<LocalId>,
    pub body: Vec<HirStmt>,
}

/// A complete program in HIR form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirProgram {
    pub body: Vec<HirStmt>,
    pub locals: Vec<LocalId>,
    pub functions: Vec<HirFunction>,
}
