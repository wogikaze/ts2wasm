# Investigate GC high-pressure OOB under repeated local-root allocation (audit reopened #222)

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 222
**Type**: bug
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 220
**Orchestration class**: implementation-ready

Problem: GC local-root fixtures preserve semantics at 2000 repeated string allocations, but raising the
same fixture shape to roughly 2500 iterations produces `Exception: out of bounds memory access` under
`iwasm`. The failure appears to be allocator/free-list pressure distinct from root registration.

Scope:

- Reproduce the OOB with a minimized fixture derived from `gc-object-root.ts` or `gc-call-frame-root.ts`.
- Identify whether sweep traversal, free-list reuse, block sizing, or allocation threshold accounting is corrupting heap state.
- Add a regression fixture or backend/runtime unit test that fails before the fix and passes after it.
- Preserve the existing 2000-iteration differential fixtures.

Out of scope:

- Generational GC and finalizers.
- Precise activation-frame root push/pop semantics tracked by 221.

Acceptance Criteria:

- [x] The minimized high-pressure local-root fixture runs without OOB under `iwasm`.
- [x] The regression triggers collection and verifies Node/iwasm output equivalence.
- [x] Existing GC differential fixtures continue to pass.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
```

## Completion evidence

- Root cause: `$concat` still wrote directly through the old bump `$heap` path, bypassing `$alloc_heap`,
  GC headers, free-list reuse, and bounded memory growth. The combined root fixture failed only when a
  live temporary string crossed a collecting function call.
- Fix: `$concat` now allocates managed heap strings through `$alloc_heap`, copies source string data
  through `$copy`, and no longer mutates `$heap` directly. Runtime memory now has a bounded max page
  limit and `$alloc_heap` grows memory up to that limit before trapping.
- Guard: `fixtures/core-semantics/gc-high-pressure-root.ts` reproduces the previous 2500-iteration
  shape and is registered in the M3 Node/iwasm differential fixture list.
- Guard: `concat_allocates_managed_heap_strings` asserts that emitted `$concat` calls `$alloc_heap`
  and `$copy` and does not perform direct `(global.set $heap ...)`.
- Validation on 2026-04-28:
  - `cargo fmt --all --check`: pass
  - `cargo nextest run -p ts2wasm-backend-wasm concat_allocates_managed_heap_strings gc_sweep_and_free_list_reuse_contract_is_emitted alloc_heap_emits_gc_header_and_trigger_contract`: pass, 3 passed / 9 skipped
  - `cargo nextest run -p ts2wasm-runtime-abi memory_max_pages_cover_initial_pages initial_memory_pages_cover_single_max_stdin_heap_allocation_from_heap_start`: pass, 2 passed / 7 skipped
  - `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm`: pass, 1 passed / 19 skipped
  - `cargo nextest run -p ts2wasm-cli --test m1_iwasm oom_alloc_check_must_fail_iwasm`: pass, 1 passed / 1 skipped
