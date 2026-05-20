pub mod consts;
pub mod layout;
pub mod layout_snapshot;
pub mod value;

/// Canonical name for the ts2wasm runtime ABI.
pub const RUNTIME_ABI_NAME: &str = "ts2wasm-runtime-abi";

/// Current runtime ABI version.
///
/// Bump this when layout/tag/offset constants change in a way that breaks
/// backward compatibility with previously compiled wasm modules.
/// Must match `RuntimeConst::ABI_VERSION`.
pub const RUNTIME_ABI_VERSION: u32 = 2;

/// Stack-effect metadata describing how many i32 values a runtime function
/// consumes from and produces on the wasm stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackEffect {
    pub params: usize,
    pub results: usize,
}

impl StackEffect {
    pub const fn take_one_return_one() -> Self {
        Self {
            params: 1,
            results: 1,
        }
    }

    pub const fn property_get() -> Self {
        Self {
            params: 3,
            results: 1,
        }
    }

    pub const fn property_set() -> Self {
        Self {
            params: 4,
            results: 1,
        }
    }
}

// Re-export commonly used types for convenience
pub use consts::{RuntimeConst, RuntimeString};
pub use layout::Layout;
pub use layout_snapshot::{check_abi_snapshot_compat, dump_runtime_abi_snapshot, LayoutSnapshot};
pub use value::{HeapPtr, LocalRawValue, TaggedValue, ValueTag, WasmTaggedJsWire};
