---
id: 1325
title: "Implement Collisionthisexpressionandlocalvarinconstructor"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed after splitting the current blocker to
`issues/open/5273-parse-nested-zero-argument-arrow-returns.md`. Fresh triage
shows the first failure is a parser gap for a nested zero-argument arrow
returned from another arrow in an object literal property.

## Problem

Reference test results show 1 case failing in directory
`collisionThisExpressionAndLocalVarInConstructor`. Fresh triage confirms tokens
succeed, but AST construction stops at `(callback) => () => { ... }`.

Problem: `collisionThisExpressionAndLocalVarInConstructor.ts` reports
`unsupported expression: Some(SpannedToken { kind: RightParen ... })` at the
second arrow in `doStuff: (callback) => () => { ... }`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm related arrow issues do not exactly own this nested no-argument arrow parser gap
- [x] Split one observable behavior into child issue 5273
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

- [x] Duplicate candidates below are confirmed and the exact behavior is split to 5273
- [x] Child issue 5273 contains an exact `python scripts/manager.py reference-triage ...` command
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts
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

- [x] created: `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts`

Source context:

```ts
class class1 {
    constructor() {
        var x2 = {
            doStuff: (callback) => () => {
                var _this = 2;
                return callback(this);
            }
        }
    }
}
```

## Duplicate detection

- `issues/done/5240-w2-docs-audit-and-stale-entries.md` is related but
  not exact: it handles async arrows, not plain nested `() =>`.
- `issues/done/5152-support-class-constructor-outer-callback-captures.md` is
  related but not exact: it handles constructor nested callback lowering after
  parsing succeeds.
- Existing broad arrow-function issues are done or own later lowering/runtime
  behavior, not this AST construction failure at a nested zero-argument arrow.
- No exact implementation-ready issue owned this parser gap, so this bucket was
  split to issue 5273.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 132, end: 133 } }) at 134..136
Source: doStuff: (callback) => () => {
tokens: ok; includes (callback) => () => { ... }
AST: fails at RightParen before the second Arrow token
TypeScript oracle: ok, no diagnostics; nested ArrowFunction under PropertyAssignment
Child issue: 5273
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts
result: pass; reproduced nested zero-argument arrow parser failure and split child issue 5273
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5273
