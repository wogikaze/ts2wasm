use serde::Serialize;

use crate::consts::RuntimeConst;
use crate::layout::Layout;
use crate::value::ValueTag;

/// Canonical JSON layout snapshot of the runtime ABI.
///
/// Captures all key constants that, if changed, require an ABI_VERSION bump.
#[derive(Debug, Clone, Serialize)]
pub struct LayoutSnapshot {
    pub abi_version: u32,
    pub wasm_page_size: u32,
    pub memory_min_pages: u32,
    pub memory_max_pages: u32,
    pub data_start: u32,
    pub heap_start: u32,
    pub scratch_offset: u32,
    pub scratch_size: u32,
    pub stdin_buffer_offset: u32,
    pub stdin_buffer_size: u32,
    pub stdin_iovec_offset: u32,
    pub stdin_nread_offset: u32,
    pub gc_header_size: u32,
    pub gc_threshold: u32,
    pub gc_headroom_pages: u32,
    pub heap_grow_min_pages: u32,
    pub array_header_size: u32,
    pub object_header_size: u32,
    pub align: u32,
    pub tag_shift: u32,
    pub tag_mask: u32,
    pub heap_mask: u32,
    pub heap_tag: u32,
}

impl LayoutSnapshot {
    /// Build a snapshot of the current runtime ABI layout.
    pub fn current() -> Self {
        Self {
            abi_version: RuntimeConst::ABI_VERSION,
            wasm_page_size: Layout::WASM_PAGE_SIZE,
            memory_min_pages: Layout::MEMORY_MIN_PAGES,
            memory_max_pages: Layout::MEMORY_MAX_PAGES,
            data_start: Layout::DATA_START,
            heap_start: Layout::HEAP_START,
            scratch_offset: Layout::SCRATCH_OFFSET,
            scratch_size: Layout::SCRATCH_SIZE,
            stdin_buffer_offset: Layout::STDIN_BUFFER_OFFSET,
            stdin_buffer_size: Layout::STDIN_BUFFER_SIZE,
            stdin_iovec_offset: Layout::STDIN_IOVEC_OFFSET,
            stdin_nread_offset: Layout::STDIN_NREAD_OFFSET,
            gc_header_size: Layout::GC_HEADER_SIZE,
            gc_threshold: Layout::GC_THRESHOLD,
            gc_headroom_pages: Layout::GC_HEADROOM_PAGES,
            heap_grow_min_pages: Layout::HEAP_GROW_MIN_PAGES,
            array_header_size: Layout::ARRAY_HEADER_SIZE,
            object_header_size: Layout::OBJECT_HEADER_SIZE,
            align: Layout::ALIGN,
            tag_shift: ValueTag::NUMBER_SHIFT as u32,
            tag_mask: ValueTag::TAG_MASK as u32,
            heap_mask: ValueTag::HEAP_MASK as u32,
            heap_tag: ValueTag::OBJECT as u32,
        }
    }

    /// Serialize this snapshot to canonical pretty-printed JSON.
    pub fn to_canonical_json(&self) -> String {
        let mut out =
            serde_json::to_string_pretty(self).expect("LayoutSnapshot must serialize to JSON");
        out.push('\n');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::LayoutSnapshot;

    #[test]
    fn layout_json_snapshot_matches_current() {
        let snapshot = LayoutSnapshot::current();
        let json = snapshot.to_canonical_json();

        // Verify parsed JSON fields
        let parsed: serde_json::Value =
            serde_json::from_str(&json).expect("layout snapshot should be valid JSON");
        assert_eq!(parsed["abi_version"], 2, "ABI version should be 2");
        assert_eq!(
            parsed["wasm_page_size"], 65536,
            "WASM page size should be 64 KiB"
        );
        assert_eq!(parsed["heap_start"], 33280, "heap_start should be 33280");
        assert_eq!(parsed["align"], 8, "ALIGN should be 8");
        assert_eq!(parsed["tag_shift"], 3, "NUMBER_SHIFT should be 3");
        assert_eq!(parsed["tag_mask"], 7, "TAG_MASK should be 7");

        // Compare with compat archive if it exists
        let compat_path = format!(
            "{}/compat/v{}-layout.json",
            env!("CARGO_MANIFEST_DIR"),
            snapshot.abi_version
        );
        match std::fs::read_to_string(&compat_path) {
            Ok(existing) => {
                assert_eq!(
                    json, existing,
                    "Layout JSON snapshot differs from compat archive at {compat_path}.\n\
                     If you intentionally changed ABI constants, update the archive AND bump ABI_VERSION.\n\
                     Run: BLESS=1 cargo test -p ts2wasm-runtime-abi layout_json_snapshot_matches_current"
                );
            }
            Err(_) => {
                // No compat archive yet: write it if BLESS env is set
                if std::env::var("BLESS").is_ok() {
                    std::fs::write(&compat_path, &json)
                        .unwrap_or_else(|e| panic!("failed to write compat archive {compat_path}: {e}"));
                    eprintln!("wrote compat archive: {compat_path}");
                } else {
                    // First run without archive: just verify JSON structure
                    eprintln!(
                        "note: no compat archive at {compat_path}; run BLESS=1 to create it"
                    );
                }
            }
        }
    }
}
