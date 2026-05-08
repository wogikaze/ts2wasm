---
id: 3449
title: "Implement Narrowunknownbytypepredicate"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed as superseded by `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.
Fresh triage shows this bucket stops at the existing ambient `declare const`
name-resolution boundary before type-predicate narrowing is evaluated.

## Problem

Reference test results show 1 case failing in directory
`narrowUnknownByTypePredicate` with diagnostics: name-resolution. Fresh evidence
shows the current blocker is `declare const value1: unknown;` not being visible
when `isNotNullish(value1)` is resolved.

Problem: narrowUnknownByTypePredicate has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowUnknownByTypePredicate.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowUnknownByTypePredicate.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing ambient value name-resolution owner
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowUnknownByTypePredicate.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowUnknownByTypePredicate.ts
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

- [x] `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowUnknownByTypePredicate.ts`

## Duplicate detection

- Superseded by `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`,
  which owns resolver visibility for declaration-only ambient `declare const`
  values.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowUnknownByTypePredicate.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowUnknownByTypePredicate.ts

result:
Feature label: name-resolution
Diagnostic code: UnresolvedName
Message: unresolved name: `value1` at 234..240
Failure line 9, column 18:
if (isNotNullish(value1)) {
```

Compiler evidence:

```text
tokens: ok
ast: ok; ambient function declarations, if statements, and later declare const blocks parse
resolved: UnresolvedName for `value1` referenced as the first type-predicate call argument
visible symbols: isNotNullish, isNullish, value1
TypeScript oracle: ok, diagnostics=[]
TypeScript AST path: SourceFile -> IfStatement -> CallExpression -> Identifier value1
```

## Completion evidence

Closed as superseded by `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`;
no new child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowUnknownByTypePredicate.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, UnresolvedName for ambient const value1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowUnknownByTypePredicate.ts
result: pass; reproduced `UnresolvedName` for `value1` in `isNotNullish(value1)`
date: 2026-05-08
```

Remaining risks:

- After issue 5161 resolves ambient const values, this path may expose actual
  unknown/type-predicate narrowing requirements.
