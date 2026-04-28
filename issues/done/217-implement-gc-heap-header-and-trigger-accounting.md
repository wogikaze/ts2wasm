# Implement GC heap header and trigger accounting

**Status**: done
**Created**: 2026-04-28
**Updated**: 2026-04-28
**Completed**: 2026-04-28
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

- [x] Heap object header fields are emitted for each `$alloc_heap` allocation.
- [x] Existing heap payload pointer ABI remains unchanged for string/array/object users.
- [x] Allocation threshold accounting calls a collection hook before OOM.
- [x] Tests cover the emitted allocation contract.

Validation:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-runtime-abi
cargo nextest run -p ts2wasm-backend-wasm
```

Completion evidence:

```text
command: cargo nextest run -p ts2wasm-runtime-abi
result: PASS (8 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-backend-wasm
result: PASS (5 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff m5_array_object_fixtures_match_node_output_under_iwasm m5_edge_case_fixtures_match_node_output_under_iwasm
result: PASS (2 passed, 16 skipped)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m6_builtin_methods
result: PASS (27 passed)
date: 2026-04-28

command: mise run check-repo-smoke
result: PASS
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (219 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- `$alloc_heap(size)` still records `GC_KIND_UNKNOWN` because call sites do not yet pass heap kind metadata. Mark/sweep traversal is tracked by 218 and 219.
