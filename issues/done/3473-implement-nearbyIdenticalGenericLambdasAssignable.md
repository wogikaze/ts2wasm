---
id: 3473
title: "Implement Nearbyidenticalgenericlambdasassignable"
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

Closed after folding the current parser blocker into
`issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`.

## Problem

Reference test results previously showed 1 case failing in directory
`nearbyIdenticalGenericLambdasAssignable` with diagnostics: type-system.

Fresh triage on 2026-05-08 shows the first current blocker is generic arrow
parser dispatch for `const fB = <T>() => { ... }`, already owned by the
generic-arrow parser work order 5304 after adding this zero-parameter
representative.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nearbyIdenticalGenericLambdasAssignable.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nearbyIdenticalGenericLambdasAssignable.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nearbyIdenticalGenericLambdasAssignable.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nearbyIdenticalGenericLambdasAssignable.ts
```

Not run:

- `cargo fmt --all --check` (issue-only duplicate fold; no Rust changes)
- `cargo nextest run` (issue-only duplicate fold; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] updated: `issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nearbyIdenticalGenericLambdasAssignable.ts`

## Duplicate detection

- folded into `issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`
  because that issue owns generic arrow parser dispatch; this representative
  is the zero-parameter block-bodied form of the same feature family.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nearbyIdenticalGenericLambdasAssignable.ts
```

Result:

```text
Feature label: type-system
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 73, end: 74 } }) at 75..77
Source: const fB = <T>() => {
tokens: ok; Less, Ident("T"), Greater, LeftParen, RightParen, Arrow
ast: fails before AST construction
resolved: fails before AST construction
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nearbyIdenticalGenericLambdasAssignable.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-system:1
semantic_enabled=0
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nearbyIdenticalGenericLambdasAssignable.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, current parser blocker folded into issue 5304
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nearbyIdenticalGenericLambdasAssignable.ts
result: pass; UnsupportedSyntax for zero-parameter generic arrow function
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- Generic lambda assignability semantics are not proven; this bucket currently
  stops before AST construction and remains blocked by issue 5304.
