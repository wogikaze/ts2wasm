# Implement GC

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 017b
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 017a
**Orchestration class**: implementation-ready

Problem: GC strategy is designed (017a) but not implemented. Long-running programs and programs with closure escape will leak memory.

Scope:

- Implement mark phase with root set traversal
- Implement sweep phase with heap compaction
- Integrate GC trigger into $alloc_heap
- Add test fixtures for GC behavior
- Verify Node differential test passes for GC-relevant fixtures

Out of scope:

- GC design (see 017a)
- Generational GC (future enhancement)
- Write barriers (future enhancement)

Acceptance Criteria:

- [ ] Mark phase correctly identifies live objects from root set
- [ ] Sweep phase reclaims unmarked objects
- [ ] GC triggers at allocation threshold (64KB)
- [ ] Memory leaks are prevented in test fixtures
- [ ] Node differential test passes for GC-relevant fixtures

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/closure-escape.wasm
```
