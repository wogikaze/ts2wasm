---
id: 1478
title: "Implement Constructorreturningaprimitive"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: [5361]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed as a stale compile blocker. Fresh coverage shows
`constructorReturningAPrimitive.ts` now build-passes; the remaining gap is a
TypeScript constructor return-type semantic diagnostic, split to issue 5361.

## Problem

Reference test results originally showed one parser-syntax failure. Current
triage shows tokens, AST, resolve, and build all succeed:

```text
Diagnostic: BuildPass
Feature label: build-pass
```

TypeScript still reports constructor return diagnostics for `return x` in a
generic class constructor.

Problem: the generated parser-syntax bucket no longer represents a compiler
blocker and needs a semantic follow-up owner.

## Current failure

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts --detail --no-dashboard-data
```

Observed:

```text
build_pass=1
unsupported=0
```

Fresh triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts
```

Compiler evidence:

```text
tokens: ok
ast: ok; constructors with `return 1` and `return x`
resolved: ok; ClassDecl A/B and `new A()` / `new B<number>()`
diagnostic: BuildPass
```

TypeScript oracle evidence:

```text
TS2322: Type 'T' is not assignable to type 'B<T>'.
TS2409: Return type of constructor signature must be assignable to the instance type of the class.
TS2454: Variable 'x' is used before being assigned.
```

## Desired final state

This generated bucket is closed. Semantic parity proceeds through issue 5361.

## Scope

In scope:

- [x] Confirm the representative path now build-passes
- [x] Preserve the remaining TypeScript oracle evidence
- [x] Split constructor return-type semantic diagnostics to issue 5361

Out of scope:

- Direct implementation from this generated bucket
- Broad parser-syntax work
- Runtime constructor return semantics

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- semantic diagnostic tests

Do not touch:

- backend/runtime code for this issue-metadata closure

## Acceptance criteria

- [x] Fresh coverage records `constructorReturningAPrimitive.ts` as build_pass
- [x] Fresh triage records TypeScript TS2322/TS2409/TS2454 oracle diagnostics
- [x] Child issue 5361 contains exact reproduction and acceptance criteria

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5361-report-invalid-constructor-return-value-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts`

## Duplicate detection

- No exact open issue was found for constructor return-type diagnostics.

## Smart triage

Generated 2026-05-07.

```text
Path: reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts
Compiler: BuildPass
TypeScript: TS2322 / TS2409 / TS2454 at `return x`
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts --detail --no-dashboard-data
result: pass; build_pass=1, unsupported=0
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorReturningAPrimitive.ts
result: pass; BuildPass with TS2322/TS2409/TS2454 oracle diagnostics captured
date: 2026-05-07
```

Remaining risks:

- semantic diagnostic implementation remains tracked by issue 5361
