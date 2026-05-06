---
id: 1085
title: "Implement Breaktarget"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [209]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage breakTarget across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `breakTarget` with diagnostics: break-continue. Fresh smart triage shows the compiler already parses `break target;` inside a loop and reports `undefined break label target`, while TypeScript reports TS1116 at the same break statement.

Problem: `breakTarget` is not a standalone implementation order; the current failure is an oracle-matching undefined-label diagnostic covered by issue 209 labeled break/continue behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/breakTarget6.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/breakTarget6.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/done/209-implement-labeled-break-continue.md` for the current undefined break-label diagnostic. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 209's labeled break/continue diagnostic behavior
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/breakTarget6.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/breakTarget6.ts
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

- `reference/typescript/tests/cases/compiler/breakTarget6.ts`

## Duplicate detection

- `issues/done/209-implement-labeled-break-continue.md` owns labeled break/continue behavior and invalid label diagnostics.
- `issues/done/035-implement-break-continue.md` owns unlabeled break/continue support.
- Generic break/continue buckets are not exact matches; this bucket is specifically the undefined target label diagnostic.

## Smart triage

### Smart triage: break continue: breakTarget6

- Issue class: `triage-needed`
- Feature label: `break-continue`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/breakTarget6.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/breakTarget6.ts
```

Source context:

```text
1 | // @target: es2015
2 | while (true) {
3 |   break target;
4 | }
```

Current compiler failure:

```text
error: [UnsupportedSyntax] undefined break label `target` at 38..51
```

Compiler evidence:

- Tokens succeed and include `While`, `Break`, `Ident("target")`, and `Semicolon`.
- AST succeeds as `While { body: [Break { label: Some("target") }] }`.
- Validation rejects the undefined break label before lowering.

TypeScript oracle evidence:

```text
TS1116: A 'break' statement can only jump to a label of an enclosing statement.
AST path: WhileStatement -> Block -> BreakStatement.
```

Resolution:

```text
The current compiler diagnostic is an expected invalid labeled-break diagnostic at the same break statement as TypeScript's TS1116. No new implementation child is created from this generated bucket.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/done/209-implement-labeled-break-continue.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/breakTarget6.ts
result: pass; reproduced oracle-matching undefined break-label diagnostic
date: 2026-05-06
```

Remaining risks:

- none
