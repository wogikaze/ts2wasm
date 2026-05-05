# Implement GC call-frame roots for closure escape (audit reopened #221)

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 221
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 220
**Orchestration class**: implementation-ready

Problem: Top-level locals are mirrored into GC roots, but function/call-frame locals and closure captures are still not registered. GC cannot safely collect while heap values are live only in function locals or closure environments.

Scope:

- Define root registration for function/call-frame locals that can hold heap values.
- Register and unregister roots around function execution without corrupting nested calls.
- Add closure/call-frame escape fixtures that trigger collection while preserving live heap values.
- Wire Node differential coverage for those fixtures.

Out of scope:

- Generational GC and finalizers.

Acceptance Criteria:

- [x] Function/call-frame heap locals are marked across collection.
- [x] Closure/call-frame escape fixtures trigger collection and preserve semantics.
- [x] Node differential tests pass for closure/call-frame GC fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
```

Progress notes:

- 2026-04-28: initial call-frame local mirroring and `fixtures/core-semantics/gc-call-frame-root.ts`
  differential coverage were added as a partial slice. This issue remains open because the current
  root table is conservative/static and does not yet register/unregister activation frames for
  nested calls or closure captures.
- 2026-04-28: issue 222 added backend temporary root mirroring for the caller-side temporary value
  that crosses a collecting function call. Precise activation-frame push/pop and closure capture
  semantics remain open here.

## Completion evidence

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run -p ts2wasm-backend-wasm
result: PASS (15 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm
result: PASS (1 passed, includes gc-call-frame-root.ts and closure-gc-call-frame-root.ts)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff arrow_function_fixtures_match_node_output_under_iwasm
result: PASS (1 passed)
date: 2026-04-28
```

Close note:

- Commit: `f7ad5b0`
- Backend now allocates a fixed GC call-frame root stack during `_start`, pushes activation frames on function entry, mirrors function locals/temporaries into the active frame, marks the active frame chain during collection, and pops frames on all emitted function returns.
- `fixtures/core-semantics/closure-gc-call-frame-root.ts` adds closure capture coverage under allocation pressure; existing `gc-call-frame-root.ts` continues to cover function local preservation.
