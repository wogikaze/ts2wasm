---
id: 234
title: "Cover static ES module execution"
type: test
area: tests/fixtures
class: implementation-ready
priority: P1
depends_on: [231, 232, 233]
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
status: done
---

## Summary

Add execution fixtures and differential coverage that prove the supported static ES module slice behaves like Node for the implemented subset.

## Problem

Module build smoke tests do not prove semantic parity. Once parser, graph, and emission work lands, the repo needs explicit fixtures showing static imports/exports execute correctly and remain classified separately from unsupported module features.

## Desired final state

The implemented static ES module subset has Node differential coverage for simple local relative modules, repeated imports, re-exports supported by the implementation, and unsupported forms that remain diagnostic-only.

## Scope

In scope:

- [x] Add multi-file fixtures under `fixtures/module-system/`
- [x] Add Node differential tests for named import/export execution
- [x] Add build or diagnostic tests for still-unsupported module forms
- [x] Ensure module fixtures are no longer only build-smoke when they claim semantic parity
- [x] Update issue/current-state evidence if semantic status changes

Out of scope:

- [x] Implement missing parser, resolver, lowering, or backend behavior
- [x] Dynamic import, package resolution, TypeScript path mapping, and CommonJS require parity

## Affected paths

Expected:

- `fixtures/module-system/`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/m9_modules.rs`
- `current-state.md`
- `issues/open/055-implement-import-export.md`

Do not touch:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `docs/`

## Acceptance criteria

- [x] At least three static ES module fixtures run with Node differential parity
- [x] Repeated import/module initialization behavior is covered
- [x] Unsupported module forms remain covered by diagnostic tests
- [x] `current-state.md` no longer describes the implemented static ES module subset as build-smoke-only
- [x] No regression in existing module, semantic-diff, or host-deny tests

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli m2_static_es_modules
cargo nextest run -p ts2wasm-cli module
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli m2_node_diff
scripts/manager check-fixture-catalog
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

Node differential evidence is required for semantic claims in this issue.

## Completion evidence

Commits:

- `f7fb0de` issue-234: close static esm execution coverage

Validation result:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-cli static_named_module_import_fixtures_match_node_output_under_iwasm: PASS (1 test, 236 skipped)
cargo nextest run -p ts2wasm-cli module: PASS (17 tests, 220 skipped)
scripts/manager update-issue-index --check: PASS
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
scripts/manager check-repo-smoke: PASS
cargo nextest run: PASS (382 tests, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- none

## Close evidence

2026-04-28 child worker `234-static-esm-exec-close-20260428T130000Z` audited the static ES module execution coverage landed with issue 233 and closed issue 234 without compiler behavior changes.

Acceptance evidence:

- Node/iwasm differential parity is covered by the `static_named_module_import_fixtures_match_node_output_under_iwasm` test in `crates/cli/tests/m2_node_diff.rs` for four local relative static named import fixtures: `static-entry.ts`, `static-entry-alias.ts`, `static-entry-shadow.ts`, and `static-entry-repeated.ts`.
- Repeated import/module initialization behavior is covered by `fixtures/module-system/static-entry-repeated.ts`, which imports the same source module twice and matches Node output under iwasm.
- Unsupported module forms remain covered by `crates/cli/tests/m9_modules.rs` diagnostics for missing modules, bare specifiers, unsupported declaration/class/default exports, re-exports, namespace imports, default imports, and missing named exports.
- `current-state.md` already records that the narrow static named ES module import/export subset has Node/iwasm differential coverage rather than build-smoke-only status.
- No parser, resolver, lowering, backend, or docs files were changed in this close-only branch.
