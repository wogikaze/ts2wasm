---
id: 3605
title: "Implement Nongenericpartialinstantiationsrelatedinbothdirections"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded by
`issues/open/5344-resolve-ambient-var-assignment-targets.md`.

Fresh smart triage shows this reference stops at the existing ambient
assignment-target name-resolution boundary for `declare let cfoo` before
partial-instantiation assignability semantics are reached.

## Problem

Reference test results show 1 case failing in directory
`nongenericPartialInstantiationsRelatedInBothDirections` with diagnostics:
name-resolution. Fresh triage shows the current first blocker is:

```ts
declare let cafoo: ObjectContaining<{ a: number, foo: number }>;
declare let cfoo: ObjectContaining<Foo>;
cfoo = cafoo;
```

Problem: the compiler erases the declaration-only ambient `let` bindings and
then reports `UnresolvedName` for the assignment target `cfoo`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nongenericPartialInstantiationsRelatedInBothDirections.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nongenericPartialInstantiationsRelatedInBothDirections.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede by the existing ambient assignment-target issue instead of creating a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] Superseding issue 5344 contains the implementation owner for this blocker
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nongenericPartialInstantiationsRelatedInBothDirections.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nongenericPartialInstantiationsRelatedInBothDirections.ts
```

Not run:

- `cargo fmt --all --check`; no Rust code changed
- `cargo nextest run`; no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5344-resolve-ambient-var-assignment-targets.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nongenericPartialInstantiationsRelatedInBothDirections.ts`

## Duplicate detection

- `issues/open/5344-resolve-ambient-var-assignment-targets.md` is a match:
  it owns declaration-only ambient `var` / `let` names used as assignment
  targets. This file fails on the same resolver boundary for
  `declare let cfoo` in `cfoo = cafoo`.
- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`
  is related but no-match for this closure because the current failing span is
  the assignment target, not a general expression reference.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nongenericPartialInstantiationsRelatedInBothDirections.ts
```

Result:

```text
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
Message: unresolved name: `cfoo` at 274..287
Failure location: line 12, column 12
Source context:
  declare let cafoo: ObjectContaining<{ a: number, foo: number }>;
  declare let cfoo: ObjectContaining<Foo>;
  cfoo = cafoo;
  cafoo = cfoo;
visible symbols: cafoo, cfoo
tokens: ok through interfaces, construct signature, ambient `declare let` declarations, and both assignments
ast: ok; ambient declarations erased; runtime AST contains `Assign cfoo = cafoo` and `Assign cafoo = cfoo`
resolved: resolve_names fails with UnresolvedName for assignment target `cfoo`
TypeScript oracle: ok, diagnostics=[]; hints include cafoo and cfoo object-containing types
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nongenericPartialInstantiationsRelatedInBothDirections.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Completion evidence

Commits:

- e6fd28cba

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nongenericPartialInstantiationsRelatedInBothDirections.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnresolvedName, name-resolution
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nongenericPartialInstantiationsRelatedInBothDirections.ts
result: pass; reproduced ambient assignment-target resolver boundary, superseded by issue 5344
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After issue 5344 resolves ambient assignment targets, this reference may
  expose partial-instantiation assignability semantics. Split that separately
  if it appears.
