/// Runtime intrinsic identifier — re-exported from runtime-catalog.
///
/// This type identifies runtime functions for WASM emission. Each variant
/// corresponds to a `RuntimeFn` in the runtime-catalog crate.
///
/// Pseudo-intrinsic variants (ArrayPushMany, HeapClosureCall, PrivateFieldGet,
/// PrivateFieldSet, PrivateBrandCheck) are also defined in the catalog but
/// are expanded during IR lowering and have no standalone WAT implementation.
pub use ts2wasm_runtime_catalog::RuntimeFn as RuntimeIntrinsic;
