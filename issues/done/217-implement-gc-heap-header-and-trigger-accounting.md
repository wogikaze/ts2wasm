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

- Implementation commit: `1ef8f90`.
- Runtime ABI constants now define a 16-byte hidden GC header, aligned payload size metadata,
  flags/type bits, and allocation/occupancy trigger thresholds.
- `$alloc_heap` aligns the payload size, writes the hidden header before the returned payload pointer,
  updates allocation pressure, and calls `$gc_collect_stub` before the OOM check when the byte or
  occupancy threshold is crossed.
- Existing payload pointer ABI is preserved because `$alloc_heap` still returns `base`, the payload
  start after `GC_HEADER_SIZE`; string/array/object tag users continue OR-ing the returned payload
  pointer with the existing low-bit tags.
- Added backend WAT contract coverage in `runtime_core::tests::alloc_heap_emits_gc_header_and_trigger_accounting_contract`
  and runtime ABI layout tests for header alignment and kind/flag bit separation.
- Validation passed on 2026-04-28:
  - `cargo fmt --all --check`
  - `cargo nextest run -p ts2wasm-runtime-abi`
  - `cargo nextest run -p ts2wasm-backend-wasm`
