---
id: 1476
title: "Implement Constructorparametersinvariabledeclarations"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: [5360]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1476.

## Summary

Closed as a stale compile blocker. Fresh coverage shows
`constructorParametersInVariableDeclarations.ts` now build-passes; the remaining
gap is TypeScript semantic diagnostic TS2301, split to issue 5360.

## Problem

Reference test results originally showed one parser-syntax failure. Current
triage shows tokens, AST, resolve, and build all succeed:

```text
Diagnostic: BuildPass
Feature label: build-pass
```

TypeScript still reports TS2301 for class field initializers that reference
constructor-scope `x`.

Problem: the generated parser-syntax bucket no longer represents a compiler
blocker and needs a semantic follow-up owner.

## Current failure

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts --detail --no-dashboard-data
```

Observed:

```text
build_pass=1
unsupported=0
```

Fresh triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts
```

Compiler evidence:

```text
tokens: ok
ast: ok; ClassDecl A and B each preserve constructor bodies
resolved: ok
diagnostic: BuildPass
```

TypeScript oracle evidence:

```text
TS2301: Initializer of instance member variable 'a' cannot reference identifier 'x' declared in the constructor.
TS2301: Initializer of instance member variable 'b' cannot reference identifier 'x' declared in the constructor.
TS2301: Initializer of instance member variable 'c' cannot reference identifier 'x' declared in the constructor.
```

## Desired final state

This generated bucket is closed. Semantic parity proceeds through issue 5360.

## Scope

In scope:

- [x] Confirm the representative path now build-passes
- [x] Preserve the remaining TypeScript oracle evidence
- [x] Split the TS2301 semantic diagnostic to issue 5360

Out of scope:

- Direct implementation from this generated bucket
- Parameter property runtime support already completed by issue 226
- Derived constructor parameter properties tracked by issue 5268

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- semantic diagnostic tests

Do not touch:

- backend/runtime code for this issue-metadata closure

## Acceptance criteria

- [x] Fresh coverage records `constructorParametersInVariableDeclarations.ts` as build_pass
- [x] Fresh triage records TypeScript TS2301 oracle diagnostics
- [x] Child issue 5360 contains exact reproduction and acceptance criteria

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts
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

- [x] created: `issues/done/5360-report-class-field-initializer-constructor-scope-captures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts`

## Duplicate detection

- `issues/done/226-implement-parameter-properties.md` completed parameter
  property runtime support and is not this TS2301 field-initializer diagnostic.
- `issues/done/5268-support-derived-constructor-parameter-properties-after-super.md`
  handles derived constructor parameter properties, not ordinary field
  initializer references to constructor-scope names.
- Existing class member initializer issues mention TS2301 as later oracle
  evidence but do not own the focused single-file semantic diagnostic.

## Smart triage

Generated 2026-05-07.

```text
Path: reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts
Compiler: BuildPass
TypeScript: TS2301 for class fields a/b/c referencing constructor-scope x
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts --detail --no-dashboard-data
result: pass; build_pass=1, unsupported=0
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts
result: pass; BuildPass with TS2301 oracle diagnostics captured
date: 2026-05-07
```

Remaining risks:

- semantic diagnostic implementation remains tracked by issue 5360
