pub mod arithmetic;
pub mod bigint;
pub mod catalog;
pub mod comparison;
pub mod control;
pub mod conversion;
pub mod emit;
pub mod memory;
/// Typed WasmIR builders for core runtime functions (migration from raw WAT).
/// See module-level docs for escape hatch documentation.
pub mod typed;
