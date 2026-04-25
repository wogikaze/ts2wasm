# Implement heap OOM check

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 013
**Type**: feature
**Area**: runtime/memory
**Priority**: P0
**Depends on**: none
**Orchestration class**: implementation-ready

Problem: `$alloc_heap` does not check `memory.size` before allocation. Large allocations can cause undefined behavior or memory corruption.

Scope:

- Add memory size check in `$alloc_heap` runtime function.
- Return error or trap when allocation exceeds available memory.
- Add test fixture for large allocation that should fail gracefully.
- Document OOM behavior in runtime ABI docs.

Acceptance Criteria:

- [ ] `$alloc_heap` checks available memory before allocation.
- [ ] OOM condition is handled with clear error or trap.
- [ ] Test fixture verifies OOM behavior.
- [ ] No undefined behavior on large allocations.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/basics-oom/oom-test.wasm
```
