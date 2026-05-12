// Entry point for lowering passes.
//
// Currently delegates to hir_to_mir for the HIR-to-MIR translation.
// Future passes (e.g., optimization, closure conversion) can be added
// as additional pipeline stages here.

use super::hir_to_mir::lower_hir_to_mir;
use crate::{HirProgram, LoweredProgram};

/// Lower a HirProgram to a LoweredProgram (MIR).
///
/// This is the entry point for IR lowering. Currently performs a
/// straightforward structural translation; future passes may be composed
/// as additional transformations.
pub fn lower(hir: &HirProgram) -> LoweredProgram {
    lower_hir_to_mir(hir)
}
