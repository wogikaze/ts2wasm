---
id: 3448
title: "Implement Narrowtypebyinstanceof"
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

Closed after split to `issues/open/5448-support-class-constructor-values-in-instanceof-rhs.md`.
Fresh triage shows this bucket parses successfully and then stops at the
`issue-5011` class constructor value boundary for an `instanceof` RHS.

## Problem

Reference test results show 1 case failing in directory
`narrowTypeByInstanceof` with diagnostics: parser-syntax. Fresh evidence shows
the current blocker is no longer parser syntax: AST construction succeeds, and
name resolution rejects `FileMatch` as a class value in
`elementA instanceof FileMatch`.

Problem: narrowTypeByInstanceof has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split this bucket to a narrow class-constructor `instanceof` RHS owner
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
- [x] Child issue contains an exact `reference-triage` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts
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

- [x] `issues/open/5448-support-class-constructor-values-in-instanceof-rhs.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts`

## Duplicate detection

- Split to `issues/open/5448-support-class-constructor-values-in-instanceof-rhs.md`,
  which owns direct class constructor values used as `instanceof` RHS operands.

Related but distinct:

- `issues/open/5192-support-first-class-class-constructor-values.md` owns
  broader class constructor values flowing through ordinary expressions and
  factory-call arguments.
- `issues/open/5447-support-instanceof-callable-prototype-rhs.md` owns
  `issue-207` for callable/prototype `instanceof` RHS values, not direct class
  declarations rejected by `issue-5011`.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts --detail --no-dashboard-data

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts

result:
Feature label: class
Diagnostic code: UnsupportedSyntax
Message: issue-5011: class `FileMatch` cannot be used as a value — class runtime is not yet supported at 367..376
Failure line 19, column 45:
if (elementA instanceof FileMatch && elementB instanceof FileMatch) {
```

Compiler evidence:

```text
tokens: ok
ast: ok; class declarations, type alias, let declaration, instanceof expressions, and chained member calls parse
resolved: issue-5011 at `FileMatch` used as the first `instanceof` RHS
visible symbols: Match, FileMatch, elementA
TypeScript oracle: parses the class RHS shape and reports later TS2454 definite-assignment diagnostics for elementA/elementB
TypeScript AST path: SourceFile -> IfStatement -> BinaryExpression -> BinaryExpression -> Identifier FileMatch
```

## Completion evidence

Closed after split to `issues/open/5448-support-class-constructor-values-in-instanceof-rhs.md`.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, issue-5011 class value boundary
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts
result: pass; parser/AST ok, reproduced issue-5011 for `FileMatch` in `instanceof`
date: 2026-05-08
```

Remaining risks:

- After issue 5192 supports class constructor values, this path may expose
  later `instanceof` narrowing and definite-assignment behavior.
