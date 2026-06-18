pub use ts2wasm_runtime_abi::value::{HeapPtr, LocalRawValue, TaggedValue, ValueTag};

pub const HEAP_F64_HEAP_KIND: u32 = 20;

pub struct HeapF64;

impl HeapF64 {
    pub fn is_heap_f64(value: TaggedValue) -> bool {
        if value.tag() != ValueTag::OBJECT {
            return false;
        }
        let ptr = value.heap_ptr();
        let header = unsafe { *(ptr.as_usize() as *const u32) };
        (header >> 2) & 0x3F == HEAP_F64_HEAP_KIND
    }
}
