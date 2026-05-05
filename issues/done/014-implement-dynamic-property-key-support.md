# Implement dynamic property key support

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Closed**: 2026-04-26
**ID**: 014
**Type**: feature
**Area**: runtime/semantics
**Priority**: P1
**Depends on**: 012
**Orchestration class**: implementation-ready

Problem: Dynamic property keys (e.g., `obj[variable]`) are not implemented. It previously diagnosed as `unsupported-dynamic-property`.

Scope:

- Implement lowering for dynamic property access with non-literal keys.
- Extend `$property_get` and `$property_set` to handle runtime string keys.
- Add fixtures for dynamic property access patterns.
- Ensure Node differential test passes.

Acceptance Criteria:

- [x] Dynamic property access `obj[key]` works with string variables.
- [x] Dynamic property set `obj[key] = value` works correctly.
- [x] Node differential test passes for dynamic property fixtures.
- [x] Diagnostic `unsupported-dynamic-property` is removed for supported cases.
- [x] `unsupported-dynamic-property` diagnosis is no longer used for supported dynamic key cases.

Validation:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli m5_array_object_fixtures_match_node_output_under_iwasm -- --exact
cargo test -p ts2wasm-cli m5_edge_case_fixtures_match_node_output_under_iwasm -- --exact
cargo nextest run
iwasm fixtures/arrays-objects/dynamic-property.wasm
iwasm fixtures/arrays-objects/dynamic-property-assignment.wasm
```

## Completion evidence

- 2026-04-26: `cargo fmt --all --check` passed.
- 2026-04-26: `cargo test -p ts2wasm-cli m5_array_object_fixtures_match_node_output_under_iwasm -- --exact` passed (`fixtures/arrays-objects/dynamic-property.ts`, `fixtures/arrays-objects/dynamic-property-assignment.ts` behavior validated via `iwasm`).
- 2026-04-26: `cargo test -p ts2wasm-cli m5_edge_case_fixtures_match_node_output_under_iwasm -- --exact` passed (`array-nonnumber-index` behavior preserved).
- 2026-04-26: `cargo nextest run` passed with 135/201 passing and one known environment-specific skip/fail due missing `reference/test262`.
- 2026-04-26: `iwasm fixtures/arrays-objects/dynamic-property.wasm` outputs `2` and `20`.
- 2026-04-26: `iwasm fixtures/arrays-objects/dynamic-property-assignment.wasm` outputs `9` and `99`.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/014-implement-dynamic-property-key-support.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
