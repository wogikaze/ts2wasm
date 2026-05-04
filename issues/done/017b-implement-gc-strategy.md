---
id: 017b
title: "Implement GC strategy"
type: feature
area: runtime/memory
class: verification-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-05-04
---

## Summary

GC strategy is designed in 017a but not implemented. Runtime needs actual GC to prevent memory leaks.

Scope:

This is now a tracking issue split into implementation slices:

- 217: Implement heap object header and GC allocation trigger accounting.
- 218: Implement mark phase root scanning for reachable heap objects.
- 219: Implement sweep/free-list reuse and GC-relevant differential fixtures.
- 220: Implement stack/local root tracking for closure/object escape GC fixtures.
- 221: Implement function/call-frame GC roots for closure escape fixtures.

Out of scope:

- Design decisions (see 017a)

Acceptance Criteria:

- [x] 217 is complete.
- [x] 218 is complete.
- [x] 219 is complete.
- [x] 220 is complete.
- [x] 221 is complete (function/call-frame GC roots: fixtures verified).
- [x] Node differential test passes for GC-relevant fixtures (gc_semantic_fixtures_match_node_output_under_iwasm).

Validation:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(/gc_semantic_fixtures/)'
```

## Completion evidence

Commits:

- `9c82638e` (test split + verification)

Validation result:

```text
cargo nextest run -E 'test(/gc_semantic_fixtures/)'
PASS [0.961s] ts2wasm-cli::m2_node_diff m2_node_diff_fixture_tests::gc_semantic_fixtures_match_node_output_under_iwasm
```

All 5 GC fixtures pass:
- gc-transient-allocation: ✓
- gc-object-root: ✓
- gc-call-frame-root: ✓
- gc-high-pressure-root: ✓
- closure-gc-call-frame-root: ✓

Remaining risks:

- none
