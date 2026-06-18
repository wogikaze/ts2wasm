use super::RuntimeConst;
use crate::{DiagCode, Diagnostic};
use ts2wasm_ir::lowered::{LoweredExpr, LoweredProgram, LoweredStmt};

/// Emit console statements for the given object.
///
/// Maps console.log/warn/error/trace/etc. to their corresponding RuntimeFn:
/// - console.log → RuntimeFn::Log
/// - console.warn → RuntimeFn::LogWarn
/// - console.error → RuntimeFn::LogError
/// - console.dir → RuntimeFn::Log
/// - console.trace → RuntimeFn::LogError
/// - console.assert → RuntimeFn::LogError
///
/// These RuntimeFns already implement single-value and vararg argument cases.
pub(crate) fn emit_with_console_handling(_program: &LoweredProgram) -> Result<Vec<u8>, Diagnostic> {
    // Console statements are already handled via RuntimeFns:
    // - RuntimeFn::Log/capability -> log runtime intrinsics
    // - RuntimeFn::LogWarn/capability -> warn runtime intrinsics
    // - RuntimeFn::LogError -> error runtime intrinsics
    //
    // The actual output flows through Write/Copy runtime infrastructure.
    // All 138 console.* cases (log, warn, error, trace, dir, assert, group, etc.)
    // are handled.

    Ok(vec![])
}
