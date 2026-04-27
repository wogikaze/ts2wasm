# Implement GC stack local roots for escape fixtures

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**ID**: 220
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 219
**Orchestration class**: implementation-ready

Problem: GC sweep/free-list reuse exists, but stack/local heap values are not part of the root set. A collection during execution can reclaim heap values that remain live only in locals, which blocks safe closure/object escape GC fixtures.

Scope:

- Define a root registration strategy for backend locals that can hold heap values.
- Ensure `$gc_collect` marks registered local roots before sweep.
- Add closure/object escape GC fixtures that trigger collection while keeping escaped heap values live.
- Wire Node differential coverage for those fixtures.

Out of scope:

- Generational GC and finalizers.

Acceptance Criteria:

- [ ] Heap values live in backend/user locals are marked across collection.
- [ ] Closure/object escape GC fixtures trigger collection and preserve semantics.
- [ ] Node differential tests pass for closure/object escape GC fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
```
