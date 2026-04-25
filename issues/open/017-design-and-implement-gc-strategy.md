# Design and implement GC strategy

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 017
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 013
**Orchestration class**: implementation-ready

Problem: Current runtime has no GC. Long-running programs and programs with closure escape will leak memory. docs/04 specifies initial mark-and-sweep or arena + explicit lifetime management.

Scope:

- Design heap object header with type tag, mark bit, size, and field layout.
- Implement initial mark-and-sweep GC or arena allocator.
- Add GC trigger points (allocation threshold, explicit collection).
- Add test fixtures for closure escape and long-running patterns.
- Document GC strategy in runtime ABI docs.

Acceptance Criteria:

- [ ] Heap object header is defined and used.
- [ ] Initial GC implementation prevents memory leaks in test fixtures.
- [ ] GC trigger points are defined and functional.
- [ ] Node differential test passes for GC-relevant fixtures.
- [ ] GC strategy is documented in docs/14-runtime-abi.md.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/closure-escape.wasm
```
