# Implement GC strategy

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-28
**ID**: 017b
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 017b1, 017b2, 017b3
**Orchestration class**: blocked

Problem: GC strategy is designed in 017a but not implemented. Runtime needs actual GC to prevent memory leaks.

Scope:

This is now a tracking issue split into implementation slices:

- 017b1: Implement heap object header and GC allocation trigger accounting.
- 017b2: Implement mark phase root scanning for reachable heap objects.
- 017b3: Implement sweep/free-list reuse and GC-relevant differential fixtures.

Out of scope:

- Design decisions (see 017a)

Acceptance Criteria:

- [ ] 017b1 is complete.
- [ ] 017b2 is complete.
- [ ] 017b3 is complete.
- [ ] Node differential test passes for GC-relevant fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/closure-escape.wasm
```
