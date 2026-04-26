# Implement heap OOM check

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
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

- [x] `$alloc_heap` checks available memory before allocation.
- [x] OOM condition is handled with clear error or trap.
- [x] Test fixture verifies OOM behavior.
- [x] No undefined behavior on large allocations.

Close:

- Date: 2026-04-26
- Evidence:
  - Added memory.size check in $alloc_heap using memory.size and memory.bytes calculation
  - OOM triggers unreachable trap when allocation exceeds available memory
  - Test fixture fixtures/basics-oom/oom-test.ts traps with "out of bounds memory access" (expected OOM behavior)
  - Documented OOM handling in docs/14-runtime-abi.md
  - All tests pass: cargo nextest run (185 passed, 4 skipped)
  - Format check passes: cargo fmt --all --check

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/basics-oom/oom-test.wasm
```
