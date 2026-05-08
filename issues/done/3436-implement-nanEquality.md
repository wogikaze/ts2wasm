---
id: 3436
title: "Implement Nanequality"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed as superseded by
`issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.

Fresh focused coverage and triage show `nanEquality.ts` currently fails before
NaN comparison diagnostics because the erased ambient declaration
`declare const x: number;` is not visible to name resolution. That exact
declaration-only ambient value name-resolution boundary is already owned by
issue 5161.

## Problem

Reference test results show 1 cases fail in directory `nanEquality` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nanEquality had 1 generated reference failure and needed smart-triage
evidence before implementation starts.

Disposition: no child issue created because the current first blocker is
covered by existing open issue 5161.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nanEquality.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nanEquality.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as superseded by an existing implementation-ready owner issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Fresh evidence contains an exact `reference-triage` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing owner issue 5161 names the exact current diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nanEquality.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nanEquality.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nanEquality.ts`

## Duplicate detection

- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`
  owns resolver visibility for declaration-only ambient `declare const` values
  such as `x` and `y`.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nanEquality.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/nanEquality.ts: UnresolvedName: name-resolution
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nanEquality.ts

result:
UnresolvedName: unresolved name: `x` at 49..50
```

Source context:

```ts
declare const x: number;

if (x === NaN) {}
if (NaN === x) {}

if (x == NaN) {}
if (NaN == x) {}
```

Compiler evidence:

```text
tokens: ok through declare const x, all equality/inequality conditions, declare let y, and functions with parameter named NaN
ast: ok; first statements are if conditions comparing x and NaN in both orders
resolved: fails in resolve_names with UnresolvedName for x at the first condition
visible symbols: extraction sees binding x from line 2, but resolver metadata does not retain it
```

TypeScript oracle evidence:

```text
TS2845: This condition will always return 'false'.  // x === NaN
TS2845: This condition will always return 'false'.  // NaN === x
TS2845: This condition will always return 'true'.   // x !== NaN
TS2845: This condition will always return 'true'.   // NaN !== x
```

The later functions where `NaN` is a parameter are accepted by TypeScript and
should not be treated as global-NaN diagnostics.

## Completion evidence

Closed as superseded by issue 5161; no additional child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nanEquality.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nanEquality.ts
result: pass; current first blocker is UnresolvedName for ambient `x`, owned by issue 5161
date: 2026-05-08
```

Remaining risks:

- After issue 5161 advances this path, TS2845-style always-true/false
  diagnostics for comparisons against the global `NaN` may need a focused
  semantic issue.
