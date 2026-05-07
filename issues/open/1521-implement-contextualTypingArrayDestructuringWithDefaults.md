---
id: 1521
title: "Implement Contextualtypingarraydestructuringwithdefaults"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1521.

## Summary

Triage contextualTypingArrayDestructuringWithDefaults across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypingArrayDestructuringWithDefaults` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingArrayDestructuringWithDefaults has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5379-lower-array-binding-object-default-initializers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Date: 2026-05-07

Command:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts
```

Result: split to
`issues/open/5379-lower-array-binding-object-default-initializers.md`.

Current diagnostic:

```text
UnsupportedRuntimeSubset: issue-251: only literal default binding initializers are supported in this runtime slice at 58..91
feature_label: runtime-subset
```

Source context:

```ts
type I = { a: "a" };
let [ c0 = {a: "a"} ]: [I?] = [];
let [ x1, c1 = {a: "a"} ]: [number, I?] = [1];
let [ c_ = {a: "a"} ]: I[] = [];
```

Compiler evidence:

- tokens: ok
- ast: ok; array binding patterns are preserved as binding names such as
  `[c0 = {a: "a"}]` and `[x1, c1 = {a: "a"}]`
- resolved/name resolution: fails with issue-251 before lowering the
  object-literal default initializer
- TypeScript oracle reaches a later TS2322 diagnostic inside the function body,
  proving the top-level array binding defaults are accepted by TypeScript

Duplicate review:

- `issues/open/5373-lower-complex-default-binding-initializers.md` is related
  but owns object binding parameter defaults such as `({ a = 0 } = {}) => a`.
- The broad destructuring issues 251/5049 are done; this bucket needs a
  narrower follow-up for array binding defaults with object-literal
  initializers.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5379-lower-array-binding-object-default-initializers.md`

Validation result:

```text
command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, current failure is UnsupportedSyntax destructuring
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts
result: pass; reproduced issue-251 array binding object default initializer boundary and split to issue 5379
date: 2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5379 implements or
  precisely diagnoses array binding object-literal default initializers.
