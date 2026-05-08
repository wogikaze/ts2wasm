---
id: 3535
title: "Implement Noimplicitanydestructuringvardeclaration"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5473]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence splits the current
initializer-less destructuring declaration diagnostic gap to issue 5473; the
second representative now build-passes.

## Problem

Fresh triage shows two different current outcomes:

- `noImplicitAnyDestructuringVarDeclaration.ts` stops at
  `UnsupportedSyntax: issue-247: binding patterns require an initializer` for
  `var [a], {b}, c, d;`. TypeScript reports TS1182 for the same
  initializer-less destructuring declarations. This is split to
  `issues/open/5473-report-destructuring-declaration-missing-initializer.md`.
- `noImplicitAnyDestructuringVarDeclaration2.ts` now build-passes with
  initialized destructuring declarations.

Problem: generated destructuring bucket is superseded by issue 5473 for the
remaining initializer-less destructuring declaration diagnostic.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration2.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration2.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
noImplicitAnyDestructuringVarDeclaration.ts: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=destructuring:1
noImplicitAnyDestructuringVarDeclaration2.ts: executed=1 build_pass=1 unsupported=0 blocked=0
```

Current failure evidence for representative 1:

```text
UnsupportedSyntax: issue-247: binding patterns require an initializer at 49..52
```

Source context:

```ts
var [a], {b}, c, d; // error
var [a1 = undefined], {b1 = null}, c1 = undefined, d1 = null; // error
var [a2]: [any], {b2}: { b2: any }, c2: any, d2: any; // error
```

Compiler evidence:

```text
tokens: ok through initializer-less and initialized destructuring declaration forms
ast/resolved: fail before AST construction with issue-247 at [a]
```

TypeScript oracle:

```text
TS1182: A destructuring declaration must have an initializer.
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5473-report-destructuring-declaration-missing-initializer.md`. Do
not implement directly from this bucket.

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
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration2.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue split.
- `cargo nextest run`; metadata-only issue split.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5473-report-destructuring-declaration-missing-initializer.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration.ts`
- `reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration2.ts`

## Duplicate detection

- `issues/done/247-implement-destructuring-binding-pattern-parser.md` is
  related parser support but does not own TS1182-style missing initializer
  diagnostics.
- `issues/done/251-implement-destructuring-binding-runtime-semantics.md` is
  related runtime support for initialized destructuring, not this missing
  initializer diagnostic.
- Split to `issues/open/5473-report-destructuring-declaration-missing-initializer.md`.

## Smart triage

### Smart triage: Triage destructuring: noImplicitAnyDestructuringVarDeclaration

- Issue class: `triage-needed`
- Feature label: `destructuring`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration.ts`

Current compiler message:

```text
issue-247: binding patterns require an initializer at 49..52
```

TypeScript oracle:

```text
TS1182: A destructuring declaration must have an initializer.
```

`noImplicitAnyDestructuringVarDeclaration2.ts` is build-pass and does not need a
new child issue from this bucket.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
