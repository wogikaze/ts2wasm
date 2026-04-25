/// Linear memory layout constants for the WASI runtime.
pub struct Layout;

impl Layout {
    /// WebAssembly page size in bytes.
    pub const WASM_PAGE_SIZE: u32 = 64 * 1024;
    /// Initial memory pages reserved for runtime scratch + heap + stdin read path.
    pub const MEMORY_MIN_PAGES: u32 = 2;
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
    /// Maximum total bytes that one `readFileSync(0, "utf8")` call may consume (64 KiB).
    pub const STDIN_READ_LIMIT: u32 = 64 * 1024;
    /// Offset of the `buf` pointer field in the `fd_write` iovec record.
    pub const IOVEC_PTR: u32 = 8;
    /// Offset of the `buf_len` field in the `fd_write` iovec record.
    pub const IOVEC_LEN: u32 = 12;
    /// Base offset of the `fd_read` iovec structure in linear memory.
    pub const STDIN_IOVEC_OFFSET: u32 = 16;
    /// Offset of the `buf` pointer field in the stdin `fd_read` iovec record.
    pub const STDIN_IOVEC_PTR: u32 = 16;
    /// Offset of the `buf_len` field in the stdin `fd_read` iovec record.
    pub const STDIN_IOVEC_LEN: u32 = 20;
    /// Offset at which `fd_read` writes the number of bytes actually read (`nread`).
    pub const STDIN_NREAD_OFFSET: u32 = 24;

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

    // ---- Module cache layout -----------------------------------------------
    /// Maximum number of concurrently cached modules.
    pub const MODULE_CACHE_MAX: u32 = 64;
    /// Size of one module cache entry in bytes (i32 loaded_flag, i32 value).
    pub const MODULE_CACHE_ENTRY_SIZE: u32 = 8;
}

#[cfg(test)]
mod tests {
    use super::Layout;

    #[test]
    fn memory_regions_are_non_overlapping_for_m6_stdin_slice() {
        let scratch_end = Layout::SCRATCH_OFFSET + Layout::SCRATCH_SIZE;
        let stdin_end = Layout::STDIN_BUFFER_OFFSET + Layout::STDIN_BUFFER_SIZE;

        assert!(Layout::DATA_START <= Layout::SCRATCH_OFFSET);
        assert!(scratch_end <= Layout::STDIN_BUFFER_OFFSET);
        assert!(stdin_end <= Layout::HEAP_START);
    }

    #[test]
    fn stdin_iovec_and_nread_do_not_overlap_stdin_buffer() {
        // nread slot occupies [STDIN_NREAD_OFFSET .. STDIN_NREAD_OFFSET + 4)
        let nread_end = Layout::STDIN_NREAD_OFFSET + 4;
        assert!(
            nread_end <= Layout::STDIN_BUFFER_OFFSET,
            "stdin iovec/nread region [{}, {}) must not reach stdin buffer ({})",
            Layout::STDIN_IOVEC_OFFSET,
            nread_end,
            Layout::STDIN_BUFFER_OFFSET
        );
    }

    #[test]
    fn heap_start_is_aligned_to_raw_value_alignment() {
        assert_eq!(
            Layout::HEAP_START % Layout::ALIGN,
            0,
            "HEAP_START must be ALIGN-aligned so heap pointers are tag-safe"
        );
    }

    #[test]
    fn stdin_read_limit_is_64k() {
        assert_eq!(Layout::STDIN_READ_LIMIT, 64 * 1024);
    }

    #[test]
    fn initial_memory_pages_cover_max_stdin_heap_allocation() {
        let bytes = Layout::MEMORY_MIN_PAGES * Layout::WASM_PAGE_SIZE;
        let max_alloc = Layout::HEAP_START + Layout::STRING_HEADER_SIZE + Layout::STDIN_READ_LIMIT;
        assert!(
            max_alloc <= bytes,
            "initial memory ({bytes} bytes) must cover max stdin allocation ({max_alloc} bytes)"
        );
    }

    #[test]
    fn scratch_stdin_heap_are_ordered() {
        assert!(Layout::SCRATCH_OFFSET < Layout::STDIN_BUFFER_OFFSET);
        assert!(Layout::STDIN_BUFFER_OFFSET < Layout::HEAP_START);
        let scratch_end = Layout::SCRATCH_OFFSET + Layout::SCRATCH_SIZE;
        let stdin_end = Layout::STDIN_BUFFER_OFFSET + Layout::STDIN_BUFFER_SIZE;
        assert!(scratch_end <= Layout::STDIN_BUFFER_OFFSET);
        assert!(stdin_end <= Layout::HEAP_START);
    }
}
