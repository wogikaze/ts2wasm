//! Source location types for ts2wasm.
//!
//! Provides `Span`, a lightweight source range used throughout
//! the compiler pipeline to track source positions.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a generated/inferred span for synthetic nodes.
    /// Use only for compiler-generated nodes; real source spans must come from the AST.
    pub fn generated(_label: &'static str) -> Self {
        Self { start: 0, end: 0 }
    }
}
