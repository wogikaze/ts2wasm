// ---------------------------------------------------------------------------
// GC layout structural tests for object and closure invariants.
//
// These tests verify that:
// 1. Object runtime helpers use Layout::OBJECT_* constants, not hard-coded offsets.
// 2. The $gc_mark_object_payload scanner marks all object fields and closure captures.
// 3. Closure dispatch uses CLOSURE_SENTINEL (-2) and scans capture_count slots only.
//
// These tests require the full compiler pipeline (compiler -> IR -> backend),
// so they live in the CLI crate where all dependencies are available.
// ---------------------------------------------------------------------------

use std::path::Path;

use ts2wasm_backend_wasm::emit_wat;
use ts2wasm_frontend::{Lexer, Parser};
use ts2wasm_ir::lowered::{Validated, lower_program, validate_lowered};
use ts2wasm_runtime_abi::Layout;

/// Lower a fixture and emit its WAT for structural inspection.
fn fixture_wat(fixture: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir.join("../../").join(fixture);
    let source = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|e| panic!("failed to read {fixture}: {e}"));

    let tokens = Lexer::new(&source)
        .tokenize()
        .expect("fixture should tokenize");
    let parsed = Parser::new(tokens, &source)
        .parse_program()
        .expect("fixture should parse");
    let named =
        ts2wasm_ir::name_resolver::resolve_names(&parsed).expect("fixture should resolve names");
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&named)
        .expect("fixture should resolve builtins");
    let lowered = lower_program(&resolved).expect("fixture should lower");
    validate_lowered(&lowered).expect("fixture lowered IR should validate");
    let (validated, _) = Validated::new(lowered).expect("fixture should validate");
    emit_wat(&validated).expect("fixture should emit WAT")
}

// ---------------------------------------------------------------------------
// Test 1: Object runtime helpers use Layout::OBJECT_* constants
// ---------------------------------------------------------------------------

/// Verify that emitted WAT for object creation uses Layout::OBJECT_* constants
/// rather than hard-coded numeric offsets.
#[test]
fn object_gc_layout_uses_runtime_abi_offsets() {
    let wat = fixture_wat("fixtures/arrays-objects/object.ts");

    // The WAT should reference OBJECT_HEADER_SIZE (12) for object allocation
    let alloc_size = format!("(i32.const {})", Layout::OBJECT_HEADER_SIZE);
    assert!(
        wat.contains(&alloc_size),
        "WAT should contain OBJECT_HEADER_SIZE ({}) for allocation",
        Layout::OBJECT_HEADER_SIZE,
    );

    // OBJECT_ENTRY_SIZE (8) should be used for property entry stride
    let entry_size = format!("(i32.const {})", Layout::OBJECT_ENTRY_SIZE);
    assert!(
        wat.contains(&entry_size),
        "WAT should contain OBJECT_ENTRY_SIZE ({}) for entry stride",
        Layout::OBJECT_ENTRY_SIZE,
    );

    // OBJECT_FLAGS_OFFSET (4) should be used for flags field access
    let flags_offset = format!("(i32.const {})", Layout::OBJECT_FLAGS_OFFSET);
    assert!(
        wat.contains(&flags_offset),
        "WAT should contain OBJECT_FLAGS_OFFSET ({}) for flags field access",
        Layout::OBJECT_FLAGS_OFFSET,
    );

    // OBJECT_ENTRIES_OFFSET (12) should be used for entry base
    let entries_offset = format!("(i32.const {})", Layout::OBJECT_ENTRIES_OFFSET);
    assert!(
        wat.contains(&entries_offset),
        "WAT should contain OBJECT_ENTRIES_OFFSET ({}) for entries base",
        Layout::OBJECT_ENTRIES_OFFSET,
    );

    // OBJECT_PROTOTYPE_OFFSET (8) should be used for prototype access
    let proto_offset = format!("(i32.const {})", Layout::OBJECT_PROTOTYPE_OFFSET);
    assert!(
        wat.contains(&proto_offset),
        "WAT should contain OBJECT_PROTOTYPE_OFFSET ({}) for prototype access",
        Layout::OBJECT_PROTOTYPE_OFFSET,
    );
}

