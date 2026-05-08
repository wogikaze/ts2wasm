---
id: 1477
title: "Implement Constructorparametersthatshadowexternalnamesinvariabledeclarations"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1477.

## Summary

Closed as a stale compile blocker. Fresh coverage shows
`constructorParametersThatShadowExternalNamesInVariableDeclarations.ts` now
build-passes; the remaining TS2301 semantic gap is already owned by
`issues/done/5360-report-class-field-initializer-constructor-scope-captures.md`.

## Problem

Reference test results originally showed one parser-syntax failure. Current
triage shows tokens, AST, resolve, and build all succeed:

```text
Diagnostic: BuildPass
Feature label: build-pass
```

TypeScript still reports TS2301 for class field initializers that reference an
`x` shadowed by a constructor parameter or constructor-local variable.

Problem: the generated parser-syntax bucket no longer represents a compiler
blocker and is superseded by the focused semantic issue 5360.

## Current failure

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorParametersThatShadowExternalNamesInVariableDeclarations.ts --detail --no-dashboard-data
```

Observed:

```text
build_pass=1
unsupported=0
```

Fresh triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorParametersThatShadowExternalNamesInVariableDeclarations.ts
```

Compiler evidence:

```text
tokens: ok
ast: ok; top-level var x plus ClassDecl A/B
resolved: ok
diagnostic: BuildPass
```

TypeScript oracle evidence:

```text
TS2301: Initializer of instance member variable 'a' cannot reference identifier 'x' declared in the constructor.
```

Representative source:

```ts
var x = 1;
class A {
    private a = x;
    constructor(x: number) {
    }
}

class B {
    private a = x;
    constructor() {
        var x = "";
    }
}
```

## Desired final state

This generated bucket is closed. Semantic parity proceeds through issue 5360.

## Scope

In scope:

- [x] Confirm the representative path now build-passes
- [x] Preserve the remaining TypeScript TS2301 oracle evidence
- [x] Supersede this bucket with issue 5360

Out of scope:

- Direct implementation from this generated bucket
- Broad parser-syntax work
- Parameter property runtime support already completed by issue 226

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- semantic diagnostic tests

Do not touch:

- backend/runtime code for this issue-metadata closure

## Acceptance criteria

- [x] Fresh coverage records `constructorParametersThatShadowExternalNamesInVariableDeclarations.ts` as build_pass
- [x] Fresh triage records TypeScript TS2301 oracle diagnostics
- [x] Issue 5360 is updated with this reference path as additional evidence

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorParametersThatShadowExternalNamesInVariableDeclarations.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorParametersThatShadowExternalNamesInVariableDeclarations.ts
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

- [x] existing: `issues/done/5360-report-class-field-initializer-constructor-scope-captures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorParametersThatShadowExternalNamesInVariableDeclarations.ts`

## Duplicate detection

- `issues/done/5360-report-class-field-initializer-constructor-scope-captures.md`
  owns the focused TS2301 semantic diagnostic for class field initializers that
  reference constructor-scope bindings.

## Smart triage

Generated 2026-05-07.

```text
Path: reference/typescript/tests/cases/compiler/constructorParametersThatShadowExternalNamesInVariableDeclarations.ts
Compiler: BuildPass
TypeScript: TS2301 for class field initializer `a = x` shadowed by constructor-scope x
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorParametersThatShadowExternalNamesInVariableDeclarations.ts --detail --no-dashboard-data
result: pass; build_pass=1, unsupported=0
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorParametersThatShadowExternalNamesInVariableDeclarations.ts
result: pass; BuildPass with TS2301 oracle diagnostics captured
date: 2026-05-07
```

Remaining risks:

- semantic diagnostic implementation remains tracked by issue 5360
