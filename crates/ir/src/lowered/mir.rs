// MIR type aliases — names in the `lowered` namespace.

/// Alias for the lowered program type, which serves as the MIR.
pub type MirProgram = LoweredProgram;
/// Alias for the lowered expression type.
pub type MirExpr = LoweredExpr;
/// Alias for the lowered statement type.
pub type MirStmt = LoweredStmt;
/// Alias for the lowered function type.
pub type MirFunction = LoweredFunction;
/// Alias for the lowered binary operator type.
pub type MirBinaryOp = LoweredBinaryOp;
/// Alias for the lowered unary operator type.
pub type MirUnaryOp = LoweredUnaryOp;
/// Alias for the lowered logical assign operator type.
pub type MirLogicalAssignOp = LoweredLogicalAssignOp;
/// Alias for the lowered array slot type.
pub type MirArraySlot = LoweredArraySlot;
/// Alias for the function call kind type.
pub type MirFunctionCallKind = FunctionCallKind;
/// Alias for the closure representation type.
pub type MirClosureRepresentation = ClosureRepresentation;
/// Alias for the class prototype reference type.
pub type MirClassPrototypeRef = ClassPrototypeRef;
/// Alias for the builtin error constructor type.
pub type MirBuiltinErrorConstructor = BuiltinErrorConstructor;
/// Alias for the module info type.
pub type MirModuleInfo = ModuleInfo;
