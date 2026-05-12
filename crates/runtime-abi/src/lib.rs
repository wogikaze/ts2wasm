pub mod consts;
pub mod layout;
pub mod value;

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
pub use value::{HeapPtr, LocalRawValue, TaggedValue, ValueTag, WasmTaggedJsWire};
