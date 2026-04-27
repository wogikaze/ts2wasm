# Implement GC strategy

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-28
**ID**: 017b
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 217, 218, 219, 220, 221
**Orchestration class**: blocked

Problem: GC strategy is designed in 017a but not implemented. Runtime needs actual GC to prevent memory leaks.

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
- [ ] 221 is complete.
- [ ] Node differential test passes for GC-relevant fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/closure-escape.wasm
```
