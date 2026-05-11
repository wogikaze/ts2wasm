// Replaced include! with real module boundaries
pub mod classes;
pub mod completion;
pub mod ctx;
pub mod facts;
pub mod hir;
pub mod lower;
pub mod mir;
pub mod object_kernel;
pub mod program;
pub mod resolver;
pub mod runtime_intrinsic;
pub mod symbols;
pub mod types;
pub mod validate;

// Re-exports for backward compatibility
pub use program::lower_program;
pub(crate) use program::*;
pub use runtime_intrinsic::RuntimeIntrinsic;
pub(crate) use types::*;
pub use types::{
    BuiltinErrorConstructor, ClassPrototypeRef, ClosureRepresentation, FuncId, FunctionCallKind,
    InferredType, LocalId, LoweredArraySlot, LoweredBinaryOp, LoweredExpr, LoweredFunction,
    LoweredLogicalAssignOp, LoweredProgram, LoweredStmt, LoweredUnaryOp, ModuleInfo, Validated,
};
pub use validate::validate_lowered;
