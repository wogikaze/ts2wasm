---
id: 1337
title: "Implement Commaoperatorinconditionalexpression"
type: spike
area: frontend/semantics
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1337.

## Summary

Closed as superseded by `issues/done/5228-w0-wasm-binary-backend-mvp.md` for the current computed object-literal key parser failure.

## Problem

Fresh triage confirms this generated bucket is too broad for direct implementation. The current first blocker is not comma-operator or ternary lowering. Parsing stops at the first computed object-literal key:

```ts
return true ? { [m]: i } : { [m]: i + 1 }
```

The parser reports `UnsupportedSyntax: expected Dot, got Some(RightBracket) at 97..98`, which is the same simple computed object key boundary already tracked by issue 5228.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/commaOperatorInConditionalExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/commaOperatorInConditionalExpression.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5228 instead of splitting a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in issue 5228 and this closure

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
- [x] Superseding issue 5228 contains exact parser failure evidence for this path
- [x] Superseding issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact simple computed object-literal key parser change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/commaOperatorInConditionalExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/commaOperatorInConditionalExpression.ts
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

- `reference/typescript/tests/cases/compiler/commaOperatorInConditionalExpression.ts`

## Duplicate detection

- `issues/done/5228-w0-wasm-binary-backend-mvp.md` owns the current `{ [m]: i }` parser failure.
- Ternary/conditional expression lowering and comma-operator semantics remain unproven until issue 5228 advances past the computed object key parser boundary.

## Smart triage

Generated 2026-05-07.

Command:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperatorInConditionalExpression.ts
```

Result:

```text
Smart triage: Triage type system: commaOperatorInConditionalExpression
Feature label: type-system
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: expected Dot, got Some(RightBracket) at 97..98
Failure location: line 4, column 29
```

Source context:

```text
1 | // @target: es2015
2 | function f (m: string) {
3 |     [1, 2, 3].map(i => {
4 |         return true? { [m]: i } : { [m]: i + 1 }
5 |     })
6 | }
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commaOperatorInConditionalExpression.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-system:1
semantic_enabled=0
```

Compiler evidence:

- Tokens: ok through function `f`, array literal, `.map`, arrow callback, ternary tokens, and computed object key tokens `LeftBracket Ident("m") RightBracket Colon`.
- AST/resolved: fail with `UnsupportedSyntax: expected Dot, got Some(RightBracket) at 97..98`.
- TypeScript oracle: accepts the source with no diagnostics; AST path reaches `ConditionalExpression -> ObjectLiteralExpression -> PropertyAssignment -> ComputedPropertyName`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closure commit pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperatorInConditionalExpression.ts
result: pass; reproduced simple computed object-literal key parser failure and updated issue 5228
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commaOperatorInConditionalExpression.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=type-system:1
date: 2026-05-07
```

Remaining risks:

- Ternary/conditional expression lowering and comma-operator semantics remain unproven until issue 5228 advances past the computed object key parser boundary.
