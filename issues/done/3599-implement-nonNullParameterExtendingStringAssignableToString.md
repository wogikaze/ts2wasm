---
id: 3599
title: "Implement Nonnullparameterextendingstringassignabletostring"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded by
`issues/open/5160-lower-plain-ternary-conditional-expressions.md`.

Fresh smart triage shows this reference parses into `Expr::Ternary`, then
builtin resolution stops at the existing ternary-lowering unsupported boundary
before non-null parameter assignability semantics are reached.

## Problem

Reference test results show 1 case failing in directory
`nonNullParameterExtendingStringAssignableToString` with diagnostics:
parser-syntax. Fresh triage shows the current first blocker is:

```ts
let three = Boolean() ? one : two;
```

Problem: the compiler stops on existing ternary lowering support before it can
exercise `foo(three!)` non-null assignability behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonNullParameterExtendingStringAssignableToString.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonNullParameterExtendingStringAssignableToString.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede by the existing ternary lowering issue instead of creating a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] Superseding issue 5160 contains the implementation owner for this blocker
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonNullParameterExtendingStringAssignableToString.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonNullParameterExtendingStringAssignableToString.ts
```

Not run:

- `cargo fmt --all --check`; no Rust code changed
- `cargo nextest run`; no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5160-lower-plain-ternary-conditional-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nonNullParameterExtendingStringAssignableToString.ts`

## Duplicate detection

- `issues/open/5160-lower-plain-ternary-conditional-expressions.md` is a
  match: it owns resolver/IR/backend support for `Expr::Ternary` after parsing
  succeeds. This file fails on the same `ternary operator not yet supported`
  diagnostic.
- `issues/open/5381-parse-arrow-functions-in-ternary-branches.md` and
  `issues/open/5382-parse-typed-arrow-ternary-branches.md` are related but
  no-match: this file's ternary branches parse and are identifiers, not arrow
  functions.
- Generic/non-null/type-system buckets are no-match for this closure because
  the compiler stops before checking `foo(three!)`.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonNullParameterExtendingStringAssignableToString.ts
```

Result:

```text
Feature label: unknown-unsupported
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: ternary operator not yet supported at 175..196
Failure location: line 6, column 22
Source context: let three = Boolean() ? one : two;
tokens: ok through generic function parameters, `Boolean() ? one : two`, and later `foo(one!)` / `foo(two!)` / `foo(three!)` calls
ast: ok; `let three` initializer is `Ternary { condition: Call(Boolean), then_expr: Ident(one), else_expr: Ident(two) }`
resolved: resolve_builtins fails with UnsupportedSyntax ternary operator not yet supported
TypeScript oracle: ok, diagnostics=[]; hints include one: T, two: U, three: T | U
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonNullParameterExtendingStringAssignableToString.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Completion evidence

Commits:

- cabe7f3cf

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonNullParameterExtendingStringAssignableToString.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedSyntax, unknown-unsupported
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonNullParameterExtendingStringAssignableToString.ts
result: pass; reproduced ternary unsupported boundary, superseded by issue 5160
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After issue 5160 is implemented, this reference may expose the intended
  non-null assertion / generic string assignability behavior around
  `foo(three!)`. Split that separately if it appears.
