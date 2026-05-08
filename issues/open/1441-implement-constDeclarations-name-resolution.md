---
id: 1441
title: "Implement Constdeclarations Name Resolution"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1441.

## Summary

Closed this generated name-resolution bucket after splitting the current
resolver work to `issues/open/5348-resolve-const-declarations-before-use.md`.

## Problem

Fresh triage shows the current two failures are const binding visibility
problems: `constDeclarations-useBeforeDefinition2.ts` reports `UnresolvedName`
for `c` before `const c = 0`, and `constDeclarations.ts` reports
`UnresolvedName` for `c6` in a multi-declarator `for` initializer condition.

Problem: the generated bucket remained blocked instead of pointing to an
implementation-ready resolver slice.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarations-useBeforeDefinition2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations-useBeforeDefinition2.ts --detail
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5348-resolve-const-declarations-before-use.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one observable resolver behavior into an implementation-ready child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match for this exact behavior
- [x] Child issue contains exact `reference-triage` commands
- [x] Child issue includes failing paths, diagnostic codes, source context, and TypeScript evidence
- [x] Child issue acceptance names the exact reference paths and diagnostic changes

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-useBeforeDefinition2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5348-resolve-const-declarations-before-use.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constDeclarations-useBeforeDefinition2.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations.ts`

## Duplicate detection

Split to `issues/open/5348-resolve-const-declarations-before-use.md`.

No exact existing owner was found. Nearby no-match issues:

- `issues/open/064-implement-name-resolution.md` is a test262 metadata bucket.
- `issues/open/437-implement-name-resolution.md` is a broad generated bucket.
- captured-let/argument/assignment name-resolution buckets cover different
  reference windows.

Current evidence:

```text
constDeclarations-useBeforeDefinition2.ts: UnresolvedName `c` at 83..84
constDeclarations.ts: UnresolvedName `c6` at 239..241
```

TypeScript oracle:

- `constDeclarations-useBeforeDefinition2.ts`: TS2448/TS2454 for `c`.
- `constDeclarations.ts`: accepted; hints include binding `c6` from the same
  `for` initializer.

## Current smart triage

Fresh triage was run for both affected files:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-useBeforeDefinition2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations.ts
```

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
