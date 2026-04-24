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
    /// Temporary stdin staging buffer offset for `fd_read` runtime path.
    pub const STDIN_BUFFER_OFFSET: u32 = 1792;
    /// Temporary stdin staging buffer size for one `fd_read` chunk.
    pub const STDIN_BUFFER_SIZE: u32 = 256;
    /// Maximum total bytes that one `readFileSync(0, "utf8")` call may consume.
    pub const STDIN_READ_MAX_BYTES: u32 = 64 * 1024;
    /// Offset of the `buf` pointer field in the `fd_write` iovec record.
    pub const IOVEC_PTR: u32 = 8;
    /// Offset of the `buf_len` field in the `fd_write` iovec record.
    pub const IOVEC_LEN: u32 = 12;
    /// Offset of the `buf` pointer field in the `fd_read` iovec record.
    pub const FD_READ_IOVEC_PTR: u32 = 16;
    /// Offset of the `buf_len` field in the `fd_read` iovec record.
    pub const FD_READ_IOVEC_LEN: u32 = 20;
    /// Offset used by `fd_read` to store bytes read (`nread`).
    pub const FD_READ_NREAD_PTR: u32 = 24;

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

#[cfg(test)]
mod tests {
    use super::Layout;

    #[test]
    fn memory_regions_are_non_overlapping_for_m6_stdin_slice() {
        let static_data_end = Layout::DATA_START;
        assert!(Layout::DATA_START <= static_data_end);

        let scratch_end = Layout::SCRATCH_OFFSET + Layout::SCRATCH_SIZE;
        let stdin_end = Layout::STDIN_BUFFER_OFFSET + Layout::STDIN_BUFFER_SIZE;

        assert!(static_data_end <= Layout::SCRATCH_OFFSET);
        assert!(scratch_end <= Layout::STDIN_BUFFER_OFFSET);
        assert!(stdin_end <= Layout::HEAP_START);
    }

    #[test]
    fn stdin_read_cap_is_fixed_to_64k() {
        assert_eq!(Layout::STDIN_READ_MAX_BYTES, 64 * 1024);
    }
}
