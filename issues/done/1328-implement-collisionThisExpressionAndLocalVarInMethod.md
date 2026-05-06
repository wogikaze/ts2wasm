---
id: 1328
title: "Implement Collisionthisexpressionandlocalvarinmethod"
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

Closed as superseded by
`issues/open/5273-parse-nested-zero-argument-arrow-returns.md`. Fresh triage
shows the first failure is the same nested zero-argument arrow parser gap as
issue 1325.

## Problem

Reference test results show 1 case failing in directory
`collisionThisExpressionAndLocalVarInMethod`. Fresh triage confirms tokens
succeed, but AST construction stops at `(callback) => () => { ... }` inside a
returned object literal.

Problem: `collisionThisExpressionAndLocalVarInMethod.ts` reports
`unsupported expression: Some(SpannedToken { kind: RightParen ... })` at the
second arrow in `doStuff: (callback) => () => { ... }`, which is already
tracked by issue 5273.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInMethod.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInMethod.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is superseded by implementation-ready issue 5273, which
owns nested zero-argument arrow returns. Do not implement directly from this
bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm issue 5273 already covers the current blocker
- [x] Close this generated bucket as superseded
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

- [x] Duplicate candidates below are confirmed and this issue is superseded by 5273
- [x] Issue 5273 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This issue records failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5273 acceptance names the exact parser diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInMethod.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInMethod.ts
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

- [x] none; superseded by existing issue 5273

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInMethod.ts`

Source context:

```ts
class a {
    method1() {
        return {
            doStuff: (callback) => () => {
                var _this = 2;
                return callback(this);
            }
        }
    }
}
```

## Duplicate detection

- `issues/open/5273-parse-nested-zero-argument-arrow-returns.md` is the exact
  owner for `(callback) => () => { ... }` parser failures.
- Broad unknown-unsupported candidates are not exact matches.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInMethod.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInMethod.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 121, end: 122 } }) at 123..125
Source: doStuff: (callback) => () => {
tokens: ok; includes (callback) => () => { ... }
AST: fails at RightParen before the second Arrow token
TypeScript oracle: ok, no diagnostics; nested ArrowFunction under PropertyAssignment
Superseded by: 5273
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInMethod.ts
result: pass; reproduced nested zero-argument arrow parser failure and confirmed issue 5273 owns it
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5273
