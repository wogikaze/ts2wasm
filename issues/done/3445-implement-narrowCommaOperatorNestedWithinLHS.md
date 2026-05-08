---
id: 3445
title: "Implement Narrowcommaoperatornestedwithinlhs"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed as superseded by `issues/open/5274-parse-general-comma-expressions.md`.
Fresh triage shows the representative path stops at the same parser boundary:
parenthesized comma expressions are rejected before AST construction.

## Problem

Reference test results show 1 case failing in directory
`narrowCommaOperatorNestedWithinLHS` with diagnostics: parser-syntax. Fresh
evidence shows this is not a distinct narrowing blocker yet: the parser rejects
`(otherValue(), value).inner` before AST construction.

Problem: narrowCommaOperatorNestedWithinLHS has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowCommaOperatorNestedWithinLHS.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowCommaOperatorNestedWithinLHS.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing general comma-expression parser owner
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the owner issue

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
- [x] Existing owner contains an exact `reference-triage` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Owner issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowCommaOperatorNestedWithinLHS.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowCommaOperatorNestedWithinLHS.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5274-parse-general-comma-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowCommaOperatorNestedWithinLHS.ts`

## Duplicate detection

- Superseded by `issues/open/5274-parse-general-comma-expressions.md`, which
  owns parser support for parenthesized comma expressions in ordinary
  expression positions.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowCommaOperatorNestedWithinLHS.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowCommaOperatorNestedWithinLHS.ts

result:
Feature label: unknown-unsupported
Diagnostic code: UnsupportedSyntax
Message: comma expressions are not supported in this parser slice at 226..247
Failure line 9, column 20:
if (typeof (otherValue(), value).inner === 'number') {
```

Compiler evidence:

```text
tokens: ok
ast: fails at TypeOfExpression -> PropertyAccessExpression -> ParenthesizedExpression "(otherValue(), value)"
visible symbols: otherValue, value, isNumber
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Closed as superseded by `issues/open/5274-parse-general-comma-expressions.md`;
no new child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowCommaOperatorNestedWithinLHS.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, UnsupportedSyntax for parenthesized comma expression
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowCommaOperatorNestedWithinLHS.ts
result: pass; reproduced UnsupportedSyntax at `(otherValue(), value).inner`
date: 2026-05-08
```

Remaining risks:

- After issue 5274 parses comma expressions, this path may expose a later
  control-flow narrowing requirement for comma-expression member receivers.
