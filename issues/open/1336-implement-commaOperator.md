---
id: 1336
title: "Implement Commaoperator"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1336.

## Summary

Closed after splitting the current blocker to
`issues/open/5274-parse-general-comma-expressions.md`. Fresh triage shows the
first failure is a parser gap for comma expressions in ordinary expression
positions.

## Problem

Reference test results show 1 case failing in directory `commaOperator`.
Fresh triage confirms tokens succeed, but AST construction stops on a
parenthesized comma expression in a variable initializer.

Problem: `commaOperator1.ts` reports `comma expressions are not supported in
this parser slice` at `((1, 2, 3), 4, 5, (6, 7))`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperator1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commaOperator1.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm related comma-expression issues do not exactly own this general expression gap
- [x] Split one observable behavior into child issue 5274
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

- [x] Duplicate candidates below are confirmed and the exact behavior is split to 5274
- [x] Child issue 5274 contains an exact `python scripts/manager.py reference-triage ...` command
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commaOperator1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperator1.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5274-parse-general-comma-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commaOperator1.ts`

Source context:

```ts
var v1 = ((1, 2, 3), 4, 5, (6, 7));
function f1() {
    var a = 1;
    return a, v1, a;
}
```

## Duplicate detection

- `issues/open/5182-parse-comma-separated-for-update-expressions.md` is
  related but explicitly out of scope for general comma expression support.
- `issues/open/1338-implement-commaOperatorLeftSideUnused.md` is related but
  is a blocked generated bucket for TS2695 diagnostics after parser support.
- No exact implementation-ready issue owned general comma expressions, so this
  bucket was split to issue 5274.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commaOperator1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperator1.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: comma expressions are not supported in this parser slice at 63..72
Source: var v1 = ((1, 2, 3), 4, 5, (6, 7));
tokens: ok; includes nested comma tokens in initializer and return expression
AST: fails at the first parenthesized comma expression
TypeScript oracle: parses BinaryExpression comma chains and reports TS2695 diagnostics
Child issue: 5274
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5274-parse-general-comma-expressions.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperator1.ts
result: pass; reproduced general comma expression parser failure and split child issue 5274
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5274
