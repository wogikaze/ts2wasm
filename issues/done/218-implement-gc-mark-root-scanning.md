# Implement GC mark root scanning (audit reopened #218)

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 218
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 217
**Orchestration class**: implementation-ready

Problem: GC can only reclaim safely after reachable heap objects are marked from runtime roots.

Scope:

- Define the initial root set for globals, module cache, and runtime-held heap values.
- Implement mark helpers for string/array/object payload layouts.
- Mark object prototype and property values, plus array elements.
- Add tests that validate mark bit updates for representative heap graphs.

Out of scope:

- Sweep/free-list reuse and long-running leak fixtures (219)

Acceptance Criteria:

- [x] Mark phase visits reachable heap objects from runtime roots.
- [x] Object prototype/property references and array elements are recursively marked.
- [x] Tests cover reachable and unreachable object graphs.

Validation:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-backend-wasm
```

## Completion evidence

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run -p ts2wasm-backend-wasm
result: PASS (8 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m8_oop_classes --test m9_modules
result: PASS (10 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff instanceof_fixture_matches_node_output_under_iwasm m5_array_object_fixtures_match_node_output_under_iwasm
result: PASS (2 passed, 16 skipped)
date: 2026-04-28

command: mise run check-repo-smoke
result: PASS
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (222 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Sweep/free-list reuse is still tracked by 219; mark bits are set but not yet consumed for reclamation.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

