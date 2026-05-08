---
id: 3425
title: "Implement Namedfunctionexpressioncall"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed after splitting the current blocker into implementation-ready child
issue `5440-support-initialized-function-expression-local-calls.md`.

Fresh triage shows the parser and resolver handle the named function
expressions, but lowering stops at the generic issue-211 function-valued local
call boundary for `recurser()`.

## Problem

Reference test results show 1 cases fail in directory `namedFunctionExpressionCall` with diagnostics: method-call. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: namedFunctionExpressionCall has 1 reference failure and needed
smart-triage evidence before implementation starts.

Disposition: implementation work is tracked by child issue `5440`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5440-support-initialized-function-expression-local-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts: UnsupportedSyntax: unknown-unsupported
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts

result:
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `recurser(...)` are not supported; call receiver.method(...) directly at 140..150
```

Source context:

```ts
var recurser = function foo() {
    // using the local name
    foo();

    // using the globally visible name
    recurser();
};

(function bar() {
    bar();
});
```

Compiler evidence:

```text
tokens: ok through named function expression assignment and inline named function expression
ast: Let recurser = FunctionExpr name foo; body contains Call foo() and Call recurser()
resolved/lowered: lower_program fails at issue-211 for recurser()
visible symbols: binding recurser and function foo
```

TypeScript oracle evidence:

```text
ok; no diagnostics
binding recurser has type () => void
```

## Completion evidence

Split into:

- `5440-support-initialized-function-expression-local-calls.md`

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts
result: pass; issue-211 function-valued local call at recurser()
date: 2026-05-08
```

Remaining risks:

- Inline `bar()` self-call may expose a later diagnostic after `recurser()`
  advances.
