---
id: 1335
title: "Implement Collisionthisexpressionandvaringlobal"
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1335.

## Summary

Closed as superseded by `issues/done/062d-function-this-and-arguments.md` for the current oracle-matching top-level arrow `this` diagnostic.

## Problem

Fresh triage shows the top-level `_this` variable parses and the current first blocker is the top-level arrow `this` expression:

```ts
var _this = 1;
var f = () => this;
```

`ts2wasm` reports issue-062d for unsupported top-level `this`, and the TypeScript oracle reports TS7041 at the same source span. That behavior is already owned by the completed issue 062d receiver/`arguments` diagnostics slice.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndVarInGlobal.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndVarInGlobal.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing issue-062d diagnostic owner rather than split a duplicate child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closure issue

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
- [x] Exact `reference-triage` and focused `reference-coverage` commands are preserved below
- [x] Issue evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Closure names the exact issue-062d diagnostic/stdout behavior

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndVarInGlobal.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndVarInGlobal.ts
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

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndVarInGlobal.ts`

## Duplicate detection

- `issues/done/062d-function-this-and-arguments.md` owns the current issue-linked unsupported top-level `this` diagnostic policy.
- Other global-collision buckets with class/function/enum/namespace/ambient declarations have already been closed against the same current oracle-matching diagnostic where applicable.

## Smart triage

Generated 2026-05-07.

Command:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndVarInGlobal.ts
```

Result:

```text
Smart triage: Triage class: collisionThisExpressionAndVarInGlobal
Feature label: class
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported

Failure:
4 | var f = () => this;
                    ^^^^
error: [UnsupportedSyntax] issue-062d: `this` is only supported inside receiver-bound functions, class constructors, and instance methods in this milestone at 69..73

TypeScript oracle:
TS7041 at line 4, character 15:
The containing arrow function captures the global value of 'this'.
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndVarInGlobal.ts --detail --no-dashboard-data
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

- Tokens: ok for `var _this = 1;` and `var f = () => this;`.
- AST: ok; contains `Let _this = Number(1)` and `Let f = ArrowFn { body: This }`.
- Resolved/lowered: fails on top-level arrow `this` with issue-062d.
- TypeScript oracle: top-level AST has both variable statements and reports TS7041 at the same `this` span.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closure commit pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndVarInGlobal.ts
result: pass; reproduced issue-062d UnsupportedSyntax matching TypeScript TS7041 for top-level arrow this
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndVarInGlobal.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

Remaining risks:

- none for this generated bucket; the current first failure is already covered by issue-062d diagnostics.
