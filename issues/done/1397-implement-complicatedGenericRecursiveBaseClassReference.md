---
id: 1397
title: "Implement Complicatedgenericrecursivebaseclassreference"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5002]
blocks: [5293]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1397.

## Summary

Triage complicatedGenericRecursiveBaseClassReference across 1 failing reference
test case and split this generated bucket into implementation-ready child issue
5293.

## Problem

Reference test results showed 1 case failing in directory
`complicatedGenericRecursiveBaseClassReference` with diagnostics:
type-system. Fresh smart triage confirms a concrete lower-program boundary
rather than a broad generated bucket.

Problem: `complicatedGenericRecursiveBaseClassReference.ts` reaches tokens and
AST, then reports an opaque `Unknown` / `unknown` blocker after
`lower_program`; issue 5293 now owns the actionable fix.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the actionable work has been split into
issue 5293. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5293-handle-recursive-generic-self-heritage-class-lowering.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts`

## Duplicate detection

Fresh smart triage listed broad same-feature type-system candidates, but none
was an exact owner for this self-referential generic class heritage
lower-program failure:

- `issues/done/1264-implement-coAndContraVariantInferences-type-system.md`
- `issues/open/2497-implement-genericCallInferenceConditionalType-type-system.md`
- `issues/open/2543-implement-genericFunctionsWithOptionalParameters-type-system.md`
- `issues/open/2560-implement-genericMergedDeclarationUsingTypeParameter-type-system.md`
- `issues/open/2592-implement-genericWithIndexerOfTypeParameterType-type-system.md`
- `issues/open/2812-implement-inferenceContextualReturnTypeUnion-type-system.md`
- `issues/open/2859-implement-inheritanceOfGenericConstructorMethod-type-system.md`
- `issues/open/3223-implement-mappedTypeRecursiveInference-type-system.md`
- `issues/done/345-implement-tsc-type-alias-coverage.md`

Related issue 5156 covers parser failures on generic type arguments in class
heritage. It is not a duplicate because this case already reaches AST and fails
later in `lower_program`.

Resolution:

```text
Split to issue 5293: handle or specifically diagnose recursive generic self-heritage class lowering.
```

## Smart triage

### Smart triage: Triage type system: complicatedGenericRecursiveBaseClassReference

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `Unknown` / `unknown`
- Path: `reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts
```

Source context:

```ts
// @target: es2015
class S18<B, A, C> extends S18<A[], { S19: A; (): A }[], C[]>
{
}
(new S18(123)).S18 = 0;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "S18",
    "line": 2,
    "column": 1
  }
]
```

Compiler evidence:

```text
tokens: ok; includes class S18 generic parameters, extends S18<A[], { S19: A; (): A }[], C[]>, and new S18(123)
ast: ok; ClassDecl S18 extends Ident S18 plus property assignment to new S18(123).S18
resolved: ok=False; pipeline reached lower_program before reporting blocked
```

TypeScript oracle evidence:

```text
TS2506: 'S18' is referenced directly or indirectly in its own base expression.
TS2554: Expected 0 arguments, but got 1.
TS2339: Property 'S18' does not exist on type 'S18<unknown, unknown, unknown>'.
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=0
blocked=1
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts
result: pass; reproduces Diagnostic Unknown / unknown after lower_program with AST available
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, blocked=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- Issue 5293 may reveal a later runtime behavior gap for JavaScript's emitted
  `class S18 extends S18 {}` temporal-dead-zone semantics after the opaque
  lower-program blocker is removed.
