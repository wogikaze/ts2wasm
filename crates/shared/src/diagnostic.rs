//! Re-export diagnostic types from ts2wasm-diagnostic for backward compatibility.
//! New code should import from `ts2wasm_diagnostic` directly.

pub use ts2wasm_diagnostic::{DiagCode, Diagnostic, InternalDiagnostic, SourceDiagnostic};
