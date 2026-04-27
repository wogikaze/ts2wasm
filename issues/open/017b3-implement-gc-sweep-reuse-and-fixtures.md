# Implement GC sweep reuse and fixtures

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**ID**: 017b3
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 017b2
**Orchestration class**: implementation-ready

Problem: Marked/unmarked heap metadata must be converted into reusable memory and covered by runtime fixtures before the GC tracking issue can close.

Scope:

- Implement sweep traversal over GC headers.
- Add free-list reuse for reclaimed blocks where size is sufficient.
- Add GC trigger retry behavior before memory growth/OOM.
- Add GC-relevant fixtures for long-running allocation and closure/object escape patterns.
- Wire Node differential coverage for the new fixtures.

Out of scope:

- Generational GC and finalizers.

Acceptance Criteria:

- [ ] Sweep recycles unmarked blocks and preserves marked blocks.
- [ ] Allocation can reuse a reclaimed block.
- [ ] Long-running GC fixtures avoid OOM/leak behavior.
- [ ] Node differential tests pass for GC-relevant fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/closure-escape.wasm
```
