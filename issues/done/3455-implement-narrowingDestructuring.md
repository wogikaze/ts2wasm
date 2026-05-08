---
id: 3455
title: "Implement Narrowingdestructuring"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: []
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed after splitting the current runtime-subset blocker to
`issues/open/5452-lower-nested-object-rest-binding-from-narrowed-source.md`.

## Problem

Reference test results show 1 case fails in directory `narrowingDestructuring`
with diagnostics: runtime-subset.

Fresh smart triage shows the current blocker is issue-251 object rest binding
for a nested binding pattern whose source is the non-literal `value` in a
discriminant-narrowed branch:

```ts
const { f: { a, ...spread } } = value;
```

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingDestructuring.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingDestructuring.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingDestructuring.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingDestructuring.ts
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

- [x] `issues/open/5452-lower-nested-object-rest-binding-from-narrowed-source.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingDestructuring.ts`

## Duplicate detection

- `issues/done/251-implement-destructuring-binding-runtime-semantics.md` is
  related but not a match: it implemented static object literal object rest and
  explicitly kept dynamic-source object rest out of scope.
- `issues/done/5049-ir-destructuring.md` is related but not a match for the
  same reason; the fresh reference triage still reports issue-251.
- `issues/done/3485-implement-nestedObjectRest.md` is related but split to the
  parser-only `issues/open/5462-parse-for-of-assignment-heads-with-nested-object-rest.md`,
  not the dynamic-source nested declaration object-rest owner for this path.
- Computed/default binding issues 5297, 5299, 5373, and 5379 are no-match for
  this nested object rest source-shape blocker.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingDestructuring.ts
```

Result:

```text
Feature label: runtime-subset
Diagnostic: UnsupportedRuntimeSubset / unsupported-feature-boundary
Message: issue-251: object rest binding currently requires a static object literal source in this runtime slice at 499..537
Failure location: line 20, column 9
Source context: const { f: { a, ...spread } } = value;
tokens: ok
ast: ok; Function func2, If value.kind === "f", Let name "{f:{a,...spread}}" expr value
resolved/lowered: issue-251 at nested object rest binding
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingDestructuring.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=destructuring:1
semantic_enabled=0
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingDestructuring.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedSyntax, destructuring
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingDestructuring.ts
result: pass; reproduced issue-251 object rest binding from non-literal source; split to issue 5452
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After 5452 is implemented, the same reference file may expose later object
  rest, array rest, or narrowing behavior; split those as separate issues if
  they appear.
