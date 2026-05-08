---
id: 3463
title: "Implement Narrowingplainjsnocrash"
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

Closed after splitting the current parser blocker to
`issues/open/5454-parse-while-statements-with-non-block-bodies.md`.

## Problem

Reference test results show 1 case fails in directory
`narrowingPlainJsNoCrash` with diagnostics: parser-syntax.

Fresh triage shows the current blocker is the empty-statement body after a
`while` condition:

```ts
while (d !== a$b);
```

The parser requires a block body and reports
`expected LeftBrace, got Some(Semicolon)`, before reaching the later
plain-JS narrowing/no-crash behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts
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

- [x] `issues/open/5454-parse-while-statements-with-non-block-bodies.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts`

## Duplicate detection

- `issues/open/5210-parse-do-while-asi-before-block-end-or-expression.md`
  is related but no-match: it owns `do ... while` ASI, not ordinary `while`
  statement bodies.
- `issues/open/5154-parse-angle-bracket-type-assertion-statements.md` is
  no-match: it owns angle-bracket type assertions in statement position.
- Broad parser issue 059 is no-match because the new issue 5454 is the narrow
  implementation owner.
- No existing open/done issue was found for `while (condition);` or
  `while (condition) expression;`.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts
```

Result:

```text
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: expected LeftBrace, got Some(Semicolon)
Failing construct: while (d !== a$b);
tokens: ok
ast: fails because parser expects LeftBrace after while condition
resolved: same parser boundary
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts --detail --no-dashboard-data
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
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedSyntax, unknown-unsupported
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts
result: pass; reproduced non-block while body parser blocker, split to issue 5454
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After 5454 is implemented, this reference may expose assignment-expression,
  property-access, loop-lowering, or JavaScript narrowing behavior. Split those
  separately if they appear.
