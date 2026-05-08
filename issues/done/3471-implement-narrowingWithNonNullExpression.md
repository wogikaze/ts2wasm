---
id: 3471
title: "Implement Narrowingwithnonnullexpression"
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

Closed after splitting the current blocker into child issue
`issues/open/5457-support-string-match-string-pattern-argument.md`.

## Problem

Reference test results previously showed 1 case failing in directory
`narrowingWithNonNullExpression` with diagnostics: parser-syntax.

Fresh triage on 2026-05-08 shows the parser now accepts the representative
non-null assertion and optional index expressions. The first remaining blocker
is a built-in lowering boundary for `String.prototype.match` with a string
literal argument, split to issue 5457.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts
```

Not run:

- `cargo fmt --all --check` (issue-only split; no Rust changes)
- `cargo nextest run` (issue-only split; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5457-support-string-match-string-pattern-argument.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts`

## Duplicate detection

- `issues/open/5020-implement-regexp-literal.md` is a broad generated RegExp
  triage bucket, not a narrow work order for the current
  `String.prototype.match(string)` boundary.
- `issues/done/051-implement-regexp.md` implemented constrained
  `String.prototype.match` support for RegExp literal and
  `new RegExp("plain")` arguments; fresh evidence shows literal string
  arguments are still rejected.
- `issues/open/5136-fix-arity-validation-regexp-string-prototype.md` covers
  zero-argument arity relaxation, not string literal arguments.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts
```

Result:

```text
Feature label: regexp-literal
Diagnostic: UnsupportedRegExp / unsupported-feature-boundary
Message: issue-051: String.prototype.match supports only RegExp literal or new RegExp("plain") arguments in this subset at 29..41
Source: const m = ''.match('');
tokens: ok
ast: ok; `m!` is represented as Ident("m") and `m?.[0]!` as OptionalIndex
resolved: fails during lower_program
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts --detail --no-dashboard-data
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
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, current blocker captured in child issue 5457
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts
result: pass; UnsupportedRegExp issue-051 for `String.prototype.match` string argument
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- Semantic parity is not proven because this generated bucket was closed after
  splitting the first current build blocker into issue 5457.
