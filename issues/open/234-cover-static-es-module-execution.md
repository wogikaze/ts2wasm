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
---

## Summary

Add execution fixtures and differential coverage that prove the supported static ES module slice behaves like Node for the implemented subset.

## Problem

Module build smoke tests do not prove semantic parity. Once parser, graph, and emission work lands, the repo needs explicit fixtures showing static imports/exports execute correctly and remain classified separately from unsupported module features.

## Desired final state

The implemented static ES module subset has Node differential coverage for simple local relative modules, repeated imports, re-exports supported by the implementation, and unsupported forms that remain diagnostic-only.

## Scope

In scope:

- [ ] Add multi-file fixtures under `fixtures/module-system/`
- [ ] Add Node differential tests for named import/export execution
- [ ] Add build or diagnostic tests for still-unsupported module forms
- [ ] Ensure module fixtures are no longer only build-smoke when they claim semantic parity
- [ ] Update issue/current-state evidence if semantic status changes

Out of scope:

- [ ] Implement missing parser, resolver, lowering, or backend behavior
- [ ] Dynamic import, package resolution, TypeScript path mapping, and CommonJS require parity

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

- [ ] At least three static ES module fixtures run with Node differential parity
- [ ] Repeated import/module initialization behavior is covered
- [ ] Unsupported module forms remain covered by diagnostic tests
- [ ] `current-state.md` no longer describes the implemented static ES module subset as build-smoke-only
- [ ] No regression in existing module, semantic-diff, or host-deny tests

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

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Node differential evidence is required for semantic claims in this issue.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
