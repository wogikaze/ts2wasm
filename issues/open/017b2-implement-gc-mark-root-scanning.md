# Implement GC mark root scanning

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**ID**: 017b2
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 017b1
**Orchestration class**: implementation-ready

Problem: GC can only reclaim safely after reachable heap objects are marked from runtime roots.

Scope:

- Define the initial root set for globals, module cache, and runtime-held heap values.
- Implement mark helpers for string/array/object payload layouts.
- Mark object prototype and property values, plus array elements.
- Add tests that validate mark bit updates for representative heap graphs.

Out of scope:

- Sweep/free-list reuse and long-running leak fixtures (017b3)

Acceptance Criteria:

- [ ] Mark phase visits reachable heap objects from runtime roots.
- [ ] Object prototype/property references and array elements are recursively marked.
- [ ] Tests cover reachable and unreachable object graphs.

Validation:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-backend-wasm
```
