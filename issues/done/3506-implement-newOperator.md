---
id: 3506
title: "Implement Newoperator"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage newOperator across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed as superseded by the completed issue 5150 empty element access
diagnostic boundary.

## Problem

Reference test results show 1 cases fail in directory `newOperator` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: newOperator has 1 current reference failure, but fresh triage shows
the first blocker is the already implemented issue-5150 diagnostic for empty
element access `expr[]`, not a new broad `new` operator implementation slice.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newOperator.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newOperator.ts --detail
```

## Desired final state

This generated bucket is closed. Do not implement from this issue; the current
observed behavior is covered by
`issues/done/5150-report-empty-element-access-diagnostics.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close the bucket as superseded by an existing completed diagnostic owner
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing owner contains the relevant diagnostic contract
- [x] Triage evidence includes failing path, diagnostic code, source context,
  visible symbols, parser evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic boundary

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newOperator.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newOperator.ts
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

- [x] superseded by `issues/done/5150-report-empty-element-access-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/newOperator.ts`

## Duplicate detection

- `issues/done/5150-report-empty-element-access-diagnostics.md` owns the
  current issue-5150 empty element access diagnostic.
- `issues/done/1104-implement-cannotInvokeNewOnErrorExpression.md` already
  closes the adjacent `new M.ClassA[]` generated bucket against the same
  completed issue-5150 diagnostic.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage unknown unsupported: newOperator

- Issue class: triage-needed
- Feature label: unknown-unsupported
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/newOperator.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: issue-5150: empty element access `expr[]` requires an index expression
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0

reference/typescript/tests/cases/compiler/newOperator.ts: UnsupportedSyntax: unknown-unsupported
```

Representative source includes multiple `new` forms; the first current parser
boundary is an empty element access after a type-only `string` callee:

```ts
var t3 = new string[]( );
var t4 =
new
string
[
    ]
    (
        );
```

The same file later includes the class-member form:

```ts
return new M.T[];
```

Compiler evidence:

```text
tokens: ok
ast/resolved: fail with issue-5150 empty element access diagnostic
visible symbols before failure include i, x, y, t1, t2, t3, t4, f, t5,
union, ctorUnion, ctorUnion2, namespace class T, and class S
```

TypeScript oracle evidence:

```text
TS1011: An element access expression should take an argument.
TS2693: 'string' only refers to a type, but is being used as a value here.
Additional later TS2351/TS2304/TS2564 diagnostics are present after this
empty-element-access boundary.
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newOperator.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newOperator.ts
result: pass; reproduced completed issue-5150 empty element access diagnostic
date: 2026-05-08
```

Remaining risks:

- none for this generated bucket; broader semantic diagnostics in
  `newOperator.ts` remain hidden behind or after the completed issue-5150
  parser diagnostic and are not tracked by this generated bucket.
