---
id: 1030
title: "Implement Badoverloaderror"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage badOverloadError across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `badOverloadError` with diagnostics: unknown-unsupported. Fresh smart triage on 2026-05-06 shows the case now builds successfully, so this generated bucket is stale.

Problem: `badOverloadError` no longer has a current compiler blocker; no child implementation issue is needed for this generated bucket.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badOverloadError.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badOverloadError.ts --detail
```

## Desired final state

This generated bucket is closed as stale because the only affected reference case currently reports `BuildPass` / `pass`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] No child issue created because fresh triage found no current compiler blocker
- [x] Preserve exact reproduction commands and representative diagnostic evidence in this closed issue

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is stale
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes the reference path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence records the exact fixture/reference path and diagnostic result

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badOverloadError.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/badOverloadError.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/badOverloadError.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Build pass: badOverloadError

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/badOverloadError.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badOverloadError.ts
```

Source context:

```text
// @target: es2015
function method() {
    var dictionary = <{ [index: string]: string; }>{};
}
```

Compiler evidence:

```text
AST and resolved dumps succeed. The type assertion is erased and `dictionary` initializes to an empty object literal.
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none.
Binding hint: `dictionary` has type `{ [index: string]: string; }`.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending local commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badOverloadError.ts
result: pass; emitted BuildPass / pass smart-triage report for the only affected reference path
date: 2026-05-06
```

Remaining risks:

- none
