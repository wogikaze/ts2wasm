//! HIR → MIR lowering pass.
//!
//! This module defines the lowering pass that transforms HIR (JavaScript
//! semantic operations) into MIR (runtime ABI operations). The current
//! codegen path goes directly from `ResolvedExpr` to `LoweredExpr`, skipping
//! the HIR stage entirely for the main pipeline.
//!
//! Design note (docs/24 §7.1): The HIR/MIR separation is aspirational for
//! this milestone. The `lower_hir_to_mir` function is a stub that documents
//! the signature and behavior expected once migration begins.
//!
//! When migration starts:
//! 1. The current `lower_expr` → `LoweredExpr` becomes `lower_expr` → `HirExpr`
//! 2. `lower_hir_to_mir` converts `HirExpr` to `MirExpr`
//! 3. The WASM emitter consumes `MirExpr` instead of `LoweredExpr`
//! 4. `LoweredExpr` is either migrated or deprecated

use crate::lowered::mir::{MirExpr, MirProgram, MirStmt};

// Forward declarations for Hir types
use crate::lowered::hir::{HirExpr, HirProgram, HirStmt};

/// Lower a complete HIR program to MIR.
///
/// This pass translates JavaScript-semantic operations (property access,
/// arithmetic, object construction) into runtime ABI operations (runtime
/// intrinsic calls, WASM primitives, memory operations).
///
/// # Stub
///
/// This is a stub — it will be implemented when the HIR/MIR separation
/// migration begins. For now, it returns an error indicating that the
/// lowering is not yet implemented.
pub fn lower_hir_to_mir(_program: HirProgram) -> Result<MirProgram, String> {
    Err(
        "HIR→MIR lowering is not yet implemented. See docs/24 §7.1 for the migration plan."
            .to_owned(),
    )
}

/// Lower a single HIR expression to MIR.
///
/// Translates JS-semantic operations to runtime ABI calls.
///
/// # Stub
pub fn lower_hir_expr(_expr: HirExpr) -> Result<MirExpr, String> {
    Err("HIR→MIR expression lowering is not yet implemented.".to_owned())
}

/// Lower a single HIR statement to MIR.
///
/// # Stub
pub fn lower_hir_stmt(_stmt: HirStmt) -> Result<MirStmt, String> {
    Err("HIR→MIR statement lowering is not yet implemented.".to_owned())
}
