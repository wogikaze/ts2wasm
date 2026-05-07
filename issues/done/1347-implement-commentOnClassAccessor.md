---
id: 1347
title: "Implement Commentonclassaccessor"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed as a stale generated reference bucket. The representative
`commentOnClassAccessor2.ts` case now builds successfully and smart triage
reports `BuildPass`, so there is no current compiler blocker to split from this
bucket.

## Problem

Reference test results previously showed 1 case failing in directory
`commentOnClassAccessor` with diagnostics: duplicate-function. Re-running the
exact representative path on 2026-05-06 shows the compiler now accepts the
getter/setter accessor pair.

Problem: stale generated bucket; no implementation-ready child issue is needed
for the current representative path.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnClassAccessor2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnClassAccessor2.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
reference/typescript/tests/cases/compiler/commentOnClassAccessor2.ts: build_pass
Diagnostic: BuildPass / pass
Feature label: build-pass
TypeScript oracle: ok, no diagnostics
```

## Desired final state

This generated bucket is closed as stale with the current smart-triage and
coverage evidence preserved.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale when the representative path is `BuildPass`
- [x] Preserve exact reproduction commands and representative evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded/stale
- [x] Exact `reference-triage` command is preserved
- [x] Current path, diagnostic code, source context, visible symbols, parser AST, and TypeScript oracle evidence are recorded
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnClassAccessor2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnClassAccessor2.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentOnClassAccessor2.ts`

## Duplicate detection

- Smart triage found only this issue by exact path.
- `issues/done/5073-implement-duplicate-function.md` is a broad superseded
  duplicate-function bucket and listed 1347 as a duplicate candidate, but the
  current representative path now builds and does not need to be merged into a
  duplicate-function child.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Build pass: commentOnClassAccessor2

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/commentOnClassAccessor2.ts
```

Source context:

```ts
// @target: es2015
class C {
  /**
   * Getter.
   */
  get bar(): number { return 1;}

  /**
   * Setter.
   */
  set bar(v) { }
}
```

AST/resolved evidence:

```text
ClassDecl C
- Function name: "get bar"
- Function name: "set bar"
Resolved class methods:
- ClassMethod name: "get bar"
- ClassMethod name: "set bar"
```

TypeScript oracle:

```text
ok: true
diagnostics: []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnClassAccessor2.ts --detail --no-dashboard-data
result: build_pass=1, semantic_pass=0, fail=0
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnClassAccessor2.ts
result: BuildPass / pass
date: 2026-05-06
```

Remaining risks:

- Semantic coverage is not enabled for this path, so this closure only proves
  the generated build-blocker bucket is stale.
