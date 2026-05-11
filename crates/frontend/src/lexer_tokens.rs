//! Re-export token types from ts2wasm-syntax for backward compatibility.
//! New code should import from `ts2wasm_syntax` directly.

pub use ts2wasm_syntax::token::{SpannedToken, Token, TokenKind};
