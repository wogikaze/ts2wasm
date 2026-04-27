# Investigate GC high-pressure OOB under repeated local-root allocation

**Status**: done
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

Completion evidence:

```text
implementation commit: fd35d94

command: node fixtures/core-semantics/gc-high-pressure-root.ts && target/debug/ts2wasm build fixtures/core-semantics/gc-high-pressure-root.ts -o /tmp/ts2wasm-issue222-gc-high-pressure-root.wasm && iwasm /tmp/ts2wasm-issue222-gc-high-pressure-root.wasm
result: PASS (Node and iwasm stdout: top:function)
date: 2026-04-28

command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run -p ts2wasm-backend-wasm
result: PASS (11 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm
result: PASS (1 passed, 19 skipped)
date: 2026-04-28

command: scripts/manager update-issue-index --check
result: PASS
date: 2026-04-28

command: scripts/manager check-issue-index
result: PASS
date: 2026-04-28

command: scripts/manager check-agent-state
result: PASS
date: 2026-04-28

command: scripts/manager check-issue-health
result: PASS
date: 2026-04-28

command: cargo nextest run
result: PASS (230 passed, 4 skipped)
date: 2026-04-28

command: scripts/manager check-repo-smoke
result: PASS
date: 2026-04-28
```