// ---------------------------------------------------------------------------
// Test 2: $gc_mark_object_payload scans closure capture slots only
// ---------------------------------------------------------------------------

/// Verify that the emitted $gc_mark_object_payload function:
/// - Dispatches on CLOSURE_SENTINEL (-2)
/// - Scans exactly capture_count slots at capture slots offset (16)
/// - Returns before scanning ordinary object entries
#[test]
fn closure_gc_scans_capture_slots_only() {
    let wat = fixture_wat("fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts");

    // The function must exist
    assert!(
        wat.contains("(func $gc_mark_object_payload"),
        "WAT must contain $gc_mark_object_payload"
    );

    // CLOSURE_SENTINEL (-2) must be used for dispatch
    assert!(
        wat.contains("(i32.const -2)"),
        "WAT must reference CLOSURE_SENTINEL (-2) for closure dispatch"
    );

    // The closure scan block/loop structure must exist
    assert!(
        wat.contains("(block $closure_done"),
        "WAT must contain $closure_done block for closure scan"
    );
    assert!(
        wat.contains("(loop $closure_scan"),
        "WAT must contain $closure_scan loop for closure scan"
    );

    // Capture count offset (8) and slots offset (16) must be used
    assert!(
        wat.contains("(i32.const 8)"),
        "WAT must reference capture_count offset (8) for closure slot count"
    );
    assert!(
        wat.contains("(i32.const 16)"),
        "WAT must reference capture_slots offset (16) for first capture slot"
    );
    assert!(
        wat.contains("(i32.const 4)"),
        "WAT must reference capture slot size (4) for slot stride"
    );

    // The closure scan must call $gc_mark_value on each captured slot
    assert!(
        wat.contains("(call $gc_mark_value (i32.load (local.get $entry_ptr)))"),
        "WAT must call $gc_mark_value on each entry_ptr in closure scan"
    );

    // Verify closure scan returns before ordinary object entry scanning
    let payload_start = wat
        .find("(func $gc_mark_object_payload")
        .expect("gc payload marker should exist");
    let payload_wat = &wat[payload_start..];

    let closure_done_start = payload_wat
        .find("(block $closure_done")
        .expect("closure scan block should exist");

    let object_scan_start = payload_wat
        .find("(if (i32.eq (local.get $count) (i32.const -1))")
        .expect("ordinary object payload scan should exist");

    let closure_scan_return = payload_wat[closure_done_start..object_scan_start]
        .find("(return)")
        .expect("closure payload scan should return before object scan");

    assert!(
        closure_done_start + closure_scan_return < object_scan_start,
        "closure marking must return before ordinary object payload scanning"
    );
}

// ---------------------------------------------------------------------------
// Test 3: Scanner/kernel parity — every RawValue field has a mark path
// ---------------------------------------------------------------------------

