---
id: 1088
title: "Implement Cacheresolutions"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5175]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage cacheResolutions across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `cacheResolutions` with diagnostics: import-export. Fresh smart triage shows the compiler currently stops on `export let x = 1;` with the generic variable-export boundary before it can reach the intended duplicate binding/module-resolution behavior.

Problem: `cacheResolutions` is not a standalone implementation order; the current blocker is the same `export let` parser/module-syntax gap tracked by issue 5175.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cacheResolutions.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cacheResolutions.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/5175-support-export-let-destructuring-declarations.md`, which now owns both identifier and destructuring `export let` declarations.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold this bucket into the existing implementation-ready `export let` child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the owner issue

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cacheResolutions.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cacheResolutions.ts
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

- [x] folded into `issues/open/5175-support-export-let-destructuring-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cacheResolutions.ts`

## Duplicate detection

- `issues/open/5175-support-export-let-destructuring-declarations.md` is an exact owner for the current `export let` variable-export parser/module-syntax boundary.
- Broad import/export buckets are not exact matches; this bucket stops before module-resolution caching behavior.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/cacheResolutions.ts`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-055: unsupported variable export; module resolution and loading are not implemented at 114..120`
- Source context: `export let x = 1;`
- Compiler evidence: tokens succeed for three `export let x = 1;` declarations; AST construction fails at the first `Export` token.
- TypeScript oracle: accepts the syntax and reports three TS2451 duplicate block-scoped variable diagnostics for `x`.
- Superseded by child: `issues/open/5175-support-export-let-destructuring-declarations.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cacheResolutions.ts
result: pass; current blocker is `export let` variable-export parsing, folded into issue 5175
date: 2026-05-06
```

Remaining risks:

- none
