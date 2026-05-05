---
id: 1026
title: "Implement Badarrayindex"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage badArrayIndex across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `badArrayIndex` with diagnostics: unknown-unsupported. Fresh smart triage shows this bucket is a narrow empty element access diagnostic gap and is superseded by `issues/done/5150-report-empty-element-access-diagnostics.md`.

Problem: `badArrayIndex` is not a standalone implementation order; the executable parser diagnostic work is split to issue 5150.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badArrayIndex.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badArrayIndex.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by an implementation-ready child issue. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badArrayIndex.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/badArrayIndex.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5150-report-empty-element-access-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/badArrayIndex.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: unknown unsupported: badArrayIndex

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/badArrayIndex.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badArrayIndex.ts
```

Source context:

```text
// @target: es2015
var results = number[];
```

Current compiler failure:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightBracket, span: Span { start: 40, end: 41 } }) at 41..42
```

Token evidence:

```text
Var results = Ident("number") LeftBracket RightBracket ;
```

TypeScript oracle evidence:

```text
TS2693: 'number' only refers to a type, but is being used as a value here.
TS1011: An element access expression should take an argument.
AST path: ElementAccessExpression, text `number[]`.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/done/5150-report-empty-element-access-diagnostics.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badArrayIndex.ts
result: pass; reproduced generic UnsupportedSyntax on empty element access
date: 2026-05-06
```

Remaining risks:

- none
