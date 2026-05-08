---
id: 3393
title: "Close moduleUnassignedVariable bucket after build pass"
type: maintenance
area: frontend/syntax
class: completed
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import-export bucket because the current compiler already
builds the representative reference path.

## Problem

The original issue was generated from an older coverage window that classified
`reference/typescript/tests/cases/compiler/moduleUnassignedVariable.ts` as
`import-export`.

Fresh triage now reports:

```text
BuildPass: ts2wasm build succeeded
```

## Current failure

None for the current build step.

Focused coverage result on 2026-05-08:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
```

## Desired final state

This obsolete generated bucket remains closed. No successor issue is needed
unless a future semantic coverage gate reports a concrete mismatch.

## Scope

Completed:

- [x] Re-ran focused coverage for `moduleUnassignedVariable.ts`.
- [x] Re-ran smart triage for `moduleUnassignedVariable.ts`.
- [x] Confirmed the current build path succeeds.

Out of scope:

- Adding semantic parity work without a failing semantic gate.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleUnassignedVariable.ts`

## Acceptance criteria

- [x] Focused coverage reports `build_pass`.
- [x] Smart triage reports `BuildPass`.
- [x] The issue is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleUnassignedVariable.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleUnassignedVariable.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

TypeScript oracle reports no diagnostics for this path in the current triage
view, so no semantic follow-up is created from this bucket.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage commands listed above
result: pass; coverage build_pass=1, triage BuildPass
date: 2026-05-08
```

Remaining risks:

- none for build coverage.
