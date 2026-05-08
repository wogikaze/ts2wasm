---
id: 1419
title: "Implement Conditionalexpressionnewline"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1419.

## Summary

Triage conditionalExpressionNewLine across 10 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 10 cases in directory
`conditionalExpressionNewLine` with name-resolution diagnostics. Fresh triage
on 2026-05-07 shows the representative source is invalid TypeScript and the
compiler's current `UnresolvedName` diagnostic matches the first TypeScript
TS2304 oracle diagnostic.

Problem: conditionalExpressionNewLine has 10 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalExpressionNewLine1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalExpressionNewLine1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as oracle-matching unresolved-name diagnostics
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

- [x] Duplicate candidates below are confirmed as no-match for required implementation work
- [x] No child issue needed because the first diagnostic is already oracle-matching
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact 10-file reference window and diagnostic classification

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 20
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalExpressionNewLine1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalExpressionNewLine1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine1.ts`
- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine4.ts`
- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine10.ts`
- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine5.ts`
- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine2.ts`
- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine3.ts`
- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine8.ts`
- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine6.ts`
- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine9.ts`
- `reference/typescript/tests/cases/compiler/conditionalExpressionNewLine7.ts`

## Duplicate detection

- Broad name-resolution duplicate candidates are not a match: the current
  representative diagnostic is a genuine unresolved value name.
- Issue 056 established `UnresolvedName` as the compiler diagnostic for
  genuinely missing identifiers; no implementation child is needed for this
  bucket.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
Path: reference/typescript/tests/cases/compiler/conditionalExpressionNewLine1.ts
Failure: unresolved name: `a` at 27..28
line 2, column 9
Source context:
1 | // @target: es2015
2 | var v = a ? b : c;
Visible symbols:
- binding v
```

Compiler evidence:

```text
tokens: ok; Ident("a") Question Ident("b") Colon Ident("c")
ast: ok; Let v = Ternary { condition: Ident("a"), then_expr: Ident("b"), else_expr: Ident("c") }
resolved: UnresolvedName for `a` at 27..28
TypeScript oracle: TS2304 Cannot find name 'a' at 27..28; later TS2304 for `b` and `c`
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter conditionalExpressionNewLine --detail --no-dashboard-data
result: executed=10, build_pass=0, unsupported=10, blocked=0, unsupported_diagcodes=UnresolvedName:10, unsupported_features=name-resolution:10
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as oracle-matching unresolved-name diagnostics; no child issue created.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- The runner still classifies the cases as unsupported because it records the first compiler diagnostic rather than comparing it to the TypeScript TS2304 oracle.
