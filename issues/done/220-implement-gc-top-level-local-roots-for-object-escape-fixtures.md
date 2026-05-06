# Implement GC top-level local roots for object escape fixtures (audit reopened #220)

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 220
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 219
**Orchestration class**: implementation-ready

Problem: GC sweep/free-list reuse exists, but top-level heap values are not part of the root set. A collection during execution can reclaim heap values that remain live only in top-level locals, which blocks object escape GC fixtures.

Scope:

- Define a root registration strategy for top-level user locals that can hold heap values.
- Ensure `$gc_collect` marks registered top-level local roots before sweep.
- Add an object escape GC fixture that triggers collection while keeping an escaped heap object live.
- Wire Node differential coverage for those fixtures.

Out of scope:

- Generational GC and finalizers.
- Function/call-frame local roots and closure capture roots (see 221).

Acceptance Criteria:

- [x] Heap values live in top-level user locals are marked across collection.
- [x] Object escape GC fixture triggers collection and preserves semantics.
- [x] Node differential tests pass for object escape GC fixture.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
```

## Completion evidence

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run -p ts2wasm-backend-wasm
result: PASS (10 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm
result: PASS (1 passed, 19 skipped)
date: 2026-04-28

command: mise run check-repo-smoke
result: PASS
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (229 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Function/call-frame locals and closure captures are not yet registered as roots; this is tracked by 221.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

