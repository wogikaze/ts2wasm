use crate::value::LocalId;
use crate::reference::SemReference;
use ts2wasm_runtime_core::value::TaggedValue;
use ts2wasm_source::Span;

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod, Pow,
    StrictEqual, EqualEqual,
    Less, LessEqual, Greater, GreaterEqual,
    And, Or, BitwiseAnd, BitwiseOr, BitwiseXor,
    Shl, Shr, ShrU,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not, Negate, Plus, TypeOf, Void, BitwiseNot, Delete,
}

#[derive(Debug, Clone)]
pub enum SemExpr {
    Constant(TaggedValue, Span),
    Local(LocalId, Span),
    Unary { op: UnaryOp, expr: Box<SemExpr>, span: Span },
    Binary { left: Box<SemExpr>, op: BinaryOp, right: Box<SemExpr>, span: Span },
    PropertyGet { object: Box<SemExpr>, key: String, span: Span },
    PropertyGetDynamic { object: Box<SemExpr>, key: Box<SemExpr>, span: Span },
    PropertySet { object: Box<SemExpr>, key: String, value: Box<SemExpr>, span: Span },
    Call { callee: Box<SemExpr>, args: Vec<SemExpr>, span: Span },
    Construct { constructor: Box<SemExpr>, args: Vec<SemExpr>, new_target: Option<Box<SemExpr>>, span: Span },
    ArrayLiteral { elements: Vec<SemExpr>, span: Span },
    ObjectLiteral { properties: Vec<(String, SemExpr)>, span: Span },
    FunctionExpr { function_id: u32, span: Span },
    This(Span),
    Super { property: Option<String>, span: Span },
    Import { module: String, name: String, span: Span },
    Reference(SemReference, Span),
}
