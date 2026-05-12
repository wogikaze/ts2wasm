// Real module boundaries replace the former generated module splice.
pub mod captures;
pub mod classes;
pub mod completion;
pub mod ctx;
pub mod facts;
pub mod hir;
pub mod hir_dump;
pub mod hir_to_mir;
pub mod hir_validate;
pub mod lower;
pub mod mir;
pub mod mir_dump;
pub mod mir_validate;
pub mod object_kernel;
pub mod program;
pub mod resolver;
pub mod symbols;
pub mod types;
pub mod validate;

// Re-exports for backward compatibility
pub use hir_dump::dump_hir;
pub use hir_to_mir::{lower_hir_to_mir, lower_hir_to_mir_native};
pub use hir_validate::validate_hir;
pub use mir::{
    MirArraySlot, MirBinaryOp, MirBuiltinErrorConstructor, MirClassPrototypeRef,
    MirClosureRepresentation, MirExpr, MirFunction, MirFunctionCallKind, MirLogicalAssignOp,
    MirModuleInfo, MirProgram, MirStmt, MirUnaryOp,
};
pub use mir_dump::dump_mir;
pub use mir_validate::validate_mir;
pub use program::lower_program;
pub(crate) use program::*;
pub(crate) use symbols::FunctionSignature;
pub use ts2wasm_runtime_catalog::RuntimeFn;
pub(crate) use types::*;
pub use types::{
    BuiltinErrorConstructor, ClassPrototypeRef, ClosureRepresentation, FuncId, FunctionCallKind,
    GeneratorState, InferredType, LocalId, LoweredArraySlot, LoweredBinaryOp, LoweredExpr,
    LoweredFunction, LoweredLogicalAssignOp, LoweredProgram, LoweredStmt, LoweredUnaryOp,
    ModuleInfo, SuspendPoint, Validated,
};
pub use validate::validate_lowered;
