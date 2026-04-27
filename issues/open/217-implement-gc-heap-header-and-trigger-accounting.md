# Implement GC heap header and trigger accounting

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**ID**: 217
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 017a
**Orchestration class**: implementation-ready

Problem: Runtime allocation still returns raw bump-allocated payload blocks with no GC object header or allocation trigger accounting, so later mark/sweep work has no block metadata to traverse.

Scope:

- Add runtime ABI constants for the selected GC header layout.
- Change `$alloc_heap` so it allocates a GC header before each heap payload while returning the existing payload pointer to callers.
- Record aligned payload size and flags/type metadata in the header.
- Add allocation accounting and a functional threshold trigger point that calls a GC stub before OOM checks.
- Add backend tests that inspect emitted WAT for the header/accounting contract.

Out of scope:

- Root scanning and marking (218)
- Sweep/free-list reuse and long-running leak fixtures (219)

Acceptance Criteria:

- [ ] Heap object header fields are emitted for each `$alloc_heap` allocation.
- [ ] Existing heap payload pointer ABI remains unchanged for string/array/object users.
- [ ] Allocation threshold accounting calls a collection hook before OOM.
- [ ] Tests cover the emitted allocation contract.

Validation:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-runtime-abi
cargo nextest run -p ts2wasm-backend-wasm
```
