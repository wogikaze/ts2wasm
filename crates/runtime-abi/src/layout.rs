/// Linear memory layout constants for the WASI runtime.
pub struct Layout;

impl Layout {
    /// WebAssembly page size in bytes.
    pub const WASM_PAGE_SIZE: u32 = 64 * 1024;
    /// Initial memory pages reserved for runtime scratch + heap + stdin read path.
    /// Current guarantee scope: enough for one max-size stdin allocation from HEAP_START.
    pub const MEMORY_MIN_PAGES: u32 = 2;
    /// Maximum pages the core wasm runtime may grow to before trapping allocation.
    /// This bounded default covers the ABC451 depth-8 live-set reducer while
    /// preserving an explicit OOM trap boundary.
    pub const MEMORY_MAX_PAGES: u32 = 185;
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
    /// Bytes before the element payload for the current sparse-capable array slice:
    /// i32 length, i32 capacity, i32 presence_word_count,
    /// i32 elements_offset_from_payload_start, and one u32 presence word.
    pub const ARRAY_HEADER_SIZE: u32 = 20;
    pub const ARRAY_CAPACITY_OFFSET: u32 = 4;
    pub const ARRAY_PRESENCE_WORD_COUNT_OFFSET: u32 = 8;
    pub const ARRAY_ELEMENTS_OFFSET_OFFSET: u32 = 12;
    pub const ARRAY_PRESENCE_WORDS_OFFSET: u32 = 16;
    /// Shift to compute element byte offset from index (each element is 4 bytes).
    pub const ARRAY_ELEM_SHIFT: u32 = 2;

    // ---- Object heap layout -----------------------------------------------
    /// Bytes before the property entries:
    /// - [0 .. 4): property count (i32)
    /// - [4 .. 8): flags (i32, bit 0 = FROZEN, bit 1 = SEALED, bits 2+ = per-property non-enumerable mask)
    /// - [8 .. 12): prototype pointer (raw object heap pointer, i32)
    pub const OBJECT_HEADER_SIZE: u32 = 12;
    /// Offset of the flags field inside the object header.
    pub const OBJECT_FLAGS_OFFSET: u32 = 4;
    /// Offset of the prototype pointer inside the object header.
    pub const OBJECT_PROTOTYPE_OFFSET: u32 = 8;
    /// Offset where object entries start.
    pub const OBJECT_ENTRIES_OFFSET: u32 = 12;
    /// Each entry: (i32 key_raw_value, i32 value) = 8 bytes; shift = 3.
    pub const OBJECT_ENTRY_SHIFT: u32 = 3;
    /// Byte offset of the value field within one property entry.
    pub const OBJECT_VALUE_OFFSET: u32 = 4;
    /// Size of one property entry in bytes.
    pub const OBJECT_ENTRY_SIZE: u32 = 8;
    /// Bit 0 of flags: object is frozen (all properties non-writable, non-configurable).
    pub const OBJECT_FLAG_FROZEN: u32 = 1;
    /// Bit 1 of flags: object is sealed (properties non-configurable).
    pub const OBJECT_FLAG_SEALED: u32 = 2;
    /// Bits 2+ of flags: per-property non-enumerable mask (bit (2+i) = property i is non-enumerable).
    pub const OBJECT_NON_ENUM_SHIFT: u32 = 2;

