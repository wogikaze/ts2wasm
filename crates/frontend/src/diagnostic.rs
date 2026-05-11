//! Re-export diagnostic types from ts2wasm-shared for backward compatibility.
//! New code should import from `ts2wasm_shared` directly.

pub use ts2wasm_shared::diagnostic::{
    DiagCode, Diagnostic, InternalDiagnostic, SourceDiagnostic,
};
pub use ts2wasm_source::Span;
