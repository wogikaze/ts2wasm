---
id: 1243
title: "Implement Classusedbeforeinitializedvariables"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed after splitting the current blocker to
`issues/open/5269-parse-optional-class-property-declarations.md`. Fresh triage
shows the first failure is a parser gap for optional class property
declarations.

## Problem

Reference test results show 1 case fails in directory
`classUsedBeforeInitializedVariables`. Fresh triage confirms the current
blocker is the optional class property declaration `p5?: number;`.

Problem: `classUsedBeforeInitializedVariables.ts` reports `expected LeftParen,
got Some(Question)` at `p5?: number;`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm related issue 3437 records the same gap but no implementation-ready child exists
- [x] Split one observable behavior into child issue 5269
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

- [x] Duplicate candidates below are confirmed and the exact behavior is split to 5269
- [x] Child issue 5269 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5269-parse-optional-class-property-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts`

Source context:

```ts
class Test {
    p1 = 0;
    p2 = this.p1;
    p3 = this.p4;
    p4 = 0;
    p5?: number;

    p6?: string;
}
```

## Duplicate detection

- `issues/done/3437-implement-narrowByBooleanComparison.md` records the same
  `Question` parser gap at `status?: number;`, but did not create an
  implementation-ready child.
- No existing child issue owned optional class property declarations, so this
  bucket was split to issue 5269.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: expected LeftParen, got Some(Question) at 111..112
Source: p5?: number;
tokens: ok; Ident("p5"), Question, Colon, Ident("number"), Semicolon
AST: fails before optional class property construction
resolved: same parser failure
TypeScript oracle: parses; expected diagnostics include TS2729 and TS2683
Child issue: 5269
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5269-parse-optional-class-property-declarations.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts
result: pass; reproduced optional class property parser failure and split child issue 5269
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5269