    // ---- GC layout --------------------------------------------------------
    /// Bytes reserved immediately before each GC-managed heap payload.
    ///
    /// `$alloc_heap(size)` returns the payload pointer, so the header starts at
    /// `payload_ptr - GC_HEADER_SIZE`.
    pub const GC_HEADER_SIZE: u32 = 16;
    /// Offset of flags/type metadata from the GC header base.
    pub const GC_FLAGS_AND_TYPE_OFFSET: u32 = 0;
    /// Offset of the aligned payload size from the GC header base.
    pub const GC_BODY_SIZE_OFFSET: u32 = 4;
    /// Offset of sweep/free-list linkage from the GC header base.
    pub const GC_SWEEP_NEXT_OFFSET: u32 = 8;
    /// Offset of reserved generation/finalizer metadata from the GC header base.
    pub const GC_RESERVED_OFFSET: u32 = 12;
    /// Initial GC threshold in bytes (trigger GC when heap exceeds this).
    pub const GC_THRESHOLD: u32 = 64 * 1024;
    /// Reserved memory headroom that makes allocation-pressure GC run before
    /// the bump pointer reaches the currently reserved memory end.
    pub const GC_HEADROOM_PAGES: u32 = 12;
    /// Minimum page count requested by one successful heap growth when far
    /// enough from `MEMORY_MAX_PAGES`.
    pub const HEAP_GROW_MIN_PAGES: u32 = 16;
    /// Bytes reserved in the root table allocation for active function call frames.
    ///
    /// Function entry/exit manages this as a LIFO stack so call-frame roots can be
    /// registered without allocating during the call prologue.
    pub const GC_CALL_FRAME_ROOT_STACK_BYTES: u32 = 16 * 1024;
    /// Header words stored before each call-frame root slot payload:
    /// previous frame pointer and slot count.
    pub const GC_CALL_FRAME_HEADER_WORDS: u32 = 2;
    /// Header size, in bytes, before the first call-frame root slot.
    pub const GC_CALL_FRAME_HEADER_SIZE: u32 = Self::GC_CALL_FRAME_HEADER_WORDS * 4;
    /// Mark flag for the 017a GC header flags/type field.
    pub const GC_MARK_FLAG: u32 = 0x1;
    /// Reserved finalizer flag for the 017a GC header flags/type field.
    pub const GC_FINALIZABLE_FLAG: u32 = 0x2;
    /// Bit shift for heap kind in the 017a GC header flags/type field.
    pub const GC_KIND_SHIFT: u32 = 2;
    /// Unknown heap kind used by the current `$alloc_heap(size)` ABI.
    pub const GC_KIND_UNKNOWN: u32 = 0;
    /// String heap kind.
    pub const GC_KIND_STRING: u32 = 1 << Self::GC_KIND_SHIFT;
    /// Array heap kind.
    pub const GC_KIND_ARRAY: u32 = 2 << Self::GC_KIND_SHIFT;
    /// Object heap kind.
    pub const GC_KIND_OBJECT: u32 = 3 << Self::GC_KIND_SHIFT;
    /// BigInt heap kind.
    pub const GC_KIND_BIGINT: u32 = 4 << Self::GC_KIND_SHIFT;
    /// Mask for heap kind bits in the 017a GC header flags/type field.
    pub const GC_KIND_MASK: u32 = 0x7 << Self::GC_KIND_SHIFT;
    /// BigInt payload offset: sign (-1, 0, or 1).
    pub const BIGINT_SIGN_OFFSET: u32 = 0;
    /// BigInt payload offset: limb count for the canonical magnitude prefix.
    pub const BIGINT_LIMB_COUNT_OFFSET: u32 = 4;
    /// BigInt payload offset: low 32 bits of the first limb for the literal slice.
    pub const BIGINT_LIMB0_LOW_OFFSET: u32 = 8;
    /// BigInt payload offset: high 32 bits of the first limb for the literal slice.
    pub const BIGINT_LIMB0_HIGH_OFFSET: u32 = 12;
    /// BigInt payload offset: byte length of the cached decimal spelling.
    pub const BIGINT_DECIMAL_LEN_OFFSET: u32 = 16;
    /// BigInt payload offset: cached decimal UTF-8 bytes without an `n` suffix.
    pub const BIGINT_DECIMAL_DATA_OFFSET: u32 = 20;

    // ---- Heap number layout -----------------------------------------------
    /// Heap number sentinel stored in the object count slot.
    pub const HEAP_NUMBER_SENTINEL: i32 = -1;
    /// Heap number byte offset for cached decimal spelling length.
    pub const HEAP_NUMBER_DECIMAL_LEN_OFFSET: u32 = 8;
    /// Heap number byte offset for cached decimal UTF-8 bytes.
    pub const HEAP_NUMBER_DECIMAL_DATA_OFFSET: u32 = 12;

    /// Bit mask for object type in type_tag field.
    pub const OBJECT_TYPE_MASK: u32 = 0x7F;
    /// Mark bit for GC mark phase.
    pub const GC_MARK_BIT: u32 = 0x80000000;

    // ---- Module cache layout -----------------------------------------------
    /// Maximum number of concurrently cached modules.
    pub const MODULE_CACHE_MAX: u32 = 64;
    /// Size of one module cache entry in bytes (i32 loaded_flag, i32 value).
    pub const MODULE_CACHE_ENTRY_SIZE: u32 = 8;
}

#[cfg(test)]
mod tests {
    use super::Layout;

    fn assert_le(left: u32, right: u32) {
        assert!(left <= right);
    }

    fn assert_lt(left: u32, right: u32) {
        assert!(left < right);
    }

