---
id: 3394
title: "Close moduleVariableArrayIndexer bucket after build pass"
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
`reference/typescript/tests/cases/compiler/moduleVariableArrayIndexer.ts` as
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

TypeScript oracle reports a later semantic diagnostic:

```text
TS18050: The value 'undefined' cannot be used here.
```

## Desired final state

This obsolete generated bucket remains closed. No successor issue is created
from this unsupported-build bucket unless semantic coverage later reports a
concrete mismatch.

## Scope

Completed:

- [x] Re-ran focused coverage for `moduleVariableArrayIndexer.ts`.
- [x] Re-ran smart triage for `moduleVariableArrayIndexer.ts`.
- [x] Confirmed the current build path succeeds.

Out of scope:

- Adding semantic parity work without a failing semantic gate.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleVariableArrayIndexer.ts`

## Acceptance criteria

- [x] Focused coverage reports `build_pass`.
- [x] Smart triage reports `BuildPass`.
- [x] The issue is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleVariableArrayIndexer.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleVariableArrayIndexer.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Semantic diagnostic parity for TS18050 should be tracked only if a semantic
coverage gate reports a concrete failure for this path.

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
