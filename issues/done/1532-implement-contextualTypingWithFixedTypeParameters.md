---
id: 1532
title: "Implement Contextualtypingwithfixedtypeparameters"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5273]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualTypingWithFixedTypeParameters across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypingWithFixedTypeParameters` with diagnostics: arrow-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingWithFixedTypeParameters has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingWithFixedTypeParameters1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingWithFixedTypeParameters1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingWithFixedTypeParameters1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingWithFixedTypeParameters1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingWithFixedTypeParameters1.ts`

## Duplicate detection

- Superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`.
  The current first blocker is the same nested zero-argument arrow return
  parser boundary, here in `() => a => a.foo`.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingWithFixedTypeParameters1.ts

result:
Feature label: arrow-function
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
error: expected Comma, got Some(Arrow) at 120..122

source context:
declare var f10: <T>(x: T, b: () => (a: T) => void, y: T) => T;
f10('', () => a => a.foo, ''); // a is ""
var r9 = f10('', () => (a => a.foo), 1); // error

compiler evidence:
tokens: ok through the nested arrow `() => a => a.foo`
ast: fails at the inner arrow token
visible symbols: ambient binding `f10`
TypeScript AST: CallExpression -> ArrowFunction `() => a => a.foo` -> ArrowFunction `a => a.foo`
TypeScript oracle: TS2339 for `a.foo` and TS2345 for final argument after parsing succeeds
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingWithFixedTypeParameters1.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingWithFixedTypeParameters1.ts
result:
pass; reproduced nested zero-argument arrow return parser failure at `() => a => a.foo`
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5273 parses nested
  zero-argument arrow returns.
