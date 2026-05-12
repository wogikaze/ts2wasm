//! Core Wasm IR types for ts2wasm backends.
//!
//! Provides `WasmInstr`, `WasmModule`, `WasmValType`, `WasmFunction`,
//! and other core wasm types used by the emission backends.

pub mod wasm_ir;

pub use wasm_ir::{
    WasmDataSegment, WasmExport, WasmExportKind, WasmFunction, WasmGlobal, WasmImport, WasmInstr,
    WasmMemory, WasmModule, WasmValType,
};
