---
id: 1333
title: "Implement Collisionthisexpressionandparameter"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1333.

## Summary

Closed as superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md` for the current nested zero-argument arrow parser failure.

## Problem

Fresh triage confirms this generated bucket is too broad for direct implementation. The current first blocker is not the later `_this` parameter collision behavior. Parsing stops in this constructor object-literal property initializer:

```ts
class Foo1 {
    constructor(_this: number) {
        var x2 = {
            doStuff: (callback) => () => {
                return callback(this);
            }
        }
    }
}
```

The failing shape is the same `(callback) => () => { ... }` nested zero-argument arrow expression already tracked by issue 5273.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndParameter.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndParameter.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5273 instead of splitting a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in issue 5273 and this closure

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
- [x] Superseding issue 5273 contains exact parser failure evidence for this path
- [x] Superseding issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact nested zero-argument arrow parser change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndParameter.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndParameter.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected: issue metadata only

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndParameter.ts`

## Duplicate detection

- `issues/open/5273-parse-nested-zero-argument-arrow-returns.md` owns the current `(callback) => () => { ... }` parser failure.
- Constructor `_this` parameter collision behavior, overload handling, ambient `declare`, and duplicate global `console` diagnostics remain unproven until issue 5273 advances past the parser failure.

## Smart triage

Generated 2026-05-07.

Command:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndParameter.ts
```

Result:

```text
Smart triage: Triage unknown unsupported: collisionThisExpressionAndParameter
Feature label: unknown-unsupported
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 1071, end: 1072 } }) at 1073..1075
Failure location: line 39, column 39
```

Source context:

```text
36 | class Foo1 {
37 |     constructor(_this: number) { // Error
38 |         var x2 = {
39 |             doStuff: (callback) => () => {
40 |                 return callback(this);
41 |             }
42 |         }
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndParameter.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Compiler evidence:

- Tokens: ok through the `Foo1` constructor object literal and the nested `(callback) => () => { return callback(this); }` shape.
- AST/resolved: fail at the second arrow's empty parameter list with `UnsupportedSyntax`.
- Visible symbols before failure include class `Foo`, nested function `inner`, arrow bindings `lamda`/`lambda`, class `Foo1`, and binding `x2`.
- TypeScript oracle accepts the nested arrow shape and reports TS2683 implicit-`this` diagnostics plus duplicate global `console` diagnostics later in the file.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closure commit pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndParameter.ts
result: pass; reproduced nested zero-argument arrow parser failure and updated issue 5273
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndParameter.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

Remaining risks:

- Later `_this` parameter collision behavior remains unproven until issue 5273 advances past the nested arrow parser failure.
- Duplicate global `console`, ambient declaration, overload, and implicit `this` compatibility diagnostics are outside this closure.