    #[test]
    fn memory_regions_are_non_overlapping_for_m6_stdin_slice() {
        let scratch_end = Layout::SCRATCH_OFFSET + Layout::SCRATCH_SIZE;
        let stdin_end = Layout::STDIN_BUFFER_OFFSET + Layout::STDIN_BUFFER_SIZE;

        assert_le(Layout::DATA_START, Layout::SCRATCH_OFFSET);
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
    fn gc_header_preserves_payload_pointer_alignment() {
        assert_eq!(
            Layout::GC_HEADER_SIZE % Layout::ALIGN,
            0,
            "GC header size must preserve payload pointer alignment"
        );
        assert_eq!(Layout::GC_FLAGS_AND_TYPE_OFFSET, 0);
        assert_eq!(Layout::GC_BODY_SIZE_OFFSET, 4);
        assert_eq!(Layout::GC_SWEEP_NEXT_OFFSET, 8);
        assert_eq!(Layout::GC_RESERVED_OFFSET, 12);
    }

    #[test]
    fn gc_kind_flags_do_not_overlap_mark_bits() {
        assert_eq!(Layout::GC_KIND_UNKNOWN, 0);
        assert_eq!(Layout::GC_KIND_STRING & Layout::GC_MARK_FLAG, 0);
        assert_eq!(Layout::GC_KIND_ARRAY & Layout::GC_FINALIZABLE_FLAG, 0);
        assert_eq!(
            Layout::GC_KIND_OBJECT & (Layout::GC_MARK_FLAG | Layout::GC_FINALIZABLE_FLAG),
            0
        );
        assert_eq!(
            Layout::GC_KIND_BIGINT & (Layout::GC_MARK_FLAG | Layout::GC_FINALIZABLE_FLAG),
            0
        );
        assert_eq!(
            Layout::GC_KIND_BIGINT & Layout::GC_KIND_MASK,
            Layout::GC_KIND_BIGINT
        );
    }

    #[test]
    fn gc_call_frame_stack_layout_is_word_aligned() {
        assert_eq!(Layout::GC_CALL_FRAME_ROOT_STACK_BYTES % 4, 0);
        assert_eq!(Layout::GC_CALL_FRAME_HEADER_SIZE, 8);
    }

    #[test]
    fn stdin_read_limit_is_64k() {
        assert_eq!(Layout::STDIN_READ_LIMIT, 64 * 1024);
    }

    #[test]
    fn initial_memory_pages_cover_single_max_stdin_heap_allocation_from_heap_start() {
        let bytes = Layout::MEMORY_MIN_PAGES * Layout::WASM_PAGE_SIZE;
        let max_alloc = Layout::HEAP_START + Layout::STRING_HEADER_SIZE + Layout::STDIN_READ_LIMIT;
        assert!(
            max_alloc <= bytes,
            "initial memory ({bytes} bytes) must cover one max stdin allocation from HEAP_START ({max_alloc} bytes)"
        );
    }

    #[test]
    fn memory_max_pages_cover_initial_pages() {
        assert_le(Layout::MEMORY_MIN_PAGES, Layout::MEMORY_MAX_PAGES);
    }

    #[test]
    fn scratch_stdin_heap_are_ordered() {
        assert_lt(Layout::SCRATCH_OFFSET, Layout::STDIN_BUFFER_OFFSET);
        assert_lt(Layout::STDIN_BUFFER_OFFSET, Layout::HEAP_START);
        let scratch_end = Layout::SCRATCH_OFFSET + Layout::SCRATCH_SIZE;
        let stdin_end = Layout::STDIN_BUFFER_OFFSET + Layout::STDIN_BUFFER_SIZE;
        assert!(scratch_end <= Layout::STDIN_BUFFER_OFFSET);
        assert!(stdin_end <= Layout::HEAP_START);
    }

    #[test]
    fn all_memory_regions_do_not_overlap() {
        let regions = [
            ("DATA_START", Layout::DATA_START, Layout::DATA_START + 1), // byte marker
            (
                "SCRATCH",
                Layout::SCRATCH_OFFSET,
                Layout::SCRATCH_OFFSET + Layout::SCRATCH_SIZE,
            ),
            (
                "STDIN_IOVEC_AND_NREAD",
                Layout::STDIN_IOVEC_OFFSET,
                Layout::STDIN_NREAD_OFFSET + 4,
            ),
            (
                "STDIN_BUFFER",
                Layout::STDIN_BUFFER_OFFSET,
                Layout::STDIN_BUFFER_OFFSET + Layout::STDIN_BUFFER_SIZE,
            ),
            ("HEAP_START", Layout::HEAP_START, Layout::HEAP_START + 1),
        ];
        for (i, (name_a, start_a, end_a)) in regions.iter().enumerate() {
            for (name_b, start_b, end_b) in regions.iter().skip(i + 1) {
                assert!(
                    end_a <= start_b || end_b <= start_a,
                    "region `{name_a}` [{start_a}, {end_a}) overlaps `{name_b}` [{start_b}, {end_b})"
                );
            }
        }
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn data_start_does_not_overlap_wasi_iovec_region() {
        // DATA_START (256) sits after the iovec+nread region (16..28).
        assert!(
            Layout::STDIN_NREAD_OFFSET + 4 <= Layout::DATA_START,
            "iovec/nread region (0..{}) must not reach DATA_START ({})",
            Layout::STDIN_NREAD_OFFSET + 4,
            Layout::DATA_START
        );
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn gc_headroom_pages_cover_alloc_pressure_before_heap_growth() {
        // GC headroom must cover: scratch buffer + stdin buffer + call frame root stack + GC threshold
        let headroom_bytes = Layout::GC_HEADROOM_PAGES * Layout::WASM_PAGE_SIZE;
        let max_pre_grow_alloc = Layout::SCRATCH_SIZE
            + Layout::STDIN_BUFFER_SIZE
            + Layout::GC_CALL_FRAME_ROOT_STACK_BYTES
            + Layout::GC_THRESHOLD;
        assert!(
            headroom_bytes >= max_pre_grow_alloc,
            "GC headroom ({headroom_bytes}B) must cover pre-growth allocation pressure ({max_pre_grow_alloc}B: SCRATCH={} + STDIN_BUFFER={} + CALL_FRAME_STACK={} + GC_THRESHOLD={})",
            Layout::SCRATCH_SIZE,
            Layout::STDIN_BUFFER_SIZE,
            Layout::GC_CALL_FRAME_ROOT_STACK_BYTES,
            Layout::GC_THRESHOLD,
        );
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn heap_grow_min_pages_is_reasonable() {
        // Must be >= GC headroom (to grow past headroom) and <= MEMORY_MAX_PAGES - MEMORY_MIN_PAGES
        assert!(
            Layout::HEAP_GROW_MIN_PAGES >= Layout::GC_HEADROOM_PAGES,
            "HEAP_GROW_MIN_PAGES ({}) >= GC_HEADROOM_PAGES ({})",
            Layout::HEAP_GROW_MIN_PAGES,
            Layout::GC_HEADROOM_PAGES
        );
        assert!(
            Layout::HEAP_GROW_MIN_PAGES <= Layout::MEMORY_MAX_PAGES - Layout::MEMORY_MIN_PAGES,
            "HEAP_GROW_MIN_PAGES ({}) <= MEMORY_MAX_PAGES - MEMORY_MIN_PAGES ({} - {} = {})",
            Layout::HEAP_GROW_MIN_PAGES,
            Layout::MEMORY_MAX_PAGES,
            Layout::MEMORY_MIN_PAGES,
            Layout::MEMORY_MAX_PAGES - Layout::MEMORY_MIN_PAGES
        );
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn gc_threshold_fits_in_headroom() {
        assert!(
            Layout::GC_THRESHOLD <= Layout::GC_HEADROOM_PAGES * Layout::WASM_PAGE_SIZE,
            "GC_THRESHOLD ({}) must fit within GC headroom ({} * {})",
            Layout::GC_THRESHOLD,
            Layout::GC_HEADROOM_PAGES,
            Layout::WASM_PAGE_SIZE
        );
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn memory_regions_have_no_unnecessary_gaps_below_heap() {
        // Data segment table marker must be within first page
        assert!(Layout::DATA_START < Layout::WASM_PAGE_SIZE);
        // SCRATCH and STDIN_BUFFER must be within first page
        assert!(Layout::SCRATCH_OFFSET + Layout::SCRATCH_SIZE < Layout::WASM_PAGE_SIZE);
        assert!(Layout::STDIN_BUFFER_OFFSET + Layout::STDIN_BUFFER_SIZE < Layout::WASM_PAGE_SIZE);
        // HEAP_START should be at most a small multiple of page size from start
        assert!(Layout::HEAP_START <= 2 * Layout::WASM_PAGE_SIZE);
    }

    #[test]
    fn abi_layout_golden_snapshot() {
        use crate::consts::RuntimeConst;
        use crate::value::ValueTag;

        let snapshot = format!(
            "ABI v{abi_version}\n\
             WASM_PAGE_SIZE={wasm_page}\n\
             MEMORY_MIN_PAGES={min_pages} MEMORY_MAX_PAGES={max_pages}\n\
             DATA_START={data_start} HEAP_START={heap_start}\n\
             SCRATCH_OFFSET={scratch_off} SCRATCH_SIZE={scratch_sz}\n\
             STDIN_BUFFER_OFFSET={stdin_buf_off} STDIN_BUFFER_SIZE={stdin_buf_sz}\n\
             STDIN_IOVEC_OFFSET={iovec_off} STDIN_NREAD_OFFSET={nread_off}\n\
             GC_HEADER_SIZE={gc_hdr} GC_THRESHOLD={gc_thresh}\n\
             GC_HEADROOM_PAGES={gc_headroom} HEAP_GROW_MIN_PAGES={heap_grow}\n\
             ARRAY_HEADER_SIZE={arr_hdr} OBJECT_HEADER_SIZE={obj_hdr}\n\
             ALIGN={align}\n\
             TAG_SHIFT={tag_shift} TAG_MASK={tag_mask}\n\
             HEAP_MASK={hm} HEAP_TAG={heap_tag}",
            abi_version = RuntimeConst::ABI_VERSION,
            wasm_page = Layout::WASM_PAGE_SIZE,
            min_pages = Layout::MEMORY_MIN_PAGES,
            max_pages = Layout::MEMORY_MAX_PAGES,
            data_start = Layout::DATA_START,
            heap_start = Layout::HEAP_START,
            scratch_off = Layout::SCRATCH_OFFSET,
            scratch_sz = Layout::SCRATCH_SIZE,
            stdin_buf_off = Layout::STDIN_BUFFER_OFFSET,
            stdin_buf_sz = Layout::STDIN_BUFFER_SIZE,
            iovec_off = Layout::STDIN_IOVEC_OFFSET,
            nread_off = Layout::STDIN_NREAD_OFFSET,
            gc_hdr = Layout::GC_HEADER_SIZE,
            gc_thresh = Layout::GC_THRESHOLD,
            gc_headroom = Layout::GC_HEADROOM_PAGES,
            heap_grow = Layout::HEAP_GROW_MIN_PAGES,
            arr_hdr = Layout::ARRAY_HEADER_SIZE,
            obj_hdr = Layout::OBJECT_HEADER_SIZE,
            align = Layout::ALIGN,
            tag_shift = ValueTag::NUMBER_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            hm = ValueTag::HEAP_MASK,
            heap_tag = ValueTag::OBJECT,
        );
        let expected = "\
ABI v1\n\
WASM_PAGE_SIZE=65536\n\
MEMORY_MIN_PAGES=2 MEMORY_MAX_PAGES=185\n\
DATA_START=256 HEAP_START=2048\n\
SCRATCH_OFFSET=1500 SCRATCH_SIZE=256\n\
STDIN_BUFFER_OFFSET=1792 STDIN_BUFFER_SIZE=256\n\
STDIN_IOVEC_OFFSET=16 STDIN_NREAD_OFFSET=24\n\
GC_HEADER_SIZE=16 GC_THRESHOLD=65536\n\
GC_HEADROOM_PAGES=12 HEAP_GROW_MIN_PAGES=16\n\
ARRAY_HEADER_SIZE=20 OBJECT_HEADER_SIZE=12\n\
ALIGN=8\n\
TAG_SHIFT=3 TAG_MASK=7\n\
HEAP_MASK=-8 HEAP_TAG=7";
        assert_eq!(
            snapshot, expected,
            "ABI golden snapshot mismatch!\n\
             If you intentionally changed ABI constants, update the golden value\n\
             AND bump RuntimeConst::ABI_VERSION.\n\n\
             Got:\n{snapshot}\n\n\
             Expected:\n{expected}"
        );
    }
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn backward_compat_v1_archive_matches_current() {
        use crate::consts::RuntimeConst;
        assert!(
            RuntimeConst::ABI_VERSION <= 1,
            "ABI v{} backward-compat archive needed: constants changed since v1.
             When v1 modules exist in CI artifacts, create a `compat/v1-snapshot.txt`
             reference file and compare current constants against it.",
            RuntimeConst::ABI_VERSION
        );
    }
}