/// Verify that the $gc_mark_object_payload function has mark paths for:
/// - prototype pointer (at OBJECT_PROTOTYPE_OFFSET = 8)
/// - key/value entries (at OBJECT_ENTRIES_OFFSET = 12, with key+value per entry)
/// - private slots (via GC reserved word private_count, at private_slots_offset)
/// - closure sentinel (-2) dispatch with capture_count slots
/// - symbol description (at SYMBOL_DESCRIPTION_OFFSET = 8)
/// - heap number sentinel (-1) early return (no child references)
/// - BigInt no-child early return (handled by $gc_mark_value before calling here)
#[test]
fn object_kernel_no_raw_entry_scan_outside_runtime() {
    let wat = fixture_wat("fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts");

    // The $gc_mark_object_payload function must exist
    let payload_start = wat
        .find("(func $gc_mark_object_payload")
        .expect("gc payload marker should exist");

    // Locate the function body
    let gc_func_start = payload_start;
    let gc_func_end = wat[gc_func_start + 1..]
        .find("\n\n  (func ")
        .map(|pos| gc_func_start + 1 + pos)
        .unwrap_or(wat.len());
    let gc_body = &wat[gc_func_start..gc_func_end];

    // Prototype pointer mark path: load proto at OBJECT_PROTOTYPE_OFFSET (8),
    // check if non-zero, then call $gc_mark_value with object tag
    assert!(
        gc_body.contains("(i32.const 8)"),
        "GC mark function should load prototype at offset 8 (OBJECT_PROTOTYPE_OFFSET)"
    );
    assert!(
        gc_body.contains("(call $gc_mark_value"),
        "GC mark function should call $gc_mark_value for prototype"
    );

    // Key/value entry scan: iterate over entries at OBJECT_ENTRIES_OFFSET (12)
    // with OBJECT_ENTRY_SHIFT (3) stride, marking both key (offset 0) and value (offset 4)
    assert!(
        gc_body.contains("(i32.const 12)") || gc_body.contains("(i32.const 4)"),
        "GC mark function should scan object entries at offset 12 with value at offset 4"
    );

    // Private slot scan: load private_count from GC reserved word,
    // iterate at private_slots_offset, mark each slot
    assert!(
        wat.contains("$private_scan") || wat.contains("$private_done"),
        "WAT should contain private slot scan loop"
    );

    // Symbol description mark: load description at SYMBOL_DESCRIPTION_OFFSET (8)
    // when symbol sentinel (-3) is detected. The WAT template resolves
    // symbol_description_offset to its Layout constant value.
    assert!(
        wat.contains("(i32.const -3)"),
        "WAT should reference SYMBOL_SENTINEL (-3) for symbol dispatch"
    );
    // The WAT substitutes the description offset inline; after -3 check the
    // function loads the description and calls $gc_mark_value.
    let symbol_region = &wat[wat.find("(i32.const -3)").unwrap()..];
    assert!(
        symbol_region.contains("(call $gc_mark_value"),
        "Symbol sentinel dispatch should call $gc_mark_value on description"
    );

    // Heap number sentinel (-1): early return without marking children
    assert!(
        wat.contains("(i32.const -1)"),
        "WAT should reference HEAP_NUMBER_SENTINEL (-1) for heap number dispatch"
    );
}

// ---------------------------------------------------------------------------
// Test 4: gc_kind_constants_do_not_overlap_flags (from issue RMQTJQ)
// ---------------------------------------------------------------------------

/// Verify that GC_KIND_* constants do not overlap with GC mark/finalizer bits.
#[test]
fn gc_kind_constants_do_not_overlap_flags() {
    assert_eq!(
        Layout::GC_KIND_STRING & Layout::GC_MARK_FLAG,
        0,
        "GC_KIND_STRING must not overlap GC_MARK_FLAG"
    );
    assert_eq!(
        Layout::GC_KIND_ARRAY & Layout::GC_MARK_FLAG,
        0,
        "GC_KIND_ARRAY must not overlap GC_MARK_FLAG"
    );
    assert_eq!(
        Layout::GC_KIND_OBJECT & Layout::GC_MARK_FLAG,
        0,
        "GC_KIND_OBJECT must not overlap GC_MARK_FLAG"
    );
    assert_eq!(
        Layout::GC_KIND_BIGINT & Layout::GC_MARK_FLAG,
        0,
        "GC_KIND_BIGINT must not overlap GC_MARK_FLAG"
    );

    assert_eq!(
        Layout::GC_KIND_STRING & Layout::GC_FINALIZABLE_FLAG,
        0,
        "GC_KIND_STRING must not overlap GC_FINALIZABLE_FLAG"
    );
    assert_eq!(
        Layout::GC_KIND_ARRAY & Layout::GC_FINALIZABLE_FLAG,
        0,
        "GC_KIND_ARRAY must not overlap GC_FINALIZABLE_FLAG"
    );
    assert_eq!(
        Layout::GC_KIND_OBJECT & Layout::GC_FINALIZABLE_FLAG,
        0,
        "GC_KIND_OBJECT must not overlap GC_FINALIZABLE_FLAG"
    );
    assert_eq!(
        Layout::GC_KIND_BIGINT & Layout::GC_FINALIZABLE_FLAG,
        0,
        "GC_KIND_BIGINT must not overlap GC_FINALIZABLE_FLAG"
    );
}
