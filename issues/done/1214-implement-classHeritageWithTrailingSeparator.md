---
id: 1214
title: "Implement Classheritagewithtrailingseparator"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5260]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed by splitting the current class heritage trailing comma parser diagnostic
to `issues/open/5260-report-class-heritage-trailing-comma.md`.

## Problem

Reference test results showed 1 case in `classHeritageWithTrailingSeparator`
with diagnostic `parser-syntax`.

Problem: fresh triage shows the parser reaches `class D extends C, {}` and
reports `expected LeftBrace, got Some(Comma)` instead of a source-spanned
TypeScript trailing-comma diagnostic.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no existing open/done issue owns the exact current boundary
- [x] Split one observable behavior into an implementation-ready child issue
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

- [x] Duplicate candidates below are confirmed as no-match for the exact current boundary
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5260-report-class-heritage-trailing-comma.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts`

Source context:

```ts
class C { foo: number }
class D extends C, {
}
```

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: expected LeftBrace, got Some(Comma) at 62..63
Tokens: ok
AST: fails at comma in `class D extends C, {`
```

TypeScript reports `TS1009: Trailing comma not allowed.` at the same comma and
still produces an AST path through `ClassDeclaration -> HeritageClause ->
ExpressionWithTypeArguments -> Identifier`.

Split issue:

- `issues/open/5260-report-class-heritage-trailing-comma.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5260-report-class-heritage-trailing-comma.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classHeritageWithTrailingSeparator.ts
result: pass; current blocker split to issue 5260
date: 2026-05-06
```

Remaining risks:

- The same reference also has TypeScript definite-assignment diagnostic TS2564
  for `foo`, but the current ts2wasm blocker is the trailing comma parser
  boundary.
