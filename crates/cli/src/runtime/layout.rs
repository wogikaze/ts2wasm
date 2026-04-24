/// Linear memory layout constants for the WASI runtime.
pub struct Layout;

impl Layout {
    /// First byte offset used for the static string data segment table.
    pub const DATA_START: u32 = 256;
    /// Byte alignment for heap allocations and data segment entries.
    pub const ALIGN: u32 = 8;
    pub const ALIGN_MASK: u32 = Self::ALIGN - 1;
    pub const STRING_HEADER_SIZE: u32 = 4;
    pub const HEAP_BUMP_PADDING: u32 = Self::STRING_HEADER_SIZE + Self::ALIGN_MASK;
    /// Initial value of the `$heap` global (base of the dynamic heap).
    pub const HEAP_START: u32 = 2048;
    /// Linear memory offset used as a scratch buffer by `$log` /
    /// `$value_to_string_into`.
    pub const SCRATCH_OFFSET: u32 = 1500;
    /// Scratch buffer size (in bytes) reserved for temporary runtime string output.
    pub const SCRATCH_SIZE: u32 = 256;
    /// Offset of the `buf` pointer field in the `fd_write` iovec record.
    pub const IOVEC_PTR: u32 = 8;
    /// Offset of the `buf_len` field in the `fd_write` iovec record.
    pub const IOVEC_LEN: u32 = 12;

    // ---- Array heap layout ------------------------------------------------
    /// Bytes before the element payload: i32 length.
    pub const ARRAY_HEADER_SIZE: u32 = 4;
    /// Shift to compute element byte offset from index (each element is 4 bytes).
    pub const ARRAY_ELEM_SHIFT: u32 = 2;

    // ---- Object heap layout -----------------------------------------------
    /// Bytes before the property entries: i32 property count.
    pub const OBJECT_HEADER_SIZE: u32 = 4;
    /// Each entry: (i32 key_raw_value, i32 value) = 8 bytes; shift = 3.
    pub const OBJECT_ENTRY_SHIFT: u32 = 3;
    /// Byte offset of the value field within one property entry.
    pub const OBJECT_VALUE_OFFSET: u32 = 4;
    /// Size of one property entry in bytes.
    pub const OBJECT_ENTRY_SIZE: u32 = 8;
}
