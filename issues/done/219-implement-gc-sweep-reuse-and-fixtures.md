# Implement GC sweep reuse and fixtures

**Status**: done
**Created**: 2026-04-28
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 219
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 218
**Orchestration class**: implementation-ready

Problem: Marked/unmarked heap metadata must be converted into reusable memory and covered by runtime fixtures before the GC tracking issue can close.

Scope:

- Implement sweep traversal over GC headers.
- Add free-list reuse for reclaimed blocks where size is sufficient.
- Add GC trigger retry behavior before memory growth/OOM.
- Add a GC-relevant fixture for long-running transient allocation.
- Wire Node differential coverage for the new fixtures.

Out of scope:

- Generational GC and finalizers.
- Stack/local root tracking for closure/object escape patterns (see 220).

Acceptance Criteria:

- [x] Sweep recycles unmarked blocks and preserves marked blocks.
- [x] Allocation can reuse a reclaimed block.
- [x] Long-running transient GC fixture avoids OOM/leak behavior.
- [x] Node differential tests pass for GC-relevant fixture.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/closure-escape.wasm
```

Completion evidence:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run -p ts2wasm-backend-wasm
result: PASS (9 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm
result: PASS (1 passed, 17 skipped)
date: 2026-04-28

command: mise run check-repo-smoke
result: PASS
date: 2026-04-28

command: cargo nextest run
result: PASS (223 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Stack/local roots are not yet tracked, so closure/object escape GC fixtures remain in follow-up issue 220.
