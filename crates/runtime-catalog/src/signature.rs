/// Stack-effect signature for a runtime function call.
///
/// Describes how many i32 values the function consumes (params) and
/// produces (results) on the wasm stack.  All runtime functions use
/// i32 for heap pointers, so per-type tracking is deferred to the
/// backend layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeSignature {
    pub params: usize,
    pub results: usize,
}

impl RuntimeSignature {
    pub const fn new(params: usize, results: usize) -> Self {
        Self { params, results }
    }

    pub const fn take_n_return_one(n: usize) -> Self {
        Self {
            params: n,
            results: 1,
        }
    }

    pub const fn take_one_return_one() -> Self {
        Self {
            params: 1,
            results: 1,
        }
    }
}
