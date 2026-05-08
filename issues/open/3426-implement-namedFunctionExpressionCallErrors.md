---
id: 3426
title: "Implement Namedfunctionexpressioncallerrors"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed as an expected negative diagnostic bucket.

Fresh triage shows ts2wasm reports `UnresolvedName` for the out-of-scope
function-expression name `foo`, and the TypeScript oracle reports TS2304 at the
same identifier. There is no current missing-feature blocker to split into a
child issue.

## Problem

Reference test results show 1 cases fail in directory `namedFunctionExpressionCallErrors` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: namedFunctionExpressionCallErrors had 1 generated reference failure
and needed smart-triage evidence before implementation starts.

Disposition: no child issue created because the current diagnostic matches the
reference's intended negative name-resolution behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close expected negative diagnostic bucket when fresh evidence matches the oracle
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
- [x] Fresh evidence contains an exact `reference-triage` command
- [x] Evidence includes path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] No child issue is needed because ts2wasm reports the same intended unresolved-name diagnostic family

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts
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

- `reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts: UnresolvedName: name-resolution
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts

result:
UnresolvedName: unresolved name: `foo` at 101..104
TypeScript oracle reports TS2304 Cannot find name 'foo' at the same identifier.
```

Source context:

```ts
var recurser = function foo() {
};

// Error: foo should not be visible here
foo();

// recurser should be
recurser();

(function bar() {
    // Error: foo should not be visible here either
    foo();
});

// Error: bar should not be visible here
bar();
```

Compiler evidence:

```text
tokens: ok through named function expression assignment, out-of-scope foo(), recurser(), inline function bar(), and out-of-scope bar()
ast: ok; Let recurser = FunctionExpr foo, Expr Call foo(), Expr Call recurser(), Expr FunctionExpr bar, Expr Call bar()
resolved: fails in resolve_names with UnresolvedName for foo at the first intended negative call
visible symbols: binding recurser and function foo from the function expression
```

TypeScript oracle evidence:

```text
TS2304: Cannot find name 'foo'.  // top-level foo()
TS2304: Cannot find name 'foo'.  // foo() inside unrelated function bar()
TS2304: Cannot find name 'bar'.  // top-level bar()
```

## Completion evidence

Closed as expected negative diagnostic; no implementation child created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCallErrors.ts
result: pass; UnresolvedName foo matches TypeScript TS2304 at the same identifier
date: 2026-05-08
```

Remaining risks:

- Later `recurser()` behavior is not reached before the intended `foo`
  diagnostic; initialized function-expression local calls are tracked by
  `5440`.
