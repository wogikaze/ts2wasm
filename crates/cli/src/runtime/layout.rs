/// Linear memory layout constants for the WASI runtime.
pub struct Layout;

impl Layout {
    /// First byte offset used for the static string data segment table.
    pub const DATA_START: u32 = 256;
    /// Byte alignment for heap allocations and data segment entries.
    pub const ALIGN: u32 = 8;
    /// Initial value of the `$heap` global (base of the dynamic heap).
    pub const HEAP_START: u32 = 2048;
    /// Linear memory offset used as a scratch buffer by `$log` /
    /// `$value_to_string_into`.
    pub const SCRATCH_OFFSET: u32 = 1500;
    /// Offset of the `buf` pointer field in the `fd_write` iovec record.
    pub const IOVEC_PTR: u32 = 8;
    /// Offset of the `buf_len` field in the `fd_write` iovec record.
    pub const IOVEC_LEN: u32 = 12;
}
