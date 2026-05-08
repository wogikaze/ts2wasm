---
id: 3458
title: "Implement Narrowingnoinfer"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5002]
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as superseded by
`issues/open/5160-lower-plain-ternary-conditional-expressions.md`.

Fresh smart triage shows this reference parses into `Expr::Ternary`, then
builtin resolution stops at the existing ternary-lowering unsupported boundary
before any `NoInfer` narrowing behavior is reached.

## Problem

Reference test results show 1 case fails in directory `narrowingNoInfer` with
diagnostics: type-system.

The current blocker is the conditional expression in the callback passed to
`map`:

```ts
const something = map(m, (_) =>
  _.result._tag === "a" ? { ..._, result: _.result } : null,
);
```

This is not yet a `NoInfer`-specific type-system slice. The parser already
creates a ternary AST node, but resolver/builtin handling rejects it.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingNoInfer1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingNoInfer1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingNoInfer1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingNoInfer1.ts
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

- [x] superseded by `issues/open/5160-lower-plain-ternary-conditional-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingNoInfer1.ts`

## Duplicate detection

- `issues/open/5160-lower-plain-ternary-conditional-expressions.md` is a
  match: it owns resolver/IR/backend support for `Expr::Ternary` after parsing
  succeeds.
- `issues/open/5381-parse-arrow-functions-in-ternary-branches.md` and
  `issues/open/5382-parse-typed-arrow-ternary-branches.md` are related but
  no-match: this file's ternary branches parse and are not arrow functions.
- Generic/type-system buckets such as 2497 and 3595 are no-match for this
  closure because the compiler stops before NoInfer inference or narrowing.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingNoInfer1.ts
```

Result:

```text
Feature label: type-system
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: ternary operator not yet supported at 389..446
Failure location: line 19, column 3
Source context: _.result._tag === "a" ? { ..._, result: _.result } : null
tokens: ok
ast: ok; Let something = Call map(..., ArrowFn body Ternary)
resolved: resolve_builtins fails with UnsupportedSyntax ternary operator not yet supported
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingNoInfer1.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-system:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingNoInfer1.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedSyntax, type-system
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingNoInfer1.ts
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

- After issue 5160 is implemented, this reference may expose object spread,
  generic callback, ambient `declare function`, `as const`, or NoInfer
  narrowing behavior. Split those separately if they appear.
