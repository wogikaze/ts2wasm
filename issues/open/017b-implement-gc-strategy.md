# Implement GC strategy

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 017b
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 017a
**Orchestration class**: implementation-ready

Problem: GC strategy is designed in 017a but not implemented. Runtime needs actual GC to prevent memory leaks.

Scope:

- Implement heap object header as designed in 017a.
- Implement chosen GC strategy (mark-and-sweep or arena).
- Add GC trigger points (allocation threshold, explicit collection).
- Add test fixtures for closure escape and long-running patterns.

Out of scope:

- Design decisions (see 017a)

Acceptance Criteria:

- [ ] Heap object header is implemented as designed.
- [ ] GC implementation prevents memory leaks in test fixtures.
- [ ] GC trigger points are functional.
- [ ] Node differential test passes for GC-relevant fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/closure-escape.wasm
```
