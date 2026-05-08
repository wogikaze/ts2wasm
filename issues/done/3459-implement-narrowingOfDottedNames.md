---
id: 3459
title: "Implement Narrowingofdottednames"
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
`issues/open/5448-support-class-constructor-values-in-instanceof-rhs.md`.

Fresh smart triage shows the current blocker is direct class constructor value
use in an `instanceof` RHS, before any dotted-name narrowing behavior is
reached.

## Problem

Reference test results show 1 case fails in directory
`narrowingOfDottedNames` with diagnostics: parser-syntax.

Fresh triage shows parser/AST now succeed. Name resolution rejects class `A`
as the RHS of `x instanceof A`:

```ts
class A {
    prop!: { a: string; };
}

function isA(x: any): x is A {
    return x instanceof A;
}
```

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingOfDottedNames.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingOfDottedNames.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingOfDottedNames.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingOfDottedNames.ts
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

- [x] superseded by `issues/open/5448-support-class-constructor-values-in-instanceof-rhs.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingOfDottedNames.ts`

## Duplicate detection

- `issues/open/5448-support-class-constructor-values-in-instanceof-rhs.md` is
  a match: it owns direct class constructor bindings used as `instanceof` RHS
  values that currently report issue-5011.
- `issues/open/5192-support-first-class-class-constructor-values.md` is
  related but broader; 5448 is the narrower exact owner for `instanceof`.
- `issues/open/5447-support-instanceof-callable-prototype-rhs.md` is
  no-match: this case uses a direct class declaration RHS, not callable
  prototype objects that report issue-207.
- `issues/open/421-implement-class.md` is a broad generated class epic and
  should not be selected when 5448 is available.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingOfDottedNames.ts
```

Result:

```text
Feature label: class
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: issue-5011: class `A` cannot be used as a value — class runtime is not yet supported at 206..207
Failure location: line 18, column 9
Source context: return x instanceof A;
tokens: ok
ast: ok; ClassDecl A, ClassDecl B, Function isA, Return InstanceOf x instanceof A
resolved: issue-5011 at class `A` used as instanceof RHS
TypeScript oracle: has later TS2564 diagnostics for Foo1/Foo2 class fields, not this first blocker
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingOfDottedNames.ts --detail --no-dashboard-data
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
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingOfDottedNames.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedSyntax, unknown-unsupported
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingOfDottedNames.ts
result: pass; reproduced issue-5011 direct class constructor instanceof RHS blocker, superseded by issue 5448
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After issue 5448 is implemented, this reference may expose later class field,
  definite assignment, `instanceof` narrowing, or dotted-name narrowing
  behavior. Split those separately if they appear.
