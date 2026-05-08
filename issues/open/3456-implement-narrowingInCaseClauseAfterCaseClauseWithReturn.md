---
id: 3456
title: "Implement Narrowingincaseclauseaftercaseclausewithreturn"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as superseded by
`issues/open/5303-parse-trailing-comma-in-typed-function-parameters.md`.

Fresh smart triage shows this reference still fails before AST construction at
the closing `)` after a trailing comma in an ordinary typed function parameter
list.

## Problem

Reference test results show 1 case fails in directory
`narrowingInCaseClauseAfterCaseClauseWithReturn` with diagnostics:
parser-syntax.

The current blocker is not case-clause narrowing. It is the parser's trailing
parameter comma handling:

```ts
function test3(
  foo:
    | { kind: "a"; prop: string }
    | { kind: "b"; prop: number }
    | { kind: "c"; prop: boolean },
  bar?: {
    type: "b";
  },
) {
```

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingInCaseClauseAfterCaseClauseWithReturn.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingInCaseClauseAfterCaseClauseWithReturn.ts --detail
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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingInCaseClauseAfterCaseClauseWithReturn.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingInCaseClauseAfterCaseClauseWithReturn.ts
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

- [x] superseded by `issues/open/5303-parse-trailing-comma-in-typed-function-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingInCaseClauseAfterCaseClauseWithReturn.ts`

## Duplicate detection

- `issues/open/5303-parse-trailing-comma-in-typed-function-parameters.md` is
  a match: it owns trailing commas before `)` in ordinary typed function
  declaration parameter lists.
- `issues/open/5278-parse-trailing-comma-in-function-parameters-with-comments.md`
  is related but no-match: it is comment-specific.
- `issues/open/5149-parse-trailing-comma-in-typed-class-method-parameters.md`
  is related but no-match: it is class-method-specific.
- `issues/open/059-implement-parser-syntax-extensions.md` is the broad parser
  epic and should not be selected directly when 5303 is the narrower owner.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingInCaseClauseAfterCaseClauseWithReturn.ts
```

Result:

```text
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: issue-247: expected binding identifier or pattern, got Some(RightParen) at 903..904
Failure location: line 46, column 1
Source context: closing ")" after `bar?: { type: "b"; },`
tokens: ok
ast: fails before AST construction
resolved: same parser diagnostic
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingInCaseClauseAfterCaseClauseWithReturn.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingInCaseClauseAfterCaseClauseWithReturn.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedSyntax, unknown-unsupported
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingInCaseClauseAfterCaseClauseWithReturn.ts
result: pass; reproduced issue-247 trailing typed parameter comma, superseded by issue 5303
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After issue 5303 is implemented, this reference file may expose the intended
  switch case-clause narrowing behavior; split that separately if needed.
